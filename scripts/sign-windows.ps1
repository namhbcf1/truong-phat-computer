<#
.SYNOPSIS
    Signs a Windows binary with the project Authenticode certificate.

.DESCRIPTION
    Tauri calls this script for the executables it produces when
    `bundle.windows.signCommand` is configured, and `scripts/build-privileged.mjs`
    calls it through TRUONGPHATCOMPUTER_PRIVILEGED_SIGN_COMMAND so the embedded
    privileged helper is signed before packaging.

    The certificate is selected by SHA-1 thumbprint from the current user's
    personal store. Set TRUONGPHATCOMPUTER_SIGN_CERT_SHA1 to select a different
    certificate; the default is the project self-signed certificate, which is
    trusted only on machines where its public part was installed.

    Certificate lookup and verification use .NET and signtool rather than the
    PowerShell Security module. Windows PowerShell started from a PowerShell 7
    parent inherits an incompatible PSModulePath, so the Cert: drive and
    Get-AuthenticodeSignature are unavailable exactly in the build environment
    this script runs in.

    Signing fails closed: a missing certificate, a missing private key, a missing
    tool, a rejected timestamp, or an unverifiable result all exit non-zero so a
    release build never silently ships unsigned binaries.
#>
[CmdletBinding()]
param(
    [Parameter(Mandatory = $true, Position = 0)]
    [string]$File,

    [string]$Thumbprint = $env:TRUONGPHATCOMPUTER_SIGN_CERT_SHA1,

    [string]$TimestampUrl = 'http://timestamp.digicert.com'
)

$ErrorActionPreference = 'Stop'

if ([string]::IsNullOrWhiteSpace($Thumbprint)) {
    $Thumbprint = '98EE97324712B4D8C5912B6B852A51A0BFA95186'
}
$Thumbprint = ($Thumbprint -replace '\s', '').ToUpperInvariant()

$target = (Resolve-Path -LiteralPath $File).ProviderPath
if (-not (Test-Path -LiteralPath $target -PathType Leaf)) {
    Write-Error "Signing target not found: $File"
    exit 1
}

$store = New-Object System.Security.Cryptography.X509Certificates.X509Store('My', 'CurrentUser')
try {
    $store.Open([System.Security.Cryptography.X509Certificates.OpenFlags]::ReadOnly)
    $certificate = $store.Certificates |
        Where-Object { $_.Thumbprint -eq $Thumbprint } |
        Select-Object -First 1
}
finally {
    $store.Close()
}

if (-not $certificate) {
    Write-Error "Code-signing certificate $Thumbprint is not installed in the current user's personal store"
    exit 1
}
if (-not $certificate.HasPrivateKey) {
    Write-Error "Code-signing certificate $Thumbprint has no private key on this machine"
    exit 1
}

$signTool = Get-ChildItem -Path 'C:\Program Files (x86)\Windows Kits\10\bin' -Recurse -Filter 'signtool.exe' -ErrorAction SilentlyContinue |
    Where-Object { $_.FullName -match '\\x64\\signtool\.exe$' } |
    Sort-Object FullName -Descending |
    Select-Object -First 1

if (-not $signTool) {
    Write-Error 'signtool.exe was not found under C:\Program Files (x86)\Windows Kits\10\bin'
    exit 1
}

& $signTool.FullName sign /sha1 $Thumbprint /fd sha256 /tr $TimestampUrl /td sha256 $target
if ($LASTEXITCODE -ne 0) {
    Write-Error "signtool sign failed for $target with exit code $LASTEXITCODE"
    exit $LASTEXITCODE
}

# /pa applies the Authenticode policy, which is what a client machine evaluates.
# Verifying separately prevents reporting success for a signature a target machine
# would reject because the certificate chain is not trusted there.
& $signTool.FullName verify /pa /q $target
if ($LASTEXITCODE -ne 0) {
    Write-Error "Authenticode verification failed for $target with exit code $LASTEXITCODE"
    exit $LASTEXITCODE
}

Write-Output "Signed and verified: $target"
exit 0

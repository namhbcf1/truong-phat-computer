# Windows code signing

Every Windows release is Authenticode-signed. Three pieces cooperate:

| Piece | Responsibility |
| --- | --- |
| `scripts/sign-windows.ps1` | Signs one file with signtool, a SHA-256 digest, an RFC 3161 timestamp, and verifies the result with `signtool verify /pa`. Fails closed. |
| `src-tauri/tauri.windows.conf.json` | `bundle.windows.certificateThumbprint` makes Tauri sign the application binary, the NSIS plugin DLLs, and the installer. |
| `.github/workflows/release-windows.yml` | Imports the certificate from repository secrets, builds, signs, and verifies the installer on every tag. |

`scripts/build-privileged.mjs` signs the embedded privileged helper through `TRUONGPHATCOMPUTER_PRIVILEGED_SIGN_COMMAND` and refuses to package it when the signature does not verify. That gate is why a release build needs a certificate even locally.

## Current certificate

The project ships with a self-signed certificate:

- Subject: `CN=TRUONG PHAT COMPUTER, O=TRUONG PHAT COMPUTER, C=VN`
- Thumbprint: `98EE97324712B4D8C5912B6B852A51A0BFA95186`
- Valid until: 2029-10-18, RSA-2048, EKU Code Signing

A self-signed signature is valid only where that certificate is trusted. Clients that install the public certificate stop seeing the unknown-publisher prompt:

```powershell
certutil -addstore Root TruongPhatComputer-CodeSigning.cer
certutil -addstore TrustedPublisher TruongPhatComputer-CodeSigning.cer
```

## Repository secrets

`release-windows.yml` reads two secrets. Generate the values from the certificate that holds the private key:

```powershell
$thumbprint = '98EE97324712B4D8C5912B6B852A51A0BFA95186'
$password = -join ((48..57) + (65..90) + (97..122) | Get-Random -Count 24 | ForEach-Object { [char]$_ })
$secure = ConvertTo-SecureString -String $password -AsPlainText -Force
Export-PfxCertificate -Cert "Cert:\CurrentUser\My\$thumbprint" -FilePath code-signing.pfx -Password $secure | Out-Null
$base64 = [Convert]::ToBase64String([IO.File]::ReadAllBytes('code-signing.pfx'))

$base64 | gh secret set SIGNING_PFX_BASE64
$password | gh secret set SIGNING_PFX_PASSWORD

Remove-Item code-signing.pfx
```

Never commit the `.pfx` or its password. `/.local/` is already ignored for local copies.

## Publicly trusted signing with SignPath Foundation

SignPath Foundation issues a free code-signing certificate to open-source projects. The private key stays in their HSM, so nothing secret reaches this repository.

1. Publish the repository publicly. The project is GPL-3.0-only, which satisfies their licence requirement.
2. Apply at <https://signpath.org/apply> with the repository URL. Approval is manual and can take days.
3. In SignPath, create a project for the repository, a signing policy, and an artifact configuration that signs the NSIS installer (`*.exe` produced by the `nsis-installer` artifact).
4. Store the connection values:

```powershell
gh secret set SIGNPATH_API_TOKEN
gh secret set SIGNPATH_ORGANIZATION_ID
gh variable set SIGNPATH_PROJECT_SLUG
gh variable set SIGNPATH_SIGNING_POLICY_SLUG
gh variable set SIGNPATH_ARTIFACT_CONFIGURATION_SLUG
```

The `signpath` job runs only when `SIGNPATH_PROJECT_SLUG` exists, so the workflow stays green before approval.

SignPath signs the installer it receives. The application binary and the privileged helper inside the installer keep the certificate that produced them, which is why the self-signed certificate remains configured in `tauri.windows.conf.json`. To move them to a publicly trusted certificate, replace the thumbprint and the `SIGNING_*` secrets with the new certificate; the workflow and scripts need no other change.

## Azure Trusted Signing (paid alternative)

Roughly 10 USD per month, no hardware token, works with an individual developer account. It uses the `Trusted Signing` signing plugin instead of a local certificate, so `certificateThumbprint` would be replaced by `signCommand` invoking `TrustedSigningClientTool` or the Azure CLI. The verification step in `release-windows.yml` stays unchanged.

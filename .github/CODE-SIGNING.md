# Code signing policy

Free code signing provided by [SignPath.io](https://about.signpath.io), certificate by [SignPath Foundation](https://signpath.org).

Every Windows release is Authenticode-signed. The signature is produced in the
`Release Windows installer` workflow
([`.github/workflows/release-windows.yml`](workflows/release-windows.yml)) from
the source code at the tagged commit, so a valid signature also proves that the
published installer was built from this repository.

## Team roles

The project is maintained by a single owner account, so the same person holds
every role. No change reaches a release without the owner reviewing it.

- Committers and reviewers: [@namhbcf1](https://github.com/namhbcf1)
- Approvers: [@namhbcf1](https://github.com/namhbcf1)

Every signing request is approved manually by the approver before SignPath
signs the artifact.

## Privacy policy

This program will not transfer any information to other networked systems
unless specifically requested by the user or the person installing or operating
it.

The application analyzes local disks and performs local cleanup. It contains no
telemetry, no analytics, and no update channel. Network access is limited to the
installer downloading the Microsoft WebView2 runtime when it is not already
present; the released installer embeds that runtime instead, so it needs no
network access at all. No user data leaves the machine. See
[`SECURITY.md`](../SECURITY.md) for the security policy and how to report a
vulnerability.

## What is signed

- `TruongPhatComputer.exe`, the application binary
- `TruongPhatComputerPrivileged.exe`, the privileged helper embedded in the
  application binary and verified by Authenticode before it is executed
- `TruongPhatComputer_<version>_x64-setup.exe`, the NSIS installer
- The NSIS plugin DLLs that the installer carries

## Verifying a download

```powershell
Get-AuthenticodeSignature .\TruongPhatComputer_1.2.0_x64-setup.exe
```

A correct download reports `Status: Valid` and a signer whose subject is
`CN=SignPath Foundation`.

## Until the SignPath certificate is active

Releases published before the SignPath certificate was issued carry a
self-signed project certificate (`CN=TRUONG PHAT COMPUTER`). Such a signature is
valid only on machines that trust that certificate; the release notes state the
certificate file needed. Releases signed by SignPath Foundation do not require
any extra trust step.

## Builds outside SignPath

`scripts/sign-windows.ps1` signs a file with a certificate from the local
certificate store and verifies the result with `signtool verify /pa`. Local
development builds use it; release signing goes through SignPath.
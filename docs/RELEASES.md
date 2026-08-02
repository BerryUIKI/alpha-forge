# Release Process

This MVP is distributed as a macOS Apple Silicon DMG and a Windows NSIS EXE. @BerryUIKI owns release preparation and signing credentials.

## Before publishing

1. Build and test the target platform from the approved release branch.
2. Produce the DMG on macOS Apple Silicon with `pnpm tauri build --bundles dmg` and the NSIS EXE on Windows with `pnpm tauri build --bundles nsis`.
3. Sign release artifacts only with credentials kept in the approved release environment.
4. Publish checksums, release notes, and the artifacts to GitHub Releases.
5. Verify the GitHub Releases page and a clean-device manual download.

## Update behavior

The app checks GitHub Releases only when a user requests it from Settings. It opens a newer release only when GitHub returns an HTTPS page for this repository. It never downloads or installs an update automatically.

## Known distribution limits

- macOS notarization is not planned for the MVP, so users may see a Gatekeeper warning.
- GitHub access may be unreliable for users in Mainland China. A mirror is not part of this release and needs a separate decision.
- Rollback consists of removing or marking a faulty GitHub Release and publishing a corrected release. The release owner must preserve the prior artifact and release notes for comparison.

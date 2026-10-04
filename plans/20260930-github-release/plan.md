# GitHub CI and tagged releases

Date: 2026-09-30

## Scope

Run reproducible Rust checks on pull requests and `main`, then build CLI archives
and publish a GitHub Release from a matching SemVer tag. This phase does not
publish Rust crates or Python wheels to package registries.

## Tasks

- [x] Inspect workspace versions, test boundaries, and dependency access.
- [x] Implement reusable CI and its push, pull-request, and manual entry points.
- [x] Implement tag validation, native builds, archive checksums, and publication.
- [x] Document release steps and repository setup.
- [x] Validate workflow syntax with actionlint and run local Rust tests, Python
  extension checks, and release-script tests.
- [x] Check the workspace on Rust 1.96.0; build, package, extract, and smoke-test
  the native Windows release; test all four archive targets and missing assets.
- [ ] Resolve private dependency access and activate the workflows on GitHub.

## Dependency access

The public repository depends on private Git repositories: `exr-rs`, `jpg-rs`,
`jph-rs`, `codec-simd-rs`, and `murmur3`. Choose public dependency access or
explicit, read-only authentication before expecting hosted builds to pass.
Fork pull requests cannot use repository secrets; never run their code through
`pull_request_target` with credentials. Preserve locked revisions and use
`--locked` for Cargo operations.

## CI contract

- Stable Rust tests on Linux, Windows, and macOS.
- Exclude `exiftool-py` from Rust tests; check its extension-module build separately.
- Check Rust 1.96, required by the locked dependency graph; update the workspace
  minimum to match.
- Build API documentation with denied warnings as an advisory baseline check.
- Report formatting, Clippy, and Rustdoc initially as advisory because the baseline
  has failures. Make them required after a separately reviewed cleanup.
- Pin external actions by commit, minimize token permissions, use timeouts, and
  cancel superseded branch runs. Do not cache private dependency sources or
  compiled outputs in the public repository.

## Release contract

- Accept `vMAJOR.MINOR.PATCH` with an optional SemVer prerelease suffix;
  require the version without `v` to equal `workspace.package.version`.
- Reuse the complete required CI gate for the tagged commit.
- Require the tagged commit to be an ancestor of `main`.
- Build `exif` natively for Linux x86_64 (`ubuntu-22.04`), Windows x86_64
  (`windows-2022`), macOS x86_64 (`macos-15-intel`), and macOS arm64 (`macos-15`).
- Smoke-test `--help` and `--version`; include README and all three license files
  in each archive; produce SHA-256 checksums.
- Assemble every asset before publication. Use a draft during upload, detect
  prereleases from SemVer, and publish only after assets are complete.
- Resume draft uploads safely; refuse to replace assets of a published release.
- Grant `contents: write` only to publication; retain failed-build diagnostics.

## Acceptance

Invalid tags fail before publication. Required CI failures prevent releases.
Each valid tag produces four archives and checksums, with no partial public
release. The guide at [docs/releasing.md](../../docs/releasing.md) describes the
same workflow and its authentication prerequisite.

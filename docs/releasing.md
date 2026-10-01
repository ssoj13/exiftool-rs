# Releasing the CLI

Push a version tag to build CLI binaries and publish a GitHub Release. This
workflow does not publish to crates.io or PyPI.

## Prepare the repository

Enable GitHub Actions and ensure runners can fetch every locked Git dependency.
The dependencies currently include private repositories: `exr-rs`, `jpg-rs`,
`jph-rs`, `codec-simd-rs`, and `murmur3`. Making this repository public does not
make those dependencies readable. Configure read-only dependency access or make
the dependencies public before starting the first hosted build. For private
dependencies, add the Actions secret `DEPENDENCIES_TOKEN` with read-only access
to all five repositories. The checkout's `GITHUB_TOKEN` cannot read other private
repositories. Fork pull requests do not receive this secret and cannot complete
dependency-based checks while the dependencies remain private.

Require all three **Test** checks and **Minimum Rust (1.96)** in branch protection
after the first successful run. Formatting, Clippy, and Rustdoc initially report
existing baseline problems as advisory checks; resolve that baseline before
making those checks release requirements.

## Create a version tag

1. Update `workspace.package.version` in the root `Cargo.toml`. For example, set
   it to `0.1.1` for tag `v0.1.1`. Update the changelog for the release.
2. Commit the release changes, merge them into `main`, and wait for required CI
   checks to pass.
3. Tag that commit and push the tag:

   ```bash
   git switch main
   git pull --ff-only
   git tag -a v0.1.1 -m "Release v0.1.1"
   git push origin v0.1.1
   ```

The workflow verifies that the tag contains a valid SemVer version, matches
the workspace version exactly, and points to a commit included in `main`.
For a prerelease, use matching values such as
`0.2.0-rc.1` and `v0.2.0-rc.1`; the GitHub Release is marked as a prerelease.
Tags with SemVer build metadata (`+...`) are not supported.
The Python package has its own version in `crates/exiftool-py/Cargo.toml` and is
not published by this workflow.

## Verify the release

Open the release workflow in **Actions**. Required CI checks run again for the
tagged commit before release builds start. Publication happens only after all
four native builds and their smoke checks succeed.

The release contains CLI archives for Linux x86_64, Windows x86_64, macOS
x86_64, and macOS arm64, plus SHA-256 checksums. Each archive includes `exif`
(`exif.exe` on Windows), `README.md`, `LICENSE`, `LICENSE-ARTISTIC`, and
`LICENSE-GPL`. The Linux binary targets GNU/Linux; it is not a musl build.

Download the archive for your platform, verify its checksum, extract it, and run
`exif --version` and `exif --help`. On Windows, use `exif.exe`.

## Recover a failed run

Inspect the failed job before rerunning it. Fix dependency credentials in the
repository configuration when authentication failed. For a source defect,
commit the fix and create a new version tag; do not move an existing published
tag. A failed upload can leave a draft release, which must contain every expected
asset before publication. Rerunning the workflow resumes a draft upload; it
refuses to replace assets of an already published release.

See [building from source](src/building.md) for local build instructions.

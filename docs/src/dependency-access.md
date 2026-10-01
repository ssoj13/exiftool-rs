# Dependency access

Source builds require access to every Git repository pinned in `Cargo.lock`,
including transitive dependencies. Making `exiftool-rs` public does not grant
access to other private repositories.

## Required Git repositories

| Repository | Packages in the lockfile | Relationship |
|------------|--------------------------|--------------|
| [exr-rs](https://github.com/ssoj13/exr-rs) | `exr-core`, `deflate-rs`, `imath-rs` | Direct through `exr-core` |
| [jpg-rs](https://github.com/ssoj13/jpg-rs) | `jpg-rs` | Direct |
| [jph-rs](https://github.com/ssoj13/jph-rs) | `jph-rs` | Direct and transitive source entries |
| [codec-simd-rs](https://github.com/ssoj13/codec-simd-rs) | `codec-simd` | Transitive |
| [murmur3](https://github.com/ssoj13/murmur3) | `murmur3` | Transitive |

The lockfile is authoritative for revisions. A private repository can appear
as a missing repository or revision to callers without access.

## Local builds

The manifests use SSH Git URLs. Authenticate your SSH key with GitHub and ensure
your account can read every repository above, then fetch from the project root:

```bash
cargo fetch --locked
```

If using HTTPS instead, configure Git authentication and URL rewriting in your
own environment. Keep credentials out of manifests, lockfiles, and committed
configuration. A successful `gh auth status` alone does not configure Cargo's
SSH access.

## GitHub Actions

The local `fetch-dependencies` action rewrites GitHub SSH URLs to HTTPS for its
fetch process. When supplied, it uses the **`DEPENDENCIES_TOKEN`** Actions secret.
The workflow's ordinary `GITHUB_TOKEN` is scoped to this repository and cannot
read other private repositories.

Create a dedicated fine-grained personal access token in
[GitHub settings](https://github.com/settings/personal-access-tokens/new):

1. Select resource owner `ssoj13` and only the five dependency repositories above.
2. Set repository **Contents** permission to **Read-only**.
3. Choose an expiration and create the token.
4. In your terminal, save it as a repository Actions secret:

   ```bash
   gh secret set DEPENDENCIES_TOKEN --repo ssoj13/exiftool-rs
   ```

Paste it at the interactive prompt. `gh secret set` stores an existing token;
it does not generate a personal access token. Do not substitute a broadly scoped
interactive login token for the dedicated dependency credential.

See GitHub's [token documentation](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens)
and [secret command](https://cli.github.com/manual/gh_secret_set).

## Verify or recover a failed fetch

```bash
gh secret list --repo ssoj13/exiftool-rs
gh run list --repo ssoj13/exiftool-rs --limit 5
gh run view --repo ssoj13/exiftool-rs --log-failed
```

Select the failed run and inspect the dependency-fetch step. After fixing access,
rerun it with `gh run rerun RUN_ID --failed --repo ssoj13/exiftool-rs`, replacing
`RUN_ID` with the actual run number. Authentication errors alone are not a reason
to update dependency revisions.

## Public contribution builds

Fork pull requests do not receive repository secrets. To support unrestricted
source builds and fork CI, make all required dependency repositories publicly
readable or replace their private dependencies. A CI-only token fixes hosted
build access, but does not make public source builds reproducible without access.

See [installation](getting-started/installation.md), [CI/CD](ci-cd.md),
and [release preparation](../releasing.md).

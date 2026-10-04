"""Validate release tags and package native CLI binaries without extra dependencies."""

import argparse
import hashlib
import os
from pathlib import Path
import re
import shutil
import tarfile
import tomllib
import zipfile

ROOT = Path(__file__).resolve().parents[2]
IDENTIFIER = r"(?:0|[1-9][0-9]*|[0-9]*[A-Za-z-][0-9A-Za-z-]*)"
SEMVER = re.compile(
    rf"v(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)"
    rf"(?:-({IDENTIFIER}(?:\.{IDENTIFIER})*))?"
)
TARGETS = {
    "x86_64-unknown-linux-gnu",
    "x86_64-pc-windows-msvc",
    "x86_64-apple-darwin",
    "aarch64-apple-darwin",
}


def validate(tag, version):
    match = SEMVER.fullmatch(tag)
    if match is None:
        raise ValueError("Expected vMAJOR.MINOR.PATCH or vMAJOR.MINOR.PATCH-prerelease")
    if tag[1:] != version:
        raise ValueError(f"Tag {tag} does not match workspace version {version}")
    return bool(match.group(4))


def package(tag, target):
    if target not in TARGETS:
        raise ValueError(f"Unsupported target: {target}")
    version = tomllib.loads((ROOT / "Cargo.toml").read_text())["workspace"]["package"]["version"]
    validate(tag, version)
    windows = "windows" in target
    name = f"exiftool-rs-{tag}-{target}"
    stage = ROOT / "dist" / name
    stage.mkdir(parents=True, exist_ok=True)
    binary = "exif.exe" if windows else "exif"
    shutil.copy2(ROOT / "target" / target / "release" / binary, stage / binary)
    for filename in ("README.md", "LICENSE", "LICENSE-ARTISTIC", "LICENSE-GPL"):
        shutil.copy2(ROOT / filename, stage / filename)
    archive = ROOT / "dist" / (name + (".zip" if windows else ".tar.gz"))
    if windows:
        with zipfile.ZipFile(archive, "w", compression=zipfile.ZIP_DEFLATED) as output:
            for path in sorted(stage.iterdir()):
                output.write(path, f"{name}/{path.name}")
    else:
        with tarfile.open(archive, "w:gz") as output:
            output.add(stage, arcname=name)
    print(archive)


def checksums(directory, tag):
    version = tomllib.loads((ROOT / "Cargo.toml").read_text())["workspace"]["package"]["version"]
    validate(tag, version)
    expected = {
        f"exiftool-rs-{tag}-{target}" + (".zip" if "windows" in target else ".tar.gz")
        for target in TARGETS
    }
    actual = {path.name for path in directory.iterdir() if path.is_file()}
    if actual != expected:
        raise ValueError(f"Unexpected release assets: missing={expected - actual}, extra={actual - expected}")
    lines = []
    for name in sorted(expected):
        with (directory / name).open("rb") as source:
            digest = hashlib.file_digest(source, "sha256").hexdigest()
        lines.append(f"{digest}  {name}\n")
    (directory / "SHA256SUMS").write_text("".join(lines), encoding="ascii")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=("validate", "package", "checksums"))
    parser.add_argument("--tag", default=os.environ.get("GITHUB_REF_NAME"))
    parser.add_argument("--target")
    parser.add_argument("--directory", type=Path, default=ROOT / "dist")
    args = parser.parse_args()
    version = tomllib.loads((ROOT / "Cargo.toml").read_text())["workspace"]["package"]["version"]
    prerelease = validate(args.tag or "", version)
    if args.command == "validate":
        if os.environ.get("GITHUB_OUTPUT"):
            with open(os.environ["GITHUB_OUTPUT"], "a", encoding="utf-8") as output:
                output.write(f"prerelease={str(prerelease).lower()}\n")
        print(f"Validated {args.tag} (prerelease={prerelease})")
    elif args.command == "package":
        package(args.tag, args.target)
    else:
        checksums(args.directory, args.tag)


if __name__ == "__main__":
    main()

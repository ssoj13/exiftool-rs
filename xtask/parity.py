#!/usr/bin/env python3
"""Compare FileType from Perl ExifTool vs exiftool-rs CLI on golden testdata."""
from __future__ import annotations

import json
import os
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TESTDATA = ROOT / "crates" / "exiftool-formats" / "tests" / "testdata"
DEFAULT_PERL = Path(r"C:/projects/projects.rust.cg/vfx.ref/exiftool/exiftool")


def perl_exiftool() -> Path:
    env = os.environ.get("EXIFTOOL_PERL")
    if env:
        return Path(env)
    if DEFAULT_PERL.exists():
        return DEFAULT_PERL
    alt = ROOT / "_ref" / "exiftool" / "exiftool"
    if alt.exists():
        return alt
    sys.exit("Perl ExifTool not found; set EXIFTOOL_PERL")


def perl_filetype(tool: Path, image: Path) -> str:
    cmd = ["perl", str(tool), "-j", "-File:FileType", "-q", "-q", str(image)]
    out = subprocess.check_output(cmd, text=True, encoding="utf-8", errors="replace")
    data = json.loads(out)
    if not data:
        return ""
    return str(data[0].get("FileType") or data[0].get("File:FileType") or "")


def rust_filetype(image: Path) -> str:
    exe = ROOT / "target" / "debug" / "exif.exe"
    if not exe.exists():
        exe = ROOT / "target" / "debug" / "exif"
    cmd = [str(exe), "-f", "json", str(image)]
    out = subprocess.check_output(cmd, text=True, encoding="utf-8", errors="replace")
    data = json.loads(out)
    obj = data[0] if isinstance(data, list) else data
    for key in ("Format", "File:FileType", "FileType"):
        if key in obj:
            return str(obj[key])
    return ""


def main() -> int:
    tool = perl_exiftool()
    images = [p for p in TESTDATA.iterdir() if p.is_file() and p.suffix.lower() not in {".md", ".txt"}]
    if not images:
        print("no testdata images", file=sys.stderr)
        return 1
    subprocess.check_call(["cargo", "build", "-p", "exiftool-cli", "-q"], cwd=ROOT)
    failed = 0
    for image in sorted(images):
        try:
            perl_ft = perl_filetype(tool, image)
            rust_ft = rust_filetype(image)
        except subprocess.CalledProcessError as e:
            print(f"FAIL {image.name}: command {e}")
            failed += 1
            continue
        ok = perl_ft.lower() == rust_ft.lower() or (
            perl_ft and rust_ft and perl_ft.lower() in rust_ft.lower()
        )
        status = "OK" if ok else "MISMATCH"
        print(f"{status:8} {image.name:20} perl={perl_ft!r} rust={rust_ft!r}")
        if not ok:
            failed += 1
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())

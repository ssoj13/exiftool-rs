#!/usr/bin/env python3
"""Generate dicom_tags.rs from ExifTool DICOM.pm (SSOT)."""
from __future__ import annotations

import re
from pathlib import Path

SRC = Path(r"C:/projects/projects.rust.cg/vfx.ref/exiftool/lib/Image/ExifTool/DICOM.pm")
OUT = Path(r"C:/projects/projects.rust.cg/cglibs/exiftool-rs/crates/exiftool-formats/src/dicom_tags.rs")


def main() -> None:
    text = SRC.read_text(encoding="utf-8", errors="replace").split("sub ProcessDICOM")[0]
    pat = re.compile(
        r"'([0-9A-Fa-fx]{4}),([0-9A-Fa-fx]{4})'\s*=>\s*\{(.*?)\}",
        re.S,
    )
    name_pat = re.compile(r"Name\s*=>\s*'([^']+)'")
    tags: dict[int, str] = {}
    skipped = 0
    for m in pat.finditer(text):
        g, e, body = m.group(1), m.group(2), m.group(3)
        if any(c in g + e for c in "xX"):
            skipped += 1
            continue
        nm = name_pat.search(body)
        if not nm:
            skipped += 1
            continue
        key = (int(g, 16) << 16) | int(e, 16)
        tags[key] = nm.group(1)
    items = sorted(tags.items())
    lines = [
        "//! DICOM tag names from ExifTool `DICOM.pm` Main table (generated, do not edit).",
        "//! Source: vfx.ref/exiftool/lib/Image/ExifTool/DICOM.pm",
        "",
        "/// Lookup ExifTool tag name for a DICOM (group, element) pair.",
        "#[must_use]",
        "pub fn lookup(group: u16, element: u16) -> Option<&'static str> {",
        "    let key = ((group as u32) << 16) | u32::from(element);",
        "    TAGS.binary_search_by_key(&key, |t| t.0).ok().map(|i| TAGS[i].1)",
        "}",
        "",
        "const TAGS: &[(u32, &str)] = &[",
    ]
    for key, name in items:
        escaped = name.replace("\\", "\\\\").replace('"', '\\"')
        lines.append(f"    (0x{key:08X}, \"{escaped}\"),")
    lines.append("];")
    lines.append("")
    OUT.write_text("\n".join(lines), encoding="utf-8")
    print(f"wrote {len(items)} tags, skipped {skipped} -> {OUT}")


if __name__ == "__main__":
    main()

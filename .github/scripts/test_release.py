import hashlib
import os
from pathlib import Path
import tarfile
import tempfile
import unittest
from unittest.mock import patch
import zipfile

import release
from release import validate


class TagValidation(unittest.TestCase):
    def test_stable(self):
        self.assertFalse(validate("v1.2.3", "1.2.3"))

    def test_prerelease(self):
        self.assertTrue(validate("v1.2.3-rc.1", "1.2.3-rc.1"))
        self.assertTrue(validate("v0.1.0-beta", "0.1.0-beta"))

    def test_mismatched_version(self):
        with self.assertRaises(ValueError):
            validate("v1.2.3", "1.2.4")

    def test_reject_unsafe_or_malformed_tags(self):
        for tag in ("1.2.3", "v1.2", "v01.2.3", "v1.2.3-01", "v1.2.3-rc..1", "v1.2.3/other", "v1.2.3\n", "v1.2.3+build"):
            with self.subTest(tag=tag), self.assertRaises(ValueError):
                validate(tag, tag.removeprefix("v"))


class ReleaseAssets(unittest.TestCase):
    def test_archives_and_checksum_manifest(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            (root / "Cargo.toml").write_text('[workspace.package]\nversion = "1.2.3"\n')
            for name in ("README.md", "LICENSE", "LICENSE-ARTISTIC", "LICENSE-GPL"):
                (root / name).write_text(name)
            with patch.object(release, "ROOT", root):
                for target in release.TARGETS:
                    windows = "windows" in target
                    binary = "exif.exe" if windows else "exif"
                    source = root / "target" / target / "release" / binary
                    source.parent.mkdir(parents=True)
                    source.write_bytes(b"native binary fixture")
                    source.chmod(0o755)
                    release.package("v1.2.3", target)
                    prefix = f"exiftool-rs-v1.2.3-{target}"
                    if windows:
                        with zipfile.ZipFile(root / "dist" / f"{prefix}.zip") as archive:
                            self.assertEqual(archive.read(f"{prefix}/{binary}"), source.read_bytes())
                            self.assertIn(f"{prefix}/LICENSE", archive.namelist())
                    else:
                        with tarfile.open(root / "dist" / f"{prefix}.tar.gz") as archive:
                            self.assertEqual(archive.extractfile(f"{prefix}/{binary}").read(), source.read_bytes())
                            if os.name != "nt":
                                self.assertTrue(archive.getmember(f"{prefix}/{binary}").mode & 0o111)
                release.checksums(root / "dist", "v1.2.3")
                manifest = (root / "dist" / "SHA256SUMS").read_text().splitlines()
                self.assertEqual(len(manifest), 4)
                for line in manifest:
                    digest, name = line.split("  ")
                    self.assertEqual(digest, hashlib.sha256((root / "dist" / name).read_bytes()).hexdigest())

    def test_incomplete_assets_fail(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            (root / "Cargo.toml").write_text('[workspace.package]\nversion = "1.2.3"\n')
            (root / "dist").mkdir()
            with patch.object(release, "ROOT", root), self.assertRaises(ValueError):
                release.checksums(root / "dist", "v1.2.3")

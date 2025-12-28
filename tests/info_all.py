"""Test exiftool_py - print all metadata from all test files."""
import json
from pathlib import Path

import exiftool_py as exif

TEST_DIR = Path(__file__).parent
EXTENSIONS = (".jpg", ".jpeg", ".png", ".tiff", ".tif", ".heic", ".exr", ".hdr", ".raf", ".nef")


def main():
    print(f"exiftool_py v{exif.__version__}")
    print(f"Scanning: {TEST_DIR}")
    print("=" * 70)

    for path in sorted(TEST_DIR.iterdir()):
        if path.suffix.lower() not in EXTENSIONS:
            continue

        print(f"\n{'=' * 70}")
        print(f"FILE: {path.name}")
        print("=" * 70)

        try:
            img = exif.open(str(path))
            print(f"Format: {img.format}")
            print(f"Tags: {len(img)}")
            print()

            # Common properties
            print("--- Properties ---")
            print(f"  Make: {img.make}")
            print(f"  Model: {img.model}")
            print(f"  DateTime: {img.datetime}")
            print(f"  DateTimeOriginal: {img.datetime_original}")
            print(f"  ISO: {img.iso}")
            print(f"  FNumber: {img.fnumber}")
            print(f"  Exposure: {img.exposure_time}")
            print(f"  FocalLength: {img.focal_length}")
            print(f"  Width: {img.width}")
            print(f"  Height: {img.height}")
            print(f"  GPS: {img.gps}")
            print()

            # All tags
            print("--- All Tags ---")
            for tag, value in sorted(img.items()):
                # Truncate long values
                s = str(value)
                if len(s) > 60:
                    s = s[:57] + "..."
                print(f"  {tag}: {s}")

            # JSON output
            print()
            print("--- JSON ---")
            d = dict(img)
            # Convert non-serializable to strings
            for k, v in d.items():
                if isinstance(v, bytes):
                    d[k] = f"<{len(v)} bytes>"
                elif hasattr(v, 'as_tuple'):  # Rational
                    d[k] = str(v)
            print(json.dumps(d, indent=2, default=str)[:500] + "...")

        except exif.FormatError as e:
            print(f"ERROR: {e}")

    print("\n" + "=" * 70)
    print("Done!")


if __name__ == "__main__":
    main()

"""exiftool_py - Fast image metadata library.

High-performance EXIF/XMP/IPTC metadata parser built in Rust.

Supported formats:
    - JPEG, PNG, TIFF, DNG, WebP
    - HEIC, AVIF
    - Canon CR2, CR3
    - Nikon NEF
    - Sony ARW
    - Olympus ORF
    - Panasonic RW2
    - Pentax PEF
    - Fujifilm RAF
    - OpenEXR, Radiance HDR

Example:
    >>> import exiftool_py as exif
    >>> img = exif.open("photo.jpg")
    >>> print(img.make, img.model)
    Canon EOS R5
    >>> img.artist = "John Doe"
    >>> img.save()
"""

from exiftool_py._core import (
    # Main functions
    open,
    scan,
    scan_dir,
    # Classes
    Image,
    Rational,
    GPS,
    PyScanResult as ScanResult,
    # Exceptions
    ExifError,
    FormatError,
    WriteError,
    TagError,
)

__version__ = "0.1.0"
__all__ = [
    # Functions
    "open",
    "scan",
    "scan_dir",
    # Classes
    "Image",
    "Rational",
    "GPS",
    "ScanResult",
    # Exceptions
    "ExifError",
    "FormatError",
    "WriteError",
    "TagError",
]

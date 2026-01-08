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

import asyncio
from typing import List, Optional

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
    ValidationIssue,
    # Exceptions
    ExifError,
    FormatError,
    WriteError,
    TagError,
)

__version__ = "0.1.0"


# =============================================================================
# Async API
# =============================================================================

async def open_async(path: str) -> Image:
    """Open an image file asynchronously.

    Args:
        path: Path to the image file

    Returns:
        Image object with metadata

    Example:
        >>> img = await exif.open_async("photo.jpg")
        >>> print(img.make)
    """
    return await asyncio.to_thread(open, path)


async def scan_async(
    pattern: str,
    parallel: bool = True,
    ignore_errors: bool = True,
) -> ScanResult:
    """Scan files matching glob pattern asynchronously.

    Args:
        pattern: Glob pattern like "photos/**/*.jpg"
        parallel: Use parallel processing (default: True)
        ignore_errors: Skip files that fail to parse (default: True)

    Returns:
        ScanResult iterator over Image objects

    Example:
        >>> result = await exif.scan_async("photos/**/*.jpg")
        >>> for img in result:
        ...     print(img.path)
    """
    return await asyncio.to_thread(scan, pattern, parallel, ignore_errors)


async def scan_dir_async(
    directory: str,
    extensions: Optional[List[str]] = None,
    parallel: bool = True,
) -> ScanResult:
    """Scan single directory asynchronously.

    Args:
        directory: Directory path
        extensions: File extensions to scan (default: common image formats)
        parallel: Use parallel processing (default: True)

    Returns:
        ScanResult iterator over Image objects
    """
    return await asyncio.to_thread(scan_dir, directory, extensions, parallel)


__all__ = [
    # Sync functions
    "open",
    "scan",
    "scan_dir",
    # Async functions
    "open_async",
    "scan_async",
    "scan_dir_async",
    # Classes
    "Image",
    "Rational",
    "GPS",
    "ScanResult",
    "ValidationIssue",
    # Exceptions
    "ExifError",
    "FormatError",
    "WriteError",
    "TagError",
]

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
"""

from typing import Any, Iterator, Optional, Union
from os import PathLike

__version__: str

# Exceptions
class ExifError(Exception):
    """Base exception for all exif_tool_rs errors."""
    ...

class FormatError(ExifError):
    """Raised when file format cannot be parsed."""
    ...

class WriteError(ExifError):
    """Raised when metadata cannot be written."""
    ...

class TagError(ExifError):
    """Raised when tag is not found or invalid."""
    ...

# Classes
class Rational:
    """Rational number (numerator/denominator)."""
    
    @property
    def numerator(self) -> int: ...
    
    @property
    def denominator(self) -> int: ...
    
    @property
    def value(self) -> float: ...
    
    def as_tuple(self) -> tuple[int, int]: ...
    
    def __float__(self) -> float: ...
    def __int__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class GPS:
    """GPS coordinates with decimal and DMS access."""
    
    @property
    def latitude(self) -> Optional[float]:
        """Latitude in decimal degrees (negative = South)."""
        ...
    
    @property
    def longitude(self) -> Optional[float]:
        """Longitude in decimal degrees (negative = West)."""
        ...
    
    @property
    def altitude(self) -> Optional[float]:
        """Altitude in meters."""
        ...
    
    @property
    def latitude_dms(self) -> Optional[tuple[float, float, float]]:
        """Latitude as (degrees, minutes, seconds)."""
        ...
    
    @property
    def longitude_dms(self) -> Optional[tuple[float, float, float]]:
        """Longitude as (degrees, minutes, seconds)."""
        ...
    
    @property
    def latitude_ref(self) -> Optional[str]:
        """'N' or 'S'."""
        ...
    
    @property
    def longitude_ref(self) -> Optional[str]:
        """'E' or 'W'."""
        ...
    
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class Image:
    """Image with metadata access."""
    
    # File info
    @property
    def path(self) -> str: ...
    
    @property
    def format(self) -> str: ...
    
    @property
    def is_camera_raw(self) -> bool:
        """True if this is a camera RAW file (read-only)."""
        ...
    
    @property
    def is_writable(self) -> bool:
        """True if format supports writing (JPEG, PNG, TIFF, DNG, EXR, HDR)."""
        ...
    
    # Common EXIF properties
    @property
    def make(self) -> Optional[str]: ...
    @make.setter
    def make(self, value: str) -> None: ...
    
    @property
    def model(self) -> Optional[str]: ...
    @model.setter
    def model(self, value: str) -> None: ...
    
    @property
    def artist(self) -> Optional[str]: ...
    @artist.setter
    def artist(self, value: str) -> None: ...
    
    @property
    def copyright(self) -> Optional[str]: ...
    @copyright.setter
    def copyright(self, value: str) -> None: ...
    
    @property
    def software(self) -> Optional[str]: ...
    @software.setter
    def software(self, value: str) -> None: ...
    
    @property
    def datetime(self) -> Optional[str]: ...
    @datetime.setter
    def datetime(self, value: str) -> None: ...
    
    @property
    def datetime_original(self) -> Optional[str]: ...
    @datetime_original.setter
    def datetime_original(self, value: str) -> None: ...
    
    @property
    def description(self) -> Optional[str]: ...
    @description.setter
    def description(self, value: str) -> None: ...
    
    # Camera settings
    @property
    def iso(self) -> Optional[int]: ...
    
    @property
    def exposure_time(self) -> Optional[Rational]: ...
    
    @property
    def fnumber(self) -> Optional[Rational]: ...
    
    @property
    def focal_length(self) -> Optional[Rational]: ...
    
    @property
    def focal_length_35mm(self) -> Optional[int]: ...
    
    # Image dimensions
    @property
    def width(self) -> Optional[int]: ...
    
    @property
    def height(self) -> Optional[int]: ...
    
    @property
    def orientation(self) -> Optional[int]: ...
    @orientation.setter
    def orientation(self, value: int) -> None: ...
    
    # GPS
    @property
    def gps(self) -> GPS: ...
    
    # Dict-like access
    def __getitem__(self, key: str) -> Any: ...
    def __setitem__(self, key: str, value: Any) -> None: ...
    def __delitem__(self, key: str) -> None: ...
    def __contains__(self, key: str) -> bool: ...
    def __iter__(self) -> Iterator[str]: ...
    def __len__(self) -> int: ...
    
    def keys(self) -> list[str]: ...
    def values(self) -> list[Any]: ...
    def items(self) -> list[tuple[str, Any]]: ...
    def get(self, key: str, default: Any = None) -> Any: ...
    def to_dict(self) -> dict[str, Any]: ...
    
    # Modification
    def save(self, path: Optional[str] = None) -> None:
        """Save changes to file."""
        ...
    
    # Context manager
    def __enter__(self) -> "Image": ...
    def __exit__(self, exc_type: Any, exc_val: Any, exc_tb: Any) -> None: ...
    
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class ScanResult:
    """Iterator over scanned images."""
    
    @property
    def count(self) -> int:
        """Number of successfully parsed files."""
        ...
    
    def to_list(self) -> list[Image]: ...
    
    def __iter__(self) -> Iterator[Image]: ...
    def __next__(self) -> Image: ...
    def __len__(self) -> int: ...

# Functions
def open(path: Union[str, PathLike[str]]) -> Image:
    """Open an image file and parse its metadata.
    
    Args:
        path: Path to the image file
        
    Returns:
        Image object with metadata
        
    Raises:
        FormatError: If the file cannot be parsed
    """
    ...

def scan(
    pattern: str,
    parallel: bool = True,
    ignore_errors: bool = True,
) -> ScanResult:
    """Scan files matching glob pattern.
    
    Args:
        pattern: Glob pattern like "photos/**/*.jpg"
        parallel: Use parallel processing (default: True)
        ignore_errors: Skip files that fail to parse (default: True)
        
    Returns:
        Iterator of Image objects
        
    Example:
        >>> for img in exif.scan("photos/**/*.jpg"):
        ...     print(img.make, img.model)
    """
    ...

def scan_dir(
    directory: str,
    extensions: Optional[list[str]] = None,
    parallel: bool = True,
) -> ScanResult:
    """Scan single directory for image files.
    
    Args:
        directory: Directory path
        extensions: File extensions to scan (default: common image formats)
        parallel: Use parallel processing (default: True)
        
    Returns:
        Iterator of Image objects
    """
    ...

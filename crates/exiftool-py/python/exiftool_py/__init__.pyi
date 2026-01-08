"""exiftool_py - Fast image metadata library.

High-performance EXIF/XMP/IPTC metadata parser built in Rust.

Supported formats:
    Images: JPEG, PNG, TIFF, GIF, BMP, ICO, TGA, PCX, SGI, PNM, SVG, EPS, AI
    Modern: HEIC, AVIF, WebP, JXL, JP2, EXR, HDR, DPX
    RAW: CR2, CR3, CRW, NEF, NRW, ARW, SRF, ORF, RW2, RWL, PEF, RAF, SRW, ERF,
         MEF, MRW, MOS, X3F, IIQ, DCR, BRAW, FFF, R3D, DNG
    Video: MP4, MKV, AVI, FLV, MXF, RM, MPEG-TS, ASF
    Audio: MP3, FLAC, WAV, AIFF, OGG, AAC, ALAC, APE, WavPack, DSF, TAK, MIDI
"""

from typing import Any, Coroutine, Iterator, Optional, Union, List, Tuple, Dict
from os import PathLike

__version__: str

# =============================================================================
# Exceptions
# =============================================================================

class ExifError(Exception):
    """Base exception for all exiftool_py errors."""
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


# =============================================================================
# Data Classes
# =============================================================================

class Rational:
    """Rational number (numerator/denominator) for EXIF values."""
    
    @property
    def numerator(self) -> int:
        """Numerator of the rational."""
        ...
    
    @property
    def denominator(self) -> int:
        """Denominator of the rational."""
        ...
    
    @property
    def value(self) -> float:
        """Decimal value (numerator / denominator)."""
        ...
    
    def as_tuple(self) -> Tuple[int, int]:
        """Return as (numerator, denominator) tuple."""
        ...
    
    def __float__(self) -> float: ...
    def __int__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...


class GPS:
    """GPS coordinates with decimal and DMS (degrees/minutes/seconds) access."""
    
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
        """Altitude in meters (negative = below sea level)."""
        ...
    
    @property
    def latitude_dms(self) -> Optional[Tuple[float, float, float]]:
        """Latitude as (degrees, minutes, seconds) tuple."""
        ...
    
    @property
    def longitude_dms(self) -> Optional[Tuple[float, float, float]]:
        """Longitude as (degrees, minutes, seconds) tuple."""
        ...
    
    @property
    def latitude_ref(self) -> Optional[str]:
        """Latitude reference: 'N' (North) or 'S' (South)."""
        ...
    
    @property
    def longitude_ref(self) -> Optional[str]:
        """Longitude reference: 'E' (East) or 'W' (West)."""
        ...
    
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...


class PageInfo:
    """Information about a page in multi-page files (TIFF, etc.)."""
    
    @property
    def index(self) -> int:
        """Page index (0-based)."""
        ...
    
    @property
    def width(self) -> int:
        """Image width in pixels."""
        ...
    
    @property
    def height(self) -> int:
        """Image height in pixels."""
        ...
    
    @property
    def bits_per_sample(self) -> int:
        """Bits per sample (color depth)."""
        ...
    
    @property
    def compression(self) -> int:
        """TIFF compression type."""
        ...
    
    @property
    def subfile_type(self) -> int:
        """TIFF subfile type flags."""
        ...
    
    @property
    def is_thumbnail(self) -> bool:
        """True if this page is a thumbnail/reduced resolution image."""
        ...
    
    @property
    def is_page(self) -> bool:
        """True if this is a page of a multi-page document."""
        ...
    
    def __repr__(self) -> str: ...


class TrackPoint:
    """Single point in a GPX track."""
    
    @property
    def latitude(self) -> float:
        """Latitude in decimal degrees."""
        ...
    
    @property
    def longitude(self) -> float:
        """Longitude in decimal degrees."""
        ...
    
    @property
    def elevation(self) -> Optional[float]:
        """Elevation in meters."""
        ...
    
    @property
    def timestamp(self) -> int:
        """Unix timestamp (seconds since epoch)."""
        ...
    
    def __repr__(self) -> str: ...


class GpxTrack:
    """GPX track for geotagging photos."""
    
    @staticmethod
    def from_file(path: str) -> "GpxTrack":
        """Load GPX track from file.
        
        Args:
            path: Path to .gpx file
            
        Returns:
            GpxTrack object
        """
        ...
    
    @property
    def points(self) -> List[TrackPoint]:
        """All track points."""
        ...
    
    @property
    def point_count(self) -> int:
        """Number of track points."""
        ...
    
    def find_position(self, timestamp: int) -> Optional[Tuple[float, float, Optional[float]]]:
        """Find interpolated position at timestamp.
        
        Args:
            timestamp: Unix timestamp
            
        Returns:
            Tuple of (latitude, longitude, elevation) or None if out of range
        """
        ...
    
    def __len__(self) -> int: ...
    def __repr__(self) -> str: ...


class ValidationIssue:
    """Validation issue found in metadata."""

    @property
    def tag(self) -> str:
        """Tag name with the issue."""
        ...

    @property
    def message(self) -> str:
        """Description of the problem."""
        ...

    @property
    def severity(self) -> str:
        """Severity: 'error' or 'warning'."""
        ...

    def __repr__(self) -> str: ...


class ScanError:
    """Error info for a file that failed to parse during scanning."""

    @property
    def path(self) -> str:
        """Path to the file that failed."""
        ...
    
    @property
    def error(self) -> str:
        """Error message."""
        ...
    
    def __repr__(self) -> str: ...


class ScanResult:
    """Result of scanning files - iterable over successfully parsed images."""
    
    @property
    def count(self) -> int:
        """Number of successfully parsed files."""
        ...
    
    @property
    def errors(self) -> List[ScanError]:
        """List of errors from failed files."""
        ...
    
    @property
    def error_count(self) -> int:
        """Number of files that failed to parse."""
        ...
    
    def to_list(self) -> List["Image"]:
        """Convert to list of images."""
        ...
    
    def __iter__(self) -> Iterator["Image"]: ...
    def __next__(self) -> "Image": ...
    def __len__(self) -> int: ...


# =============================================================================
# Main Image Class
# =============================================================================

class Image:
    """Image with metadata access.
    
    Provides dict-like access to EXIF tags and properties for common fields.
    
    Example:
        >>> img = exiftool_py.open("photo.jpg")
        >>> print(img.make, img.model)
        >>> print(img["ISO"])
        >>> img["Artist"] = "John Doe"
        >>> img.save()
    """
    
    # -------------------------------------------------------------------------
    # File Information
    # -------------------------------------------------------------------------
    
    @property
    def path(self) -> Optional[str]:
        """File path (None if created from bytes)."""
        ...
    
    @property
    def format(self) -> str:
        """File format (JPEG, PNG, TIFF, etc.)."""
        ...
    
    @property
    def is_camera_raw(self) -> bool:
        """True if this is a camera RAW file (typically read-only)."""
        ...
    
    @property
    def is_writable(self) -> bool:
        """True if format supports writing (JPEG, PNG, TIFF, DNG, WebP, HEIC, EXR, HDR)."""
        ...
    
    @property
    def exif_offset(self) -> Optional[int]:
        """Raw EXIF data offset in file (if available)."""
        ...
    
    # -------------------------------------------------------------------------
    # Multi-page Support
    # -------------------------------------------------------------------------
    
    @property
    def page_count(self) -> int:
        """Number of pages/frames in the file."""
        ...
    
    @property
    def is_multi_page(self) -> bool:
        """True if file has multiple pages (multi-page TIFF, etc.)."""
        ...
    
    @property
    def pages(self) -> List[PageInfo]:
        """Page info for multi-page files."""
        ...
    
    def get_page(self, index: int) -> Optional[PageInfo]:
        """
        Get page info by index.
        
        Args:
            index: Page index (0-based)
            
        Returns:
            PageInfo for the specified page, or None if index out of range
        """
        ...
    
    # -------------------------------------------------------------------------
    # Common EXIF Properties (Read/Write)
    # -------------------------------------------------------------------------
    
    @property
    def make(self) -> Optional[str]:
        """Camera manufacturer."""
        ...
    @make.setter
    def make(self, value: str) -> None: ...
    
    @property
    def model(self) -> Optional[str]:
        """Camera model."""
        ...
    @model.setter
    def model(self, value: str) -> None: ...
    
    @property
    def artist(self) -> Optional[str]:
        """Artist/author."""
        ...
    @artist.setter
    def artist(self, value: str) -> None: ...
    
    @property
    def copyright(self) -> Optional[str]:
        """Copyright notice."""
        ...
    @copyright.setter
    def copyright(self, value: str) -> None: ...
    
    @property
    def software(self) -> Optional[str]:
        """Software used to create/process the image."""
        ...
    @software.setter
    def software(self, value: str) -> None: ...
    
    @property
    def description(self) -> Optional[str]:
        """Image description."""
        ...
    @description.setter
    def description(self, value: str) -> None: ...
    
    # -------------------------------------------------------------------------
    # Date/Time Properties
    # -------------------------------------------------------------------------
    
    @property
    def date_time_original(self) -> Optional[str]:
        """Date/time when photo was taken (YYYY:MM:DD HH:MM:SS)."""
        ...
    
    # -------------------------------------------------------------------------
    # Camera Settings (Read-only)
    # -------------------------------------------------------------------------
    
    @property
    def iso(self) -> Optional[int]:
        """ISO sensitivity."""
        ...
    
    @property
    def exposure_time(self) -> Optional[Rational]:
        """Exposure time in seconds (e.g., 1/125)."""
        ...
    
    @property
    def fnumber(self) -> Optional[Rational]:
        """F-number (aperture)."""
        ...
    
    @property
    def focal_length(self) -> Optional[Rational]:
        """Focal length in mm."""
        ...
    
    @property
    def focal_length_35mm(self) -> Optional[int]:
        """Focal length equivalent in 35mm format."""
        ...
    
    # -------------------------------------------------------------------------
    # Image Dimensions
    # -------------------------------------------------------------------------
    
    @property
    def width(self) -> Optional[int]:
        """Image width in pixels."""
        ...
    
    @property
    def height(self) -> Optional[int]:
        """Image height in pixels."""
        ...
    
    @property
    def orientation(self) -> Optional[int]:
        """EXIF orientation (1-8)."""
        ...
    
    # -------------------------------------------------------------------------
    # GPS
    # -------------------------------------------------------------------------
    
    @property
    def gps(self) -> Optional[GPS]:
        """GPS coordinates (if available)."""
        ...
    
    # -------------------------------------------------------------------------
    # Raw Data Access
    # -------------------------------------------------------------------------
    
    @property
    def xmp(self) -> Optional[str]:
        """Raw XMP XML data."""
        ...
    
    @property
    def thumbnail(self) -> Optional[bytes]:
        """Thumbnail image bytes (small preview, typically 160x120)."""
        ...
    
    @property
    def preview(self) -> Optional[bytes]:
        """Preview image bytes (larger preview from RAW files)."""
        ...
    
    @property
    def icc(self) -> Optional[bytes]:
        """ICC color profile bytes."""
        ...
    @icc.setter
    def icc(self, data: Optional[bytes]) -> None: ...
    
    # -------------------------------------------------------------------------
    # Dict-like Access
    # -------------------------------------------------------------------------
    
    def __getitem__(self, key: str) -> Any:
        """Get tag value by name.
        
        Raises:
            KeyError: If tag not found
        """
        ...
    
    def __setitem__(self, key: str, value: Any) -> None:
        """Set tag value."""
        ...
    
    def __delitem__(self, key: str) -> None:
        """Delete tag.
        
        Raises:
            KeyError: If tag not found
        """
        ...
    
    def __contains__(self, key: str) -> bool:
        """Check if tag exists."""
        ...
    
    def __iter__(self) -> Iterator[str]:
        """Iterate over tag names."""
        ...
    
    def __len__(self) -> int:
        """Number of tags."""
        ...
    
    def keys(self) -> List[str]:
        """Get all tag names."""
        ...
    
    def values(self) -> List[Any]:
        """Get all tag values."""
        ...
    
    def items(self) -> List[Tuple[str, Any]]:
        """Get all (name, value) pairs."""
        ...
    
    def get(self, key: str, default: Any = None) -> Any:
        """Get tag value with default.
        
        Args:
            key: Tag name
            default: Value to return if tag not found
            
        Returns:
            Tag value or default
        """
        ...
    
    def clear(self) -> None:
        """Remove all EXIF tags."""
        ...
    
    def strip_metadata(self) -> None:
        """Remove all metadata (EXIF, XMP, IPTC, ICC, thumbnails).
        
        Use this to strip all metadata from an image before sharing.
        After calling this, call save() to write changes.
        
        Example:
            >>> img.strip_metadata()
            >>> img.save()  # Writes file without metadata
        """
        ...
    
    def copy_tags(self, source: "Image", tags: Optional[List[str]] = None) -> None:
        """Copy tags from another image.
        
        Args:
            source: Source image to copy tags from
            tags: Optional list of tag names to copy. If None, copies all tags.
            
        Example:
            >>> dst.copy_tags(src)  # Copy all tags
            >>> dst.copy_tags(src, ["Make", "Model", "DateTimeOriginal"])
        """
        ...
    
    # -------------------------------------------------------------------------
    # Value Interpretation
    # -------------------------------------------------------------------------
    
    def get_interpreted(self, key: str) -> Optional[str]:
        """Get human-readable interpretation of a tag value.
        
        Example:
            >>> img["Orientation"]  # Returns: 6
            >>> img.get_interpreted("Orientation")  # Returns: "Rotate 90 CW"
        """
        ...
    
    def get_display(self, key: str) -> Optional[str]:
        """Get formatted display value with units.
        
        Example:
            >>> img.get_display("FocalLength")  # Returns: "50.0 mm"
        """
        ...
    
    # -------------------------------------------------------------------------
    # Conversion
    # -------------------------------------------------------------------------
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert all metadata to dictionary."""
        ...
    
    @staticmethod
    def from_bytes(data: bytes) -> "Image":
        """Create Image from raw file bytes.
        
        Args:
            data: Raw file bytes
            
        Returns:
            Image object with parsed metadata
        """
        ...
    
    # -------------------------------------------------------------------------
    # Modification Methods
    # -------------------------------------------------------------------------
    
    def save(self, path: Optional[str] = None) -> None:
        """Save metadata changes to file.
        
        Args:
            path: Output path. If None, overwrites original file.
            
        Raises:
            WriteError: If format doesn't support writing or write fails
        """
        ...
    
    def shift_time(self, offset: str) -> None:
        """Shift all DateTime tags by offset.
        
        Args:
            offset: Offset string like "+2:30" (hours:minutes) or "-30" (minutes)
            
        Example:
            >>> img.shift_time("+2:00")  # Add 2 hours
            >>> img.shift_time("-30")    # Subtract 30 minutes
        """
        ...
    
    def set_gps(self, lat: float, lon: float, alt: Optional[float] = None) -> None:
        """Set GPS coordinates directly.
        
        Args:
            lat: Latitude in decimal degrees (negative = South)
            lon: Longitude in decimal degrees (negative = West)
            alt: Optional altitude in meters (negative = below sea level)
            
        Example:
            >>> img.set_gps(55.7558, 37.6173)  # Moscow
            >>> img.set_gps(51.5074, -0.1278, 11.0)  # London with altitude
        """
        ...
    
    def geotag(self, gpx_path: str) -> Optional[Tuple[float, float]]:
        """Add GPS coordinates from a GPX track file.
        
        Matches photo timestamp (DateTimeOriginal) to track points.
        
        Args:
            gpx_path: Path to .gpx file
            
        Returns:
            Tuple of (latitude, longitude) if matched, None if no match
        """
        ...
    
    def set_icc_from_file(self, path: str) -> None:
        """Load and embed ICC color profile from file.
        
        Args:
            path: Path to .icc or .icm profile file
        """
        ...
    
    def add_composite(self) -> None:
        """Calculate and add composite tags.
        
        Adds derived tags like:
        - ImageSize: "4000x3000"
        - Megapixels: 12.0
        - GPSPosition: Combined lat/lon
        """
        ...

    def validate(self) -> List[ValidationIssue]:
        """Validate metadata for common issues.

        Checks:
        - GPS coordinates in valid range (-90/90 lat, -180/180 lon)
        - Orientation value (1-8)
        - ISO reasonable range (1-10000000)
        - DateTime format validity
        - Dimensions > 0
        - ExposureTime > 0
        - FNumber > 0

        Returns:
            List of ValidationIssue objects describing problems found.
            Empty list if no issues detected.

        Example:
            >>> issues = img.validate()
            >>> for issue in issues:
            ...     print(f"{issue.tag}: {issue.message}")
        """
        ...

    # -------------------------------------------------------------------------
    # XMP Sidecar Support
    # -------------------------------------------------------------------------

    def has_sidecar(self) -> bool:
        """Check if XMP sidecar file exists for this image.

        Sidecar files have the same name but .xmp extension: photo.jpg -> photo.xmp

        Returns:
            True if sidecar file exists.
        """
        ...

    def sidecar_path(self) -> Optional[str]:
        """Get sidecar file path for this image.

        Returns:
            Path to sidecar file (may not exist), or None if image has no path.
        """
        ...

    def load_sidecar(self) -> bool:
        """Load XMP metadata from sidecar file and merge into current metadata.

        Sidecar values override embedded values.

        Returns:
            True if sidecar was loaded, False if no sidecar exists.

        Example:
            >>> if img.load_sidecar():
            ...     print("Loaded sidecar metadata")
        """
        ...

    def save_sidecar(self, path: Optional[str] = None) -> None:
        """Save current XMP metadata to sidecar file.

        Creates or overwrites the .xmp file next to the image.

        Args:
            path: Optional explicit path for sidecar. If None, uses image.xmp.

        Example:
            >>> img.save_sidecar()  # Creates photo.xmp next to photo.jpg
        """
        ...

    # -------------------------------------------------------------------------
    # Context Manager
    # -------------------------------------------------------------------------

    def __enter__(self) -> "Image": ...
    def __exit__(self, exc_type: Any, exc_val: Any, exc_tb: Any) -> bool: ...
    
    # -------------------------------------------------------------------------
    # String Representation
    # -------------------------------------------------------------------------
    
    def __str__(self) -> str:
        """Pretty print all tags."""
        ...
    
    def __repr__(self) -> str: ...


# =============================================================================
# Module Functions
# =============================================================================

def open(path: Union[str, PathLike[str]]) -> Image:
    """Open an image file and parse its metadata.
    
    Args:
        path: Path to the image file
        
    Returns:
        Image object with metadata
        
    Raises:
        FormatError: If the file format is not supported or cannot be parsed
        
    Example:
        >>> img = exiftool_py.open("photo.jpg")
        >>> print(img.make, img.model)
        Canon EOS R5
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
        parallel: Use parallel processing with rayon (default: True)
        ignore_errors: Skip files that fail to parse (default: True)
        
    Returns:
        ScanResult iterator over Image objects
        
    Example:
        >>> result = exiftool_py.scan("photos/**/*.jpg")
        >>> for img in result:
        ...     print(img.make, img.model)
        >>> print(f"Errors: {result.error_count}")
    """
    ...


def scan_dir(
    directory: str,
    extensions: Optional[List[str]] = None,
    parallel: bool = True,
) -> ScanResult:
    """Scan single directory for image files.
    
    Args:
        directory: Directory path
        extensions: File extensions to scan (default: jpg, jpeg, png, tiff, tif, 
                   heic, cr2, cr3, nef, arw, dng)
        parallel: Use parallel processing (default: True)
        
    Returns:
        ScanResult iterator over Image objects
        
    Example:
        >>> for img in exiftool_py.scan_dir("./photos"):
        ...     print(img.path)
    """
    ...


# =============================================================================
# Async API
# =============================================================================

async def open_async(path: Union[str, PathLike[str]]) -> Image:
    """Open an image file asynchronously.
    
    Args:
        path: Path to the image file
        
    Returns:
        Image object with metadata
        
    Example:
        >>> img = await exiftool_py.open_async("photo.jpg")
        >>> print(img.make)
    """
    ...


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
        >>> result = await exiftool_py.scan_async("photos/**/*.jpg")
        >>> for img in result:
        ...     print(img.path)
    """
    ...


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
        
    Example:
        >>> result = await exiftool_py.scan_dir_async("./photos")
        >>> for img in result:
        ...     print(img.path)
    """
    ...

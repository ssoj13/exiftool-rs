# Performance and resource use

Native parsing avoids starting a metadata subprocess. Actual latency and memory
use depend on file size, container structure, preview extraction, storage,
registry setup, and the tags requested. This guide describes measurement and
resource behavior rather than promising a fixed speedup over ExifTool.

## Memory behavior

The API accepts `Read + Seek`, but some parsers and writers buffer entire files.
`read_with_limit` uses `MAX_FILE_SIZE`, currently **100 MiB**. Such operations
can reject larger inputs with `Error::FileTooLarge`; a recognized format does
not guarantee that every file size is supported.

Thumbnails, previews, profiles, and parsed values are owned allocations.
Ignoring a returned preview does not undo the work or allocation used to extract
it. Batch scans in Python collect parsed images and errors before returning a
`ScanResult`; iteration does not make that scan a lazy file stream.

## Reuse the registry

Construct a registry once for a sequential batch. Drop each result after
processing if you do not need to retain its payloads.

```rust
use exiftool_formats::FormatRegistry;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = FormatRegistry::new();
    for file in std::env::args().skip(1) {
        let metadata = registry.parse_file(Path::new(&file))?;
        println!("{file}: {} tags", metadata.exif.len());
    }
    Ok(())
}
```

Parallel scans can improve throughput, but multiply active parser allocations
and increase I/O contention. Measure representative batches before increasing
concurrency.

## Run the included benchmarks

```bash
cargo bench -p exiftool-formats --bench parser_bench
```

The Criterion harness includes small in-memory parser inputs and registry
measurements. It does not establish end-to-end throughput for real camera files
or equivalence with ExifTool's full extraction behavior.

For comparisons, record the tool versions, command options, file corpus,
hardware, warm/cold cache conditions, elapsed time, and peak memory. Distinguish
ExifTool process startup from a persistent/batch invocation, and compare the
same metadata operations. Keep measured results in benchmark artifacts or
release discussions rather than presenting estimated timings as a guarantee.

See [architecture](architecture.md) and [reading](reading.md).

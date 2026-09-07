# Golden testdata

Copies of small files from ExifTool `t/images` (Phil Harvey, same license as the `_ref/exiftool` tree). Used by `cargo test -p exiftool-formats --test golden` and `python xtask/parity.py`.

Extra fixtures (not golden — files >1MB are skipped by the golden glob):

- `RAW_SONY_A100.ARW` — Sony DSLR-A100 from [rawsamples.ch](https://rawsamples.ch/index.php/en/sony) (`http://www.rawsamples.ch/raws/sony/a100/RAW_SONY_A100.ARW`). Developer RAW sample (CC-BY-NC-SA on that site). Used for ExifTool `FinishARW` (Minolta MRW trailer + `A100DataOffset`).

Regenerate expected JSON:

```
set UPDATE_GOLDEN=1
cargo test -p exiftool-formats --test golden
```

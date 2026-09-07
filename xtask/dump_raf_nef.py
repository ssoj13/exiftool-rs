from pathlib import Path
import struct

base = Path(r"C:/projects/projects.rust.cg/cglibs/exiftool-rs/crates/exiftool-formats/tests/testdata")

raf = (base / "FujiFilm.raf").read_bytes()
print("RAF len", len(raf))
print("magic", raf[:16])
print("ver", raf[0x3C:0x40])
for off in (0x48, 0x4C, 0x54, 0x58, 0x5C, 0x60, 0x64, 0x68, 0x78, 0x7C, 0x80, 0x84):
    v = struct.unpack_from(">I", raf, off)[0]
    print(f"  {off:#06x} {v} {v:#x}")
jp, jl = struct.unpack_from(">II", raf, 0x54)
print("jpeg", hex(jp), jl, "end", hex(jp + jl), "soi", raf[jp : jp + 2])
nextp = struct.unpack_from(">I", raf, 0x5C)[0]
print("nextPtr/pad", hex(nextp), nextp - (jp + jl))

nef = (base / "Nikon.nef").read_bytes()
print("\nNEF len", len(nef), "order", nef[:2], "magic", struct.unpack_from("<H", nef, 2)[0])
ifd0 = struct.unpack_from("<I", nef, 4)[0]
n = struct.unpack_from("<H", nef, ifd0)[0]
print("IFD0 at", ifd0, "entries", n)
for i in range(n):
    p = ifd0 + 2 + i * 12
    tag, typ, cnt, val = struct.unpack_from("<HHI I", nef, p)
    print(f"  tag {tag:#06x} type {typ} count {cnt} val {val}")

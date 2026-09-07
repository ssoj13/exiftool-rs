from pathlib import Path
import struct

nef = Path(r"C:/projects/projects.rust.cg/cglibs/exiftool-rs/crates/exiftool-formats/tests/testdata/Nikon.nef").read_bytes()

def u16(p):
    return struct.unpack_from("<H", nef, p)[0]
def u32(p):
    return struct.unpack_from("<I", nef, p)[0]

def dump_ifd(off, name):
    n = u16(off)
    print(f"\n{name} @ {off} entries {n}")
    for i in range(n):
        p = off + 2 + i * 12
        tag, typ, cnt, val = struct.unpack_from("<HHII", nef, p)
        print(f"  {tag:#06x} type={typ} count={cnt} val={val}")
    nxt = u32(off + 2 + n * 12)
    print(f"  next={nxt}")
    return nxt

ifd0 = u32(4)
nxt = dump_ifd(ifd0, "IFD0")
# SubIFDs at 0x014a count 2 offset 414
sub_off = 414
s0 = u32(sub_off)
s1 = u32(sub_off + 4)
print("subifd offsets", s0, s1)
dump_ifd(s0, "SubIFD0")
dump_ifd(s1, "SubIFD1")
dump_ifd(1394, "ExifIFD")

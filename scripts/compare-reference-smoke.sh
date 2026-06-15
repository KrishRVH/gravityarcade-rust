#!/usr/bin/env bash
set -euo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$ROOT"

if [[ "${1:-}" == "--capture" ]]; then
    scripts/capture-reference-swf.sh
    scripts/capture-native-smoke.sh
elif [[ "${1:-}" != "" ]]; then
    echo "usage: $0 [--capture]" >&2
    exit 1
fi

python3 - "$ROOT" <<'PY'
import struct
import sys
import zlib
from dataclasses import dataclass
from pathlib import Path

root = Path(sys.argv[1])


@dataclass(frozen=True)
class Threshold:
    changed_pct: float
    mean_abs: float
    big80_pct: float


THRESHOLDS = {
    "menu": Threshold(changed_pct=8.0, mean_abs=5.5, big80_pct=2.5),
    "menu-polarisation-opposite": Threshold(changed_pct=8.0, mean_abs=5.5, big80_pct=2.5),
    "menu-polarisation-same": Threshold(changed_pct=8.0, mean_abs=5.5, big80_pct=2.5),
    "menu-polarisation-all": Threshold(changed_pct=8.0, mean_abs=5.5, big80_pct=2.5),
    "menu-matches-5": Threshold(changed_pct=8.0, mean_abs=5.5, big80_pct=2.5),
    "menu-matches-7": Threshold(changed_pct=8.0, mean_abs=5.5, big80_pct=2.5),
    "menu-matches-1": Threshold(changed_pct=8.0, mean_abs=5.5, big80_pct=2.5),
    "menu-gravity-low": Threshold(changed_pct=8.0, mean_abs=5.5, big80_pct=2.5),
    "menu-gravity-high": Threshold(changed_pct=8.0, mean_abs=5.5, big80_pct=2.5),
    "menu-gravity-very-high": Threshold(changed_pct=8.0, mean_abs=5.5, big80_pct=2.5),
    "menu-gravity-black-hole": Threshold(changed_pct=8.0, mean_abs=5.5, big80_pct=2.5),
    "menu-speed-fast": Threshold(changed_pct=8.0, mean_abs=5.5, big80_pct=2.5),
    "menu-after-help": Threshold(changed_pct=8.0, mean_abs=5.5, big80_pct=2.5),
    "help": Threshold(changed_pct=16.0, mean_abs=12.0, big80_pct=6.0),
    "playing-idle": Threshold(changed_pct=5.0, mean_abs=3.5, big80_pct=1.2),
    "playing": Threshold(changed_pct=6.0, mean_abs=4.5, big80_pct=2.5),
    "playing-red": Threshold(changed_pct=6.0, mean_abs=4.5, big80_pct=2.5),
    "round-intro-1": Threshold(changed_pct=10.0, mean_abs=8.0, big80_pct=4.0),
}


def paeth(a: int, b: int, c: int) -> int:
    p = a + b - c
    pa = abs(p - a)
    pb = abs(p - b)
    pc = abs(p - c)
    if pa <= pb and pa <= pc:
        return a
    if pb <= pc:
        return b
    return c


def read_png(path: Path) -> tuple[int, int, int, list[bytes]]:
    data = path.read_bytes()
    if data[:8] != b"\x89PNG\r\n\x1a\n":
        raise SystemExit(f"{path}: not a PNG")

    offset = 8
    width = height = color_type = None
    idat = bytearray()
    while offset < len(data):
        length = struct.unpack(">I", data[offset : offset + 4])[0]
        kind = data[offset + 4 : offset + 8]
        body = data[offset + 8 : offset + 8 + length]
        offset += 12 + length
        if kind == b"IHDR":
            width, height, bit_depth, color_type = struct.unpack(">IIBB", body[:10])
            if bit_depth != 8 or color_type not in (2, 6):
                raise SystemExit(f"{path}: unsupported PNG format {bit_depth=} {color_type=}")
        elif kind == b"IDAT":
            idat.extend(body)
        elif kind == b"IEND":
            break

    if width != 550 or height != 400:
        raise SystemExit(f"{path}: expected 550x400, got {width}x{height}")
    channels = 4 if color_type == 6 else 3
    stride = width * channels
    raw = zlib.decompress(bytes(idat))
    rows: list[bytes] = []
    previous = bytearray(stride)
    pos = 0
    for _ in range(height):
        filter_type = raw[pos]
        pos += 1
        row = bytearray(raw[pos : pos + stride])
        pos += stride
        for i, value in enumerate(row):
            left = row[i - channels] if i >= channels else 0
            up = previous[i]
            upper_left = previous[i - channels] if i >= channels else 0
            match filter_type:
                case 0:
                    decoded = value
                case 1:
                    decoded = value + left
                case 2:
                    decoded = value + up
                case 3:
                    decoded = value + ((left + up) // 2)
                case 4:
                    decoded = value + paeth(left, up, upper_left)
                case _:
                    raise SystemExit(f"{path}: unsupported PNG filter {filter_type}")
            row[i] = decoded & 0xFF
        rows.append(bytes(row))
        previous = row
    return width, height, channels, rows


def compare(name: str) -> None:
    reference = root / "target" / "reference-swf" / f"{name}.png"
    native = root / "target" / "debug" / "visual-smoke" / f"{name}.png"
    if not reference.exists() or not native.exists():
        raise SystemExit(
            f"missing captures for {name}; run scripts/compare-reference-smoke.sh --capture"
        )

    width, height, ref_channels, ref_rows = read_png(reference)
    native_width, native_height, native_channels, native_rows = read_png(native)
    if (width, height) != (native_width, native_height):
        raise SystemExit(f"{name}: size mismatch {width}x{height} vs {native_width}x{native_height}")

    total = width * height
    changed = 0
    sum_abs = 0
    big80 = 0
    max_abs = 0
    for y, (ref_row, native_row) in enumerate(zip(ref_rows, native_rows, strict=True)):
        _ = y
        for x in range(width):
            ref_i = x * ref_channels
            native_i = x * native_channels
            diff = sum(
                abs(ref_row[ref_i + channel] - native_row[native_i + channel])
                for channel in range(3)
            )
            if diff:
                changed += 1
                sum_abs += diff
                max_abs = max(max_abs, diff)
                if diff > 80:
                    big80 += 1

    changed_pct = changed * 100.0 / total
    mean_abs = sum_abs / total
    big80_pct = big80 * 100.0 / total
    threshold = THRESHOLDS[name]
    if changed_pct > threshold.changed_pct:
        raise SystemExit(
            f"{name}: changed {changed_pct:.2f}% exceeds {threshold.changed_pct:.2f}%"
        )
    if mean_abs > threshold.mean_abs:
        raise SystemExit(f"{name}: mean abs {mean_abs:.2f} exceeds {threshold.mean_abs:.2f}")
    if big80_pct > threshold.big80_pct:
        raise SystemExit(f"{name}: big80 {big80_pct:.2f}% exceeds {threshold.big80_pct:.2f}%")

    print(
        f"ok {name}: changed={changed_pct:.2f}% mean_abs={mean_abs:.2f} "
        f"big80={big80_pct:.2f}% max_abs={max_abs}"
    )


for state_name in THRESHOLDS:
    compare(state_name)
PY

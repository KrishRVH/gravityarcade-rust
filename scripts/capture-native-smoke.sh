#!/usr/bin/env bash
set -euo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$ROOT"

cargo build --quiet

BIN="$ROOT/target/debug/gravityarcade"
OUT_DIR="$ROOT/target/debug/visual-smoke"
mkdir -p "$OUT_DIR"

capture_env=()
if [[ "$(uname -s)" == "Linux" ]]; then
    ALSA_CONFIG="$OUT_DIR/asoundrc"
    cat >"$ALSA_CONFIG" <<'EOF'
pcm.!default {
    type null
}
EOF
    capture_env+=(ALSA_CONFIG_PATH="$ALSA_CONFIG")
fi

capture() {
    local name=$1
    local warp=$2
    local ticks=${3:-}
    local output="$OUT_DIR/$name.png"
    local shot="$output"
    if [[ -n "$ticks" ]]; then
        shot="$shot:$ticks"
    fi
    rm -f "$output"
    env "${capture_env[@]}" GRAVITYARCADE_WARP="$warp" GRAVITYARCADE_SHOT="$shot" "$BIN" >/dev/null
    echo "$output"
}

captures=()
captures+=("$(capture startup startup)")
captures+=("$(capture menu menu)")
captures+=("$(capture menu-polarisation-opposite menu_polarisation_opposite)")
captures+=("$(capture menu-polarisation-same menu_polarisation_same)")
captures+=("$(capture menu-polarisation-all menu_polarisation_all)")
captures+=("$(capture menu-matches-5 menu_matches_5)")
captures+=("$(capture menu-matches-7 menu_matches_7)")
captures+=("$(capture menu-matches-1 menu_matches_1)")
captures+=("$(capture menu-gravity-low menu_gravity_low)")
captures+=("$(capture menu-gravity-high menu_gravity_high)")
captures+=("$(capture menu-gravity-very-high menu_gravity_very_high)")
captures+=("$(capture menu-gravity-black-hole menu_gravity_black_hole)")
captures+=("$(capture menu-speed-fast menu_speed_fast)")
captures+=("$(capture help help)")
captures+=("$(capture menu-after-help menu)")
captures+=("$(capture playing-idle playing_idle)")
captures+=("$(capture playing playing)")
captures+=("$(capture playing-red playing_red)")
captures+=("$(capture score-ramps score_ramps)")
captures+=("$(capture score-max score_max)")
captures+=("$(capture round-intro-1 round_intro_1)")
captures+=("$(capture round-intro round_intro)")
captures+=("$(capture blue-win blue_win)")
captures+=("$(capture red-win red_win)")
captures+=("$(capture blue-final blue_final)")
captures+=("$(capture red-final red_final)")

python3 - "$ROOT" "${captures[@]}" <<'PY'
import struct
import sys
import zlib
from pathlib import Path

root = Path(sys.argv[1])
paths = [Path(path) for path in sys.argv[2:]]


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


def read_png(path: Path) -> tuple[int, int, int, int, list[bytes]]:
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
    unique = set()
    bpp = channels
    for _ in range(height):
        filter_type = raw[pos]
        pos += 1
        row = bytearray(raw[pos : pos + stride])
        pos += stride
        for i, value in enumerate(row):
            left = row[i - bpp] if i >= bpp else 0
            up = previous[i]
            upper_left = previous[i - bpp] if i >= bpp else 0
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
        previous = row
        rows.append(bytes(row))
        if len(unique) < 256:
            for i in range(0, len(row), channels):
                unique.add(bytes(row[i : i + channels]))

    if len(unique) < 8:
        raise SystemExit(f"{path}: too few unique pixels ({len(unique)})")
    return width, height, channels, len(unique), rows


def count_matching_pixels(
    rows: list[bytes],
    channels: int,
    bounds: tuple[int, int, int, int],
    predicate,
) -> int:
    x0, y0, x1, y1 = bounds
    total = 0
    for row in rows[y0:y1]:
        for x in range(x0, x1):
            i = x * channels
            if predicate(row[i], row[i + 1], row[i + 2]):
                total += 1
    return total


def assert_region_color(
    path: Path,
    rows: list[bytes],
    channels: int,
    label: str,
    bounds: tuple[int, int, int, int],
    minimum: int,
    predicate,
) -> str:
    count = count_matching_pixels(rows, channels, bounds, predicate)
    if count < minimum:
        raise SystemExit(f"{path}: expected at least {minimum} {label} pixels, got {count}")
    return f"{label}>={count}"


def assert_expected_content(path: Path, rows: list[bytes], channels: int) -> list[str]:
    checks = {
        "score-ramps.png": [
            ("blue-meter", (5, 35, 545, 395), 600, lambda r, g, b: r < 80 and g > 160 and b > 180),
            ("red-meter", (5, 35, 545, 395), 600, lambda r, g, b: r > 180 and 80 < g < 210 and b < 90),
        ],
        "score-max.png": [
            ("blue-meter", (5, 35, 545, 395), 600, lambda r, g, b: r < 80 and g > 160 and b > 180),
            ("red-meter", (5, 35, 545, 395), 1_000, lambda r, g, b: r > 180 and 80 < g < 210 and b < 90),
        ],
        "round-intro.png": [
            ("white-announcement", (220, 135, 330, 260), 1_000, lambda r, g, b: r > 200 and g > 200 and b > 200),
        ],
        "blue-win.png": [
            ("cyan-announcement", (95, 100, 455, 190), 2_500, lambda r, g, b: r < 170 and g > 180 and b > 180),
        ],
        "red-win.png": [
            ("orange-announcement", (100, 100, 450, 190), 2_500, lambda r, g, b: r > 180 and g > 110 and b < 100),
        ],
        "blue-final.png": [
            ("cyan-final", (100, 70, 455, 360), 20_000, lambda r, g, b: r < 170 and g > 180 and b > 180),
        ],
        "red-final.png": [
            ("orange-final", (100, 70, 455, 360), 15_000, lambda r, g, b: r > 180 and g > 110 and b < 100),
        ],
    }

    return [
        assert_region_color(path, rows, channels, label, bounds, minimum, predicate)
        for label, bounds, minimum, predicate in checks.get(path.name, [])
    ]


def count_changed_pixels(
    left_rows: list[bytes],
    left_channels: int,
    right_rows: list[bytes],
    right_channels: int,
    bounds: tuple[int, int, int, int],
    threshold: int,
) -> int:
    x0, y0, x1, y1 = bounds
    total = 0
    for left_row, right_row in zip(left_rows[y0:y1], right_rows[y0:y1], strict=True):
        for x in range(x0, x1):
            left_i = x * left_channels
            right_i = x * right_channels
            diff = sum(
                abs(left_row[left_i + channel] - right_row[right_i + channel])
                for channel in range(3)
            )
            if diff > threshold:
                total += 1
    return total


def assert_changed_region(
    left_name: str,
    right_name: str,
    decoded: dict[str, tuple[int, int, list[bytes]]],
    label: str,
    bounds: tuple[int, int, int, int],
    threshold: int,
    minimum: int,
    maximum: int | None = None,
) -> str:
    left_channels, _, left_rows = decoded[left_name]
    right_channels, _, right_rows = decoded[right_name]
    count = count_changed_pixels(
        left_rows,
        left_channels,
        right_rows,
        right_channels,
        bounds,
        threshold,
    )
    if count < minimum:
        raise SystemExit(
            f"{left_name} vs {right_name}: expected at least {minimum} {label} pixels, got {count}"
        )
    if maximum is not None and count > maximum:
        raise SystemExit(
            f"{left_name} vs {right_name}: expected at most {maximum} {label} pixels, got {count}"
        )
    if maximum is None:
        return f"{label}>={count}"
    return f"{label}={count}"


def assert_playfield_state_pair(decoded: dict[str, tuple[int, int, list[bytes]]]) -> list[str]:
    return [
        assert_changed_region(
            "playing-idle.png",
            "playing.png",
            decoded,
            "right-launch-delta",
            (500, 180, 523, 248),
            30,
            500,
            1_200,
        ),
        assert_changed_region(
            "playing-idle.png",
            "playing.png",
            decoded,
            "central-field-delta",
            (70, 40, 480, 390),
            30,
            0,
            50,
        ),
        assert_changed_region(
            "playing-idle.png",
            "playing-red.png",
            decoded,
            "left-launch-delta",
            (27, 180, 60, 248),
            30,
            500,
            1_200,
        ),
        assert_changed_region(
            "playing-idle.png",
            "playing-red.png",
            decoded,
            "central-field-red-delta",
            (70, 40, 480, 390),
            30,
            0,
            50,
        ),
    ]


decoded: dict[str, tuple[int, int, list[bytes]]] = {}
for path in paths:
    width, height, channels, unique, rows = read_png(path)
    decoded[path.name] = (channels, unique, rows)
    content = assert_expected_content(path, rows, channels)
    rel = path.relative_to(root)
    suffix = f" {' '.join(content)}" if content else ""
    print(f"ok {rel} {width}x{height} unique>={unique}{suffix}")

print(f"ok playfield-state-pair {' '.join(assert_playfield_state_pair(decoded))}")
PY

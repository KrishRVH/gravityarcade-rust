#!/usr/bin/env bash
set -euo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$ROOT"

OUT_DIR=${1:-"$ROOT/target/reference-swf"}
mkdir -p "$OUT_DIR"

find_browse() {
    local candidates=()
    if [[ -n "${BROWSE:-}" ]]; then
        candidates+=("$BROWSE")
    fi
    candidates+=(
        "$HOME/.codex/skills/gstack/browse/dist/browse"
        "$ROOT/.agents/skills/gstack/browse/dist/browse"
        "$HOME/gstack/.agents/skills/gstack/browse/dist/browse"
        "$HOME/gstack/browse/dist/browse"
    )
    for candidate in "${candidates[@]}"; do
        if [[ -x "$candidate" ]]; then
            printf '%s\n' "$candidate"
            return 0
        fi
    done
    if command -v browse >/dev/null 2>&1; then
        command -v browse
        return 0
    fi
    return 1
}

BROWSE_BIN=$(find_browse) || {
    echo "gstack browse binary not found; set BROWSE=/path/to/browse" >&2
    exit 1
}

PORT=$(
    python3 - <<'PY'
import socket

with socket.socket() as sock:
    sock.bind(("127.0.0.1", 0))
    print(sock.getsockname()[1])
PY
)

python3 -m http.server "$PORT" --bind 127.0.0.1 >"$OUT_DIR/http.log" 2>&1 &
SERVER_PID=$!

cleanup() {
    kill "$SERVER_PID" >/dev/null 2>&1 || true
    wait "$SERVER_PID" >/dev/null 2>&1 || true
}
trap cleanup EXIT

for _ in {1..50}; do
    if python3 - "$PORT" <<'PY' >/dev/null 2>&1
import sys
import urllib.request

urllib.request.urlopen(f"http://127.0.0.1:{sys.argv[1]}/reference/play_swf.html", timeout=0.2).read(1)
PY
    then
        break
    fi
    sleep 0.1
done

capture() {
    local name=$1
    "$BROWSE_BIN" screenshot "$OUT_DIR/$name.png" --clip 0,0,550,400 >/dev/null
    echo "$OUT_DIR/$name.png"
}

click_stage() {
    local x=$1
    local y=$2
    "$BROWSE_BIN" js "(()=>{const canvas=document.querySelector('ruffle-player')?.shadowRoot?.querySelector('canvas');if(!canvas){return 'missing-canvas';}const rect=canvas.getBoundingClientRect();const clientX=rect.left+$x;const clientY=rect.top+$y;for(const type of ['pointermove','mousemove','pointerdown','mousedown','pointerup','mouseup','click']){canvas.dispatchEvent(new MouseEvent(type,{clientX,clientY,bubbles:true,composed:true,button:0,buttons:type.includes('down')?1:0}));}return 'clicked';})()" >/dev/null
}

key_stage() {
    local type=$1
    local key=$2
    local code=$3
    local key_code=$4
    "$BROWSE_BIN" js "(()=>{const canvas=document.querySelector('ruffle-player')?.shadowRoot?.querySelector('canvas');const targets=[window,document,canvas].filter(Boolean);for(const target of targets){target.dispatchEvent(new KeyboardEvent('$type',{key:'$key',code:'$code',keyCode:$key_code,which:$key_code,bubbles:true,composed:true}));}return '$type';})()" >/dev/null
}

TARGET_URL="http://127.0.0.1:$PORT/reference/play_swf.html"

load_menu() {
    local current_url=""
    for _ in {1..5}; do
        "$BROWSE_BIN" goto "$TARGET_URL" >/dev/null || true
        sleep 0.5
        current_url=$("$BROWSE_BIN" url 2>/dev/null || true)
        if [[ "$current_url" == "$TARGET_URL" ]]; then
            break
        fi
    done
    if [[ "$current_url" != "$TARGET_URL" ]]; then
        echo "browse did not navigate to $TARGET_URL; current URL is ${current_url:-unknown}" >&2
        exit 1
    fi
    "$BROWSE_BIN" wait --networkidle >/dev/null
    sleep 4
}

"$BROWSE_BIN" viewport 550x400 >/dev/null
"$BROWSE_BIN" status >/dev/null || true

captures=()
load_menu
captures+=("$(capture menu)")

load_menu
click_stage 266 121
sleep 1
captures+=("$(capture menu-polarisation-opposite)")

load_menu
click_stage 266 121
sleep 0.2
click_stage 266 121
sleep 1
captures+=("$(capture menu-polarisation-same)")

load_menu
click_stage 266 121
sleep 0.2
click_stage 266 121
sleep 0.2
click_stage 266 121
sleep 1
captures+=("$(capture menu-polarisation-all)")

load_menu
click_stage 266 178
sleep 1
captures+=("$(capture menu-matches-5)")

load_menu
click_stage 266 178
sleep 0.2
click_stage 266 178
sleep 1
captures+=("$(capture menu-matches-7)")

load_menu
click_stage 266 178
sleep 0.2
click_stage 266 178
sleep 0.2
click_stage 266 178
sleep 1
captures+=("$(capture menu-matches-1)")

load_menu
click_stage 266 235
sleep 0.2
click_stage 266 235
sleep 0.2
click_stage 266 235
sleep 0.2
click_stage 266 235
sleep 1
captures+=("$(capture menu-gravity-low)")

load_menu
click_stage 266 235
sleep 1
captures+=("$(capture menu-gravity-high)")

load_menu
click_stage 266 235
sleep 0.2
click_stage 266 235
sleep 1
captures+=("$(capture menu-gravity-very-high)")

load_menu
click_stage 266 235
sleep 0.2
click_stage 266 235
sleep 0.2
click_stage 266 235
sleep 1
captures+=("$(capture menu-gravity-black-hole)")

load_menu
click_stage 266 293
sleep 1
captures+=("$(capture menu-speed-fast)")

load_menu
click_stage 266 63
sleep 1
captures+=("$(capture help)")
click_stage 268 349
sleep 1
captures+=("$(capture menu-after-help)")

load_menu
click_stage 266 349
sleep 0.9
captures+=("$(capture round-intro-1)")

load_menu
click_stage 271 350
sleep 1
click_stage 266 349
sleep 3
captures+=("$(capture playing-idle)")
key_stage keydown ArrowLeft ArrowLeft 37
sleep 0.05
captures+=("$(capture playing)")
key_stage keyup ArrowLeft ArrowLeft 37

load_menu
click_stage 271 350
sleep 1
click_stage 266 349
sleep 3
key_stage keydown d KeyD 68
sleep 0.05
captures+=("$(capture playing-red)")
key_stage keyup d KeyD 68

"$BROWSE_BIN" console >"$OUT_DIR/console.txt" || true
"$BROWSE_BIN" network >"$OUT_DIR/network.txt" || true

python3 - "${captures[@]}" <<'PY'
import struct
import sys
import zlib
from pathlib import Path


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
    previous = bytearray(stride)
    rows: list[bytes] = []
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
        previous = row
        rows.append(bytes(row))
    return width, height, channels, rows


def count_pixels(rows: list[bytes], channels: int, predicate) -> int:
    total = 0
    for row in rows:
        for x in range(550):
            i = x * channels
            if predicate(row[i], row[i + 1], row[i + 2]):
                total += 1
    return total


MENU_CHECKS = (
    ("dark-blue-stage", 150_000, lambda r, g, b: r < 20 and g < 20 and b > 55),
    ("white-text", 3_000, lambda r, g, b: r > 210 and g > 210 and b > 210),
    ("cyan-buttons", 500, lambda r, g, b: r < 120 and g > 150 and b > 150),
)

CHECKS = {
    "menu": MENU_CHECKS,
    "menu-polarisation-opposite": MENU_CHECKS,
    "menu-polarisation-same": MENU_CHECKS,
    "menu-polarisation-all": MENU_CHECKS,
    "menu-matches-5": MENU_CHECKS,
    "menu-matches-7": MENU_CHECKS,
    "menu-matches-1": MENU_CHECKS,
    "menu-gravity-low": MENU_CHECKS,
    "menu-gravity-high": MENU_CHECKS,
    "menu-gravity-very-high": MENU_CHECKS,
    "menu-gravity-black-hole": MENU_CHECKS,
    "menu-speed-fast": MENU_CHECKS,
    "menu-after-help": MENU_CHECKS,
    "help": (
        ("dark-blue-stage", 140_000, lambda r, g, b: r < 20 and g < 20 and b > 55),
        ("white-help-copy", 3_000, lambda r, g, b: r > 210 and g > 210 and b > 210),
        ("orange-help-labels", 400, lambda r, g, b: r > 180 and 70 < g < 170 and b < 80),
    ),
    "playing-idle": (
        ("dark-blue-stage", 170_000, lambda r, g, b: r < 20 and g < 20 and b > 55),
        ("white-chrome", 2_000, lambda r, g, b: r > 210 and g > 210 and b > 210),
        ("cyan-paddle", 300, lambda r, g, b: r < 120 and g > 150 and b > 150),
        ("yellow-paddle", 300, lambda r, g, b: r > 180 and g > 150 and b < 80),
    ),
    "playing": (
        ("dark-blue-stage", 170_000, lambda r, g, b: r < 20 and g < 20 and b > 55),
        ("white-chrome", 2_000, lambda r, g, b: r > 210 and g > 210 and b > 210),
        ("cyan-paddle", 300, lambda r, g, b: r < 120 and g > 150 and b > 150),
        ("yellow-paddle", 300, lambda r, g, b: r > 180 and g > 150 and b < 80),
    ),
    "playing-red": (
        ("dark-blue-stage", 170_000, lambda r, g, b: r < 20 and g < 20 and b > 55),
        ("white-chrome", 2_000, lambda r, g, b: r > 210 and g > 210 and b > 210),
        ("cyan-paddle", 300, lambda r, g, b: r < 120 and g > 150 and b > 150),
        ("yellow-paddle", 300, lambda r, g, b: r > 180 and g > 150 and b < 80),
    ),
    "round-intro-1": (
        ("dark-blue-stage", 165_000, lambda r, g, b: r < 20 and g < 20 and b > 55),
        ("white-announcement", 3_500, lambda r, g, b: r > 210 and g > 210 and b > 210),
    ),
}


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
    decoded: dict[str, tuple[int, list[bytes]]],
    label: str,
    bounds: tuple[int, int, int, int],
    threshold: int,
    minimum: int,
    maximum: int | None = None,
) -> str:
    left_channels, left_rows = decoded[left_name]
    right_channels, right_rows = decoded[right_name]
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


def assert_playfield_state_pair(decoded: dict[str, tuple[int, list[bytes]]]) -> list[str]:
    return [
        assert_changed_region(
            "playing-idle",
            "playing",
            decoded,
            "right-launch-delta",
            (490, 180, 523, 248),
            30,
            500,
            1_200,
        ),
        assert_changed_region(
            "playing-idle",
            "playing",
            decoded,
            "central-field-delta",
            (70, 40, 480, 390),
            30,
            0,
            80,
        ),
        assert_changed_region(
            "playing-idle",
            "playing-red",
            decoded,
            "left-launch-delta",
            (27, 180, 60, 248),
            30,
            500,
            1_200,
        ),
        assert_changed_region(
            "playing-idle",
            "playing-red",
            decoded,
            "central-field-red-delta",
            (70, 40, 480, 390),
            30,
            0,
            80,
        ),
    ]


decoded: dict[str, tuple[int, list[bytes]]] = {}
for raw_path in sys.argv[1:]:
    path = Path(raw_path)
    width, height, channels, rows = read_png(path)
    decoded[path.stem] = (channels, rows)
    content = []
    for label, minimum, predicate in CHECKS[path.stem]:
        count = count_pixels(rows, channels, predicate)
        if count < minimum:
            raise SystemExit(f"{path}: expected at least {minimum} {label} pixels, got {count}")
        content.append(f"{label}>={count}")
    print(f"ok {path} {width}x{height} {' '.join(content)}")

print(f"ok reference-playfield-state-pair {' '.join(assert_playfield_state_pair(decoded))}")
PY

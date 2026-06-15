#!/usr/bin/env python3
"""Extract SWF shape records for auditing the Rust renderer.

This is intentionally small and focused on the Flash 5 shape tags used by
gravity_arcade.swf. It preserves edge direction and active fill/line styles so
shared-boundary fills can be reconstructed without relying on temporary dumps.
"""

from __future__ import annotations

import argparse
import json
import struct
import zlib
from dataclasses import dataclass
from pathlib import Path
from typing import Any


TAG_NAMES = {
    2: "DefineShape",
    22: "DefineShape2",
    32: "DefineShape3",
}


@dataclass
class Bits:
    data: bytes
    byte: int = 0
    bit: int = 0

    def ub(self, n: int) -> int:
        value = 0
        for _ in range(n):
            value = (value << 1) | ((self.data[self.byte] >> (7 - self.bit)) & 1)
            self.bit += 1
            if self.bit == 8:
                self.bit = 0
                self.byte += 1
        return value

    def sb(self, n: int) -> int:
        value = self.ub(n)
        if n and value & (1 << (n - 1)):
            value -= 1 << n
        return value

    def fb(self, n: int) -> float:
        return self.sb(n) / 65536.0

    def align(self) -> None:
        if self.bit:
            self.bit = 0
            self.byte += 1


def read_rect(bits: Bits) -> list[float]:
    nbits = bits.ub(5)
    rect = [bits.sb(nbits) / 20.0 for _ in range(4)]
    bits.align()
    return rect


def read_color(bits: Bits, alpha: bool) -> dict[str, int]:
    bits.align()
    pos = bits.byte
    red, green, blue = bits.data[pos : pos + 3]
    bits.byte += 3
    out = {"r": red, "g": green, "b": blue}
    if alpha:
        out["a"] = bits.data[bits.byte]
        bits.byte += 1
    return out


def color_hex(color: dict[str, int]) -> str:
    rgb = f"#{color['r']:02x}{color['g']:02x}{color['b']:02x}"
    if "a" in color:
        return f"{rgb}@{color['a']}"
    return rgb


def skip_matrix(bits: Bits) -> None:
    if bits.ub(1):
        nbits = bits.ub(5)
        bits.fb(nbits)
        bits.fb(nbits)
    if bits.ub(1):
        nbits = bits.ub(5)
        bits.fb(nbits)
        bits.fb(nbits)
    nbits = bits.ub(5)
    bits.sb(nbits)
    bits.sb(nbits)
    bits.align()


def read_count(bits: Bits) -> int:
    bits.align()
    count = bits.data[bits.byte]
    bits.byte += 1
    if count == 0xFF:
        count = struct.unpack_from("<H", bits.data, bits.byte)[0]
        bits.byte += 2
    return count


def read_fill_style(bits: Bits, alpha: bool) -> dict[str, Any]:
    bits.align()
    fill_type = bits.data[bits.byte]
    bits.byte += 1
    if fill_type == 0x00:
        color = read_color(bits, alpha)
        return {"type": "solid", "color": color_hex(color)}
    if fill_type in (0x10, 0x12, 0x13):
        skip_matrix(bits)
        bits.align()
        gradient_count = bits.data[bits.byte]
        bits.byte += 1
        records = []
        for _ in range(gradient_count):
            ratio = bits.data[bits.byte]
            bits.byte += 1
            color = read_color(bits, alpha)
            records.append({"ratio": ratio, "color": color_hex(color)})
        return {"type": f"gradient_{fill_type:02x}", "records": records}
    if fill_type in (0x40, 0x41, 0x42, 0x43):
        bits.align()
        bitmap_id = struct.unpack_from("<H", bits.data, bits.byte)[0]
        bits.byte += 2
        skip_matrix(bits)
        return {"type": f"bitmap_{fill_type:02x}", "bitmap_id": bitmap_id}
    return {"type": f"unknown_{fill_type:02x}"}


def read_fill_styles(bits: Bits, alpha: bool) -> list[dict[str, Any]]:
    return [read_fill_style(bits, alpha) for _ in range(read_count(bits))]


def read_line_styles(bits: Bits, alpha: bool) -> list[dict[str, Any]]:
    lines = []
    for _ in range(read_count(bits)):
        bits.align()
        width = struct.unpack_from("<H", bits.data, bits.byte)[0] / 20.0
        bits.byte += 2
        color = read_color(bits, alpha)
        lines.append({"width": width, "color": color_hex(color)})
    return lines


def read_style_arrays(bits: Bits, alpha: bool) -> tuple[list[dict[str, Any]], list[dict[str, Any]]]:
    return read_fill_styles(bits, alpha), read_line_styles(bits, alpha)


def read_shape_records(
    bits: Bits,
    alpha: bool,
    num_fill_bits: int,
    num_line_bits: int,
) -> list[dict[str, Any]]:
    records = []
    x = 0
    y = 0
    fill0 = 0
    fill1 = 0
    line = 0

    while True:
        edge_record = bits.ub(1)
        if edge_record:
            straight = bits.ub(1)
            if straight:
                nbits = bits.ub(4) + 2
                general = bits.ub(1)
                if general:
                    dx = bits.sb(nbits)
                    dy = bits.sb(nbits)
                else:
                    vertical = bits.ub(1)
                    if vertical:
                        dx = 0
                        dy = bits.sb(nbits)
                    else:
                        dx = bits.sb(nbits)
                        dy = 0
                next_x = x + dx
                next_y = y + dy
                records.append(
                    {
                        "type": "line",
                        "from": [x / 20.0, y / 20.0],
                        "to": [next_x / 20.0, next_y / 20.0],
                        "fill0": fill0,
                        "fill1": fill1,
                        "line": line,
                    }
                )
                x = next_x
                y = next_y
            else:
                nbits = bits.ub(4) + 2
                control_x = x + bits.sb(nbits)
                control_y = y + bits.sb(nbits)
                next_x = control_x + bits.sb(nbits)
                next_y = control_y + bits.sb(nbits)
                records.append(
                    {
                        "type": "curve",
                        "from": [x / 20.0, y / 20.0],
                        "control": [control_x / 20.0, control_y / 20.0],
                        "to": [next_x / 20.0, next_y / 20.0],
                        "fill0": fill0,
                        "fill1": fill1,
                        "line": line,
                    }
                )
                x = next_x
                y = next_y
            continue

        state_new_styles = bits.ub(1)
        state_line = bits.ub(1)
        state_fill1 = bits.ub(1)
        state_fill0 = bits.ub(1)
        state_move = bits.ub(1)
        if not (state_new_styles or state_line or state_fill1 or state_fill0 or state_move):
            records.append({"type": "end"})
            break

        record: dict[str, Any] = {"type": "style_change"}
        if state_move:
            move_bits = bits.ub(5)
            x = bits.sb(move_bits)
            y = bits.sb(move_bits)
            record["move_to"] = [x / 20.0, y / 20.0]
        if state_fill0:
            fill0 = bits.ub(num_fill_bits)
            record["fill0"] = fill0
        if state_fill1:
            fill1 = bits.ub(num_fill_bits)
            record["fill1"] = fill1
        if state_line:
            line = bits.ub(num_line_bits)
            record["line"] = line
        if state_new_styles:
            fills, lines = read_style_arrays(bits, alpha)
            record["new_fills"] = fills
            record["new_lines"] = lines
            num_fill_bits = bits.ub(4)
            num_line_bits = bits.ub(4)
            record["num_fill_bits"] = num_fill_bits
            record["num_line_bits"] = num_line_bits
        records.append(record)

    return records


def point_key(point: list[float]) -> tuple[float, float]:
    return point[0], point[1]


def fill_style_key(style: dict[str, Any] | None) -> str:
    if style is None:
        return ""
    return json.dumps(style, sort_keys=True, separators=(",", ":"))


def edge_from_record(record: dict[str, Any], reverse: bool) -> dict[str, Any]:
    if reverse:
        edge = {
            "type": record["type"],
            "from": record["to"],
            "to": record["from"],
        }
    else:
        edge = {
            "type": record["type"],
            "from": record["from"],
            "to": record["to"],
        }
    if record["type"] == "curve":
        edge["control"] = record["control"]
    return edge


def segment_from_edge(edge: dict[str, Any]) -> dict[str, Any]:
    segment = {
        "type": edge["type"],
        "to": edge["to"],
    }
    if edge["type"] == "curve":
        segment["control"] = edge["control"]
    return segment


def build_fill_contours(records: list[dict[str, Any]], fills: list[dict[str, Any]]) -> list[dict[str, Any]]:
    current_fills = list(fills)
    edges_by_fill: dict[tuple[int, str], list[dict[str, Any]]] = {}
    style_by_fill: dict[tuple[int, str], dict[str, Any] | None] = {}
    fill_order: list[tuple[int, str]] = []

    def style_for(fill_index: int) -> dict[str, Any] | None:
        if 0 < fill_index <= len(current_fills):
            return current_fills[fill_index - 1]
        return None

    def add_edge(fill_index: int, record: dict[str, Any], reverse: bool) -> None:
        if fill_index == 0:
            return
        style = style_for(fill_index)
        key = (fill_index, fill_style_key(style))
        if key not in edges_by_fill:
            edges_by_fill[key] = []
            style_by_fill[key] = style
            fill_order.append(key)
        edges_by_fill[key].append(edge_from_record(record, reverse))

    for record in records:
        if record["type"] in ("line", "curve"):
            add_edge(record["fill0"], record, reverse=True)
            add_edge(record["fill1"], record, reverse=False)
        elif record["type"] == "style_change" and "new_fills" in record:
            current_fills.extend(record["new_fills"])

    contours = []
    for key in fill_order:
        fill_index, _ = key
        unused = list(edges_by_fill[key])
        while unused:
            path = [unused.pop(0)]
            start = point_key(path[0]["from"])
            cursor = point_key(path[-1]["to"])
            while cursor != start:
                next_index = next(
                    (index for index, edge in enumerate(unused) if point_key(edge["from"]) == cursor),
                    None,
                )
                if next_index is None:
                    break
                path.append(unused.pop(next_index))
                cursor = point_key(path[-1]["to"])
            contours.append(
                {
                    "fill": fill_index,
                    "style": style_by_fill[key],
                    "start": path[0]["from"],
                    "closed": cursor == start,
                    "edge_count": len(path),
                    "segments": [segment_from_edge(edge) for edge in path],
                }
            )
    return contours


def parse_shape(tag_code: int, body: bytes, include_contours: bool = False) -> dict[str, Any]:
    shape_id = struct.unpack_from("<H", body, 0)[0]
    alpha = tag_code == 32
    bits = Bits(body, 2)
    bounds = read_rect(bits)
    fills, lines = read_style_arrays(bits, alpha)
    num_fill_bits = bits.ub(4)
    num_line_bits = bits.ub(4)
    records = read_shape_records(bits, alpha, num_fill_bits, num_line_bits)
    shape = {
        "id": shape_id,
        "tag": TAG_NAMES[tag_code],
        "bounds": bounds,
        "fills": fills,
        "lines": lines,
        "records": records,
    }
    if include_contours:
        shape["contours"] = build_fill_contours(records, fills)
    return shape


def swf_body(path: Path) -> bytes:
    raw = path.read_bytes()
    signature = raw[:3]
    if signature == b"FWS":
        return raw
    if signature == b"CWS":
        return raw[:8] + zlib.decompress(raw[8:])
    raise ValueError(f"unsupported SWF signature {signature!r}")


def iter_tags(data: bytes):
    bits = Bits(data, 8)
    read_rect(bits)
    bits.align()
    pos = bits.byte + 4
    while pos < len(data):
        header = struct.unpack_from("<H", data, pos)[0]
        pos += 2
        tag_code = header >> 6
        length = header & 0x3F
        if length == 0x3F:
            length = struct.unpack_from("<I", data, pos)[0]
            pos += 4
        body = data[pos : pos + length]
        pos += length
        yield tag_code, body
        if tag_code == 0:
            break


def extract_shapes(
    path: Path,
    shape_ids: set[int] | None,
    include_contours: bool = False,
) -> list[dict[str, Any]]:
    shapes = []
    for tag_code, body in iter_tags(swf_body(path)):
        if tag_code not in TAG_NAMES:
            continue
        shape = parse_shape(tag_code, body, include_contours)
        if shape_ids is None or shape["id"] in shape_ids:
            shapes.append(shape)
    return shapes


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("swf", type=Path)
    parser.add_argument("--shape", type=int, action="append", dest="shape_ids")
    parser.add_argument("--contours", action="store_true", help="reconstruct fill contours from oriented edges")
    parser.add_argument("--indent", type=int, default=2)
    args = parser.parse_args()

    selected = set(args.shape_ids) if args.shape_ids else None
    print(json.dumps(extract_shapes(args.swf, selected, args.contours), indent=args.indent))


if __name__ == "__main__":
    main()

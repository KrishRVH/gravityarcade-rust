#!/usr/bin/env python3
"""Generate recovered font-26 digit contours for the rounds-played counter."""

from __future__ import annotations

import argparse
import json
import struct
from collections import Counter, defaultdict
from dataclasses import dataclass
from pathlib import Path
from statistics import mode
from typing import Iterable

from extract_shapes import Bits, build_fill_contours, iter_tags, read_rect, read_shape_records, swf_body


FONT_ID = 26
TEXT_HEIGHT = 12.0
BASELINE_Y = 11.8
EM_UNITS = 1024.0
TWIPS_PER_PIXEL = 20.0
DIGITS = "012345678"


@dataclass(frozen=True)
class Segment:
    kind: str
    to: tuple[float, float]
    control: tuple[float, float] | None = None


@dataclass(frozen=True)
class Glyph:
    code: int
    advance: float
    contours: tuple[tuple[tuple[float, float], tuple[Segment, ...]], ...]


def parse_define_font(body: bytes) -> tuple[int, tuple[tuple[tuple[float, float], tuple[Segment, ...]], ...]]:
    font_id = struct.unpack_from("<H", body, 0)[0]
    first_offset = struct.unpack_from("<H", body, 2)[0]
    glyph_count = first_offset // 2
    offsets = [struct.unpack_from("<H", body, 2 + index * 2)[0] for index in range(glyph_count)]
    glyphs = []
    for index, offset in enumerate(offsets):
        start = 2 + offset
        end = 2 + (offsets[index + 1] if index + 1 < glyph_count else len(body) - 2)
        bits = Bits(body[start:end])
        num_fill_bits = bits.ub(4)
        num_line_bits = bits.ub(4)
        records = read_shape_records(bits, False, num_fill_bits, num_line_bits)
        shape_contours = build_fill_contours(records, [{"type": "solid", "color": "#000000"}])
        glyphs.append(tuple(contour_from_shape(contour) for contour in shape_contours))
    return font_id, tuple(glyphs)


def parse_font_info(body: bytes) -> tuple[int, tuple[int, ...]]:
    font_id = struct.unpack_from("<H", body, 0)[0]
    name_len = body[2]
    codes = tuple(body[4 + name_len :])
    return font_id, codes


def contour_from_shape(shape_contour: dict) -> tuple[tuple[float, float], tuple[Segment, ...]]:
    segments = []
    for segment in shape_contour["segments"]:
        if segment["type"] == "line":
            segments.append(Segment("line", tuple(segment["to"])))
        else:
            segments.append(Segment("quad", tuple(segment["to"]), tuple(segment["control"])))
    return tuple(shape_contour["start"]), tuple(segments)


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


def parse_define_text_advances(body: bytes) -> list[tuple[int, float, int, float]]:
    bits = Bits(body, 2)
    read_rect(bits)
    skip_matrix(bits)
    bits.align()
    glyph_bits = bits.data[bits.byte]
    advance_bits = bits.data[bits.byte + 1]
    bits.byte += 2

    active_font = 0
    active_height = 0.0
    advances = []
    while True:
        bits.align()
        flags = bits.data[bits.byte]
        bits.byte += 1
        if flags == 0:
            break
        if flags & 0x08:
            active_font = struct.unpack_from("<H", bits.data, bits.byte)[0]
            bits.byte += 2
        if flags & 0x04:
            bits.byte += 3
        if flags & 0x01:
            bits.byte += 2
        if flags & 0x02:
            bits.byte += 2
        if flags & 0x08:
            active_height = struct.unpack_from("<H", bits.data, bits.byte)[0] / TWIPS_PER_PIXEL
            bits.byte += 2

        bits.align()
        glyph_count = bits.data[bits.byte]
        bits.byte += 1
        for _ in range(glyph_count):
            glyph_index = bits.ub(glyph_bits)
            advance = bits.sb(advance_bits) / TWIPS_PER_PIXEL
            advances.append((active_font, active_height, glyph_index, advance))
    return advances


def transform_point(point: tuple[float, float]) -> tuple[float, float]:
    scale = TEXT_HEIGHT * TWIPS_PER_PIXEL / EM_UNITS
    return (point[0] * scale, BASELINE_Y + point[1] * scale)


def transform_contours(
    contours: tuple[tuple[tuple[float, float], tuple[Segment, ...]], ...],
) -> tuple[tuple[tuple[float, float], tuple[Segment, ...]], ...]:
    transformed = []
    for start, segments in contours:
        transformed.append(
            (
                transform_point(start),
                tuple(
                    Segment(
                        segment.kind,
                        transform_point(segment.to),
                        transform_point(segment.control) if segment.control else None,
                    )
                    for segment in segments
                ),
            )
        )
    return tuple(transformed)


def contour_bounds(
    contours: tuple[tuple[tuple[float, float], tuple[Segment, ...]], ...],
) -> tuple[float, float, float, float]:
    xs = []
    ys = []
    for start, segments in contours:
        xs.append(start[0])
        ys.append(start[1])
        for segment in segments:
            xs.append(segment.to[0])
            ys.append(segment.to[1])
            if segment.control:
                xs.append(segment.control[0])
                ys.append(segment.control[1])
    return min(xs), max(xs), min(ys), max(ys)


def centipx(value: float) -> int:
    return round(value * 100.0)


def point_expr(point: tuple[float, float]) -> str:
    return f"[{centipx(point[0])}, {centipx(point[1])}]"


def rust_array(values: Iterable[int]) -> str:
    return "[" + ", ".join(str(value) for value in values) + "]"


def segment_expr(segment: Segment) -> str:
    if segment.kind == "line":
        return f"CounterDigitSegment::Line({point_expr(segment.to)})"
    return (
        "CounterDigitSegment::Quad { "
        f"control: {point_expr(segment.control or (0.0, 0.0))}, "
        f"to: {point_expr(segment.to)} "
        "}"
    )


def generate_module(glyphs: dict[str, Glyph]) -> str:
    output = [
        "// Generated from gravity_arcade.swf DefineFont 26 and DefineFontInfo 26.",
        "// Coordinates are stored as SWF text-local centipixels; divide by 100 for pixels.",
        "",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq)]",
        "pub enum CounterDigitSegment {",
        "    Line([i16; 2]),",
        "    Quad { control: [i16; 2], to: [i16; 2] },",
        "}",
        "",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq)]",
        "pub struct CounterDigitContour {",
        "    pub(super) start: [i16; 2],",
        "    pub(super) segments: &'static [CounterDigitSegment],",
        "}",
        "",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq)]",
        "pub struct CounterDigitDefinition {",
        "    pub(super) text: &'static str,",
        "    pub(super) font_id: u16,",
        "    pub(super) color_rgb: [u8; 3],",
        "    pub(super) advance_centipx: i16,",
        "    pub(super) bounds_centipx: [i16; 4],",
        "    pub(super) contours: &'static [CounterDigitContour],",
        "}",
        "",
    ]

    for digit in DIGITS:
        name = f"DIGIT_{digit}"
        glyph = glyphs[digit]
        contours = transform_contours(glyph.contours)
        bounds = [centipx(value) for value in contour_bounds(contours)]
        for index, (_, segments) in enumerate(contours):
            output.append(f"const {name}_CONTOUR_{index}: [CounterDigitSegment; {len(segments)}] = [")
            for segment in segments:
                output.append(f"    {segment_expr(segment)},")
            output.append("];")
            output.append("")

        output.append(f"const {name}_CONTOURS: [CounterDigitContour; {len(contours)}] = [")
        for index, (start, _) in enumerate(contours):
            output.append("    CounterDigitContour {")
            output.append(f"        start: {point_expr(start)},")
            output.append(f"        segments: &{name}_CONTOUR_{index},")
            output.append("    },")
        output.append("];")
        output.append("")

        output.append(f"pub const {name}: CounterDigitDefinition = CounterDigitDefinition {{")
        output.append(f"    text: {json.dumps(digit)},")
        output.append(f"    font_id: {FONT_ID},")
        output.append("    color_rgb: [140, 140, 176],")
        output.append(f"    advance_centipx: {centipx(glyph.advance)},")
        output.append(f"    bounds_centipx: {rust_array(bounds)},")
        output.append(f"    contours: &{name}_CONTOURS,")
        output.append("};")
        output.append("")

    return "\n".join(output)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("swf", type=Path)
    parser.add_argument("--out", type=Path, default=Path("src/counter_digit_texts.rs"))
    args = parser.parse_args()

    font_contours = None
    codes = None
    advances_by_glyph: dict[int, list[float]] = defaultdict(list)
    for tag_code, body in iter_tags(swf_body(args.swf)):
        if tag_code == 10 and struct.unpack_from("<H", body, 0)[0] == FONT_ID:
            _, font_contours = parse_define_font(body)
        elif tag_code == 13 and struct.unpack_from("<H", body, 0)[0] == FONT_ID:
            _, codes = parse_font_info(body)
        elif tag_code == 11:
            for font_id, height, glyph_index, advance in parse_define_text_advances(body):
                if font_id == FONT_ID and abs(height - TEXT_HEIGHT) < 0.01:
                    advances_by_glyph[glyph_index].append(advance)

    if font_contours is None:
        raise SystemExit(f"missing DefineFont {FONT_ID}")
    if codes is None:
        raise SystemExit(f"missing DefineFontInfo {FONT_ID}")

    digit_advances = [
        advance
        for glyph_index, code in enumerate(codes)
        if chr(code).isdigit()
        for advance in advances_by_glyph[glyph_index]
    ]
    fallback_digit_advance = mode(digit_advances)
    glyphs = {}
    for glyph_index, code in enumerate(codes):
        char = chr(code)
        if char not in DIGITS:
            continue
        advances = advances_by_glyph[glyph_index]
        advance = Counter(advances).most_common(1)[0][0] if advances else fallback_digit_advance
        glyphs[char] = Glyph(code, advance, font_contours[glyph_index])

    missing = set(DIGITS) - set(glyphs)
    if missing:
        raise SystemExit(f"missing embedded counter digits: {sorted(missing)}")

    args.out.write_text(generate_module(glyphs), encoding="utf-8")


if __name__ == "__main__":
    main()

#!/usr/bin/env python3
"""Generate recovered Rust contours for the sprite-162 round number edit text.

The SWF round number is DefineEditText 154 using embedded Georgia font 153 with
UseOutlines enabled. That font embeds comma plus digits 1, 2, and 3; this script
emits centered contours for the available digit glyphs.
"""

from __future__ import annotations

import argparse
import json
import struct
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable

from extract_shapes import Bits, build_fill_contours, iter_tags, read_rect, read_shape_records, swf_body


FONT_ID = 153
EDIT_TEXT_ID = 154
TEXT_HEIGHT = 12.0
BASELINE_Y = 12.0
FIELD_BOUNDS = (-2.0, 50.15, -2.0, 18.2)
FIELD_CENTER_X = (FIELD_BOUNDS[0] + FIELD_BOUNDS[1]) * 0.5
EM_UNITS = 1024.0
TWIPS_PER_PIXEL = 20.0
DIGIT_NAMES = {
    "1": "ONE",
    "2": "TWO",
    "3": "THREE",
}


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


def parse_font2(body: bytes) -> dict[str, Glyph]:
    pos = 0
    font_id = struct.unpack_from("<H", body, pos)[0]
    pos += 2
    flags = body[pos]
    pos += 1
    has_layout = bool(flags & 0x80)
    wide_offsets = bool(flags & 0x08)
    wide_codes = bool(flags & 0x04)
    pos += 1  # language code
    name_len = body[pos]
    pos += 1 + name_len
    glyph_count = struct.unpack_from("<H", body, pos)[0]
    pos += 2

    offset_size = 4 if wide_offsets else 2
    unpack_offset = "<I" if wide_offsets else "<H"
    offsets = [
        struct.unpack_from(unpack_offset, body, pos + index * offset_size)[0]
        for index in range(glyph_count)
    ]
    pos += glyph_count * offset_size
    code_table_offset = struct.unpack_from(unpack_offset, body, pos)[0]
    pos += offset_size

    offset_table_start = pos - offset_size - glyph_count * offset_size
    glyph_contours = []
    for index, offset in enumerate(offsets):
        start = offset_table_start + offset
        end = offset_table_start + (offsets[index + 1] if index + 1 < glyph_count else code_table_offset)
        bits = Bits(body[start:end])
        num_fill_bits = bits.ub(4)
        num_line_bits = bits.ub(4)
        records = read_shape_records(bits, False, num_fill_bits, num_line_bits)
        shape_contours = build_fill_contours(records, [{"type": "solid", "color": "#000000"}])
        glyph_contours.append(tuple(contour_from_shape(contour) for contour in shape_contours))

    code_pos = offset_table_start + code_table_offset
    code_size = 2 if wide_codes else 1
    unpack_code = "<H" if wide_codes else "B"
    codes = [
        struct.unpack_from(unpack_code, body, code_pos + index * code_size)[0]
        for index in range(glyph_count)
    ]
    pos = code_pos + glyph_count * code_size

    if not has_layout:
        raise ValueError(f"font {font_id} has no layout advances")

    pos += 6  # ascent, descent, leading
    advances = [struct.unpack_from("<h", body, pos + index * 2)[0] / TWIPS_PER_PIXEL for index in range(glyph_count)]

    return {
        chr(code): Glyph(code, advance, contours)
        for code, advance, contours in zip(codes, advances, glyph_contours, strict=True)
    }


def contour_from_shape(shape_contour: dict) -> tuple[tuple[float, float], tuple[Segment, ...]]:
    segments = []
    for segment in shape_contour["segments"]:
        if segment["type"] == "line":
            segments.append(Segment("line", tuple(segment["to"])))
        else:
            segments.append(Segment("quad", tuple(segment["to"]), tuple(segment["control"])))
    return tuple(shape_contour["start"]), tuple(segments)


def transform_point(point: tuple[float, float], start_x: float) -> tuple[float, float]:
    scale = TEXT_HEIGHT * TWIPS_PER_PIXEL / EM_UNITS
    return (start_x + point[0] * scale, BASELINE_Y + point[1] * scale)


def digit_contours(glyph: Glyph) -> tuple[tuple[tuple[float, float], tuple[Segment, ...]], ...]:
    scale = TEXT_HEIGHT * TWIPS_PER_PIXEL / EM_UNITS
    start_x = FIELD_CENTER_X - glyph.advance * scale * 0.5
    contours = []
    for start, segments in glyph.contours:
        contours.append(
            (
                transform_point(start, start_x),
                tuple(
                    Segment(
                        segment.kind,
                        transform_point(segment.to, start_x),
                        transform_point(segment.control, start_x) if segment.control else None,
                    )
                    for segment in segments
                ),
            )
        )
    return tuple(contours)


def centipx(value: float) -> int:
    return round(value * 100.0)


def point_expr(point: tuple[float, float]) -> str:
    return f"[{centipx(point[0])}, {centipx(point[1])}]"


def rust_array(values: Iterable[int]) -> str:
    return "[" + ", ".join(str(value) for value in values) + "]"


def segment_expr(segment: Segment) -> str:
    if segment.kind == "line":
        return f"RoundNumberSegment::Line({point_expr(segment.to)})"
    return (
        "RoundNumberSegment::Quad { "
        f"control: {point_expr(segment.control or (0.0, 0.0))}, "
        f"to: {point_expr(segment.to)} "
        "}"
    )


def generate_module(glyphs: dict[str, Glyph]) -> str:
    output = [
        "// Generated from gravity_arcade.swf DefineEditText 154 and DefineFont2 153.",
        "// Coordinates are stored as SWF text-local centipixels; divide by 100 for pixels.",
        "",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq)]",
        "pub enum RoundNumberSegment {",
        "    Line([i16; 2]),",
        "    Quad { control: [i16; 2], to: [i16; 2] },",
        "}",
        "",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq)]",
        "pub struct RoundNumberContour {",
        "    pub(super) start: [i16; 2],",
        "    pub(super) segments: &'static [RoundNumberSegment],",
        "}",
        "",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq)]",
        "pub struct RoundNumberDefinition {",
        "    pub(super) text: &'static str,",
        "    pub(super) define_edit_text_id: u16,",
        "    pub(super) font_id: u16,",
        "    pub(super) color_rgb: [u8; 3],",
        "    pub(super) bounds_centipx: [i16; 4],",
        "    pub(super) contours: &'static [RoundNumberContour],",
        "}",
        "",
    ]

    bounds = [centipx(value) for value in FIELD_BOUNDS]
    for digit, name in DIGIT_NAMES.items():
        contours = digit_contours(glyphs[digit])
        for index, (_, segments) in enumerate(contours):
            output.append(f"const {name}_CONTOUR_{index}: [RoundNumberSegment; {len(segments)}] = [")
            for segment in segments:
                output.append(f"    {segment_expr(segment)},")
            output.append("];")
            output.append("")

        output.append(f"const {name}_CONTOURS: [RoundNumberContour; {len(contours)}] = [")
        for index, (start, _) in enumerate(contours):
            output.append("    RoundNumberContour {")
            output.append(f"        start: {point_expr(start)},")
            output.append(f"        segments: &{name}_CONTOUR_{index},")
            output.append("    },")
        output.append("];")
        output.append("")

        output.append(f"pub const {name}: RoundNumberDefinition = RoundNumberDefinition {{")
        output.append(f"    text: {json.dumps(digit)},")
        output.append(f"    define_edit_text_id: {EDIT_TEXT_ID},")
        output.append(f"    font_id: {FONT_ID},")
        output.append("    color_rgb: [255, 255, 255],")
        output.append(f"    bounds_centipx: {rust_array(bounds)},")
        output.append(f"    contours: &{name}_CONTOURS,")
        output.append("};")
        output.append("")

    return "\n".join(output)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("swf", type=Path)
    parser.add_argument("--out", type=Path, default=Path("src/round_number_texts.rs"))
    args = parser.parse_args()

    glyphs = None
    edit_text_found = False
    for tag_code, body in iter_tags(swf_body(args.swf)):
        if tag_code == 48 and struct.unpack_from("<H", body, 0)[0] == FONT_ID:
            glyphs = parse_font2(body)
        elif tag_code == 37 and struct.unpack_from("<H", body, 0)[0] == EDIT_TEXT_ID:
            bits = Bits(body, 2)
            bounds = tuple(read_rect(bits))
            if tuple(round(value, 2) for value in bounds) != FIELD_BOUNDS:
                raise ValueError(f"unexpected edit-text bounds {bounds}")
            edit_text_found = True

    if glyphs is None:
        raise SystemExit(f"missing DefineFont2 {FONT_ID}")
    if not edit_text_found:
        raise SystemExit(f"missing DefineEditText {EDIT_TEXT_ID}")
    missing = set(DIGIT_NAMES) - set(glyphs)
    if missing:
        raise SystemExit(f"missing embedded round digit glyphs: {sorted(missing)}")

    args.out.write_text(generate_module(glyphs), encoding="utf-8")


if __name__ == "__main__":
    main()

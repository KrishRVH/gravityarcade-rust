#!/usr/bin/env python3
"""Generate recovered Rust contours for the sprite-100 sponsor logo text layers."""

from __future__ import annotations

import argparse
import json
import struct
from dataclasses import dataclass
from pathlib import Path

from extract_shapes import Bits, build_fill_contours, iter_tags, read_rect, read_shape_records, swf_body


EM_UNITS = 1024.0
TWIPS_PER_PIXEL = 20.0
TEXTS = {
    33: ("DARK", "neodelight"),
    34: ("OLIVE", "neodelight"),
}


@dataclass(frozen=True)
class Matrix:
    sx: float
    sy: float
    r0: float
    r1: float
    tx: float
    ty: float


@dataclass(frozen=True)
class Segment:
    kind: str
    to: tuple[float, float]
    control: tuple[float, float] | None = None


@dataclass(frozen=True)
class Glyph:
    contours: tuple[tuple[tuple[float, float], tuple[Segment, ...]], ...]


@dataclass(frozen=True)
class TextRun:
    font_id: int
    color_rgb: tuple[int, int, int]
    x: float
    y: float
    height: float
    glyphs: tuple[tuple[int, float], ...]


@dataclass(frozen=True)
class TextDefinition:
    text_id: int
    bounds: tuple[float, float, float, float]
    matrix: Matrix
    runs: tuple[TextRun, ...]


def read_matrix(bits: Bits) -> Matrix:
    sx = 1.0
    sy = 1.0
    r0 = 0.0
    r1 = 0.0
    if bits.ub(1):
        nbits = bits.ub(5)
        sx = bits.fb(nbits)
        sy = bits.fb(nbits)
    if bits.ub(1):
        nbits = bits.ub(5)
        r0 = bits.fb(nbits)
        r1 = bits.fb(nbits)
    nbits = bits.ub(5)
    tx = bits.sb(nbits) / TWIPS_PER_PIXEL
    ty = bits.sb(nbits) / TWIPS_PER_PIXEL
    bits.align()
    return Matrix(sx, sy, r0, r1, tx, ty)


def parse_define_font(body: bytes) -> tuple[int, tuple[Glyph, ...]]:
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
        contours = build_fill_contours(records, [{"type": "solid", "color": "#000000"}])
        glyphs.append(Glyph(tuple(contour_from_shape(contour) for contour in contours)))
    return font_id, tuple(glyphs)


def contour_from_shape(shape_contour: dict) -> tuple[tuple[float, float], tuple[Segment, ...]]:
    segments = []
    for segment in shape_contour["segments"]:
        if segment["type"] == "line":
            segments.append(Segment("line", tuple(segment["to"])))
        else:
            segments.append(Segment("quad", tuple(segment["to"]), tuple(segment["control"])))
    return tuple(shape_contour["start"]), tuple(segments)


def parse_define_text(body: bytes) -> TextDefinition:
    text_id = struct.unpack_from("<H", body, 0)[0]
    bits = Bits(body, 2)
    bounds = tuple(read_rect(bits))
    matrix = read_matrix(bits)
    bits.align()
    glyph_bits = bits.data[bits.byte]
    advance_bits = bits.data[bits.byte + 1]
    bits.byte += 2

    active_font = 0
    active_color = (0, 0, 0)
    active_x = 0.0
    active_y = 0.0
    active_height = 0.0
    runs = []
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
            active_color = tuple(bits.data[bits.byte : bits.byte + 3])
            bits.byte += 3
        if flags & 0x01:
            active_x = struct.unpack_from("<h", bits.data, bits.byte)[0] / TWIPS_PER_PIXEL
            bits.byte += 2
        if flags & 0x02:
            active_y = struct.unpack_from("<h", bits.data, bits.byte)[0] / TWIPS_PER_PIXEL
            bits.byte += 2
        if flags & 0x08:
            active_height = struct.unpack_from("<H", bits.data, bits.byte)[0] / TWIPS_PER_PIXEL
            bits.byte += 2

        bits.align()
        glyph_count = bits.data[bits.byte]
        bits.byte += 1
        glyphs = []
        for _ in range(glyph_count):
            glyphs.append((bits.ub(glyph_bits), bits.sb(advance_bits) / TWIPS_PER_PIXEL))
        runs.append(TextRun(active_font, active_color, active_x, active_y, active_height, tuple(glyphs)))
    return TextDefinition(text_id, bounds, matrix, tuple(runs))


def apply_matrix(matrix: Matrix, point: tuple[float, float]) -> tuple[float, float]:
    x, y = point
    return (
        matrix.tx + matrix.sx * x + matrix.r1 * y,
        matrix.ty + matrix.r0 * x + matrix.sy * y,
    )


def transform_point(
    point: tuple[float, float],
    text: TextDefinition,
    run: TextRun,
    pen_x: float,
) -> tuple[float, float]:
    scale = run.height * TWIPS_PER_PIXEL / EM_UNITS
    return apply_matrix(text.matrix, (run.x + pen_x + point[0] * scale, run.y + point[1] * scale))


def text_contours(text: TextDefinition, fonts: dict[int, tuple[Glyph, ...]]):
    contours = []
    for run in text.runs:
        pen_x = 0.0
        for glyph_index, advance in run.glyphs:
            for start, segments in fonts[run.font_id][glyph_index].contours:
                contours.append(
                    (
                        transform_point(start, text, run, pen_x),
                        tuple(
                            Segment(
                                segment.kind,
                                transform_point(segment.to, text, run, pen_x),
                                transform_point(segment.control, text, run, pen_x)
                                if segment.control
                                else None,
                            )
                            for segment in segments
                        ),
                    )
                )
            pen_x += advance
    return tuple(contours)


def centipx(value: float) -> int:
    return round(value * 100.0)


def point_expr(point: tuple[float, float]) -> str:
    return f"[{centipx(point[0])}, {centipx(point[1])}]"


def rust_array(values) -> str:
    return "[" + ", ".join(str(value) for value in values) + "]"


def segment_expr(segment: Segment) -> str:
    if segment.kind == "line":
        return f"SponsorLogoTextSegment::Line({point_expr(segment.to)})"
    return (
        "SponsorLogoTextSegment::Quad { "
        f"control: {point_expr(segment.control or (0.0, 0.0))}, "
        f"to: {point_expr(segment.to)} "
        "}"
    )


def generate_module(texts: dict[int, TextDefinition], fonts: dict[int, tuple[Glyph, ...]]) -> str:
    output = [
        "// Generated from gravity_arcade.swf sponsor-logo DefineText tags.",
        "// Coordinates are stored as SWF text-local centipixels; divide by 100 for pixels.",
        "",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq)]",
        "pub enum SponsorLogoTextSegment {",
        "    Line([i16; 2]),",
        "    Quad { control: [i16; 2], to: [i16; 2] },",
        "}",
        "",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq)]",
        "pub struct SponsorLogoTextContour {",
        "    pub(super) start: [i16; 2],",
        "    pub(super) segments: &'static [SponsorLogoTextSegment],",
        "}",
        "",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq)]",
        "pub struct SponsorLogoTextDefinition {",
        "    pub(super) text: &'static str,",
        "    pub(super) define_text_id: u16,",
        "    pub(super) font_ids: &'static [u16],",
        "    pub(super) color_rgb: [u8; 3],",
        "    pub(super) bounds_centipx: [i16; 4],",
        "    pub(super) contours: &'static [SponsorLogoTextContour],",
        "}",
        "",
    ]

    for text_id, (name, label) in TEXTS.items():
        text = texts[text_id]
        font_ids = tuple(dict.fromkeys(run.font_id for run in text.runs))
        color = text.runs[0].color_rgb
        bounds = [centipx(value) for value in text.bounds]
        contours = text_contours(text, fonts)
        output.append(f"const {name}_FONT_IDS: [u16; {len(font_ids)}] = {rust_array(font_ids)};")
        output.append("")
        for index, (_, segments) in enumerate(contours):
            output.append(f"const {name}_CONTOUR_{index}: [SponsorLogoTextSegment; {len(segments)}] = [")
            for segment in segments:
                output.append(f"    {segment_expr(segment)},")
            output.append("];")
            output.append("")
        output.append(f"const {name}_CONTOURS: [SponsorLogoTextContour; {len(contours)}] = [")
        for index, (start, _) in enumerate(contours):
            output.append("    SponsorLogoTextContour {")
            output.append(f"        start: {point_expr(start)},")
            output.append(f"        segments: &{name}_CONTOUR_{index},")
            output.append("    },")
        output.append("];")
        output.append("")
        output.append(
            f"pub const {name}: SponsorLogoTextDefinition = SponsorLogoTextDefinition {{"
        )
        output.append(f"    text: {json.dumps(label)},")
        output.append(f"    define_text_id: {text_id},")
        output.append(f"    font_ids: &{name}_FONT_IDS,")
        output.append(f"    color_rgb: {rust_array(color)},")
        output.append(f"    bounds_centipx: {rust_array(bounds)},")
        output.append(f"    contours: &{name}_CONTOURS,")
        output.append("};")
        output.append("")
    return "\n".join(output)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("swf", type=Path)
    parser.add_argument("--out", type=Path, default=Path("src/sponsor_logo_texts.rs"))
    args = parser.parse_args()

    fonts = {}
    texts = {}
    for tag_code, body in iter_tags(swf_body(args.swf)):
        if tag_code == 10:
            font_id, glyphs = parse_define_font(body)
            fonts[font_id] = glyphs
        elif tag_code == 11:
            text_id = struct.unpack_from("<H", body, 0)[0]
            if text_id in TEXTS:
                texts[text_id] = parse_define_text(body)

    missing = set(TEXTS) - set(texts)
    if missing:
        raise SystemExit(f"missing DefineText ids: {sorted(missing)}")

    args.out.write_text(generate_module(texts, fonts), encoding="utf-8")


if __name__ == "__main__":
    main()

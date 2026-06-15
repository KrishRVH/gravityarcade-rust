#!/usr/bin/env python3
"""Generate a checked-in Rust contour module for one SWF DefineText record."""

from __future__ import annotations

import argparse
import json
import struct
from pathlib import Path

from extract_announce_texts import (
    Contour,
    TextDefinition,
    centipx,
    iter_tags,
    parse_define_font,
    parse_define_text,
    rust_array,
    swf_body,
    text_contours,
    write_segment,
)


def generate_module(
    *,
    text: TextDefinition,
    fonts: dict[int, tuple[tuple[Contour, ...], ...]],
    label: str,
    prefix: str,
    font_id: int,
    header: str,
) -> str:
    contours = text_contours(text, fonts)
    output = [
        header,
        "// Coordinates are stored as SWF text-local centipixels; divide by 100 for pixels.",
        "",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq)]",
        f"pub enum {prefix}Segment {{",
        "    Line([i16; 2]),",
        "    Quad { control: [i16; 2], to: [i16; 2] },",
        "}",
        "",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq)]",
        f"pub struct {prefix}Contour {{",
        "    pub(super) start: [i16; 2],",
        f"    pub(super) segments: &'static [{prefix}Segment],",
        "}",
        "",
        f"pub const TEXT: &str = {json.dumps(label)};",
        f"pub const FONT_ID: u16 = {font_id};",
        f"pub const DEFINE_TEXT_ID: u16 = {text.text_id};",
        f"pub const BOUNDS_CENTIPX: [i16; 4] = {rust_array([centipx(value) for value in text.bounds])};",
        "",
    ]

    for index, contour in enumerate(contours):
        output.append(f"const CONTOUR_{index}: [{prefix}Segment; {len(contour.segments)}] = [")
        for segment in contour.segments:
            output.append(f"    {write_segment(prefix, segment)},")
        output.append("];")
        output.append("")

    output.append(f"pub const CONTOURS: [{prefix}Contour; {len(contours)}] = [")
    for index, contour in enumerate(contours):
        output.append(f"    {prefix}Contour {{")
        output.append(f"        start: {rust_array([centipx(contour.start[0]), centipx(contour.start[1])])},")
        output.append(f"        segments: &CONTOUR_{index},")
        output.append("    },")
    output.append("];")

    return "\n".join(output)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("swf", type=Path)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--text-id", type=int, required=True)
    parser.add_argument("--font-id", type=int, required=True)
    parser.add_argument("--label", required=True)
    parser.add_argument("--prefix", required=True)
    parser.add_argument("--header", required=True)
    args = parser.parse_args()

    fonts: dict[int, tuple[tuple[Contour, ...], ...]] = {}
    text = None
    for tag_code, body in iter_tags(swf_body(args.swf)):
        if tag_code == 10:
            font_id, glyphs = parse_define_font(body)
            fonts[font_id] = glyphs
        elif tag_code == 11 and struct.unpack_from("<H", body, 0)[0] == args.text_id:
            text = parse_define_text(body)

    if args.font_id not in fonts:
        raise SystemExit(f"missing DefineFont {args.font_id}")
    if text is None:
        raise SystemExit(f"missing DefineText {args.text_id}")

    text_font_ids = {run.font_id for run in text.runs}
    if text_font_ids != {args.font_id}:
        raise SystemExit(f"DefineText {args.text_id} used unexpected fonts: {sorted(text_font_ids)}")

    args.out.write_text(
        generate_module(
            text=text,
            fonts=fonts,
            label=args.label,
            prefix=args.prefix,
            font_id=args.font_id,
            header=args.header,
        ),
        encoding="utf-8",
    )


if __name__ == "__main__":
    main()

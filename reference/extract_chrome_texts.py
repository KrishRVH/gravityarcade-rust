#!/usr/bin/env python3
"""Generate recovered Rust contours for chrome/header/footer DefineText tags."""

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


CHROME_TEXTS = {
    73: ("VERSION", "version 1.33 - (c) by :neokolor 2001-2 and Sir Isaac Newton 1686"),
    95: ("SPONSOR_UP", "to neodelight"),
    96: ("SPONSOR_OVER", "to neodelight"),
    97: ("SPONSOR_DOWN", "to neodelight"),
    120: ("BACK_UP", "back to menu"),
    121: ("BACK_OVER", "back to menu"),
    122: ("BACK_DOWN", "back to menu"),
}


def generate_module(
    texts: dict[int, TextDefinition],
    fonts: dict[int, tuple[tuple[Contour, ...], ...]],
) -> str:
    prefix = "ChromeText"
    output = [
        "// Generated from gravity_arcade.swf header/version DefineText tags.",
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
        "#[derive(Debug, Clone, Copy, PartialEq, Eq)]",
        f"pub struct {prefix}Definition {{",
        "    pub(super) text: &'static str,",
        "    pub(super) define_text_id: u16,",
        "    pub(super) font_ids: &'static [u16],",
        "    pub(super) color_rgb: [u8; 3],",
        "    pub(super) bounds_centipx: [i16; 4],",
        f"    pub(super) contours: &'static [{prefix}Contour],",
        "}",
        "",
    ]

    for text_id, (name, label) in CHROME_TEXTS.items():
        text = texts[text_id]
        font_ids = tuple(dict.fromkeys(run.font_id for run in text.runs))
        color = text.runs[0].color_rgb
        bounds = [centipx(value) for value in text.bounds]
        contours = text_contours(text, fonts)

        output.append(f"const {name}_FONT_IDS: [u16; {len(font_ids)}] = {rust_array(font_ids)};")
        output.append("")
        for index, contour in enumerate(contours):
            output.append(f"const {name}_CONTOUR_{index}: [{prefix}Segment; {len(contour.segments)}] = [")
            for segment in contour.segments:
                output.append(f"    {write_segment(prefix, segment)},")
            output.append("];")
            output.append("")

        output.append(f"const {name}_CONTOURS: [{prefix}Contour; {len(contours)}] = [")
        for index, contour in enumerate(contours):
            output.append(f"    {prefix}Contour {{")
            output.append(f"        start: {rust_array([centipx(contour.start[0]), centipx(contour.start[1])])},")
            output.append(f"        segments: &{name}_CONTOUR_{index},")
            output.append("    },")
        output.append("];")
        output.append("")

        output.append(f"pub const {name}: {prefix}Definition = {prefix}Definition {{")
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
    parser.add_argument("--out", type=Path, default=Path("src/chrome_texts.rs"))
    args = parser.parse_args()

    fonts: dict[int, tuple[tuple[Contour, ...], ...]] = {}
    texts: dict[int, TextDefinition] = {}
    for tag_code, body in iter_tags(swf_body(args.swf)):
        if tag_code == 10:
            font_id, glyphs = parse_define_font(body)
            fonts[font_id] = glyphs
        elif tag_code == 11:
            text_id = struct.unpack_from("<H", body, 0)[0]
            if text_id in CHROME_TEXTS:
                texts[text_id] = parse_define_text(body)

    missing = set(CHROME_TEXTS) - set(texts)
    if missing:
        raise SystemExit(f"missing DefineText ids: {sorted(missing)}")

    args.out.write_text(generate_module(texts, fonts), encoding="utf-8")


if __name__ == "__main__":
    main()

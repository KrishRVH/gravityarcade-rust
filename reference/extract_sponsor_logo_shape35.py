#!/usr/bin/env python3
"""Generate recovered Rust contours for sponsor-logo DefineShape 35."""

from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any

from extract_shapes import extract_shapes


SHAPE_ID = 35


def twips(value: float) -> int:
    return round(value * 20.0)


def point_expr(point: list[float]) -> str:
    return f"[{twips(point[0])}, {twips(point[1])}]"


def rust_array(values: list[int]) -> str:
    return "[" + ", ".join(str(value) for value in values) + "]"


def rgb_expr(color: str) -> str:
    return rust_array([int(color[offset : offset + 2], 16) for offset in (1, 3, 5)])


def segment_expr(segment: dict[str, Any]) -> str:
    if segment["type"] == "line":
        return f"Shape35Segment::Line({point_expr(segment['to'])})"
    return (
        "Shape35Segment::Quad { "
        f"control: {point_expr(segment['control'])}, "
        f"to: {point_expr(segment['to'])} "
        "}"
    )


def generate_module(shape: dict[str, Any]) -> str:
    output = [
        "// Generated from gravity_arcade.swf DefineShape 35 via reference/extract_shapes.py --contours.",
        "// Coordinates are stored in SWF twips; divide by 20 for stage-local pixels.",
        "",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq)]",
        "pub enum Shape35Segment {",
        "    Line([i16; 2]),",
        "    Quad { control: [i16; 2], to: [i16; 2] },",
        "}",
        "",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq)]",
        "pub struct Shape35Contour {",
        "    pub(super) start: [i16; 2],",
        "    pub(super) segments: &'static [Shape35Segment],",
        "}",
        "",
        f"pub const BOUNDS_TWIPS: [i16; 4] = {rust_array([twips(value) for value in shape['bounds']])};",
        f"pub const FILL_RGB: [u8; 3] = {rgb_expr(shape['fills'][0]['color'])};",
        "",
    ]

    contours = shape["contours"]
    for index, contour in enumerate(contours):
        output.append(
            f"const CONTOUR_{index}: [Shape35Segment; {len(contour['segments'])}] = ["
        )
        for segment in contour["segments"]:
            output.append(f"    {segment_expr(segment)},")
        output.append("];")
        output.append("")

    output.append(f"pub const CONTOURS: [Shape35Contour; {len(contours)}] = [")
    for index, contour in enumerate(contours):
        output.append("    Shape35Contour {")
        output.append(f"        start: {point_expr(contour['start'])},")
        output.append(f"        segments: &CONTOUR_{index},")
        output.append("    },")
    output.append("];")

    return "\n".join(output)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("swf", type=Path)
    parser.add_argument("--out", type=Path, default=Path("src/sponsor_logo_shape35.rs"))
    args = parser.parse_args()

    shapes = extract_shapes(args.swf, {SHAPE_ID}, include_contours=True)
    if len(shapes) != 1:
        raise SystemExit(f"expected DefineShape {SHAPE_ID}, got {len(shapes)} matches")

    args.out.write_text(generate_module(shapes[0]), encoding="utf-8")


if __name__ == "__main__":
    main()

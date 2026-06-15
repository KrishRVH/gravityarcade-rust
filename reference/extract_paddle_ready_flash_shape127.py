#!/usr/bin/env python3
"""Generate recovered Rust contours for paddle ready-flash DefineShape 127."""

from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any

from extract_shapes import extract_shapes


SHAPE_ID = 127


def rust_float(value: float) -> str:
    if value == 0:
        return "0.0"
    out = f"{value:.6f}".rstrip("0").rstrip(".")
    if "." not in out:
        out += ".0"
    return out


def twips(value: float) -> int:
    return round(value * 20.0)


def point_expr(point: list[float]) -> str:
    return f"SwfPoint::new({rust_float(point[0])}, {rust_float(point[1])})"


def rust_array(values: list[int]) -> str:
    return "[" + ", ".join(str(value) for value in values) + "]"


def rgba_expr(color: str) -> str:
    rgb, alpha = color.split("@")
    return rust_array(
        [*(int(rgb[offset : offset + 2], 16) for offset in (1, 3, 5)), int(alpha)]
    )


def segment_expr(segment: dict[str, Any]) -> str:
    if segment["type"] == "line":
        return f"SwfPathSegment::Line({point_expr(segment['to'])})"
    return (
        "SwfPathSegment::Quad { "
        f"control: {point_expr(segment['control'])}, "
        f"to: {point_expr(segment['to'])} "
        "}"
    )


def generate_module(shape: dict[str, Any]) -> str:
    contour = shape["contours"][0]
    fill = shape["fills"][0]
    bounds = shape["bounds"]
    output = [
        "// Generated from gravity_arcade.swf DefineShape 127 via reference/extract_shapes.py --contours.",
        "// Coordinates are stored as SWF shape-local pixels.",
        "",
        "use super::{PaddleReadyFlashShapeDefinition, SwfBounds, SwfPathSegment, SwfPoint};",
        "",
        "pub const FLATTEN_STEPS: u8 = 4;",
        "#[cfg(test)]",
        f"pub const DEFINE_SHAPE_ID: u16 = {SHAPE_ID};",
        "#[cfg(test)]",
        f"pub const BOUNDS_TWIPS: [i16; 4] = {rust_array([twips(value) for value in bounds])};",
        f"pub const FILL_RGBA: [u8; 4] = {rgba_expr(fill['color'])};",
        "",
        f"const SEGMENTS: [SwfPathSegment; {len(contour['segments'])}] = [",
    ]
    for segment in contour["segments"]:
        output.append(f"    {segment_expr(segment)},")
    output += [
        "];",
        "",
        "pub const SHAPE: PaddleReadyFlashShapeDefinition = PaddleReadyFlashShapeDefinition {",
        "    bounds: SwfBounds {",
        f"        x_min: {rust_float(bounds[0])},",
        f"        x_max: {rust_float(bounds[1])},",
        f"        y_min: {rust_float(bounds[2])},",
        f"        y_max: {rust_float(bounds[3])},",
        "    },",
        f"    start: {point_expr(contour['start'])},",
        "    segments: &SEGMENTS,",
        "};",
    ]
    return "\n".join(output)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("swf", type=Path)
    parser.add_argument("--out", type=Path, default=Path("src/paddle_ready_flash_shape127.rs"))
    args = parser.parse_args()

    shapes = extract_shapes(args.swf, {SHAPE_ID}, include_contours=True)
    if len(shapes) != 1:
        raise SystemExit(f"expected DefineShape {SHAPE_ID}, got {len(shapes)} matches")

    args.out.write_text(generate_module(shapes[0]), encoding="utf-8")


if __name__ == "__main__":
    main()

#!/usr/bin/env python3
"""Generate recovered Rust contours for goal-line DefineShape 84 and 87."""

from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any

from extract_shapes import extract_shapes


RED_SHAPE_ID = 84
BLUE_SHAPE_ID = 87


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


def rgb_expr(color: str) -> str:
    return rust_array([int(color[offset : offset + 2], 16) for offset in (1, 3, 5)])


def segment_expr(segment: dict[str, Any]) -> str:
    if segment["type"] == "line":
        return f"SwfPathSegment::Line({point_expr(segment['to'])})"
    return (
        "SwfPathSegment::Quad { "
        f"control: {point_expr(segment['control'])}, "
        f"to: {point_expr(segment['to'])} "
        "}"
    )


def generate_module(shapes: list[dict[str, Any]]) -> str:
    by_id = {shape["id"]: shape for shape in shapes}
    red = by_id[RED_SHAPE_ID]
    blue = by_id[BLUE_SHAPE_ID]
    red_contour = red["contours"][0]
    blue_contour = blue["contours"][0]
    if red["bounds"] != blue["bounds"]:
        raise SystemExit("expected matching red/blue goal-line bounds")
    if red_contour["start"] != blue_contour["start"]:
        raise SystemExit("expected matching red/blue goal-line contour starts")
    if red_contour["segments"] != blue_contour["segments"]:
        raise SystemExit("expected matching red/blue goal-line segments")

    output = [
        "// Generated from gravity_arcade.swf DefineShape 84 and 87 via reference/extract_shapes.py --contours.",
        "// Coordinates are stored as SWF shape-local pixels.",
        "",
        "use super::{GoalLineShapeDefinition, SwfBounds, SwfPathSegment, SwfPoint};",
        "",
        "#[cfg(test)]",
        f"pub const RED_DEFINE_SHAPE_ID: u16 = {RED_SHAPE_ID};",
        "#[cfg(test)]",
        f"pub const BLUE_DEFINE_SHAPE_ID: u16 = {BLUE_SHAPE_ID};",
        "#[cfg(test)]",
        f"pub const BOUNDS_TWIPS: [i16; 4] = {rust_array([twips(value) for value in red['bounds']])};",
        f"pub const RED_FILL_RGB: [u8; 3] = {rgb_expr(red['fills'][0]['color'])};",
        f"pub const BLUE_FILL_RGB: [u8; 3] = {rgb_expr(blue['fills'][0]['color'])};",
        "",
        f"const SEGMENTS: [SwfPathSegment; {len(red_contour['segments'])}] = [",
    ]
    for segment in red_contour["segments"]:
        output.append(f"    {segment_expr(segment)},")
    output += [
        "];",
        "",
        "pub const SHAPE: GoalLineShapeDefinition = GoalLineShapeDefinition {",
        "    bounds: SwfBounds {",
        f"        x_min: {rust_float(red['bounds'][0])},",
        f"        x_max: {rust_float(red['bounds'][1])},",
        f"        y_min: {rust_float(red['bounds'][2])},",
        f"        y_max: {rust_float(red['bounds'][3])},",
        "    },",
        f"    start: {point_expr(red_contour['start'])},",
        "    segments: &SEGMENTS,",
        "};",
    ]
    return "\n".join(output)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("swf", type=Path)
    parser.add_argument("--out", type=Path, default=Path("src/goal_line_shapes.rs"))
    args = parser.parse_args()

    shapes = extract_shapes(args.swf, {RED_SHAPE_ID, BLUE_SHAPE_ID}, include_contours=True)
    found = {shape["id"] for shape in shapes}
    expected = {RED_SHAPE_ID, BLUE_SHAPE_ID}
    if found != expected:
        raise SystemExit(f"missing goal-line DefineShape ids: {sorted(expected - found)}")

    args.out.write_text(generate_module(shapes), encoding="utf-8")


if __name__ == "__main__":
    main()

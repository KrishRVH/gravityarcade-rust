#!/usr/bin/env python3
"""Generate recovered Rust contours for side/score marker DefineShape records."""

from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any

from extract_shapes import extract_shapes


RED_FILL_SHAPE_ID = 92
OUTLINE_SHAPE_ID = 93
BLUE_FILL_SHAPE_ID = 94


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


def edge_segment_expr(edge: dict[str, Any]) -> str:
    if edge["type"] == "line":
        return f"SwfPathSegment::Line({point_expr(edge['to'])})"
    return (
        "SwfPathSegment::Quad { "
        f"control: {point_expr(edge['control'])}, "
        f"to: {point_expr(edge['to'])} "
        "}"
    )


def shape_bounds_fields(bounds: list[float]) -> list[str]:
    return [
        "    bounds: SwfBounds {",
        f"        x_min: {rust_float(bounds[0])},",
        f"        x_max: {rust_float(bounds[1])},",
        f"        y_min: {rust_float(bounds[2])},",
        f"        y_max: {rust_float(bounds[3])},",
        "    },",
    ]


def write_segments(output: list[str], name: str, segments: list[dict[str, Any]]) -> None:
    output.append(f"const {name}_SEGMENTS: [SwfPathSegment; {len(segments)}] = [")
    for segment in segments:
        output.append(f"    {edge_segment_expr(segment)},")
    output += [
        "];",
        "",
    ]


def write_shape(
    output: list[str],
    const_name: str,
    shape: dict[str, Any],
    start: list[float],
    segment_name: str,
) -> None:
    output += [
        f"pub const {const_name}: SideMarkerShapeDefinition = SideMarkerShapeDefinition {{",
        *shape_bounds_fields(shape["bounds"]),
        "    contour: SideMarkerContour {",
        f"        start: {point_expr(start)},",
        f"        segments: &{segment_name}_SEGMENTS,",
        "    },",
        "};",
    ]


def shape_edges(shape: dict[str, Any]) -> list[dict[str, Any]]:
    return [record for record in shape["records"] if record["type"] in ("line", "curve")]


def contour_for_single_fill(shape: dict[str, Any]) -> dict[str, Any]:
    contours = shape["contours"]
    if len(contours) != 1:
        raise SystemExit(f"expected one fill contour for shape {shape['id']}, got {len(contours)}")
    contour = contours[0]
    if not contour["closed"]:
        raise SystemExit(f"expected closed fill contour for shape {shape['id']}")
    return contour


def generate_module(shapes: list[dict[str, Any]]) -> str:
    by_id = {shape["id"]: shape for shape in shapes}
    red = by_id[RED_FILL_SHAPE_ID]
    outline = by_id[OUTLINE_SHAPE_ID]
    blue = by_id[BLUE_FILL_SHAPE_ID]
    red_contour = contour_for_single_fill(red)
    blue_contour = contour_for_single_fill(blue)
    outline_edges = shape_edges(outline)
    if not outline_edges:
        raise SystemExit("expected line edges for marker outline shape")

    output = [
        "// Generated from gravity_arcade.swf DefineShape 92, 93, and 94 via reference/extract_shapes.py --contours.",
        "// Coordinates are stored as SWF shape-local pixels.",
        "",
        "use super::{SideMarkerContour, SideMarkerShapeDefinition, SwfBounds, SwfPathSegment, SwfPoint};",
        "",
        "pub const FLATTEN_STEPS: u8 = 4;",
        "#[cfg(test)]",
        f"pub const RED_FILL_DEFINE_SHAPE_ID: u16 = {RED_FILL_SHAPE_ID};",
        "#[cfg(test)]",
        f"pub const OUTLINE_DEFINE_SHAPE_ID: u16 = {OUTLINE_SHAPE_ID};",
        "#[cfg(test)]",
        f"pub const BLUE_FILL_DEFINE_SHAPE_ID: u16 = {BLUE_FILL_SHAPE_ID};",
        "#[cfg(test)]",
        f"pub const RED_FILL_BOUNDS_TWIPS: [i16; 4] = {rust_array([twips(value) for value in red['bounds']])};",
        "#[cfg(test)]",
        f"pub const OUTLINE_BOUNDS_TWIPS: [i16; 4] = {rust_array([twips(value) for value in outline['bounds']])};",
        "#[cfg(test)]",
        f"pub const BLUE_FILL_BOUNDS_TWIPS: [i16; 4] = {rust_array([twips(value) for value in blue['bounds']])};",
        f"pub const RED_FILL_RGB: [u8; 3] = {rgb_expr(red['fills'][0]['color'])};",
        f"pub const OUTLINE_RGB: [u8; 3] = {rgb_expr(outline['lines'][0]['color'])};",
        f"pub const BLUE_FILL_RGB: [u8; 3] = {rgb_expr(blue['fills'][0]['color'])};",
        f"pub const OUTLINE_LINE_WIDTH: f32 = {rust_float(outline['lines'][0]['width'])};",
        "",
    ]

    write_segments(output, "RED_FILL", red_contour["segments"])
    write_segments(output, "OUTLINE", outline_edges)
    write_segments(output, "BLUE_FILL", blue_contour["segments"])

    write_shape(output, "RED_FILL_SHAPE", red, red_contour["start"], "RED_FILL")
    output.append("")
    write_shape(output, "OUTLINE_SHAPE", outline, outline_edges[0]["from"], "OUTLINE")
    output.append("")
    write_shape(output, "BLUE_FILL_SHAPE", blue, blue_contour["start"], "BLUE_FILL")
    return "\n".join(output)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("swf", type=Path)
    parser.add_argument("--out", type=Path, default=Path("src/side_marker_shapes.rs"))
    args = parser.parse_args()

    expected = {RED_FILL_SHAPE_ID, OUTLINE_SHAPE_ID, BLUE_FILL_SHAPE_ID}
    shapes = extract_shapes(args.swf, expected, include_contours=True)
    found = {shape["id"] for shape in shapes}
    if found != expected:
        raise SystemExit(f"missing side-marker DefineShape ids: {sorted(expected - found)}")

    args.out.write_text(generate_module(shapes), encoding="utf-8")


if __name__ == "__main__":
    main()

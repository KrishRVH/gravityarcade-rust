#!/usr/bin/env python3
"""Generate recovered Rust contours for help-control DefineShape records."""

from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any

from extract_shapes import extract_shapes


KEYCAP_SHAPE_ID = 105
ARROW_SHAPE_ID = 110


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


def write_segments(output: list[str], name: str, segments: list[dict[str, Any]]) -> None:
    output.append(f"const {name}_SEGMENTS: [SwfPathSegment; {len(segments)}] = [")
    for segment in segments:
        output.append(f"    {edge_segment_expr(segment)},")
    output += [
        "];",
        "",
    ]


def write_contour(output: list[str], field: str, name: str, start: list[float]) -> None:
    output += [
        f"    {field}: HelpControlContour {{",
        f"        start: {point_expr(start)},",
        f"        segments: &{name}_SEGMENTS,",
        "    },",
    ]


def shape_bounds_fields(bounds: list[float]) -> list[str]:
    return [
        "    bounds: SwfBounds {",
        f"        x_min: {rust_float(bounds[0])},",
        f"        x_max: {rust_float(bounds[1])},",
        f"        y_min: {rust_float(bounds[2])},",
        f"        y_max: {rust_float(bounds[3])},",
        "    },",
    ]


def shape_edges(shape: dict[str, Any]) -> list[dict[str, Any]]:
    return [record for record in shape["records"] if record["type"] in ("line", "curve")]


def generate_module(shapes: list[dict[str, Any]]) -> str:
    by_id = {shape["id"]: shape for shape in shapes}
    keycap = by_id[KEYCAP_SHAPE_ID]
    arrow = by_id[ARROW_SHAPE_ID]
    keycap_contours = {contour["fill"]: contour for contour in keycap["contours"]}
    arrow_contour = arrow["contours"][0]
    keycap_edges = shape_edges(keycap)
    keycap_top_edges = keycap_edges[0:3]
    keycap_inner_edges = keycap_edges[3:14]
    keycap_outer_edges = keycap_edges[14:]

    output = [
        "// Generated from gravity_arcade.swf DefineShape 105 and 110 via reference/extract_shapes.py --contours.",
        "// Coordinates are stored as SWF shape-local pixels.",
        "",
        "use super::{HelpArrowShapeDefinition, HelpControlContour, HelpKeycapShapeDefinition, SwfBounds, SwfPathSegment, SwfPoint};",
        "",
        "pub const FLATTEN_STEPS: u8 = 4;",
        "#[cfg(test)]",
        f"pub const KEYCAP_DEFINE_SHAPE_ID: u16 = {KEYCAP_SHAPE_ID};",
        "#[cfg(test)]",
        f"pub const KEYCAP_BOUNDS_TWIPS: [i16; 4] = {rust_array([twips(value) for value in keycap['bounds']])};",
        f"pub const KEYCAP_FILL_RGB: [u8; 3] = {rgb_expr(keycap['fills'][0]['color'])};",
        f"pub const KEYCAP_SHADOW_RGB: [u8; 3] = {rgb_expr(keycap['fills'][1]['color'])};",
        f"pub const KEYCAP_LINE_RGB: [u8; 3] = {rgb_expr(keycap['lines'][0]['color'])};",
        f"pub const KEYCAP_LINE_WIDTH: f32 = {rust_float(keycap['lines'][0]['width'])};",
        "#[cfg(test)]",
        f"pub const ARROW_DEFINE_SHAPE_ID: u16 = {ARROW_SHAPE_ID};",
        "#[cfg(test)]",
        f"pub const ARROW_BOUNDS_TWIPS: [i16; 4] = {rust_array([twips(value) for value in arrow['bounds']])};",
        f"pub const ARROW_FILL_RGB: [u8; 3] = {rgb_expr(arrow['fills'][0]['color'])};",
        "",
    ]

    write_segments(output, "KEYCAP_FILL", keycap_contours[1]["segments"])
    write_segments(output, "KEYCAP_SHADOW", keycap_contours[2]["segments"])
    write_segments(output, "KEYCAP_TOP_STROKE", keycap_top_edges)
    write_segments(output, "KEYCAP_INNER_STROKE", keycap_inner_edges)
    write_segments(output, "KEYCAP_OUTER_STROKE", keycap_outer_edges)
    write_segments(output, "ARROW_FILL", arrow_contour["segments"])

    output += [
        "pub const KEYCAP_SHAPE: HelpKeycapShapeDefinition = HelpKeycapShapeDefinition {",
        *shape_bounds_fields(keycap["bounds"]),
    ]
    write_contour(output, "fill", "KEYCAP_FILL", keycap_contours[1]["start"])
    write_contour(output, "shadow", "KEYCAP_SHADOW", keycap_contours[2]["start"])
    write_contour(output, "top_stroke", "KEYCAP_TOP_STROKE", keycap_top_edges[0]["from"])
    write_contour(output, "inner_stroke", "KEYCAP_INNER_STROKE", keycap_inner_edges[0]["from"])
    write_contour(output, "outer_stroke", "KEYCAP_OUTER_STROKE", keycap_outer_edges[0]["from"])
    output += [
        "};",
        "",
        "pub const ARROW_SHAPE: HelpArrowShapeDefinition = HelpArrowShapeDefinition {",
        *shape_bounds_fields(arrow["bounds"]),
    ]
    write_contour(output, "fill", "ARROW_FILL", arrow_contour["start"])
    output.append("};")
    return "\n".join(output)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("swf", type=Path)
    parser.add_argument("--out", type=Path, default=Path("src/help_control_shapes.rs"))
    args = parser.parse_args()

    shapes = extract_shapes(
        args.swf,
        {KEYCAP_SHAPE_ID, ARROW_SHAPE_ID},
        include_contours=True,
    )
    found = {shape["id"] for shape in shapes}
    expected = {KEYCAP_SHAPE_ID, ARROW_SHAPE_ID}
    if found != expected:
        raise SystemExit(f"missing help-control DefineShape ids: {sorted(expected - found)}")

    args.out.write_text(generate_module(shapes), encoding="utf-8")


if __name__ == "__main__":
    main()

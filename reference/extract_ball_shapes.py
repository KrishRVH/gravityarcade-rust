#!/usr/bin/env python3
"""Generate recovered Rust contours for ball radial DefineShape records."""

from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any

from extract_shapes import extract_shapes


SHAPES = {
    2: "BALL_GLOW",
    4: "BALL_DIE_RING",
    9: "BALL_FIRE",
}


def twips(value: float) -> int:
    return round(value * 20.0)


def rust_float(value: float) -> str:
    if value == 0:
        return "0.0"
    out = f"{value:.6f}".rstrip("0").rstrip(".")
    if "." not in out:
        out += ".0"
    return out


def point_expr(point: list[float]) -> str:
    return f"SwfPoint::new({rust_float(point[0])}, {rust_float(point[1])})"


def rust_array(values: list[int]) -> str:
    return "[" + ", ".join(str(value) for value in values) + "]"


def gradient_expr(fill: dict[str, Any]) -> str:
    records = []
    for record in fill["records"]:
        color, alpha = record["color"].split("@")
        rgb = [int(color[offset : offset + 2], 16) for offset in (1, 3, 5)]
        records.append([record["ratio"], *rgb, int(alpha)])
    return "[" + ", ".join(rust_array(record) for record in records) + "]"


def segment_expr(segment: dict[str, Any]) -> str:
    if segment["type"] == "line":
        return f"SwfPathSegment::Line({point_expr(segment['to'])})"
    return (
        "SwfPathSegment::Quad { "
        f"control: {point_expr(segment['control'])}, "
        f"to: {point_expr(segment['to'])} "
        "}"
    )


def generate_shape(output: list[str], shape: dict[str, Any]) -> None:
    name = SHAPES[shape["id"]]
    contour = shape["contours"][0]
    segments = contour["segments"]
    bounds = shape["bounds"]
    base_radius = max(abs(bounds[0]), abs(bounds[1]), abs(bounds[2]), abs(bounds[3]))
    fill = shape["fills"][0]

    output += [
        "#[cfg(test)]",
        f"pub const {name}_DEFINE_SHAPE_ID: u16 = {shape['id']};",
        "#[cfg(test)]",
        f"pub const {name}_BOUNDS_TWIPS: [i16; 4] = {rust_array([twips(value) for value in bounds])};",
        f"pub const {name}_GRADIENT_RGBA: [[u8; 5]; {len(fill['records'])}] = {gradient_expr(fill)};",
        "",
        f"const {name}_SEGMENTS: [SwfPathSegment; {len(segments)}] = [",
    ]
    for segment in segments:
        output.append(f"    {segment_expr(segment)},")
    output += [
        "];",
        "",
        f"pub const {name}_SHAPE: RadialGradientShape = RadialGradientShape {{",
        f"    base_radius: {rust_float(base_radius)},",
        f"    start: {point_expr(contour['start'])},",
        f"    segments: &{name}_SEGMENTS,",
        "};",
        "",
    ]


def generate_module(shapes: list[dict[str, Any]]) -> str:
    output = [
        "// Generated from gravity_arcade.swf DefineShape 2, 4, and 9 via reference/extract_shapes.py --contours.",
        "// Coordinates are stored as SWF shape-local pixels.",
        "",
        "use super::{RadialGradientShape, SwfPathSegment, SwfPoint};",
        "",
    ]
    by_id = {shape["id"]: shape for shape in shapes}
    for shape_id in SHAPES:
        generate_shape(output, by_id[shape_id])
    return "\n".join(output)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("swf", type=Path)
    parser.add_argument("--out", type=Path, default=Path("src/ball_shapes.rs"))
    args = parser.parse_args()

    shapes = extract_shapes(args.swf, set(SHAPES), include_contours=True)
    found = {shape["id"] for shape in shapes}
    if found != set(SHAPES):
        raise SystemExit(f"missing ball DefineShape ids: {sorted(set(SHAPES) - found)}")

    args.out.write_text(generate_module(shapes), encoding="utf-8")


if __name__ == "__main__":
    main()

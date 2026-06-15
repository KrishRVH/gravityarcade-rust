#!/usr/bin/env python3
"""Generate recovered Rust paths for gravity-preview arrow DefineShape 41."""

from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any

from extract_shapes import extract_shapes


ARROW_SHAPE_ID = 41


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


def shape_bounds_fields(bounds: list[float]) -> list[str]:
    return [
        "    bounds: SwfBounds {",
        f"        x_min: {rust_float(bounds[0])},",
        f"        x_max: {rust_float(bounds[1])},",
        f"        y_min: {rust_float(bounds[2])},",
        f"        y_max: {rust_float(bounds[3])},",
        "    },",
    ]


def line_paths(records: list[dict[str, Any]]) -> list[dict[str, Any]]:
    paths: list[dict[str, Any]] = []
    current: dict[str, Any] | None = None
    for record in records:
        if record["type"] == "style_change":
            if current is not None:
                paths.append(current)
            current = None
            if "move_to" in record:
                current = {"start": record["move_to"], "points": []}
        elif record["type"] == "line":
            if current is None:
                current = {"start": record["from"], "points": []}
            current["points"].append(record["to"])
        elif record["type"] == "curve":
            raise SystemExit("shape 41 unexpectedly contains a curve edge")
    if current is not None:
        paths.append(current)
    return paths


def write_points(output: list[str], name: str, points: list[list[float]]) -> None:
    output.append(f"const {name}_POINTS: [SwfPoint; {len(points)}] = [")
    for point in points:
        output.append(f"    {point_expr(point)},")
    output += [
        "];",
        "",
    ]


def write_polyline(output: list[str], field: str, name: str, start: list[float]) -> None:
    output += [
        f"    {field}: GravityPreviewArrowPolyline {{",
        f"        start: {point_expr(start)},",
        f"        points: &{name}_POINTS,",
        "    },",
    ]


def generate_module(shapes: list[dict[str, Any]]) -> str:
    shape = shapes[0]
    paths = line_paths(shape["records"])
    if len(paths) != 2:
        raise SystemExit(f"expected two arrow line paths, got {len(paths)}")
    head, shaft = paths
    if len(head["points"]) != 2 or len(shaft["points"]) != 1:
        raise SystemExit("unexpected shape 41 arrow path layout")

    output = [
        "// Generated from gravity_arcade.swf DefineShape 41 via reference/extract_shapes.py.",
        "// Coordinates are stored as SWF shape-local pixels.",
        "",
        "use super::{GravityPreviewArrowPolyline, GravityPreviewArrowShapeDefinition, SwfBounds, SwfPoint};",
        "",
        "#[cfg(test)]",
        f"pub const DEFINE_SHAPE_ID: u16 = {ARROW_SHAPE_ID};",
        "#[cfg(test)]",
        f"pub const BOUNDS_TWIPS: [i16; 4] = {rust_array([twips(value) for value in shape['bounds']])};",
        f"pub const LINE_RGB: [u8; 3] = {rgb_expr(shape['lines'][0]['color'])};",
        f"pub const LINE_WIDTH: f32 = {rust_float(shape['lines'][0]['width'])};",
        "",
    ]
    write_points(output, "HEAD", head["points"])
    write_points(output, "SHAFT", shaft["points"])
    output += [
        "pub const SHAPE: GravityPreviewArrowShapeDefinition = GravityPreviewArrowShapeDefinition {",
        *shape_bounds_fields(shape["bounds"]),
    ]
    write_polyline(output, "head", "HEAD", head["start"])
    write_polyline(output, "shaft", "SHAFT", shaft["start"])
    output.append("};")
    return "\n".join(output)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("swf", type=Path)
    parser.add_argument(
        "--out",
        type=Path,
        default=Path("src/gravity_preview_arrow_shape41.rs"),
    )
    args = parser.parse_args()

    shapes = extract_shapes(args.swf, {ARROW_SHAPE_ID}, include_contours=False)
    if len(shapes) != 1:
        raise SystemExit("missing gravity preview arrow DefineShape 41")
    args.out.write_text(generate_module(shapes), encoding="utf-8")


if __name__ == "__main__":
    main()

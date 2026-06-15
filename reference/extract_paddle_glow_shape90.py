#!/usr/bin/env python3
"""Generate recovered Rust contours for paddle glow DefineShape 90."""

from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any

from extract_shapes import extract_shapes


SHAPE_ID = 90
BODY_FILL = 1
TOP_CAP_FILL = 2
BOTTOM_CAP_FILL = 3


def twips(value: float) -> int:
    return round(value * 20.0)


def rust_float(value: float) -> str:
    if value == 0:
        return "0.0"
    out = f"{value:.6f}".rstrip("0").rstrip(".")
    if "." not in out:
        out += ".0"
    return out


def ratio_expr(value: int) -> str:
    if value == 0:
        return "0.0"
    if value == 255:
        return "1.0"
    return f"{value}.0 / 255.0"


def point_expr(point: list[float]) -> str:
    return f"SwfPoint::new({rust_float(point[0])}, {rust_float(point[1])})"


def rust_array(values: list[int]) -> str:
    return "[" + ", ".join(str(value) for value in values) + "]"


def rgba_records(fill: dict[str, Any]) -> list[list[int]]:
    records = []
    for record in fill["records"]:
        color, alpha = record["color"].split("@")
        rgb = [int(color[offset : offset + 2], 16) for offset in (1, 3, 5)]
        records.append([record["ratio"], *rgb, int(alpha)])
    return records


def gradient_expr(fill: dict[str, Any]) -> str:
    return "[" + ", ".join(rust_array(record) for record in rgba_records(fill)) + "]"


def alpha_stop_expr(record: list[int]) -> str:
    return (
        "AlphaStop { "
        f"ratio: {ratio_expr(record[0])}, "
        f"alpha: {ratio_expr(record[4])} "
        "}"
    )


def contour_bounds(contour: dict[str, Any]) -> list[float]:
    xs = [contour["start"][0]]
    ys = [contour["start"][1]]
    for segment in contour["segments"]:
        xs.append(segment["to"][0])
        ys.append(segment["to"][1])
        if "control" in segment:
            xs.append(segment["control"][0])
            ys.append(segment["control"][1])
    return [min(xs), max(xs), min(ys), max(ys)]


def segment_expr(segment: dict[str, Any]) -> str:
    if segment["type"] == "line":
        return f"SwfPathSegment::Line({point_expr(segment['to'])})"
    return (
        "SwfPathSegment::Quad { "
        f"control: {point_expr(segment['control'])}, "
        f"to: {point_expr(segment['to'])} "
        "}"
    )


def write_segments(output: list[str], name: str, contour: dict[str, Any]) -> None:
    output.append(f"const {name}_SEGMENTS: [SwfPathSegment; {len(contour['segments'])}] = [")
    for segment in contour["segments"]:
        output.append(f"    {segment_expr(segment)},")
    output += [
        "];",
        "",
    ]


def write_contour(output: list[str], field_name: str, segment_name: str, contour: dict[str, Any]) -> None:
    output += [
        f"    {field_name}: PaddleGlowContour {{",
        f"        start: {point_expr(contour['start'])},",
        f"        segments: &{segment_name}_SEGMENTS,",
        "    },",
    ]


def generate_module(shape: dict[str, Any]) -> str:
    contours_by_fill = {contour["fill"]: contour for contour in shape["contours"]}
    body = contours_by_fill[BODY_FILL]
    top_cap = contours_by_fill[TOP_CAP_FILL]
    bottom_cap = contours_by_fill[BOTTOM_CAP_FILL]
    body_fill = shape["fills"][BODY_FILL - 1]
    top_cap_fill = shape["fills"][TOP_CAP_FILL - 1]
    bottom_cap_fill = shape["fills"][BOTTOM_CAP_FILL - 1]
    body_records = rgba_records(body_fill)
    top_cap_records = rgba_records(top_cap_fill)
    bottom_cap_records = rgba_records(bottom_cap_fill)
    if top_cap_records != bottom_cap_records:
        raise SystemExit("expected matching top and bottom cap gradients")

    output = [
        "// Generated from gravity_arcade.swf DefineShape 90 via reference/extract_shapes.py --contours.",
        "// Coordinates are stored as SWF shape-local pixels.",
        "",
        "use super::{AlphaStop, PaddleGlowContour, PaddleGlowShapeDefinition, SwfBounds, SwfPathSegment, SwfPoint};",
        "",
        "pub const FLATTEN_STEPS: u8 = 4;",
        "#[cfg(test)]",
        f"pub const DEFINE_SHAPE_ID: u16 = {SHAPE_ID};",
        "#[cfg(test)]",
        f"pub const BOUNDS_TWIPS: [i16; 4] = {rust_array([twips(value) for value in shape['bounds']])};",
        "#[cfg(test)]",
        f"pub const BODY_GRADIENT_RGBA: [[u8; 5]; {len(body_records)}] = {gradient_expr(body_fill)};",
        "#[cfg(test)]",
        f"pub const TOP_CAP_GRADIENT_RGBA: [[u8; 5]; {len(top_cap_records)}] = {gradient_expr(top_cap_fill)};",
        "#[cfg(test)]",
        f"pub const BOTTOM_CAP_GRADIENT_RGBA: [[u8; 5]; {len(bottom_cap_records)}] = {gradient_expr(bottom_cap_fill)};",
        "",
        f"pub const LINEAR_ALPHA_STOPS: [AlphaStop; {len(body_records)}] = [",
    ]
    for record in body_records:
        output.append(f"    {alpha_stop_expr(record)},")
    output += [
        "];",
        f"pub const RADIAL_PEAK_STOP_RATIO: f32 = {ratio_expr(top_cap_records[0][0])};",
        "",
    ]
    write_segments(output, "BODY", body)
    write_segments(output, "TOP_CAP", top_cap)
    write_segments(output, "BOTTOM_CAP", bottom_cap)
    output += [
        "pub const SHAPE: PaddleGlowShapeDefinition = PaddleGlowShapeDefinition {",
        "    bounds: SwfBounds {",
        f"        x_min: {rust_float(shape['bounds'][0])},",
        f"        x_max: {rust_float(shape['bounds'][1])},",
        f"        y_min: {rust_float(shape['bounds'][2])},",
        f"        y_max: {rust_float(shape['bounds'][3])},",
        "    },",
        "    body_bounds: SwfBounds {",
        f"        x_min: {rust_float(contour_bounds(body)[0])},",
        f"        x_max: {rust_float(contour_bounds(body)[1])},",
        f"        y_min: {rust_float(contour_bounds(body)[2])},",
        f"        y_max: {rust_float(contour_bounds(body)[3])},",
        "    },",
    ]
    write_contour(output, "body", "BODY", body)
    write_contour(output, "top_cap", "TOP_CAP", top_cap)
    write_contour(output, "bottom_cap", "BOTTOM_CAP", bottom_cap)
    output.append("};")
    return "\n".join(output)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("swf", type=Path)
    parser.add_argument("--out", type=Path, default=Path("src/paddle_glow_shape90.rs"))
    args = parser.parse_args()

    shapes = extract_shapes(args.swf, {SHAPE_ID}, include_contours=True)
    if len(shapes) != 1:
        raise SystemExit(f"expected DefineShape {SHAPE_ID}, got {len(shapes)} matches")

    args.out.write_text(generate_module(shapes[0]), encoding="utf-8")


if __name__ == "__main__":
    main()

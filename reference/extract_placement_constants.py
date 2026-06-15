#!/usr/bin/env python3
"""Generate placement-derived constants from the SWF reference dump."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from extract_shapes import extract_shapes


SHAPE_IDS = {30, 74, 98, 123}
HEADER_PLACEMENTS = {
    "SPONSOR_HEADER_HIT": (56, 99, 127, 98),
    "BACK_HEADER_HIT": (57, 124, 127, 123),
}


def rust_float(value: float) -> str:
    if value == 0:
        return "0.0"
    out = f"{value:.7f}".rstrip("0").rstrip(".")
    if "." not in out:
        out += ".0"
    whole, _, fraction = out.partition(".")
    if len(fraction) > 3:
        fraction = "_".join(fraction[index : index + 3] for index in range(0, len(fraction), 3))
        out = f"{whole}.{fraction}"
    return out


def rust_array(values: list[int]) -> str:
    return "[" + ", ".join(str(value) for value in values) + "]"


def point_expr(point: list[float]) -> str:
    return f"SwfPoint::new({rust_float(point[0])}, {rust_float(point[1])})"


def bounds_expr(bounds: list[float]) -> list[str]:
    return [
        "SwfBounds {",
        f"    x_min: {rust_float(bounds[0])},",
        f"    x_max: {rust_float(bounds[1])},",
        f"    y_min: {rust_float(bounds[2])},",
        f"    y_max: {rust_float(bounds[3])},",
        "}",
    ]


def find_one(records: list[dict[str, Any]], **criteria: Any) -> dict[str, Any]:
    matches = [
        record
        for record in records
        if all(record.get(key) == value for key, value in criteria.items())
    ]
    if len(matches) != 1:
        raise SystemExit(f"expected one record for {criteria}, got {len(matches)}")
    return matches[0]


def matrix_value(
    records: list[dict[str, Any]],
    *,
    field: str,
    frame: int,
    sprite: int | None,
    char: int,
    depth: int,
) -> float:
    record = find_one(
        records,
        tag="PlaceObject2",
        frame=frame,
        sprite=sprite,
        char=char,
        depth=depth,
    )
    matrix = record.get("matrix")
    if not matrix:
        raise SystemExit(f"missing matrix for placement char={char} depth={depth}")
    return float(matrix[field])


def color_transform_add_rgb(
    records: list[dict[str, Any]],
    *,
    frame: int,
    sprite: int | None,
    depth: int,
    char: int | None = None,
) -> list[int]:
    criteria: dict[str, Any] = {
        "tag": "PlaceObject2",
        "frame": frame,
        "sprite": sprite,
        "depth": depth,
    }
    if char is not None:
        criteria["char"] = char
    record = find_one(records, **criteria)
    cxform = record.get("cxform")
    if not cxform:
        raise SystemExit(f"missing color transform for {criteria}")
    mult = cxform.get("mult")
    add = cxform.get("add")
    if mult != [0, 0, 0, 256] or add is None:
        raise SystemExit(f"expected additive RGB transform for {criteria}, got {cxform}")
    return [int(add[0]), int(add[1]), int(add[2])]


def shape_bounds(shapes: dict[int, dict[str, Any]], shape_id: int) -> list[float]:
    return [float(value) for value in shapes[shape_id]["bounds"]]


def contour_points(shape: dict[str, Any]) -> list[list[float]]:
    contours = shape["contours"]
    if len(contours) != 1:
        raise SystemExit(f"expected one contour for shape {shape['id']}, got {len(contours)}")
    contour = contours[0]
    if not contour["closed"] or len(contour["segments"]) != 4:
        raise SystemExit(f"expected closed 4-edge contour for shape {shape['id']}")
    points = [contour["start"]]
    points.extend(segment["to"] for segment in contour["segments"][:-1])
    return points


def placed_header_points(
    records: list[dict[str, Any]],
    shapes: dict[int, dict[str, Any]],
    *,
    frame: int,
    button_id: int,
    depth: int,
    shape_id: int,
) -> list[list[float]]:
    tx = matrix_value(records, field="tx", frame=frame, sprite=None, char=button_id, depth=depth)
    ty = matrix_value(records, field="ty", frame=frame, sprite=None, char=button_id, depth=depth)
    return [[point[0] + tx, point[1] + ty] for point in contour_points(shapes[shape_id])]


def write_const(output: list[str], name: str, expr: str) -> None:
    output.append(f"pub const {name}: {expr};")


def write_bounds_const(output: list[str], name: str, bounds: list[float]) -> None:
    output.append(f"pub const {name}: SwfBounds = {bounds_expr(bounds)[0]}")
    output.extend(bounds_expr(bounds)[1:])
    output[-1] += ";"
    output.append("")


def write_point_array(output: list[str], name: str, points: list[list[float]]) -> None:
    output.append(f"pub const {name}: [SwfPoint; {len(points)}] = [")
    for point in points:
        output.append(f"    {point_expr(point)},")
    output.append("];")
    output.append("")


def generate_module(swf: Path, deep_records: list[dict[str, Any]]) -> str:
    shapes = {
        shape["id"]: shape
        for shape in extract_shapes(swf, SHAPE_IDS, include_contours=True)
    }
    if set(shapes) != SHAPE_IDS:
        raise SystemExit(f"missing hit shapes: {sorted(SHAPE_IDS - set(shapes))}")

    output = [
        "// Generated from gravity_arcade.swf placements and hit shapes.",
        "// Placement/color-transform source: reference/swf_deep.json.",
        "",
        "use super::{SwfBounds, SwfPoint};",
        "",
        "pub const BALL_RED_ADD_RGB: [u8; 3] = "
        f"{rust_array(color_transform_add_rgb(deep_records, frame=2, sprite=3, depth=1))};",
        "pub const BALL_FIRE_ADD_RGB: [u8; 3] = "
        f"{rust_array(color_transform_add_rgb(deep_records, frame=3, sprite=10, depth=1))};",
        "pub const GRAVITY_PREVIEW_RED_RIGHT_ADD_RGB: [u8; 3] = "
        f"{rust_array(color_transform_add_rgb(deep_records, frame=2, sprite=42, char=2, depth=2))};",
        "pub const GRAVITY_PREVIEW_RED_LEFT_ADD_RGB: [u8; 3] = "
        f"{rust_array(color_transform_add_rgb(deep_records, frame=2, sprite=42, char=2, depth=4))};",
        "pub const STATIC_RIGHT_PADDLE_GLOW_ADD_RGB: [u8; 3] = "
        f"{rust_array(color_transform_add_rgb(deep_records, frame=56, sprite=None, char=90, depth=64))};",
        "pub const STATIC_LEFT_PADDLE_GLOW_ADD_RGB: [u8; 3] = "
        f"{rust_array(color_transform_add_rgb(deep_records, frame=56, sprite=None, char=90, depth=65))};",
        "",
        f"pub const STATIC_PADDLE_GLOW_SCALE_X: f32 = {rust_float(matrix_value(deep_records, field='sx', frame=56, sprite=None, char=90, depth=64))};",
        f"pub const STATIC_PADDLE_GLOW_SCALE_Y: f32 = {rust_float(matrix_value(deep_records, field='sy', frame=56, sprite=None, char=90, depth=64))};",
        "",
        f"pub const SPONSOR_LOGO_ROOT_X: f32 = {rust_float(matrix_value(deep_records, field='tx', frame=56, sprite=None, char=100, depth=129))};",
        f"pub const SPONSOR_LOGO_ROOT_Y: f32 = {rust_float(matrix_value(deep_records, field='ty', frame=56, sprite=None, char=100, depth=129))};",
        f"pub const SPONSOR_LOGO_ROOT_SCALE: f32 = {rust_float(matrix_value(deep_records, field='sx', frame=56, sprite=None, char=100, depth=129))};",
        f"pub const SPONSOR_LOGO_BUTTON_TX: f32 = {rust_float(matrix_value(deep_records, field='tx', frame=1, sprite=100, char=31, depth=1))};",
        f"pub const SPONSOR_LOGO_BUTTON_TY: f32 = {rust_float(matrix_value(deep_records, field='ty', frame=1, sprite=100, char=31, depth=1))};",
        "",
        f"pub const NEOKOLOR_LINK_ROOT_X: f32 = {rust_float(matrix_value(deep_records, field='tx', frame=56, sprite=None, char=76, depth=44))};",
        f"pub const NEOKOLOR_LINK_ROOT_Y: f32 = {rust_float(matrix_value(deep_records, field='ty', frame=56, sprite=None, char=76, depth=44))};",
        f"pub const NEOKOLOR_LINK_BUTTON_TX: f32 = {rust_float(matrix_value(deep_records, field='tx', frame=1, sprite=76, char=75, depth=2))};",
        f"pub const NEOKOLOR_LINK_BUTTON_TY: f32 = {rust_float(matrix_value(deep_records, field='ty', frame=1, sprite=76, char=75, depth=2))};",
        f"pub const NEOKOLOR_LINK_BUTTON_SCALE_X: f32 = {rust_float(matrix_value(deep_records, field='sx', frame=1, sprite=76, char=75, depth=2))};",
        f"pub const NEOKOLOR_LINK_BUTTON_SCALE_Y: f32 = {rust_float(matrix_value(deep_records, field='sy', frame=1, sprite=76, char=75, depth=2))};",
        "",
        "#[cfg(test)]",
        "pub const SPONSOR_LOGO_BUTTON_HIT_DEFINE_SHAPE_ID: u16 = 30;",
        "#[cfg(test)]",
        "pub const NEOKOLOR_LINK_BUTTON_HIT_DEFINE_SHAPE_ID: u16 = 74;",
        "#[cfg(test)]",
        "pub const SPONSOR_HEADER_HIT_DEFINE_SHAPE_ID: u16 = 98;",
        "#[cfg(test)]",
        "pub const BACK_HEADER_HIT_DEFINE_SHAPE_ID: u16 = 123;",
        "",
    ]
    write_bounds_const(output, "SPONSOR_LOGO_BUTTON_HIT_BOUNDS", shape_bounds(shapes, 30))
    write_bounds_const(output, "NEOKOLOR_LINK_BUTTON_HIT_BOUNDS", shape_bounds(shapes, 74))
    for name, (frame, button_id, depth, shape_id) in HEADER_PLACEMENTS.items():
        write_point_array(
            output,
            name,
            placed_header_points(
                deep_records,
                shapes,
                frame=frame,
                button_id=button_id,
                depth=depth,
                shape_id=shape_id,
            ),
        )
    return "\n".join(output)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("swf", type=Path)
    parser.add_argument("deep_json", type=Path)
    parser.add_argument("--out", type=Path, default=Path("src/placement_constants.rs"))
    args = parser.parse_args()

    deep_records = json.loads(args.deep_json.read_text(encoding="utf-8"))
    args.out.write_text(generate_module(args.swf, deep_records), encoding="utf-8")


if __name__ == "__main__":
    main()

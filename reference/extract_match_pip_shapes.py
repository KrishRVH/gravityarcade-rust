#!/usr/bin/env python3
"""Generate recovered Rust contours for match-pip DefineShape 147 and 151."""

from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any

from extract_shapes import extract_shapes


BLUE_SHAPE_ID = 147
RED_SHAPE_ID = 151

BLUE_SLOT_BY_COLOR = {
    "#8c8cb0": "Outer",
    "#00b8d1": "Core",
    "#73e8ff": "Highlight",
    "#00c5ed": "Mid",
    "#ccf9ff": "Shine",
    "#4ad4ff": "Accent",
}
RED_SLOT_BY_COLOR = {
    "#8c8cb0": "Outer",
    "#ffde73": "Highlight",
    "#d19500": "Core",
    "#edaa00": "Mid",
    "#ffcc4a": "Accent",
    "#fff1cc": "Shine",
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


def rgb_expr(color: str) -> str:
    return (
        "["
        + ", ".join(str(int(color[offset : offset + 2], 16)) for offset in (1, 3, 5))
        + "]"
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


def contours_by_color(shape: dict[str, Any]) -> dict[str, list[dict[str, Any]]]:
    by_color: dict[str, list[dict[str, Any]]] = {}
    for contour in shape["contours"]:
        by_color.setdefault(contour["style"]["color"], []).append(contour)
    return by_color


def contour_key(color: str, contour: dict[str, Any]) -> str:
    start = contour["start"]
    return (
        color.upper()
        .replace("#", "")
        .replace("@", "_")
        + f"_{twips(start[0])}_{twips(start[1])}".replace("-", "NEG_")
    )


def emit_contour(
    output: list[str],
    *,
    name: str,
    slot: str,
    contour: dict[str, Any],
) -> str:
    segment_name = f"{name}_SEGMENTS"
    output.append(
        f"const {segment_name}: [SwfPathSegment; {len(contour['segments'])}] = ["
    )
    for segment in contour["segments"]:
        output.append(f"    {segment_expr(segment)},")
    output.append("];")
    output.append("")

    contour_name = f"{name}_CONTOUR"
    output.append(f"const {contour_name}: MatchPipContour = MatchPipContour {{")
    output.append(f"    slot: MatchPipSlot::{slot},")
    output.append(f"    start: {point_expr(contour['start'])},")
    output.append(f"    segments: &{segment_name},")
    output.append("};")
    output.append("")
    return contour_name


def shape_contour_names(
    output: list[str],
    *,
    prefix: str,
    shape: dict[str, Any],
    slot_by_color: dict[str, str],
) -> dict[str, list[str]]:
    by_color = contours_by_color(shape)
    names_by_color = {}
    for color in slot_by_color:
        names_by_color[color] = []
        for contour in by_color[color]:
            name = f"{prefix}_{contour_key(color, contour)}"
            names_by_color[color].append(
                emit_contour(
                    output,
                    name=name,
                    slot=slot_by_color[color],
                    contour=contour,
                )
            )
    return names_by_color


def generate_module(blue: dict[str, Any], red: dict[str, Any]) -> str:
    output = [
        "// Generated from gravity_arcade.swf DefineShape 147 and 151 via reference/extract_shapes.py --contours.",
        "// Coordinates are stored as SWF shape-local pixels.",
        "",
        "use super::{MatchPipContour, MatchPipSlot, SwfPathSegment, SwfPoint};",
        "",
        "pub const MATCH_PIP_FLATTEN_STEPS: u8 = 4;",
        "#[cfg(test)]",
        f"pub const BLUE_DEFINE_SHAPE_ID: u16 = {BLUE_SHAPE_ID};",
        "#[cfg(test)]",
        f"pub const RED_DEFINE_SHAPE_ID: u16 = {RED_SHAPE_ID};",
        "#[cfg(test)]",
        f"pub const BLUE_BOUNDS_TWIPS: [i16; 4] = {rust_array([twips(value) for value in blue['bounds']])};",
        "#[cfg(test)]",
        f"pub const RED_BOUNDS_TWIPS: [i16; 4] = {rust_array([twips(value) for value in red['bounds']])};",
        "pub const BLUE_FILL_RGB: [[u8; 3]; 6] = [",
    ]
    for fill in blue["fills"]:
        output.append(f"    {rgb_expr(fill['color'])},")
    output += [
        "];",
        "pub const RED_FILL_RGB: [[u8; 3]; 6] = [",
    ]
    for fill in red["fills"]:
        output.append(f"    {rgb_expr(fill['color'])},")
    output += [
        "];",
        "",
    ]

    blue_names = shape_contour_names(
        output,
        prefix="BLUE",
        shape=blue,
        slot_by_color=BLUE_SLOT_BY_COLOR,
    )
    red_names = shape_contour_names(
        output,
        prefix="RED",
        shape=red,
        slot_by_color=RED_SLOT_BY_COLOR,
    )

    blue_main = (
        blue_names["#8c8cb0"]
        + blue_names["#00b8d1"]
        + blue_names["#73e8ff"]
        + blue_names["#00c5ed"]
        + blue_names["#ccf9ff"]
    )
    blue_accent = blue_names["#4ad4ff"][0]
    red_shine = red_names["#fff1cc"]
    red_main = (
        red_names["#8c8cb0"]
        + red_names["#ffde73"]
        + red_names["#d19500"]
        + red_names["#edaa00"]
        + red_names["#ffcc4a"]
        + red_shine[:2]
    )
    red_top_shine = red_shine[2]

    output.append(f"pub const BLUE_MATCH_PIP_CONTOURS: [MatchPipContour; {len(blue_main)}] = [")
    for name in blue_main:
        output.append(f"    {name},")
    output.append("];")
    output.append(f"pub const BLUE_MATCH_PIP_ACCENT: MatchPipContour = {blue_accent};")
    output.append("")

    output.append(f"pub const RED_MATCH_PIP_CONTOURS: [MatchPipContour; {len(red_main)}] = [")
    for name in red_main:
        output.append(f"    {name},")
    output.append("];")
    output.append(f"pub const RED_MATCH_PIP_TOP_SHINE: MatchPipContour = {red_top_shine};")

    return "\n".join(output)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("swf", type=Path)
    parser.add_argument("--out", type=Path, default=Path("src/match_pip_shapes.rs"))
    args = parser.parse_args()

    shapes = extract_shapes(args.swf, {BLUE_SHAPE_ID, RED_SHAPE_ID}, include_contours=True)
    by_id = {shape["id"]: shape for shape in shapes}
    if set(by_id) != {BLUE_SHAPE_ID, RED_SHAPE_ID}:
        raise SystemExit(f"missing match-pip DefineShape ids: {sorted({BLUE_SHAPE_ID, RED_SHAPE_ID} - set(by_id))}")

    args.out.write_text(generate_module(by_id[BLUE_SHAPE_ID], by_id[RED_SHAPE_ID]), encoding="utf-8")


if __name__ == "__main__":
    main()

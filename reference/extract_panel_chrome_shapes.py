#!/usr/bin/env python3
"""Generate recovered Rust contours for panel chrome DefineShape records."""

from __future__ import annotations

import argparse
from pathlib import Path
from typing import Any

from extract_shapes import extract_shapes


PANEL_FILL_SHAPE_ID = 37
RETAINED_MASK_SHAPE_ID = 80
PANEL_OUTLINE_SHAPE_ID = 91
PLAYFIELD_MASK_SHAPE_ID = 141


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


def segment_expr(edge: dict[str, Any]) -> str:
    if edge["type"] == "line":
        return f"SwfPathSegment::Line({point_expr(edge['to'])})"
    return (
        "SwfPathSegment::Quad { "
        f"control: {point_expr(edge['control'])}, "
        f"to: {point_expr(edge['to'])} "
        "}"
    )


def reverse_edge(edge: dict[str, Any]) -> dict[str, Any]:
    reversed_edge = {
        "type": edge["type"],
        "from": edge["to"],
        "to": edge["from"],
    }
    if edge["type"] == "curve":
        reversed_edge["control"] = edge["control"]
    return reversed_edge


def shape_bounds_fields(bounds: list[float]) -> list[str]:
    return [
        "    bounds: SwfBounds {",
        f"        x_min: {rust_float(bounds[0])},",
        f"        x_max: {rust_float(bounds[1])},",
        f"        y_min: {rust_float(bounds[2])},",
        f"        y_max: {rust_float(bounds[3])},",
        "    },",
    ]


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


def bounds_const_expr(bounds: list[float]) -> list[str]:
    return [
        "SwfBounds {",
        f"    x_min: {rust_float(bounds[0])},",
        f"    x_max: {rust_float(bounds[1])},",
        f"    y_min: {rust_float(bounds[2])},",
        f"    y_max: {rust_float(bounds[3])},",
        "}",
    ]


def write_segments(output: list[str], name: str, segments: list[dict[str, Any]]) -> None:
    output.append(f"const {name}_SEGMENTS: [SwfPathSegment; {len(segments)}] = [")
    for segment in segments:
        output.append(f"    {segment_expr(segment)},")
    output += [
        "];",
        "",
    ]


def write_contour(output: list[str], field: str, name: str, start: list[float]) -> None:
    output += [
        f"    {field}: PanelChromeContour {{",
        f"        start: {point_expr(start)},",
        f"        segments: &{name}_SEGMENTS,",
        "    },",
    ]


def edge_lookup(records: list[dict[str, Any]]) -> dict[tuple[tuple[float, float], tuple[float, float]], dict[str, Any]]:
    return {
        ((record["from"][0], record["from"][1]), (record["to"][0], record["to"][1])): record
        for record in records
        if record["type"] in ("line", "curve")
    }


def line_paths(records: list[dict[str, Any]]) -> list[dict[str, Any]]:
    paths: list[dict[str, Any]] = []
    current: dict[str, Any] | None = None
    current_line = 0

    def flush() -> None:
        nonlocal current
        if current is not None and current["segments"]:
            paths.append(current)
        current = None

    for record in records:
        if record["type"] == "style_change":
            if "line" in record:
                current_line = record["line"]
            if "move_to" in record:
                flush()
                if current_line:
                    current = {"start": record["move_to"], "segments": []}
        elif record["type"] in ("line", "curve"):
            if not record["line"]:
                flush()
                continue
            if current is None:
                current = {"start": record["from"], "segments": []}
            current["segments"].append(record)
    flush()
    return paths


def panel_fill_contours(shape: dict[str, Any]) -> dict[str, dict[str, Any]]:
    center = next(
        contour
        for contour in shape["contours"]
        if contour["fill"] == 1 and contour["edge_count"] == 4
    )
    shadows = [contour for contour in shape["contours"] if contour["fill"] == 2]
    if len(shadows) != 2:
        raise SystemExit("expected exactly two panel side-shadow fill contours")
    left_shadow = min(shadows, key=lambda contour: contour["start"][0])
    right_shadow = max(shadows, key=lambda contour: contour["start"][0])
    return {
        "center": center,
        "left_shadow": left_shadow,
        "right_shadow": right_shadow,
    }


def edge_between(
    lookup: dict[tuple[tuple[float, float], tuple[float, float]], dict[str, Any]],
    start: list[float],
    end: list[float],
) -> dict[str, Any]:
    key = ((start[0], start[1]), (end[0], end[1]))
    if key in lookup:
        return lookup[key]
    reverse_key = ((end[0], end[1]), (start[0], start[1]))
    if reverse_key in lookup:
        return reverse_edge(lookup[reverse_key])
    raise SystemExit(f"missing shape edge from {start} to {end}")


def playfield_mask_corner_paths(shape: dict[str, Any]) -> dict[str, dict[str, Any]]:
    contour = max(shape["contours"], key=lambda item: item["edge_count"])
    bounds = contour_bounds(contour)
    x_min, x_max, y_min, y_max = bounds
    edges = edge_lookup(shape["records"])

    top_left_start = [x_min, 41.0]
    top_right_start = [538.0, y_min]
    bottom_left_start = [x_min, 387.95]
    bottom_right_start = [538.0, y_max]

    return {
        "top_left": {
            "start": top_left_start,
            "segments": [
                {"type": "line", "from": top_left_start, "to": [x_min, y_min]},
                {"type": "line", "from": [x_min, y_min], "to": [12.05, y_min]},
                edge_between(edges, [12.05, y_min], [8.5, 37.45]),
                edge_between(edges, [8.5, 37.45], top_left_start),
            ],
        },
        "top_right": {
            "start": top_right_start,
            "segments": [
                {"type": "line", "from": top_right_start, "to": [x_max, y_min]},
                {"type": "line", "from": [x_max, y_min], "to": [x_max, 41.0]},
                edge_between(edges, [x_max, 41.0], [541.55, 37.5]),
                edge_between(edges, [541.55, 37.5], [541.5, 37.45]),
                edge_between(edges, [541.5, 37.45], top_right_start),
            ],
        },
        "bottom_left": {
            "start": bottom_left_start,
            "segments": [
                {"type": "line", "from": bottom_left_start, "to": [x_min, y_max]},
                {"type": "line", "from": [x_min, y_max], "to": [12.05, y_max]},
                edge_between(edges, [12.05, y_max], [8.5, 391.45]),
                edge_between(edges, [8.5, 391.45], bottom_left_start),
            ],
        },
        "bottom_right": {
            "start": bottom_right_start,
            "segments": [
                {"type": "line", "from": bottom_right_start, "to": [x_max, y_max]},
                {"type": "line", "from": [x_max, y_max], "to": [x_max, 387.95]},
                edge_between(edges, [x_max, 387.95], [x_max, 388.1]),
                edge_between(edges, [x_max, 388.1], [x_max, 388.15]),
                edge_between(edges, [x_max, 388.15], [542.75, 389.5]),
                edge_between(edges, [542.75, 389.5], [541.55, 391.5]),
                edge_between(edges, [541.55, 391.5], [539.55, 392.7]),
                edge_between(edges, [539.55, 392.7], [538.2, y_max]),
                edge_between(edges, [538.2, y_max], [538.1, y_max]),
                edge_between(edges, [538.1, y_max], bottom_right_start),
            ],
        },
    }


def generate_module(shapes: list[dict[str, Any]]) -> str:
    by_id = {shape["id"]: shape for shape in shapes}
    panel_fill = by_id[PANEL_FILL_SHAPE_ID]
    retained_mask = by_id[RETAINED_MASK_SHAPE_ID]
    panel_outline = by_id[PANEL_OUTLINE_SHAPE_ID]
    playfield_mask = by_id[PLAYFIELD_MASK_SHAPE_ID]
    fill_contours = panel_fill_contours(panel_fill)
    playfield_mask_corners = playfield_mask_corner_paths(playfield_mask)
    retained_paths = line_paths(retained_mask["records"])
    outline_paths = line_paths(panel_outline["records"])
    if len(retained_paths) == 1:
        retained_segments = retained_paths[0]["segments"]
        retained_paths = [
            {
                "start": retained_paths[0]["start"],
                "segments": retained_segments[:-2],
            },
            {
                "start": retained_segments[-2]["from"],
                "segments": retained_segments[-2:],
            },
        ]
    if len(retained_paths) != 2:
        raise SystemExit(f"expected two retained frame-mask outline paths, got {len(retained_paths)}")
    if len(outline_paths) != 2:
        raise SystemExit(f"expected two panel outline paths, got {len(outline_paths)}")
    if panel_outline["lines"][0] != retained_mask["lines"][0]:
        raise SystemExit("expected panel outline and retained mask line styles to match")

    output = [
        "// Generated from gravity_arcade.swf DefineShape 37, 80, and 91 via reference/extract_shapes.py --contours.",
        "// Coordinates are stored as SWF stage pixels.",
        "",
        "use super::{PanelChromeContour, PanelChromeFillShapeDefinition, PanelChromeLineShapeDefinition, PanelChromeMaskShapeDefinition, SwfBounds, SwfPathSegment, SwfPoint};",
        "",
        "pub const FLATTEN_STEPS: u8 = 4;",
        "#[cfg(test)]",
        f"pub const PANEL_FILL_DEFINE_SHAPE_ID: u16 = {PANEL_FILL_SHAPE_ID};",
        "#[cfg(test)]",
        f"pub const RETAINED_MASK_DEFINE_SHAPE_ID: u16 = {RETAINED_MASK_SHAPE_ID};",
        "#[cfg(test)]",
        f"pub const PANEL_OUTLINE_DEFINE_SHAPE_ID: u16 = {PANEL_OUTLINE_SHAPE_ID};",
        "#[cfg(test)]",
        f"pub const PLAYFIELD_MASK_DEFINE_SHAPE_ID: u16 = {PLAYFIELD_MASK_SHAPE_ID};",
        "#[cfg(test)]",
        f"pub const PANEL_FILL_BOUNDS_TWIPS: [i16; 4] = {rust_array([twips(value) for value in panel_fill['bounds']])};",
        "#[cfg(test)]",
        f"pub const RETAINED_MASK_BOUNDS_TWIPS: [i16; 4] = {rust_array([twips(value) for value in retained_mask['bounds']])};",
        "#[cfg(test)]",
        f"pub const PANEL_OUTLINE_BOUNDS_TWIPS: [i16; 4] = {rust_array([twips(value) for value in panel_outline['bounds']])};",
        "#[cfg(test)]",
        f"pub const PLAYFIELD_MASK_BOUNDS_TWIPS: [i16; 4] = {rust_array([twips(value) for value in playfield_mask['bounds']])};",
        f"pub const PANEL_FILL_RGB: [u8; 3] = {rgb_expr(panel_fill['fills'][0]['color'])};",
        f"pub const PANEL_SHADOW_RGB: [u8; 3] = {rgb_expr(panel_fill['fills'][1]['color'])};",
        f"pub const MASK_FILL_RGB: [u8; 3] = {rgb_expr(retained_mask['fills'][0]['color'])};",
        f"pub const OUTLINE_RGB: [u8; 3] = {rgb_expr(panel_outline['lines'][0]['color'])};",
        f"pub const OUTLINE_LINE_WIDTH: f32 = {rust_float(panel_outline['lines'][0]['width'])};",
        "",
        "pub const PANEL_CENTER_BOUNDS: SwfBounds =",
        *bounds_const_expr(contour_bounds(fill_contours["center"])),
        ";",
        "",
    ]

    write_segments(output, "PANEL_CENTER", fill_contours["center"]["segments"])
    write_segments(output, "PANEL_LEFT_SHADOW", fill_contours["left_shadow"]["segments"])
    write_segments(output, "PANEL_RIGHT_SHADOW", fill_contours["right_shadow"]["segments"])
    write_segments(output, "PANEL_OUTLINE_PRIMARY", outline_paths[0]["segments"])
    write_segments(output, "PANEL_OUTLINE_LOWER_RIGHT", outline_paths[1]["segments"])
    write_segments(output, "RETAINED_MASK_OUTLINE_PRIMARY", retained_paths[0]["segments"])
    write_segments(output, "RETAINED_MASK_OUTLINE_LOWER_RIGHT", retained_paths[1]["segments"])
    write_segments(output, "PLAYFIELD_MASK_TOP_LEFT", playfield_mask_corners["top_left"]["segments"])
    write_segments(output, "PLAYFIELD_MASK_TOP_RIGHT", playfield_mask_corners["top_right"]["segments"])
    write_segments(output, "PLAYFIELD_MASK_BOTTOM_LEFT", playfield_mask_corners["bottom_left"]["segments"])
    write_segments(output, "PLAYFIELD_MASK_BOTTOM_RIGHT", playfield_mask_corners["bottom_right"]["segments"])

    output += [
        "pub const PANEL_FILL_SHAPE: PanelChromeFillShapeDefinition = PanelChromeFillShapeDefinition {",
        *shape_bounds_fields(panel_fill["bounds"]),
    ]
    write_contour(output, "center", "PANEL_CENTER", fill_contours["center"]["start"])
    write_contour(output, "left_shadow", "PANEL_LEFT_SHADOW", fill_contours["left_shadow"]["start"])
    write_contour(output, "right_shadow", "PANEL_RIGHT_SHADOW", fill_contours["right_shadow"]["start"])
    output += [
        "};",
        "",
        "pub const PANEL_OUTLINE_SHAPE: PanelChromeLineShapeDefinition = PanelChromeLineShapeDefinition {",
        *shape_bounds_fields(panel_outline["bounds"]),
    ]
    write_contour(output, "primary", "PANEL_OUTLINE_PRIMARY", outline_paths[0]["start"])
    write_contour(output, "lower_right", "PANEL_OUTLINE_LOWER_RIGHT", outline_paths[1]["start"])
    output += [
        "};",
        "",
        "pub const RETAINED_MASK_OUTLINE_SHAPE: PanelChromeLineShapeDefinition = PanelChromeLineShapeDefinition {",
        *shape_bounds_fields(retained_mask["bounds"]),
    ]
    write_contour(output, "primary", "RETAINED_MASK_OUTLINE_PRIMARY", retained_paths[0]["start"])
    write_contour(
        output,
        "lower_right",
        "RETAINED_MASK_OUTLINE_LOWER_RIGHT",
        retained_paths[1]["start"],
    )
    output += [
        "};",
        "",
        "pub const PLAYFIELD_MASK_SHAPE: PanelChromeMaskShapeDefinition = PanelChromeMaskShapeDefinition {",
        *shape_bounds_fields(playfield_mask["bounds"]),
    ]
    write_contour(
        output,
        "top_left",
        "PLAYFIELD_MASK_TOP_LEFT",
        playfield_mask_corners["top_left"]["start"],
    )
    write_contour(
        output,
        "top_right",
        "PLAYFIELD_MASK_TOP_RIGHT",
        playfield_mask_corners["top_right"]["start"],
    )
    write_contour(
        output,
        "bottom_left",
        "PLAYFIELD_MASK_BOTTOM_LEFT",
        playfield_mask_corners["bottom_left"]["start"],
    )
    write_contour(
        output,
        "bottom_right",
        "PLAYFIELD_MASK_BOTTOM_RIGHT",
        playfield_mask_corners["bottom_right"]["start"],
    )
    output.append("};")
    return "\n".join(output)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("swf", type=Path)
    parser.add_argument("--out", type=Path, default=Path("src/panel_chrome_shapes.rs"))
    args = parser.parse_args()

    expected = {
        PANEL_FILL_SHAPE_ID,
        RETAINED_MASK_SHAPE_ID,
        PANEL_OUTLINE_SHAPE_ID,
        PLAYFIELD_MASK_SHAPE_ID,
    }
    shapes = extract_shapes(args.swf, expected, include_contours=True)
    found = {shape["id"] for shape in shapes}
    if found != expected:
        raise SystemExit(f"missing panel chrome DefineShape ids: {sorted(expected - found)}")
    args.out.write_text(generate_module(shapes), encoding="utf-8")


if __name__ == "__main__":
    main()

#!/usr/bin/env python3
"""Generate score-meter placement and color tables from the SWF reference dump."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any


ROOT_PLAYFIELD_FRAME = 58
PLAYFIELD_SPRITE_ID = 140
RED_SCORE_METER_SPRITE_ID = 136
BLUE_SCORE_METER_SPRITE_ID = 139
RED_SCORE_METER_DEPTH = 13
BLUE_SCORE_METER_DEPTH = 42
RED_SCORE_FILL_SHAPE_ID = 92
BLUE_SCORE_FILL_SHAPE_ID = 94
SCORE_OUTLINE_SHAPE_ID = 93
FINAL_OVERLAY_OUTLINE_SHAPE_ID = 133
FINAL_FRAME = 71
PHASE_FRAMES = range(2, 11)


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


def rust_u8_array(values: list[int]) -> str:
    return "[" + ", ".join(str(value) for value in values) + "]"


def point_expr(x: float, y: float) -> str:
    return f"SwfPoint::new({rust_float(x)}, {rust_float(y)})"


def final_marker_expr(placement: dict[str, Any]) -> str:
    return (
        "ScoreMeterFinalMarkerPlacement::new("
        f"{rust_float(placement['local_x'])}, "
        f"{rust_float(placement['local_y'])}, "
        f"{rust_float(placement['scale'])}, "
        f"{placement['mult']}, "
        f"{rust_u8_array(placement['add_rgb'])}"
        ")"
    )


def find_one(records: list[dict[str, Any]], **criteria: Any) -> dict[str, Any]:
    matches = [
        record
        for record in records
        if all(record.get(key) == value for key, value in criteria.items())
    ]
    if len(matches) != 1:
        raise SystemExit(f"expected one record for {criteria}, got {len(matches)}")
    return matches[0]


def placements(
    records: list[dict[str, Any]],
    *,
    sprite: int | None,
    frame: int,
) -> list[dict[str, Any]]:
    return [
        record
        for record in records
        if record.get("tag") == "PlaceObject2"
        and record.get("sprite") == sprite
        and record.get("frame") == frame
    ]


def matrix(record: dict[str, Any]) -> dict[str, float]:
    value = record.get("matrix")
    if not value:
        raise SystemExit(f"missing matrix for placement {record}")
    return {key: float(value[key]) for key in ("sx", "sy", "tx", "ty")}


def cxform_rgb(record: dict[str, Any]) -> tuple[int, list[int]]:
    cxform = record.get("cxform")
    if not cxform:
        raise SystemExit(f"missing cxform for placement {record}")
    mult = cxform.get("mult")
    if not mult:
        raise SystemExit(f"missing cxform multiplier for placement {record}")
    if mult[:3] != [mult[0], mult[0], mult[0]]:
        raise SystemExit(f"expected equal RGB multipliers for placement {record}")
    add = cxform.get("add") or [0, 0, 0, 0]
    return int(mult[0]), [int(add[0]), int(add[1]), int(add[2])]


def additive_rgb(record: dict[str, Any]) -> list[int]:
    mult, add_rgb = cxform_rgb(record)
    cxform = record["cxform"]
    if mult != 0 or cxform.get("add") is None:
        raise SystemExit(f"expected additive color transform for placement {record}")
    return add_rgb


def score_meter_root(records: list[dict[str, Any]], *, sprite_id: int, depth: int) -> dict[str, Any]:
    return find_one(
        records,
        tag="PlaceObject2",
        sprite=PLAYFIELD_SPRITE_ID,
        frame=1,
        char=sprite_id,
        depth=depth,
    )


def normal_marker_placement(records: list[dict[str, Any]], *, sprite_id: int) -> dict[str, Any]:
    return find_one(
        records,
        tag="PlaceObject2",
        sprite=sprite_id,
        frame=1,
        depth=1,
    )


def score_meter_base_y(records: list[dict[str, Any]]) -> float:
    root = matrix(
        find_one(
            records,
            tag="PlaceObject2",
            sprite=None,
            frame=ROOT_PLAYFIELD_FRAME,
            char=PLAYFIELD_SPRITE_ID,
        )
    )
    blue_meter = matrix(
        score_meter_root(
            records,
            sprite_id=BLUE_SCORE_METER_SPRITE_ID,
            depth=BLUE_SCORE_METER_DEPTH,
        )
    )
    return root["ty"] + blue_meter["ty"]


def score_meter_x(records: list[dict[str, Any]], *, sprite_id: int, depth: int) -> float:
    root = matrix(
        find_one(
            records,
            tag="PlaceObject2",
            sprite=None,
            frame=ROOT_PLAYFIELD_FRAME,
            char=PLAYFIELD_SPRITE_ID,
        )
    )
    meter_root = matrix(score_meter_root(records, sprite_id=sprite_id, depth=depth))
    marker = matrix(normal_marker_placement(records, sprite_id=sprite_id))
    return root["tx"] + meter_root["tx"] + marker["tx"]


def score_marker_groups(records: list[dict[str, Any]]) -> tuple[list[list[int]], list[list[float]]]:
    marker_records = [
        record
        for record in placements(records, sprite=BLUE_SCORE_METER_SPRITE_ID, frame=1)
        if record.get("depth", 0) % 2 == 1
    ]
    marker_records.sort(key=lambda record: int(record["depth"]))
    if len(marker_records) != 13:
        raise SystemExit(f"expected 13 score marker placements, got {len(marker_records)}")

    groups: list[list[dict[str, Any]]] = [[marker_records[0]]]
    for index in range(1, len(marker_records), 2):
        groups.append(marker_records[index : index + 2])
    if len(groups) != 7 or any(len(group) not in (1, 2) for group in groups):
        raise SystemExit(f"unexpected score marker grouping: {groups}")

    depth_groups: list[list[int]] = []
    y_groups: list[list[float]] = []
    for group in groups:
        sorted_group = sorted(group, key=lambda record: matrix(record)["ty"])
        depths = [int(record["depth"]) for record in sorted_group]
        local_ys = [matrix(record)["ty"] for record in sorted_group]
        if len(local_ys) == 1:
            local_ys.append(local_ys[0])
        depth_groups.append(depths)
        y_groups.append(local_ys)
    return depth_groups, y_groups


def phase_tables(records: list[dict[str, Any]], *, sprite_id: int) -> tuple[list[float], list[int], list[list[int]]]:
    scales: list[float] = []
    mults: list[int] = []
    adds: list[list[int]] = []
    for frame in PHASE_FRAMES:
        record = find_one(
            records,
            tag="PlaceObject2",
            sprite=sprite_id,
            frame=frame,
            depth=1,
        )
        record_matrix = matrix(record)
        mult, add_rgb = cxform_rgb(record)
        scales.append(record_matrix["sx"])
        mults.append(mult)
        adds.append(add_rgb)
    return scales, mults, adds


def group_start_frames(
    records: list[dict[str, Any]],
    *,
    depth_groups: list[list[int]],
) -> list[int]:
    starts: list[int] = []
    for group in depth_groups:
        candidates = []
        for frame in range(2, FINAL_FRAME):
            frame_records = [
                record
                for record in placements(records, sprite=BLUE_SCORE_METER_SPRITE_ID, frame=frame)
                if record.get("depth") in group and record.get("cxform", {}).get("add") is not None
            ]
            if frame_records:
                candidates.append(frame)
        if not candidates:
            raise SystemExit(f"missing start frame for depth group {group}")
        starts.append(candidates[0])
    return starts


def final_sprite_placement(
    records: list[dict[str, Any]], *, score_sprite_id: int, final_sprite_id: int
) -> dict[str, float]:
    record = find_one(
        records,
        tag="PlaceObject2",
        sprite=score_sprite_id,
        frame=FINAL_FRAME,
        char=final_sprite_id,
        depth=1,
    )
    return matrix(record)


def final_marker_placements(
    records: list[dict[str, Any]],
    *,
    score_sprite_id: int,
    final_sprite_id: int,
    fill_shape_id: int,
) -> list[dict[str, Any]]:
    final_root = final_sprite_placement(
        records,
        score_sprite_id=score_sprite_id,
        final_sprite_id=final_sprite_id,
    )
    normal_marker_tx = matrix(normal_marker_placement(records, sprite_id=score_sprite_id))["tx"]
    output = []
    for record in sorted(
        [
            record
            for record in placements(records, sprite=final_sprite_id, frame=1)
            if record.get("char") == fill_shape_id
        ],
        key=lambda record: int(record["depth"]),
    ):
        record_matrix = matrix(record)
        mult, add_rgb = cxform_rgb(record)
        output.append(
            {
                "local_x": final_root["tx"] + record_matrix["tx"] - normal_marker_tx,
                "local_y": final_root["ty"] + record_matrix["ty"],
                "scale": record_matrix["sx"],
                "mult": mult,
                "add_rgb": add_rgb,
            }
        )
    if len(output) != 13:
        raise SystemExit(f"expected 13 final marker placements, got {len(output)}")
    return output


def final_overlay_positions(
    records: list[dict[str, Any]],
    *,
    score_sprite_id: int,
    final_sprite_id: int,
    overlay_sprite_id: int,
) -> list[tuple[float, float]]:
    final_root = final_sprite_placement(
        records,
        score_sprite_id=score_sprite_id,
        final_sprite_id=final_sprite_id,
    )
    normal_marker_tx = matrix(normal_marker_placement(records, sprite_id=score_sprite_id))["tx"]
    output = []
    for record in sorted(
        [
            record
            for record in placements(records, sprite=final_sprite_id, frame=1)
            if record.get("char") == overlay_sprite_id
        ],
        key=lambda record: int(record["depth"]),
    ):
        record_matrix = matrix(record)
        output.append(
            (
                final_root["tx"] + record_matrix["tx"] - normal_marker_tx,
                final_root["ty"] + record_matrix["ty"],
            )
        )
    if len(output) != 13:
        raise SystemExit(f"expected 13 final overlay positions, got {len(output)}")
    return output


def overlay_rgb_cycle(records: list[dict[str, Any]], *, overlay_sprite_id: int) -> list[list[int]]:
    frames = []
    frame = 1
    while True:
        frame_records = placements(records, sprite=overlay_sprite_id, frame=frame)
        if not frame_records:
            break
        if len(frame_records) != 1:
            raise SystemExit(f"expected one overlay color placement on frame {frame}")
        frames.append(additive_rgb(frame_records[0]))
        frame += 1
    if len(frames) != 13:
        raise SystemExit(f"expected 13 overlay color frames, got {len(frames)}")
    return frames


def final_sprite_id(records: list[dict[str, Any]], *, score_sprite_id: int) -> int:
    record = find_one(
        records,
        tag="PlaceObject2",
        sprite=score_sprite_id,
        frame=FINAL_FRAME,
        depth=1,
    )
    return int(record["char"])


def overlay_sprite_id(records: list[dict[str, Any]], *, final_sprite_id: int) -> int:
    overlay_ids = {
        int(record["char"])
        for record in placements(records, sprite=final_sprite_id, frame=1)
        if record.get("char") not in {RED_SCORE_FILL_SHAPE_ID, BLUE_SCORE_FILL_SHAPE_ID, SCORE_OUTLINE_SHAPE_ID}
    }
    if len(overlay_ids) != 1:
        raise SystemExit(f"expected one overlay sprite in final sprite {final_sprite_id}, got {overlay_ids}")
    return overlay_ids.pop()


def write_float_array(output: list[str], name: str, values: list[float], size: str) -> None:
    output.append(f"pub const {name}: [f32; {size}] = [")
    for value in values:
        output.append(f"    {rust_float(value)},")
    output.append("];")
    output.append("")


def write_u8_array(output: list[str], name: str, values: list[int], size: str) -> None:
    output.append(f"pub const {name}: [u8; {size}] = {rust_u8_array(values)};")


def write_rgb_table(output: list[str], name: str, values: list[list[int]], size: str) -> None:
    output.append(f"pub const {name}: [[u8; 3]; {size}] = [")
    for value in values:
        output.append(f"    {rust_u8_array(value)},")
    output.append("];")
    output.append("")


def write_point_array(output: list[str], name: str, values: list[tuple[float, float]], size: str) -> None:
    output.append(f"pub const {name}: [SwfPoint; {size}] = [")
    for x, y in values:
        output.append(f"    {point_expr(x, y)},")
    output.append("];")
    output.append("")


def write_final_markers(output: list[str], name: str, values: list[dict[str, Any]]) -> None:
    output.append(f"pub const {name}: [ScoreMeterFinalMarkerPlacement; FINAL_MARKERS] = [")
    for placement in values:
        output.append(f"    {final_marker_expr(placement)},")
    output.append("];")
    output.append("")


def generate_module(records: list[dict[str, Any]]) -> str:
    depth_groups, local_ys = score_marker_groups(records)
    red_scales, red_mults, red_adds = phase_tables(records, sprite_id=RED_SCORE_METER_SPRITE_ID)
    blue_scales, blue_mults, blue_adds = phase_tables(records, sprite_id=BLUE_SCORE_METER_SPRITE_ID)
    starts = group_start_frames(records, depth_groups=depth_groups)
    red_final_id = final_sprite_id(records, score_sprite_id=RED_SCORE_METER_SPRITE_ID)
    blue_final_id = final_sprite_id(records, score_sprite_id=BLUE_SCORE_METER_SPRITE_ID)
    red_overlay_id = overlay_sprite_id(records, final_sprite_id=red_final_id)
    blue_overlay_id = overlay_sprite_id(records, final_sprite_id=blue_final_id)

    output = [
        "// Generated from gravity_arcade.swf score-meter placements.",
        "// Placement/color-transform source: reference/swf_deep.json.",
        "",
        "use super::{ScoreMeterFinalMarkerPlacement, SwfPoint};",
        "",
        "#[cfg(test)]",
        f"pub const PLAYFIELD_SPRITE_ID: u16 = {PLAYFIELD_SPRITE_ID};",
        "#[cfg(test)]",
        f"pub const RED_SCORE_METER_SPRITE_ID: u16 = {RED_SCORE_METER_SPRITE_ID};",
        "#[cfg(test)]",
        f"pub const BLUE_SCORE_METER_SPRITE_ID: u16 = {BLUE_SCORE_METER_SPRITE_ID};",
        "#[cfg(test)]",
        f"pub const RED_FINAL_SCORE_METER_SPRITE_ID: u16 = {red_final_id};",
        "#[cfg(test)]",
        f"pub const BLUE_FINAL_SCORE_METER_SPRITE_ID: u16 = {blue_final_id};",
        "#[cfg(test)]",
        f"pub const RED_FINAL_OVERLAY_SPRITE_ID: u16 = {red_overlay_id};",
        "#[cfg(test)]",
        f"pub const BLUE_FINAL_OVERLAY_SPRITE_ID: u16 = {blue_overlay_id};",
        "#[cfg(test)]",
        f"pub const FINAL_OVERLAY_OUTLINE_SHAPE_ID: u16 = {FINAL_OVERLAY_OUTLINE_SHAPE_ID};",
        "",
        "pub const RAMP_STEPS: usize = 9;",
        "pub const FINAL_MARKERS: usize = 13;",
        "pub const FINAL_OVERLAY_FRAMES: usize = 13;",
        f"pub const BASE_Y: f32 = {rust_float(score_meter_base_y(records))};",
        f"pub const BLUE_X: f32 = {rust_float(score_meter_x(records, sprite_id=BLUE_SCORE_METER_SPRITE_ID, depth=BLUE_SCORE_METER_DEPTH))};",
        f"pub const RED_X: f32 = {rust_float(score_meter_x(records, sprite_id=RED_SCORE_METER_SPRITE_ID, depth=RED_SCORE_METER_DEPTH))};",
        f"pub const GROUP_START_FRAMES: [u8; {len(starts)}] = {rust_u8_array(starts)};",
        "",
        f"pub const LOCAL_YS: [[f32; 2]; {len(local_ys)}] = [",
    ]
    for local_y in local_ys:
        output.append(
            f"    [{rust_float(local_y[0])}, {rust_float(local_y[1])}],"
        )
    output.append("];")
    output.append("")

    write_float_array(output, "RED_PHASE_SCALES", red_scales, "RAMP_STEPS")
    write_float_array(output, "BLUE_PHASE_SCALES", blue_scales, "RAMP_STEPS")
    write_u8_array(output, "RED_PHASE_MULTS", red_mults, "RAMP_STEPS")
    write_u8_array(output, "BLUE_PHASE_MULTS", blue_mults, "RAMP_STEPS")
    output.append("")
    write_rgb_table(output, "RED_PHASE_ADDS", red_adds, "RAMP_STEPS")
    write_rgb_table(output, "BLUE_PHASE_ADDS", blue_adds, "RAMP_STEPS")
    write_final_markers(
        output,
        "RED_FINAL_MARKERS",
        final_marker_placements(
            records,
            score_sprite_id=RED_SCORE_METER_SPRITE_ID,
            final_sprite_id=red_final_id,
            fill_shape_id=RED_SCORE_FILL_SHAPE_ID,
        ),
    )
    write_final_markers(
        output,
        "BLUE_FINAL_MARKERS",
        final_marker_placements(
            records,
            score_sprite_id=BLUE_SCORE_METER_SPRITE_ID,
            final_sprite_id=blue_final_id,
            fill_shape_id=BLUE_SCORE_FILL_SHAPE_ID,
        ),
    )
    write_point_array(
        output,
        "RED_FINAL_OVERLAY_LOCAL_POSITIONS",
        final_overlay_positions(
            records,
            score_sprite_id=RED_SCORE_METER_SPRITE_ID,
            final_sprite_id=red_final_id,
            overlay_sprite_id=red_overlay_id,
        ),
        "FINAL_OVERLAY_FRAMES",
    )
    write_point_array(
        output,
        "BLUE_FINAL_OVERLAY_LOCAL_POSITIONS",
        final_overlay_positions(
            records,
            score_sprite_id=BLUE_SCORE_METER_SPRITE_ID,
            final_sprite_id=blue_final_id,
            overlay_sprite_id=blue_overlay_id,
        ),
        "FINAL_OVERLAY_FRAMES",
    )
    write_rgb_table(
        output,
        "RED_FINAL_OVERLAY_RGB",
        overlay_rgb_cycle(records, overlay_sprite_id=red_overlay_id),
        "FINAL_OVERLAY_FRAMES",
    )
    write_rgb_table(
        output,
        "BLUE_FINAL_OVERLAY_RGB",
        overlay_rgb_cycle(records, overlay_sprite_id=blue_overlay_id),
        "FINAL_OVERLAY_FRAMES",
    )
    return "\n".join(output)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("deep_json", type=Path)
    parser.add_argument("--out", type=Path, default=Path("src/score_meter_constants.rs"))
    args = parser.parse_args()

    records = json.loads(args.deep_json.read_text(encoding="utf-8"))
    args.out.write_text(generate_module(records), encoding="utf-8")


if __name__ == "__main__":
    main()

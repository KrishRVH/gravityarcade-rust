#!/usr/bin/env python3
"""Generate recovered Rust contours for menu and help DefineText tags."""

from __future__ import annotations

import argparse
import json
import struct
from dataclasses import dataclass
from pathlib import Path

from extract_announce_texts import (
    Contour,
    TextDefinition,
    centipx,
    iter_tags,
    parse_define_font,
    parse_define_text,
    rust_array,
    swf_body,
    text_contours,
    write_segment,
)


HELP_BODY = (
    "1. Try to shoot as many balls as possible into your opponents goal-line."
    "2. Big balls score more points than small balls."
    "3. Balls are subject to forces, the attract each other...    ...or reject eatch other - that "
    "depends on the gravity mode you play."
    "4. Touching balls merge into bigger ones."
    "5. Very big balls start to burn - don't touch them, they paralyze you."
    "6. In SpeedGravity-Mode faster balls score more     and the balls take up speed, when they "
    "are reflected by a paddel."
)


@dataclass(frozen=True)
class TextSpec:
    text_id: int
    const_name: str
    label: str


@dataclass(frozen=True)
class ModuleSpec:
    header: str
    prefix: str
    definition_name: str
    texts: tuple[TextSpec, ...]
    font_id: int | None = None


MODULES = {
    "menu-labels": ModuleSpec(
        header="// Generated from gravity_arcade.swf menu DefineText tags and DefineFont 26.",
        prefix="MenuLabel",
        definition_name="MenuLabelDefinition",
        font_id=26,
        texts=(
            TextSpec(53, "HELP", "how to play"),
            TextSpec(49, "POLARISATION", "gravity mode"),
            TextSpec(51, "MATCHES", "match type"),
            TextSpec(65, "GRAVITY", "gravity strenght"),
            TextSpec(67, "SPEED", "SpeedGravity"),
            TextSpec(40, "START", "start game"),
        ),
    ),
    "menu-values": ModuleSpec(
        header="// Generated from gravity_arcade.swf menu value DefineText tags.",
        prefix="MenuValue",
        definition_name="MenuValueDefinition",
        texts=(
            TextSpec(55, "QUESTION", "?"),
            TextSpec(43, "MATCH_SINGLE", "single match"),
            TextSpec(44, "MATCH_BEST_OF_3", "best of 3"),
            TextSpec(45, "MATCH_BEST_OF_5", "best of 5"),
            TextSpec(46, "MATCH_BEST_OF_7", "best of 7"),
            TextSpec(56, "GRAVITY_LOW", "low"),
            TextSpec(57, "GRAVITY_MEDIUM", "medium"),
            TextSpec(58, "GRAVITY_HIGH", "high"),
            TextSpec(59, "GRAVITY_VERY_HIGH", "very high"),
            TextSpec(62, "GRAVITY_BLACK_HOLE", "black hole"),
            TextSpec(68, "SPEED_DISABLED", "disabled"),
            TextSpec(69, "SPEED_ENABLED", "enabled"),
        ),
    ),
    "help-labels": ModuleSpec(
        header="// Generated from gravity_arcade.swf help-screen DefineText tags.",
        prefix="HelpLabel",
        definition_name="HelpLabelDefinition",
        texts=(
            TextSpec(101, "TITLE", "How to play ?"),
            TextSpec(102, "BODY", HELP_BODY),
            TextSpec(104, "BACK", "back"),
            TextSpec(107, "KEY_W", "w"),
            TextSpec(108, "KEY_D", "d"),
            TextSpec(109, "KEY_S", "s"),
            TextSpec(111, "RED_MOVE_UP", "move up"),
            TextSpec(112, "RED_MOVE_DOWN", "move down"),
            TextSpec(113, "RED_SHOOT", "shoot"),
            TextSpec(114, "BLUE_SHOOT", "shoot"),
            TextSpec(115, "BLUE_MOVE_UP", "move up"),
            TextSpec(116, "BLUE_MOVE_DOWN", "move down"),
            TextSpec(117, "PLAYER_RED", "Player Red"),
            TextSpec(118, "PLAYER_BLUE", "Player Blue"),
        ),
    ),
}


def module_ids(spec: ModuleSpec) -> set[int]:
    return {text.text_id for text in spec.texts}


def generate_module(
    *,
    spec: ModuleSpec,
    texts: dict[int, TextDefinition],
    fonts: dict[int, tuple[tuple[Contour, ...], ...]],
) -> str:
    output = [
        spec.header,
        "// Coordinates are stored as SWF text-local centipixels; divide by 100 for pixels.",
        "",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq)]",
        f"pub enum {spec.prefix}Segment {{",
        "    Line([i16; 2]),",
        "    Quad { control: [i16; 2], to: [i16; 2] },",
        "}",
        "",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq)]",
        f"pub struct {spec.prefix}Contour {{",
        "    pub(super) start: [i16; 2],",
        f"    pub(super) segments: &'static [{spec.prefix}Segment],",
        "}",
        "",
        "#[derive(Debug, Clone, Copy, PartialEq, Eq)]",
        f"pub struct {spec.definition_name} {{",
        "    pub(super) text: &'static str,",
        "    pub(super) define_text_id: u16,",
    ]
    if spec.font_id is None:
        output.append("    pub(super) font_ids: &'static [u16],")
    else:
        output.append("    pub(super) font_id: u16,")
    output += [
        "    pub(super) color_rgb: [u8; 3],",
        "    pub(super) bounds_centipx: [i16; 4],",
        f"    pub(super) contours: &'static [{spec.prefix}Contour],",
        "}",
        "",
    ]
    if spec.font_id is not None:
        output.append(f"pub const FONT_ID: u16 = {spec.font_id};")
        output.append("")

    for text_spec in spec.texts:
        text = texts[text_spec.text_id]
        font_ids = tuple(dict.fromkeys(run.font_id for run in text.runs))
        if spec.font_id is not None and font_ids != (spec.font_id,):
            raise SystemExit(
                f"DefineText {text_spec.text_id} used unexpected fonts: {list(font_ids)}"
            )

        color = text.runs[0].color_rgb
        bounds = [centipx(value) for value in text.bounds]
        contours = text_contours(text, fonts)

        if spec.font_id is None:
            output.append(
                f"const {text_spec.const_name}_FONT_IDS: [u16; {len(font_ids)}] = "
                f"{rust_array(font_ids)};"
            )
            output.append("")

        for index, contour in enumerate(contours):
            output.append(
                f"const {text_spec.const_name}_CONTOUR_{index}: "
                f"[{spec.prefix}Segment; {len(contour.segments)}] = ["
            )
            for segment in contour.segments:
                output.append(f"    {write_segment(spec.prefix, segment)},")
            output.append("];")
            output.append("")

        output.append(
            f"const {text_spec.const_name}_CONTOURS: "
            f"[{spec.prefix}Contour; {len(contours)}] = ["
        )
        for index, contour in enumerate(contours):
            output.append(f"    {spec.prefix}Contour {{")
            output.append(
                f"        start: "
                f"{rust_array([centipx(contour.start[0]), centipx(contour.start[1])])},"
            )
            output.append(f"        segments: &{text_spec.const_name}_CONTOUR_{index},")
            output.append("    },")
        output.append("];")
        output.append("")

        output.append(
            f"pub const {text_spec.const_name}: {spec.definition_name} = "
            f"{spec.definition_name} {{"
        )
        output.append(f"    text: {json.dumps(text_spec.label)},")
        output.append(f"    define_text_id: {text_spec.text_id},")
        if spec.font_id is None:
            output.append(f"    font_ids: &{text_spec.const_name}_FONT_IDS,")
        else:
            output.append("    font_id: FONT_ID,")
        output.append(f"    color_rgb: {rust_array(color)},")
        output.append(f"    bounds_centipx: {rust_array(bounds)},")
        output.append(f"    contours: &{text_spec.const_name}_CONTOURS,")
        output.append("};")
        output.append("")

    return "\n".join(output)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("swf", type=Path)
    parser.add_argument("--module", choices=MODULES.keys(), required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    spec = MODULES[args.module]
    wanted_text_ids = module_ids(spec)
    fonts: dict[int, tuple[tuple[Contour, ...], ...]] = {}
    texts: dict[int, TextDefinition] = {}

    for tag_code, body in iter_tags(swf_body(args.swf)):
        if tag_code == 10:
            font_id, glyphs = parse_define_font(body)
            fonts[font_id] = glyphs
        elif tag_code == 11:
            text_id = struct.unpack_from("<H", body, 0)[0]
            if text_id in wanted_text_ids:
                texts[text_id] = parse_define_text(body)

    missing = wanted_text_ids - set(texts)
    if missing:
        raise SystemExit(f"missing DefineText ids: {sorted(missing)}")

    args.out.write_text(generate_module(spec=spec, texts=texts, fonts=fonts), encoding="utf-8")


if __name__ == "__main__":
    main()

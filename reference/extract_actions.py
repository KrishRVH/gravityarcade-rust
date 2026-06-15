#!/usr/bin/env python3
"""Extract SWF5/ActionScript 1 action records from gravity_arcade.swf.

This is not a full decompiler. It preserves enough structure to recover the
original frame scripts, movie-clip handlers, and button handlers while keeping
the extraction auditable for the Rust port.
"""

from __future__ import annotations

import argparse
import json
import struct
import zlib
from dataclasses import dataclass
from pathlib import Path
from typing import Any


TAG_NAMES = {
    0: "End",
    1: "ShowFrame",
    2: "DefineShape",
    4: "PlaceObject",
    5: "RemoveObject",
    6: "DefineBits",
    7: "DefineButton",
    8: "JPEGTables",
    9: "SetBackgroundColor",
    10: "DefineFont",
    11: "DefineText",
    12: "DoAction",
    13: "DefineFontInfo",
    14: "DefineSound",
    15: "StartSound",
    17: "DefineButtonSound",
    18: "SoundStreamHead",
    19: "SoundStreamBlock",
    20: "DefineBitsLossless",
    21: "DefineBitsJPEG2",
    22: "DefineShape2",
    24: "Protect",
    26: "PlaceObject2",
    28: "RemoveObject2",
    32: "DefineShape3",
    33: "DefineText2",
    34: "DefineButton2",
    35: "DefineBitsJPEG3",
    36: "DefineBitsLossless2",
    37: "DefineEditText",
    39: "DefineSprite",
    43: "FrameLabel",
    45: "SoundStreamHead2",
    46: "DefineMorphShape",
    48: "DefineFont2",
    56: "ExportAssets",
    59: "DoInitAction",
}


ACTION_NAMES = {
    0x04: "NextFrame",
    0x05: "PreviousFrame",
    0x06: "Play",
    0x07: "Stop",
    0x08: "ToggleQuality",
    0x09: "StopSounds",
    0x0A: "Add",
    0x0B: "Subtract",
    0x0C: "Multiply",
    0x0D: "Divide",
    0x0E: "Equals",
    0x0F: "Less",
    0x10: "And",
    0x11: "Or",
    0x12: "Not",
    0x13: "StringEquals",
    0x14: "StringLength",
    0x15: "StringExtract",
    0x17: "Pop",
    0x18: "ToInteger",
    0x1C: "GetVariable",
    0x1D: "SetVariable",
    0x20: "SetTarget2",
    0x21: "StringAdd",
    0x22: "GetProperty",
    0x23: "SetProperty",
    0x24: "CloneSprite",
    0x25: "RemoveSprite",
    0x26: "Trace",
    0x27: "StartDrag",
    0x28: "EndDrag",
    0x29: "StringLess",
    0x30: "RandomNumber",
    0x31: "MBStringLength",
    0x32: "Ord",
    0x33: "Chr",
    0x34: "GetTimer",
    0x35: "MBStringExtract",
    0x36: "MBOrd",
    0x37: "MBChr",
    0x3A: "Delete",
    0x3B: "Delete2",
    0x3C: "DefineLocal",
    0x3D: "CallFunction",
    0x3E: "Return",
    0x3F: "Modulo",
    0x40: "NewObject",
    0x41: "DefineLocal2",
    0x42: "InitArray",
    0x43: "InitObject",
    0x44: "TypeOf",
    0x45: "TargetPath",
    0x46: "Enumerate",
    0x47: "Add2",
    0x48: "Less2",
    0x49: "Equals2",
    0x4A: "ToNumber",
    0x4B: "ToString",
    0x4C: "PushDuplicate",
    0x4D: "StackSwap",
    0x4E: "GetMember",
    0x4F: "SetMember",
    0x50: "Increment",
    0x51: "Decrement",
    0x52: "CallMethod",
    0x53: "NewMethod",
    0x54: "InstanceOf",
    0x55: "Enumerate2",
    0x60: "BitAnd",
    0x61: "BitOr",
    0x62: "BitXor",
    0x63: "BitLShift",
    0x64: "BitRShift",
    0x65: "BitURShift",
    0x66: "StrictEquals",
    0x67: "Greater",
    0x68: "StringGreater",
    0x69: "Extends",
    0x81: "GotoFrame",
    0x83: "GetURL",
    0x87: "StoreRegister",
    0x88: "ConstantPool",
    0x8A: "WaitForFrame",
    0x8B: "SetTarget",
    0x8C: "GoToLabel",
    0x8D: "WaitForFrame2",
    0x8E: "DefineFunction2",
    0x8F: "Try",
    0x94: "With",
    0x96: "Push",
    0x99: "Jump",
    0x9A: "GetURL2",
    0x9B: "DefineFunction",
    0x9D: "If",
    0x9E: "Call",
    0x9F: "GotoFrame2",
}


PROPERTIES = {
    0: "_x",
    1: "_y",
    2: "_xscale",
    3: "_yscale",
    4: "_currentframe",
    5: "_totalframes",
    6: "_alpha",
    7: "_visible",
    8: "_width",
    9: "_height",
    10: "_rotation",
    11: "_target",
    12: "_framesloaded",
    13: "_name",
    14: "_droptarget",
    15: "_url",
    16: "_highquality",
    17: "_focusrect",
    18: "_soundbuftime",
    19: "_quality",
    20: "_xmouse",
    21: "_ymouse",
}


CLIP_EVENTS = {
    0x0001: "load",
    0x0002: "enterFrame",
    0x0004: "unload",
    0x0008: "mouseMove",
    0x0010: "mouseDown",
    0x0020: "mouseUp",
    0x0040: "keyDown",
    0x0080: "keyUp",
    0x0100: "data",
    0x0200: "initialize",
    0x0400: "press",
    0x0800: "release",
    0x1000: "releaseOutside",
    0x2000: "rollOver",
    0x4000: "rollOut",
    0x8000: "dragOver",
    0x10000: "dragOut",
    0x20000: "keyPress",
    0x40000: "construct",
}


BUTTON_EVENTS = {
    0x0001: "idleToOverUp",
    0x0002: "overUpToIdle",
    0x0004: "overUpToOverDown",
    0x0008: "overDownToOverUp",
    0x0010: "overDownToOutDown",
    0x0020: "outDownToOverDown",
    0x0040: "outDownToIdle",
    0x0080: "idleToOverDown",
    0x0100: "overDownToIdle",
    0x0200: "overDownToOutDown",
}


@dataclass
class Bits:
    data: bytes
    byte: int = 0
    bit: int = 0

    def ub(self, n: int) -> int:
        value = 0
        for _ in range(n):
            value = (value << 1) | ((self.data[self.byte] >> (7 - self.bit)) & 1)
            self.bit += 1
            if self.bit == 8:
                self.bit = 0
                self.byte += 1
        return value

    def sb(self, n: int) -> int:
        value = self.ub(n)
        if n and value & (1 << (n - 1)):
            value -= 1 << n
        return value

    def fb(self, n: int) -> float:
        return self.sb(n) / 65536.0

    def align(self) -> None:
        if self.bit:
            self.bit = 0
            self.byte += 1


def cstr(data: bytes, pos: int) -> tuple[str, int]:
    end = data.index(b"\x00", pos)
    return data[pos:end].decode("latin-1"), end + 1


def read_rect(bits: Bits) -> tuple[int, int, int, int]:
    nbits = bits.ub(5)
    rect = tuple(bits.sb(nbits) for _ in range(4))
    bits.align()
    return rect


def read_matrix(data: bytes, pos: int) -> tuple[dict[str, float], int]:
    bits = Bits(data, pos)
    sx = sy = 1.0
    r0 = r1 = 0.0
    if bits.ub(1):
        nbits = bits.ub(5)
        sx = bits.fb(nbits)
        sy = bits.fb(nbits)
    if bits.ub(1):
        nbits = bits.ub(5)
        r0 = bits.fb(nbits)
        r1 = bits.fb(nbits)
    nbits = bits.ub(5)
    tx = bits.sb(nbits) / 20.0
    ty = bits.sb(nbits) / 20.0
    bits.align()
    return {"sx": sx, "sy": sy, "r0": r0, "r1": r1, "tx": tx, "ty": ty}, bits.byte


def skip_cxform(data: bytes, pos: int, alpha: bool) -> int:
    bits = Bits(data, pos)
    has_add = bits.ub(1)
    has_mult = bits.ub(1)
    nbits = bits.ub(4)
    channels = 4 if alpha else 3
    for _ in range((has_add + has_mult) * channels):
        bits.sb(nbits)
    bits.align()
    return bits.byte


def event_names(flags: int, names: dict[int, str]) -> list[str]:
    return [name for bit, name in names.items() if flags & bit]


def parse_push(data: bytes, constants: list[str]) -> list[Any]:
    out: list[Any] = []
    pos = 0
    while pos < len(data):
        push_type = data[pos]
        pos += 1
        if push_type == 0:
            value, pos = cstr(data, pos)
            out.append(value)
        elif push_type == 1:
            out.append(struct.unpack_from("<f", data, pos)[0])
            pos += 4
        elif push_type == 2:
            out.append(None)
        elif push_type == 3:
            out.append({"undefined": True})
        elif push_type == 4:
            out.append({"register": data[pos]})
            pos += 1
        elif push_type == 5:
            out.append(bool(data[pos]))
            pos += 1
        elif push_type == 6:
            raw = data[pos : pos + 8]
            # SWF doubles are stored with the two 32-bit words swapped.
            out.append(struct.unpack("<d", raw[4:8] + raw[0:4])[0])
            pos += 8
        elif push_type == 7:
            out.append(struct.unpack_from("<i", data, pos)[0])
            pos += 4
        elif push_type == 8:
            idx = data[pos]
            pos += 1
            out.append(constants[idx] if idx < len(constants) else {"constant8": idx})
        elif push_type == 9:
            idx = struct.unpack_from("<H", data, pos)[0]
            pos += 2
            out.append(constants[idx] if idx < len(constants) else {"constant16": idx})
        else:
            out.append({"unknown_push_type": push_type, "remaining_hex": data[pos:].hex()})
            break
    return out


def parse_actions(data: bytes, inherited_constants: list[str] | None = None) -> list[dict[str, Any]]:
    actions: list[dict[str, Any]] = []
    constants: list[str] = list(inherited_constants or [])
    pos = 0
    while pos < len(data):
        offset = pos
        code = data[pos]
        pos += 1
        if code == 0:
            actions.append({"offset": offset, "code": code, "op": "End"})
            break
        length = 0
        body = b""
        if code >= 0x80:
            length = struct.unpack_from("<H", data, pos)[0]
            pos += 2
            body = data[pos : pos + length]
            pos += length
        action: dict[str, Any] = {
            "offset": offset,
            "code": code,
            "op": ACTION_NAMES.get(code, f"Action_{code:02x}"),
        }
        if code >= 0x80:
            action["length"] = length
        if code == 0x81:
            action["frame"] = struct.unpack_from("<H", body, 0)[0]
        elif code == 0x83:
            url, p = cstr(body, 0)
            target, _ = cstr(body, p)
            action.update(url=url, target=target)
        elif code == 0x87:
            action["register"] = body[0]
        elif code == 0x88:
            count = struct.unpack_from("<H", body, 0)[0]
            p = 2
            constants = []
            for _ in range(count):
                value, p = cstr(body, p)
                constants.append(value)
            action["constants"] = constants
        elif code == 0x8A:
            action["frame"] = struct.unpack_from("<H", body, 0)[0]
            action["skip_count"] = body[2]
        elif code == 0x8B:
            target, _ = cstr(body, 0)
            action["target"] = target
        elif code == 0x8C:
            label, _ = cstr(body, 0)
            action["label"] = label
        elif code == 0x8D:
            action["skip_count"] = body[0]
        elif code == 0x94:
            size = struct.unpack_from("<H", body, 0)[0]
            action["size"] = size
            action["body"] = parse_actions(data[pos : pos + size], constants)
            pos += size
        elif code == 0x96:
            action["values"] = parse_push(body, constants)
        elif code in (0x99, 0x9D):
            branch = struct.unpack_from("<h", body, 0)[0]
            action["branch_offset"] = branch
            action["target_offset"] = pos + branch
        elif code == 0x9A:
            flags = body[0]
            action["send_vars_method"] = flags & 0x03
            action["load_target"] = bool(flags & 0x40)
            action["load_variables"] = bool(flags & 0x80)
        elif code == 0x9B:
            name, p = cstr(body, 0)
            count = struct.unpack_from("<H", body, p)[0]
            p += 2
            params = []
            for _ in range(count):
                value, p = cstr(body, p)
                params.append(value)
            size = struct.unpack_from("<H", body, p)[0]
            p += 2
            action.update(name=name, params=params, body_size=size)
            action["body"] = parse_actions(data[pos : pos + size], constants)
            pos += size
        elif code == 0x9F:
            flags = body[0] if body else 0
            action["play"] = bool(flags & 0x01)
            action["scene_bias_flag"] = bool(flags & 0x02)
            if flags & 0x02 and len(body) >= 3:
                action["scene_bias"] = struct.unpack_from("<H", body, 1)[0]
        actions.append(action)
    return actions


def parse_button2_actions(body: bytes, swf_version: int) -> list[dict[str, Any]]:
    del swf_version
    if len(body) < 5:
        return []
    action_offset = struct.unpack_from("<H", body, 3)[0]
    if action_offset == 0:
        return []
    p = 5
    while p < len(body) and body[p] != 0:
        p += 1
        p += 2
        p += 2
        _, p = read_matrix(body, p)
        p = skip_cxform(body, p, True)
    if p >= len(body):
        return []
    p += 1
    handlers: list[dict[str, Any]] = []
    while p < len(body):
        size = struct.unpack_from("<H", body, p)[0]
        p += 2
        flags = struct.unpack_from("<H", body, p)[0]
        p += 2
        end = len(body) if size == 0 else p - 4 + size
        handlers.append(
            {
                "conditions": flags,
                "events": event_names(flags, BUTTON_EVENTS),
                "actions": parse_actions(body[p:end]),
            }
        )
        p = end
        if size == 0:
            break
    return handlers


def parse_clip_actions(data: bytes, pos: int, swf_version: int) -> tuple[list[dict[str, Any]], int]:
    del swf_version
    p = pos
    # ClipActions starts with a reserved UI16 in SWF5.
    p += 2
    all_flags = struct.unpack_from("<H", data, p)[0]
    p += 2
    records: list[dict[str, Any]] = []
    while p < len(data):
        flags = struct.unpack_from("<H", data, p)[0]
        p += 2
        if flags == 0:
            break
        size = struct.unpack_from("<I", data, p)[0]
        p += 4
        key_code = None
        action_start = p
        if flags & 0x20000:
            key_code = data[p]
            p += 1
            action_start = p
        action_end = p + size - (action_start - (p if key_code is None else p - 1))
        records.append(
            {
                "flags": flags,
                "events": event_names(flags, CLIP_EVENTS),
                "key_code": key_code,
                "actions": parse_actions(data[action_start:action_end]),
            }
        )
        p = action_end
    return [{"all_flags": all_flags, "records": records}], p


def parse_place_object2_clip_actions(
    body: bytes, frame: int, sprite_id: int | None, swf_version: int
) -> list[dict[str, Any]]:
    if not body:
        return []
    flags = body[0]
    if not (flags & 0x80):
        return []
    depth = struct.unpack_from("<H", body, 1)[0]
    p = 3
    char_id = None
    name = None
    if flags & 0x02:
        char_id = struct.unpack_from("<H", body, p)[0]
        p += 2
    if flags & 0x04:
        _, p = read_matrix(body, p)
    if flags & 0x08:
        p = skip_cxform(body, p, True)
    if flags & 0x10:
        p += 2
    if flags & 0x20:
        name, p = cstr(body, p)
    if flags & 0x40:
        p += 2
    clips, _ = parse_clip_actions(body, p, swf_version)
    out: list[dict[str, Any]] = []
    for clip in clips:
        for record in clip["records"]:
            out.append(
                {
                    "kind": "clip",
                    "sprite": sprite_id,
                    "frame": frame,
                    "depth": depth,
                    "char_id": char_id,
                    "name": name,
                    "events": record["events"],
                    "actions": record["actions"],
                }
            )
    return out


def parse_tags(
    data: bytes,
    pos: int,
    end: int,
    swf_version: int,
    out: list[dict[str, Any]],
    sprite_id: int | None = None,
) -> None:
    frame = 1
    while pos < end:
        code_len = struct.unpack_from("<H", data, pos)[0]
        pos += 2
        code = code_len >> 6
        length = code_len & 0x3F
        if length == 0x3F:
            length = struct.unpack_from("<I", data, pos)[0]
            pos += 4
        body = data[pos : pos + length]
        pos += length
        if code == 0:
            break
        if code == 1:
            frame += 1
            continue
        if code == 12:
            out.append(
                {
                    "kind": "frame",
                    "tag": TAG_NAMES[code],
                    "sprite": sprite_id,
                    "frame": frame,
                    "actions": parse_actions(body),
                }
            )
        elif code == 43:
            label, _ = cstr(body, 0)
            out.append(
                {
                    "kind": "label",
                    "tag": TAG_NAMES[code],
                    "sprite": sprite_id,
                    "frame": frame,
                    "label": label,
                }
            )
        elif code == 26:
            out.extend(parse_place_object2_clip_actions(body, frame, sprite_id, swf_version))
        elif code == 34:
            button_id = struct.unpack_from("<H", body, 0)[0]
            for handler in parse_button2_actions(body, swf_version):
                out.append(
                    {
                        "kind": "button",
                        "button_id": button_id,
                        "events": handler["events"],
                        "conditions": handler["conditions"],
                        "actions": handler["actions"],
                    }
                )
        elif code == 39:
            nested_sprite = struct.unpack_from("<H", body, 0)[0]
            parse_tags(data=body, pos=4, end=len(body), swf_version=swf_version, out=out, sprite_id=nested_sprite)
        elif code == 59:
            sprite_ref = struct.unpack_from("<H", body, 0)[0]
            out.append(
                {
                    "kind": "init",
                    "sprite_ref": sprite_ref,
                    "sprite": sprite_id,
                    "frame": frame,
                    "actions": parse_actions(body[2:]),
                }
            )


def load_swf(path: Path) -> tuple[dict[str, Any], bytes, int]:
    raw = path.read_bytes()
    signature = raw[:3]
    version = raw[3]
    file_len = struct.unpack_from("<I", raw, 4)[0]
    if signature == b"CWS":
        data = raw[:8] + zlib.decompress(raw[8:])
    else:
        data = raw
    bits = Bits(data, 8)
    stage = read_rect(bits)
    pos = bits.byte
    frac = data[pos]
    integer = data[pos + 1]
    frames = struct.unpack_from("<H", data, pos + 2)[0]
    header = {
        "signature": signature.decode("latin-1"),
        "version": version,
        "file_len": file_len,
        "stage_twips": stage,
        "stage_px": [v / 20.0 for v in stage],
        "fps": integer + frac / 256.0,
        "frames": frames,
        "first_tag_pos": pos + 4,
    }
    return header, data, pos + 4


def compact_action(action: dict[str, Any]) -> str:
    op = action["op"]
    if op == "DefineFunction":
        name = action.get("name") or "<anonymous>"
        params = ", ".join(action.get("params", []))
        return (
            f'{action["offset"]:04x}: DefineFunction {name}({params}) '
            f'body_size={action.get("body_size")}'
        )
    if op == "With":
        return f'{action["offset"]:04x}: With size={action.get("size")}'
    if op == "Push":
        return f'{action["offset"]:04x}: Push {action.get("values", [])!r}'
    if op == "ConstantPool":
        return f'{action["offset"]:04x}: ConstantPool {action.get("constants", [])!r}'
    if op in ("If", "Jump"):
        return f'{action["offset"]:04x}: {op} -> {action.get("target_offset")}'
    details = []
    for key in ("frame", "label", "target", "url", "register", "play", "send_vars_method"):
        if key in action:
            details.append(f"{key}={action[key]!r}")
    suffix = " " + " ".join(details) if details else ""
    return f'{action["offset"]:04x}: {op}{suffix}'


def compact_actions(actions: list[dict[str, Any]], indent: str = "") -> list[str]:
    lines: list[str] = []
    for action in actions:
        lines.append(f"{indent}{compact_action(action)}")
        body = action.get("body")
        if isinstance(body, list):
            lines.extend(compact_actions(body, indent + "  "))
    return lines


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("swf", type=Path)
    parser.add_argument("--json", type=Path)
    parser.add_argument("--text", type=Path)
    args = parser.parse_args()

    header, data, pos = load_swf(args.swf)
    records: list[dict[str, Any]] = []
    parse_tags(data, pos, len(data), header["version"], records)
    payload = {"header": header, "records": records}

    if args.json:
        args.json.write_text(json.dumps(payload, indent=2), encoding="utf-8")
    if args.text:
        lines = [json.dumps(header, sort_keys=True), ""]
        for rec in records:
            if rec["kind"] == "frame":
                title = f'frame {rec["frame"]}'
                if rec.get("sprite") is not None:
                    title = f'sprite {rec["sprite"]} frame {rec["frame"]}'
            elif rec["kind"] == "clip":
                title = (
                    f'clip sprite={rec.get("sprite")} frame={rec["frame"]} '
                    f'depth={rec["depth"]} char={rec.get("char_id")} '
                    f'name={rec.get("name")} events={",".join(rec["events"])}'
                )
            elif rec["kind"] == "button":
                title = f'button {rec["button_id"]} events={",".join(rec["events"])}'
            elif rec["kind"] == "label":
                title = f'label {rec["label"]!r} frame {rec["frame"]}'
                if rec.get("sprite") is not None:
                    title = f'sprite {rec["sprite"]} {title}'
            else:
                title = f'init sprite_ref={rec.get("sprite_ref")} frame={rec.get("frame")}'
            lines.append(f"## {title}")
            if "actions" in rec:
                lines.extend(compact_actions(rec["actions"]))
            lines.append("")
        args.text.write_text("\n".join(lines), encoding="utf-8")
    if not args.json and not args.text:
        print(json.dumps(payload, indent=2))
    else:
        print(f"{len(records)} action records extracted")


if __name__ == "__main__":
    main()

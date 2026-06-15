#!/usr/bin/env python3
"""Extract MP3 DefineSound payloads from gravity_arcade.swf.

Flash MP3 DefineSound data starts with a signed UI16 seek-samples value before
the first MP3 frame. Runtime audio players expect the frame stream only, so the
extractor strips those two bytes and writes normal `.mp3` files.
"""

from __future__ import annotations

import argparse
import json
import struct
from pathlib import Path


SOUND_NAMES = {
    1: "reflect",
    6: "merge",
    12: "shot",
    85: "score_line",
    131: "paddle_stun",
    149: "round_lost",
    156: "round_start",
    159: "blue_match_win",
    161: "red_match_win",
}

RATES = [5512, 11025, 22050, 44100]
FORMATS = {
    0: "pcm_be",
    1: "adpcm",
    2: "mp3",
    3: "pcm_le",
    6: "nellymoser",
}


class Bits:
    def __init__(self, data: bytes, pos: int = 0) -> None:
        self.data = data
        self.byte = pos
        self.bit = 0

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

    def align(self) -> None:
        if self.bit:
            self.bit = 0
            self.byte += 1


def skip_rect(bits: Bits) -> None:
    nbits = bits.ub(5)
    for _ in range(4):
        bits.sb(nbits)
    bits.align()


def iter_tags(data: bytes, pos: int, end: int, sprite: int | None = None):
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
        yield sprite, frame, code, body
        if code == 39:
            nested_sprite = struct.unpack_from("<H", body, 0)[0]
            yield from iter_tags(body, 4, len(body), nested_sprite)


def extract(swf_path: Path, out_dir: Path) -> list[dict[str, object]]:
    raw = swf_path.read_bytes()
    bits = Bits(raw, 8)
    skip_rect(bits)
    pos = bits.byte + 4
    out_dir.mkdir(parents=True, exist_ok=True)

    manifest: list[dict[str, object]] = []
    for sprite, frame, code, body in iter_tags(raw, pos, len(raw)):
        if code != 14:
            continue
        sound_id = struct.unpack_from("<H", body, 0)[0]
        flags = body[2]
        sound_format = (flags >> 4) & 0x0F
        rate = RATES[(flags >> 2) & 0x03]
        bits_per_sample = 16 if flags & 0x02 else 8
        channels = 2 if flags & 0x01 else 1
        sample_count = struct.unpack_from("<I", body, 3)[0]
        payload = body[7:]
        if sound_format == 2:
            payload = payload[2:]
        name = SOUND_NAMES.get(sound_id, f"sound_{sound_id}")
        ext = "mp3" if sound_format == 2 else FORMATS.get(sound_format, "bin")
        target = out_dir / f"{name}.{ext}"
        target.write_bytes(payload)
        manifest.append(
            {
                "id": sound_id,
                "name": name,
                "file": target.as_posix(),
                "format": FORMATS.get(sound_format, str(sound_format)),
                "rate": rate,
                "bits": bits_per_sample,
                "channels": channels,
                "samples": sample_count,
                "sprite": sprite,
                "frame": frame,
                "bytes": len(payload),
            }
        )
    return manifest


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("swf", type=Path)
    parser.add_argument("--out", type=Path, default=Path("assets/sounds"))
    parser.add_argument("--manifest", type=Path, default=Path("reference/sounds.json"))
    args = parser.parse_args()

    manifest = extract(args.swf, args.out)
    args.manifest.write_text(json.dumps(manifest, indent=2), encoding="utf-8")
    print(f"extracted {len(manifest)} sounds")


if __name__ == "__main__":
    main()

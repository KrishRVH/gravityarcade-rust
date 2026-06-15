#!/usr/bin/env python3
"""Apply SWF SoundInfo envelopes to runtime WAV assets."""

from __future__ import annotations

import argparse
import shutil
import struct
import wave
from dataclasses import dataclass
from pathlib import Path
from typing import Iterator


DEFINE_SPRITE = 39
DEFINE_BUTTON_SOUND = 17
DEFINE_SOUND = 14
SHOW_FRAME = 1
START_SOUND = 15
END = 0

SOUND_NAMES = {
    1: "reflect",
    6: "merge",
    12: "shot",
    23: "sound_23",
    24: "sound_24",
    85: "score_line",
    131: "paddle_stun",
    149: "round_lost",
    156: "round_start",
    159: "blue_match_win",
    161: "red_match_win",
}

RUNTIME_SOUND_INFO_SOURCES = {
    "sound_23": ("button", 25, "idleToOverUp"),
    "merge": ("start", 8, 2, 6),
    "shot": ("start", 16, 1, 12),
    "paddle_stun": ("start", 132, 4, 131),
    "round_lost": ("start", 162, 8, 149),
    "round_start": ("start", 162, 212, 156),
}

BUTTON_SOUND_TRANSITIONS = [
    "overUpToIdle",
    "idleToOverUp",
    "overUpToOverDown",
    "overDownToOverUp",
]


@dataclass(frozen=True)
class SwfTag:
    sprite: int | None
    frame: int
    code: int
    body: bytes


@dataclass(frozen=True)
class SoundInfo:
    in_point_44100: int | None
    out_point_44100: int | None
    loop_count: int | None
    envelope: tuple[tuple[int, int, int], ...]

    @property
    def has_runtime_transform(self) -> bool:
        return (
            self.in_point_44100 is not None
            or self.out_point_44100 is not None
            or self.envelope
        )


class Bits:
    def __init__(self, data: bytes, pos: int = 0) -> None:
        self.data = data
        self.byte = pos
        self.bit = 0

    def ub(self, bits: int) -> int:
        value = 0
        for _ in range(bits):
            value = (value << 1) | ((self.data[self.byte] >> (7 - self.bit)) & 1)
            self.bit += 1
            if self.bit == 8:
                self.bit = 0
                self.byte += 1
        return value

    def sb(self, bits: int) -> int:
        value = self.ub(bits)
        if bits and value & (1 << (bits - 1)):
            value -= 1 << bits
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


def first_tag_offset(data: bytes) -> int:
    bits = Bits(data, 8)
    skip_rect(bits)
    return bits.byte + 4


def iter_tags(data: bytes, pos: int, end: int, sprite: int | None = None) -> Iterator[SwfTag]:
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
        if code == END:
            break
        if code == SHOW_FRAME:
            frame += 1
            continue
        yield SwfTag(sprite, frame, code, body)
        if code == DEFINE_SPRITE:
            nested_sprite = struct.unpack_from("<H", body, 0)[0]
            yield from iter_tags(body, 4, len(body), nested_sprite)


def read_sound_info(body: bytes, offset: int) -> tuple[SoundInfo, int]:
    flags = body[offset]
    offset += 1
    in_point = None
    out_point = None
    loop_count = None
    envelope: list[tuple[int, int, int]] = []
    if flags & 0x01:
        in_point = struct.unpack_from("<I", body, offset)[0]
        offset += 4
    if flags & 0x02:
        out_point = struct.unpack_from("<I", body, offset)[0]
        offset += 4
    if flags & 0x04:
        loop_count = struct.unpack_from("<H", body, offset)[0]
        offset += 2
    if flags & 0x08:
        envelope_count = body[offset]
        offset += 1
        for _ in range(envelope_count):
            mark, left, right = struct.unpack_from("<IHH", body, offset)
            offset += 8
            envelope.append((mark, left, right))
    return SoundInfo(in_point, out_point, loop_count, tuple(envelope)), offset


def collect_start_sound_infos(data: bytes) -> dict[tuple[int | None, int, int], SoundInfo]:
    infos = {}
    for tag in iter_tags(data, first_tag_offset(data), len(data)):
        if tag.code != START_SOUND:
            continue
        sound_id = struct.unpack_from("<H", tag.body, 0)[0]
        sound_info, offset = read_sound_info(tag.body, 2)
        if offset != len(tag.body):
            raise SystemExit(f"unparsed StartSound bytes for sound {sound_id}")
        infos[(tag.sprite, tag.frame, sound_id)] = sound_info
    return infos


def collect_mp3_seek_samples(data: bytes) -> dict[int, int]:
    seek_samples = {}
    for tag in iter_tags(data, first_tag_offset(data), len(data)):
        if tag.code != DEFINE_SOUND:
            continue
        sound_id = struct.unpack_from("<H", tag.body, 0)[0]
        flags = tag.body[2]
        sound_format = (flags >> 4) & 0x0F
        if sound_format == 2:
            seek_samples[sound_id] = struct.unpack_from("<h", tag.body, 7)[0]
    return seek_samples


def collect_button_sound_infos(data: bytes) -> dict[tuple[int, str], tuple[int, SoundInfo]]:
    infos = {}
    for tag in iter_tags(data, first_tag_offset(data), len(data)):
        if tag.code != DEFINE_BUTTON_SOUND:
            continue
        button_id = struct.unpack_from("<H", tag.body, 0)[0]
        offset = 2
        for transition in BUTTON_SOUND_TRANSITIONS:
            sound_id = struct.unpack_from("<H", tag.body, offset)[0]
            offset += 2
            if sound_id:
                sound_info, offset = read_sound_info(tag.body, offset)
                infos[(button_id, transition)] = (sound_id, sound_info)
        if offset != len(tag.body):
            raise SystemExit(f"unparsed DefineButtonSound bytes for button {button_id}")
    return infos


def sound_info_for_runtime_name(
    runtime_name: str,
    start_infos: dict[tuple[int | None, int, int], SoundInfo],
    button_infos: dict[tuple[int, str], tuple[int, SoundInfo]],
) -> tuple[int, SoundInfo | None]:
    source = RUNTIME_SOUND_INFO_SOURCES.get(runtime_name)
    if source is None:
        sound_id = next(
            sound_id for sound_id, name in SOUND_NAMES.items() if name == runtime_name
        )
        return sound_id, None
    if source[0] == "start":
        _, sprite, frame, sound_id = source
        return sound_id, start_infos[(sprite, frame, sound_id)]
    _, button_id, transition = source
    sound_id, sound_info = button_infos[(button_id, transition)]
    return sound_id, sound_info


def mark_to_sample(mark_44100: int, sample_rate: int) -> int:
    return round(mark_44100 * sample_rate / 44_100)


def envelope_levels(envelope: tuple[tuple[int, int, int], ...], sample_count: int, sample_rate: int) -> list[tuple[float, float]]:
    if not envelope:
        return [(1.0, 1.0)] * sample_count

    points = [
        (
            max(0, min(sample_count, mark_to_sample(mark, sample_rate))),
            left / 32_768.0,
            right / 32_768.0,
        )
        for mark, left, right in envelope
    ]
    points.sort(key=lambda point: point[0])
    levels: list[tuple[float, float]] = []
    point_index = 0
    for sample_index in range(sample_count):
        while point_index + 1 < len(points) and sample_index > points[point_index + 1][0]:
            point_index += 1
        current = points[point_index]
        if sample_index <= current[0]:
            levels.append((current[1], current[2]))
            continue
        if point_index + 1 >= len(points):
            levels.append((current[1], current[2]))
            continue
        next_point = points[point_index + 1]
        span = next_point[0] - current[0]
        if span <= 0:
            levels.append((next_point[1], next_point[2]))
            continue
        t = (sample_index - current[0]) / span
        left = current[1] + (next_point[1] - current[1]) * t
        right = current[2] + (next_point[2] - current[2]) * t
        levels.append((left, right))
    return levels


def clamp_i16(value: float) -> int:
    return max(-32_768, min(32_767, round(value)))


def apply_sound_info(
    input_wav: Path,
    output_wav: Path,
    sound_info: SoundInfo | None,
    seek_samples: int,
) -> None:
    seek_samples = max(0, seek_samples)
    if seek_samples == 0 and (sound_info is None or not sound_info.has_runtime_transform):
        shutil.copyfile(input_wav, output_wav)
        return

    with wave.open(str(input_wav), "rb") as source:
        channels = source.getnchannels()
        sample_width = source.getsampwidth()
        sample_rate = source.getframerate()
        source_sample_count = source.getnframes()
        frames = source.readframes(source_sample_count)

    if sample_width != 2:
        raise SystemExit(f"{input_wav}: expected 16-bit WAV, got {sample_width * 8}-bit")

    seek_samples = min(source_sample_count, seek_samples)
    sample_count = source_sample_count - seek_samples
    start = 0
    end = sample_count
    if sound_info is not None and sound_info.in_point_44100 is not None:
        start = mark_to_sample(sound_info.in_point_44100, sample_rate)
    if sound_info is not None and sound_info.out_point_44100 is not None:
        end = mark_to_sample(sound_info.out_point_44100, sample_rate)
    start = max(0, min(sample_count, start))
    end = max(start, min(sample_count, end))

    samples = list(struct.unpack("<" + "h" * (len(frames) // 2), frames))
    envelope = sound_info.envelope if sound_info is not None else ()
    levels = envelope_levels(envelope, sample_count, sample_rate)
    processed: list[int] = []
    for sample_index in range(start, end):
        source_sample_index = seek_samples + sample_index
        left_level, right_level = levels[sample_index]
        if channels == 1:
            level = (left_level + right_level) / 2.0
            processed.append(clamp_i16(samples[source_sample_index] * level))
        elif channels == 2:
            base = source_sample_index * 2
            processed.append(clamp_i16(samples[base] * left_level))
            processed.append(clamp_i16(samples[base + 1] * right_level))
        else:
            raise SystemExit(f"{input_wav}: unsupported channel count {channels}")

    output_wav.parent.mkdir(parents=True, exist_ok=True)
    with wave.open(str(output_wav), "wb") as target:
        target.setnchannels(channels)
        target.setsampwidth(sample_width)
        target.setframerate(sample_rate)
        target.writeframes(struct.pack("<" + "h" * len(processed), *processed))


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("swf", type=Path)
    parser.add_argument("--raw-dir", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    swf = args.swf.read_bytes()
    seek_samples = collect_mp3_seek_samples(swf)
    start_infos = collect_start_sound_infos(swf)
    button_infos = collect_button_sound_infos(swf)
    args.out.mkdir(parents=True, exist_ok=True)
    for sound_id, runtime_name in SOUND_NAMES.items():
        source_id, sound_info = sound_info_for_runtime_name(
            runtime_name, start_infos, button_infos
        )
        if source_id != sound_id:
            raise SystemExit(f"{runtime_name}: expected sound {sound_id}, got {source_id}")
        apply_sound_info(
            args.raw_dir / f"{runtime_name}.wav",
            args.out / f"{runtime_name}.wav",
            sound_info,
            seek_samples.get(sound_id, 0),
        )


if __name__ == "__main__":
    main()

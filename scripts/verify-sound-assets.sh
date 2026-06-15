#!/usr/bin/env bash
set -euo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$ROOT"

OUT_DIR="$ROOT/target/generated-sounds"
MP3_DIR="$OUT_DIR/mp3"
WAV_DIR="$OUT_DIR/wav"
RUNTIME_WAV_DIR="$OUT_DIR/runtime-wav"
MP3_MANIFEST="$OUT_DIR/mp3_manifest.json"
WAV_MANIFEST="$OUT_DIR/sounds.json"

mkdir -p "$MP3_DIR" "$WAV_DIR" "$RUNTIME_WAV_DIR"
find "$MP3_DIR" -maxdepth 1 -type f -name '*.mp3' -delete
find "$WAV_DIR" -maxdepth 1 -type f -name '*.wav' -delete
find "$RUNTIME_WAV_DIR" -maxdepth 1 -type f -name '*.wav' -delete

PYTHONDONTWRITEBYTECODE=1 python3 - <<'PY'
from reference.apply_sound_info_to_wavs import envelope_levels

levels = envelope_levels(((4, 16384, 8192), (8, 32768, 32768)), 10, 44_100)
assert levels[0] == (0.5, 0.25)
assert levels[4] == (0.5, 0.25)
assert levels[6] == (0.75, 0.625)
assert levels[9] == (1.0, 1.0)
PY

python3 reference/extract_sounds.py gravity_arcade.swf --out "$MP3_DIR" --manifest "$MP3_MANIFEST" >/dev/null

for mp3 in "$MP3_DIR"/*.mp3; do
    wav="$WAV_DIR/$(basename "${mp3%.mp3}").wav"
    ffmpeg -y -hide_banner -loglevel error -i "$mp3" "$wav"
done

python3 reference/apply_sound_info_to_wavs.py \
    gravity_arcade.swf \
    --raw-dir "$WAV_DIR" \
    --out "$RUNTIME_WAV_DIR"

python3 reference/write_wav_sound_manifest.py \
    "$MP3_MANIFEST" \
    --wav-dir "$RUNTIME_WAV_DIR" \
    --out "$WAV_MANIFEST"

cmp "$WAV_MANIFEST" reference/sounds.json

generated_count=$(find "$RUNTIME_WAV_DIR" -maxdepth 1 -type f -name '*.wav' | wc -l)
asset_count=$(find assets/sounds -maxdepth 1 -type f -name '*.wav' | wc -l)
if [[ "$generated_count" != "$asset_count" ]]; then
    echo "generated $generated_count sound assets, but assets/sounds has $asset_count" >&2
    exit 1
fi

for wav in "$RUNTIME_WAV_DIR"/*.wav; do
    cmp "$wav" "assets/sounds/$(basename "$wav")"
done

echo "ok SWF sound WAV assets match checked-in runtime sounds"

#!/usr/bin/env python3
"""Write the checked-in WAV sound manifest from extracted SWF sound metadata."""

from __future__ import annotations

import argparse
import json
from pathlib import Path


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("mp3_manifest", type=Path)
    parser.add_argument("--wav-dir", type=Path, required=True)
    parser.add_argument("--asset-dir", default="assets/sounds")
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    manifest = json.loads(args.mp3_manifest.read_text(encoding="utf-8"))
    for entry in manifest:
        name = entry["name"]
        wav_file = args.wav_dir / f"{name}.wav"
        entry["file"] = f"{args.asset_dir}/{name}.wav"
        entry["format"] = "wav"
        entry["bytes"] = wav_file.stat().st_size

    args.out.write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")


if __name__ == "__main__":
    main()

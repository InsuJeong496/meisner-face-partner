# /// script
# requires-python = ">=3.12"
# dependencies = [
#   "supertonic==1.3.1",
# ]
# ///

from __future__ import annotations

import os
import tempfile
import winsound
from pathlib import Path

from supertonic import TTS


def play_speech(text: str) -> None:
    engine = TTS(model="supertonic-3", auto_download=True)
    voice_style = engine.get_voice_style("F2")
    waveform, _ = engine.synthesize(
        text=text,
        voice_style=voice_style,
        total_steps=10,
        speed=0.95,
        lang="ko",
        verbose=False,
    )

    descriptor, raw_path = tempfile.mkstemp(prefix="meisner-supertonic-", suffix=".wav")
    os.close(descriptor)
    output_path = Path(raw_path)

    try:
        engine.save_audio(waveform, str(output_path))
        winsound.PlaySound(str(output_path), winsound.SND_FILENAME)
    finally:
        output_path.unlink(missing_ok=True)


def main() -> None:
    text = os.environ.get("MEISNER_TTS_TEXT", "").strip()
    if text:
        play_speech(text)


if __name__ == "__main__":
    main()

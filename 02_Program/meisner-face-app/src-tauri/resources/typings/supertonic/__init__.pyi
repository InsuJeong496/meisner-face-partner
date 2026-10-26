import numpy as np
import numpy.typing as npt

class Style: ...

class TTS:
    def __init__(
        self,
        model: str = "supertonic-3",
        model_dir: str | None = None,
        auto_download: bool = True,
        intra_op_num_threads: int | None = None,
        inter_op_num_threads: int | None = None,
    ) -> None: ...
    def get_voice_style(self, voice_name: str) -> Style: ...
    def synthesize(
        self,
        text: str,
        voice_style: Style,
        total_steps: int = 8,
        speed: float = 1.05,
        max_chunk_length: int | None = None,
        silence_duration: float = 0.3,
        lang: str | None = None,
        verbose: bool = False,
    ) -> tuple[npt.NDArray[np.float32], npt.NDArray[np.float32]]: ...
    def save_audio(self, wav: npt.NDArray[np.float32], output_path: str) -> None: ...

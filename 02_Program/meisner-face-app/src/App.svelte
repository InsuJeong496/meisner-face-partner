<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { debounce } from "es-toolkit";
  import { onDestroy, onMount } from "svelte";

  type PracticeMode = "appearance" | "change";

  type SpeechResult = {
    readonly backend: string;
    readonly voice: string;
  };

  const ffhqTeaserUrl =
    "https://raw.githubusercontent.com/NVlabs/ffhq-dataset/master/ffhq-teaser.png";
  const waveformBarIndexes: readonly number[] = Array.from({ length: 24 }, (_, index) => index);
  const restingWaveform: readonly number[] = waveformBarIndexes.map(
    (index) => 0.1 + (index % 4) * 0.025,
  );

  let mode: PracticeMode = "appearance";
  let observation = "너는 나를 바라보고 있어";
  let response = "그러면 나는 나를 바라보고 있어";
  let isSpeaking = false;
  let isListening = false;
  let isPreparingMicrophone = false;
  let speechRecognitionSupported = false;
  let waveformLevels: readonly number[] = restingWaveform;
  let statusMessage = "상대 얼굴을 보고 현재 보이는 행동을 말해보세요.";
  let activeRecognition: BrowserSpeechRecognition | null = null;
  let microphoneStream: MediaStream | null = null;
  let microphoneContext: AudioContext | null = null;
  let waveformFrameId: number | null = null;
  let microphoneRequestGeneration = 0;

  onMount(() => {
    speechRecognitionSupported = Boolean(
      window.SpeechRecognition ?? window.webkitSpeechRecognition,
    );
  });

  onDestroy(() => {
    activeRecognition?.stop();
    activeRecognition = null;
    void stopMicrophoneMonitor();
  });

  const examples: Record<PracticeMode, string[]> = {
    appearance: [
      "너는 정면을 보고 있어",
      "너는 입을 다물고 있어",
      "너는 눈을 크게 뜨고 있어",
    ],
    change: [
      "너는 조금 더 긴장하고 있어",
      "너는 아까보다 천천히 숨 쉬고 있어",
      "너는 방금 시선을 피하고 있어",
    ],
  };

  function fallbackTransform(value: string): string {
    const trimmed = value.trim();
    if (!trimmed) {
      return "그러면 나는 지금 아무 말도 하지 않고 있어";
    }

    let next = trimmed
      .replace(/^너는\s*/, "그러면 나는 ")
      .replace(/^당신은\s*/, "그러면 나는 ")
      .replace(/^네가\s*/, "그러면 내가 ")
      .replace(/^너가\s*/, "그러면 내가 ");

    if (next === trimmed) {
      next = `그러면 나는 ${trimmed}`;
    }

    return next;
  }

  async function transformObservation(value: string): Promise<string> {
    try {
      return await invoke<string>("transform_observation", { observation: value });
    } catch {
      return fallbackTransform(value);
    }
  }

  const previewObservation = debounce((value: string) => {
    response = fallbackTransform(value);
  }, 180);

  function speakWithBrowser(text: string) {
    window.speechSynthesis.cancel();
    const utterance = new SpeechSynthesisUtterance(text);
    utterance.lang = "ko-KR";
    utterance.rate = 0.92;
    utterance.pitch = 1.0;
    utterance.onstart = () => {
      isSpeaking = true;
      statusMessage = "상대가 당신의 관찰을 반복하고 있습니다.";
    };
    utterance.onend = () => {
      isSpeaking = false;
      statusMessage = "다음 외형 또는 변화 관찰을 이어가세요.";
    };
    utterance.onerror = () => {
      isSpeaking = false;
      statusMessage = "TTS 재생에 실패했습니다. 입력 문장은 변환되었습니다.";
    };
    window.speechSynthesis.speak(utterance);
  }

  async function speak(text: string) {
    isSpeaking = true;
    statusMessage =
      "고품질 음성을 준비하고 있습니다. 첫 실행은 모델 준비에 시간이 걸릴 수 있습니다.";

    try {
      const result = await invoke<SpeechResult>("speak_text", { text });
      isSpeaking = false;
      if (result.backend === "supertonic-3") {
        statusMessage = `Supertonic 3 ${result.voice} 음성으로 반복했습니다. 다음 관찰을 이어가세요.`;
      } else {
        statusMessage = result.voice
          ? `${result.voice} 폴백 음성으로 반복했습니다. 다음 관찰을 이어가세요.`
          : "다음 외형 또는 변화 관찰을 이어가세요.";
      }
    } catch {
      speakWithBrowser(text);
    }
  }

  async function submitObservation() {
    response = await transformObservation(observation);
    await speak(response);
  }

  function useExample(example: string) {
    observation = example;
    response = fallbackTransform(example);
    statusMessage = "예시 관찰문이 입력되었습니다.";
  }

  async function stopMicrophoneMonitor(): Promise<void> {
    microphoneRequestGeneration += 1;

    if (waveformFrameId !== null) {
      window.cancelAnimationFrame(waveformFrameId);
      waveformFrameId = null;
    }

    microphoneStream?.getTracks().forEach((track) => track.stop());
    microphoneStream = null;

    const context = microphoneContext;
    microphoneContext = null;
    waveformLevels = restingWaveform;

    if (context && context.state !== "closed") {
      await context.close();
    }
  }

  async function startMicrophoneMonitor(): Promise<boolean> {
    if (!navigator.mediaDevices?.getUserMedia) {
      statusMessage = "이 환경에서는 마이크 입력 파형을 표시할 수 없습니다.";
      return false;
    }
    if (!window.AudioContext) {
      statusMessage = "이 WebView에서는 실시간 녹음 파형을 지원하지 않습니다.";
      return false;
    }

    const requestGeneration = microphoneRequestGeneration + 1;
    microphoneRequestGeneration = requestGeneration;

    try {
      const stream = await navigator.mediaDevices.getUserMedia({
        audio: {
          autoGainControl: true,
          echoCancellation: true,
          noiseSuppression: true,
        },
      });

      if (requestGeneration !== microphoneRequestGeneration) {
        stream.getTracks().forEach((track) => track.stop());
        return false;
      }

      const context = new window.AudioContext();
      const analyser = context.createAnalyser();
      const source = context.createMediaStreamSource(stream);

      analyser.fftSize = 1024;
      analyser.smoothingTimeConstant = 0.72;
      const samples = new Uint8Array(analyser.fftSize);
      source.connect(analyser);

      microphoneStream = stream;
      microphoneContext = context;

      if (context.state === "suspended") {
        await context.resume();
      }

      if (requestGeneration !== microphoneRequestGeneration) {
        stream.getTracks().forEach((track) => track.stop());
        if (context.state !== "closed") {
          await context.close();
        }
        return false;
      }

      const updateWaveform = () => {
        analyser.getByteTimeDomainData(samples);
        const segmentSize = Math.floor(samples.length / waveformBarIndexes.length);

        waveformLevels = waveformBarIndexes.map((index) => {
          const start = index * segmentSize;
          const end = Math.min(start + segmentSize, samples.length);
          let squareTotal = 0;

          for (let sampleIndex = start; sampleIndex < end; sampleIndex += 1) {
            const centeredSample = (samples[sampleIndex] - 128) / 128;
            squareTotal += centeredSample * centeredSample;
          }

          const rootMeanSquare = Math.sqrt(squareTotal / Math.max(end - start, 1));
          return Math.min(1, Math.max(0.1, 0.1 + rootMeanSquare * 5.5));
        });

        waveformFrameId = window.requestAnimationFrame(updateWaveform);
      };

      updateWaveform();
      return true;
    } catch (error) {
      await stopMicrophoneMonitor();
      if (error instanceof DOMException) {
        statusMessage =
          error.name === "NotAllowedError"
            ? "녹음 파형을 표시하려면 마이크 권한을 허용해 주세요."
            : "마이크를 시작하지 못했습니다. 다른 앱이 사용 중인지 확인해 주세요.";
        return false;
      }
      throw error;
    }
  }

  async function acceptRecognitionResult(
    event: BrowserSpeechRecognitionEvent,
    recognition: BrowserSpeechRecognition,
  ): Promise<void> {
    observation = event.results[0][0].transcript;
    recognition.stop();
    isListening = false;
    await stopMicrophoneMonitor();
    statusMessage = "말한 내용을 관찰문으로 받았습니다.";
    await submitObservation();
  }

  async function startListening(): Promise<void> {
    const Recognition = window.SpeechRecognition ?? window.webkitSpeechRecognition;
    if (!Recognition) {
      statusMessage = "이 WebView에서는 음성 인식이 지원되지 않습니다. 텍스트 입력을 사용하세요.";
      return;
    }

    isPreparingMicrophone = true;
    statusMessage = "마이크 권한을 확인하고 있습니다.";
    let monitorStarted = false;
    try {
      monitorStarted = await startMicrophoneMonitor();
    } catch (error) {
      await stopMicrophoneMonitor();
      if (error instanceof Error) {
        statusMessage = "마이크 처리 초기화에 실패했습니다. 텍스트 입력을 사용해 주세요.";
      } else {
        throw error;
      }
    }
    isPreparingMicrophone = false;
    if (!monitorStarted) {
      return;
    }

    const recognition = new Recognition();
    activeRecognition = recognition;
    recognition.lang = "ko-KR";
    recognition.interimResults = false;
    recognition.continuous = false;
    recognition.onresult = (event) => {
      void acceptRecognitionResult(event, recognition);
    };
    recognition.onend = () => {
      activeRecognition = null;
      isListening = false;
      void stopMicrophoneMonitor();
    };
    recognition.onerror = () => {
      activeRecognition = null;
      isListening = false;
      statusMessage = "음성 인식에 실패했습니다. 다시 말하거나 텍스트로 입력하세요.";
      void stopMicrophoneMonitor();
    };

    isListening = true;
    statusMessage = "듣고 있습니다. '너는 ~하고 있어'라고 말해보세요.";
    try {
      recognition.start();
    } catch (error) {
      activeRecognition = null;
      isListening = false;
      await stopMicrophoneMonitor();
      if (error instanceof DOMException) {
        statusMessage = "음성 인식을 시작하지 못했습니다. 잠시 후 다시 시도해 주세요.";
        return;
      }
      throw error;
    }
  }

  $: previewObservation(observation);
</script>

<main class="app-shell">
  <section class="stage-panel" aria-label="상대 얼굴 관찰 영역">
    <div class="source-chip">Temporary FFHQ reference</div>
    <div class="face-card">
      <div
        class="face-crop"
        aria-label="FFHQ 임시 얼굴 이미지"
        style:background-image={`url("${ffhqTeaserUrl}")`}
      ></div>
      <div class="attention-ring"></div>
    </div>
    <div class="partner-copy">
      <p class="eyebrow">Meisner repetition partner</p>
      <h1>현재 순간을 보고 말하기</h1>
      <p>
        지금은 얼굴 생성 없이 FFHQ 샘플 이미지를 임시 상대역으로 사용합니다. 상대의 외형이나
        방금 보이는 변화를 “너는 ~하고 있어”로 말하면, 앱이 “그러면 나는 ~하고 있어”로
        받아칩니다.
      </p>
    </div>
  </section>

  <section class="practice-panel" aria-label="반복 훈련 입력 영역">
    <div class="mode-tabs" aria-label="관찰 모드">
      <button class:active={mode === "appearance"} type="button" onclick={() => (mode = "appearance")}>
        1. 외형 관찰
      </button>
      <button class:active={mode === "change"} type="button" onclick={() => (mode = "change")}>
        2. 변화 관찰
      </button>
    </div>

    <label for="observation">내 관찰문</label>
    <textarea
      id="observation"
      bind:value={observation}
      rows="4"
      placeholder="너는 나를 바라보고 있어"
    ></textarea>

    <div class="button-row">
      <button class="primary" type="button" onclick={submitObservation} disabled={isSpeaking}>
        TTS로 반복하기
      </button>
      {#if speechRecognitionSupported}
        <button
          type="button"
          onclick={startListening}
          disabled={isListening || isPreparingMicrophone || isSpeaking}
        >
          {isPreparingMicrophone ? "연결 중" : isListening ? "듣는 중" : "말로 입력"}
        </button>
      {/if}
    </div>

    {#if speechRecognitionSupported}
      <div
        class="microphone-monitor"
        class:active={isListening}
        class:connecting={isPreparingMicrophone}
        role="status"
        aria-live="polite"
      >
        <div class="microphone-state">
          <span class="recording-dot" aria-hidden="true"></span>
          <strong>
            {isPreparingMicrophone ? "마이크 연결 중" : isListening ? "음성 입력 중" : "마이크 대기"}
          </strong>
          <span>{isListening ? "목소리 크기를 확인하고 있습니다" : "말로 입력을 누르면 파형이 표시됩니다"}</span>
        </div>
        <div class="waveform" aria-hidden="true">
          {#each waveformLevels as level, index (index)}
            <span style:transform={`scaleY(${level})`}></span>
          {/each}
        </div>
      </div>
    {/if}

    <div class="response-card" aria-live="polite">
      <span>상대의 응답</span>
      <strong>{response}</strong>
    </div>

    <p class="status">{statusMessage}</p>

    <div class="examples" aria-label="관찰문 예시">
      <span>예시</span>
      {#each examples[mode] as example}
        <button type="button" onclick={() => useExample(example)}>{example}</button>
      {/each}
    </div>
  </section>
</main>

type BrowserSpeechRecognitionResult = {
  readonly 0: {
    readonly transcript: string;
  };
};

type BrowserSpeechRecognitionEvent = Event & {
  readonly results: {
    readonly 0: BrowserSpeechRecognitionResult;
  };
};

type BrowserSpeechRecognition = EventTarget & {
  lang: string;
  interimResults: boolean;
  continuous: boolean;
  start: () => void;
  stop: () => void;
  onresult: ((event: BrowserSpeechRecognitionEvent) => void) | null;
  onend: (() => void) | null;
  onerror: (() => void) | null;
};

type BrowserSpeechRecognitionConstructor = new () => BrowserSpeechRecognition;

interface Window {
  SpeechRecognition?: BrowserSpeechRecognitionConstructor;
  webkitSpeechRecognition?: BrowserSpeechRecognitionConstructor;
}

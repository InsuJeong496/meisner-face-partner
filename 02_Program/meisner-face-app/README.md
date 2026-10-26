# Meisner Face Partner 앱 실행 안내

작성일: **2026년 10월 27일 (한국시간)**

**2026 Meta Global AI Developer Hackathon 참가 프로젝트**의 Windows 데스크톱 프로토타입입니다. 프로젝트 소개와 기존 코드·행사 신규 개발 범위는 [루트 README](../../README.md)를 참고하세요.

## 현재 기능

- 외형 관찰 / 변화 관찰 모드와 한국어 예시 문장
- “너는 정면을 보고 있어” → “그러면 나는 정면을 보고 있어” 형태의 규칙 기반 문장 변환
- Supertonic 3 → Windows 한국어 TTS → 브라우저 음성 합성 순서의 재생 경로
- Web Speech Recognition을 제공하는 환경에서 표시되는 음성 입력
- FFHQ teaser 이미지를 이용한 임시 상대 얼굴

이 브랜치의 Meta 모델 연동은 준비 중입니다. 별도 Python worker는 mock 응답만 반환하며, 얼굴 생성·분석 모델이나 실제 AI 연동 완료를 뜻하지 않습니다.

## 개발 환경

Windows 기준 준비물:

- Vite 6을 지원하는 Node.js와 pnpm
- Rust의 MSVC 툴체인 및 Visual Studio C++ Build Tools
- Microsoft Edge WebView2 Runtime
- 고품질 TTS를 사용하려면 `uv`를 PATH에 설치하고 Python 3.12 이상을 사용할 수 있어야 함
- Windows TTS 폴백을 사용하려면 System.Speech에서 사용할 수 있는 한국어 음성이 필요함

Supertonic 경로는 `uv run --script`로 실행됩니다. 첫 실행은 Python 의존성과 모델 다운로드에 네트워크 연결 및 추가 시간이 필요합니다. Supertonic 실행이 실패하면 Windows TTS를 시도하고, 네이티브 명령이 실패하면 브라우저 음성 합성을 시도합니다. 사용 가능한 한국어 음성은 환경마다 다릅니다.

## 데스크톱 실행

저장소 루트에서 실행합니다.

```powershell
pnpm --dir 02_Program/meisner-face-app install --frozen-lockfile
pnpm --dir 02_Program/meisner-face-app tauri dev
```

이미 pnpm 11로 설치된 `node_modules`를 사용하는 환경에서는 같은 pnpm 주 버전을 사용하세요. 새로운 체크아웃은 커밋된 lockfile로 설치하며 다른 작업 폴더의 `node_modules`를 복사하지 않습니다.

## 브라우저 미리보기

```powershell
pnpm --dir 02_Program/meisner-face-app dev
```

`http://localhost:1420`을 엽니다. 브라우저에서는 JavaScript 문장 변환과 브라우저 음성 기능을 사용하며 Rust 명령·Windows 네이티브 음성 경로는 검증되지 않습니다. 음성 입력 버튼은 해당 런타임의 Web Speech Recognition 지원 여부에 따라 표시됩니다.

## 연습 흐름

1. 외형 관찰 또는 변화 관찰 모드를 선택합니다.
2. 예시를 누르거나 관찰문을 입력합니다.
3. **TTS로 반복하기**를 눌러 변환된 문장을 듣습니다.
4. 다음 관찰문으로 반복을 이어갑니다.

마이크 입력과 한국어 음질, 실제 Windows 앱에서의 전체 연습 흐름은 기기에서 따로 확인해야 합니다.

## 검사

저장소 루트에서 PowerShell 7과 OpenSpec CLI를 사용합니다.

```powershell
pwsh -NoProfile -File scripts/verify.ps1
```

OpenSpec strict validation, Svelte/TypeScript 검사, Vite 빌드, Rust 테스트를 실행합니다. 결과는 로컬 `artifacts/verification/`에 저장됩니다. 실제 음성이 재생되는 Rust 테스트는 기본 제외됩니다.

## Python 실험 worker

```powershell
python 02_Program/meisner-face-app/model-worker/worker.py
```

이 worker는 앱 실행에 필수가 아닌 별도 mock 서비스입니다. 역할과 실행 주소는 [worker README](model-worker/README.md)를 참고하세요.

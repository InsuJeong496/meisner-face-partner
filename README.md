# Meisner Face Partner

### 2026 Meta Global AI Developer Hackathon · Participant Project

**README date / 작성일: October 27, 2026 · 2026년 10월 27일 (KST)**

**A Korean-language repetition practice partner for actors rehearsing on their own.**

Meisner Face Partner explores a simple practice loop: observe a partner, say what you notice, hear it repeated from the partner's perspective, and respond again. The goal is to make short, focused rehearsal sessions easier to start when another actor is unavailable.

**Status:** Prototype in development for the hackathon. Meta model integration and the submission demo are in preparation.

[About the hackathon](https://cerebralvalley.ai/e/meta-ai-developer-hackathon) · [Participant guide](https://cerebralvalley.ai/e/meta-ai-developer-hackathon/details) · [Run the app](#run-the-app) · [Development scope](#hackathon-development-scope) · [한국어 소개](#한국어-소개)

## The experience

1. Choose **appearance** or **change** observation mode.
2. Look at the reference portrait and enter a short observation in Korean.
3. Hear the partner repeat the observation from a first-person perspective.
4. Continue with the next observation.

```text
You       너는 정면을 보고 있어
Partner   그러면 나는 정면을 보고 있어
```

The current prototype uses rule-based text conversion and speech playback. The hackathon direction is to develop this into a more responsive practice partner using Meta models.

## Why this project

Repetition exercises depend on attention and exchange with a partner. This project focuses on the practical gap between wanting to rehearse and having someone available to rehearse with.

**Proposed challenge fit: Build for Your World.** The intended outcome is an accessible rehearsal tool built around a specific acting practice, with Meta AI contributing to the interaction inside the product.

## What works today

This describes the code in this branch, reviewed on **October 27, 2026**.

| Area | Current implementation |
| --- | --- |
| Practice interface | Appearance/change modes, Korean examples and text input |
| Repetition | Rule-based conversion of the opening pronoun into the partner's perspective |
| Speech output | Supertonic 3 on Windows, then Windows Korean TTS, with a browser speech fallback |
| Speech input | Available when the runtime exposes Web Speech Recognition; support varies by environment |
| Partner portrait | Temporary FFHQ reference image |
| Meta AI | Integration planned; no live Meta model call in this branch |
| Python worker | Separate mock service for future AI experiments |

Real microphone input, Korean audio quality and the complete native practice flow still need device-level verification. A successful web build or unit test does not establish those results.

## Meta AI in the hackathon build

The planned integration starts with **Korean speech transcription**, with Muse Voice Transcribe listed in the participant guide as a candidate. Model access, API behavior and quality must be verified before it is presented as a working feature. The final demo will identify the model actually used and show its role in the practice loop.

The current TTS and text-conversion paths are existing prototype functionality. They are not presented as Meta-powered features. AI-assisted development tools are documented separately from AI features used by the app.

## Hackathon development scope

This public repository history begins with a snapshot prepared on **October 27, 2026 (KST)**. The snapshot includes an existing local prototype. Its publication-history date does not represent the original creation date of every included file. Existing work and event-period contributions will be identified separately in the submission.

| Scope | Status |
| --- | --- |
| Existing prototype | Practice interface, rule-based repetition, speech playback and development setup |
| Public repository preparation | Project introduction and reproducible setup notes, dated October 27, 2026 |
| Event-period product contributions | Submission inventory pending integration and verification |

The **three-minute demo** will highlight only the features built during the event, with their source changes and verification evidence. Existing-code reuse eligibility for the Indie track and the guide/kickoff difference in the development start date still require organizer clarification.

**Submission deadline:** November 4, 2026, 9:00 PM EST / **November 5, 2026, 11:00 AM KST**. The demo video has not been published yet.

## Run the app

The desktop prototype targets **Windows**. Install Node.js compatible with Vite 6, pnpm, Rust, the MSVC C++ build tools and WebView2. See the [app setup guide](02_Program/meisner-face-app/README.md) for audio prerequisites and fallback behavior.

From the repository root:

```powershell
pnpm --dir 02_Program/meisner-face-app install --frozen-lockfile
pnpm --dir 02_Program/meisner-face-app tauri dev
```

For a browser preview:

```powershell
pnpm --dir 02_Program/meisner-face-app dev
```

Open `http://localhost:1420`. The browser preview uses browser fallbacks; it does not exercise native Tauri speech commands.

## Built with

**Svelte 5 · TypeScript · Vite · Tauri 2 · Rust**

Python supports the Windows Supertonic TTS helper and a separate experimental mock worker. OpenSpec defines the verification requirements.

| Location | Contents |
| --- | --- |
| [Application](02_Program/meisner-face-app/) | UI, native commands and setup instructions |
| [Verification requirements](openspec/specs/development-verification/spec.md) | Scope and acceptance criteria for checks |
| [Verification script](scripts/verify.ps1) | Specification, type, build and Rust checks |

With PowerShell 7 and the OpenSpec CLI installed:

```powershell
pwsh -NoProfile -File scripts/verify.ps1
```

The verification script writes local reports under `artifacts/verification/`. Generated reports, private research and local agent setup are excluded from this repository.

## Credits and third-party assets

- The temporary portrait reference comes from [NVIDIA's FFHQ dataset repository](https://github.com/NVlabs/ffhq-dataset). Follow its dataset and individual-image license and attribution requirements; final demo asset clearance is pending.
- The optional neural speech path uses **Supertonic 3**, through the `supertonic` Python package. Third-party model and dependency terms apply separately.
- A project source-code license has not yet been selected.

This is an independent participant project for the Meta Global AI Developer Hackathon.

## 한국어 소개

**Meisner Face Partner는 2026 Meta Global AI Developer Hackathon 참가를 위해 개발 중인 마이즈너 반복 연습 프로젝트입니다.** 혼자 연습하는 배우가 상대를 관찰하고, 짧은 관찰문을 입력하고, 상대 관점의 반복을 듣는 흐름을 제공합니다.

현재는 기존 프로토타입의 문장 변환과 한국어 음성 재생을 실행할 수 있습니다. Meta 모델 연결과 제출 데모는 준비 중이며, 행사에서 새로 개발한 기능은 기존 코드와 구분하여 소개합니다. 자세한 실행 방법은 [앱 README](02_Program/meisner-face-app/README.md)를 참고하세요.

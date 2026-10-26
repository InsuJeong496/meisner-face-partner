# Meisner Face App Design System

## 1. Atmosphere & Identity

The practice screen should feel focused, observant, and slightly theatrical without becoming decorative. Its signature is the split between a dark partner stage and a warm, work-focused practice surface; live microphone feedback uses the existing green accent as a truthful signal that the app is listening.

## 2. Color

| Role | Token / value | Usage |
| --- | --- | --- |
| Stage | `#152b35` | Partner side background |
| Practice surface | `#f7f4ef` | Input side background |
| App surface | `#f5f3ed` | Global and portrait frame |
| Elevated surface | `#ffffff` | Inputs, response, controls |
| Text primary | `#18212b` | Default text |
| Text strong | `#17202a` | Primary command and response |
| Text secondary | `#5d6670` / `#63707a` | Status and labels |
| Border | `#cfd7de` / `#d8dedf` | Inputs and cards |
| Accent | `#1d7a6b` | Active modes, focus, live microphone |
| Accent hover | `#1f7a6d` | Interactive hover |
| Warm highlight | `#ffb15c` | Stage eyebrow and attention frame |

Accent color communicates active input or interaction. Warm highlight remains exclusive to the partner stage.

## 3. Typography

- Primary stack: Inter, Pretendard, Malgun Gothic, system UI.
- Display: `clamp(36px, 5vw, 62px)`, weight 700, line height 1.05.
- Response: `clamp(24px, 3.2vw, 34px)`, weight 700, line height 1.28.
- Body: 16px, line height 1.6 to 1.72.
- Label: 14px, weight 800.
- Caption: 12px to 13px, weight 700 to 800.
- Korean copy uses `word-break: keep-all` and `overflow-wrap: break-word`.

## 4. Spacing & Layout

- Base spacing unit: 4px.
- Main shell: two columns above 900px, one document-flow column at and below 900px.
- Standard control height: at least 44px.
- Practice stack gap: 16px.
- Stage spacing: 28px, desktop padding 48px.
- Practice desktop padding: 56px; compact padding: 32px 22px.
- The microphone monitor has a stable block size so listening state changes do not shift nearby controls.

## 5. Components

### Segmented observation mode
- **Structure**: two real buttons in a grid.
- **States**: default, hover, active, focus, disabled.
- **Accessibility**: visible text and keyboard-native buttons.

### Practice input
- **Structure**: label, textarea, commands, response, status, examples.
- **States**: idle, speaking, listening, recognition error.
- **Accessibility**: explicit label, minimum target size, live response and status.

### Microphone monitor
- **Structure**: state label plus 24 vertical level bars.
- **States**: idle baseline and active audio-responsive waveform.
- **Accessibility**: `role="status"`, textual state is authoritative; bars are hidden from assistive technology.
- **Motion**: bars update from the Web Audio analyser only while listening. Reduced-motion mode keeps the monitor visible but removes easing between samples.
- **Layout**: fixed-height full-width band under the speech controls.

## 6. Motion & Interaction

- Reference mechanism: beui.dev `animated-badge` live-status feedback, adapted without adding a motion dependency.
- Micro state changes use 120ms ease-out opacity and transform transitions.
- The waveform visualizes measured microphone energy; it is not decorative or autonomous.
- Only `transform` and `opacity` animate.
- `prefers-reduced-motion: reduce` disables transition interpolation while preserving measured levels and state text.

## 7. Depth & Surface

Mixed strategy inherited from the existing screen: borders separate operational controls; the face portrait alone receives a prominent shadow to read as the stage focal object. The microphone monitor uses a border and subtle accent tint, not another floating card.

## 8. Accessibility Constraints & Accepted Debt

- Target WCAG 2.2 AA, visible focus, keyboard operation, 44px command targets, and non-color listening labels.
- Microphone permission failure must produce actionable Korean status copy.
- Audio tracks, the audio context, speech recognition, and animation frames must stop on recognition end, error, or component destruction.
- Existing raw CSS values predate this document. This change reuses those values and introduces no new palette role; token consolidation is intentionally outside this focused interaction change.

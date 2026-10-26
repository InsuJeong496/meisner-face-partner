use serde::Serialize;
use std::path::{Path, PathBuf};
use tauri::path::BaseDirectory;
use tauri::Manager;

#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
struct SpeechResult {
    backend: &'static str,
    voice: String,
}

#[tauri::command]
fn transform_observation(observation: String) -> String {
    let trimmed = observation.trim();

    if trimmed.is_empty() {
        return "그러면 나는 지금 아무 말도 하지 않고 있어".to_string();
    }

    let replacements = [
        ("너는", "그러면 나는"),
        ("당신은", "그러면 나는"),
        ("네가", "그러면 내가"),
        ("너가", "그러면 내가"),
    ];

    for (source, target) in replacements {
        if let Some(rest) = trimmed.strip_prefix(source) {
            let rest = rest.trim_start();
            return if rest.is_empty() {
                target.to_string()
            } else {
                format!("{target} {rest}")
            };
        }
    }

    format!("그러면 나는 {trimmed}")
}

#[tauri::command]
async fn speak_text(text: String, app: tauri::AppHandle) -> Result<SpeechResult, String> {
    let script_path = app
        .path()
        .resolve("resources/supertonic_tts.py", BaseDirectory::Resource)
        .map_err(|error| format!("TTS 리소스 경로를 찾지 못했습니다: {error}"))?;

    speak_text_with_script(text, script_path).await
}

async fn speak_text_with_script(
    text: String,
    script_path: PathBuf,
) -> Result<SpeechResult, String> {
    let trimmed = text.trim();

    if trimmed.is_empty() {
        return Ok(SpeechResult {
            backend: "none",
            voice: String::new(),
        });
    }

    let speech_text = trimmed.to_owned();
    tauri::async_runtime::spawn_blocking(move || {
        speak_text_with_fallback(
            &speech_text,
            |speech| speak_text_supertonic(speech, &script_path),
            speak_text_native,
        )
    })
    .await
    .map_err(|error| format!("TTS 작업을 완료하지 못했습니다: {error}"))?
}

fn speak_text_with_fallback<Neural, Native>(
    text: &str,
    neural: Neural,
    native: Native,
) -> Result<SpeechResult, String>
where
    Neural: FnOnce(&str) -> Result<SpeechResult, String>,
    Native: FnOnce(&str) -> Result<SpeechResult, String>,
{
    neural(text).or_else(|_| native(text))
}

#[cfg(target_os = "windows")]
fn speak_text_supertonic(text: &str, script_path: &Path) -> Result<SpeechResult, String> {
    use std::os::windows::process::CommandExt;

    const CREATE_NO_WINDOW: u32 = 0x0800_0000;

    if !script_path.is_file() {
        return Err(format!(
            "Supertonic Python 리소스를 찾을 수 없습니다: {}",
            script_path.display()
        ));
    }

    let mut command = std::process::Command::new("uv");
    command
        .args(["run", "--script"])
        .arg(script_path)
        .env("MEISNER_TTS_TEXT", text)
        .env("PYTHONUTF8", "1")
        .creation_flags(CREATE_NO_WINDOW);

    let output = command
        .output()
        .map_err(|error| format!("Supertonic 실행 환경을 시작하지 못했습니다: {error}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let detail = stderr.trim();
        return Err(if detail.is_empty() {
            format!("Supertonic이 종료 코드 {}로 실패했습니다.", output.status)
        } else {
            format!("Supertonic 음성 합성에 실패했습니다: {detail}")
        });
    }

    Ok(SpeechResult {
        backend: "supertonic-3",
        voice: "F2".to_string(),
    })
}

#[cfg(not(target_os = "windows"))]
fn speak_text_supertonic(_text: &str, _script_path: &Path) -> Result<SpeechResult, String> {
    Err("이 플랫폼에서는 Supertonic 재생이 아직 지원되지 않습니다.".to_string())
}

#[cfg(target_os = "windows")]
fn speak_text_native(text: &str) -> Result<SpeechResult, String> {
    let script = concat!(
        "$ErrorActionPreference = 'Stop';",
        "[Console]::OutputEncoding = [System.Text.UTF8Encoding]::new();",
        "Add-Type -AssemblyName System.Speech;",
        "$synth = New-Object System.Speech.Synthesis.SpeechSynthesizer;",
        "try {",
        "$voice = $synth.GetInstalledVoices() | Where-Object {",
        "$_.Enabled -and $_.VoiceInfo.Culture.Name -eq 'ko-KR'",
        "} | Sort-Object { $_.VoiceInfo.Name -like '*Desktop*' } | Select-Object -First 1;",
        "if ($null -eq $voice) { throw '설치된 한국어 음성을 찾을 수 없습니다.' };",
        "$synth.SelectVoice($voice.VoiceInfo.Name);",
        "$synth.Rate = 0;",
        "$synth.Volume = 100;",
        "$synth.Speak($env:MEISNER_TTS_TEXT);",
        "[Console]::Out.Write($voice.VoiceInfo.Name);",
        "} finally { $synth.Dispose() }"
    );

    let output = std::process::Command::new("powershell.exe")
        .args([
            "-NoLogo",
            "-NoProfile",
            "-NonInteractive",
            "-WindowStyle",
            "Hidden",
            "-Command",
            script,
        ])
        .env("MEISNER_TTS_TEXT", text)
        .output()
        .map_err(|error| format!("Windows TTS를 시작하지 못했습니다: {error}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let detail = stderr.trim();
        return Err(if detail.is_empty() {
            format!("Windows TTS가 종료 코드 {}로 실패했습니다.", output.status)
        } else {
            format!("Windows TTS 재생에 실패했습니다: {detail}")
        });
    }

    Ok(SpeechResult {
        backend: "windows-system-speech",
        voice: String::from_utf8_lossy(&output.stdout).trim().to_owned(),
    })
}

#[cfg(not(target_os = "windows"))]
fn speak_text_native(_text: &str) -> Result<SpeechResult, String> {
    Err("이 플랫폼에서는 native TTS가 아직 지원되지 않습니다.".to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![speak_text, transform_observation])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[cfg(target_os = "windows")]
    use super::speak_text_native;
    use super::speak_text_with_script;
    use super::transform_observation;
    use super::{speak_text_with_fallback, SpeechResult};

    #[test]
    fn transforms_second_person_observation_to_first_person() {
        assert_eq!(
            transform_observation("너는 나를 바라보고 있어".to_string()),
            "그러면 나는 나를 바라보고 있어"
        );
    }

    #[test]
    fn preserves_sentence_ending_when_transforming_observation() {
        assert_eq!(
            transform_observation("  너는 긴장하고 있어.  ".to_string()),
            "그러면 나는 긴장하고 있어."
        );
    }

    #[test]
    fn transforms_subject_form_when_observation_uses_nega() {
        assert_eq!(
            transform_observation("네가 웃고 있어".to_string()),
            "그러면 내가 웃고 있어"
        );
    }

    #[test]
    fn handles_empty_observation() {
        assert_eq!(
            transform_observation("  ".to_string()),
            "그러면 나는 지금 아무 말도 하지 않고 있어"
        );
    }

    #[test]
    fn accepts_empty_tts_text() {
        let result = tauri::async_runtime::block_on(speak_text_with_script(
            "  ".to_string(),
            PathBuf::new(),
        ));

        assert!(result.is_ok());
    }

    #[test]
    fn prefers_supertonic_when_neural_speech_succeeds() {
        let result = speak_text_with_fallback(
            "그러면 나는 긴장하고 있어.",
            |_| {
                Ok(SpeechResult {
                    backend: "supertonic-3",
                    voice: "F2".to_string(),
                })
            },
            |_| panic!("Windows fallback must not run after Supertonic succeeds"),
        )
        .expect("Supertonic result should be returned");

        assert_eq!(result.backend, "supertonic-3");
        assert_eq!(result.voice, "F2");
    }

    #[test]
    fn falls_back_to_windows_speech_when_supertonic_fails() {
        let result = speak_text_with_fallback(
            "그러면 나는 긴장하고 있어.",
            |_| Err("Supertonic is unavailable".to_string()),
            |_| {
                Ok(SpeechResult {
                    backend: "windows-system-speech",
                    voice: "Microsoft Heami Desktop".to_string(),
                })
            },
        )
        .expect("Windows fallback should recover from a Supertonic error");

        assert_eq!(result.backend, "windows-system-speech");
        assert_eq!(result.voice, "Microsoft Heami Desktop");
    }

    #[cfg(target_os = "windows")]
    #[test]
    #[ignore = "plays audible speech and requires an installed ko-KR voice"]
    fn speaks_korean_with_installed_windows_voice() {
        let result = speak_text_native("그러면 나는 긴장하고 있어.")
            .expect("the installed Korean voice should synthesize the test phrase");

        assert_eq!(result.backend, "windows-system-speech");
        assert!(!result.voice.is_empty());
        println!("backend={} voice={}", result.backend, result.voice);
    }
}

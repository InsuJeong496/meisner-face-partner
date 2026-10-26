# development-verification Specification

## Purpose
개발자와 에이전트가 동일한 명령으로 현재 프로젝트의 자동 검사를 실행하고, 검사한 코드와 결과를 연결하며 실제로 확인하지 않은 기능을 완료로 오인하지 않도록 한다.

## Requirements

### Requirement: Repeatable development checks
검증 진입점은 명세 유효성, 프론트엔드 타입, 웹 빌드와 백엔드 단위 테스트를 SHALL 검사한다.

#### Scenario: All automated checks succeed
- **WHEN** 필수 도구와 의존성이 준비된 프로젝트에서 검증을 실행하고 모든 검사가 통과한다
- **THEN** 실행은 종료 코드 0과 성공 보고서를 반환한다

### Requirement: Failed checks prevent success
검증 진입점은 검사 실패나 실행 도구 부재를 SHALL 실패로 기록하고 0이 아닌 코드로 종료한다. 실행하지 못한 후속 검사는 통과로 기록하지 않는다.

#### Scenario: Invalid change selection
- **WHEN** 존재하지 않는 변경을 대상으로 검증한다
- **THEN** 명세 검사가 실패하고 실행 결과는 실패이며 후속 검사는 실행되지 않은 것으로 남는다

### Requirement: Verification evidence identifies the inspected source
검증 보고서는 실행 시각, Git 커밋, 미커밋 변경 유무, 각 검사 상태·종료 코드·로그 위치를 SHALL 기록한다.

#### Scenario: Source has uncommitted edits
- **WHEN** 작업 파일을 변경한 상태에서 검증한다
- **THEN** 보고서는 미커밋 변경이 있음을 나타내고 검사 로그를 보존한다

### Requirement: Hardware verification remains explicit
자동 검증 보고서는 마이크 입력과 실제 한국어 음성 출력이 자동 검사에 포함되지 않았음을 SHALL 명시한다.

#### Scenario: Automated tests pass without audio playback
- **WHEN** 오디오 장치를 사용하지 않고 자동 검증이 통과한다
- **THEN** 보고서는 실제 음성 재생을 통과했다고 주장하지 않는다

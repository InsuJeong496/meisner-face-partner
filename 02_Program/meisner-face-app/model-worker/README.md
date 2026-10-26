# Python AI Worker

이 폴더는 얼굴 생성, 얼굴 관찰, identity preserving generation, talking face 같은 AI 기능을 Python으로 분리하기 위한 경계입니다.

현재 MVP 기능 1은 AI 모델 없이 Rust command와 Web Speech TTS만으로 동작합니다. 이후 기능에서 아래 worker를 실제 PyTorch/ONNX/FastAPI 기반 모델 서버로 교체합니다.

## 실행

```powershell
python .\model-worker\worker.py
```

기본 주소는 `http://127.0.0.1:8765` 입니다.

## 예정 역할

- 얼굴 검출 및 crop 품질 검사
- 사용자 얼굴 embedding 추출
- FFHQ 또는 임시 상대 얼굴 선택
- “상대방의 외형 관찰” 후보 생성
- “상대방의 변화 관찰” 후보 생성
- SDXL, InstantID, PhotoMaker, LivePortrait 같은 Python 모델 실행

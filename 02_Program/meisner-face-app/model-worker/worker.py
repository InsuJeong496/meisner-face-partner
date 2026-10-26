import json
from http.server import BaseHTTPRequestHandler, HTTPServer


HOST = "127.0.0.1"
PORT = 8765


class WorkerHandler(BaseHTTPRequestHandler):
    def _send_json(self, status: int, payload: dict[str, object]) -> None:
        data = json.dumps(payload, ensure_ascii=False).encode("utf-8")
        self.send_response(status)
        self.send_header("Content-Type", "application/json; charset=utf-8")
        self.send_header("Content-Length", str(len(data)))
        self.end_headers()
        self.wfile.write(data)

    def do_GET(self) -> None:
        if self.path == "/health":
            self._send_json(200, {"status": "ok", "backend": "python-ai-worker", "mode": "mock"})
            return

        self._send_json(404, {"error": "not_found"})

    def do_POST(self) -> None:
        length_header = self.headers.get("Content-Length", "0")
        try:
            length = int(length_header)
        except ValueError:
            length = 0

        raw_body = self.rfile.read(length) if length > 0 else b"{}"
        try:
            body = json.loads(raw_body.decode("utf-8"))
        except json.JSONDecodeError:
            self._send_json(400, {"error": "invalid_json"})
            return

        if self.path == "/observe":
            mode = body.get("mode")
            if mode == "change":
                observation = "너는 방금 시선을 조금 바꾸고 있어"
            else:
                observation = "너는 정면을 바라보고 있어"
            self._send_json(200, {"observation": observation, "mode": mode or "appearance"})
            return

        self._send_json(404, {"error": "not_found"})


def main() -> None:
    server = HTTPServer((HOST, PORT), WorkerHandler)
    print(f"Python AI worker listening on http://{HOST}:{PORT}")
    server.serve_forever()


if __name__ == "__main__":
    main()

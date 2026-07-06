import json
import urllib.request

def call_raw_http():
    url = "http://localhost:11434/v1/messages"
    headers = {
        "Content-Type": "application/json"
    }
    body = json.dumps({
        "model": "qwen3.5:0.8b",
        "max_tokens": 65536,
        "messages": [{"role": "user", "content": "What is a neural network in one sentence?"}],
        "options": {
            "temperature": 0.5,
            "top_p": 0.5
        }
    }).encode()

    req = urllib.request.Request(url, data=body, headers=headers, method="POST")
    with urllib.request.urlopen(req, timeout=30) as resp:
        raw = resp.read()

    if not raw:
        print("Raw HTTP response: <empty body>")
        return

    try:
        result = json.loads(raw)
    except json.JSONDecodeError:
        print(f"Raw HTTP response: <non-JSON body> {raw[:200]!r}")
        return

    # Anthropic-style shape: content is a list of blocks
    content_blocks = result.get("content") or []
    text = None
    for block in reversed(content_blocks):
        if isinstance(block, dict) and block.get("text"):
            text = block["text"]
            break

    # Fallback: OpenAI-style shape
    if text is None:
        choices = result.get("choices") or []
        if choices and isinstance(choices[0], dict):
            message = choices[0].get("message") or {}
            text = message.get("content")

    if text is None:
        text = "<no text field in response>"

    usage = result.get("usage") or {}
    in_tokens = usage.get("input_tokens", usage.get("prompt_tokens", 0))
    out_tokens = usage.get("output_tokens", usage.get("completion_tokens", 0))

    print(f"Raw HTTP response: {text}")
    print(f"Tokens used: {in_tokens} in, {out_tokens} out")

if __name__ == "__main__":
    print("=== API Calls ===\n")
    print("\n2. Using raw HTTP:")
    call_raw_http()

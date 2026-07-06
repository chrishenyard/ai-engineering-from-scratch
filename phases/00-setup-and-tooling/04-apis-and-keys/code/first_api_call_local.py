import os
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
    }).encode()

    req = urllib.request.Request(url, data=body, headers=headers, method="POST")
    with urllib.request.urlopen(req) as resp:
        result = json.loads(resp.read())
        print(f"Raw HTTP response: {result['content'][len(result['content']) - 1]['text']}")
        print(f"Tokens used: {result['usage']['input_tokens']} in, {result['usage']['output_tokens']} out")

if __name__ == "__main__":
    print("=== API Calls ===\n")
    # print("1. Using the SDK:")
    # call_with_sdk()
    print("\n2. Using raw HTTP:")
    call_raw_http()

import process from "node:process";

type Options = {
  temperature: number;
  top_p: number;
};

type MessagesRequest = {
  model: string;
  max_tokens: number;
  messages: { role: "user" | "assistant"; content: string }[];
  options: Options;
};

type MessagesResponse = {
  content: { type: string; text: string }[];
  usage: { input_cas: number; output_tokens: number };
};

async function callMessages(request: MessagesRequest): Promise<MessagesResponse> {
  const resp = await fetch("http://localhost:11434/v1/messages", {
    method: "POST",
    headers: {
      "content-type": "application/json",
    },
    body: JSON.stringify(request),
  });

  if (!resp.ok) {
    const body = await resp.text();
    throw new Error(`anthropic ${resp.status}: ${body.slice(0, 200)}`);
  }
  return (await resp.json()) as MessagesResponse;
}

async function main(): Promise<number> {
  process.stdout.write("=== API Calls ===\n\n");
  const request: MessagesRequest = {
    model: "qwen3.5:0.8b",
    max_tokens: 65536,
    messages: [{ role: "user", content: "What is a neural network in one sentence?" }],
    options: { temperature: 0.5, top_p: 0.5 },
  };

  try {
    const response = await callMessages(request);

    if (response.content.length === 0) {
      process.stderr.write("request failed: no content in response\n");
      return 1;
    }
    
    const text = response.content[response.content.length - 1].text ?? "";
    process.stdout.write(`response: ${text}\n`);
    process.stdout.write(
      `tokens: ${response.usage.input_tokens} in, ${response.usage.output_tokens} out\n`,
    );
    return 0;
  } catch (err) {
    process.stderr.write(`request failed: ${(err as Error).message}\n`);
    return 1;
  }
}

main().then((code) => process.exit(code));

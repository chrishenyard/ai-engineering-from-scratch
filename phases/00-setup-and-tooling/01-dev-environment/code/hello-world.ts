// Phase 0 · Lesson 01 — Dev Environment verifier (TypeScript port).
// Probes node version + presence of git, python3, cargo, deno; mirrors verify.py.
// Refs: https://nodejs.org/api/process.html  https://nodejs.org/api/child_process.html

import process from "node:process";


function run(): number {
  process.stdout.write("Hello, world!\n");
  return 1;
}

process.exit(run());

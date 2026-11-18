import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";

const repoRoot = fileURLToPath(new URL("../..", import.meta.url));
const protocolBindings = fileURLToPath(
  new URL("../../crates/pulse-protocol/bindings/protocol.ts", import.meta.url),
);

export default defineConfig({
  plugins: [react(), tailwindcss()],
  resolve: { alias: { "@pulse/protocol": protocolBindings } },
  server: { port: 5173, strictPort: true, fs: { allow: [repoRoot] } },
  test: { environment: "jsdom" },
});

import { useEffect, useState } from "react";

export default function useWasm(url: string) {
  const [wasmModule, setWasmModule] = useState(null);

  useEffect(() => {
    async function loadWasm() {
      try {
        const response = await fetch(url);
        const wasm = await WebAssembly.instantiateStreaming(response);
        setWasmModule(wasm.instance.exports);
      } catch (error) {
        console.error("Failed to load WASM module:", error);
      }
    }
    loadWasm();
  }, []);

  return wasmModule;
}

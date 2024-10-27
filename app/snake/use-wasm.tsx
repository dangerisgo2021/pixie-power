import { useEffect, useState } from "react";

export default function useWasm(url: string) {
  const [wasmModule, setWasmModule] = useState(null);

  useEffect(() => {
    async function loadWasm() {
      console.log({ url, wasmModule });

      try {
        const script = document.createElement("script");
        const body = document.getElementsByTagName("body")[0];
        script.src = url;
        body.appendChild(script);
        script.onload = () => {};
        const response = await fetch(url);
        console.log({ response, url, wasmModule });

        const x = await wasmModule.default();
        console.log({ response, x, wasmModule });
        //const wasm = await WebAssembly.instantiateStreaming(response);
        setWasmModule(wasm.instance.exports);
      } catch (error) {
        console.error("Failed to load WASM module:", error);
      }
    }
    loadWasm();
  }, []);

  return wasmModule;
}

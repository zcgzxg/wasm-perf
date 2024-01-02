onmessage = function (e) {
  importScripts("./pkg/b64_bench.js");

  wasm_bindgen("./pkg/b64_bench_bg.wasm").then(async (wasm) => {
    let len = 0;
    performance.mark("wasm-start");
    for (let i = 0; i < e.data; i++) {
      // len += wasm.generate_b64(100000, 1000);
      len += wasm.bench(100000, 1000);
      console.log(i, len);
    }
    console.log("wasm: len ", len);
    performance.measure("wasm", "wasm-start");
    this.postMessage(performance.getEntriesByName("wasm")[0].duration);
  });
};

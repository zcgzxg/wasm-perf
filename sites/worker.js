onmessage = function (e) {
  import("./pkg/b64_bench.js").then(async (res) => {
    await res.default();
    await res.initThreadPool(6);

    let len = 0;
    performance.mark("wasm-start");

    for (let i = 0; i < e.data; i++) {
      // len += res.generate_b64(10000, 1000);
      len += res.bench(10000, 1000);
    }

    console.log("wasm: len ", len);
    performance.measure("wasm", "wasm-start");
    this.postMessage(performance.getEntriesByName("wasm")[0].duration);
  });
};

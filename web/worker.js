import init, { ncm_dump } from "./pkg/ncmdump.js";

await init();
self.postMessage({ type: "ready" });

self.onmessage = (event) => {
  const { id, bytes } = event.data;

  try {
    const result = ncm_dump(bytes);
    const format = result.format;
    const data = result.data;
    result.free();
    self.postMessage({ id, ok: true, format, data }, [data.buffer]);
  } catch (err) {
    self.postMessage({ id, ok: false, error: String(err) });
  }
};

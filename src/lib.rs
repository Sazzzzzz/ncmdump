mod crypto;
pub mod ncm;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct NcmDumpResult {
    format: String,
    data: Vec<u8>,
}

#[wasm_bindgen]
impl NcmDumpResult {
    #[wasm_bindgen(getter)]
    pub fn format(&self) -> String {
        self.format.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn data(&self) -> Vec<u8> {
        self.data.clone()
    }
}

/// Decrypt an NCM file given its raw bytes.
/// Returns `{ format, data }` where `data` is the decrypted audio.
#[wasm_bindgen]
pub fn ncm_dump(input: &[u8]) -> Result<NcmDumpResult, JsValue> {
    ncm::ncm_dump_bytes(input)
        .map(|out| NcmDumpResult {
            format: out.format,
            data: out.data,
        })
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

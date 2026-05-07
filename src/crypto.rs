use aes::Aes128;
use aes::cipher::{BlockCipherDecrypt, KeyInit};
// cspell: disable-next-line
pub const CORE_KEY: &[u8; 16] = b"hzHRAmso5kInbaxW";
pub const META_KEY: &[u8; 16] = b"#14ljk_!\\]&0U<'(";

pub fn unpad(s: &[u8]) -> &[u8] {
    if s.is_empty() {
        return s;
    }
    let pad_len = s[s.len() - 1] as usize;
    if pad_len > 0 && pad_len <= s.len() {
        &s[..s.len() - pad_len]
    } else {
        s
    }
}

pub fn decrypt_aes_128_ecb(key: &[u8; 16], data: &mut [u8]) {
    let cipher = Aes128::new(key.into());
    for chunk in data.chunks_mut(16) {
        if chunk.len() == 16 {
            let block: &mut [u8; 16] = chunk.try_into().unwrap();
            cipher.decrypt_block(block.into());
        }
    }
}

pub fn build_key_box(key_data: &[u8]) -> [u8; 256] {
    let mut key_box: [u8; 256] = std::array::from_fn(|i| i as u8);
    let key_length = key_data.len();
    if key_length == 0 {
        return key_box;
    }
    let mut last_byte = 0usize;
    let mut key_offset = 0usize;

    for i in 0..256 {
        let swap = key_box[i];
        let c = (swap as usize + last_byte + key_data[key_offset] as usize) & 0xFF;
        key_offset = (key_offset + 1) % key_length;
        key_box[i] = key_box[c];
        key_box[c] = swap;
        last_byte = c;
    }
    key_box
}

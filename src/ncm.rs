use crate::crypto::{CORE_KEY, META_KEY, build_key_box, decrypt_aes_128_ecb, unpad};
use anyhow::{Context, Result, bail};
use base64::{Engine, engine::general_purpose::STANDARD as BASE64};
use serde::Deserialize;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NcmMetadata {
    pub format: String,
    // Other fields exist but format is the only one required for the output filename
}

pub fn dump<P: AsRef<Path>>(file_path: P) -> Result<PathBuf> {
    let file_path = file_path.as_ref();
    let mut f = File::open(file_path).context("Failed to open NCM file")?;

    let mut header = [0u8; 8];
    f.read_exact(&mut header).context("Failed to read header")?;
    //cspell: disable-next-line
    if header != b"CTENFDAM"[..] {
        bail!("Invalid NCM file header");
    }

    // Skip 2 bytes
    f.seek(SeekFrom::Current(2))?;

    // Read Key Data Length
    let mut len_buf = [0u8; 4];
    f.read_exact(&mut len_buf)
        .context("Failed to read key length")?;
    let key_length = u32::from_le_bytes(len_buf) as usize;

    // Read Key Data
    let mut key_data = vec![0u8; key_length];
    f.read_exact(&mut key_data)
        .context("Failed to read key data")?;

    for byte in &mut key_data {
        *byte ^= 0x64;
    }

    decrypt_aes_128_ecb(CORE_KEY, &mut key_data);
    let key_data_unpadded = unpad(&key_data);
    if key_data_unpadded.len() < 17 {
        bail!("Decrypted key data too short");
    }
    let actual_key_data = &key_data_unpadded[17..];
    let key_box = build_key_box(actual_key_data);

    // Read Meta Data Length
    f.read_exact(&mut len_buf)
        .context("Failed to read metadata length")?;
    let meta_length = u32::from_le_bytes(len_buf) as usize;

    // Read Meta Data
    let mut meta_data = vec![0u8; meta_length];
    f.read_exact(&mut meta_data)
        .context("Failed to read metadata")?;

    for byte in &mut meta_data {
        *byte ^= 0x63;
    }

    // skip "163 key(Don't modify):" which is 22 bytes
    if meta_data.len() < 22 {
        bail!("Meta data too short");
    }

    let decoded_meta = BASE64
        .decode(&meta_data[22..])
        .context("Failed to base64 decode meta")?;
    let mut decoded_meta = decoded_meta;

    decrypt_aes_128_ecb(META_KEY, &mut decoded_meta);
    let meta_unpadded = unpad(&decoded_meta);

    // skip "music:" which is 6 bytes
    if meta_unpadded.len() < 6 {
        bail!("Decrypted meta data too short");
    }

    let meta_json_str =
        std::str::from_utf8(&meta_unpadded[6..]).context("Invalid UTF-8 in metadata")?;

    let metadata: NcmMetadata =
        serde_json::from_str(meta_json_str).context("Failed to parse metadata JSON")?;

    // Skip 5 bytes: CRC32 (4 bytes) + 1 byte gap
    // (f.seek(5, 1) in both python and C++ reference)
    f.seek(SeekFrom::Current(5))?;

    // Read image frame length (total bytes in this section)
    f.read_exact(&mut len_buf)
        .context("Failed to read image frame length")?;
    let cover_frame_len = u32::from_le_bytes(len_buf) as u64;

    // Read actual image data length
    f.read_exact(&mut len_buf)
        .context("Failed to read image data length")?;
    let img_len = u32::from_le_bytes(len_buf) as u64;

    // Skip image data
    if img_len > 0 {
        f.seek(SeekFrom::Current(img_len as i64))?;
    }

    // Skip any remaining padding in the cover frame
    if cover_frame_len > img_len {
        f.seek(SeekFrom::Current((cover_frame_len - img_len) as i64))?;
    }

    // Create Output file path
    let file_name = format!(
        "{}.{}",
        file_path.file_stem().unwrap().to_string_lossy(),
        metadata.format
    );
    let out_path = file_path.with_file_name(&file_name);

    if out_path.exists() {
        eprintln!("警告: {} 已存在，该文件将被覆盖。", out_path.display());
    }

    let mut out_file = File::create(&out_path).context("Failed to create output file")?;
    let mut chunk = vec![0u8; 0x8000];

    loop {
        let n = f.read(&mut chunk)?;
        if n == 0 {
            break;
        }

        for i in 1..=n {
            let j = i & 0xFF;
            let idx =
                (key_box[j] as usize + key_box[(key_box[j] as usize + j) & 0xFF] as usize) & 0xFF;
            chunk[i - 1] ^= key_box[idx];
        }
        out_file.write_all(&chunk[..n])?;
    }

    Ok(out_path)
}

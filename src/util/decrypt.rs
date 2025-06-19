use aes::Aes256;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use hex::{decode as hex_decode, encode as hex_encode};
use rand::{rngs::OsRng, RngCore};
use std::env;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub fn decrypt(hex_data: &str) -> Result<String, Box<dyn std::error::Error>> {
    let key = env::var("ENCRYPTION_KEY")?;
    let key_bytes = key.as_bytes();
    let data = hex_decode(hex_data)?;
    let (iv, ciphertext) = data.split_at(16);

    let cipher = Aes256Cbc::new_from_slices(key_bytes, iv)?;
    let decrypted_data = cipher.decrypt_vec(ciphertext)?;

    let data = String::from_utf8(decrypted_data)?;

    return Ok(data);
}

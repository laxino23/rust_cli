use anyhow::Ok;
use base64::{
    Engine as _,
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
};

use crate::{
    cli::base64::Base64Format,
    utils::{read_input, write_output},
};

pub fn process_encode(
    input: &str,
    format: Base64Format,
) -> anyhow::Result<String> {
    let buf = read_input(input, true)?;
    let encoder = get_encoder(format);
    let encode = encoder(&buf);
    Ok(encode)
}

pub fn process_decode(
    input: &str,
    format: Base64Format,
) -> anyhow::Result<Vec<u8>> {
    let buf = read_input(input, true)?;
    let buf_str = String::from_utf8_lossy(&buf).trim().to_string();
    let decoder = get_decoder(format);
    let decode = decoder(&buf_str)?;
    // NOTE the result may not be valid UTF-8 string
    write_output(&decode)?;
    Ok(decode)
}

fn get_decoder(
    format: Base64Format,
) -> impl Fn(&str) -> anyhow::Result<Vec<u8>> {
    move |s: &str| {
        let value = match format {
            Base64Format::Standard => STANDARD.decode(s)?,
            Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(s)?,
        };
        Ok(value)
    }
}

fn get_encoder(format: Base64Format) -> impl Fn(&[u8]) -> String {
    move |bytes: &[u8]| match format {
        Base64Format::Standard => STANDARD.encode(bytes),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(bytes),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_file_decode() {
        let input = "fixtures/temp.b64";
        let format = Base64Format::Standard;
        assert!(process_decode(input, format).is_ok());
    }
    #[test]
    fn test_file_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        assert!(process_encode(input, format).is_ok());
    }

    #[test]
    fn test_encode_string() {
        let input = "Hello, world!";
        let format = Base64Format::Standard;
        let encoded = encode_str(input, format).unwrap();
        assert_eq!(encoded, "SGVsbG8sIHdvcmxkIQ==");
    }

    #[test]
    fn test_decode_string() {
        let input = "SGVsbG8sIHdvcmxkIQ==";
        let format = Base64Format::Standard;
        let decoded = decode_str(input, format).unwrap();
        assert_eq!(decoded, b"Hello, world!");
    }

    /// 测试用：直接编码字符串
    fn encode_str(input: &str, format: Base64Format) -> anyhow::Result<String> {
        let encoder = super::get_encoder(format);
        Ok(encoder(input.as_bytes()))
    }

    /// 测试用：直接解码字符串
    fn decode_str(
        input: &str,
        format: Base64Format,
    ) -> anyhow::Result<Vec<u8>> {
        let decoder = super::get_decoder(format);
        decoder(input)
    }
}

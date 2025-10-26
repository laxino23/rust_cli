use std::{
    fs::File,
    io::{Read, Write, stdin},
};

use anyhow::{Ok, Result as aResult};

pub fn read_input(input: &str, trim: bool) -> aResult<Vec<u8>> {
    let mut reader: Box<dyn Read> = if input == "-" {
        Box::new(stdin())
    } else {
        Box::new(File::open(input)?)
    };
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    if trim {
        let trimmed = String::from_utf8(buf)
            .map_err(|e| anyhow::anyhow!("not valid UTF-8: {}", e))?
            .trim()
            .to_string();

        Ok(trimmed.as_bytes().to_vec())
    } else {
        Ok(buf)
    }
}

pub fn write_output(output: &[u8]) -> aResult<()> {
    let stdout = std::io::stdout();
    let mut handle = stdout.lock();
    handle.write_all(output)?;
    Ok(())
}

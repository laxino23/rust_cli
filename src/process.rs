use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")] // first letter uppercase
pub struct Player {
    pub name: String,
    pub position: String,
    #[serde(rename = "DOB")]
    pub dob: String,
    pub nationality: String,
    #[serde(rename = "Kit Number")]
    pub kit: u8,
}

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut container = Vec::with_capacity(128);

    for result in reader.deserialize::<Player>() {
        let record = result?;
        container.push(record);
    }

    let json = serde_json::to_string_pretty(&container)?;
    fs::write(output, json)?;
    Ok(())
}

use crate::cli::csv::OutputFormat;
use csv::Reader;
use serde_json::Value;
use std::fs;

pub fn process_csv(input: &str, output: &str, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut container = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();

    for result in reader.records() {
        let record = result?;
        let json_value = serde_json::Value::Object(
            headers
                .iter()
                .zip(record.iter())
                .map(|(h, v)| (h.to_string(), serde_json::Value::String(v.to_string())))
                .collect(),
        );
        container.push(json_value);
    }
    let content = match format.into() {
        "json" => serde_json::to_string_pretty(&container)?,
        "yaml" => serde_yaml::to_string(&container)?,
        "toml" => {
            let toml_values: Vec<toml::Value> = container.iter().map(json_to_toml).collect();
            // TOML 不支持顶层是数组 → 包一层 table
            let mut root = toml::map::Map::new();
            root.insert("data".to_string(), toml::Value::Array(toml_values));
            toml::to_string(&root)?
        }
        _ => unreachable!("Unsupported format"), // This should never happen due to prior validation
    };

    // let json = serde_json::to_string_pretty(&container)?;
    fs::write(output, content)?;
    Ok(())
}

fn json_to_toml(json_str: &Value) -> toml::Value {
    match json_str {
        Value::Null => toml::Value::String("null".to_string()),
        Value::Bool(b) => toml::Value::Boolean(*b),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                toml::Value::Integer(i)
            } else if let Some(f) = n.as_f64() {
                toml::Value::Float(f)
            } else {
                toml::Value::String(n.to_string())
            }
        }
        Value::String(s) => toml::Value::String(s.clone()),
        Value::Array(arr) => {
            let toml_array: Vec<toml::Value> = arr.iter().map(json_to_toml).collect();
            toml::Value::Array(toml_array)
        }
        Value::Object(obj) => {
            let toml_table: toml::value::Table = obj
                .iter()
                .map(|(k, v)| (k.clone(), json_to_toml(v)))
                .collect();
            toml::Value::Table(toml_table)
        }
    }
}

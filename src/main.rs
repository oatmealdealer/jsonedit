use anyhow::{Context, Result};
use clap::Parser;
use serde_json::Value;
use serde_json_path::JsonPath;
use std::{fs::File, path::PathBuf};

#[derive(Debug, Parser)]
struct Args {
    /// Path to JSON file.
    file: PathBuf,
    /// A valid JSONPath query according to RFC 9535.
    query: JsonPath,
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Clone, Parser)]
enum Command {
    /// Set a property on exactly one object returned by the query.
    Set {
        /// The property to set on the object.
        key: String,
        /// A valid JSON string.
        value: String,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();
    let mut json_value: serde_json::Value = serde_json::from_reader(&File::open(&args.file)?)?;
    let result = args.query.query_located(&json_value);
    match args.command {
        Command::Set { key, value } => {
            let parsed_value: Value = serde_json::from_str(value.as_str())?;
            // https://github.com/hiltontj/serde_json_path/issues/66
            let pointer = result.exactly_one()?.location().to_json_pointer();
            if let Some(val) = json_value.pointer_mut(&pointer) {
                val.as_object_mut()
                    .context(format!(
                        "{} returned something other than Object",
                        &args.query
                    ))?
                    .insert(key, parsed_value);
            }
        }
    }

    let mut tmpfile = tempfile::NamedTempFile::new_in(
        args.file
            .canonicalize()?
            .parent()
            .context("file must have parent dir")?,
    )?;
    serde_json::to_writer_pretty(&mut tmpfile, &json_value)?;
    tmpfile.persist(&args.file)?;

    Ok(())
}

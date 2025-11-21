pub mod info;
pub mod stats;
pub mod list;
pub mod inspect;
pub mod search;
pub mod extract;
pub mod docs;

use crate::types::{DocpackGraph, PackageMetadata, Documentation};
use anyhow::{Context, Result};
use std::io::Read;
use std::path::Path;

pub fn load_docpack(path: impl AsRef<Path>) -> Result<(DocpackGraph, PackageMetadata, Option<Documentation>)> {
    let file = std::fs::File::open(path.as_ref())
        .context("Failed to open .docpack file")?;

    let mut archive = zip::ZipArchive::new(file)
        .context("Failed to read .docpack as zip archive")?;

    let mut graph_json = String::new();
    archive.by_name("graph.json")
        .context("graph.json not found in .docpack")?
        .read_to_string(&mut graph_json)?;

    let graph: DocpackGraph = serde_json::from_str(&graph_json)
        .context("Failed to parse graph.json")?;

    let mut metadata_json = String::new();
    archive.by_name("metadata.json")
        .context("metadata.json not found in .docpack")?
        .read_to_string(&mut metadata_json)?;

    let metadata: PackageMetadata = serde_json::from_str(&metadata_json)
        .context("Failed to parse metadata.json")?;

    let documentation = if let Ok(mut doc_file) = archive.by_name("documentation.json") {
        let mut doc_json = String::new();
        doc_file.read_to_string(&mut doc_json)?;
        match serde_json::from_str(&doc_json) {
            Ok(doc) => Some(doc),
            Err(e) => {
                eprintln!("Warning: Failed to parse documentation.json: {}", e);
                None
            }
        }
    } else {
        None
    };

    Ok((graph, metadata, documentation))
}

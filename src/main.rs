mod document;
mod language;
mod embeddings;
mod index;
mod project;
mod ast;

use document::Document;
use index::{IndexConfig, VectorIndex};
use project::Project;

#[tokio::main]
async fn main() {
    let project = Project::from_dir("/home/synoet/dev/endex".to_string());
}

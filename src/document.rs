use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub text: String,
    pub id: String,
    pub nodes: Option<Vec<DocumentNode>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentNode {
    pub text: String,
    pub id: String,
}

impl Document {
    pub fn new(text: String, id: String, chunk_size: usize) -> Self {
        Self {
            text,
            id,
            nodes: None,
        }
    }

    pub fn chunk(&mut self, size: usize) {
        self.nodes = Some(Vec::new());
        for (index, chunk) in self
            .text
            .split_whitespace()
            .collect::<Vec<&str>>()
            .chunks(size)
            .enumerate()
        {
            if let Some(nodes) = &mut self.nodes {
                nodes.push(DocumentNode {
                    text: chunk.join(" ").to_string(),
                    id: format!("{}", index),
                });
            }
        }
    }

    pub fn from_file(path: &str) -> Self {
        let text = std::fs::read_to_string(path).unwrap();
        let id = path.to_string();
        Self {
            text,
            id,
            nodes: None,
        }
    }
}

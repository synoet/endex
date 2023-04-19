use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub text: String,
    pub id: String,
    pub node_id: usize,
}

impl Document {
    pub fn new(text: String, id: String, node_id: Option<usize>) -> Self {
        Self {
            text,
            id,
            node_id: node_id.unwrap_or(0),
        }
    }

    pub fn chunk(&mut self, size: usize) -> Vec<Self> {
        let mut chunks: Vec<Self> = Vec::new();

        for (index, chunk) in self
            .text
            .split_whitespace()
            .collect::<Vec<&str>>()
            .chunks(size)
            .enumerate()
        {
            chunks.push(Document::new(chunk.join(" "), self.id.clone(), Some(index)));
        }

        chunks
    }

    pub fn from_file(path: &str) -> Self {
        let text = std::fs::read_to_string(path).unwrap();
        let id = path.to_string();
        Self {
            text,
            id,
            node_id: 0,
        }
    }
}

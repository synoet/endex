use tree_sitter::Language as TreeSitterLanguage;
use tree_sitter_cpp::language as language_cpp;
use tree_sitter_go::language as language_go;
use tree_sitter_python::language as language_python;
use tree_sitter_rust::language as language_rust;
use tree_sitter_typescript::{language_typescript, language_tsx};
use tree_sitter_javascript::language as language_javascript;
use tree_sitter_json::language as language_json;
use tree_sitter_toml::language as language_toml;
use tree_sitter_md::language as language_markdown;

pub enum Language {
    TypeScript,
    TSX,
    Python,
    Rust,
    Go,
    CPP,
    JavaScript,
    JSON,
    TOML,
    Markdown,
}

impl Language {
    pub fn tree_sitter_language(&self) -> TreeSitterLanguage {
        match self {
            Language::TypeScript => language_typescript(),
            Language::TSX => language_tsx(),
            Language::Python => language_python(),
            Language::Rust => language_rust(),
            Language::Go => language_go(),
            Language::CPP => language_cpp(),
            Language::JavaScript => language_javascript(),
            Language::JSON => language_json(),
            Language::TOML => language_toml(),
            Language::Markdown => language_markdown(),
        }
    }

    pub fn from_extension(extension: &str) -> Self {
        match extension {
            "ts" => Language::TypeScript,
            "tsx" => Language::TSX,
            "py" => Language::Python,
            "rs" => Language::Rust,
            "go" => Language::Go,
            "cpp" => Language::CPP,
            "js" => Language::JavaScript,
            "json" => Language::JSON,
            "toml" => Language::TOML,
            "md" => Language::Markdown,
            _ => panic!("Unsupported file extension: {}", extension),
        }
    }
}

use crate::project::BaseFile;
use tree_sitter::{Parser, LanguageError, Node, Tree};
use crate::language::Language;


pub struct EndexAST {
    tree: Tree,
    pub file: BaseFile,
}

impl EndexAST {

    pub fn from_file(file: BaseFile) -> Result<EndexAST, LanguageError> {
        let mut parser = Parser::new();
        let language = Language::from_extension(&file.extension);
        parser.set_language(language.tree_sitter_language())?;

        let tree = parser.parse(&file.content, None).unwrap();

        Ok(EndexAST {
            file: file.clone(),
            tree,
        })

    }

    pub fn walk(&mut self) -> Vec<Node> {
        let mut tree_cursor = self.tree.walk();
        let mut nodes: Vec<Node> = vec![];

        while tree_cursor.goto_next_sibling() {
            let node = tree_cursor.node();
            nodes.push(node);
        }

        nodes
    }

    pub fn get_node_content(&self, node: &Node) -> String {
        let start_byte = node.start_byte();
        let end_byte = node.end_byte();
        let content = &self.file.content[start_byte..end_byte];
        content.to_string()
    }
}

use std::fs;
use std::ffi::OsStr;
use regex::Regex;
use walkdir::WalkDir;
use crate::ast::EndexAST;

pub struct Project {
    name: String,
    path: String,
    branches: Vec<Branch>,
}

struct Branch {
    id: String,
    name: String,
    index_id: String,
    files: Vec<ProjectFile>,
}

struct ProjectFile {
    path: String,
    content: String,
    last_indexed: Option<usize>,
    document_id: String,
}

impl Project {
    pub fn from_dir(path: String) -> Project {
        let name = path.split("/").last().unwrap().to_string();
        let files = get_files(&path).unwrap();

        for file in files {
            let ast = EndexAST::from_file(file);

        }

        let branches = Vec::new();

        Project {
            name,
            path,
            branches,
        }

    }
}



#[derive(Clone)]
pub struct BaseFile {
    pub path: String,
    pub name: String,
    pub extension: String,
    pub content: String,
}

pub fn get_files(path: &str) -> Result<Vec<BaseFile>, std::io::Error> {

    let git_ignore = fs::read_to_string(format!("{}/.gitignore", path)).unwrap_or(String::new());
    let endex_ignore =
        fs::read_to_string(format!("{}/.endexignore", path)).unwrap_or(String::new());
    let default_ignores: Vec<String> = vec![
        ".git".to_string(),
    ];

    let mut ignore_patterns: Vec<String> = Vec::new();
    ignore_patterns.extend(git_ignore.lines().map(|s| s.to_string()));
    ignore_patterns.extend(endex_ignore.lines().map(|s| s.to_string()));
    ignore_patterns.extend(default_ignores);

    let mut files = Vec::new();

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {

            let entry_path = entry.path().to_string_lossy().to_string();
            let should_ignore = ignore_patterns.iter().any(|pattern| {
                let re = Regex::new(pattern).unwrap();
                re.is_match(&entry_path)
            });

            if !should_ignore {
                let content = fs::read_to_string(entry.path())?;
                let file = BaseFile {
                    path: entry.path().to_string_lossy().to_string(),
                    name: entry.file_name().to_string_lossy().to_string(),
                    extension: entry.path().extension().unwrap_or(OsStr::new(" ")).to_string_lossy().to_string(),
                    content,
                };
                files.push(file);
            }
        }
    }

    Ok(files)
}

use glob::glob;
use serde::{Deserialize, Serialize};
use serde_json::Error;
use std::default::Default;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CellOutput {
    name: Option<String>,
    output_type: Option<String>,
    text: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct JupyterCellMetaData {
    source_hidden: Option<bool>,
    outputs_hidden: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CellMetaData {
    jupyter: Option<JupyterCellMetaData>,
    // TODO execution
    collapsed: Option<bool>,
    // TODO scrolled
    name: Option<String>,
    tags: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CodeCell {
    pub id: Option<String>,
    pub metadata: CellMetaData,
    pub execution_count: Option<i32>,
    pub outputs: Vec<CellOutput>,
    // Source as array of lines
    pub source: Vec<String>,
    pub idx: Option<usize>,
}

// default
impl Default for CodeCell {
    fn default() -> Self {
        CodeCell {
            id: None,
            metadata: CellMetaData {
                jupyter: None,
                collapsed: None,
                name: None,
                tags: None,
            },
            execution_count: Some(1),
            outputs: vec![],
            source: vec![],
            idx: Some(0),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MarkdownCell {
    pub id: Option<String>,
    pub metadata: CellMetaData,
    pub source: Vec<String>,
    pub idx: Option<usize>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "cell_type", rename_all = "snake_case")]
pub enum Cell {
    Code(CodeCell),
    Markdown(MarkdownCell),
}

fn extract_code_cells(cells: &[Cell]) -> Vec<&CodeCell> {
    cells
        .iter()
        .filter_map(|cell| match cell {
            Cell::Code(c) => Some(c),
            _ => None,
        })
        .collect()
}

fn extract_markdown_cells(cells: &[Cell]) -> Vec<&MarkdownCell> {
    cells
        .iter()
        .filter_map(|cell| match cell {
            Cell::Markdown(c) => Some(c),
            _ => None,
        })
        .collect()
}

#[derive(Serialize, Deserialize, Debug)]
struct KernelSpec {
    display_name: String,
    language: Option<String>,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct CodeMirrorMode {
    name: String,
    version: i8,
}

#[derive(Serialize, Deserialize, Debug)]
struct LanguageInfo {
    name: String,
    codemirror_mode: Option<CodeMirrorMode>,
    file_extension: Option<String>,
    mimetype: Option<String>,
    nbconvert_exporter: Option<String>,
    pygments_lexer: Option<String>,
    version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct VsCodeInterpreter {
    hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct VsCode {
    interpreter: VsCodeInterpreter,
}

#[derive(Serialize, Deserialize, Debug)]
struct Author {
    name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct NotebookMeta {
    kernelspec: Option<KernelSpec>,
    language_info: Option<LanguageInfo>,
    orig_nbformat: Option<i8>,
    title: Option<String>,
    vscode: Option<VsCode>,
    authors: Option<Vec<Author>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Notebook {
    pub filename: Option<PathBuf>,
    pub cells: Vec<Cell>,
    pub nbformat: i32,
    pub nbformat_minor: i32,
    pub metadata: NotebookMeta,
}

impl Notebook {
    //Build and empty notebook with this filename
    pub fn new(filename: &str) -> Notebook {
        let mut notebook = Notebook {
            filename: Some(PathBuf::from(filename)),
            cells: Vec::new(),
            nbformat: 0,
            nbformat_minor: 0,
            metadata: NotebookMeta::default(),
        };
        notebook.add_cell_indices();
        notebook
    }

    pub fn rglob(root: &Path) -> Option<Vec<Notebook>> {
        if root.is_dir() {
            let root_str = root.to_str()?;
            let glob_str = format!("{}/**/*.ipynb", root_str);
            let files = glob(&glob_str).unwrap();
            let result: Vec<Notebook> = files
                .map(|p| Notebook::from_file(p.unwrap()).unwrap())
                .collect();
            Some(result)
        } else if root.extension().unwrap_or_else(|| "".as_ref()) == ".ipynb" {
            return Some(vec![Notebook::from_file(root.to_path_buf()).unwrap()]);
        } else {
            return Some(vec![]);
        }
    }

    pub fn from_file(path: PathBuf) -> Result<Notebook, Error> {
        let contents = fs::read_to_string(path.clone()).expect("Error reading file");
        let mut notebook: Notebook = serde_json::from_str(&contents)?;
        notebook.filename = Some(path);
        Ok(notebook)
    }

    pub fn filename_str(&self) -> &str {
        match &self.filename {
            Some(f) => f.to_str().unwrap_or("???"),
            None => "???",
        }
    }

    pub fn add_cell_indices(&mut self) {
        for (i, cell) in self.cells.iter_mut().enumerate() {
            match cell {
                Cell::Code(c) => c.idx = Some(i),
                Cell::Markdown(c) => c.idx = Some(i),
            }
        }
    }

    pub fn code_cells(&self) -> Vec<&CodeCell> {
        extract_code_cells(&self.cells)
    }
    pub fn markdown_cells(&self) -> Vec<&MarkdownCell> {
        extract_markdown_cells(&self.cells)
    }
}

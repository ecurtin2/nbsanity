use crate::notebook::{Cell, Notebook};
use enum_dispatch::enum_dispatch;
use serde::{ser::Error, Deserialize, Serialize};

#[enum_dispatch]
trait CheckTrait {
    fn check(&self, notebook: &Notebook) -> AnalysisResult;
}

#[enum_dispatch(CheckTrait)]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde()]
pub enum Check {
    FileNotNamedUntitled,
    CellExecutionIsSequential,
    NoEmptyCells,
}

impl Check {
    pub fn from_str(s: &str) -> Self {
        match s {
            "FileNotNamedUntitled" => Check::FileNotNamedUntitled(FileNotNamedUntitled {}),
            "CellExecutionIsSequential" => {
                Check::CellExecutionIsSequential(CellExecutionIsSequential {})
            }
            "NoEmptyCells" => Check::NoEmptyCells(NoEmptyCells {}),
            _ => panic!("Unknown check: {}", s),
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Check::FileNotNamedUntitled(_) => "FileNotNamedUntitled",
            Check::CellExecutionIsSequential(_) => "CellExecutionIsSequential",
            Check::NoEmptyCells(_) => "NoEmptyCells",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct FileNotNamedUntitled;

impl CheckTrait for FileNotNamedUntitled {
    fn check(&self, notebook: &Notebook) -> AnalysisResult {
        let mut result: AnalysisResult = AnalysisResult {
            pass: true,
            check: Check::FileNotNamedUntitled(self.clone()),
            failures: vec![],
        };
        if notebook.filename_str().to_lowercase().contains("untitled") {
            result.pass = false;
            result.failures.push(ResultFailure {
                cell_id: 0,
                description: "Notebook filename contains 'Untitled'".to_string(),
            })
        }
        result
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct CellExecutionIsSequential;
impl CheckTrait for CellExecutionIsSequential {
    fn check(&self, notebook: &Notebook) -> AnalysisResult {
        let mut previous: i32 = 0;
        let mut result: AnalysisResult = AnalysisResult {
            pass: true,
            check: Check::CellExecutionIsSequential(self.clone()),
            failures: vec![],
        };
        for cell in notebook.code_cells().iter() {
            match cell.execution_count {
                Some(count) => {
                    if count != previous + 1 {
                        result.pass = false;
                        result.failures.push(ResultFailure {
                            cell_id: cell.idx.unwrap_or(std::usize::MAX),
                            description: format!("Not executed in order, got {}", count),
                        });
                    }
                }
                None => {
                    result.pass = false;
                    result.failures.push(ResultFailure {
                        cell_id: cell.idx.unwrap_or(std::usize::MAX),
                        description: format!("Cell was not run"),
                    })
                }
            }
            previous += 1;
        }
        return result;
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NoEmptyCells;
impl CheckTrait for NoEmptyCells {
    fn check(&self, notebook: &Notebook) -> AnalysisResult {
        let mut result: AnalysisResult = AnalysisResult {
            pass: true,
            check: Check::NoEmptyCells(self.clone()),
            failures: vec![],
        };
        let empty_cell_idxs: Vec<usize> = notebook
            .cells
            .clone()
            .into_iter()
            .filter_map(|cell| match cell {
                Cell::Code(c) => {
                    if c.source.is_empty() || c.source.iter().all(|s| s.trim().is_empty()) {
                        c.idx
                    } else {
                        None
                    }
                }
                Cell::Markdown(c) => {
                    if c.source.is_empty() || c.source.iter().all(|s| s.trim().is_empty()) {
                        c.idx
                    } else {
                        None
                    }
                }
            })
            .collect();

        result.pass = empty_cell_idxs.is_empty();
        for i in empty_cell_idxs {
            result.failures.push(ResultFailure {
                cell_id: i,
                description: "Cell is empty".to_string(),
            })
        }
        return result;
    }
}

#[derive(Debug)]
struct ResultFailure {
    cell_id: usize,
    description: String,
}

#[derive(Debug)]
pub struct AnalysisResult {
    pass: bool,
    check: Check,
    failures: Vec<ResultFailure>,
}

pub fn analyze(notebook: &Notebook, exclude: &Vec<Check>) -> Vec<AnalysisResult> {
    let r = vec![
        Check::CellExecutionIsSequential(CellExecutionIsSequential {}).check(notebook),
        Check::FileNotNamedUntitled(FileNotNamedUntitled {}).check(notebook),
        Check::NoEmptyCells(NoEmptyCells {}).check(notebook),
    ];
    r
    // Remove analysisresult if it's in the exclude list
    // r.into_iter()
    //     .filter(|r| !exclude.contains(&r.check))
    //     .collect()
}

pub fn any_failed(results: &Vec<AnalysisResult>) -> bool {
    results.iter().any(|r| !r.pass)
}

pub fn display_errors(results: &Vec<AnalysisResult>, notebook: &Notebook) {
    for r in results.iter() {
        if !r.pass {
            for failure in &r.failures {
                println!(
                    "{} <Cell: {}> {} [{}]",
                    notebook.filename_str(),
                    failure.cell_id,
                    failure.description,
                    r.check.to_str()
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::notebook::CodeCell;

    #[test]
    fn not_untitled_error_if_untitled() {
        let notebook = Notebook::new("Untitled.ipynb".into());
        let result = Check::FileNotNamedUntitled(FileNotNamedUntitled {}).check(&notebook);
        assert_eq!(result.pass, false);
        assert_eq!(result.failures.len(), 1);
        assert_eq!(
            result.failures[0].description,
            "Notebook filename contains 'Untitled'"
        );
    }

    // test that untitled check does nothing if the filename is not untitled
    #[test]
    fn not_untitled_pass_if_not_untitled() {
        let notebook = Notebook::new("something else".into());
        let result = Check::FileNotNamedUntitled(FileNotNamedUntitled {}).check(&notebook);
        assert_eq!(result.pass, true);
        assert_eq!(result.failures.len(), 0);
    }

    // test analyze
    #[test]
    fn analyze_returns_all_results() {
        let notebook = Notebook::new("Untitled.ipynb".into());
        let results = analyze(&notebook, &vec![]);
        assert_eq!(results.len(), 3);
    }

    // test any failed
    #[test]
    fn any_failed_returns_true_if_any_failed() {
        let notebook = Notebook::new("Untitled.ipynb".into());
        let results = analyze(&notebook, &vec![]);
        assert_eq!(any_failed(&results), true);
    }

    // test any failed on untitled is false if excluded
    // #[test]
    // fn any_failed_returns_false_if_untitled_is_excluded() {
    //     let notebook = Notebook::new("Untitled.ipynb".into());
    //     let results = analyze(&notebook, &vec![Check::FileNotNamedUntitled]);
    //     assert_eq!(any_failed(&results), false);
    // }

    // test check empty cells true if any cell is empty
    #[test]
    fn check_empty_cells_returns_true_if_any_cell_is_empty() {
        let mut notebook = Notebook::new("test.ipynb".into());
        notebook.cells.push(Cell::Code(CodeCell::default()));
        let results = analyze(&notebook, &vec![]);
        assert_eq!(any_failed(&results), true);
    }

    #[test]
    fn check_string_roundtrip() {
        let check = Check::FileNotNamedUntitled(FileNotNamedUntitled);
        assert_eq!(Check::from_str(check.to_str()), check);
    }

    #[test]
    fn check_from_string() {
        let check = Check::from_str("FileNotNamedUntitled");
        assert_eq!(check, Check::FileNotNamedUntitled(FileNotNamedUntitled));
    }

    // // test check empty cells false if has cells that aren't empty
    #[test]
    fn check_empty_cells_returns_false_if_all_cells_are_not_empty() {
        let mut notebook = Notebook::new("test.ipynb".into());
        let mut cell = CodeCell::default();
        cell.source = vec!["print('hello')".into()];
        notebook.cells = vec![Cell::Code(cell)];
        let results = analyze(&notebook, &vec![]);
        assert_eq!(any_failed(&results), false);
    }
}

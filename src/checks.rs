use crate::notebook::{Cell, Notebook};
use enum_dispatch::enum_dispatch;
use std::string::ToString;
extern crate strsim;
use strsim::levenshtein;
use std::str::FromStr;
use strum_macros::{EnumString, Display};
use std::default::Default;
use enum_iterator::{all, Sequence};


#[enum_dispatch]
trait CheckTrait {
    fn check(&self, notebook: &Notebook) -> AnalysisResult;
}

#[enum_dispatch(CheckTrait)]
#[derive(Debug, PartialEq, Clone, EnumString, Display, Sequence)]
pub enum Check {
    FileNotNamedUntitled,
    CellExecutionIsSequential,
    NoEmptyCells,
    HasTitleCell,
}

impl Check {
    pub fn all() -> Vec<Check> {
        all::<Check>().collect::<Vec<_>>()
}
}

pub fn find_closest(s: String) -> Check {
    let checks = Check::all();
    let closest = checks.iter()
        .map(|c| (levenshtein(&c.to_string(), &s), c))
        .min_by(|l, r| l.0.cmp(&r.0))
        .unwrap();
    closest.1.clone()
}

#[derive(Debug, PartialEq, Clone, Default, Sequence)]
pub struct FileNotNamedUntitled;

impl CheckTrait for FileNotNamedUntitled {
    fn check(&self, notebook: &Notebook) -> AnalysisResult {
        let mut result = AnalysisResult::new(Check::FileNotNamedUntitled(self.clone()));
        if notebook.filename_str().to_lowercase().contains("untitled") {
            result.add_failure(0, "Notebook filename contains 'Untitled'".to_string())
        }
        result
    }
}

#[derive(Debug, PartialEq, Clone, Default, Sequence)]
pub struct CellExecutionIsSequential;
impl CheckTrait for CellExecutionIsSequential {
    fn check(&self, notebook: &Notebook) -> AnalysisResult {
        let mut result = AnalysisResult::new(Check::CellExecutionIsSequential(self.clone()));
        for (previous, cell) in (0_i32..).zip(notebook.code_cells().iter()) {
            match cell.execution_count {
                Some(count) => {
                    if count != previous + 1 {
                        result.add_failure(
                            cell.idx.unwrap_or(std::usize::MAX),
                            format!("Not executed in order, got {}", count),
                        )
                    }
                }
                None => result.add_failure(
                    cell.idx.unwrap_or(std::usize::MAX),
                    "Cell was not run".to_string(),
                ),
            }
        }
        result
    }
}

#[derive(Debug, PartialEq, Clone, Default, Sequence)]
pub struct NoEmptyCells;
impl CheckTrait for NoEmptyCells {
    fn check(&self, notebook: &Notebook) -> AnalysisResult {
        let mut result = AnalysisResult::new(Check::NoEmptyCells(self.clone()));
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

        for i in empty_cell_idxs {
            result.add_failure(i, "Cell is empty".to_string())
        }
        result
    }
}

#[derive(Debug, PartialEq, Clone, Default, Sequence)]
pub struct HasTitleCell;
impl CheckTrait for HasTitleCell {
    fn check(&self, notebook: &Notebook) -> AnalysisResult {
        let mut result = AnalysisResult::new(Check::HasTitleCell(self.clone()));
        let mut pass = false;
        if let Some(Cell::Markdown(first)) = notebook.cells.first() {
            if !first.source.is_empty() && first.source[0].starts_with("'#'") {
                pass = true;
            }
        };

        if !pass {
            result.add_failure(0, "Notebook does not have a title cell".to_string())
        }

        result
    }
}

#[derive(Debug)]
struct ResultFailure {
    cell_id: usize,
    description: String,
}

#[derive(Debug)]
pub struct AnalysisResult {
    check: Check,
    failures: Vec<ResultFailure>,
}

impl AnalysisResult {
    fn fail(&self) -> bool {
        !self.failures.is_empty()
    }

    fn pass(&self) -> bool {
        self.failures.is_empty()
    }

    fn add_failure(&mut self, cell_id: usize, description: String) {
        self.failures.push(ResultFailure {
            cell_id,
            description,
        })
    }

    fn new(check: Check) -> Self {
        AnalysisResult {
            check,
            failures: vec![],
        }
    }
}

pub fn analyze(notebook: &Notebook, exclude: &[Check]) -> Vec<AnalysisResult> {
    Check::all().iter()
        .filter(|c| !exclude.contains(c))
        .map(|c| c.check(notebook))
        .collect()
}

pub fn any_failed(results: &[AnalysisResult]) -> bool {
    results.iter().any(|r| !r.pass())
}

pub fn display_errors(results: &[AnalysisResult], notebook: &Notebook) {
    for r in results.iter() {
        if !r.pass() {
            for failure in &r.failures {
                println!(
                    "{} <Cell: {}> {} [{}]",
                    notebook.filename_str(),
                    failure.cell_id,
                    failure.description,
                    r.check
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::notebook::CodeCell;

    // test check find closest
    #[test]
    fn test_find_closest() {
        let found = find_closest("CellExecutionIsSequentialX".to_string());
        assert_eq!(
            found,
            Check::CellExecutionIsSequential(CellExecutionIsSequential {})
        );
    }

    #[test]
    fn not_untitled_error_if_untitled() {
        let notebook = Notebook::new("Untitled.ipynb".into());
        let result = Check::FileNotNamedUntitled(FileNotNamedUntitled {}).check(&notebook);
        assert!(result.fail());
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
        assert!(result.pass());
    }

    // test analyze
    #[test]
    fn analyze_returns_all_results() {
        let notebook = Notebook::new("Untitled.ipynb".into());
        let results = analyze(&notebook, &vec![]);
        assert_eq!(results.len(), Check::all().len());
    }

    // test any failed
    #[test]
    fn any_failed_returns_true_if_any_failed() {
        let notebook = Notebook::new("Untitled.ipynb".into());
        let results = analyze(&notebook, &vec![]);
        assert!(any_failed(&results));
    }
    // test check empty cells true if any cell is empty
    #[test]
    fn check_empty_cells_fail_if_any_cell_is_empty() {
        let mut notebook = Notebook::new("test.ipynb".into());
        notebook.cells.push(Cell::Code(CodeCell::default()));
        let got = Check::NoEmptyCells(NoEmptyCells {}).check(&notebook);
        assert!(got.fail());
    }

    #[test]
    fn check_string_roundtrip() {
        for check in Check::all().iter() {
            assert_eq!(Check::from_str(&check.to_string()).unwrap(), *check);
        }
    }

    #[test]
    fn check_from_str_strum() {
        assert_eq!(Check::from_str("FileNotNamedUntitled").unwrap(), Check::FileNotNamedUntitled(FileNotNamedUntitled))
    }

    #[test]
    fn check_display_strum() {
        let got = format!("{}", Check::FileNotNamedUntitled(FileNotNamedUntitled));
        assert_eq!(got, "FileNotNamedUntitled")
    }

    #[test]
    fn check_to_str_strum() {
        let got = Check::FileNotNamedUntitled(FileNotNamedUntitled).to_string();
        assert_eq!(got, "FileNotNamedUntitled")
    }

    #[test]
    fn check_from_string() {
        let check = Check::from_str("FileNotNamedUntitled");
        assert_eq!(
            check.unwrap(),
            Check::FileNotNamedUntitled(FileNotNamedUntitled)
        );
    }

    // #[test]
    // fn check_from_string_bad_input() {
    //     let check = Check::from_str("NotArealCheck");
    //     assert!(format!("{}", check.unwrap_err()).contains("Unknown check"));
    // }

    // // test check empty cells false if has cells that aren't empty
    #[test]
    fn check_empty_cells_pass_if_all_cells_are_not_empty() {
        let mut notebook = Notebook::new("test.ipynb".into());
        let mut cell = CodeCell::default();
        cell.source = vec!["print('hello')".into()];
        notebook.cells = vec![Cell::Code(cell)];
        let got = Check::NoEmptyCells(NoEmptyCells {}).check(&notebook);
        assert!(got.pass());
    }
}

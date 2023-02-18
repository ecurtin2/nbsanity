use crate::notebook::{Cell, CodeCell, Notebook};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum Check {
    CellExecutionIsSequential,
    NoEmptyCells,
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

fn is_execution_count_sorted(cells: &Vec<&CodeCell>) -> AnalysisResult {
    let mut previous: i32 = 0;
    let mut result: AnalysisResult = AnalysisResult {
        pass: true,
        check: Check::CellExecutionIsSequential,
        failures: vec![],
    };
    for cell in cells.iter() {
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

fn check_empty_cells(cells: &Vec<Cell>) -> AnalysisResult {
    let mut result: AnalysisResult = AnalysisResult {
        pass: true,
        check: Check::NoEmptyCells,
        failures: vec![],
    };
    let empty_cell_idxs: Vec<usize> = cells
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

pub fn analyze(notebook: &Notebook, exclude: &Vec<Check>) -> Vec<AnalysisResult> {
    vec![
        is_execution_count_sorted(&notebook.code_cells()),
        check_empty_cells(&notebook.cells),
    ]
}

pub fn any_failed(results: &Vec<AnalysisResult>) -> bool {
    results.iter().any(|r| !r.pass)
}

pub fn display_errors(results: &Vec<AnalysisResult>, notebook: &Notebook) {
    for r in results.iter() {
        if !r.pass {
            for failure in &r.failures {
                println!(
                    "{} <Cell: {}> {} [{:?}]",
                    notebook.filename_str(), failure.cell_id, failure.description, r.check
                )
            }
        }
    }
}

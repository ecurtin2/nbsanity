mod checks;
mod config;
mod notebook;
use anyhow::Result;
use checks::{analyze, any_failed, display_errors, find_closest, Check};
use config::Config;
use notebook::Notebook;
use serde_json::Error;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "nbsanity",
    about = "The blazingly fast linter for Jupyter notebooks"
)]
struct CliOpts {
    /// Don't output to stdout if successfull
    #[structopt(short, long)]
    quiet: bool,
}

fn main() -> Result<(), Error> {
    let opts = CliOpts::from_args();
    let conf = Config::build();
    let mut notebooks = Notebook::rglob(conf.root_path()).unwrap();
    let mut global_any_failed = false;
    let (disabled, errors): (Vec<_>, Vec<_>) = conf
        .disable
        .unwrap_or_default()
        .iter()
        .map(|s| Check::from_str(s))
        .partition(Result::is_ok);

    let disabled: Vec<_> = disabled.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    if !errors.is_empty() {
        for e in errors {
            // TODO: this is using the "Unkown check:" in the
            // error message to find the closest match. This is
            // a bit hacky, but it works for now.
            let error_str = format!("{}", e);
            let closest = find_closest(error_str);
            println!("{}, did you mean {} ?", e, closest.to_str());
        }
        std::process::exit(1);
    }

    for notebook in notebooks.iter_mut() {
        notebook.add_cell_indices();
        let analysis = analyze(notebook, &disabled);
        let failed = any_failed(&analysis);
        if failed {
            global_any_failed = true;
            display_errors(&analysis, notebook);
        } else if !opts.quiet {
            println!("{} \u{2705}", notebook.filename_str());
        }
    }

    match global_any_failed {
        false => std::process::exit(1),
        true => std::process::exit(0),
    }
}

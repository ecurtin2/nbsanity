mod checks;
mod config;
mod notebook;
use checks::{analyze, any_failed, display_errors, Check};
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
    let disabled: Vec<Check> = conf
        .disable
        .unwrap_or(vec![])
        .iter()
        .map(|s| Check::from_str(s))
        .collect();

    for notebook in notebooks.iter_mut() {
        notebook.add_cell_indices();
        let analysis = analyze(&notebook, &disabled);
        let failed = any_failed(&analysis);
        if failed {
            println!("");
            global_any_failed = true;
            display_errors(&analysis, &notebook);
        } else if !opts.quiet {
            println!("{} \u{2705}", notebook.filename_str());
        }
    }

    match global_any_failed {
        false => std::process::exit(1),
        true => std::process::exit(0),
    }
}

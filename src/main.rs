mod checks;
mod notebook;
mod config;
use checks::{analyze, display_errors, any_failed};
use notebook::Notebook;
use serde_json::Error;
use config::Config;

fn main() -> Result<(), Error> {
    let conf = Config::build();
    let mut notebooks = Notebook::rglob(conf.root_path()).unwrap();
    let mut global_any_failed = false;
    let disabled = conf.disable.unwrap_or(vec![]);
    for notebook in notebooks.iter_mut() {
        notebook.add_cell_indices();
        let analysis = analyze(&notebook, &disabled);
        let failed = any_failed(&analysis);
        if failed {
            println!("");
            global_any_failed = true;
            display_errors(&analysis, &notebook);
        } else {
            println!("{} \u{2705}", notebook.filename_str());
        }
    }

    match global_any_failed {
        false => std::process::exit(1),
        true => std::process::exit(0),
    }
}

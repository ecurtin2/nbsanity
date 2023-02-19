# nbsanity

Nbsanity is a Jupyter Notebook linter inspired by [ruff](https://github.com/charliermarsh/ruff). It is designed to help users maintain high-quality Jupyter Notebooks. The tool is implemented in Rust, which makes it lightweight and fast. The linter currently supports several options to help ensure that notebooks are well-organized, well-documented, and error-free.

Jupyter Notebooks are popular tools for data analysis, machine learning, and scientific computing. They allow users to combine code, visualizations, and narrative in a single document. However, as notebooks grow in complexity, it becomes increasingly difficult to ensure that they are well-organized, well-documented, and error-free.

Nbsanity is designed to help solve these problems by providing a fast and lightweight tool for checking the quality of notebooks. By using the linter, you can quickly identify common issues in notebooks. 



## Install

### Run from Prebuilt binary

Prebuilt binary is available for `x86_64-unknown-linux-gnu` architecture.
Download from the [releases page](https://github.com/ecurtin2/nbsanity/releases)

```
mkdir -p ~/bin
wget https://github.com/ecurtin2/nbsanity/releases/download/0.1.5/nbsanity -o ~/bin/nbsanity
chmod +x ~/bin/nbsanity
```

### Install from cargo

Cargo is rust's package manager: https://doc.rust-lang.org/cargo/
This might work on other operating systems but I haven't tried.

```
cargo install nbsanity
```

## Run

Run from the root of your project (requires pyproject.toml)
```
nbsanity
```

By default, the linter will check all notebooks in the current directory and its subdirectories. TODO: You can specify a single notebook or a directory to check by providing the path as an argument to the command.

## Configuration


| Option                    | Description                                                                                     |
|---------------------------|-------------------------------------------------------------------------------------------------|
| FileNotNamedUntitled      | Check if the notebook file has been named.                                                     |
| CellExecutionIsSequential | Check if the notebook cells have been executed in sequential order.                             |
| NoEmptyCells              | Check if the notebook contains empty cells.                                                    |
| HasTitleCell              | Check if the notebook has a title cell (i.e., a cell with a Markdown header).                   |



### Configure via pyproject.toml
```
[tool.nbsanity]
disable = ["CellExecutionIsSequential"]
```

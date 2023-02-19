# nbsanity

`nbsanity` is a linter for jupyter notebooks inspired by [ruff](https://github.com/charliermarsh/ruff).
I wrote it as a side project to learn rust, so you probably don't want to rely on this for anything serious.



## Install

### Run from Prebuilt binary

Prebuilt binary is available for `x86_64-unknown-linux-gnu` architecture.
Download from the [releases page](https://github.com/ecurtin2/nbsanity/releases)

```
mkdir -p ~/bin
wget https://github.com/ecurtin2/nbsanity/releases/download/0.1.4/nbsanity -o ~/bin/nbsanity
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

## Configuration


|       Feature                |     Description                                                      |
|------------------------------|----------------------------------------------------------------------|
| CellExecutionIsSequential    | Each cell in a notebook is executed in sequence, one after another.  |
|     NoEmptyCells             | There are no empty cells in a notebook. |


### Configure via pyproject.toml
```
[tool.nbsanity]
disable = ["CellExecutionIsSequential"]
```

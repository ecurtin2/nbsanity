# nbsanity
jupyter notebook linter


## Install

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


### pyproject.toml
```
[tool.nbsanity]
disable = ["CellExecutionIsSequential"]
```

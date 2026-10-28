# xny-cli

A way to use [Learn X in Y Minutes](https://learnxinyminutes.com/) on the command line.

I look up languages often on this page, and I thought it would be nice to make it similar to `man` pages in linux, so I wrote a small script.

![usage gif](https://github.com/YesSeri/xny-cli/blob/main/docs/media/xny.gif)

## Installation

Install with command: `cargo install xny`


If that is not possible you can download a binary from releases and put in path.


### Install from source

```bash
git clone git@github.com:YesSeri/xny-cli.git

cargo build --release

# put file in path.
```
    
## Usage

```bash
# print the guide to stdout
xny python

# use bat as viewer
xny python --viewer bat

# list all languages
xny -s

# refresh the local language catalogue from the upstream repository
xny --update-catalog

# refresh the catalogue, then display a guide
xny python --update-catalog
```

`--viewer bat` would show the output in [bat](https://github.com/sharkdp/bat), the `cat` clone which syntax highlights. It is of course possible to pipe the content into bat, but then you need to set the syntax highlighting manually.

The catalogue is created from the English guide metadata in the upstream [Learn X in Y Minutes docs repository](https://github.com/adambard/learnxinyminutes-docs) the first time `xny` runs. It is stored in the platform's standard cache directory under `xny`, alongside downloaded guides. Listing languages and opening already downloaded guides works offline. Run `xny --update-catalog` to refresh the catalogue. `xny -c` clears the catalogue and downloaded guides. `--viewer` is available when opening a guide, and cannot be combined with `--show-languages` or `--clear-cache`; `--show-languages` cannot be combined with a language argument. `--update-catalog` may be combined with either a language or `--show-languages`.

## Credit

Big thank you to the creator of [Learn X in Y](https://github.com/adambard/learnxinyminutes-docs)!

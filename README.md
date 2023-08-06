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
# default viewer is less
xny python

# use bat as viewer
xny python --viewer bat

# list all languages
xny -s
```

`--viewer bat` would show the output in [bat](https://github.com/sharkdp/bat), the `cat` clone which syntax highlights. It is of course possible to pipe the content into bat, but then you need to set the syntax highlighting manually.

## Credit

Big thank you to the creator of [Learn X in Y](https://github.com/adambard/learnxinyminutes-docs)!

# xiny-cli

A way to use [Learn X in Y Minutes](https://learnxinyminutes.com/) on the command line.

I look up languages often on this page, and I thought it would be nice to make it similar to `man` pages in linux, so I wrote a small script.



## Installation

```bash
git clone https://github.com/YesSeri/x-in-y-cli
cargo build --release  
# put file on path.
```
    
## Usage/Examples

```bash
x-in-y --language python
# default viewer is less
x-in-y --language python --viewer bat
```
`--viewer bat` would show the output in [bat](https://github.com/sharkdp/bat), the `cat` clone which syntax highlights.


## Credit

Big thank you to the creator of [Learn X in Y](https://github.com/adambard/learnxinyminutes-docs)!

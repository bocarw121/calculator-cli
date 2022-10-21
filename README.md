# Calcul8 - Calculator CLI
[![Actions Status](https://github.com/bocarw121/calculator-cli/workflows/Build%20and%20Test/badge.svg)](https://github.com//bocarw121/calculator-cli/actions)
## Prerequisites
  - Install [rustup](https://rustup.rs/)

## Installation
  - From the terminal, run `cargo install calcul8`


## Usage
```sh
# Addition operation - you can substitute the `add` command with -a or --addition
$ calcul8 add 1 2 3 4 5 
15

# Subtraction operation - you can substitute the `sub` command with -s or --subtraction
$ calcul8 sub 8 2 1
5

# Multiplication operation - you can substitute the `mul` command with -m or --multiplication
$ calcul8 mul 20 7 4
560

# Division operation - you can substitute the `div` command with -d or --division
$ calcul8 div 8 4
2

```

### All commands take a precision option which is the number of decimal places for the result. The default is 2.
```sh
$ calcul8 -p 3  add  30.12 40.2121 10.12312
80.455
$ calcul8 -p 4  add  30.12 40.2121 10.12312
80.4552
```

To see all the commands and options, run `calcul8 --help`

## Contributing
  - Fork the repo
  - Create a branch for your feature
  - Run `cargo run` to build app and run it
  - Run `cargo run -- --help` to see all the commands and options
  - Make your changes
  - Add tests for your changes
  - Run `cargo test` to ensure all tests pass
  - Run `cargo fmt` to ensure your code is formatted correctly
  - Create a pull request








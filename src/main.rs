pub mod cli;
pub mod operations;
pub mod utils;

use clap::Parser;
use cli::{Cli, Operators};
use operations::{add, divide, multiply, subtract};
use utils::{check_for_lone_zero_decimal, print_result};

fn main() {
    let args = Cli::parse();

    match args.command {
        Operators::Add { addition } => {
            let result = check_for_lone_zero_decimal(add(addition), args.precision);
            print_result(result)
        }
        Operators::Sub { subtraction } => {
            let result = check_for_lone_zero_decimal(subtract(subtraction), args.precision);
            print_result(result)
        }
        Operators::Mul { multiplication } => {
            let result = check_for_lone_zero_decimal(multiply(multiplication), args.precision);
            print_result(result)
        }
        Operators::Div { division } => {
            let result = check_for_lone_zero_decimal(divide(division), args.precision);
            print_result(result)
        }
    }
}

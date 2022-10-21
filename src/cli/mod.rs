use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Calcul8")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Operators,
    #[arg(short, long, default_value_t = 2)]
    pub precision: usize,
}

#[derive(Subcommand)]
pub enum Operators {
    /// Addition operation
    #[command(short_flag = 'a', long_flag = "addition")]
    Add { addition: Vec<f32> },
    /// Subtraction operation
    #[command(short_flag = 's', long_flag = "subtraction")]
    Sub { subtraction: Vec<f32> },
    /// Multiplication operation
    #[command(short_flag = 'm', long_flag = "multiplication")]
    Mul { multiplication: Vec<f32> },
    /// Division operation
    #[command(short_flag = 'd', long_flag = "division")]
    Div { division: Vec<f32> },
}

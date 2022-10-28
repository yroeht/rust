extern crate clap;
extern crate rand;

use clap::{
    Args,
    Parser,
    Subcommand
};

mod multiply;
mod gen;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct MyArgs {
    #[clap(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Multiply two matrices of dimensions (m, k) and (k, n), print the result
    Multiply(MultiplyArgs),
    /// Generate a matrix of dimensions (m, n) , print the result
    Generate(GenArgs),
}

#[derive(Args)]
pub struct MultiplyArgs {
    matrix1 :String,
    matrix2 :String,
}

#[derive(Args)]
pub struct GenArgs {
    m :i32,
    n :i32,
}

fn main() {
    let args = MyArgs::parse();
    match args.commands {
        Commands::Multiply(MultiplyArgs{matrix1, matrix2})
            => multiply::multiply(matrix1, matrix2),
        Commands::Generate(GenArgs{m, n})
            => gen::gen(m, n),
    }
}


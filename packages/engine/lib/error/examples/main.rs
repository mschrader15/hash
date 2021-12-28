use std::{error::Error, fmt};

use error::{
    self, bail, ensure, format_err,
    provider::{Provider, Requisition},
    tags::ErrorMessage,
    Report, Result, WrapReport,
};

#[derive(Debug)]
pub struct ExampleError;

impl fmt::Display for ExampleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ExampleError")
    }
}

impl Error for ExampleError {}

#[derive(Debug)]
pub struct ExampleWrappingError;

impl fmt::Display for ExampleWrappingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ExampleWrappingError")
    }
}

impl Provider for ExampleWrappingError {
    fn provide<'a>(&'a self, mut req: Requisition<'a, '_>) {
        req.provide_value::<u64, _>(|| 10)
            .provide_value::<u64, _>(|| 11)
            .provide_value::<u64, _>(|| 12)
            .provide_with::<ErrorMessage, _>(|| self.to_string().into_boxed_str());
    }
}

fn main() -> Result<()> {
    let err = four().unwrap_err();
    ensure!(2 == 3, "Assertion failed");
    println!("Display:");
    println!("{err}");
    println!("\nDisplay alternate:");
    println!("{err:#}");
    println!("\nDebug:");
    println!("{err:?}");
    println!("\nDebug alternate:");
    println!("{err:#?}");
    Ok(())
}

fn one() -> Result<()> {
    Err(Report::from_error(ExampleError).context("one"))
}

fn two() -> Result<()> {
    one().wrap_err("two")
}

fn three() -> Result<()> {
    two().provide(ExampleWrappingError)
}

fn four() -> Result<()> {
    three().wrap_err_with(|| format!("This is #{} from line {}", 4, line!()))
}

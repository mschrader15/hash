use std::{error::Error, fmt};

use error::{
    self,
    provider::{Provider, Requisition},
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
            .provide_value::<u64, _>(|| 12);
    }
}

fn main() {
    let e = four().unwrap_err();
    report(&e);
}

fn one() -> Result<()> {
    Err(Report::from_error(ExampleError).context("one"))
}

fn two() -> Result<()> {
    one().context("two")
}

fn three() -> Result<()> {
    two().provide(ExampleWrappingError)
}

fn four() -> Result<()> {
    three().context_with(|| format!("This is #{}", 4))
}

pub fn report(report: &Report) {
    println!("\nReturn trace:");
    for (idx, frame) in report.chain().enumerate() {
        println!("   {idx}: {frame}");
        println!("             at {}", frame.location());
    }

    println!("\nBacktrace:");
    println!("{}", report.backtrace());

    for value in report.request_value::<u64>() {
        println!("values: {value}");
    }
}

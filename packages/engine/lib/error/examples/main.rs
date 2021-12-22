#![feature(backtrace)]

use core::{fmt, panic::Location};
use std::backtrace::Backtrace;

use error::{error::Error, provider::Request};

#[derive(Debug)]
pub struct ExampleError {
    frames: Vec<&'static Location<'static>>,
    backtrace: Option<Backtrace>,
}

impl Default for ExampleError {
    #[track_caller]
    fn default() -> Self {
        Self {
            frames: vec![Location::caller()],
            backtrace: Some(Backtrace::force_capture()),
            // backtrace: None,
        }
    }
}

impl fmt::Display for ExampleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ExampleError")
    }
}

impl Error for ExampleError {
    fn provide_context<'a>(&'a self, request: &mut Request<'a>) {
        request
            .provide_ref::<[&'static Location<'static>]>(&self.frames)
            .provide_ref::<Option<Backtrace>>(&self.backtrace);
    }
}

#[derive(Debug)]
pub struct ExampleWrappingError {
    frames: Vec<&'static Location<'static>>,
    source: ExampleError,
    backtrace: Option<Backtrace>,
}

impl From<ExampleError> for ExampleWrappingError {
    fn from(source: ExampleError) -> Self {
        Self {
            frames: vec![Location::caller()],
            backtrace: Some(Backtrace::force_capture()),
            source,
        }
    }
}

impl fmt::Display for ExampleWrappingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ExampleWrappingError")
    }
}

impl Error for ExampleWrappingError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }

    fn provide_context<'a>(&'a self, request: &mut Request<'a>) {
        request
            .provide_ref::<[&'static Location<'static>]>(&self.frames)
            .provide_ref::<Option<Backtrace>>(&self.backtrace);
    }
}

fn main() {
    let e = four().unwrap_err();
    report(&e);
}

fn one() -> Result<(), ExampleError> {
    Err(ExampleError::default())
}

fn two() -> Result<(), ExampleError> {
    Ok(one()?)
}

fn three() -> Result<(), ExampleWrappingError> {
    Ok(two()?)
}

fn four() -> Result<(), ExampleWrappingError> {
    Ok(three()?)
}

pub fn report(error: &(dyn Error + 'static)) {
    let locations = error
        .chain()
        .filter_map(|e| e.context_ref::<[&'static Location<'static>]>())
        .flatten();

    println!("\nFull Return Trace:");
    for (i, loc) in locations.enumerate() {
        println!("    {}: {}", i, loc);
    }

    let backtraces = error
        .chain()
        .filter_map(|e| e.context_ref::<Option<Backtrace>>())
        .flatten()
        .last();

    if let Some(backtrace) = backtraces {
        println!("\nBacktrace:");
        println!("{backtrace}");
    }
}

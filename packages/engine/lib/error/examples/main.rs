#![feature(backtrace)]

use core::{fmt, panic::Location};
use std::{
    backtrace::{Backtrace, BacktraceStatus},
    thread,
};

use error::{
    provider::{tags, Requisition, TypeTag},
    Error,
};

#[derive(Debug)]
pub struct ExampleError {
    frames: Vec<&'static Location<'static>>,
    backtrace: Option<Backtrace>,
}

struct LocationTag;

impl TypeTag<'_> for LocationTag {
    type Type = Vec<&'static Location<'static>>;
}

impl Default for ExampleError {
    #[track_caller]
    fn default() -> Self {
        Self {
            frames: vec![Location::caller()],
            backtrace: Some(Backtrace::capture()),
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
    fn provide<'ctx>(&'ctx self, mut req: Requisition<'ctx, '_>) {
        req.provide_ref(&*self.frames);
        req.provide_value(Backtrace::capture);
        if let Some(ref backtrace) = self.backtrace {
            req.provide_ref(backtrace);
        }
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
            backtrace: Some(Backtrace::capture()),
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
    fn provide<'a>(&'a self, mut req: Requisition<'a, '_>) {
        req.provide_value::<u64, _>(|| 10)
            .provide_value::<u64, _>(|| 11)
            .provide_value::<u64, _>(|| 12)
            .provide_ref::<dyn Error>(&self.source)
            .provide_ref::<[&'static Location<'static>]>(&self.frames);
        if let Some(backtrace) = self.backtrace.as_ref() {
            req.provide_ref(backtrace);
        }
    }
}

fn main() {
    let t = thread::spawn(move || four().unwrap_err());
    let e = t.join().unwrap();
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
    println!("\nReturn Trace:");
    for (idx, location) in error
        .chain()
        .filter_map(|err| err.request_ref::<[&'static Location<'static>]>())
        .flatten()
        .enumerate()
    {
        println!("   {idx}:");
        println!("             at {location}");
    }

    // use core::any::Any;
    // dbg!(error.is::<ExampleError>());

    if let Some(backtrace) = error.chain().find_map(|err| err.request_ref::<Backtrace>()) {
        println!("\nBacktrace:");
        println!("{backtrace}");
    }

    if let Some(backtrace) = error
        .chain()
        .find_map(|err| err.request_value::<Backtrace>())
    {
        println!("\nBacktrace:");
        println!("{backtrace}");
    }

    for value in error.chain().filter_map(|err| err.request_value::<u64>()) {
        println!("values: {value}");
    }
}

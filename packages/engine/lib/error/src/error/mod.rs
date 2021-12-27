mod frame;
mod iter;
mod report;
mod tags;
mod wrap;

use alloc::boxed::Box;
use core::{fmt, panic::Location};

pub use self::iter::{Chain, Request};
use self::report::{ErrorType, ReportImpl};
use crate::provider::Provider;

pub struct Report {
    // TODO: Use thin pointer + vtable to reduce size of `Report`
    inner: Box<ReportImpl>,
}

pub struct Frame {
    error: ErrorType,
    location: &'static Location<'static>,
    source: Option<Box<Frame>>,
}

pub type Result<T, E = Report> = core::result::Result<T, E>;

pub trait WrapReport<T> {
    fn context<C>(self, context: C) -> Result<T>
    where
        C: fmt::Display + Send + Sync + 'static;

    fn context_with<C, F>(self, context: F) -> Result<T>
    where
        C: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C;

    fn provide<P>(self, provider: P) -> Result<T>
    where
        P: Provider + Send + Sync + 'static;

    fn provide_with<P, F>(self, provider: F) -> Result<T>
    where
        P: Provider + Send + Sync + 'static,
        F: FnOnce() -> P;
}

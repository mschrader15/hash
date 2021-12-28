mod frame;
mod iter;
mod macros;
mod report;
pub mod tags;
mod wrap;

use alloc::boxed::Box;
use core::{fmt, panic::Location};

use self::{frame::ErrorType, report::ReportImpl};
pub use self::{
    iter::{Chain, Request},
    macros::*,
};
use crate::provider::Provider;

#[must_use]
pub struct Report {
    inner: Box<ReportImpl>,
}

pub struct Frame {
    error: ErrorType,
    location: &'static Location<'static>,
    source: Option<Box<Frame>>,
}

pub type Result<T, E = Report> = core::result::Result<T, E>;

pub trait WrapReport<T> {
    fn wrap_err<C>(self, context: C) -> Result<T>
    where
        C: fmt::Display + fmt::Debug + Send + Sync + 'static;

    fn wrap_err_with<C, F>(self, context: F) -> Result<T>
    where
        C: fmt::Display + fmt::Debug + Send + Sync + 'static,
        F: FnOnce() -> C;

    fn provide<P>(self, provider: P) -> Result<T>
    where
        P: Provider + fmt::Display + fmt::Debug + Send + Sync + 'static;

    fn provide_with<P, F>(self, provider: F) -> Result<T>
    where
        P: Provider + fmt::Display + fmt::Debug + Send + Sync + 'static,
        F: FnOnce() -> P;
}

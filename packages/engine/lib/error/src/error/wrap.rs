use core::fmt;

use crate::{provider::Provider, Report, Result, WrapReport};

impl<T> WrapReport<T> for Result<T, Report> {
    #[track_caller]
    fn context<C>(self, context: C) -> Self
    where
        C: fmt::Display + Send + Sync + 'static,
    {
        match self {
            Ok(t) => Ok(t),
            Err(report) => Err(report.context(context)),
        }
    }

    #[track_caller]
    fn context_with<C, F>(self, context: F) -> Self
    where
        C: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        match self {
            Ok(t) => Ok(t),
            Err(report) => Err(report.context(context())),
        }
    }

    #[track_caller]
    fn provide<P>(self, provider: P) -> Self
    where
        P: Provider + Send + Sync + 'static,
    {
        match self {
            Ok(t) => Ok(t),
            Err(report) => Err(report.provide(provider)),
        }
    }

    #[track_caller]
    fn provide_with<P, F>(self, provider: F) -> Self
    where
        P: Provider + Send + Sync + 'static,
        F: FnOnce() -> P,
    {
        match self {
            Ok(t) => Ok(t),
            Err(report) => Err(report.provide(provider())),
        }
    }
}

impl<T> WrapReport<T> for Option<T> {
    #[track_caller]
    fn context<C>(self, context: C) -> Result<T>
    where
        C: fmt::Display + Send + Sync + 'static,
    {
        match self {
            Some(t) => Ok(t),
            None => Err(Report::from_message(context)),
        }
    }

    #[track_caller]
    fn context_with<C, F>(self, context: F) -> Result<T>
    where
        C: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        match self {
            Some(t) => Ok(t),
            None => Err(Report::from_message(context())),
        }
    }

    #[track_caller]
    fn provide<P>(self, provider: P) -> Result<T>
    where
        P: Provider + Send + Sync + 'static,
    {
        match self {
            Some(t) => Ok(t),
            None => Err(Report::from_provider(provider)),
        }
    }

    #[track_caller]
    fn provide_with<P, F>(self, provider: F) -> Result<T>
    where
        P: Provider + Send + Sync + 'static,
        F: FnOnce() -> P,
    {
        match self {
            Some(t) => Ok(t),
            None => Err(Report::from_provider(provider())),
        }
    }
}

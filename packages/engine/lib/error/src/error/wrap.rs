use core::fmt;

use crate::{provider::Provider, Report, Result, WrapReport};

#[cfg(feature = "std")]
impl<T, E> WrapReport<T> for Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    #[track_caller]
    fn wrap_err<C>(self, context: C) -> Result<T>
    where
        C: fmt::Display + fmt::Debug + Send + Sync + 'static,
    {
        match self {
            Ok(t) => Ok(t),
            Err(error) => Err(Report::from_error(error).context(context)),
        }
    }

    #[track_caller]
    fn wrap_err_with<C, F>(self, context: F) -> Result<T>
    where
        C: fmt::Display + fmt::Debug + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        match self {
            Ok(t) => Ok(t),
            Err(error) => Err(Report::from_error(error).context(context())),
        }
    }

    #[track_caller]
    fn provide<P>(self, provider: P) -> Result<T>
    where
        P: Provider + fmt::Display + fmt::Debug + Send + Sync + 'static,
    {
        match self {
            Ok(t) => Ok(t),
            Err(error) => Err(Report::from_error(error).provide(provider)),
        }
    }

    #[track_caller]
    fn provide_with<P, F>(self, provider: F) -> Result<T>
    where
        P: Provider + fmt::Display + fmt::Debug + Send + Sync + 'static,
        F: FnOnce() -> P,
    {
        match self {
            Ok(t) => Ok(t),
            Err(error) => Err(Report::from_error(error).provide(provider())),
        }
    }
}

impl<T> WrapReport<T> for Result<T, Report> {
    #[track_caller]
    fn wrap_err<C>(self, context: C) -> Self
    where
        C: fmt::Display + fmt::Debug + Send + Sync + 'static,
    {
        match self {
            Ok(t) => Ok(t),
            Err(report) => Err(report.context(context)),
        }
    }

    #[track_caller]
    fn wrap_err_with<C, F>(self, context: F) -> Self
    where
        C: fmt::Display + fmt::Debug + Send + Sync + 'static,
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
        P: Provider + fmt::Display + fmt::Debug + Send + Sync + 'static,
    {
        match self {
            Ok(t) => Ok(t),
            Err(report) => Err(report.provide(provider)),
        }
    }

    #[track_caller]
    fn provide_with<P, F>(self, provider: F) -> Self
    where
        P: Provider + fmt::Display + fmt::Debug + Send + Sync + 'static,
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
    fn wrap_err<C>(self, context: C) -> Result<T>
    where
        C: fmt::Display + fmt::Debug + Send + Sync + 'static,
    {
        match self {
            Some(t) => Ok(t),
            None => Err(Report::from_message(context)),
        }
    }

    #[track_caller]
    fn wrap_err_with<C, F>(self, context: F) -> Result<T>
    where
        C: fmt::Display + fmt::Debug + Send + Sync + 'static,
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
        P: Provider + fmt::Display + fmt::Debug + Send + Sync + 'static,
    {
        match self {
            Some(t) => Ok(t),
            None => Err(Report::from_provider(provider)),
        }
    }

    #[track_caller]
    fn provide_with<P, F>(self, provider: F) -> Result<T>
    where
        P: Provider + fmt::Display + fmt::Debug + Send + Sync + 'static,
        F: FnOnce() -> P,
    {
        match self {
            Some(t) => Ok(t),
            None => Err(Report::from_provider(provider())),
        }
    }
}

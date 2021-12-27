use alloc::boxed::Box;
use core::{fmt, panic::Location};
#[cfg(feature = "backtrace")]
use std::backtrace::Backtrace;

use super::Frame;
use crate::{
    provider::{tags, Provider, TypeTag},
    Chain, Report, Request,
};

pub(super) struct ReportImpl {
    pub(super) error: Frame,
    #[cfg(feature = "backtrace")]
    backtrace: Backtrace,
}

pub(super) enum ErrorType {
    Message(Box<dyn fmt::Display + Send + Sync + 'static>),
    #[cfg(feature = "std")]
    Error(Box<dyn std::error::Error + Send + Sync + 'static>),
    Provider(Box<dyn Provider + Send + Sync + 'static>),
}

impl Report {
    #[track_caller]
    fn new(#[cfg(feature = "backtrace")] backtrace: Backtrace, error: ErrorType) -> Self {
        Self {
            inner: Box::new(ReportImpl {
                #[cfg(feature = "backtrace")]
                backtrace,
                error: Frame {
                    error,
                    location: Location::caller(),
                    source: None,
                },
            }),
        }
    }

    #[track_caller]
    pub fn from_message(message: impl fmt::Display + Send + Sync + 'static) -> Self {
        Self::new(
            #[cfg(feature = "backtrace")]
            Backtrace::capture(),
            ErrorType::Message(Box::new(message)),
        )
    }

    #[track_caller]
    #[cfg(feature = "std")]
    #[cfg_attr(doc, doc(cfg(feature = "std")))]
    pub fn from_error(error: impl std::error::Error + Send + Sync + 'static) -> Self {
        Self::new(
            #[cfg(feature = "backtrace")]
            Backtrace::capture(),
            ErrorType::Error(Box::new(error)),
        )
    }

    #[track_caller]
    pub fn from_provider(provider: impl Provider + Send + Sync + 'static) -> Self {
        Self::new(
            #[cfg(feature = "backtrace")]
            Backtrace::capture(),
            ErrorType::Provider(Box::new(provider)),
        )
    }

    #[track_caller]
    fn wrap(self, error: ErrorType) -> Self {
        Self {
            inner: Box::new(ReportImpl {
                #[cfg(feature = "backtrace")]
                backtrace: self.inner.backtrace,
                error: Frame {
                    error,
                    location: Location::caller(),
                    source: Some(Box::new(self.inner.error)),
                },
            }),
        }
    }

    #[track_caller]
    pub fn context<C>(self, context: C) -> Self
    where
        C: fmt::Display + Send + Sync + 'static,
    {
        self.wrap(ErrorType::Message(Box::new(context)))
    }

    #[track_caller]
    pub fn provide<P>(self, provider: P) -> Self
    where
        P: Provider + Send + Sync + 'static,
    {
        self.wrap(ErrorType::Provider(Box::new(provider)))
    }

    #[cfg(feature = "backtrace")]
    #[cfg_attr(doc, doc(cfg(feature = "backtrace")))]
    pub fn backtrace(&self) -> &Backtrace {
        &self.inner.backtrace
    }

    pub const fn chain(&self) -> Chain<'_> {
        Chain::new(self)
    }

    /// Request a reference to context of type `T`.
    pub fn request<'p, I: 'static>(&'p self) -> Request<'p, I>
    where
        I: TypeTag<'p>,
    {
        Request::new(self)
    }

    /// Request a reference to context of type `T`.
    pub fn request_ref<T: ?Sized + 'static>(&self) -> Request<'_, tags::Ref<T>> {
        Request::new(self)
    }

    /// Request a value of type `T`.
    pub fn request_value<T: 'static>(&self) -> Request<'_, tags::Value<T>> {
        Request::new(self)
    }
}

pub(super) use alloc::boxed::Box;
use core::{fmt, fmt::Formatter, panic::Location};
#[cfg(feature = "backtrace")]
use std::backtrace::{Backtrace, BacktraceStatus};

#[cfg(feature = "tracing-error")]
use tracing_error::{SpanTrace, SpanTraceStatus};

use super::{ErrorType, Frame};
use crate::{
    provider::{tags, Provider, TypeTag},
    Chain, Report, Request,
};

pub(super) struct ReportImpl {
    pub(super) error: Frame,
    #[cfg(feature = "backtrace")]
    backtrace: Backtrace,
    #[cfg(feature = "tracing-error")]
    span_trace: SpanTrace,
}

impl Report {
    #[track_caller]
    fn new(#[cfg(feature = "backtrace")] backtrace: Backtrace, error: ErrorType) -> Self {
        let report = Self {
            inner: Box::new(ReportImpl {
                #[cfg(feature = "backtrace")]
                backtrace,
                #[cfg(feature = "tracing-error")]
                span_trace: SpanTrace::capture(),
                error: Frame {
                    error,
                    location: Location::caller(),
                    source: None,
                },
            }),
        };
        report
    }

    #[track_caller]
    pub fn from_message(message: impl fmt::Display + fmt::Debug + Send + Sync + 'static) -> Self {
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
    pub fn from_provider(
        provider: impl Provider + fmt::Display + fmt::Debug + Send + Sync + 'static,
    ) -> Self {
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
                #[cfg(feature = "tracing-error")]
                span_trace: self.inner.span_trace,
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
        C: fmt::Display + fmt::Debug + Send + Sync + 'static,
    {
        self.wrap(ErrorType::Message(Box::new(context)))
    }

    #[track_caller]
    pub fn provide<P>(self, provider: P) -> Self
    where
        P: Provider + fmt::Display + fmt::Debug + Send + Sync + 'static,
    {
        self.wrap(ErrorType::Provider(Box::new(provider)))
    }

    #[cfg(feature = "backtrace")]
    #[cfg_attr(doc, doc(cfg(feature = "backtrace")))]
    pub fn backtrace(&self) -> &Backtrace {
        &self.inner.backtrace
    }

    #[cfg(feature = "tracing-error")]
    #[cfg_attr(doc, doc(cfg(feature = "spantrace")))]
    pub fn span_trace(&self) -> &SpanTrace {
        &self.inner.span_trace
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
    pub const fn request_ref<T: ?Sized + 'static>(&self) -> Request<'_, tags::Ref<T>> {
        Request::new(self)
    }

    /// Request a value of type `T`.
    pub const fn request_value<T: 'static>(&self) -> Request<'_, tags::Value<T>> {
        Request::new(self)
    }
}

impl fmt::Display for Report {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        let mut chain = self.chain();
        let error = chain.next().expect("No error occurred");
        fmt::Display::fmt(&error, fmt)?;
        if let Some(cause) = chain.next() {
            if fmt.alternate() {
                write!(fmt, ": {cause}")?;
            }
        }
        Ok(())
    }
}

impl fmt::Debug for Report {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        if fmt.alternate() {
            let mut debug = fmt.debug_struct("Report");
            debug.field("frames", &self.chain());
            #[cfg(feature = "backtrace")]
            debug.field("backtrace", self.backtrace());
            debug.finish()
        } else {
            let mut chain = self.chain();
            let error = chain.next().expect("No error occurred");
            write!(fmt, "{error}")?;
            write!(fmt, "\n             at {}", error.location())?;

            for (idx, frame) in chain.enumerate() {
                if idx == 0 {
                    fmt.write_str("\n\nCaused by:")?;
                }
                write!(fmt, "\n   {idx}: {frame}")?;
                write!(fmt, "\n             at {}", frame.location())?;
            }

            #[cfg(feature = "backtrace")]
            if self.backtrace().status() == BacktraceStatus::Captured {
                fmt.write_str("\n\nStack backtrace:\n")?;
                write!(fmt, "{}", self.backtrace())?;
            }

            #[cfg(feature = "tracing-error")]
            if self.span_trace().status() == SpanTraceStatus::CAPTURED {
                fmt.write_str("\n\nSpan trace:\n")?;
                write!(fmt, "{}", self.span_trace())?;
            }

            Ok(())
        }
    }
}

#[cfg(feature = "std")]
impl<E> From<E> for Report
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn from(error: E) -> Self {
        Self::from_error(error)
    }
}

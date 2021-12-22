use core::{
    any::{Any, TypeId},
    fmt::{Debug, Display},
};

use crate::{
    provider::{request, Request},
    tag::{Ref, Value},
};

/// `Error` is a trait representing the basic expectations for error values, i.e., values of type
/// `E` in [`Result<T, E>`].
///
/// Errors must describe themselves through the [`Display`] and [`Debug`], and must be [`Send`] and
/// [`Sync`] traits. Error messages are typically concise lowercase sentences without trailing
/// punctuation:
///
/// ```
/// let err = "NaN".parse::<u32>().unwrap_err();
/// assert_eq!(err.to_string(), "invalid digit found in string");
/// ```
///
/// Errors may provide cause chain information. [`Error::source()`] is generally used when errors
/// cross "abstraction boundaries". If one module must report an error that is caused by an error
/// from a lower-level module, it can allow accessing that error via [`Error::source()`]. This makes
/// it possible for the high-level module to provide its own errors while also revealing some of the
/// implementation for debugging via `source` chains.
pub trait Error: Debug + Display + Send + Sync {
    /// The lower-level source of this error, if any.
    ///
    /// # Examples
    ///
    /// ```
    /// use core::fmt;
    ///
    /// use error::Error;
    ///
    /// #[derive(Debug)]
    /// struct SuperError {
    ///     side: SuperErrorSideKick,
    /// }
    ///
    /// impl fmt::Display for SuperError {
    ///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         write!(f, "SuperError is here!")
    ///     }
    /// }
    ///
    /// impl Error for SuperError {
    ///     fn source(&self) -> Option<&(dyn Error + 'static)> {
    ///         Some(&self.side)
    ///     }
    /// }
    ///
    /// #[derive(Debug)]
    /// struct SuperErrorSideKick;
    ///
    /// impl fmt::Display for SuperErrorSideKick {
    ///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         write!(f, "SuperErrorSideKick is here!")
    ///     }
    /// }
    ///
    /// impl Error for SuperErrorSideKick {}
    ///
    /// fn get_super_error() -> Result<(), SuperError> {
    ///     Err(SuperError {
    ///         side: SuperErrorSideKick,
    ///     })
    /// }
    ///
    /// fn main() {
    ///     match get_super_error() {
    ///         Err(e) => {
    ///             println!("Error: {}", e);
    ///             println!("Caused by: {}", e.source().unwrap());
    ///         }
    ///         _ => println!("No error"),
    ///     }
    /// }
    /// ```
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    /// Provides type based access to context intended for error reports
    ///
    /// Used in conjunction with [`context`] and [`context_ref`] to extract
    /// references to member variables from `dyn Error` trait objects.
    ///
    /// # Example
    ///
    /// ```rust
    /// #![feature(backtrace)]
    /// use core::fmt;
    /// use std::backtrace::Backtrace;
    ///
    /// use error::{error::Error, provider::Request};
    ///
    /// #[derive(Debug)]
    /// struct MyError {
    ///     backtrace: Backtrace,
    /// }
    ///
    /// impl fmt::Display for MyError {
    ///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         write!(f, "Example Error")
    ///     }
    /// }
    ///
    /// impl Error for MyError {
    ///     fn provide_context<'tag>(&'tag self, mut request: &mut Request<'tag>) {
    ///         request.provide_ref::<Backtrace>(&self.backtrace);
    ///     }
    /// }
    ///
    /// let backtrace = Backtrace::capture();
    /// let error = MyError { backtrace };
    /// let dyn_error = &error as &dyn Error;
    /// let backtrace_ref = dyn_error.context_ref::<Backtrace>().unwrap();
    ///
    /// assert!(core::ptr::eq(&error.backtrace, backtrace_ref));
    /// ```
    fn provide_context<'a>(&'a self, request: &mut Request<'a>) {
        let _ = request;
    }
}

impl dyn Error {
    /// Returns `true` if the boxed type is the same as `T`
    #[inline]
    pub fn is<T: Error + 'static>(&self) -> bool {
        TypeId::of::<T>() == self.type_id()
    }

    /// Returns some reference to the boxed value if it is of type `T`, or
    /// `None` if it isn't.
    #[inline]
    pub fn downcast_ref<T: Error + 'static>(&self) -> Option<&T> {
        if self.is::<T>() {
            unsafe { Some(&*(self as *const dyn Error as *const T)) }
        } else {
            None
        }
    }

    /// Returns some mutable reference to the boxed value if it is of type `T`, or
    /// `None` if it isn't.
    #[inline]
    pub fn downcast_mut<T: Error + 'static>(&mut self) -> Option<&mut T> {
        if self.is::<T>() {
            unsafe { Some(&mut *(self as *mut dyn Error as *mut T)) }
        } else {
            None
        }
    }

    pub fn context_ref<T: ?Sized + 'static>(&self) -> Option<&T> {
        request::<Ref<T>, _>(|request| self.provide_context(request))
    }

    pub fn context<T: 'static>(&self) -> Option<T> {
        request::<Value<T>, _>(|request| self.provide_context(request))
    }

    pub fn chain(&self) -> Chain<'_> {
        Chain {
            current: Some(self),
        }
    }
}

pub struct Chain<'a> {
    current: Option<&'a (dyn Error + 'static)>,
}

impl<'a> Iterator for Chain<'a> {
    type Item = &'a (dyn Error + 'static);

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;
        self.current = self.current.and_then(Error::source);
        current
    }
}

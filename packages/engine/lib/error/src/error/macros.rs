#[doc(hidden)]
pub mod __private {
    use core::fmt;

    use crate::Report;

    pub mod kinds {
        use core::fmt;

        use crate::Report;

        pub trait AdhocKind: Sized {
            fn kind(&self) -> Adhoc {
                Adhoc
            }
        }
        impl<T> AdhocKind for &T where T: ?Sized + fmt::Display + fmt::Debug + Send + Sync + 'static {}

        pub struct Adhoc;
        impl Adhoc {
            pub fn report<M>(self, message: M) -> Report
            where
                M: fmt::Display + fmt::Debug + Send + Sync + 'static,
            {
                Report::from_message(message)
            }
        }

        pub trait TraitKind: Sized {
            fn kind(&self) -> Trait {
                Trait
            }
        }
        impl<E> TraitKind for E where E: Into<Report> {}

        pub struct Trait;
        impl Trait {
            pub fn report<E: Into<Report>>(self, error: E) -> Report {
                error.into()
            }
        }
    }

    pub fn format_err(args: fmt::Arguments) -> Report {
        Report::from_message(alloc::format!("{}", args))
    }
}

/// Creates a [`Report`] from the given parameters.
///
/// [`Report`]: crate::Report
///
/// # Example
///
/// ```
/// # fn has_permission(user: usize, resource: usize) -> bool { true }
/// # let user = 0;
/// # let resource = 0;
/// use error::format_err;
///
/// if !has_permission(user, resource) {
///     return Err(format_err!("permission denied for accessing {}", resource));
/// }
/// # error::Result::Ok(())
/// ```
#[macro_export]
macro_rules! format_err {
    ($msg:literal $(,)?) => ({
        $crate::Report::from_message($crate::__private::format_err(core::format_args!($msg)))
    });
    ($err:expr $(,)?) => ({
        use $crate::__private::kinds::*;
        let error = $err;
        (&error).kind().report(error)
    });
    ($fmt:expr, $($arg:tt)*) => {
        $crate::Report::from_message($crate::__private::format_err(core::format_args!($fmt, $($arg)*)))
    };
}

/// Ensures `$cond` is met, otherwise return an error.
///
/// # Example
///
/// ```
/// # fn has_permission(user: usize, resource: usize) -> bool { true }
/// # let user = 0;
/// # let resource = 0;
/// use error::ensure;
///
/// ensure!(
///     has_permission(user, resource),
///     "permission denied for accessing {resource}"
/// );
/// # error::Result::Ok(())
/// ```
#[macro_export]
macro_rules! ensure {
    ($cond:expr, $msg:literal $(,)?) => ({
        if !$cond {
            $crate::bail!($msg)
        }
    });
    ($cond:expr, $err:expr $(,)?) => ({
        if !$cond {
            $crate::bail!($err)
        }
    });
    ($cond:expr, $fmt:expr, $($arg:tt)*) => {
        if !$cond {
            $crate::bail!($fmt, $($arg)*)
        }
    };
}

/// Creates a [`Report`] and returns it as [`Result`].
///
/// This is the same as calling `return Err(format_err!(...))` with the same parameters.
///
/// [`Report`]: crate::Report
///
/// # Example
///
/// ```
/// # fn has_permission(user: usize, resource: usize) -> bool { true }
/// # let user = 0;
/// # let resource = 0;
/// use error::bail;
///
/// if !has_permission(user, resource) {
///     bail!("permission denied for accessing {}", resource);
/// }
/// # error::Result::Ok(())
/// ```
#[macro_export]
macro_rules! bail {
    ($msg:literal $(,)?) => ({
        return $crate::Result::<_>::Err($crate::format_err!($msg))
    });
    ($err:expr $(,)?) => ({
        return $crate::Result::<_>::Err($crate::format_err!($err))
    });
    ($fmt:expr, $($arg:tt)*) => {
        return $crate::Result::<_>::Err($crate::format_err!($fmt, $($arg)*))
    };
}

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "backtrace", feature(backtrace))]
#![cfg_attr(doc, feature(doc_cfg))]

extern crate alloc;

pub mod provider;

mod error;
mod internal;

pub use self::error::*;

#[cfg(not(feature = "std"))]
trait StdError: core::fmt::Debug + core::fmt::Display {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}

// struct ErrorImpl<E> {
//     kind: E,
//     message: String,
//     #[cfg(feature = "backtrace")]
//     backtrace: Backtrace,
//     sources: Vec<String>,
//     source: Option<Box<dyn StdError + Send + Sync + 'static>>,
// }
//
// pub struct Error<E> {
//     inner: Box<ErrorImpl<E>>,
// }

// impl<E> Error<E> {
//     pub fn chain(&self) -> vec::IntoIter<&(dyn StdError + 'static)> {
//         self.causes.into_iter()
//     }
//
//     #[cfg(feature = "std")]
//     #[cfg_attr(doc, doc(cfg(feature = "std")))]
//     fn from_error<Err: StdError + Send + Sync + 'static>(error: Err, kind: E) -> Self {
//         let message = error.to_string();
//         let source = error.source();
//         #[cfg(feature = "backtrace")]
//         let backtrace = error
//             .backtrace()
//             .map(|b| *b)
//             .unwrap_or_else(Backtrace::capture);
//         // .unwrap_or_else(|| Backtrace::capture());
//         Self {
//             inner: Box::new(ErrorImpl {
//                 kind,
//                 message,
//                 source: None,
//
//                 #[cfg(feature = "backtrace")]
//                 backtrace,
//                 sources: Vec::new(),
//             }),
//         }
//     }
//
//     pub fn kind(&self) -> &E {
//         &self.inner.kind
//     }
//
//     pub fn context(&self) -> &[String] {
//         &self.inner.sources
//     }
// }
//
// impl<E> fmt::Debug for Error<E> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Error: {}", self)?;
//         let context = self.context();
//         if !context.is_empty() {
//             writeln!(f, "\n\nCaused by:")?;
//             for (n, cause) in context.into_iter().enumerate() {
//                 write!(f, "   {n}: {cause}");
//                 if n != context.len() {
//                     writeln!(f)?;
//                 }
//             }
//         }
//         #[cfg(feature = "backtrace")]
//         if let Some(backtrace) = self.backtrace() {
//             writeln!(f, "\n\nStack backtrace:")?;
//             write!(f, "{backtrace}")?;
//         }
//         Ok(())
//     }
// }
//
// impl<E> fmt::Display for Error<E> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let context = self.context();
//         let message = &self.inner.message;
//         match (f.alternate(), context.first()) {
//             (true, Some(context)) => write!(f, "{message}: {context}"),
//             _ => write!(f, "{message}"),
//         }
//     }
// }
//
// #[cfg(feature = "std")]
// #[cfg_attr(doc, doc(cfg(feature = "std")))]
// impl<E> StdError for Error<E> {
//     // fn source(&self) -> Option<&(dyn StdError + 'static)> {
//     //     self.inner.source.map(|s| s.as_ref())
//     // }
//
//     #[cfg(feature = "backtrace")]
//     fn backtrace(&self) -> Option<&Backtrace> {
//         Some(&self.inner.backtrace)
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_size() {
//         let size = core::mem::size_of::<Error<u64>>();
//         if cfg!(feature = "alloc") {
//             assert_eq!(size, 16);
//         } else {
//             assert_eq!(size, 8);
//         }
//     }
// }

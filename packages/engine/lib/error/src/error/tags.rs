use alloc::boxed::Box;
use core::panic::Location;

use crate::{provider::TypeTag, Frame};

pub struct ErrorLocation;

impl<'p> TypeTag<'p> for ErrorLocation {
    type Type = &'static Location<'static>;
}

pub struct ErrorSource;

impl<'p> TypeTag<'p> for ErrorSource {
    type Type = &'p Frame;
}

pub struct ErrorMessage;

impl<'p> TypeTag<'p> for ErrorMessage {
    type Type = Box<str>;
}

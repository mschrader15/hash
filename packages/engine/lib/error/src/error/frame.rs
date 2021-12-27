use alloc::{boxed::Box, string::ToString};
use core::{fmt, panic::Location};

use super::{
    report::ErrorType,
    tags::{ErrorLocation, ErrorMessage, ErrorSource},
};
use crate::{
    provider::{self, tags, Provider, Requisition, TypeTag},
    Frame,
};

impl fmt::Display for Frame {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.error {
            ErrorType::Message(msg) => fmt::Display::fmt(&msg, fmt),
            #[cfg(feature = "std")]
            ErrorType::Error(err) => fmt::Display::fmt(&err, fmt),
            ErrorType::Provider(prov) => {
                if let Some(msg) =
                    provider::request_by_type_tag::<'_, ErrorMessage, _>(prov.as_ref())
                {
                    fmt::Display::fmt(&msg, fmt)
                } else {
                    write!(fmt, "<Unknown error>")
                }
            }
        }
    }
}

impl Provider for Frame {
    fn provide<'p>(&'p self, mut req: Requisition<'p, '_>) {
        req.provide_with::<ErrorLocation, _>(|| self.location);
        req.provide_with::<ErrorMessage, _>(|| self.to_string().into_boxed_str());
        if let Some(ref cause) = self.source {
            req.provide_with::<ErrorSource, _>(|| cause);
        }
        if let ErrorType::Provider(prov) = &self.error {
            prov.provide(req);
        }
    }
}

impl Frame {
    pub const fn location(&self) -> &'static Location<'static> {
        self.location
    }

    pub fn source(&self) -> Option<&Self> {
        self.source.as_ref().map(Box::as_ref)
    }

    pub fn request<'p, I>(&'p self) -> Option<I::Type>
    where
        I: TypeTag<'p>,
    {
        provider::request_by_type_tag::<'p, I, _>(self)
    }

    pub fn request_ref<T>(&self) -> Option<&T>
    where
        T: ?Sized + 'static,
    {
        self.request::<'_, tags::Ref<T>>()
    }

    pub fn request_value<T>(&self) -> Option<T>
    where
        T: 'static,
    {
        self.request::<'_, tags::Value<T>>()
    }
}

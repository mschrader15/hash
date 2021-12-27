//! This module contains the `Provider` trait and accompanying API, which enable trait objects to
//! provide data based on typed requests, an alternate form of runtime reflection.
//!
//! `Provider` and the associated APIs support generic, type-driven access to data, and a mechanism
//! for implementers to provide such data. The key parts of the interface are the `Provider` trait
//! for objects which can provide data, and the [`request_by_type_tag`] function for data from an
//! object which implements `Provider`. Note that end users should not call requesting
//! `request_by_type_tag` directly, it is a helper function for intermediate implementers to use to
//! implement a user-facing interface.
//!
//! Typically, a data provider is a trait object of a trait which extends `Provider`. A user will
//! request data from the trait object by specifying the type or a type tag (a type tag is a type
//! used only as a type parameter to identify the type which the user wants to receive).
//!
//! ## Data flow
//!
//! * A user requests an object, which is delegated to `request_by_type_tag`
//! * `request_by_type_tag` creates a `Requisition` object and passes it to `Provider::provide`
//! * The object provider's implementation of `Provider::provide` tries providing values of
//!   different types using `Requisition::provide_*`. If the type tag matches the type requested by
//!   the user, it will be stored in the `Requisition` object.
//! * `request_by_type_tag` unpacks the `Requisition` object and returns any stored value to the
//!   user.

// Heavily inspired by https://github.com/rust-lang/project-error-handling/issues/3:
//   The project-error-handling tries to improves the error trait. In order to move the trait into
//   `core`, an alternative solution to backtrace provisioning had to be found. This is, where the
//   provider API comes from.
//
//   TODO: replace module with https://github.com/rust-lang/project-error-handling/issues/3.

pub mod tags;

mod internal;
mod requisition;

use core::any::TypeId;

use self::internal::{TagValue, Tagged};
pub use self::requisition::*;

///////////////////////////////////////////////////////////////////////////////
// Provider trait
///////////////////////////////////////////////////////////////////////////////

/// Trait implemented by a type which can dynamically provide tagged values.
pub trait Provider {
    /// Object providers should implement this method to provide *all* values they are able to
    /// provide using `req`.
    fn provide<'p>(&'p self, req: Requisition<'p, '_>);
}

/// Request a specific value by a given tag from the `Provider`.
pub fn request_by_type_tag<'p, I, P: Provider + ?Sized>(provider: &'p P) -> Option<I::Type>
where
    I: TypeTag<'p>,
{
    let mut req: ConcreteRequisition<'p, I> = RequisitionImpl {
        tagged: TagValue(None),
    };
    provider.provide(Requisition(&mut req));
    req.tagged.0
}

/// This trait is implemented by specific `TypeTag` types in order to allow describing a type which
/// can be requested for a given lifetime `'p`.
///
/// A few example implementations for type-driven `TypeTag`s can be found in the [`tags`] module,
/// although crates may also implement their own tags for more complex types with internal
/// lifetimes.
pub trait TypeTag<'p>: Sized + 'static {
    /// The type of values which may be tagged by this `TypeTag` for the given lifetime.
    type Type: 'p;
}

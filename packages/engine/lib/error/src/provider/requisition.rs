use super::{TagValue, Tagged};
use crate::provider::{tags, TypeTag};

/// A helper object for providing objects by type.
///
/// An object provider provides values by calling this type's provide methods.
#[allow(missing_debug_implementations)]
pub struct Requisition<'p, 'r>(pub(super) &'r mut RequisitionImpl<dyn Tagged<'p> + 'p>);

// /// A helper object for providing objects by type.
// ///
// /// An object provider provides values by calling this type's provide methods. Since this version
// /// is `Send` it can be sent between threads to facilitate data being accessed and provided on
// /// different threads. However, this restricts the data which can be provided to `Send` data.
// #[allow(missing_debug_implementations)]
// pub struct SendRequisition<'p, 'r>(&'r mut RequisitionImpl<dyn Tagged<'p> + 'p + Send>);

macro_rules! req_methods {
    ($($T: ident),*) => {$(
        impl<'p> $T<'p, '_> {
            /// Provide a value with the given `TypeTag`.
            pub fn provide<I>(&mut self, value: I::Type) -> &mut Self
            where
                I: TypeTag<'p>,
            {
                if let Some(res @ TagValue(None)) = self.0.tagged.downcast_mut::<tags::OptionTag<I>>() {
                    res.0 = Some(value);
                }
                self
            }

            /// Provide a value or other type with only static lifetimes.
            pub fn provide_value<T, F>(&mut self, f: F) -> &mut Self
            where
                T: 'static,
                F: FnOnce() -> T,
            {
                self.provide_with::<tags::Value<T>, F>(f)
            }

            /// Provide a reference, note that the referee type must be bounded by `'static`, but
            /// may be unsized.
            pub fn provide_ref<T: ?Sized + 'static>(&mut self, value: &'p T) -> &mut Self {
                self.provide::<tags::Ref<T>>(value)
            }

            /// Provide a value with the given `TypeTag`, using a closure to prevent unnecessary
            /// work.
            pub fn provide_with<I, F>(&mut self, f: F) -> &mut Self
            where
                I: TypeTag<'p>,
                F: FnOnce() -> I::Type,
            {
                if let Some(res @ TagValue(Option::None)) = self.0.tagged.downcast_mut::<tags::OptionTag<I>>() {
                    res.0 = Some(f());
                }
                self
            }
        }
    )*};
}

req_methods!(Requisition /* , SendRequisition */);

/// A concrete request for a tagged value. Can be coerced to `Requisition` to be passed to provider
/// methods.
pub(super) type ConcreteRequisition<'p, I> = RequisitionImpl<TagValue<'p, tags::OptionTag<I>>>;

/// Implementation detail shared between `Requisition` and `ConcreteRequisition`.
///
/// Generally this value is used through the `Requisition` type as an `&mut Requisition<'p>` out
/// parameter, or constructed with the `ConcreteRequisition<'p, I>` type alias.
#[repr(transparent)]
pub(super) struct RequisitionImpl<T: ?Sized> {
    pub(super) tagged: T,
}

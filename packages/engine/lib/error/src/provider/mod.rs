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

use core::any::TypeId;

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

///////////////////////////////////////////////////////////////////////////////
// Type tags and the Tagged trait
///////////////////////////////////////////////////////////////////////////////

/// This trait is implemented by specific `TypeTag` types in order to allow
/// describing a type which can be requested for a given lifetime `'p`.
///
/// A few example implementations for type-driven `TypeTag`s can be found in the
/// [`tags`] module, although crates may also implement their own tags for more
/// complex types with internal lifetimes.
pub trait TypeTag<'p>: Sized + 'static {
    /// The type of values which may be tagged by this `TypeTag` for the given
    /// lifetime.
    type Type: 'p;
}

pub mod tags {
    //! Type tags are used to identify a type using a separate value. This module includes type tags
    //! for some very common types.
    //!
    //! Many users of the provider APIs will not need to use type tags at all. But if you want to
    //! use them with more complex types (typically those including lifetime parameters), you will
    //! need to write your own tags.

    use core::marker::PhantomData;

    use super::TypeTag;

    /// Type-based `TypeTag` for `&'p T` types.
    #[derive(Debug)]
    pub struct Ref<T: ?Sized + 'static>(PhantomData<T>);

    impl<'p, T: ?Sized + 'static> TypeTag<'p> for Ref<T> {
        type Type = &'p T;
    }

    /// Type-based `TypeTag` for `&'p mut T` types.
    #[derive(Debug)]
    pub struct RefMut<T: ?Sized + 'static>(PhantomData<T>);

    impl<'p, T: ?Sized + 'static> TypeTag<'p> for RefMut<T> {
        type Type = &'p mut T;
    }

    /// Type-based `TypeTag` for static `T` types.
    #[derive(Debug)]
    pub struct Value<T: 'static>(PhantomData<T>);

    impl<'p, T: 'static> TypeTag<'p> for Value<T> {
        type Type = T;
    }

    /// Tag combinator to wrap the given tag's value in an `Option<T>`
    #[derive(Debug)]
    pub struct OptionTag<I>(PhantomData<I>);

    impl<'p, I: TypeTag<'p>> TypeTag<'p> for OptionTag<I> {
        type Type = Option<I::Type>;
    }

    /// Tag combinator to wrap the given tag's value in an `Result<T, E>`
    #[derive(Debug)]
    pub struct ResultTag<I, E>(PhantomData<I>, PhantomData<E>);

    impl<'p, I: TypeTag<'p>, E: TypeTag<'p>> TypeTag<'p> for ResultTag<I, E> {
        type Type = Result<I::Type, E::Type>;
    }
}

/// Sealed trait representing a type-erased tagged object.
///
/// # Safety
///
/// This trait must be  exclusively implemented by the `TagValue` type.
unsafe trait Tagged<'p>: 'p {
    /// The `TypeId` of the `TypeTag` this value was tagged with.
    fn tag_id(&self) -> TypeId;
}

/// A concrete tagged value for a given tag `I`.
///
/// This is the only type which implements the `Tagged` trait, and encodes additional information
/// about the specific `TypeTag` into the type. This allows for multiple different tags to support
/// overlapping value ranges, for example, both the `Ref<str>` and `Value<&'static str>` tags can be
/// used to tag a value of type `&'static str`.
#[repr(transparent)]
struct TagValue<'p, I: TypeTag<'p>>(I::Type);

unsafe impl<'p, I> Tagged<'p> for TagValue<'p, I>
where
    I: TypeTag<'p>,
{
    fn tag_id(&self) -> TypeId {
        TypeId::of::<I>()
    }
}

macro_rules! tagged_methods {
    ($($T: ty),*) => {$(
        impl<'p> $T {
            /// Returns `true` if the dynamic type is tagged with `I`.
            #[inline]
            fn is<I>(&self) -> bool
            where
                I: TypeTag<'p>,
            {
                self.tag_id() == TypeId::of::<I>()
            }

            /// Returns some reference to the dynamic value if it is tagged with `I`, or `None` if
            /// it isn't.
            #[inline]
            fn downcast_mut<I>(&mut self) -> Option<&mut TagValue<'p, I>>
            where
                I: TypeTag<'p>,
            {
                if self.is::<I>() {
                    // SAFETY: Just checked whether we're pointing to a
                    // `TagValue<'p, I>`.
                    unsafe { Some(&mut *(self as *mut Self as *mut TagValue<'p, I>)) }
                } else {
                    None
                }
            }
        }
    )*};
}

tagged_methods!(dyn Tagged<'p>, dyn Tagged<'p> + Send);

///////////////////////////////////////////////////////////////////////////////
// Requisition and its methods
///////////////////////////////////////////////////////////////////////////////

/// A helper object for providing objects by type.
///
/// An object provider provides values by calling this type's provide methods.
#[allow(missing_debug_implementations)]
pub struct Requisition<'p, 'r>(&'r mut RequisitionImpl<dyn Tagged<'p> + 'p>);

/// A helper object for providing objects by type.
///
/// An object provider provides values by calling this type's provide methods. Since this version
/// is `Send` it can be sent between threads to facilitate data being accessed and provided on
/// different threads. However, this restricts the data which can be provided to `Send` data.
#[allow(missing_debug_implementations)]
pub struct SendRequisition<'p, 'r>(&'r mut RequisitionImpl<dyn Tagged<'p> + 'p + Send>);

macro_rules! req_methods {
    ($($T: ident),*) => {$(
        impl<'p> $T<'p, '_> {
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

            /// Provide a value with the given `TypeTag`, using a closure to prevent unnecessary
            /// work.
            pub fn provide_with<I, F>(&mut self, f: F) -> &mut Self
            where
                I: TypeTag<'p>,
                F: FnOnce() -> I::Type,
            {
                if let Some(res @ TagValue(None)) = self.0.tagged.downcast_mut::<tags::OptionTag<I>>() {
                    res.0 = Some(f());
                }
                self
            }
        }
    )*};
}

req_methods!(Requisition, SendRequisition);

/// A concrete request for a tagged value. Can be coerced to `Requisition` to be
/// passed to provider methods.
type ConcreteRequisition<'p, I> = RequisitionImpl<TagValue<'p, tags::OptionTag<I>>>;

/// Implementation detail shared between `Requisition` and `ConcreteRequisition`.
///
/// Generally this value is used through the `Requisition` type as an `&mut
/// Requisition<'p>` out parameter, or constructed with the `ConcreteRequisition<'p, I>`
/// type alias.
#[repr(transparent)]
struct RequisitionImpl<T: ?Sized> {
    tagged: T,
}

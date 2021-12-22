use core::any::TypeId;

use crate::private;

/// An identifier which may be used to tag a specific
pub trait Tag<'tag>: Sized + 'static {
    /// The type of values which may be tagged by this `Tag`.
    type Type: 'tag;
}

/// Sealed trait representing a type-erased tagged object.
pub(crate) unsafe trait Tagged<'tag>: private::Sealed + 'tag {
    /// The `TypeId` of the `Tag` this value was tagged with.
    fn tag_id(&self) -> TypeId;
}

/// Internal wrapper type with the same representation as a known external type.
#[repr(transparent)]
struct TaggedImpl<'tag, T>
where
    T: Tag<'tag>,
{
    _type: T::Type,
}

impl<'tag, T> private::Sealed for TaggedImpl<'tag, T> where T: Tag<'tag> {}

unsafe impl<'tag, T> Tagged<'tag> for TaggedImpl<'tag, T>
where
    T: Tag<'tag>,
{
    fn tag_id(&self) -> TypeId {
        TypeId::of::<T>()
    }
}

use core::marker::PhantomData;

/// Type-based `Tag` for `&'tag T` types.
pub(crate) struct Ref<T: ?Sized + 'static> {
    _marker: PhantomData<T>,
}

impl<'tag, T: ?Sized + 'static> Tag<'tag> for Ref<T> {
    type Type = &'tag T;
}

/// Type-based `Tag` for `&'tag mut T` types.
pub(crate) struct RefMut<T: ?Sized + 'static> {
    _marker: PhantomData<T>,
}

impl<'tag, T: ?Sized + 'static> Tag<'tag> for RefMut<T> {
    type Type = &'tag mut T;
}

/// Type-based `Tag` for concrete types.
pub(crate) struct Value<T: 'static> {
    _marker: PhantomData<T>,
}

impl<T: 'static> Tag<'_> for Value<T> {
    type Type = T;
}

// FIXME: This should also handle the cases for `dyn Tagged<'a> + Send`,
// `dyn Tagged<'a> + Send + Sync` and `dyn Tagged<'a> + Sync`...
//
// Should be easy enough to do it with a macro...
impl<'a> dyn Tagged<'a> {
    /// Tag a reference to a concrete type with a given `Tag`.
    ///
    /// This is like an unsizing coercion, but must be performed explicitly to
    /// specify the specific tag.
    pub fn tag_ref<I>(value: &I::Type) -> &dyn Tagged<'a>
    where
        I: Tag<'a>,
    {
        // SAFETY: `TaggedImpl<'a, I>` has the same representation as `I::Type`
        // due to `#[repr(transparent)]`.
        unsafe { &*(value as *const I::Type as *const TaggedImpl<'a, I>) }
    }

    /// Tag a reference to a concrete type with a given `Tag`.
    ///
    /// This is like an unsizing coercion, but must be performed explicitly to
    /// specify the specific tag.
    pub fn tag_mut<I>(value: &mut I::Type) -> &mut dyn Tagged<'a>
    where
        I: Tag<'a>,
    {
        // SAFETY: `TaggedImpl<'a, I>` has the same representation as `I::Type`
        // due to `#[repr(transparent)]`.
        unsafe { &mut *(value as *mut I::Type as *mut TaggedImpl<'a, I>) }
    }

    /// Tag a Box of a concrete type with a given `Tag`.
    ///
    /// This is like an unsizing coercion, but must be performed explicitly to
    /// specify the specific tag.
    #[cfg(feature = "alloc")]
    pub fn tag_box<I>(value: Box<I::Type>) -> Box<dyn Tagged<'a>>
    where
        I: Tag<'a>,
    {
        // SAFETY: `TaggedImpl<'a, I>` has the same representation as `I::Type`
        // due to `#[repr(transparent)]`.
        unsafe { Box::from_raw(Box::into_raw(value) as *mut TaggedImpl<'a, I>) }
    }

    /// Returns `true` if the dynamic type is tagged with `I`.
    #[inline]
    pub fn is<T>(&self) -> bool
    where
        T: Tag<'a>,
    {
        self.tag_id() == TypeId::of::<T>()
    }

    /// Returns some reference to the dynamic value if it is tagged with `I`,
    /// or `None` if it isn't.
    #[inline]
    pub fn downcast_ref<I>(&self) -> Option<&I::Type>
    where
        I: Tag<'a>,
    {
        if self.is::<I>() {
            // SAFETY: Just checked whether we're pointing to a
            // `TaggedImpl<'a, I>`, which was cast to from an `I::Type`.
            unsafe { Some(&*(self as *const dyn Tagged<'a> as *const I::Type)) }
        } else {
            None
        }
    }

    /// Returns some reference to the dynamic value if it is tagged with `I`,
    /// or `None` if it isn't.
    #[inline]
    pub fn downcast_mut<I>(&mut self) -> Option<&mut I::Type>
    where
        I: Tag<'a>,
    {
        if self.is::<I>() {
            // SAFETY: Just checked whether we're pointing to a
            // `TaggedImpl<'a, I>`, which was cast to from an `I::Type`.
            unsafe { Some(&mut *(self as *mut dyn Tagged<'a> as *mut I::Type)) }
        } else {
            None
        }
    }

    #[inline]
    #[cfg(feature = "alloc")]
    pub fn downcast_box<I>(self: Box<Self>) -> Result<Box<I::Type>, Box<Self>>
    where
        I: Tag<'a>,
    {
        if self.is::<I>() {
            unsafe {
                // SAFETY: Just checked whether we're pointing to a
                // `TaggedImpl<'a, I>`, which was cast to from an `I::Type`.
                let raw: *mut dyn Tagged<'a> = Box::into_raw(self);
                Ok(Box::from_raw(raw as *mut I::Type))
            }
        } else {
            Err(self)
        }
    }
}

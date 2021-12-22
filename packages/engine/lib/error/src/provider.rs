use crate::tag::{Ref, Tag, Tagged};

pub trait Provider {
    fn provide<'a>(&'a self, request: &mut Request<'a>);
}

impl dyn Provider {
    pub fn request<'a, I>(&'a self) -> Option<I::Type>
    where
        I: Tag<'a>,
    {
        request::<I, _>(|request| self.provide(request))
    }
}

pub fn request<'tag, T, F>(f: F) -> Option<<T as Tag<'tag>>::Type>
where
    T: Tag<'tag>,
    F: FnOnce(&mut Request<'tag>),
{
    let mut result: Option<<T as Tag<'tag>>::Type> = None;
    f(Request::<'tag>::new(
        <dyn Tagged>::tag_mut::<RequestTag<T>>(&mut result),
    ));
    result
}

/// An untyped and unsized request for a value of a specific type.
#[repr(transparent)]
pub struct Request<'tag> {
    tagged: dyn Tagged<'tag> + 'tag,
}

/// Implementation detail: Specific `Tag` tag used by the `Request` code under the hood.
///
/// Composition of `Tag` types!
struct RequestTag<I>(I);

impl<'req, I: Tag<'req>> Tag<'req> for RequestTag<I> {
    type Type = Option<I::Type>;
}

impl<'req> Request<'req> {
    /// Helper for performing transmutes as `Request<'a>` has the same layout as
    /// `dyn Tagged<'a> + 'a`, just with a different type!
    ///
    /// This is just to have our own methods on it, and less of the interface
    /// exposed on the `provide` implementation.
    fn new<'tag>(t: &'tag mut (dyn Tagged<'req> + 'req)) -> &'tag mut Self {
        // SAFETY: `dyn Tagged<'req>` has the same representation as `Request<'req>` due to
        // `#[repr(transparent)]`.
        unsafe { &mut *(t as *mut (dyn Tagged<'req> + 'req) as *mut Request<'req>) }
    }

    pub fn is<T>(&self) -> bool
    where
        T: Tag<'req>,
    {
        self.tagged.is::<RequestTag<T>>()
    }

    pub fn provide<I>(&mut self, value: I::Type) -> &mut Self
    where
        I: Tag<'req>,
    {
        if let Some(res @ None) = self.tagged.downcast_mut::<RequestTag<I>>() {
            *res = Some(value);
        }
        self
    }

    pub fn provide_ref<I: ?Sized + 'static>(&mut self, value: &'req I) -> &mut Self {
        if let Some(res @ None) = self.tagged.downcast_mut::<RequestTag<Ref<I>>>() {
            *res = Some(value);
        }
        self
    }

    pub fn provide_with<I, F>(&mut self, f: F) -> &mut Self
    where
        I: Tag<'req>,
        F: FnOnce() -> I::Type,
    {
        if let Some(res @ None) = self.tagged.downcast_mut::<RequestTag<I>>() {
            *res = Some(f());
        }
        self
    }
}

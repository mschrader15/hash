use crate::tag::{Tag, Tagged};

/// An untyped and unsized request for a value of a specific type.
#[repr(transparent)]
pub struct Requisition<'tag> {
    tagged: dyn Tagged<'tag> + 'tag,
}

/// Implementation detail: Specific `Tag` tag used by the `Request` code under the hood.
struct RequestTag<I>(I);

impl<'req, I: Tag<'req>> Tag<'req> for RequestTag<I> {
    type Type = Option<I::Type>;
}

impl<'ctx> Requisition<'ctx> {
    fn new<'a>(t: &'a mut (dyn Tagged<'ctx> + 'ctx)) -> &'a mut Self {
        // SAFETY: `dyn Tagged<'req>` has the same representation as `Request<'req>` due to
        // `#[repr(transparent)]`.
        unsafe { &mut *(t as *mut (dyn Tagged<'ctx> + 'ctx) as *mut Requisition<'ctx>) }
    }

    pub fn is<T>(&self) -> bool
    where
        T: Tag<'ctx>,
    {
        self.tagged.is::<RequestTag<T>>()
    }

    pub fn provide<I>(&mut self, value: I::Type) -> &mut Self
    where
        I: Tag<'ctx>,
    {
        if let Some(res @ Option::None) = self.tagged.downcast::<RequestTag<I>>() {
            *res = Some(value);
        }
        self
    }

    pub fn provide_with<I, F>(&mut self, f: F) -> &mut Self
    where
        I: Tag<'ctx>,
        F: FnOnce() -> I::Type,
    {
        if let Some(res @ Option::None) = self.tagged.downcast::<RequestTag<I>>() {
            *res = Some(f());
        }
        self
    }
}

pub(super) fn request_by_tag<'tag, T, F>(f: F) -> Option<<T as Tag<'tag>>::Type>
where
    T: Tag<'tag>,
    F: FnOnce(&mut Requisition<'tag>),
{
    let mut result: Option<<T as Tag<'tag>>::Type> = None;
    f(Requisition::<'tag>::new(
        <dyn Tagged>::tag::<RequestTag<T>>(&mut result),
    ));
    result
}

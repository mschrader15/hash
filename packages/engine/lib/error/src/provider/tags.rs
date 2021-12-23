//! Type tags are used to identify a type using a separate value. This module includes type tags
//! for some very common types.

use core::{fmt, marker::PhantomData};

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
    type Type = &'p mut T;
}

/// Tag combinator to wrap the given tag's value in an `Option<T>`
#[derive(Debug)]
pub struct OptionTag<I>(PhantomData<I>);

impl<'p, I> TypeTag<'p> for OptionTag<I>
where
    I: TypeTag<'p>,
{
    type Type = Option<I::Type>;
}

/// Tag combinator to wrap the given tag's value in an `Result<T, E>`
#[derive(Debug)]
pub struct ResultTag<I, E>(PhantomData<(I, E)>);

impl<'p, I, E> TypeTag<'p> for ResultTag<I, E>
where
    I: TypeTag<'p>,
    E: TypeTag<'p>,
{
    type Type = Result<I::Type, E::Type>;
}

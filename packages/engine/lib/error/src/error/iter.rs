use alloc::boxed::Box;
use core::{iter::FusedIterator, marker::PhantomData};

use super::Frame;
use crate::{provider::TypeTag, Report};

pub struct Chain<'r> {
    current: Option<&'r Frame>,
}

impl<'r> Chain<'r> {
    pub(super) const fn new(report: &'r Report) -> Self {
        Self {
            current: Some(&report.inner.error),
        }
    }
}

impl<'r> Iterator for Chain<'r> {
    type Item = &'r Frame;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|current| {
            self.current = current.source.as_ref().map(Box::as_ref);
            current
        })
    }
}

impl<'r> FusedIterator for Chain<'r> {}

pub struct Request<'r, I: TypeTag<'r>> {
    chain: Chain<'r>,
    _marker: PhantomData<I>,
}

impl<'r, I: TypeTag<'r>> Request<'r, I> {
    pub(super) fn new(report: &'r Report) -> Self {
        Self {
            chain: report.chain(),
            _marker: PhantomData,
        }
    }
}

impl<'r, I: TypeTag<'r>> Iterator for Request<'r, I> {
    type Item = I::Type;

    fn next(&mut self) -> Option<Self::Item> {
        self.chain.by_ref().find_map(Frame::request::<I>)
    }
}

impl<'r, I: TypeTag<'r>> FusedIterator for Request<'r, I> {}

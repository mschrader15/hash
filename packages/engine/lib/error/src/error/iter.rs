use super::Error;

/// An iterator over an [`Error`] and its sources.
///
/// If you want to omit the initial error and only process its sources, use `skip(1)`.
#[derive(Clone, Debug)]
pub struct Chain<'e> {
    current: Option<&'e (dyn Error + 'static)>,
}

impl<'e> Chain<'e> {
    pub(super) fn new(error: &'e (dyn Error + 'static)) -> Self {
        Self {
            current: Some(error),
        }
    }
}

impl<'e> Iterator for Chain<'e> {
    type Item = &'e (dyn Error + 'static);

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|current| {
            self.current = current.source();
            current
        })
    }
}

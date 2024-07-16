use std::cell::UnsafeCell;

pub trait PayloadContainer: Sized {
    type Element: Sized;

    fn new_empty() -> Self;

    fn new(element: Self::Element) -> Self;

    fn is_some(&self) -> bool;

    fn as_ref(&self) -> Option<&Self::Element>;

    fn as_mut(&mut self) -> Option<&mut Self::Element>;

    fn replace(&mut self, element: Self::Element) -> Option<Self::Element>;

    fn take(&mut self) -> Option<Self::Element>;
}


impl<T> PayloadContainer for Option<T> {
    type Element = T;

    #[inline]
    fn new_empty() -> Self {
        Self::None
    }

    #[inline]
    fn new(element: Self::Element) -> Self {
        Self::Some(element)
    }

    #[inline]
    fn is_some(&self) -> bool {
        Option::is_some(self)
    }

    #[inline]
    fn as_ref(&self) -> Option<&Self::Element> {
        Option::as_ref(self)
    }

    #[inline]
    fn as_mut(&mut self) -> Option<&mut Self::Element> {
        Option::as_mut(self)
    }

    #[inline]
    fn replace(&mut self, element: Self::Element) -> Option<Self::Element> {
        Option::replace(self, element)
    }

    #[inline]
    fn take(&mut self) -> Option<Self::Element> {
        Option::take(self)
    }
}
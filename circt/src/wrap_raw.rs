use std::borrow::Borrow;

pub trait HasRaw
where
    Self: Sized,
{
    type RawType: Sized;

    fn raw(&self) -> Self::RawType;
    fn raw_ref(&self) -> &Self::RawType;

    fn raw_mut(&mut self) -> &mut Self::RawType;

    /// Consume self into underlying C pointer
    fn take_raw(self) -> Self::RawType {
        self.raw()
    }
}
/// Trait for Rust types that wrap an underlying raw C pointer.
/// The pointer is checked to be non-null before wrapping.
pub trait WrapRawPtr: HasRaw + Sized {
    /// Wrap an existing raw C pointer.
    fn try_from_raw(raw: Self::RawType) -> Option<Self>;

    fn from_raw(raw: Self::RawType) -> Self {
        Self::try_from_raw(raw).unwrap()
    }
}

pub trait ToRawVec<T>: IntoIterator
where
    T: HasRaw,
{
    fn to_raw_vec(self) -> Vec<T::RawType>;
}

impl<T, S, I> ToRawVec<T> for I
where
    T: HasRaw,
    I: IntoIterator<Item = S>,
    S: Borrow<T>,
{
    fn to_raw_vec(self) -> Vec<T::RawType> {
        self.into_iter().map(|e| e.borrow().raw()).collect()
    }
}


// impl<T> ToRawVec<T> for &[T]
// where
//     T: HasRaw + Sized,
// {
//     fn to_raw_vec(&self) -> Vec<T::RawType> {
//         self.iter().map(|e| e.raw()).collect()
//     }
// }

// impl<T, S> ToRawVec<T> for S
// where
//     S: Iterator<Item = T> + ?Sized,
//     T: HasRaw + ?Sized,
// {
//     fn to_raw_vec(&self) -> Vec<T::RawType> {
//         self.into_iter().map(|e| e.raw()).collect()
//     }
// }

// Copyright (c) 2016-2021 Fabian Schuiki

/// Common facilities for types that wrap an underlying raw C pointer.
pub trait WrapRaw: HasRaw + Sized {
    /// Wrap an existing raw C pointer or object.
    fn from_raw(raw: Self::RawType) -> Self;
}

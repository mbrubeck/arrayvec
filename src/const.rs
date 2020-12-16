use std::mem::MaybeUninit;

use crate::arrayvec_impl::ArrayVecImpl;
use crate::CapacityError;

/// A vector with a fixed capacity.
///
/// The `ArrayVec` is a vector backed by a fixed size array. It keeps track of
/// the number of initialized elements.
///
/// The vector is a contiguous value that you can store directly on the stack
/// if needed.
///
/// It offers a simple API but also dereferences to a slice, so
/// that the full slice API is available.
///
/// ArrayVec can be converted into a by value iterator.
pub struct ArrayVec<T, const CAP: usize> {
    //xs: MaybeUninit<T>,
    xs: [MaybeUninit<T>; CAP],
    len: usize,
}

impl<T, const CAP: usize> Drop for ArrayVec<T, CAP> {
    fn drop(&mut self) {
        self.clear();
        // MaybeUninit inhibits array's drop
    }
}

macro_rules! panic_oob {
    ($method_name:expr, $index:expr, $len:expr) => {
        panic!(concat!("ArrayVec::", $method_name, ": index {} is out of bounds in vector of length {}"),
               $index, $len)
    }
}

impl<T, const CAP: usize> ArrayVec<T, CAP> {
    /// Create a new empty `ArrayVec`.
    ///
    /// Capacity is inferred from the type parameter.
    ///
    /// ```
    /// use arrayvec::ArrayVec;
    ///
    /// let mut array = ArrayVec::<[_; 16]>::new();
    /// array.push(1);
    /// array.push(2);
    /// assert_eq!(&array[..], &[1, 2]);
    /// assert_eq!(array.capacity(), 16);
    /// ```
    pub fn new() -> ArrayVec<T, CAP> {
        unsafe {
            ArrayVec { xs: MaybeUninit::uninit().assume_init(), len: 0 }
        }
    }

    /// Return the number of elements in the `ArrayVec`.
    ///
    /// ```
    /// use arrayvec::ArrayVec;
    ///
    /// let mut array = ArrayVec::from([1, 2, 3]);
    /// array.pop();
    /// assert_eq!(array.len(), 2);
    /// ```
    #[inline]
    pub fn len(&self) -> usize { self.len as usize }

    /// Returns whether the `ArrayVec` is empty.
    ///
    /// ```
    /// use arrayvec::ArrayVec;
    ///
    /// let mut array = ArrayVec::from([1]);
    /// array.pop();
    /// assert_eq!(array.is_empty(), true);
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool { self.len() == 0 }

    /// Return the capacity of the `ArrayVec`.
    ///
    /// ```
    /// use arrayvec::ArrayVec;
    ///
    /// let array = ArrayVec::from([1, 2, 3]);
    /// assert_eq!(array.capacity(), 3);
    /// ```
    #[inline(always)]
    pub fn capacity(&self) -> usize { CAP }

    /// Return if the `ArrayVec` is completely filled.
    ///
    /// ```
    /// use arrayvec::ArrayVec;
    ///
    /// let mut array = ArrayVec::<[_; 1]>::new();
    /// assert!(!array.is_full());
    /// array.push(1);
    /// assert!(array.is_full());
    /// ```
    pub fn is_full(&self) -> bool { self.len() == self.capacity() }

    /// Returns the capacity left in the `ArrayVec`.
    ///
    /// ```
    /// use arrayvec::ArrayVec;
    ///
    /// let mut array = ArrayVec::from([1, 2, 3]);
    /// array.pop();
    /// assert_eq!(array.remaining_capacity(), 1);
    /// ```
    pub fn remaining_capacity(&self) -> usize {
        self.capacity() - self.len()
    }

    pub fn push(&mut self, element: T) {
        ArrayVecImpl::push(self, element)
    }

    pub fn try_push(&mut self, element: T) -> Result<(), CapacityError<T>> {
        ArrayVecImpl::try_push(self, element)
    }

    pub unsafe fn push_unchecked(&mut self, element: T) {
        ArrayVecImpl::push_unchecked(self, element)
    }

    pub fn pop(&mut self) -> Option<T> {
        ArrayVecImpl::pop(self)
    }

    pub fn truncate(&mut self, new_len: usize) {
        ArrayVecImpl::truncate(self, new_len)
    }

    pub fn clear(&mut self) {
        ArrayVecImpl::clear(self)
    }
}

impl<T, const CAP: usize> ArrayVecImpl for ArrayVec<T, CAP> {
    type Item = T;
    const CAPACITY: usize = CAP;

    fn len(&self) -> usize { self.len() }

    unsafe fn set_len(&mut self, length: usize) {
        debug_assert!(length <= CAP);
        self.len = length;
    }

    fn as_slice(&self) -> &[Self::Item] {
        unsafe {
            std::slice::from_raw_parts(self.as_ptr(), self.len())
        }
    }

    fn as_mut_slice(&mut self) -> &mut [Self::Item] {
        let len = self.len();
        unsafe {
            std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
        }
    }

    fn as_ptr(&self) -> *const Self::Item {
        self.xs.as_ptr() as _
    }

    fn as_mut_ptr(&mut self) -> *mut Self::Item {
        self.xs.as_mut_ptr() as _
    }
}

pub use self::core::MoveCell;

#[allow(unsafe_code)]
mod core {
    use std::cell::UnsafeCell;
    use std::mem;

    /// A mutable memory location that steals ownership.
    pub struct MoveCell<T> {
        value: UnsafeCell<T>
    }

    unsafe impl<T> Send for MoveCell<T> where T: Send {}

    impl<T> MoveCell<T> {
        /// Creates a new `MoveCell` containing `value`.
        ///
        /// # Examples
        ///
        /// ```
        /// use mitochondria::MoveCell;
        ///
        /// let c = MoveCell::new("Hello cytosol!".to_owned());
        /// ```
        #[inline]
        pub fn new(value: T) -> Self {
            MoveCell {
                value: UnsafeCell::new(value)
            }
        }

        /// Consumes the `MoveCell`, returning the wrapped value.
        ///
        /// # Examples
        ///
        /// ```
        /// use mitochondria::MoveCell;
        ///
        /// let c = MoveCell::new("Hello matrix!".to_owned());
        ///
        /// let greeting = c.into_inner();
        /// ```
        pub fn into_inner(self) -> T {
            unsafe { self.value.into_inner() }
        }

        /// Replaces the value of this cell, returning the old one.
        ///
        /// # Examples
        ///
        /// ```
        /// use mitochondria::MoveCell;
        ///
        /// let c = MoveCell::new("Hello lysosome!".to_owned());
        ///
        /// let greeting = c.replace("Goodbye!".to_owned());
        /// ```
        #[inline]
        pub fn replace(&self, value: T) -> T {
            unsafe {
                mem::replace(&mut *self.value.get(), value)
            }
        }

        /// Returns a raw pointer to the underlying value in this cell.
        ///
        /// # Examples
        ///
        /// ```
        /// use mitochondria::MoveCell;
        ///
        /// let c = MoveCell::new("Hello centrosome!".to_owned());
        ///
        /// let ptr = c.as_ptr();
        /// ```
        #[inline]
        pub fn as_ptr(&self) -> *mut T {
            self.value.get()
        }

        /// Returns a mutable reference to the underlying value.
        ///
        /// This call borrows `MoveCell` mutably (at compile-time) which
        /// guarantees that we possess the only reference.
        ///
        /// ```
        /// use mitochondria::MoveCell;
        ///
        /// let mut c = MoveCell::new("Porins,".to_owned());
        /// *c.as_mut() += " unite!";
        ///
        /// assert_eq!(c.into_inner(), "Porins, unite!");
        /// ```
        pub fn as_mut(&mut self) -> &mut T {
            unsafe { &mut *self.value.get() }
        }
    }
}

impl<T: Default> Default for MoveCell<T> {
    #[inline]
    fn default() -> Self {
        MoveCell::new(Default::default())
    }
}

impl<T> From<T> for MoveCell<T> {
    #[inline]
    fn from(value: T) -> Self {
        MoveCell::new(value)
    }
}

#[cfg(test)]
mod tests {
    use MoveCell;

    #[test]
    fn smoketest() {
        let x = MoveCell::new("ribosome");
        assert_eq!(x.replace("nucleolus"), "ribosome");
        assert_eq!(x.into_inner(), "nucleolus");
    }

    #[allow(unsafe_code)]
    #[test]
    fn as_ptr() {
        let x = MoveCell::new("ribosome");
        let ptr = x.as_ptr();
        unsafe { *ptr = "nucleolus"; }
        assert_eq!(x.into_inner(), "nucleolus");
    }

    #[test]
    fn as_mut() {
        let mut x = MoveCell::new("ribosome");
        *x.as_mut() = "nucleolus";
        assert_eq!(x.into_inner(), "nucleolus");
    }

    #[test]
    fn default() {
        let x = MoveCell::<usize>::default();
        assert_eq!(x.into_inner(), 0);
    }

    #[test]
    fn from() {
        let x = MoveCell::from("ribosome");
        assert_eq!(x.into_inner(), "ribosome");
    }

    #[test]
    fn send() {
        fn assert_send<T: Send>() {}
        assert_send::<MoveCell<usize>>();
    }
}

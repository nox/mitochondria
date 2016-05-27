pub use self::core::MoveCell;

#[allow(unsafe_code)]
mod core {
    use std::cell::UnsafeCell;
    use std::mem;

    /// A mutable memory location that clones its contents on retrieval.
    pub struct MoveCell<T>(UnsafeCell<T>);

    impl<T> MoveCell<T> {
        /// Creates a new `MoveCell` containing the given value.
        ///
        /// # Example
        ///
        /// ```
        /// use mitochondria::MoveCell;
        ///
        /// let c = MoveCell::new("Hello cytosol!".to_owned());
        /// ```
        #[inline]
        pub fn new(value: T) -> Self {
            MoveCell(UnsafeCell::new(value))
        }

        /// Replaces the value of this cell, returning the old one.
        ///
        /// # Example
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
                mem::replace(&mut *self.0.get(), value)
            }
        }
    }
}

impl<T> MoveCell<T> {
    /// Sets the value of this cell, dropping the old one.
    ///
    /// # Example
    ///
    /// ```
    /// use mitochondria::MoveCell;
    ///
    /// let c = MoveCell::new("Hello vacuole!".to_owned());
    ///
    /// c.set("Hello cytoskeleton!".to_owned());
    /// ```
    #[inline]
    pub fn set(&self, value: T) {
        drop(self.replace(value));
    }
}

impl<T: Default> MoveCell<T> {
    /// Returns the value in this cell, replacing it by `T::default`.
    ///
    /// # Example
    ///
    /// ```
    /// use mitochondria::MoveCell;
    ///
    /// let c = MoveCell::new(vec!["vacuole", "cytoskeleton"]);
    ///
    /// assert_eq!(c.take(), &["vacuole", "cytoskeleton"]);
    /// assert_eq!(c.take(), &[] as &[&str]);
    /// ```
    #[inline]
    pub fn take(&self) -> T {
        self.replace(T::default())
    }
}

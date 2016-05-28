pub use self::core::OnceCell;

use std::fmt;

#[allow(unsafe_code)]
mod core {
    use std::cell::UnsafeCell;

    /// A mutable memory location that can be set only once.
    pub struct OnceCell<T>(UnsafeCell<Option<T>>);

    unsafe impl<T> Send for OnceCell<T> where T: Send {}

    impl<T> OnceCell<T> {
        /// Creates a new `OnceCell` that may already be initialized.
        ///
        /// # Example
        ///
        /// ```
        /// use mitochondria::OnceCell;
        ///
        /// let c = OnceCell::new(Some("Hello vesicle!".to_owned()));
        /// ```
        #[inline]
        pub fn new(value: Option<T>) -> Self {
            OnceCell(UnsafeCell::new(value))
        }

        /// Calls a function to try to initialize this cell.
        ///
        /// If the cell was already-initialized, the function is *not* called.
        /// Otherwise, if the function returns `Ok(value)`, the cell is
        /// initialized with `value`.
        ///
        /// This method returns `None` if the cell could not be initialized,
        /// or `Some(&value)` otherwise.
        ///
        /// # Examples
        ///
        /// ```
        /// use mitochondria::OnceCell;
        ///
        /// let c = OnceCell::new(None);
        ///
        /// assert_eq!(c.try_init_once(|| Err(())), None);
        ///
        /// let greeting: &str = c.try_init_once(|| {
        ///     Ok("Hello ribosome!".to_owned())
        /// }).unwrap();
        /// ```
        ///
        /// ```
        /// use mitochondria::OnceCell;
        ///
        /// let c = OnceCell::new(Some("Hello reticulum!".to_owned()));
        ///
        /// // Calls to `try_init_once` on initialized cells are ignored.
        /// assert_eq!(c.try_init_once(|| Ok("Goodbye!".to_owned())).unwrap(),
        ///            "Hello reticulum!");
        /// ```
        #[allow(unsafe_code)]
        #[inline]
        pub fn try_init_once<F>(&self, f: F) -> Option<&T>
            where F: FnOnce() -> Result<T, ()>
        {
            if self.borrow().is_none() {
                if let Ok(value) = f() {
                    // f() may have initialised the value already.
                    if self.borrow().is_none() {
                        unsafe { *self.0.get() = Some(value); }
                    }
                }
            }
            self.borrow()
        }

        /// Borrows the contained value, if initialized.
        ///
        /// # Examples
        ///
        /// ```
        /// use mitochondria::OnceCell;
        ///
        /// let c = OnceCell::new(None);
        ///
        /// assert!(c.borrow().is_none());
        ///
        /// let greeting = c.init_once(|| "Hello nucleus!".to_owned());
        /// assert_eq!(c.borrow(), Some(greeting));
        /// ```
        #[inline]
        pub fn borrow(&self) -> Option<&T> {
            unsafe { (*self.0.get()).as_ref() }
        }
    }
}

impl<T> OnceCell<T> {
    /// Calls a function to initialize this cell and borrows its value.
    ///
    /// If the cell was already-initialized, the function is *not* called and
    /// the returned value is the one that was already there.
    ///
    /// # Examples
    ///
    /// ```
    /// use mitochondria::OnceCell;
    ///
    /// let c = OnceCell::new(None);
    ///
    /// let greeting: &str = c.init_once(|| "Hello ribosome!".to_owned());
    /// ```
    ///
    /// ```
    /// use mitochondria::OnceCell;
    ///
    /// let c = OnceCell::new(Some("Hello reticulum!".to_owned()));
    ///
    /// // Calls to `init_once` on initialized cells are ignored.
    /// assert_eq!(c.init_once(|| "Goodbye!".to_owned()),
    ///            "Hello reticulum!");
    /// ```
    #[inline]
    pub fn init_once<F>(&self, f: F) -> &T where F: FnOnce() -> T {
        self.try_init_once(|| Ok(f())).unwrap()
    }
}

impl<T: Clone> Clone for OnceCell<T> {
    #[inline]
    fn clone(&self) -> Self {
        OnceCell::new(self.borrow().cloned())
    }
}

impl<T: fmt::Debug> fmt::Debug for OnceCell<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("OnceCell").field(&self.borrow()).finish()
    }
}

impl<T> Default for OnceCell<T> {
    #[inline]
    fn default() -> Self {
        OnceCell::new(None)
    }
}

#[cfg(test)]
mod tests {
    use OnceCell;

    #[test]
    fn reentrancy() {
        let c = OnceCell::new(None);

        let value = *c.init_once(|| {
            c.init_once(|| "ribosome");
            "nucleolus"
        });

        assert_eq!(value, "ribosome");
    }
}

pub use self::core::OnceCell;

use std::fmt;

#[allow(unsafe_code)]
mod core {
    use std::cell::UnsafeCell;

    /// A mutable memory location that can be set only once.
    ///
    /// This is useful to lazily build some value or to cache an expensive
    /// computation without the overhead of `RefCell`.
    ///
    /// # Usage
    ///
    /// ```
    /// use mitochondria::OnceCell;
    ///
    /// let c = OnceCell::new();
    ///
    /// assert_eq!(c.as_ref(), None);
    ///
    /// let value = c.init_once(|| "ribosome");
    ///
    /// assert_eq!(value, &"ribosome");
    /// assert_eq!(c.as_ref(), Some(&"ribosome"));
    ///
    /// let value_again = c.init_once(|| "nucleolus");
    ///
    /// assert_eq!(value_again, &"ribosome");
    /// ```
    ///
    /// # Reentrancy
    ///
    /// To ensure that the cell is initialized only once in all its lifetime,
    /// if `try_init_once` or `init_once` are called reentranly from
    /// the `f` argument they take, the result of that reentrant call will be
    /// used and the return value of their caller `f` will be ignored.
    ///
    /// ```
    /// use mitochondria::OnceCell;
    ///
    /// let x = OnceCell::new();
    /// let value = x.init_once(|| {
    ///     x.init_once(|| "ribosome");
    ///     "nucleolus"
    /// });
    ///
    /// assert_eq!(value, &"ribosome");
    /// assert_eq!(x.as_ref(), Some(&"ribosome"));
    ///
    /// let y = OnceCell::new();
    /// let value = y.try_init_once::<(), _>(|| {
    ///     let _ = y.try_init_once::<(), _>(|| Ok("ribosome"));
    ///     Ok("nucleolus")
    /// });
    ///
    /// assert_eq!(value, Ok(&"ribosome"));
    /// assert_eq!(y.as_ref(), Some(&"ribosome"));
    ///
    /// let z = OnceCell::new();
    /// let value = z.try_init_once::<(), _>(|| {
    ///     z.init_once(|| "ribosome");
    ///     Err(())
    /// });

    /// assert_eq!(value, Ok(&"ribosome"));
    /// assert_eq!(z.as_ref(), Some(&"ribosome"));
    /// ```
    pub struct OnceCell<T>(UnsafeCell<Option<T>>);

    unsafe impl<T> Send for OnceCell<T> where T: Send {}

    impl<T> OnceCell<T> {
        /// Creates a new `OnceCell`.
        ///
        /// # Examples
        ///
        /// ```
        /// use mitochondria::OnceCell;
        ///
        /// let c = OnceCell::<String>::new();
        /// ```
        #[inline]
        pub fn new() -> Self {
            OnceCell(UnsafeCell::new(None))
        }

        /// Creates a new `OnceCell` initialised with `value`.
        ///
        /// # Examples
        ///
        /// ```
        /// use mitochondria::OnceCell;
        ///
        /// let c = OnceCell::new_with_value(Some("Hello vesicle!".to_owned()));
        /// ```
        #[inline]
        pub fn new_with_value(value: T) -> Self {
            OnceCell(UnsafeCell::new(Some(value)))
        }

        /// Calls a function to try to initialize this cell.
        ///
        /// If the cell was already-initialized, the function is *not* called.
        /// Otherwise, if the function returns `Ok(value)`, the cell is
        /// initialized with `value`.
        ///
        /// This method returns `Err(error)` if `f` was called and returned an
        /// error, or `Ok(&value)` otherwise.
        ///
        /// # Examples
        ///
        /// ```
        /// use mitochondria::OnceCell;
        ///
        /// let c = OnceCell::new();
        ///
        /// assert_eq!(c.try_init_once(|| Err(())), Err(()));
        ///
        /// let greeting = c.try_init_once::<(), _>(|| {
        ///     Ok("Hello ribosome!".to_owned())
        /// }).unwrap();
        /// ```
        ///
        /// ```
        /// use mitochondria::OnceCell;
        ///
        /// let c = OnceCell::new_with_value("Hello reticulum!".to_owned());
        ///
        /// // Calls to `try_init_once` on initialized cells are ignored.
        /// assert_eq!(
        ///     c.try_init_once::<(), _>(|| Ok("Goodbye!".to_owned())).unwrap(),
        ///     "Hello reticulum!");
        /// ```
        #[inline]
        pub fn try_init_once<E, F>(&self, f: F) -> Result<&T, E>
            where F: FnOnce() -> Result<T, E>
        {
            if let Some(value) = self.as_ref() {
                // The cell was already initialised.
                return Ok(value);
            }
            let result = f();
            // Even if f() returned an error, the function may have initialised
            // the cell in a reentrant way, so we need to check again.
            if let Some(value) = self.as_ref() {
                return Ok(value);
            }
            let value = try!(result);
            unsafe { *self.0.get() = Some(value); }
            Ok(self.as_ref().unwrap())
        }

        /// Returns `None` if the cell is not initialised, or else returns a
        /// reference to the value wrapped in `Some`.
        ///
        /// # Examples
        ///
        /// ```
        /// use mitochondria::OnceCell;
        ///
        /// let c = OnceCell::new();
        ///
        /// assert!(c.as_ref().is_none());
        ///
        /// let greeting = c.init_once(|| "Hello nucleus!".to_owned());
        /// assert_eq!(c.as_ref(), Some(greeting));
        /// ```
        #[inline]
        pub fn as_ref(&self) -> Option<&T> {
            unsafe { (*self.0.get()).as_ref() }
        }

        /// Returns `None` if the cell is not initialised, or else returns a
        /// mutable reference to the value wrapped in `Some`.
        ///
        /// This call borrows `OnceCell` mutably (at compile-time) which
        /// guarantees that we possess the only reference.
        ///
        /// # Examples
        ///
        /// ```
        /// use mitochondria::OnceCell;
        ///
        /// let mut c = OnceCell::new();
        ///
        /// assert!(c.as_mut().is_none());
        ///
        /// c.init_once(|| "Nucleo".to_owned());
        /// *c.as_mut().unwrap() += "lus!";
        /// assert_eq!(c.as_ref().unwrap(), "Nucleolus!");
        /// ```
        #[inline]
        pub fn as_mut(&mut self) -> Option<&mut T> {
            unsafe { (*self.0.get()).as_mut() }
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
    /// let c = OnceCell::new();
    ///
    /// let greeting: &str = c.init_once(|| "Hello ribosome!".to_owned());
    /// ```
    ///
    /// ```
    /// use mitochondria::OnceCell;
    ///
    /// let c = OnceCell::new_with_value("Hello reticulum!".to_owned());
    ///
    /// // Calls to `init_once` on initialized cells are ignored.
    /// assert_eq!(
    ///     c.init_once(|| "Goodbye!".to_owned()),
    ///     "Hello reticulum!");
    /// ```
    #[inline]
    pub fn init_once<F>(&self, f: F) -> &T where F: FnOnce() -> T {
        self.try_init_once(|| Ok::<T, ()>(f())).unwrap()
    }
}

impl<T: Clone> Clone for OnceCell<T> {
    #[inline]
    fn clone(&self) -> Self {
        self.as_ref()
            .cloned()
            .map(OnceCell::new_with_value)
            .unwrap_or(OnceCell::new())
    }
}

impl<T: fmt::Debug> fmt::Debug for OnceCell<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("OnceCell").field(&self.as_ref()).finish()
    }
}

impl<T> Default for OnceCell<T> {
    #[inline]
    fn default() -> Self {
        OnceCell::new()
    }
}

impl<T> From<T> for OnceCell<T> {
    #[inline]
    fn from(value: T) -> Self {
        OnceCell::new_with_value(value)
    }
}

#[cfg(test)]
mod tests {
    use OnceCell;

    #[test]
    fn smoketest() {
        let x = OnceCell::new();
        assert_eq!(x.as_ref(), None);
        assert_eq!(x.init_once(|| "ribosome"), &"ribosome");
        assert_eq!(x.as_ref(), Some(&"ribosome"));
        assert_eq!(x.init_once(|| "nucleolus"), &"ribosome");
        assert_eq!(x.as_ref(), Some(&"ribosome"));
        assert_eq!(
            x.try_init_once::<(), _>(|| Ok("nucleolus")),
            Ok(&"ribosome"));
        assert_eq!(x.as_ref(), Some(&"ribosome"));

        let y = OnceCell::new();
        assert_eq!(y.try_init_once(|| Err(())), Err(()));
        assert_eq!(
            y.try_init_once::<(), _>(|| Ok("ribosome")),
            Ok(&"ribosome"));
        assert_eq!(y.as_ref(), Some(&"ribosome"));
        assert_eq!(
            y.try_init_once::<(), _>(|| Ok("nucleolus")),
            Ok(&"ribosome"));
        assert_eq!(y.as_ref(), Some(&"ribosome"));
        assert_eq!(y.init_once(|| "nucleolus"), &"ribosome");
        assert_eq!(y.as_ref(), Some(&"ribosome"));

        let z = OnceCell::new_with_value("ribosome");
        assert_eq!(z.as_ref(), Some(&"ribosome"));
    }

    #[test]
    fn clone() {
        let x = OnceCell::new();
        assert_eq!(x.clone().as_ref(), None);
        x.init_once(|| "ribosome");
        assert_eq!(x.clone().as_ref(), Some(&"ribosome"));
    }

    #[cfg(not(feature = "no_std"))]
    #[test]
    fn debug() {
        let x = OnceCell::new();
        assert_eq!(format!("{:?}", x), "OnceCell(None)");
        x.init_once(|| "ribosome");
        assert_eq!(format!("{:?}", x), "OnceCell(Some(\"ribosome\"))");
    }

    #[test]
    fn default() {
        let x = OnceCell::<usize>::default();
        assert_eq!(x.as_ref(), None);
    }

    #[test]
    fn from() {
        let x = OnceCell::from("ribosome");
        assert_eq!(x.as_ref(), Some(&"ribosome"));
    }

    #[test]
    fn send() {
        fn assert_send<T: Send>() {}
        assert_send::<OnceCell<usize>>();
    }
}

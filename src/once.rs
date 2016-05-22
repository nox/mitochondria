use std::borrow::Borrow;
use std::cell::UnsafeCell;

/// A mutable memory location that can be set only once.
pub struct OnceCell<T>(UnsafeCell<Option<T>>);

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

    /// Performs an initialization routine once and only once. The given closure
    /// will be executed if this is the first time `init_once` has been called,
    /// and otherwise the routine will *not* be invoked.
    ///
    /// The result of the given closure is then used to set the contents of this
    /// cell and borrowed as the return value of this method.
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
    /// // Calls to `init_once` on already-initialized cells are ignored.
    /// assert_eq!(c.init_once(|| "Goodbye!".to_owned()), "Hello reticulum!");
    /// ```
    pub fn init_once<F: FnOnce() -> T>(&self, f: F) -> &T {
        let mut value = unsafe { &mut *self.0.get() };
        if value.is_none() {
            let new = f();
            // f() may have changed value
            if value.is_none() {
                *value = Some(new);
            }
        }
        value.as_ref().unwrap().borrow()
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

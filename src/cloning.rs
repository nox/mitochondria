use std::cell::UnsafeCell;

/// A mutable memory location that clones its contents on retrieval.
pub struct CloningCell<T: Clone>(UnsafeCell<T>);

impl<T: Clone> CloningCell<T> {
    /// Creates a new `CloningCell` containing the given value.
    ///
    /// # Example
    ///
    /// ```
    /// use mitochondria::CloningCell;
    ///
    /// let c = CloningCell::new("Hello cytosol!".to_owned());
    /// ```
    #[inline]
    pub fn new(value: T) -> Self {
        CloningCell(UnsafeCell::new(value))
    }

    /// Returns a clone of the contained value.
    ///
    /// # Example
    ///
    /// ```
    /// use mitochondria::CloningCell;
    ///
    /// let c = CloningCell::new("Hello lysosome!".to_owned());
    ///
    /// let greeting = c.get();
    /// ```
    #[inline]
    pub fn get(&self) -> T {
        unsafe { (*self.0.get()).clone() }
    }

    /// Sets the contained value.
    ///
    /// # Example
    ///
    /// ```
    /// use mitochondria::CloningCell;
    ///
    /// let c = CloningCell::new("Hello vacuole!".to_owned());
    ///
    /// c.set("Hello cytoskeleton!".to_owned());
    /// ```
    #[inline]
    pub fn set(&self, value: T) {
        unsafe { *self.0.get() = value; } 
    }
}

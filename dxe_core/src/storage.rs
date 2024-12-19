use alloc::vec::Vec;

/// A sparse vector that can store values at arbitrary indices.
pub struct SparseVec<V> {
    values: Vec<Option<V>>,
}

impl<V> SparseVec<V> {
    /// Creates a new empty [SparseVec].
    pub const fn new() -> Self {
        Self { values: Vec::new() }
    }

    #[inline]
    /// Returns true if the [SparseVec] contains a value at the given index.
    pub fn contains(&self, index: usize) -> bool {
        self.values.get(index).map(|v| v.is_some()).unwrap_or(false)
    }

    #[inline]
    /// Returns the value at the given index, if it exists.
    pub fn get(&self, index: usize) -> Option<&V> {
        self.values.get(index).map(|v| v.as_ref()).unwrap_or(None)
    }

    #[inline]
    /// Inserts a value at the given index.
    pub fn insert(&mut self, index: usize, value: V) {
        if index >= self.values.len() {
            self.values.resize_with(index + 1, || None);
        }
        self.values[index] = Some(value);
    }
}

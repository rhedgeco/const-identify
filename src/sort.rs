use std::cmp::Ordering;

use crate::ConstId;

/// An array of [`ConstId`] structs that is guaranteed to be in sorted order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OrderedIdArray<const SIZE: usize> {
    ids: [ConstId; SIZE],
}

impl<const SIZE: usize> OrderedIdArray<SIZE> {
    /// Returns a new ordered array containing the sorted version of `ids`.
    pub const fn new(ids: [ConstId; SIZE]) -> Self {
        Self {
            ids: const_sort(ids),
        }
    }

    /// Returns the underlying id slice.
    pub const fn as_raw_slice(&self) -> &[ConstId] {
        &self.ids
    }

    /// Returns an [`OrderedIdSlice`] with this arrays content.
    pub const fn as_slice(&self) -> OrderedIdSlice {
        OrderedIdSlice::from_arr(self)
    }

    /// Consumes `self` and returns the underlying id array
    pub const fn into_raw(self) -> [ConstId; SIZE] {
        self.ids
    }

    /// Consumes `self` and returns a [`UniqueIdArray`].
    ///
    /// Returns `Err(Self)` if the items in this array contains duplicates
    pub const fn into_unique(self) -> Result<UniqueIdArray<SIZE>, Self> {
        match UniqueIdArray::new(self.ids) {
            Some(unique) => Ok(unique),
            None => Err(self),
        }
    }

    crate::impl_cmp!();
}

/// An unsized slice of [`ConstId`] structs that is guaranteed to be in sorted order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OrderedIdSlice<'a> {
    ids: &'a [ConstId],
}

impl<'a> OrderedIdSlice<'a> {
    /// Creates a new slice out of the data in `arr`.
    pub const fn from_arr<const SIZE: usize>(arr: &'a OrderedIdArray<SIZE>) -> Self {
        Self { ids: &arr.ids }
    }

    /// Returns the underlying slice.
    pub const fn as_raw_slice(&self) -> &[ConstId] {
        &self.ids
    }

    crate::impl_cmp!();
}

/// An array of [`ConstId`] structs that are guaranteed to be unique and in sorted order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UniqueIdArray<const SIZE: usize> {
    ids: [ConstId; SIZE],
}

impl<const SIZE: usize> UniqueIdArray<SIZE> {
    /// Returns a new unique and sorted array containing the sorted version of `ids`.
    ///
    /// Returns `None` if the `ids` contains duplicates.
    pub const fn new(ids: [ConstId; SIZE]) -> Option<Self> {
        let sorted = const_sort(ids);

        let mut i = 1;
        while i < SIZE {
            let left = sorted[i - 1].raw_value();
            let right = sorted[i].raw_value();
            if left == right {
                return None;
            }

            i += 1;
        }

        Some(Self { ids: sorted })
    }

    /// Returns the underlying id slice.
    pub const fn as_raw_slice(&self) -> &[ConstId] {
        &self.ids
    }

    /// Returns a [`UniqueIdSlice`] with this arrays content.
    pub const fn as_slice(&self) -> UniqueIdSlice {
        UniqueIdSlice::from_arr(self)
    }

    /// Consumes `self` and returns the underlying id array
    pub const fn into_raw(&self) -> [ConstId; SIZE] {
        self.ids
    }

    /// Consumes `self` and returns an [`OrderedIdArray`].
    pub const fn into_ordered(&self) -> OrderedIdArray<SIZE> {
        OrderedIdArray { ids: self.ids }
    }

    crate::impl_cmp!();
}

/// An unsized slice of [`ConstId`] structs that are guaranteed to be unique and in sorted order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UniqueIdSlice<'a> {
    ids: &'a [ConstId],
}

impl<'a> UniqueIdSlice<'a> {
    /// Creates a new slice out of the data in `arr`.
    pub const fn from_arr<const SIZE: usize>(arr: &'a UniqueIdArray<SIZE>) -> Self {
        Self { ids: &arr.ids }
    }

    /// Returns the underlying slice.
    pub const fn as_raw_slice(&self) -> &[ConstId] {
        &self.ids
    }

    crate::impl_cmp!();
}

/// Consumes an array of ids and returns the array as sorted
const fn const_sort<const SIZE: usize>(mut arr: [ConstId; SIZE]) -> [ConstId; SIZE] {
    // Bubble sort implementation pulled from this reddit comment. Thanks!
    // https://www.reddit.com/r/rust/comments/qw18oa/comment/hl05kuj
    loop {
        let mut swapped = false;
        let mut i = 1;
        while i < SIZE {
            let left = arr[i - 1].raw_value();
            let right = arr[i].raw_value();
            if left > right {
                arr[i - 1] = ConstId::from_raw(right);
                arr[i] = ConstId::from_raw(left);
                swapped = true;
            }

            i += 1;
        }
        if !swapped {
            break;
        }
    }

    arr
}

const fn const_cmp(slice1: &[ConstId], slice2: &[ConstId]) -> Ordering {
    // loop over every item in slice1
    let mut i = 0;
    while i < slice1.len() {
        // if there is nothing left in slice2,
        // slice1 has to be greater than slice2
        if i >= slice2.len() {
            return Ordering::Greater;
        }

        // compare left value to right value
        let left = slice1[i].raw_value();
        let right = slice2[i].raw_value();
        if left > right {
            // if left is greater, then slice1 is greater
            return Ordering::Greater;
        } else if left < right {
            // if left is lesser, then slice1 is lesser
            return Ordering::Less;
        }

        // increment i for the next iteration
        i += 1;
    }

    // if slice2 still has items left,
    // slice1 has to be lesser than slice2
    if i < slice2.len() {
        return Ordering::Less;
    }

    // if we make it here, then every item in each slice matched
    Ordering::Equal
}

#[macro_use]
mod sealed {
    #[macro_export]
    macro_rules! impl_cmp {
        () => {
            /// Returns the ordering between `self` and `other`
            pub const fn const_cmp_ordered<const SIZE2: usize>(
                &self,
                other: &OrderedIdArray<SIZE2>,
            ) -> Ordering {
                const_cmp(self.as_raw_slice(), other.as_raw_slice())
            }

            /// Returns the ordering between `self` and `other`
            pub const fn const_cmp_ordered_slice(&self, other: &OrderedIdSlice) -> Ordering {
                const_cmp(self.as_raw_slice(), other.as_raw_slice())
            }

            /// Returns the ordering between `self` and `other`
            pub const fn const_cmp_unique<const SIZE2: usize>(
                &self,
                other: &UniqueIdArray<SIZE2>,
            ) -> Ordering {
                const_cmp(self.as_raw_slice(), other.as_raw_slice())
            }

            /// Returns the ordering between `self` and `other`
            pub const fn const_cmp_unique_slice(&self, other: &UniqueIdSlice) -> Ordering {
                const_cmp(self.as_raw_slice(), other.as_raw_slice())
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ID1: ConstId = ConstId::from_raw(0);
    const ID2: ConstId = ConstId::from_raw(1);
    const ID3: ConstId = ConstId::from_raw(2);

    #[test]
    fn ordered() {
        let ordered = OrderedIdArray::new([ID2, ID1, ID3]);
        let slice = ordered.as_raw_slice();
        assert!(slice[0] < slice[1]);
        assert!(slice[1] < slice[2]);
    }

    #[test]
    fn unique() {
        assert!(UniqueIdArray::new([ID2, ID1, ID3, ID2]).is_none());
        let unique = UniqueIdArray::new([ID3, ID1, ID2]).unwrap();
        let slice = unique.as_raw_slice();
        assert!(slice[0] < slice[1]);
        assert!(slice[1] < slice[2]);
    }

    #[test]
    fn compare() {
        let ordered = OrderedIdArray::new([ID2, ID1]);
        let unique = UniqueIdArray::new([ID3, ID1]).unwrap();
        let ordered_eq = OrderedIdArray::new([ID1, ID2]);
        assert!(ordered.const_cmp_unique(&unique).is_lt());
        assert!(unique.const_cmp_ordered(&ordered).is_gt());
        assert!(ordered.const_cmp_ordered(&ordered_eq).is_eq());
    }

    #[test]
    fn convert() {
        let ordered = OrderedIdArray::new([ID3, ID1, ID2]);
        assert!(ordered.into_unique().is_ok());

        let ordered_duplicate = OrderedIdArray::new([ID1, ID2, ID1]);
        assert!(ordered_duplicate.into_unique().is_err());
    }
}

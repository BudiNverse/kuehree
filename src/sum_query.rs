use std::{
    num::NonZeroUsize,
    ops::{Add, Index, Sub},
};

use num::Zero;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SumQuery<T: IntoIterator> {
    prefix_sum_array: T,
}

/// This trait provides methods required for `SumQuery` types
pub trait IndexableSumQuery<T>
where
    T: Copy + Sub<Output = T> + Add<Output = T>,
{
    type PrefixSumContainer: IntoIterator<Item = T> + Index<usize, Output = T>;

    /// Construct `Self`
    ///
    /// Algorithmic complexity: O(n)
    fn new(data: impl IntoIterator<Item = T>) -> Self;

    fn prefix_sum_array(&self) -> &Self::PrefixSumContainer;

    /// Query between start range and end range
    ///
    /// Negative querying is not implemented, hence end has to be greater
    /// or equal to start
    fn query(&self, start: usize, end: usize) -> T {
        assert!(end >= start);
        let prefix_sum_array = self.prefix_sum_array();

        if start == 0 {
            prefix_sum_array[end]
        } else {
            prefix_sum_array[end] - prefix_sum_array[start - 1]
        }
    }

    /// Query between start range and end rage
    ///
    /// This function elides the branch for when `start` == 0
    fn non_zero_query(&self, start: NonZeroUsize, end: NonZeroUsize) -> T {
        assert!(end >= start);
        let prefix_sum_array = self.prefix_sum_array();
        prefix_sum_array[end.get()] - prefix_sum_array[(start.get()) - 1]
    }
}

impl<T> IndexableSumQuery<T> for SumQuery<Vec<T>>
where
    T: Copy + Sub<Output = T> + Add<Output = T>,
{
    type PrefixSumContainer = Vec<T>;

    fn new(data: impl IntoIterator<Item = T>) -> Self {
        let mut prefix_sum_array = vec![];
        for (idx, d) in data.into_iter().enumerate() {
            if idx == 0 {
                prefix_sum_array.push(d);
            } else {
                prefix_sum_array.push(d + prefix_sum_array[idx - 1]);
            }
        }
        Self { prefix_sum_array }
    }

    fn prefix_sum_array(&self) -> &Self::PrefixSumContainer {
        &self.prefix_sum_array
    }
}

impl<'a, T, const N: usize> IndexableSumQuery<T> for SumQuery<[T; N]>
where
    T: Copy + Sub<Output = T> + Add<Output = T> + Zero,
{
    type PrefixSumContainer = [T; N];

    fn new(data: impl IntoIterator<Item = T>) -> Self {
        let mut prefix_sum_array = [T::zero(); N];
        for (idx, d) in data.into_iter().enumerate() {
            if idx == 0 {
                prefix_sum_array[idx] = d;
            } else {
                prefix_sum_array[idx] = d + prefix_sum_array[idx - 1];
            }
        }
        Self { prefix_sum_array }
    }

    fn prefix_sum_array(&self) -> &Self::PrefixSumContainer {
        &self.prefix_sum_array
    }
}

impl<T, T2> From<T2> for SumQuery<Vec<T>>
where
    T2: AsRef<[T]>,
    T: Copy + Add<Output = T>,
{
    fn from(data: T2) -> Self {
        let data = data.as_ref();
        let mut prefix_sum_array = vec![];
        let mut idx = 0usize;
        for d in data {
            if idx == 0 {
                prefix_sum_array.push(*d);
            } else {
                prefix_sum_array.push(*d + prefix_sum_array[idx - 1]);
            }
            idx += 1;
        }
        Self { prefix_sum_array }
    }
}

impl<T, T2, const N: usize> From<T2> for SumQuery<[T; N]>
where
    T2: AsRef<[T]>,
    T: Copy + Add<Output = T> + Zero,
{
    fn from(data: T2) -> Self {
        let data = data.as_ref();
        let mut prefix_sum_array = [T::zero(); N];
        for (idx, d) in data.into_iter().enumerate() {
            if idx == 0 {
                prefix_sum_array[idx] = *d;
            } else {
                prefix_sum_array[idx] = *d + prefix_sum_array[idx - 1];
            }
        }
        Self { prefix_sum_array }
    }
}

#[cfg(test)]
mod test {
    use std::{collections::HashMap, mem::size_of};

    use super::*;

    #[test]
    fn test() {
        let data = [123u32];
        let data2 = vec![123u32];
        let data3 = vec![123u32];
        let data3 = data3.as_slice();
        let hm = HashMap::<u32, u32>::new();

        let _sum = SumQuery::<[_; 1]>::new(data);
        let _sum = SumQuery::<Vec<_>>::new(data2);
        let _sum = SumQuery::<Vec<_>>::from(data3);
    }

    #[test]
    fn test_query_u32_vec() {
        let data = vec![1, 3, 4, 8, 6, 1, 4, 2];
        let sum = SumQuery::<Vec<_>>::from(&data);
        data[0];

        let results = [
            (sum.query(3, 6), 19u32),
            (sum.query(0, 7), 29),
            (sum.query(0, 6), 27),
            (sum.query(1, 6), 26),
            (sum.query(2, 7), 25),
            (sum.query(5, 6), 5),
        ];

        for (l, r) in results {
            assert_eq!(l, r);
        }
    }

    #[test]
    fn test_query_u64() {
        let sum = SumQuery::<[_; 8]>::new([1, 3, 4, 8, 6, 1, 4, 2]);

        let results = [
            (sum.query(3, 6), 19u64),
            (sum.query(0, 7), 29),
            (sum.query(0, 6), 27),
            (sum.query(1, 6), 26),
            (sum.query(2, 7), 25),
            (sum.query(5, 6), 5),
        ];

        for (l, r) in results {
            assert_eq!(l, r);
        }
    }

    #[test]
    fn test_query_i8() {
        let sum = SumQuery::<[_; 8]>::new([1, 3, 4, 8, 6, 1, 4, 2]);

        let results = [
            (sum.query(3, 6), 19i8),
            (sum.query(0, 7), 29),
            (sum.query(0, 6), 27),
            (sum.query(1, 6), 26),
            (sum.query(2, 7), 25),
            (sum.query(5, 6), 5),
        ];

        for (l, r) in results {
            assert_eq!(l, r);
        }
    }

    #[test]
    fn test_query_f32() {
        let sum = SumQuery::<[_; 8]>::new([1.0f32, 3.0, 4.0, 8.0, 6.0, 1.0, 4.0, 2.0]);

        let results = [
            (sum.query(3, 6), 19.0),
            (sum.query(0, 7), 29.0),
            (sum.query(0, 6), 27.0),
            (sum.query(1, 6), 26.0),
            (sum.query(2, 7), 25.0),
            (sum.query(5, 6), 5.0),
        ];

        for (l, r) in results {
            assert_eq!(l, r);
        }
    }

    #[test]
    fn test_query_slice() {
        let slice = [1u32, 3, 4, 8, 6, 1, 4, 2];
        let slice = &slice;
        let sum = SumQuery::<[_; 8]>::from(slice);

        let results = [
            (sum.query(3, 6), 19u32),
            (sum.query(0, 7), 29),
            (sum.query(0, 6), 27),
            (sum.query(1, 6), 26),
            (sum.query(2, 7), 25),
            (sum.query(5, 6), 5),
        ];

        for (l, r) in results {
            assert_eq!(l, r);
        }
    }

    #[test]
    fn test_query_slice_vec() {
        let slice = vec![1u32, 3, 4, 8, 6, 1, 4, 2];
        // let slice_ref: &[_] = slice.as_ref();
        let sum = SumQuery::<Vec<_>>::from(&slice);

        let results = [
            (sum.query(3, 6), 19u32),
            (sum.query(0, 7), 29),
            (sum.query(0, 6), 27),
            (sum.query(1, 6), 26),
            (sum.query(2, 7), 25),
            (sum.query(5, 6), 5),
        ];

        for (l, r) in results {
            assert_eq!(l, r);
        }
    }

    #[ignore]
    #[test]
    fn test_sz() {
        let sz = size_of::<SumQuery<[u32; 256]>>();
        let sz_4 = size_of::<Box<SumQuery<[u32; 256]>>>();
        let sz_5 = size_of::<SumQuery<Vec<u32>>>();

        println!("{sz}");
        println!("{sz_4}");
        println!("{sz_5}");
    }
}

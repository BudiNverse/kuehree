use std::{
    marker::PhantomData,
    num::NonZeroUsize,
    ops::{Add, Index, Sub},
};

use num::Zero;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SumQuery<T, T3, T2 = Vec<T3>> {
    data: T,
    prefix_sum_array: T2,
    _phantom: PhantomData<T3>,
}

/// This trait provides methods required for `SumQuery` types
pub trait IndexableSumQuery<T, PrefixContainer = Vec<T>>
where
    T: Copy + Sub<Output = T>,
{
    type InternalContainer;
    type PrefixSumContainer: Index<usize, Output = T>;

    /// Construct `Self`
    ///
    /// Algorithmic complexity: O(n)
    fn new(data: Self::InternalContainer) -> Self;

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

    /// Consumes itself and returns the internal parts
    fn into_parts(self) -> (Self::InternalContainer, Self::PrefixSumContainer);
}

impl<T, const N: usize> IndexableSumQuery<T> for SumQuery<[T; N], T, [T; N]>
where
    T: Copy + Sub<Output = T> + Zero,
{
    type InternalContainer = [T; N];
    type PrefixSumContainer = [T; N];

    fn new(data: Self::InternalContainer) -> Self {
        let mut prefix_sum_array = [T::zero(); N];
        let mut idx = 0usize;

        while idx < N {
            if idx == 0 {
                prefix_sum_array[idx] = data[idx];
            } else {
                prefix_sum_array[idx] = data[idx] + prefix_sum_array[idx - 1];
            }
            idx += 1;
        }

        Self {
            data,
            prefix_sum_array,
            _phantom: PhantomData,
        }
    }

    fn into_parts(self) -> (Self::PrefixSumContainer, Self::PrefixSumContainer) {
        (self.data, self.prefix_sum_array)
    }

    fn prefix_sum_array(&self) -> &Self::PrefixSumContainer {
        &self.prefix_sum_array
    }
}

impl<T> IndexableSumQuery<T> for SumQuery<Vec<T>, T>
where
    T: Copy + Sub<Output = T> + Add<Output = T>,
{
    type InternalContainer = Vec<T>;
    type PrefixSumContainer = Vec<T>;

    fn new(data: Self::InternalContainer) -> Self {
        let mut prefix_sum_array = vec![];
        let mut idx = 0usize;

        while idx < data.len() {
            if idx == 0 {
                prefix_sum_array.push(data[idx]);
            } else {
                prefix_sum_array.push(data[idx] + prefix_sum_array[idx - 1]);
            }
            idx += 1;
        }

        Self {
            data,
            prefix_sum_array,
            _phantom: PhantomData,
        }
    }

    fn into_parts(self) -> (Self::PrefixSumContainer, Self::PrefixSumContainer) {
        (self.data, self.prefix_sum_array)
    }

    fn prefix_sum_array(&self) -> &Self::PrefixSumContainer {
        &self.prefix_sum_array
    }
}

impl<'a, T> IndexableSumQuery<T> for SumQuery<&'a [T], T>
where
    T: Copy + Sub<Output = T> + Add<Output = T>,
{
    type InternalContainer = &'a [T];
    type PrefixSumContainer = Vec<T>;

    fn new(data: Self::InternalContainer) -> Self {
        let mut prefix_sum_array = vec![];
        let mut idx = 0usize;

        while idx < data.len() {
            if idx == 0 {
                prefix_sum_array.push(data[idx]);
            } else {
                prefix_sum_array.push(data[idx] + prefix_sum_array[idx - 1]);
            }
            idx += 1;
        }

        Self {
            data,
            prefix_sum_array,
            _phantom: PhantomData,
        }
    }

    fn into_parts(self) -> (Self::InternalContainer, Self::PrefixSumContainer) {
        (self.data, self.prefix_sum_array)
    }

    fn prefix_sum_array(&self) -> &Self::PrefixSumContainer {
        &self.prefix_sum_array
    }
}

impl<'a, T, const N: usize> IndexableSumQuery<T> for SumQuery<&'a [T; N], T, [T; N]>
where
    T: Copy + Sub<Output = T> + Add<Output = T> + Zero,
{
    type InternalContainer = &'a [T; N];
    type PrefixSumContainer = [T; N];

    fn new(data: Self::InternalContainer) -> Self {
        let mut prefix_sum_array = [T::zero(); N];
        let mut idx = 0usize;

        while idx < N {
            if idx == 0 {
                prefix_sum_array[idx] = data[idx];
            } else {
                prefix_sum_array[idx] = data[idx] + prefix_sum_array[idx - 1];
            }
            idx += 1;
        }

        Self {
            data,
            prefix_sum_array,
            _phantom: PhantomData,
        }
    }

    fn into_parts(self) -> (Self::InternalContainer, Self::PrefixSumContainer) {
        (self.data, self.prefix_sum_array)
    }

    fn prefix_sum_array(&self) -> &Self::PrefixSumContainer {
        &self.prefix_sum_array
    }
}

#[cfg(test)]
mod test {
    use std::mem::size_of;

    use super::*;

    #[test]
    fn test() {
        let data = [123u32];
        let data2 = vec![123u32];
        let data3 = vec![123u32];
        let data3 = data3.as_slice();

        let _sum = SumQuery::<[_; 1], _, _>::new(data);
        let _sum = SumQuery::<Vec<_>, _>::new(data2);
        let _sum = SumQuery::<&[_], _>::new(data3);
    }

    #[test]
    fn test_query_u32_vec() {
        let sum = SumQuery::<[_; 8], _, _>::new([1, 3, 4, 8, 6, 1, 4, 2]);

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
        let sum = SumQuery::<[_; 8], _, _>::new([1, 3, 4, 8, 6, 1, 4, 2]);

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
        let sum = SumQuery::<[_; 8], _, _>::new([1, 3, 4, 8, 6, 1, 4, 2]);

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
        let sum = SumQuery::<[_; 8], _, _>::new([1.0f32, 3.0, 4.0, 8.0, 6.0, 1.0, 4.0, 2.0]);

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
        let slice = [1, 3, 4, 8, 6, 1, 4, 2];
        let sum = SumQuery::<&[_; 8], _, _>::new(&slice);

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
        let slice_ref: &[_] = slice.as_ref();
        let sum = SumQuery::<&[_], _>::new(slice_ref);

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
        let sz = size_of::<SumQuery<&[u32; 256], u32, [u32; 256]>>();
        let sz_2 = size_of::<SumQuery<&[u32; 256], u32>>();
        let sz_3 = size_of::<SumQuery<&[u32], u32>>();
        let sz_4 = size_of::<Box<SumQuery<[u32; 256], u32, [u32; 256]>>>();
        let sz_5 = size_of::<SumQuery<Vec<u32>, u32>>();
        let sz_6 = size_of::<SumQuery<Box<[u32]>, u32, Box<[u32]>>>();
        
        println!("{sz}");
        println!("{sz_2}");
        println!("{sz_3}");
        println!("{sz_4}");
        println!("{sz_5}");
        println!("{sz_6}");
    }
}

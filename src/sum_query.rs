use num::Num;

/// SumQuery type that uses `Vec<T>` as its underlying data structure
/// 
/// Heap allocation: Yes
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SumQueryVec<T> {
    data: Vec<T>,
    prefix_sum_array: Vec<T>,
}


/// SumQuery type that uses `[T; N]` as its underlying data structure
/// 
/// Heap allocation: No
/// 
/// To allocate on the heap, use `Box<T>`
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SumQueryFixed<T, const N: usize> {
    data: [T; N],
    prefix_sum_array: [T; N],
}

/// SumQuery type that uses `&[T]` as its underlying data structure
/// Internal prefix_sum_array uses `Vec<T>`
/// 
/// Heap allocation: Yes
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SumQuerySlice<'a, T> {
    data: &'a [T],
    prefix_sum_array: Vec<T>
}

/// This trait provides methods required for `SumQuery` types
pub trait SumQuery {
    type InternalContainer;
    type InternalType: Copy;

    /// Construct `Self`
    ///
    /// Algorithmic complexity: O(n)
    fn new(data: Self::InternalContainer) -> Self;

    /// Query between start range and end range
    ///
    /// Negative querying is not implemented, hence end has to be greater
    /// or equal to start
    fn query(&self, start: usize, end: usize) -> Self::InternalType;
}

impl<T: Num + Copy, const N: usize> From<[T; N]> for SumQueryFixed<T, N> {
    fn from(data: [T; N]) -> Self {
        Self::new(data)
    }
}

impl<T: Num + Copy, const N: usize> From<[T; N]> for SumQueryVec<T> {
    fn from(data: [T; N]) -> Self {
        Self::new(data.to_vec())
    }
}

impl<'a, T: Num + Copy> From<&'a [T]> for SumQuerySlice<'a, T> {
    fn from(data: &'a [T]) -> Self {
        Self::new(data)
    }
}

impl<T: Num + Copy> From<Vec<T>> for SumQueryVec<T> {
    fn from(data: Vec<T>) -> Self {
        Self::new(data)
    }
}


impl<'a, T: Num + Copy> SumQuery for SumQuerySlice<'a, T> {
    type InternalContainer = &'a [T];

    type InternalType = T;

    fn new(data: Self::InternalContainer) -> Self {
        let mut prefix_sum_array = Vec::with_capacity(data.len());
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
        }
    }

    fn query(&self, start: usize, end: usize) -> Self::InternalType {
        assert!(end >= start);

        if start == 0 {
            return self.prefix_sum_array[end];
        }

        self.prefix_sum_array[end] - self.prefix_sum_array[start - 1]
    }
}


impl<T: Num + Copy, const N: usize> SumQuery for SumQueryFixed<T, N> {
    type InternalContainer = [T; N];

    type InternalType = T;

    fn new(data: [T; N]) -> Self {
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
        }
    }

    fn query(&self, start: usize, end: usize) -> T {
        assert!(end >= start);

        if start == 0 {
            return self.prefix_sum_array[end];
        }

        self.prefix_sum_array[end] - self.prefix_sum_array[start - 1]
    }
}

impl<T: Num + Copy> SumQuery for SumQueryVec<T> {
    type InternalContainer = Vec<T>;

    type InternalType = T;

    fn new(data: Vec<T>) -> Self {
        let mut prefix_sum_array = Vec::with_capacity(data.len());
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
        }
    }

    fn query(&self, start: usize, end: usize) -> T {
        assert!(end >= start);

        if start == 0 {
            return self.prefix_sum_array[end];
        }

        self.prefix_sum_array[end] - self.prefix_sum_array[start - 1]
    }
}

#[cfg(test)]
mod tests {
    use crate::sum_query::{SumQuery, SumQueryFixed, SumQueryVec};

    #[test]
    fn test_new() {
        let sum = SumQueryFixed::from([1, 3, 4, 8, 6, 1, 4, 2]);
        assert_eq!(sum.prefix_sum_array, [1, 4, 8, 16, 22, 23, 27, 29]);
    }

    #[test]
    fn test_new_vec() {
        let sum = SumQueryVec::from([1, 3, 4, 8, 6, 1, 4, 2]);
        assert_eq!(sum.prefix_sum_array, vec![1, 4, 8, 16, 22, 23, 27, 29]);
    }

    #[test]
    fn test_query_u32() {
        let sum = SumQueryFixed::from([1, 3, 4, 8, 6, 1, 4, 2]);

        let results = [
            (sum.query(3, 6), 19u32),
            (sum.query(0, 7), 29),
            (sum.query(0, 6), 27),
            (sum.query(1, 6), 26),
            (sum.query(2, 7), 25),
            (sum.query(5, 6), 5),
            (sum.query(6, 6), 4),
        ];

        for (l, r) in results {
            assert_eq!(l, r);
        }
    }

    #[test]
    fn test_query_u32_vec() {
        let sum = SumQueryVec::from([1, 3, 4, 8, 6, 1, 4, 2]);

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
        let sum = SumQueryFixed::from([1, 3, 4, 8, 6, 1, 4, 2]);

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
        let sum = SumQueryFixed::from([1, 3, 4, 8, 6, 1, 4, 2]);

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
        let sum = SumQueryFixed::from([1.0, 3.0, 4.0, 8.0, 6.0, 1.0, 4.0, 2.0]);

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
}

use num::Integer;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SumQueryVec<T> {
    data: Vec<T>,
    prefix_sum_array: Vec<T>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SumQueryFixed<T, const N: usize> {
    data: [T; N],
    prefix_sum_array: [T; N],
}

pub trait SumQuery {
    type InternalContainer;
    type InternalType: Copy;

    fn new(data: Self::InternalContainer) -> Self;
    fn query(&self, start: usize, end: usize) -> Self::InternalType;
}

impl<T: Integer + Copy, const N: usize> From<[T; N]> for SumQueryFixed<T, N> {
    fn from(data: [T; N]) -> Self {
        Self::new(data)
    }
}

impl<T: Integer + Copy, const N: usize> SumQuery for SumQueryFixed<T, N> {
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


impl<T: Integer + Copy> SumQuery for SumQueryVec<T> {
    type InternalContainer = Vec<T>;

    type InternalType = T;

    fn new(data: Vec<T>) -> Self {
        let mut prefix_sum_array = Vec::with_capacity(data.capacity());
        let mut idx = 0usize;

        while idx < data.capacity() {
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



mod tests {
    use crate::{SumQueryFixed, SumQuery};

    #[test]
    fn test_new() {
        let sum = SumQueryFixed::from([1, 3, 4, 8, 6, 1, 4, 2]);
        assert_eq!(sum.prefix_sum_array, [1, 4, 8, 16, 22, 23, 27, 29]);
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
}

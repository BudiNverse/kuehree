#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SumQuery<T, const N: usize> {
    data: [T; N],
    prefix_sum_array: [T; N],
}

impl<const N: usize> From<[u32; N]> for SumQuery<u32, N> {
    fn from(data: [u32; N]) -> Self {
        Self::new(data)
    }
}

impl<const N: usize> SumQuery<u32, N> {
    const fn new(data: [u32; N]) -> Self {
        let mut prefix_sum_array = [0; N];
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

    const fn query(&self, start: usize, end: usize) -> u32 {
        assert!(end >= start);

        if start == 0 {
            return self.prefix_sum_array[end];
        }

        self.prefix_sum_array[end] - self.prefix_sum_array[start - 1]
    }
}

mod tests {
    use crate::SumQuery;

    #[test]
    fn test_new() {
        let sum = SumQuery::from([1, 3, 4, 8, 6, 1, 4, 2]);
        assert_eq!(sum.prefix_sum_array, [1, 4, 8, 16, 22, 23, 27, 29]);
    }

    #[test]
    fn test_query() {
        let sum = SumQuery::from([1, 3, 4, 8, 6, 1, 4, 2]);

        let results = [
            (sum.query(3, 6), 19u32),
            (sum.query(0, 7), 29),
            (sum.query(0, 6), 27),
            (sum.query(1, 6), 26),
            (sum.query(2, 7), 25),
            (sum.query(5, 6), 5),
        ];

        for (l, r) in results {
            println!("Left: {l}, right: {r}");
            assert_eq!(l, r);
        }
    }
}

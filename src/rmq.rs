use std::marker::PhantomData;

use num::Num;

pub trait SparseTable {
    type InternalContainer;
    type InternalType: Copy;
}

pub struct Max;
pub struct Min;

pub struct SegmentTree<T> {
    _phantom: PhantomData<T>
}

impl<T: Num> SegmentTree<T> {
    pub fn new(data: T) -> Self {
        todo!()
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Rmq<T, T2, M> {
    _phantom: PhantomData<T>,
    _phantom_2: PhantomData<T2>,
    _phantom_3: PhantomData<M>,
}

impl<T, T2, M> Rmq<T, T2, M> {
    pub fn new(data: T) -> Self {
        todo!()
    }
}

impl<T2: Num, const N: usize> Rmq<[T2; N], T2, Max> {
    pub fn query() {}
}

#[cfg(test)]
mod test {
    use super::{Max, Min, Rmq, SegmentTree};

    #[ignore]
    #[test]
    #[allow(unused_variables)]
    fn test() {
        let arr = [1, 3, 4, 8, 6, 1, 4, 2];
        let arr_2 = vec![1, 3, 4, 8, 6, 1, 4, 2];
        let sgtree = SegmentTree::new(10u8);

        let range_min = Rmq::<[u8; 8], u8, Min>::new(arr);
        let range_max = Rmq::<[u8; 8], u8, Max>::new(arr);
        let range_max_sgmt_tree = Rmq::<SegmentTree<u8>, u8, Max>::new(sgtree);
        let range_max_sgmt_tree = Rmq::<Vec<u8>, u8, Max>::new(arr_2);
    }
}

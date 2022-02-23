pub trait SparseTable {
    type InternalContainer;
    type InternalType: Copy;
}
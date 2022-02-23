# `Kuehree`
> ‼️ Work in progress, DO NOT USE FOR PRODUCTION

This currently implements Prefix Sum Arrays, which allows you do to Sum range queries in O(1),
with the caveat being that data data cannot be changed.

### Plans
Idk man, maybe I will add other range queries stuff to this

### Sample Code
```rust
fn test_query() {
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

    for (left_results, expected_ans) in results {
        assert_eq!(left_results, expected_ans);
    }
}
```
// Forbid warnings in release builds:
#![cfg_attr(not(debug_assertions), deny(warnings))]
#![forbid(unsafe_code)]
#![warn(
    clippy::all,
    clippy::await_holding_lock,
    clippy::char_lit_as_u8,
    clippy::checked_conversions,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::doc_markdown,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::exit,
    clippy::expl_impl_clone_on_copy,
    clippy::explicit_deref_methods,
    clippy::explicit_into_iter_loop,
    clippy::fallible_impl_from,
    clippy::filter_map_next,
    clippy::float_cmp_const,
    clippy::fn_params_excessive_bools,
    clippy::if_let_mutex,
    clippy::imprecise_flops,
    clippy::inefficient_to_string,
    clippy::invalid_upcast_comparisons,
    clippy::large_types_passed_by_value,
    clippy::let_unit_value,
    clippy::linkedlist,
    clippy::lossy_float_literal,
    clippy::macro_use_imports,
    clippy::manual_ok_or,
    clippy::map_flatten,
    clippy::match_on_vec_items,
    clippy::match_same_arms,
    clippy::match_wildcard_for_single_variants,
    clippy::mem_forget,
    clippy::mismatched_target_os,
    clippy::missing_errors_doc,
    clippy::missing_safety_doc,
    clippy::mut_mut,
    clippy::mutex_integer,
    clippy::needless_borrow,
    clippy::needless_continue,
    clippy::needless_pass_by_value,
    clippy::option_option,
    clippy::path_buf_push_overwrite,
    clippy::ptr_as_ptr,
    clippy::avoid_breaking_exported_api,
    clippy::ref_option_ref,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_functions_in_if_condition,
    clippy::string_add_assign,
    clippy::string_add,
    clippy::string_lit_as_bytes,
    clippy::string_to_string,
    clippy::todo,
    clippy::trait_duplication_in_bounds,
    clippy::unimplemented,
    clippy::unnested_or_patterns,
    clippy::unused_self,
    clippy::useless_transmute,
    clippy::verbose_file_reads,
    clippy::zero_sized_map_values,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]

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
    use crate::{SumQuery, SumQueryFixed, SumQueryVec};

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

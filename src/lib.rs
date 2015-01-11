// Copyright 2015 Cameron Hart
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate test;

use std::mem;
use std::num::Int;
use std::ptr;

////////////////////////////////////////////////////////////////////////////////
// Insertion sort (based off libstd collections slice version)
////////////////////////////////////////////////////////////////////////////////

fn insertsort_impl<T, F>(ptr: *mut T, len: isize, lt: &F) where F: Fn(&T, &T) -> bool {
    // 1 <= i < len;
    for i in (1..len) {
        // j satisfies: 0 <= j <= i;
        let mut j = i;
        unsafe {
            // `i` is in bounds.
            let read_ptr = ptr.offset(i) as *const T;

            // find where to insert, we need to do strict <,
            // rather than <=, to maintain stability.

            // 0 <= j - 1 < len, so .offset(j - 1) is in bounds.
            while j > 0 &&
                    lt(&*read_ptr, &*ptr.offset(j - 1)) {
                j -= 1;
            }

            // shift everything to the right, to make space to
            // insert this value.

            // j + 1 could be `len` (for the last `i`), but in
            // that case, `i == j` so we don't copy. The
            // `.offset(j)` is always in bounds.

            if i != j {
                let tmp = ptr::read(read_ptr);
                ptr::copy_memory(ptr.offset(j + 1),
                                 &*ptr.offset(j),
                                 (i - j) as usize);
                ptr::copy_nonoverlapping_memory(ptr.offset(j),
                                                &tmp as *const T,
                                                1);
                mem::forget(tmp);
            }
        }
    }
}

pub fn insertsort_by<T: PartialOrd, F>(v: &mut[T], lt: F) where F: Fn(&T, &T) -> bool {
    insertsort_impl(v.as_mut_ptr(), v.len() as isize, &lt);
}

pub fn insertsort<T: PartialOrd>(v: &mut[T]) {
    insertsort_by(v, |a, b| a.lt(b));
}

////////////////////////////////////////////////////////////////////////////////
// Heap sort
////////////////////////////////////////////////////////////////////////////////

/// Builds a heap in the array so that the largest element is at the root.
/// Operates on data in-place.
fn heapify<T, F>(ptr: *mut T, len: isize, lt: &F) where F: Fn(&T, &T) -> bool {
    // start is assigned to the index of the last parent node
    let mut start = (len - 2) / 2;
    let end = len - 1;
    while start >= 0 {
        // shift down the node at index start such that all nodes below start
        // are in heap order
        shift_down(ptr, start, end, lt);
        // go up the next parent node
        start = start - 1;
    }
    // after shifting down the root all nodes are in heap order
}

/// Repair the heap whose root element is at index start.
/// Assumes a valid heap struture.
fn shift_down<T, F>(ptr: *mut T, start: isize, end: isize, lt: &F) where F: Fn(&T, &T) -> bool {
    let mut root = start;
    let mut next_root = root * 2;
    // while the root has at least one child
    while next_root < end {
        // left child
        let left_child = next_root + 1;
        // keep track of child to swap with
        let mut swap = root;
        unsafe {
            if lt(&*ptr.offset(swap), &*ptr.offset(left_child)) {
                swap = left_child;
            }
            // if there is a right child and it is greater
            let right_child = left_child + 1;
            if right_child <= end && lt(&*ptr.offset(swap), &*ptr.offset(right_child)) {
                swap = right_child;
            }
            if swap == root {
                // the root holds the largest element
                return;
            }
            ptr::swap(ptr.offset(root), ptr.offset(swap));
        }
        // repeat to continue shifting down the child
        root = swap;
        next_root = root * 2;
    }
}

/// Internal heapsort implementation
fn heapsort_impl<T, F>(ptr: *mut T, len: isize, lt: &F) where F: Fn(&T, &T) -> bool {
    // build the heap in-place so the largest value is at the root
    heapify(ptr, len, lt);
    let mut end = len - 1;
    while end > 0 {
        // ptr is the root and largest value, swap it to the end of the sorted elements
        unsafe { ptr::swap(ptr.offset(end), ptr); }
        // the heap size is reduced by one
        end = end - 1;
        // the swap invalidated the heap, so restore it
        shift_down(ptr, 0, end, lt);
    }
}

pub fn heapsort_by<T: PartialOrd, F>(v: &mut[T], lt: F) where F: Fn(&T, &T) -> bool {
    let len = v.len() as isize;
    if len > 0 {
        let ptr = v.as_mut_ptr();
        heapsort_impl(ptr, len, &lt);
    }
}

pub fn heapsort<T: PartialOrd>(v: &mut[T]) {
    heapsort_by(v, |a, b| a.lt(b));
}

////////////////////////////////////////////////////////////////////////////////
// Introspection sort
////////////////////////////////////////////////////////////////////////////////

const THRESHOLD: isize = 16;

#[inline]
fn lg(n: usize) -> usize {
    mem::size_of::<usize>() * 8 - 1 - n.leading_zeros()
}

#[inline]
/// Calculates the number of elements between the first and last pointers
fn ptr_distance<T>(last: *const T, first: *const T) -> isize {
    ((last as usize - first as usize) / mem::size_of::<T>()) as isize
}

#[inline]
fn median_3<T, F>(a: *mut T, b: *mut T, c: *mut T, lt: &F) -> *mut T where F: Fn(&T, &T) -> bool {
    unsafe {
        if lt(&*a, &*b) {
            if lt(&*b, &*c) {
                b
            }
            else if lt(&*a, &*c) {
                c
            }
            else {
                a
            }
        }
        else if lt(&*a, &*c) {
            a
        }
        else if lt(&*b, &*c) {
            c
        }
        else {
            b
        }
    }
}

#[inline]
fn partition<T, F>(mut first: *mut T, mut last: *mut T, pivot: *mut T, lt: &F) -> *mut T
        where F: Fn(&T, &T) -> bool {
    unsafe {
        loop {
            // find first element greater than the pivot
            while lt(&*first, &*pivot) {
                first = first.offset(1);
            }
            // find last element smaller than the pivot
            last = last.offset(-1);
            while lt(&*pivot, &*last) {
                last = last.offset(-1);
            }
            // if first and last have met then partitioning is complete
            if !((first as usize) < (last as usize)) {
                return first;
            }
            // swap the first and last elements to be on the right side of the pivot
            ptr::swap(first, last);
            // move to the next element
            first = first.offset(1);
        }
    }
}

#[inline]
fn partition_pivot<T, F>(ptr: *mut T, len: isize, lt: &F) -> *mut T where F: Fn(&T, &T) -> bool {
    unsafe {
        // choose a pivot based on media of 3 elements
        let pivot = median_3(ptr.offset(1), ptr.offset(len / 2), ptr.offset(len - 1), lt);
        // swap the pivot with the first element so it's already partitioned
        ptr::swap(ptr, pivot);
        // partition elements on either side of the pivot
        return partition(ptr.offset(1), ptr.offset(len), ptr, lt);
    }
}

fn introsort_loop<T, F>(ptr: *mut T, mut last: *mut T, mut depth_limit: usize, lt: &F) where F: Fn(&T, &T) -> bool {
    let mut len = ptr_distance(last, ptr);
    // once threshold is reached rely on final insertion sort pass
    while len > THRESHOLD {
        // if the depth limit has been reached switch to heapsort
        if depth_limit == 0 {
            heapsort_impl(ptr, len, lt);
            return;
        }
        depth_limit -= 1;
        // choose partition and pivot
        let pivot = partition_pivot(ptr, len, lt);
        // introsort the elements after the pivot
        introsort_loop(pivot, last, depth_limit, lt);
        len = ptr_distance(pivot, ptr);
        last = pivot;
    }
}

#[inline]
fn introsort_impl<T: PartialOrd, F>(v: &mut[T], lt: F) where F: Fn(&T, &T) -> bool {
    let len = v.len() as isize;
    if len > 0 {
        let ptr = v.as_mut_ptr();
        unsafe {
            introsort_loop(ptr, ptr.offset(len), 2 * lg(len as usize), &lt);
        }
        // insertsort mostly sorted data
        insertsort_impl(ptr, len, &lt);
    }
}

///
/// Sorts the slice, in place, using `lt` to compare elements.
///
/// The order of equal elements is not guaranteed to be preserved.
///
/// This sort is `O(n log n)` worst-case and stable.
///
/// The sort is implemented using the Introsort algorithm. Introsort or
/// introspective sort is a hybrid sorting algorithm that provides both fast
/// average performance and (asymptotically) optimal worst-case performance.
/// It begins with quicksort and switches to heapsort when the recursion depth
/// exceeds a level based on (the logarithm of) the number of elements being
/// sorted. This combines the good parts of both algorithms, with practical
/// performance comparable to quicksort on typical data sets and worst-case
/// O(n log n) runtime due to the heap sort.
///
/// # Examples
///
/// ```rust
/// let mut v = [5i, 4, 1, 3, 2];
/// introsort_by(v, |a, b| a.lt(b));
/// assert!(v == [1, 2, 3, 4, 5]);
///
/// // reverse sorting
/// introsort_by(v, |a, b| b.lt(a));
/// assert!(v == [5, 4, 3, 2, 1]);
/// ```
pub fn introsort_by<T: PartialOrd, F>(v: &mut[T], lt: F) where F: Fn(&T, &T) -> bool {
    introsort_impl(v, lt);
}

/// Sorts the slice, in place.
///
/// This is equivalent to `self.sort_by(|a, b| a.cmp(b))`.
///
/// # Examples
///
/// ```rust
/// let mut v = [-5i, 4, 1, -3, 2];
///
/// v.sort();
/// assert!(v == [-5i, -3, 1, 2, 4]);
/// ```
pub fn introsort<T: PartialOrd>(v: &mut[T]) {
    introsort_impl(v, |a, b| a.lt(b))
}

////////////////////////////////////////////////////////////////////////////////
// Test harness
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use std::rand::{Rng, thread_rng};
    use insertsort;
    use insertsort_by;
    use heapsort;
    use heapsort_by;
    use introsort;
    use introsort_by;

    #[test]
    fn test_insertsort() {
        for len in range(4u, 25) {
            for _ in range(0i, 100) {
                let mut v = thread_rng().gen_iter::<usize>().take(len)
                                      .collect::<Vec<usize>>();
                let mut v1 = v.clone();

                insertsort(v.as_mut_slice());
                assert!(v.as_slice().windows(2).all(|w| w[0] <= w[1]));

                insertsort_by(v1.as_mut_slice(), |a, b| a.lt(b));
                assert!(v1.as_slice().windows(2).all(|w| w[0] <= w[1]));

                insertsort_by(v1.as_mut_slice(), |a, b| b.lt(a));
                assert!(v1.as_slice().windows(2).all(|w| w[0] >= w[1]));
            }
        }

        // shouldn't panic
        let mut v: [usize; 0] = [];
        insertsort(v.as_mut_slice());

        let mut v = [0xDEADBEEFu];
        insertsort(v.as_mut_slice());
        assert!(v == [0xDEADBEEF]);
    }

    #[test]
    fn test_heapsort() {
        for len in range(4u, 25) {
            for _ in range(0i, 100) {
                let mut v = thread_rng().gen_iter::<usize>().take(len)
                                      .collect::<Vec<usize>>();
                let mut v1 = v.clone();

                heapsort(v.as_mut_slice());
                assert!(v.as_slice().windows(2).all(|w| w[0] <= w[1]));

                heapsort_by(v1.as_mut_slice(), |a, b| a.lt(b));
                assert!(v1.as_slice().windows(2).all(|w| w[0] <= w[1]));

                heapsort_by(v1.as_mut_slice(), |a, b| b.lt(a));
                assert!(v1.as_slice().windows(2).all(|w| w[0] >= w[1]));
            }
        }

        // shouldn't panic
        let mut v: [usize; 0] = [];
        heapsort(v.as_mut_slice());

        let mut v = [0xDEADBEEFu];
        heapsort(v.as_mut_slice());
        assert!(v == [0xDEADBEEF]);
    }

    #[test]
    fn test_introsort() {
        for len in range(4u, 25) {
            for _ in range(0i, 100) {
                let mut v = thread_rng().gen_iter::<usize>().take(len)
                                      .collect::<Vec<usize>>();
                let mut v1 = v.clone();

                introsort(v.as_mut_slice());
                assert!(v.as_slice().windows(2).all(|w| w[0] <= w[1]));

                introsort_by(v1.as_mut_slice(), |a, b| a.lt(b));
                assert!(v1.as_slice().windows(2).all(|w| w[0] <= w[1]));

                introsort_by(v1.as_mut_slice(), |a, b| b.lt(a));
                assert!(v1.as_slice().windows(2).all(|w| w[0] >= w[1]));
            }
        }

        // shouldn't panic
        let mut v: [usize; 0] = [];
        introsort(v.as_mut_slice());

        let mut v = [0xDEADBEEFu];
        introsort(v.as_mut_slice());
        assert!(v == [0xDEADBEEF]);
    }
}

////////////////////////////////////////////////////////////////////////////////
// Benchmarking harness
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod bench {
    use heapsort;
    use insertsort;
    use introsort;
    use std::mem;
    use std::rand::{weak_rng, Rng};
    use test::{ Bencher };

    type BigSortable = (u64,u64,u64,u64);

    ////////////////////////////////////////////////////////////////////////////
    // Bench helpers
    ////////////////////////////////////////////////////////////////////////////

    fn bench_random_small<F>(b: &mut Bencher, sortfn: F) where F: Fn(&mut [u64]) {
        let mut rng = weak_rng();
        b.iter(|| {
            let mut v = rng.gen_iter::<u64>().take(5).collect::<Vec<u64>>();
            sortfn(v.as_mut_slice());
        });
        b.bytes = 5 * mem::size_of::<u64>() as u64;
    }

    fn bench_random_medium<F>(b: &mut Bencher, sortfn: F) where F: Fn(&mut [u64]) {
        let mut rng = weak_rng();
        b.iter(|| {
            let mut v = rng.gen_iter::<u64>().take(100).collect::<Vec<u64>>();
            sortfn(v.as_mut_slice());
        });
        b.bytes = 100 * mem::size_of::<u64>() as u64;
    }

    fn bench_random_large<F>(b: &mut Bencher, sortfn: F) where F: Fn(&mut [u64]) {
        let mut rng = weak_rng();
        b.iter(|| {
            let mut v = rng.gen_iter::<u64>().take(10000).collect::<Vec<u64>>();
            sortfn(v.as_mut_slice());
        });
        b.bytes = 10000 * mem::size_of::<u64>() as u64;
    }

    fn bench_sorted<F>(b: &mut Bencher, sortfn: F) where F: Fn(&mut [usize]) {
        let mut v = range(0u, 10000).collect::<Vec<_>>();
        b.iter(|| {
            sortfn(v.as_mut_slice());
        });
        b.bytes = (v.len() * mem::size_of_val(&v[0])) as u64;
    }

    fn bench_big_random_small<F>(b: &mut Bencher, sortfn: F) where F: Fn(&mut [BigSortable]) {
        let mut rng = weak_rng();
        b.iter(|| {
            let mut v = rng.gen_iter::<BigSortable>().take(5)
                           .collect::<Vec<BigSortable>>();
            sortfn(v.as_mut_slice());
        });
        b.bytes = 5 * mem::size_of::<BigSortable>() as u64;
    }

    fn bench_big_random_medium<F>(b: &mut Bencher, sortfn: F) where F: Fn(&mut [BigSortable]) {
        let mut rng = weak_rng();
        b.iter(|| {
            let mut v = rng.gen_iter::<BigSortable>().take(100)
                           .collect::<Vec<BigSortable>>();
            sortfn(v.as_mut_slice());
        });
        b.bytes = 100 * mem::size_of::<BigSortable>() as u64;
    }

    fn bench_big_random_large<F>(b: &mut Bencher, sortfn: F) where F: Fn(&mut [BigSortable]) {
        let mut rng = weak_rng();
        b.iter(|| {
            let mut v = rng.gen_iter::<BigSortable>().take(10000)
                           .collect::<Vec<BigSortable>>();
            sortfn(v.as_mut_slice());
        });
        b.bytes = 10000 * mem::size_of::<BigSortable>() as u64;
    }

    fn bench_big_sorted<F>(b: &mut Bencher, sortfn: F) where F: Fn(&mut [BigSortable]) {
        let mut v = range(0, 10000u64).map(|i| (i, i, i, i)).collect::<Vec<_>>();
        b.iter(|| {
            sortfn(v.as_mut_slice());
        });
        b.bytes = (v.len() * mem::size_of_val(&v[0])) as u64;
    }

    ////////////////////////////////////////////////////////////////////////////
    // Introspection sort benchmarking
    ////////////////////////////////////////////////////////////////////////////

    #[bench]
    fn introsort_random_small(b: &mut Bencher) {
        bench_random_small(b, introsort);
    }

    #[bench]
    fn introsort_random_medium(b: &mut Bencher) {
        bench_random_medium(b, introsort);
    }

    #[bench]
    fn introsort_random_large(b: &mut Bencher) {
        bench_random_large(b, introsort);
    }

    #[bench]
    fn introsort_sorted(b: &mut Bencher) {
        bench_sorted(b, introsort);
    }

    #[bench]
    fn introsort_big_random_small(b: &mut Bencher) {
        bench_big_random_small(b, introsort);
    }

    #[bench]
    fn introsort_big_random_medium(b: &mut Bencher) {
        bench_big_random_medium(b, introsort);
    }

    #[bench]
    fn introsort_big_random_large(b: &mut Bencher) {
        bench_big_random_large(b, introsort);
    }

    #[bench]
    fn introsort_big_sorted(b: &mut Bencher) {
        bench_big_sorted(b, introsort);
    }

    ////////////////////////////////////////////////////////////////////////////
    // Insertion sort benchmarking
    ////////////////////////////////////////////////////////////////////////////

    #[bench]
    fn insertsort_random_small(b: &mut Bencher) {
        bench_random_small(b, insertsort);
    }

    #[bench]
    fn insertsort_random_medium(b: &mut Bencher) {
        bench_random_medium(b, insertsort);
    }

    /*
    #[bench]
    fn insertsort_random_large(b: &mut Bencher) {
        bench_random_large(b, insertsort);
    }
    */

    #[bench]
    fn insertsort_sorted(b: &mut Bencher) {
        bench_sorted(b, insertsort);
    }

    #[bench]
    fn insertsort_big_random_small(b: &mut Bencher) {
        bench_big_random_small(b, insertsort);
    }

    #[bench]
    fn insertsort_big_random_medium(b: &mut Bencher) {
        bench_big_random_medium(b, insertsort);
    }

    /*
    #[bench]
    fn insertsort_big_random_large(b: &mut Bencher) {
        bench_big_random_large(b, insertsort);
    }
    */

    #[bench]
    fn insertsort_big_sorted(b: &mut Bencher) {
        bench_big_sorted(b, insertsort);
    }

    ////////////////////////////////////////////////////////////////////////////
    // Heap sort benchmarking
    ////////////////////////////////////////////////////////////////////////////

    #[bench]
    fn heapsort_random_small(b: &mut Bencher) {
        bench_random_small(b, heapsort);
    }

    #[bench]
    fn heapsort_random_medium(b: &mut Bencher) {
        bench_random_medium(b, heapsort);
    }

    #[bench]
    fn heapsort_random_large(b: &mut Bencher) {
        bench_random_large(b, heapsort);
    }

    #[bench]
    fn heapsort_sorted(b: &mut Bencher) {
        bench_sorted(b, heapsort);
    }

    #[bench]
    fn heapsort_big_random_small(b: &mut Bencher) {
        bench_big_random_small(b, heapsort);
    }

    #[bench]
    fn heapsort_big_random_medium(b: &mut Bencher) {
        bench_big_random_medium(b, heapsort);
    }

    #[bench]
    fn heapsort_big_random_large(b: &mut Bencher) {
        bench_big_random_large(b, heapsort);
    }

    #[bench]
    fn heapsort_big_sorted(b: &mut Bencher) {
        bench_big_sorted(b, heapsort);
    }

    ////////////////////////////////////////////////////////////////////////////
    // Merge sort (via std::slice::SliceExt::sort) benchmarking
    ////////////////////////////////////////////////////////////////////////////

    fn mergesort<T: Ord>(v: &mut[T]) { v.sort(); }

    #[bench]
    fn stdsort_random_small(b: &mut Bencher) {
        bench_random_small(b, mergesort);
    }

    #[bench]
    fn stdsort_random_medium(b: &mut Bencher) {
        bench_random_medium(b, mergesort);
    }

    #[bench]
    fn stdsort_random_large(b: &mut Bencher) {
        bench_random_large(b, mergesort);
    }

    #[bench]
    fn stdsort_sorted(b: &mut Bencher) {
        bench_sorted(b, mergesort);
    }

    #[bench]
    fn stdsort_big_random_small(b: &mut Bencher) {
        bench_big_random_small(b, mergesort);
    }

    #[bench]
    fn stdsort_big_random_medium(b: &mut Bencher) {
        bench_big_random_medium(b, mergesort);
    }

    #[bench]
    fn stdsort_big_random_large(b: &mut Bencher) {
        bench_big_random_large(b, mergesort);
    }

    #[bench]
    fn stdsort_big_sorted(b: &mut Bencher) {
        bench_big_sorted(b, mergesort);
    }
}

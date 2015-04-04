// Copyright 2015 Cameron Hart
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::mem;
use std::ptr;

////////////////////////////////////////////////////////////////////////////////
// Insertion sort (based off libstd collections slice version)
////////////////////////////////////////////////////////////////////////////////

fn insertsort_impl<T, F>(ptr: *mut T, len: isize, lt: &F) where F: Fn(&T, &T) -> bool {
    // 1 <= i < len;
    for i in 1..len {
        // j satisfies: 0 <= j <= i;
        let mut j = i;
        unsafe {
            // `i` is in bounds.
            let read_ptr = ptr.offset(i) as *const T;

            // find where to insert, we need to do strict <,
            // rather than <=, to maintain stability.

            // 0 <= j - 1 < len, so .offset(j - 1) is in bounds.
            while j > 0 && lt(&*read_ptr, &*ptr.offset(j - 1)) {
                j -= 1;
            }

            // shift everything to the right, to make space to
            // insert this value.

            // j + 1 could be `len` (for the last `i`), but in
            // that case, `i == j` so we don't copy. The
            // `.offset(j)` is always in bounds.

            if i != j {
                let tmp = ptr::read(read_ptr);
                ptr::copy(ptr.offset(j), ptr.offset(j + 1), (i - j) as usize);
                ptr::copy_nonoverlapping(&tmp, ptr.offset(j), 1);
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

#[inline]
fn lg(n: usize) -> usize {
    mem::size_of::<usize>() * 8 - 1 - n.leading_zeros() as usize
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
    // Threshold at which we stop and let the insertsort finish off
    const THRESHOLD: isize = 32;

    let mut len = ptr_distance(last, ptr);
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
/// let mut v = [5is, 4, 1, 3, 2];
/// sortrs::introsort_by(&mut v, |a, b| a.lt(b));
/// assert!(v == [1, 2, 3, 4, 5]);
///
/// // reverse sorting
/// sortrs::introsort_by(&mut v, |a, b| b.lt(a));
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
/// let mut v = [-5is, 4, 1, -3, 2];
///
/// sortrs::introsort(&mut v);
/// assert!(v == [-5is, -3, 1, 2, 4]);
/// ```
pub fn introsort<T: PartialOrd>(v: &mut[T]) {
    introsort_impl(v, |a, b| a.lt(b))
}


#![cfg(test)]
extern crate test;

use std::fmt::Show;
use std::mem;
use std::num::Int;
use std::ptr;

////////////////////////////////////////////////////////////////////////////////
// Insertion sort (based off libstd collections slice version)
////////////////////////////////////////////////////////////////////////////////

fn insertsort_impl<T, F>(first: *mut T, len: int, lt: &F) where F: Fn(&T, &T) -> bool {
    // 1 <= i < len;
    for i in range(1, len) {
        // j satisfies: 0 <= j <= i;
        let mut j = i;
        unsafe {
            // `i` is in bounds.
            let read_ptr = first.offset(i) as *const T;

            // find where to insert, we need to do strict <,
            // rather than <=, to maintain stability.

            // 0 <= j - 1 < len, so .offset(j - 1) is in bounds.
            while j > 0 &&
                    lt(&*read_ptr, &*first.offset(j - 1)) {
                j -= 1;
            }

            // shift everything to the right, to make space to
            // insert this value.

            // j + 1 could be `len` (for the last `i`), but in
            // that case, `i == j` so we don't copy. The
            // `.offset(j)` is always in bounds.

            if i != j {
                let tmp = ptr::read(read_ptr);
                ptr::copy_memory(first.offset(j + 1),
                                 &*first.offset(j),
                                 (i - j) as uint);
                ptr::copy_nonoverlapping_memory(first.offset(j),
                                                &tmp as *const T,
                                                1);
                mem::forget(tmp);
            }
        }
    }
}

pub fn insertsort_by<T: PartialOrd, F>(v: &mut[T], lt: F) where F: Fn(&T, &T) -> bool {
    insertsort_impl(v.as_mut_ptr(), v.len() as int, &lt);
}

pub fn insertsort<T: PartialOrd>(v: &mut[T]) {
    insertsort_by(v, |a, b| a.lt(b));
}

////////////////////////////////////////////////////////////////////////////////
// Heap sort
////////////////////////////////////////////////////////////////////////////////

fn heapify<T, F>(ptr: *mut T, size: int, lt: &F) where F: Fn(&T, &T) -> bool {
    let mut start = (size - 2) / 2;
    let end = size - 1;
    while start >= 0 {
        shift_down(ptr, start, end, lt);
        start = start - 1;
    }
}

fn shift_down<T, F>(ptr: *mut T, start: int, end: int, lt: &F) where F: Fn(&T, &T) -> bool {
    let mut root = start;
    let mut next_child = root * 2 + 1;
    while next_child <= end {
        let child = next_child;
        let mut swap = root;
        unsafe {
            if lt(&*ptr.offset(swap), &*ptr.offset(child)) {
                swap = child;
            }
            next_child = child + 1;
            if next_child <= end && lt(&*ptr.offset(swap), &*ptr.offset(next_child)) {
                swap = next_child;
            }
            if swap == root {
                return;
            }
            ptr::swap(ptr.offset(root), ptr.offset(swap));
        }
        root = swap;
        next_child = root * 2 + 1;
    }
}

fn heapsort_impl<T, F>(ptr: *mut T, size: int, lt: &F) where F: Fn(&T, &T) -> bool {
	heapify(ptr, size, lt);
        let mut end = size - 1;
        while end > 0 {
            unsafe {
                ptr::swap(ptr.offset(end), ptr);
            }
            end = end - 1;
            shift_down(ptr, 0, end, lt);
	}
}

pub fn heapsort_by<T: PartialOrd, F>(v: &mut[T], lt: F) where F: Fn(&T, &T) -> bool {
    let len = v.len() as int;
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

const THRESHOLD: int = 16;

fn lg(n: uint) -> uint {
	mem::size_of::<uint>() * 8 - 1 - n.leading_zeros()
}

fn ptr_diff<T>(a: *const T, b: *const T) -> int {
    let ai = a as uint;
    let bi = b as uint;
    let d =  if ai < bi { bi - ai } else { ai - bi } / mem::size_of::<T>();
    d as int
}

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

fn partition<T, F>(mut first: *mut T, mut last: *mut T, pivot: *mut T, lt: &F) -> *mut T
        where F: Fn(&T, &T) -> bool {
    unsafe {
        loop {
            while lt(&*first, &*pivot) {
                first = first.offset(1);
            }
            last = last.offset(-1);
            while lt(&*pivot, &*last) {
                last = last.offset(-1);
            }
            if !((first as uint) < (last as uint)) {
                return first;
            }
            ptr::swap(first, last);
            first = first.offset(1);
        }
    }
}

fn partition_pivot<T, F>(first: *mut T, last: *mut T, lt: &F) -> *mut T
        where F: Fn(&T, &T) -> bool {
    unsafe {
        let len = ptr_diff(first as *const T, last as *const T);
        let pivot = median_3(first.offset(1), first.offset(len / 2), first.offset(len - 1), lt);
        ptr::swap(first, pivot);
        partition(first.offset(1), last, first, lt)
    }
}

fn introsort_impl<T, F>(first: *mut T, mut last: *mut T, mut depth_limit: uint, lt: &F) where F: Fn(&T, &T) -> bool {
    while ptr_diff(last as *const T, first as *const T) > THRESHOLD {
        if depth_limit == 0 {
            insertsort_impl(first, (last as uint - first as uint) as int, lt);
            return;
        }
        depth_limit -= 1;
        let pivot = partition_pivot(first, last, lt);
        introsort_impl(pivot, last, depth_limit, lt);
        last = pivot;
    }
}

pub fn introsort_by<T: PartialOrd + Show, F>(v: &mut[T], lt: F) where F: Fn(&T, &T) -> bool {
    let size = v.len() as int;
    if size > 0 {
        let ptr = v.as_mut_ptr();
        unsafe {
            introsort_impl(ptr, ptr.offset(size), 2 * lg(size as uint), &lt);
        }
        insertsort_impl(ptr, size, &lt);
    }
}

pub fn introsort<T: PartialOrd + Show>(v: &mut[T]) {
    introsort_by(v, |a, b| a.lt(b))
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
                let mut v = thread_rng().gen_iter::<uint>().take(len)
                                      .collect::<Vec<uint>>();
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
        let mut v: [uint; 0] = [];
        insertsort(v.as_mut_slice());

        let mut v = [0xDEADBEEFu];
        insertsort(v.as_mut_slice());
        assert!(v == [0xDEADBEEF]);
    }

    #[test]
    fn test_heapsort() {
        for len in range(4u, 25) {
            for _ in range(0i, 100) {
                let mut v = thread_rng().gen_iter::<uint>().take(len)
                                      .collect::<Vec<uint>>();
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
        let mut v: [uint; 0] = [];
        heapsort(v.as_mut_slice());

        let mut v = [0xDEADBEEFu];
        heapsort(v.as_mut_slice());
        assert!(v == [0xDEADBEEF]);
    }

    #[test]
    fn test_introsort() {
        for len in range(4u, 25) {
            for _ in range(0i, 100) {
                let mut v = thread_rng().gen_iter::<uint>().take(len)
                                      .collect::<Vec<uint>>();
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
        let mut v: [uint; 0] = [];
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

    fn bench_sorted<F>(b: &mut Bencher, sortfn: F) where F: Fn(&mut [uint]) {
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

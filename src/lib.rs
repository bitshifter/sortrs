use std::mem;
use std::ptr;

fn insertion_sort_impl<T, F>(first: *mut T, len: int, lt: &F) where F: Fn(&T, &T) -> bool {
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

pub fn insertion_sort_by<T: PartialOrd, F>(v: &mut[T], lt: F) where F: Fn(&T, &T) -> bool {
    insertion_sort_impl(v.as_mut_ptr(), v.len() as int, &lt);
}

pub fn insertion_sort<T: PartialOrd>(v: &mut[T]) {
    insertion_sort_by(v, |a, b| a.lt(b));
}

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

fn heap_sort_impl<T, F>(ptr: *mut T, size: int, lt: &F) where F: Fn(&T, &T) -> bool {
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

pub fn heap_sort_by<T: PartialOrd, F>(v: &mut[T], lt: F) where F: Fn(&T, &T) -> bool {
    let len = v.len() as int;
    if len > 0 {
		let ptr = v.as_mut_ptr();
		heap_sort_impl(ptr, len, &lt);
    }
}

pub fn heap_sort<T: PartialOrd>(v: &mut[T]) {
	heap_sort_by(v, |a, b| a.lt(b));
}

#[cfg(test)]
mod tests {
    use std::rand::{Rng, thread_rng};
	use insertion_sort;
	use insertion_sort_by;
	use heap_sort;
	use heap_sort_by;
    #[test]
    fn test_insertion_sort() {
        for len in range(4u, 25) {
            for _ in range(0i, 100) {
                let mut v = thread_rng().gen_iter::<uint>().take(len)
                                      .collect::<Vec<uint>>();
                let mut v1 = v.clone();

				println!("{}", v);
                insertion_sort(v.as_mut_slice());
				println!("{}", v);
                assert!(v.as_slice().windows(2).all(|w| w[0] <= w[1]));

                insertion_sort_by(v1.as_mut_slice(), |a, b| a.lt(b));
                assert!(v1.as_slice().windows(2).all(|w| w[0] <= w[1]));

                insertion_sort_by(v1.as_mut_slice(), |a, b| b.lt(a));
                assert!(v1.as_slice().windows(2).all(|w| w[0] >= w[1]));
            }
        }

        // shouldn't panic
        let mut v: [uint; 0] = [];
        insertion_sort(v.as_mut_slice());

        let mut v = [0xDEADBEEFu];
        insertion_sort(v.as_mut_slice());
        assert!(v == [0xDEADBEEF]);
    }

    #[test]
    fn test_heap_sort() {
        for len in range(4u, 25) {
            for _ in range(0i, 100) {
                let mut v = thread_rng().gen_iter::<uint>().take(len)
                                      .collect::<Vec<uint>>();
                let mut v1 = v.clone();

				println!("{}", v);
                heap_sort(v.as_mut_slice());
				println!("{}", v);
                assert!(v.as_slice().windows(2).all(|w| w[0] <= w[1]));

                heap_sort_by(v1.as_mut_slice(), |a, b| a.lt(b));
                assert!(v1.as_slice().windows(2).all(|w| w[0] <= w[1]));

                heap_sort_by(v1.as_mut_slice(), |a, b| b.lt(a));
                assert!(v1.as_slice().windows(2).all(|w| w[0] >= w[1]));
            }
        }

        // shouldn't panic
        let mut v: [uint; 0] = [];
        heap_sort(v.as_mut_slice());

        let mut v = [0xDEADBEEFu];
        heap_sort(v.as_mut_slice());
        assert!(v == [0xDEADBEEF]);
    }
}


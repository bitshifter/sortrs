extern crate rand;
extern crate sortrs;

use rand::{Rng, thread_rng};
use sortrs::{insertsort, insertsort_by, heapsort, heapsort_by, introsort, introsort_by};

#[test]
fn test_insertsort() {
    for len in 4usize..25 {
        for _ in 0..100 {
            let mut v = thread_rng()
                .gen_iter::<usize>()
                .take(len)
                .collect::<Vec<usize>>();
            let mut v1 = v.clone();

            insertsort(&mut v);
            assert!(v.windows(2).all(|w| w[0] <= w[1]));

            insertsort_by(&mut v1, |a, b| a.lt(b));
            assert!(v1.windows(2).all(|w| w[0] <= w[1]));

            insertsort_by(&mut v1[..], |a, b| b.lt(a));
            assert!(v1.windows(2).all(|w| w[0] >= w[1]));
        }
    }

    // shouldn't panic on empty slice
    let mut v: [usize; 0] = [];
    insertsort(&mut v);

    let mut v = [0xDEADBEEFu32];
    insertsort(&mut v);
    assert!(v == [0xDEADBEEF]);
}

#[test]
fn test_heapsort() {
    for len in 4usize..25 {
        for _ in 0..100 {
            let mut v = thread_rng()
                .gen_iter::<usize>()
                .take(len)
                .collect::<Vec<usize>>();
            let mut v1 = v.clone();

            heapsort(&mut v);
            assert!(v.windows(2).all(|w| w[0] <= w[1]));

            heapsort_by(&mut v1, |a, b| a.lt(b));
            assert!(v1.windows(2).all(|w| w[0] <= w[1]));

            heapsort_by(&mut v1, |a, b| b.lt(a));
            assert!(v1.windows(2).all(|w| w[0] >= w[1]));
        }
    }

    // shouldn't panic on empty slice
    let mut v: [usize; 0] = [];
    heapsort(&mut v);

    let mut v = [0xDEADBEEFu32];
    heapsort(&mut v);
    assert!(v == [0xDEADBEEF]);
}

#[test]
fn test_introsort() {
    for len in 4usize..25 {
        for _ in 0..100 {
            let mut v = thread_rng()
                .gen_iter::<usize>()
                .take(len)
                .collect::<Vec<usize>>();
            let mut v1 = v.clone();

            introsort(&mut v);
            assert!(v.windows(2).all(|w| w[0] <= w[1]));

            introsort_by(&mut v1, |a, b| a.lt(b));
            assert!(v1.windows(2).all(|w| w[0] <= w[1]));

            introsort_by(&mut v1, |a, b| b.lt(a));
            assert!(v1.windows(2).all(|w| w[0] >= w[1]));
        }
    }

    // shouldn't panic on empty slice
    let mut v: [usize; 0] = [];
    introsort(&mut v);

    let mut v = [0xDEADBEEFu32];
    introsort(&mut v);
    assert!(v == [0xDEADBEEF]);
}

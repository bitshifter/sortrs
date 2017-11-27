#![feature(test)]

extern crate rand;
extern crate sortrs;
extern crate test;

use std::mem;
use rand::{weak_rng, Rng};
use sortrs::{insertsort, heapsort, introsort};
use test::Bencher;

type BigSortable = (u64, u64, u64, u64);

////////////////////////////////////////////////////////////////////////////
// Bench helpers
////////////////////////////////////////////////////////////////////////////

fn bench_random_small<F>(b: &mut Bencher, sortfn: F)
where
    F: Fn(&mut [u64]),
{
    let mut rng = weak_rng();
    b.iter(|| {
        let mut v = rng.gen_iter::<u64>().take(5).collect::<Vec<u64>>();
        sortfn(&mut v);
    });
    b.bytes = 5 * mem::size_of::<u64>() as u64;
}

fn bench_random_medium<F>(b: &mut Bencher, sortfn: F)
where
    F: Fn(&mut [u64]),
{
    let mut rng = weak_rng();
    b.iter(|| {
        let mut v = rng.gen_iter::<u64>().take(100).collect::<Vec<u64>>();
        sortfn(&mut v);
    });
    b.bytes = 100 * mem::size_of::<u64>() as u64;
}

fn bench_random_large<F>(b: &mut Bencher, sortfn: F)
where
    F: Fn(&mut [u64]),
{
    let mut rng = weak_rng();
    b.iter(|| {
        let mut v = rng.gen_iter::<u64>().take(10000).collect::<Vec<u64>>();
        sortfn(&mut v);
    });
    b.bytes = 10000 * mem::size_of::<u64>() as u64;
}

fn bench_sorted<F>(b: &mut Bencher, sortfn: F)
where
    F: Fn(&mut [u64]),
{
    let mut v = (0u64..10000).collect::<Vec<_>>();
    b.iter(|| { sortfn(&mut v); });
    b.bytes = (v.len() * mem::size_of_val(&v[0])) as u64;
}

fn bench_big_random_small<F>(b: &mut Bencher, sortfn: F)
where
    F: Fn(&mut [BigSortable]),
{
    let mut rng = weak_rng();
    b.iter(|| {
        let mut v = rng.gen_iter::<BigSortable>()
            .take(5)
            .collect::<Vec<BigSortable>>();
        sortfn(&mut v);
    });
    b.bytes = 5 * mem::size_of::<BigSortable>() as u64;
}

fn bench_big_random_medium<F>(b: &mut Bencher, sortfn: F)
where
    F: Fn(&mut [BigSortable]),
{
    let mut rng = weak_rng();
    b.iter(|| {
        let mut v = rng.gen_iter::<BigSortable>()
            .take(100)
            .collect::<Vec<BigSortable>>();
        sortfn(&mut v);
    });
    b.bytes = 100 * mem::size_of::<BigSortable>() as u64;
}

fn bench_big_random_large<F>(b: &mut Bencher, sortfn: F)
where
    F: Fn(&mut [BigSortable]),
{
    let mut rng = weak_rng();
    b.iter(|| {
        let mut v = rng.gen_iter::<BigSortable>()
            .take(10000)
            .collect::<Vec<BigSortable>>();
        sortfn(&mut v);
    });
    b.bytes = 10000 * mem::size_of::<BigSortable>() as u64;
}

fn bench_big_sorted<F>(b: &mut Bencher, sortfn: F)
where
    F: Fn(&mut [BigSortable]),
{
    let mut v = (0..10000u64).map(|i| (i, i, i, i)).collect::<Vec<_>>();
    b.iter(|| { sortfn(&mut v); });
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

fn mergesort<T: Ord>(v: &mut [T]) {
    v.sort();
}

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

extern crate criterion;

use std::collections::{LinkedList, VecDeque};
use std::rc::Rc;
use criterion::{black_box, Criterion, criterion_group, criterion_main};
use int_linked_list::linkedlist::int_linked_list::IntList;

fn benchmark_linked_list(c: &mut Criterion) {
    let mut group = c.benchmark_group("LinkedList");
    group.bench_function("push_back", |b| {
        b.iter(|| {
            let mut list = LinkedList::new();
            for _ in 0..10 {
                list.push_back(black_box(0));
            }
        })
    });
    group.bench_function("iter", |b| {
        let list = LinkedList::from_iter((0..10).map(black_box));
        b.iter(|| {
            for _ in &list {}
        })
    });
    group.finish();
}

fn benchmark_vec_deque(c: &mut Criterion) {
    let mut group = c.benchmark_group("VecDeque");
    group.bench_function("push_back", |b| {
        b.iter(|| {
            let mut deque = VecDeque::new();
            for _ in 0..10 {
                deque.push_back(black_box(0));
            }
        })
    });
    group.bench_function("iter", |b| {
        let deque = VecDeque::from_iter((0..10).map(black_box));
        b.iter(|| {
            for _ in &deque {}
        })
    });
    group.finish();
}

fn bench_int_list(c: &mut Criterion) {
    let mut group = c.benchmark_group("IntList");
    // group.bench_function("push_back", |b| {
    //     b.iter(|| unsafe {
    //         let mut list = IntList::new();
    //         for _ in 0..10 {
    //             list.add_last(black_box(0));
    //         }
    //     })
    // });
    group.bench_function("iter", |b| unsafe {
        let list = IntList::from_iter((0..10).map(black_box));
        let boz = Rc::new(list);
        b.iter(|| {
            for _ in *boz.clone() {}
        })
    });
    group.finish();
}

criterion_group!(benches, /*benchmark_linked_list, benchmark_vec_deque,*/ bench_int_list);
criterion_main!(benches);
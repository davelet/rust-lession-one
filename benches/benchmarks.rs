use std::collections::{LinkedList, VecDeque};
use criterion::{black_box, Criterion, criterion_group, criterion_main};

fn benchmark_linked_list(c: &mut Criterion) {
    let mut group = c.benchmark_group("LinkedList");
    group.bench_function("push_back", |b| {
        b.iter(|| {
            let mut list = LinkedList::new();
            for _ in 0..10_0 {
                list.push_back(black_box(0));
            }
        })
    });
    group.bench_function("iter", |b| {
        let list = LinkedList::from_iter((0..10_0).map(black_box));
        b.iter(|| {
            for _ in &list { }
        })
    });
    group.finish();
}

fn benchmark_vec_deque(c: &mut Criterion) {
    let mut group = c.benchmark_group("VecDeque");
    group.bench_function("push_back", |b| {
        b.iter(|| {
            let mut deque = VecDeque::new();
            for _ in 0..10_0 {
                deque.push_back(black_box(0));
            }
        })
    });
    group.bench_function("iter", |b| {
        let deque = VecDeque::from_iter((0..10_0).map(black_box));
        b.iter(|| {
            for _ in &deque { }
        })
    });
    group.finish();
}

// fn b_int_list(c: &mut Criterion) {
//     let mut group = c.benchmark_group("IntList");
//     group.bench_function("push_back", |b| {
//         b.iter(|| unsafe {
//             let mut list = IntList::new();
//             for _ in 0..10_0 {
//                 list.add_last(black_box(0));
//             }
//         })
//     });
//     group.bench_function("iter", |b| unsafe {
//         let mut list = IntList::from_iter((0..10_0).map(black_box));
//         b.iter(|| {
//             let list = list.clone();
//             for _ in list { }
//         })
//     });
//     group.finish();
// }

criterion_group!(benches, benchmark_linked_list, benchmark_vec_deque);
criterion_main!(benches);
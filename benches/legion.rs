include!("./common/includes.rs");

use legion::prelude::*;

fn bench_create_transforms(c: &mut Criterion) {
    let prepare = || {
        let universe = Universe::new(None);
        universe.create_world()
    };
    let run1000 = |mut world: World| {
        world.insert_from((), (0..=1000).map(|_| (Transform::default(),)));
    };
    let run10000 = |mut world: World| {
        world.insert_from((), (0..=10000).map(|_| (Transform::default(),)));
    };

    c.bench_function("legion create_transforms 1000", move |b| {
        b.iter_batched(prepare, run1000, BatchSize::SmallInput);
    });
    c.bench_function("legion create_transforms 10000", move |b| {
        b.iter_batched(prepare, run10000, BatchSize::SmallInput);
    });
}

criterion_group!(benches, bench_create_transforms);
criterion_main!(benches);

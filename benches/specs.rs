include!("./common/includes.rs");

use specs::prelude::*;

fn create_transforms(c: &mut Criterion) {
    let prepare = || {
        let mut world = World::new();
        world.register::<Transform>();
        world
    };
    let run1000 = |mut world: World| {
        for _ in 0..=1000 { world.create_entity().with(Transform::default()).build(); }
    };
    let run10000 = |mut world: World| {
        for _ in 0..=10000 { world.create_entity().with(Transform::default()).build(); }
    };

    c.bench_function("specs create_transforms 1000", move |b| {
        b.iter_batched(prepare, run1000, BatchSize::SmallInput);
    });
    c.bench_function("specs create_transforms 10000", move |b| {
        b.iter_batched(prepare, run10000, BatchSize::SmallInput);
    });
}

criterion_group!(benches, create_transforms);
criterion_main!(benches);
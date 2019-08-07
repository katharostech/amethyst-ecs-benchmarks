include!("./common/includes.rs");

use amethyst_core::ecs::prelude::*;

fn create_transforms(c: &mut Criterion) {
    let prepare = || {
        let mut world = World::new();
        world.register::<Transform>();
        world
    };
    let run1000 = |mut world: World| {
        for _ in 0..=1000 {
            world.create_entity().with(Transform::default()).build();
        }
    };
    let run10000 = |mut world: World| {
        for _ in 0..=10000 {
            world.create_entity().with(Transform::default()).build();
        }
    };

    c.bench_function("specs create_transforms 1000", move |b| {
        b.iter_batched(prepare, run1000, BatchSize::SmallInput);
    });
    c.bench_function("specs create_transforms 10000", move |b| {
        b.iter_batched(prepare, run10000, BatchSize::SmallInput);
    });
}

mod moving_objects {
    extern crate criterion;
    use criterion::{BatchSize, Criterion};

    use amethyst_core::{
        ecs::{prelude::*, },
        transform::*,
        SystemDesc,
    };
    use specs_hierarchy::HierarchySystem;

    pub fn moving_objects(c: &mut Criterion) {
        let setup = || {
            // Instantiate World
            let mut world = World::new();

            {
                // Create entities
                let ents: Vec<Entity> = world.create_iter().take(1000).collect();
                let mut transforms = world.write_storage::<Transform>();

                for (i, e) in ents.iter().enumerate() {
                    transforms.insert(*e, Transform::default()).unwrap();
                }
            }

            // Build Dispatcher
            let mut builder = DispatcherBuilder::new();
            builder.add(
                HierarchySystem::<Parent>::new(&mut world),
                "hierarchy_system",
                &[],
            );
            builder.add(
                TransformSystemDesc::default().build(&mut world),
                "transform_system",
                &["hierarchy_system"],
            );
            let mut dispatcher = builder.build();
            dispatcher.setup(&mut world);

            // Return world and dispatcher
            (world, dispatcher)
        };

        c.bench_function("specs transform_test", move |b| {
            b.iter_batched(
                setup,
                |(mut world, mut dispatcher)| dispatcher.dispatch(&mut world),
                BatchSize::SmallInput,
            );
        });
    }
}

use moving_objects::moving_objects;

criterion_group!(benches, create_transforms, moving_objects);
criterion_main!(benches);

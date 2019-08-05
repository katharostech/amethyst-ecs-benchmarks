#![feature(test)]
extern crate test;
use test::Bencher;
use amethyst_ecs_benchmarks::components::*;

use specs::prelude::*;

#[bench]
pub fn iter_transforms(b: &mut Bencher) {
    b.iter(|| {
        let mut world = World::new();
        world.register::<Transform>();

        for _ in 0..=100 {
            world.create_entity().with(Transform::default()).build();
        }
    })
}

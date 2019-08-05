#![feature(test)]
extern crate test;
use test::Bencher;
use amethyst_ecs_benchmarks::components::*;

use legion::prelude::*;

#[bench]
pub fn iter_transforms(b: &mut Bencher) {
    b.iter(|| {
        // create world
        let universe = Universe::new(None);
        let mut world = universe.create_world();

        world.insert_from((), (0..=100).map(|_| (Transform::default(),)));
    })
}

#![feature(test)]
extern crate test;
use amethyst_ecs_benchmarks::components::*;
use test::Bencher;

use legion::prelude::*;

#[bench]
pub fn iter_transforms(b: &mut Bencher) {
    b.iter(|| {
        let universe = Universe::new(None);
        let mut world = universe.create_world();

        world.insert_from((), (0..=100).map(|_| (Transform::default(),)));
    })
}

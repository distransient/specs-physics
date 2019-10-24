extern crate log;
extern crate simple_logger;

use specs::{Builder, DispatcherBuilder, World, WorldExt};
use specs_physics::{
    ncollide::shape::{Ball, ShapeHandle},
    nphysics::{
        math::Vector,
        object::{ColliderDesc, RigidBodyDesc},
    },
    systems::PhysicsBundle,
    EntityBuilderExt, SimplePosition,
};

fn main() {
    // initialise the logger for system logs
    simple_logger::init().unwrap();

    // initialise the Specs world; this will contain our Resources and Entities
    let mut world = World::new();

    // create the dispatcher containing all relevant Systems; alternatively to using
    // the convenience function you can add all required Systems by hand
    let mut dispatcher = DispatcherBuilder::new();
    PhysicsBundle::<f32, SimplePosition<f32>>::default().register(&mut world, &mut dispatcher);
    let mut dispatcher = dispatcher.build();
    dispatcher.setup(&mut world);

    // Add dynamic cuboid (2.0, 2.0, 1.0) with velocity of 1*x at (1.0, 1.0, 1.0)
    // Add static cuboid (2.0, 2.0, 1.0) at (3.0, 1.0, 1.0)

    // execute the dispatcher
    dispatcher.dispatch(&world);

    // fetch the translation component for the Entity with the dynamic body; the
    // position should still be approx the same
    let pos_storage = world.read_storage::<SimplePosition<f32>>();
    let pos = pos_storage.get(entity).unwrap();

    info!("updated position: {}", pos.0.translation);
}

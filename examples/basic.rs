extern crate log;
extern crate simple_logger;

use specs::{Builder, World, WorldExt};
use specs_physics::{
    ncollide::shape::{Ball, ShapeHandle},
    nphysics::{
        math::Vector,
        object::{ColliderDesc, RigidBodyDesc},
    },
    systems::physics_dispatcher,
    EntityBuilderExt, SimplePosition,
};

fn main() {
    // initialise the logger for system logs
    simple_logger::init().unwrap();

    // initialise the Specs world; this will contain our Resources and Entities
    let mut world = World::new();

    // create the dispatcher containing all relevant Systems; alternatively to using
    // the convenience function you can add all required Systems by hand
    let mut dispatcher = physics_dispatcher::<f32, SimplePosition<f32>>();
    dispatcher.setup(&mut world);

    let body_desc = RigidBodyDesc::<f32>::new().translation(Vector::x() * 2.0);
    let shape = ShapeHandle::<f32>::new(Ball::new(1.6));
    let collider_desc = ColliderDesc::new(shape);

    // create an Entity containing the required Components
    world
        .create_entity()
        .with(SimplePosition::<f32>::default())
        .with_body(body_desc.build())
        .with_collider(&collider_desc)
        .build();

    // execute the dispatcher
    dispatcher.dispatch(&world);
}

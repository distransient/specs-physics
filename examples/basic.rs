extern crate log;
extern crate simple_logger;

use specs::{Builder, DispatcherBuilder, Join, ReadStorage, System, World, WorldExt, WriteStorage};
use specs_physics::{
    ncollide::shape::{Ball, ShapeHandle},
    nphysics::{
        algebra::ForceType,
        math::{Force, Vector},
        object::{ColliderDesc, RigidBodyDesc},
    },
    systems::PhysicsBundle,
    BodyComponent, EntityBuilderExt, SimplePosition,
};

fn main() {
    // Initialise the Specs world
    // This will contain our Resources and Entities
    let mut world = World::new();

    // Create the dispatcher with our systems on it
    let mut dispatcher_builder =
        DispatcherBuilder::new().with(MyPhysicsSystem, "my_physics_system", &[]);

    // Attach the specs-physics systems to the dispatcher,
    // with our pre-physics-stepping systems as a dependency
    PhysicsBundle::<f32, SimplePosition<f32>>::default()
        .with_deps(&["my_physics_system"])
        .register(&mut world, &mut dispatcher_builder);

    // Build our dispatcher for use in the application loop
    let mut dispatcher = dispatcher_builder
        .with(
            MyRenderingSystem,
            "my_rendering_system",
            &["physics_pose_system"],
        )
        .build();
    dispatcher.setup(&mut world);

    // Build our physics data
    let body = RigidBodyDesc::new().translation(Vector::x() * 2.0).build();
    let shape = ShapeHandle::new(Ball::new(1.6));
    let collider_desc = ColliderDesc::new(shape);

    // Create an entity and attach that data to it
    world
        .create_entity()
        .with(SimplePosition::<f32>::default())
        .with_body::<f32, _>(body)
        .with_collider::<f32>(&collider_desc)
        .build();

    // Execute the dispatcher like this in your application loop
    for _ in 0..200 {
        dispatcher.dispatch(&world);
    }
}

struct MyPhysicsSystem;

impl<'s> System<'s> for MyPhysicsSystem {
    type SystemData = WriteStorage<'s, BodyComponent<f32>>;

    fn run(&mut self, mut bodies: Self::SystemData) {
        for body in (&mut bodies,).join() {
            // Operate on our bodies.
            body.0
                .apply_force(0, &Force::linear(Vector::x()), ForceType::Force, true)
        }
    }
}

struct MyRenderingSystem;

impl<'s> System<'s> for MyRenderingSystem {
    type SystemData = ReadStorage<'s, SimplePosition<f32>>;

    fn run(&mut self, positions: Self::SystemData) {
        for pos in (&positions,).join() {
            println!("(Technically) Rendering {:?}", pos);
        }
    }
}

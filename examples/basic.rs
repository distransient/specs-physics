extern crate log;
extern crate simple_logger;

use specs::{
    Builder, DispatcherBuilder, Join, ReadExpect, ReadStorage, System, World, WorldExt,
    WriteStorage,
};
use specs_physics::{
    ncollide::shape::{Ball, Cuboid, ShapeHandle},
    nphysics::{
        math::Vector,
        object::{ColliderDesc, Ground, RigidBody, RigidBodyDesc},
    },
    systems::PhysicsBundle,
    world::MechanicalWorldRes,
    BodyComponent, EntityBuilderExt, SimplePosition,
};

fn main() {
    // Initialise the Specs world
    // This will contain our Resources and Entities
    let mut world = World::new();

    /*
     * Specs Dispatcher.
     */
    // Create the dispatcher with our systems on it
    let mut dispatcher_builder =
        DispatcherBuilder::new().with(MyPhysicsSystem, "my_physics_system", &[]);

    // Attach the specs-physics systems to the dispatcher,
    // with our pre-physics-stepping systems as a dependency
    PhysicsBundle::<f32, SimplePosition<f32>>::new(Vector::y() * -9.81)
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

    // Sets up all resources for our dispatcher.
    dispatcher.setup(&mut world);

    /*
     * Physics Ground.
     */
    let ground_thickness = 0.2;

    // Create collider description for ground
    let ground_shape = ShapeHandle::new(Cuboid::new(Vector::new(3.0, ground_thickness, 3.0)));
    let ground_collider =
        ColliderDesc::new(ground_shape).translation(Vector::y() * -ground_thickness);

    // Create ground body in our world and attach our ground collider
    world
        .create_entity()
        .with(SimplePosition::<f32>::default())
        .with_body::<f32, _>(Ground::new())
        .with_collider::<f32>(&ground_collider)
        .build();

    /*
     * Physics Balls.
     */
    // Some values used to build the balls
    let ball_amount: usize = 8;
    let ball_radius = 0.1;
    let ball_shift = (ball_radius + ColliderDesc::<f32>::default_margin()) * 2.0 + 0.002;
    let ball_center = (
        ball_shift * (ball_amount as f32) / 2.0,
        ball_shift / 2.0,
        ball_shift * (ball_amount as f32) / 2.0,
    );
    let ball_height = 3.0;

    // Create collider description we can reuse per ball
    let ball_collider = ColliderDesc::new(ShapeHandle::new(Ball::new(ball_radius))).density(1.0);

    // Create ball rigidbodies and add them to our world
    for i in 0..ball_amount {
        for j in 0..ball_amount {
            for k in 0..ball_amount {
                world
                    .create_entity()
                    .with(SimplePosition::<f32>::default())
                    .with_body(
                        RigidBodyDesc::new()
                            // Offset each body to build a cube of balls
                            .translation(Vector::new(
                                i as f32 * ball_shift - ball_center.0,
                                j as f32 * ball_shift + ball_center.1 + ball_height,
                                k as f32 * ball_shift - ball_center.2,
                            ))
                            .build(),
                    )
                    .with_collider(&ball_collider)
                    .build();
            }
        }
    }

    /*
     * Running
     */
    // Execute the dispatcher in your application loop
    dispatcher.dispatch(&world);
}

struct MyPhysicsSystem;

impl<'s> System<'s> for MyPhysicsSystem {
    type SystemData = WriteStorage<'s, BodyComponent<f32>>;

    fn run(&mut self, mut bodies: Self::SystemData) {
        for (body,) in (&mut bodies,).join() {
            if let Some(_rigid_body) = body.downcast_mut::<RigidBody<f32>>() {
                // Operate on our bodies.
            }
        }
    }
}

struct MyRenderingSystem;

impl<'s> System<'s> for MyRenderingSystem {
    type SystemData = (
        ReadStorage<'s, SimplePosition<f32>>,
        ReadExpect<'s, MechanicalWorldRes<f32>>,
    );

    fn run(&mut self, (positions, mechanical_world): Self::SystemData) {
        for pos in (&positions,).join() {
            println!(
                "Body Position (X: {:?}, Y: {:?}, Z: {:?}) @ {:?}s",
                pos.0.translation.vector.x,
                pos.0.translation.vector.y,
                pos.0.translation.vector.z,
                mechanical_world.integration_parameters.t,
            );
        }
    }
}

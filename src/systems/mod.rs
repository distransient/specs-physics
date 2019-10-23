use crate::{nalgebra::RealField, position::Position};
use specs::{Dispatcher, DispatcherBuilder};

mod batch;
mod pose;
mod stepper;

pub use self::{batch::PhysicsBatchSystem, pose::PhysicsPoseSystem, stepper::PhysicsStepperSystem};

/// Convenience function for configuring and building a `Dispatcher` with all
/// required physics related `System`s.
///
/// # Examples
/// ```rust
/// use specs_physics::SimplePosition;
/// let dispatcher = specs_physics::physics_dispatcher::<f32, SimplePosition<f32>>();
/// ```
pub fn physics_dispatcher<'a, 'b, N: RealField, P: Position<N>>() -> Dispatcher<'a, 'b> {
    let mut dispatcher_builder = DispatcherBuilder::new();
    register_physics_systems::<N, P>(&mut dispatcher_builder);

    dispatcher_builder.build()
}

/// Convenience function for registering all required physics related `System`s
/// to the given `DispatcherBuilder`. This also serves as a blueprint on how
/// to properly set up the `System`s and have them depend on each other.
pub fn register_physics_systems<N: RealField, P: Position<N>>(
    dispatcher_builder: &mut DispatcherBuilder,
) {
    // add PhysicsStepperSystem after all other Systems that write data to the
    // nphysics World and has to depend on them; this System is used to progress the
    // nphysics World for all existing objects
    dispatcher_builder.add(
        PhysicsStepperSystem::<N>::default(),
        "physics_stepper_system",
        &[],
    );

    // add SyncBodiesFromPhysicsSystem last as it handles the
    // synchronisation between nphysics World bodies and the Position
    // components; this depends on the PhysicsStepperSystem
    dispatcher_builder.add(
        PhysicsPoseSystem::<N, P>::default(),
        "physics_pose_system",
        &["physics_stepper_system"],
    );
}

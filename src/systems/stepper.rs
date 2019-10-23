use crate::{
    nalgebra::{convert as na_convert, RealField},
    stepper::StepperRes,
    world::{
        BodySet, ColliderSet, ForceGeneratorSetRes, GeometricalWorldRes, JointConstraintSetRes,
        MechanicalWorldRes,
    },
};

use specs::{System, Write, WriteExpect};
use std::marker::PhantomData;

/// This system steps the physics world once when called.
/// To ensure the visual motion of the simulation matches the speeds within the
/// simulation, you will want to
pub struct PhysicsStepperSystem<N: RealField>(PhantomData<N>);

impl<'s, N: RealField> System<'s> for PhysicsStepperSystem<N> {
    type SystemData = (
        WriteExpect<'s, MechanicalWorldRes<N>>,
        WriteExpect<'s, GeometricalWorldRes<N>>,
        BodySet<'s, N>,
        ColliderSet<'s, N>,
        WriteExpect<'s, JointConstraintSetRes<N>>,
        WriteExpect<'s, ForceGeneratorSetRes<N>>,
        Option<Write<'s, StepperRes>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut mechanical_world,
            mut geometrical_world,
            mut body_set,
            mut collider_set,
            mut joint_constraint_set,
            mut force_generator_set,
            batch_time,
        ) = data;

        // If we've added a batch time step resource to the world, check if we need to
        // update our timestep from that resource.
        if let Some(mut time) = batch_time {
            if time.time_step_dirty() {
                mechanical_world.set_timestep(na_convert(time.time_step().as_secs_f64()));
                time.time_step_clean();
            }
        }

        mechanical_world.step(
            &mut *geometrical_world,
            &mut body_set,
            &mut collider_set,
            &mut *joint_constraint_set,
            &mut *force_generator_set,
        );
    }
}

impl<N: RealField> Default for PhysicsStepperSystem<N> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
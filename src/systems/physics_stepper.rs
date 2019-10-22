use crate::{
    nalgebra::RealField,
    world::{
        BodySet, ColliderSet, ForceGeneratorSetRes, GeometricalWorldRes, JointConstraintSetRes,
        MechanicalWorldRes,
    },
};

use specs::{System, WriteExpect};
use std::marker::PhantomData;

/// The `PhysicsStepperSystem` progresses the nphysics `World`.
pub struct PhysicsStepperSystem<N: RealField>(PhantomData<N>);

impl<'s, N: RealField> System<'s> for PhysicsStepperSystem<N> {
    type SystemData = (
        WriteExpect<'s, MechanicalWorldRes<N>>,
        WriteExpect<'s, GeometricalWorldRes<N>>,
        BodySet<'s, N>,
        ColliderSet<'s, N>,
        WriteExpect<'s, JointConstraintSetRes<N>>,
        WriteExpect<'s, ForceGeneratorSetRes<N>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut mechanical_world,
            mut geometrical_world,
            mut body_set,
            mut collider_set,
            mut joint_constraint_set,
            mut force_generator_set,
        ) = data;

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

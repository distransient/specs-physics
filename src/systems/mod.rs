use crate::{
    nalgebra::{convert as na_convert, RealField},
    nphysics::math::Vector,
    position::Position,
    world::{ForceGeneratorSetRes, GeometricalWorldRes, JointConstraintSetRes, MechanicalWorldRes},
};
use specs::{DispatcherBuilder, World};
use std::marker::PhantomData;

mod batch;
mod pose;
mod stepper;

pub use self::{batch::PhysicsBatchSystem, pose::PhysicsPoseSystem, stepper::PhysicsStepperSystem};

pub struct PhysicsBundle<N: RealField, P: Position<N>> {
    mechanical_world: MechanicalWorldRes<N>,
    geometrical_world: GeometricalWorldRes<N>,
    // Exercising superfluous allocations at init-time
    // is better than figuring out the lifetimes
    // for the slice version of this at programming-time.
    stepper_deps: Vec<Box<str>>,
    marker: PhantomData<P>,
}

impl<N: RealField, P: Position<N>> PhysicsBundle<N, P> {
    pub fn new(gravity: Vector<N>) -> Self {
        Self::from_deps(gravity, &[])
    }

    pub fn from_deps(gravity: Vector<N>, dep: &[&str]) -> Self {
        Self::from_parts(
            MechanicalWorldRes::<N>::new(gravity),
            GeometricalWorldRes::<N>::new(),
            dep,
        )
    }

    pub fn from_parts(
        mechanical_world: MechanicalWorldRes<N>,
        geometrical_world: GeometricalWorldRes<N>,
        dep: &[&str],
    ) -> Self {
        Self {
            mechanical_world,
            geometrical_world,
            stepper_deps: dep.iter().map(|s| Box::from(*s)).collect(),
            marker: PhantomData,
        }
    }

    /// Add dependencies which will be attached to the stepper,
    /// to be executed before physics stepping.
    pub fn with_deps(mut self, dep: &[&str]) -> Self {
        self.stepper_deps = [
            self.stepper_deps
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<&str>>()
                .as_slice(),
            dep,
        ]
        .concat()
        .iter()
        .map(|s| Box::from(*s))
        .collect();
        self
    }

    pub fn register(self, world: &mut World, builder: &mut DispatcherBuilder) {
        world.insert(self.mechanical_world);
        world.insert(self.geometrical_world);

        // TODO: These would be defaulted when converted to specs Storages.
        world.insert(JointConstraintSetRes::<N>::new());
        world.insert(ForceGeneratorSetRes::<N>::new());

        // Add PhysicsStepperSystem after all other Systems that write data to the
        // nphysics World and has to depend on them; this System is used to progress the
        // nphysics World for all existing objects.
        builder.add(
            PhysicsStepperSystem::<N>::default(),
            "physics_stepper_system",
            self.stepper_deps
                .iter()
                .map(|s| s.as_ref())
                .collect::<Vec<&str>>()
                .as_slice(),
        );

        // Add PhysicsPoseSystem last as it handles the
        // synchronisation between nphysics World bodies and the Position
        // components; this depends on the PhysicsStepperSystem.
        builder.add(
            PhysicsPoseSystem::<N, P>::default(),
            "physics_pose_system",
            &["physics_stepper_system"],
        );
    }
}

#[cfg(feature = "amethyst")]
impl<'a, 'b, N: RealField, P: Position<N>> amethyst_core::SystemBundle<'a, 'b>
    for PhysicsBundle<N, P>
{
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder,
    ) -> Result<(), amethyst_error::Error> {
        Ok(self.register(world, builder))
    }
}

impl<N: RealField, P: Position<N>> Default for PhysicsBundle<N, P> {
    fn default() -> Self {
        Self::new(Vector::<N>::y() * na_convert::<f64, N>(-9.81))
    }
}

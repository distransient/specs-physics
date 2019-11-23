use crate::{
    nalgebra::{convert as na_convert, RealField},
    nphysics::math::Vector,
    position::Pose,
    stepper::{Step, StepperRes},
    world::{ForceGeneratorSetRes, GeometricalWorldRes, JointConstraintSetRes, MechanicalWorldRes},
};
use specs::{DispatcherBuilder, World};
use std::marker::PhantomData;

mod batch;
mod pose;
mod stepper;

pub use self::{batch::PhysicsBatchSystem, pose::PhysicsPoseSystem, stepper::PhysicsStepperSystem};
#[cfg(feature = "amethyst")]
pub use self::{pose::PhysicsPoseSystemDesc, stepper::PhysicsStepperSystemDesc};

pub struct PhysicsBundle<N: RealField, P: Pose<N>> {
    mechanical_world: MechanicalWorldRes<N>,
    geometrical_world: GeometricalWorldRes<N>,
    stepper_res: Option<StepperRes>,
    // Exercising superfluous allocations at init-time
    // is better than figuring out the lifetimes
    // for the slice version of this at programming-time.
    stepper_deps: Vec<Box<str>>,
    marker: PhantomData<P>,
}

impl<N: RealField, P: Pose<N>> PhysicsBundle<N, P> {
    pub fn new(gravity: Vector<N>) -> Self {
        Self::from_deps(gravity, &[])
    }

    pub fn stepper(interval: u32) -> Self {
        Self::default().with_stepper(interval)
    }

    pub fn from_deps(gravity: Vector<N>, dep: &[&str]) -> Self {
        Self::from_parts(
            MechanicalWorldRes::<N>::new(gravity),
            GeometricalWorldRes::<N>::new(),
            None,
            dep,
        )
    }

    pub fn from_parts(
        mechanical_world: MechanicalWorldRes<N>,
        geometrical_world: GeometricalWorldRes<N>,
        stepper_res: Option<StepperRes>,
        dep: &[&str],
    ) -> Self {
        Self {
            mechanical_world,
            geometrical_world,
            stepper_res,
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

    pub fn with_stepper(mut self, interval: u32) -> Self {
        self.stepper_res = Some(StepperRes::new_fixed(interval));
        self
    }

    pub fn with_stepper_instance(mut self, stepper: StepperRes) -> Self {
        self.stepper_res = Some(stepper);
        self
    }

    fn register_resources(self, world: &mut World) {
        world.entry::<MechanicalWorldRes<N>>().or_insert(self.mechanical_world);
        world.entry::<GeometricalWorldRes<N>>().or_insert(self.geometrical_world);

        if let Some(stepper_res) = self.stepper_res {
            world.entry::<StepperRes>().or_insert(stepper_res);
            world.entry::<Step>().or_insert(Step::default());
        }

        // TODO: These would be defaulted when converted to specs Storages.
        world.entry::<JointConstraintSetRes<N>>().or_insert(JointConstraintSetRes::<N>::new());
        world.entry::<ForceGeneratorSetRes<N>>().or_insert(ForceGeneratorSetRes::<N>::new());
    }

    pub fn register(self, world: &mut World, builder: &mut DispatcherBuilder) {
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

        self.register_resources(world);
    }
}

#[cfg(feature = "amethyst")]
impl<'a, 'b, N: RealField, P: Pose<N>> amethyst::core::SystemBundle<'a, 'b>
    for PhysicsBundle<N, P>
{
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder,
    ) -> Result<(), amethyst::error::Error> {
        use amethyst::core::SystemDesc;

        // Add PhysicsStepperSystem after all other Systems that write data to the
        // nphysics World and has to depend on them; this System is used to progress the
        // nphysics World for all existing objects.
        builder.add(
            PhysicsStepperSystemDesc::<N>::default().build(world),
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
            PhysicsPoseSystemDesc::<N, P>::default().build(world),
            "physics_pose_system",
            &["physics_stepper_system"],
        );

        self.register_resources(world);
        Ok(())
    }
}

impl<N: RealField, P: Pose<N>> Default for PhysicsBundle<N, P> {
    fn default() -> Self {
        Self::new(Vector::<N>::y() * na_convert::<f64, N>(-9.81))
    }
}

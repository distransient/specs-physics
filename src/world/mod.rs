use crate::nphysics::{
    force_generator::DefaultForceGeneratorSet,
    joint::DefaultJointConstraintSet,
    world::{GeometricalWorld, MechanicalWorld},
};

pub(crate) mod body_set;
pub(crate) mod collider_set;

pub use body_set::{BodyComponent, BodyHandleType, BodySet};
pub use collider_set::{ColliderComponent, ColliderHandleType, ColliderSet};

pub type MechanicalWorldRes<N> = MechanicalWorld<N, BodyHandleType, ColliderHandleType>;
pub type GeometricalWorldRes<N> = GeometricalWorld<N, BodyHandleType, ColliderHandleType>;

// TODO: Could likely turn these into storages/provide Join methods?
pub type JointConstraintSetRes<N> = DefaultJointConstraintSet<N, BodyHandleType>;
pub type ForceGeneratorSetRes<N> = DefaultForceGeneratorSet<N, BodyHandleType>;

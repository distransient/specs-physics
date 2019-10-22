use crate::nphysics::{
    force_generator::DefaultForceGeneratorSet,
    joint::DefaultJointConstraintSet,
    object::{DefaultColliderHandle, DefaultColliderSet},
    world::{GeometricalWorld, MechanicalWorld},
};
use body_set::BodyHandleType;

pub mod body_set;

pub use body_set::{ReadBodyStorage, WriteBodyStorage};

pub type ColliderHandleType = DefaultColliderHandle;

pub type MechanicalWorldRes<N> = MechanicalWorld<N, BodyHandleType, ColliderHandleType>;
pub type GeometricalWorldRes<N> = GeometricalWorld<N, BodyHandleType, ColliderHandleType>;
pub type ColliderSetRes<N> = DefaultColliderSet<N, BodyHandleType>;

// TODO: Could likely turn these into storages?
pub type JointConstraintSetRes<N> = DefaultJointConstraintSet<N, BodyHandleType>;
pub type ForceGeneratorSetRes<N> = DefaultForceGeneratorSet<N, BodyHandleType>;

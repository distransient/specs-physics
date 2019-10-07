use crate::{
    nphysics::{
        force_generator::DefaultForceGeneratorSet,
        joint::DefaultJointConstraintSet,
        object::DefaultColliderSet,
        world::{GeometricalWorld, MechanicalWorld},
    },
};

use body_set::{BodyHandleType, BodySet};

use specs::Entity;

pub mod body_set;

pub use body_set::{ReadBodyStorage, WriteBodyStorage};

pub type ColliderHandleType = Entity;

pub type MechanicalWorldRes<'a, N> = MechanicalWorld<N, BodySet<'a, N>, ColliderHandleType>;
pub type GeometricalWorldRes<N> = GeometricalWorld<N, BodyHandleType, ColliderHandleType>;

// TODO
pub type ColliderSetRes<N> = DefaultColliderSet<N, BodyHandleType>;

// TODO: Could likely turn these into storages?
pub type JointConstraintSetRes<'a, N> = DefaultJointConstraintSet<N, BodySet<'a, N>>;
pub type ForceGeneratorSetRes<'a, N> = DefaultForceGeneratorSet<N, BodySet<'a, N>>;

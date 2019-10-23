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

// TODO: Can probably make the JointConstraintSet a Storage.
pub type JointConstraintSetRes<N> = DefaultJointConstraintSet<N, BodyHandleType>;
// TODO: Can probably make ForceGeneratorSet a Storage
// but the usefulness seems somewhat limited
pub type ForceGeneratorSetRes<N> = DefaultForceGeneratorSet<N, BodyHandleType>;

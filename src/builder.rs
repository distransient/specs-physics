use crate::{
    nalgebra::RealField,
    nphysics::object::{Body, BodyPartHandle, ColliderDesc},
    world::{BodyComponent, ColliderComponent},
};

use specs::{world::Builder, EntityBuilder, WorldExt};

pub trait EntityBuilderExt {
    fn with_body<N: RealField>(self, body: impl Body<N>) -> Self;
    fn with_collider<N: RealField>(self, collider: &ColliderDesc<N>) -> Self;
}

impl EntityBuilderExt for EntityBuilder<'_> {
    fn with_body<N: RealField>(self, body: impl Body<N>) -> Self {
        self.with(BodyComponent(Box::new(body)))
    }

    fn with_collider<N: RealField>(self, collider: &ColliderDesc<N>) -> Self {
        {
            let mut storage = self.world.write_storage::<ColliderComponent<N>>();
            storage
                .insert(
                    self.entity,
                    ColliderComponent(collider.build(BodyPartHandle(self.entity, 0))),
                )
                .unwrap();
        }
        self
    }
}

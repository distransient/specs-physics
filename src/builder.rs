use crate::{nalgebra::RealField, nphysics::object::Body, world::body_set::BodyComponent};

use specs::{world::Builder, EntityBuilder};

pub trait EntityBuilderExt {
    fn with_body<N: RealField>(self, body: impl Body<N>) -> Self;
}

impl EntityBuilderExt for EntityBuilder<'_> {
    fn with_body<N: RealField>(self, body: impl Body<N>) -> Self {
        self.with(BodyComponent(Box::new(body)))
    }
}

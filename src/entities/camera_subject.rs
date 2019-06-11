use amethyst::{
    core::Transform,
    ecs::{Entity, prelude::World},
    prelude::Builder,
    renderer::transparent::Transparent,
};

use crate::{
    components::SubjectTag,
};

pub fn load_camera_subject(world: &mut World) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(384., 176., 0.);

    world
        .create_entity()
        .with(transform)
        .with(SubjectTag::default())
        .with(Transparent)
        .build()
}

use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    ecs::{
        prelude::{Builder, Dispatcher, DispatcherBuilder, World}
    },
    prelude::{GameData, State, StateData, StateEvent, Trans},
    renderer::{Camera, PngFormat, Projection, SpriteRender, SpriteSheet,
               SpriteSheetHandle, Texture, TextureMetadata}
};
use specs_physics::{
    colliders::Shape,
    PhysicsBodyBuilder,
    PhysicsColliderBuilder,
    nalgebra::{RealField, Isometry3},
    nphysics::object::BodyStatus,
};
use crate::components;

pub struct GridPosition {
    pub x: u32,
    pub y: u32
}

#[derive(Copy, Clone)]
pub struct ScreenPosition {
    pub x: f32,
    pub y: f32
}

fn create_translated_transform(pos: ScreenPosition) -> Transform {
    let mut local_transform = Transform::default();
    local_transform.set_translation_xyz(
        pos.x,
        pos.y,
        0.0
    );
    local_transform
}

pub fn initialise_cactus(world: &mut World, pos: ScreenPosition, sprite_sheet_handle: SpriteSheetHandle) {
    let mut local_transform = create_translated_transform(pos);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle.clone(),
        sprite_number: 0
    };

    world
        .create_entity()
        .with(components::Cactus)
        .with(sprite_render.clone())
        .with(local_transform)
        .with(
            PhysicsBodyBuilder::<f32>::from(BodyStatus::Dynamic)
                .build(),
        )
        .with(
            PhysicsColliderBuilder::<f32>::from(Shape::Rectangle(32.0f32,32.0f32,0.0f32))
                .build(),
        )
        .build();
}

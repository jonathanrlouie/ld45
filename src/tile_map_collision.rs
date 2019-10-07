use amethyst::{
    core::{
        math::Vector3,
        transform::Transform
    },
    ecs::{Builder, World, WorldExt}
};
use crate::{
    collision_world,
    tile_map::TileMap
};

const WALL_WIDTH: f32 = 32.0;
const HALF_EXTENTS: (f32, f32) = (WALL_WIDTH / 2.0, WALL_WIDTH / 2.0);

// it appears that creating this many colliders actually just straight up ruins the framerate
pub fn create_map_colliders(world: &mut World, collision_world: &mut collision_world::CollisionWorld, tile_map: &TileMap) {
    let tiles = tile_map.tiles().expect("Could not find tiles");

    let mut index = 0;
    for row in tiles {
        for col in row {
            // should use map width here, but oh well
            let x: u32 = index % 32;
            let y: u32 = index / 32;
            if *col != 0 {
                let (shifted_x, shifted_y) = shift_coords(x, y);
                let wall_transform = Transform::from(Vector3::new(shifted_x, shifted_y, 0.0));
                let wall = world
                    .create_entity()
                    .build();

                collision_world.add_collision(
                    world,
                    HALF_EXTENTS,
                    collision_world::WALL_COLLISION_GROUP,
                    wall,
                    wall_transform,
                );
            }
            index += 1;
        }
    }
}

fn shift_coords(x: u32, y: u32) -> (f32, f32) {
    ((x as f32) + (WALL_WIDTH * -16.0), (y as f32) + (WALL_WIDTH * 8.0))
}
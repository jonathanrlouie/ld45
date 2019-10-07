use amethyst::{
    core::{
        math::{Vector3, Vector2},
        timing::Time,
        transform::Transform
    },
    ecs::{World, WorldExt, Entity}
};
use nalgebra as na;
use ncollide2d as nc;
use crate::{
    components
};

pub type NcCollisionWorld = nc::pipeline::world::CollisionWorld<f32, Entity>;

pub const PLAYER_COLLISION_GROUP: usize = 0;
pub const WALL_COLLISION_GROUP: usize = 1;
pub const FOOD_COLLISION_GROUP: usize = 2;
pub const EXIT_COLLISION_GROUP: usize = 3;
pub const ENEMY_COLLISION_GROUP: usize = 4;

pub struct CollisionWorld {
    pub world: NcCollisionWorld
}

impl CollisionWorld {
    pub fn update_position(
        &mut self,
        transform: &mut Transform,
        collider: &mut components::Collider,
        motion: &mut components::Motion,
        time: &Time
    ) {
        let delta_seconds = time.delta_seconds();
        let distance = Vector2::new(
            motion.velocity.x * delta_seconds,
            motion.velocity.y * delta_seconds
        );

        let ncollide_world = &mut self.world;

        let mut collision_obj = ncollide_world
            .get_mut(collider.slab_handle)
            .expect("Invalid collision object");

        let mut new_position = collision_obj.position().clone();
        new_position.append_translation_mut(&na::Translation::from(distance));
        collision_obj.set_position(new_position);

        transform.set_translation_xyz(new_position.translation.x, new_position.translation.y, 0.0);
    }

    pub fn add_collision(
        &mut self,
        world: &mut World,
        half_extents: (f32, f32),
        collision_group: usize,
        entity: Entity,
        transform: Transform
    ) {
        let shape = nc::shape::Cuboid::new(Vector2::new(half_extents.0, half_extents.1));
        let mut entity_collide_group = nc::pipeline::object::CollisionGroups::new();
        entity_collide_group.set_membership(&[collision_group]);

        let query_type = nc::pipeline::object::GeometricQueryType::Contacts(0.0, 0.0);
        let (entity_handle, _) = self.world.add(
            na::Isometry2::new(na::Vector2::new(
                transform.translation().x,
                transform.translation().y
            ), na::zero()),
            nc::shape::ShapeHandle::new(shape.clone()),
            entity_collide_group,
            query_type,
            entity
        );

        let collider = components::Collider {
            slab_handle: entity_handle
        };

        world
            .write_storage()
            .insert(entity, collider)
            .expect("Failed to add collider to entity");

        world
            .write_storage()
            .insert(entity, transform)
            .expect("Failed to add transform to entity");
    }

    pub fn update(&mut self) {
        self.world.update();
    }
}

impl Default for CollisionWorld {
    fn default() -> Self {
        CollisionWorld {
            world: NcCollisionWorld::new(0.2)
        }
    }
}
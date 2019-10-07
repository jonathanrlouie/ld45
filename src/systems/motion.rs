use amethyst::{
    core::{
        math::Vector2,
        timing::Time,
        transform::Transform
    },
    ecs::prelude::{Entities, Join, Read, Write, ReadStorage, System, WriteStorage},
};
use nalgebra as na;
use crate::{
    collision_world::CollisionWorld,
    components
};

const SPEED: f32 = 120.0;

pub struct MotionSystem;

impl<'s> System<'s> for MotionSystem {
    type SystemData = (
        WriteStorage<'s, components::Player>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, components::Collider>,
        WriteStorage<'s, components::Motion>,
        Write<'s, CollisionWorld>,
        Read<'s, Time>,
    );

    fn run(&mut self, (
        mut player_storage,
        mut transform_storage,
        mut collider_storage,
        mut motion_storage,
        mut collision_world,
        time,
    ): Self::SystemData) {
        for (
            mut player,
            mut transform,
            mut collider,
            mut motion,
        ) in (
            &mut player_storage,
            &mut transform_storage,
            &mut collider_storage,
            &mut motion_storage,
        ).join() {
            motion.velocity.x = player.lr_input_state * SPEED;
            if player.snapback.x != 0.0 {
                motion.velocity = Vector2::new(-player.snapback.x, motion.velocity.y);
                collision_world.update_position(&mut transform, &mut collider, &mut motion, &time);
            } else {
                motion.velocity += motion.acceleration;
                motion.acceleration = na::zero();

                collision_world.update_position(&mut transform, &mut collider, &mut motion, &time);
            }
        }
    }
}
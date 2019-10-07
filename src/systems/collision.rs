use amethyst::{
    core::{
        math::Vector2,
        timing::Time,
        transform::Transform
    },
    ecs::prelude::{Entities, Join, Read, Write, ReadStorage, System, WriteStorage},
};
use nalgebra as na;
use ncollide2d as nc;
use ncollide2d::pipeline::narrow_phase::ContactEvent;
use crate::{
    collision_world::*,
    components
};

pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        WriteStorage<'s, components::Player>,
        WriteStorage<'s, components::Power>,
        WriteStorage<'s, components::Motion>,
        Write<'s, CollisionWorld>,
        ReadStorage<'s, components::Food>,
        Entities<'s>
    );

    fn run(&mut self, (
        mut player_storage,
        mut power_storage,
        mut motion_storage,
        mut collision_world,
        food_storage,
        entities
    ): Self::SystemData) {
        for (
            mut player,
            mut power,
            mut motion,
        ) in (
            &mut player_storage,
            &mut power_storage,
            &mut motion_storage,
        ).join() {
            collision_world.update();

            let ncollide_world = &mut collision_world.world;

            let mut deleted_handle = None;
            for e in ncollide_world.contact_events() {
                match e {
                    ContactEvent::Started(cobj_handle1, cobj_handle2) => {
                        // I don't know what the effective_only filter does...
                        if let Some((slab_handle1, slab_handle2, _, _)) = ncollide_world
                            .contact_pair(*cobj_handle1, *cobj_handle2, false)
                        {
                            let collision_obj1 = ncollide_world
                                .collision_object(slab_handle1)
                                .expect("Invalid collision object");

                            let collision_obj2 = ncollide_world
                                .collision_object(slab_handle2)
                                .expect("Invalid collision object");

                            if collision_obj2.collision_groups().is_member_of(WALL_COLLISION_GROUP) {
                                let vec1 = collision_obj1.position().translation.vector;
                                let vec2 = collision_obj2.position().translation.vector;
                                player.snapback = vec2 - vec1;
                            }

                            if collision_obj2.collision_groups().is_member_of(FOOD_COLLISION_GROUP) {
                                let food_entity = collision_obj2.data();
                                let food = food_storage.get(*food_entity);
                                let fillingness = food.unwrap().fillingness();
                                if player.belly + fillingness <= player.belly_max() {
                                    player.belly += fillingness;
                                    power.value += 1;
                                    deleted_handle = Some(slab_handle2.clone());
                                    entities.delete(*food_entity).unwrap();
                                }
                            }

                            if collision_obj2.collision_groups().is_member_of(EXIT_COLLISION_GROUP) {
                                player.state = components::PlayerState::Exiting;
                            }
                        }
                    },
                    ContactEvent::Stopped(cobj_handle1, cobj_handle2) => {
                        player.snapback = Vector2::new(0.0, 0.0);
                    }
                }
            }

            if let Some(handle) = deleted_handle {
                ncollide_world.remove(&[handle]);
            }
        }
    }
}
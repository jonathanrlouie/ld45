use amethyst::{
    animation::{get_animation_set, AnimationSet, AnimationControlSet, AnimationCommand, EndControl},
    input::{InputHandler, StringBindings},
    ecs::prelude::{Entities, Join, Read, ReadStorage, System, WriteStorage},
    renderer::SpriteRender
};
use crate::{
    level1,
    components
};

pub struct AnimationSystem;

impl<'s> System<'s> for AnimationSystem {
    type SystemData = (
        ReadStorage<'s, AnimationSet<level1::AnimationId, SpriteRender>>,
        WriteStorage<'s, AnimationControlSet<level1::AnimationId, SpriteRender>>,
        Entities<'s>
    );

    fn run(&mut self, (animation_set_storage, mut control_set_storage, entities): Self::SystemData) {
        for (entity, animation_set) in (&entities, &animation_set_storage).join() {
            let control_set = get_animation_set(&mut control_set_storage, entity).unwrap();
            control_set.add_animation(
                level1::AnimationId::IdleRight,
                &animation_set.get(&level1::AnimationId::IdleRight).unwrap(),
                EndControl::Loop(None),
                1.0,
                AnimationCommand::Start,
            );
        }
    }
}

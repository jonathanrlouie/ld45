use amethyst::{
    input::{InputHandler, StringBindings},
    ecs::prelude::{Join, Read, ReadStorage, System, WriteStorage}
};
use crate::components;

pub struct InputSystem;

impl<'s> System<'s> for InputSystem {
    type SystemData = (
        WriteStorage<'s, components::Player>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut player_storage, input): Self::SystemData) {
        for (mut player) in (&mut player_storage).join() {
            if let Some(direction) = input.axis_value("left_right") {
                 player.lr_input_state = direction;
            }
        }
    }
}
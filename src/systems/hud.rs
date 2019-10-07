use amethyst::{
    ecs::prelude::{Entity, Join, Read, ReadExpect, ReadStorage, System, WriteStorage},
    ui::UiText
};
use crate::components;

pub struct Hud {
    pub entity: Entity
}

pub struct HudSystem;

impl<'s> System<'s> for HudSystem {
    type SystemData = (
        ReadStorage<'s, components::Player>,
        ReadStorage<'s, components::HP>,
        ReadStorage<'s, components::Power>,
        ReadExpect<'s, Hud>,
        WriteStorage<'s, UiText>,
    );

    fn run(&mut self, (player_storage, hp_storage, power_storage, hud, mut text): Self::SystemData) {
        for (player, hp, power) in (&player_storage, &hp_storage, &power_storage).join() {
            if let Some(mut text) = text.get_mut(hud.entity) {
                text.text = format!(
                    "Belly: {}  HP: {}  Power: {}",
                    player.belly.to_string(),
                    hp.value.to_string(),
                    power.value.to_string()
                ).to_string();
            }
        }
    }
}

use std::time::Duration;
use amethyst::{
    config::Config,
    core::{
        SystemBundle,
        frame_limiter::FrameRateLimitStrategy,
        transform::TransformBundle
    },
    input::{InputBundle, StringBindings},
    prelude::{Application, GameDataBuilder},
    renderer::{
        types::DefaultBackend,
        RenderDebugLines, RenderFlat2D, RenderToWindow, RenderingBundle
    },
    tiles::{RenderTiles2D, Tile, TileMap},
    ui::{RenderUi, DrawUi, UiBundle},
    utils::application_root_dir
};

mod components;
mod tile_map;
mod level1;
mod systems;
mod collision_world;
mod util;
mod tile;
mod tile_map_collision;

const FRAME_LIMIT: u32 = 60;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let display_config_path = app_root.join("resources/display.ron");

    let assets_dir = app_root.join("assets/");

    let key_bindings_path = app_root.join("resources/input.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(
            InputBundle::<StringBindings>::new()
                .with_bindings_from_file(&key_bindings_path)?
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.6, 0.85, 0.91, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default())
                .with_plugin(RenderTiles2D::<tile::TerrainTile>::default())
                .with_plugin(RenderDebugLines::default()),
        )?
        .with_bundle(UiBundle::<StringBindings>::new())?;

    let mut game = Application::build(assets_dir, level1::Level1::new())?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            FRAME_LIMIT,
        )
        .build(game_data)?;

    game.run();
    Ok(())
}
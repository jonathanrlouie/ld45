use amethyst::{
    assets::{AssetStorage, Loader},
    core::{
        math::{Vector2, Vector3, Point3},
        transform::Transform
    },
    ecs::{
        prelude::{Builder, Dispatcher, DispatcherBuilder, Entity, World, WorldExt}
    },
    prelude::{GameData, State, StateData, StateEvent, Trans},
    renderer::{
        camera::{Camera, Projection},
        formats::texture::ImageFormat,
        sprite::{SpriteSheet, SpriteSheetHandle, SpriteRender},
        SpriteSheetFormat, Texture
    },
    tiles::{Tile, TileMap as AmethystTileMap},
    window::ScreenDimensions,
    ui::{Anchor, TtfFormat, UiText, UiTransform},
};
use ncollide2d as nc;
use nalgebra as na;

use crate::{
    util::{
        PngPath,
        RonPath,
        Exiting
    },
    components,
    tile::TerrainTile,
    tile_map::{TmxFilePath, create_map},
    systems,
    collision_world::*,
    tile_map_collision,
};


pub const SPRITE_WIDTH: f32 = 32.0;
pub const HALF_WIDTH: f32 = SPRITE_WIDTH / 2.0;

pub struct Level2<'a, 'b> {
    dispatcher: Dispatcher<'a, 'b>
}

impl<'a, 'b> Level2<'a, 'b> {
    pub fn new() -> Self {
        Level2 {
            dispatcher: Level2::initialise_dispatcher()
        }
    }

    fn initialise_dispatcher() -> Dispatcher<'a, 'b> {
        let mut dispatcher_builder = DispatcherBuilder::new();

        dispatcher_builder.add(systems::motion::MotionSystem, "motion_system", &[]);
        dispatcher_builder.add(systems::collision::CollisionSystem, "collision_system", &[]);
        dispatcher_builder.add(systems::input::InputSystem, "input_system", &[]);
        dispatcher_builder.add(systems::hud::HudSystem, "hud_system", &[]);

        dispatcher_builder.build()
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Level2<'a, 'b> {
    fn on_start(&mut self, data: StateData<GameData<'a, 'b>>) {
        let StateData { mut world, .. } = data;

        let mut collision_world = CollisionWorld::default();

        self.dispatcher.setup(&mut world);

        let (width, height) = {
            let screen_dimensions = world.read_resource::<ScreenDimensions>();
            (screen_dimensions.width(), screen_dimensions.height())
        };

        // Initialise the camera
        initialise_camera(&mut world, Transform::from(Vector3::new(-HALF_WIDTH, HALF_WIDTH, 1.0)), Camera::standard_2d(width, height));

        // Initialise the tilemap
        // panic with the given error message if we fail to load the tile map
        let (amethyst_map, tile_map) = create_map::<TerrainTile>(
            TmxFilePath(".\\resources\\level1.tmx"),
            &mut world
        ).unwrap_or_else(|e| panic!("Problem loading tile map: {:?}", e));


        world
            .create_entity()
            .with(amethyst_map)
            .with(Transform::default())
            .build();

        // Initialise the player
        let player_sheet_handle = load_sprite_sheet(
            &mut world,
            PngPath("textures/player.png"),
            RonPath("textures/player.ron")
        );

        let mut player_transform = Transform::default();
        player_transform.set_translation_xyz(SPRITE_WIDTH * -12.0, SPRITE_WIDTH * -6.0, 0.0);

        let player = world
            .create_entity()
            .with(components::Player {
                lr_input_state: 0.0,
                snapback: Vector2::new(0.0, 0.0),
                state: components::PlayerState::Idle,
                belly: 0,
            })
            .with(components::HP { value: 30 })
            .with(components::Power { value: 1 })
            .with(SpriteRender {
                sprite_sheet: player_sheet_handle.clone(),
                sprite_number: 8
            })
            .with(components::Motion{
                velocity: Vector2::new(0.0, 0.0),
                acceleration: Vector2::new(0.0, 0.0)
            })
            .build();

        collision_world.add_collision(
            &mut world,
            (HALF_WIDTH, HALF_WIDTH),
            PLAYER_COLLISION_GROUP,
            player,
            player_transform
        );

        // Initialise wall collision
        let wall_transform = Transform::from(Vector3::new(SPRITE_WIDTH * 15.0, SPRITE_WIDTH * -6.0, 0.0));
        let wall = world
            .create_entity()
            .build();

        collision_world.add_collision(
            world,
            (HALF_WIDTH, HALF_WIDTH),
            WALL_COLLISION_GROUP,
            wall,
            wall_transform,
        );

        let wall_transform = Transform::from(Vector3::new(SPRITE_WIDTH * -16.0, SPRITE_WIDTH * -6.0, 0.0));
        let wall = world
            .create_entity()
            .build();

        collision_world.add_collision(
            world,
            (HALF_WIDTH, HALF_WIDTH),
            WALL_COLLISION_GROUP,
            wall,
            wall_transform,
        );

        // Initialise food
        let objects_sheet_handle = load_sprite_sheet(
            &mut world,
            PngPath("textures/objects.png"),
            RonPath("textures/objects.ron")
        );

        let blueberries_transform = Transform::from(Vector3::new(SPRITE_WIDTH * -4.0, SPRITE_WIDTH * -2.0, -1.0));
        let blueberries = world
            .create_entity()
            .with(components::Food::new(components::FoodType::Blueberries))
            .with(SpriteRender {
                sprite_sheet: objects_sheet_handle.clone(),
                sprite_number: 3
            })
            .build();

        collision_world.add_collision(
            &mut world,
            (HALF_WIDTH, HALF_WIDTH),
            FOOD_COLLISION_GROUP,
            blueberries,
            blueberries_transform
        );

        let apple_transform = Transform::from(Vector3::new(SPRITE_WIDTH * 0.0, SPRITE_WIDTH * 2.0, -1.0));
        let apple = world
            .create_entity()
            .with(components::Food::new(components::FoodType::Apple))
            .with(SpriteRender {
                sprite_sheet: objects_sheet_handle.clone(),
                sprite_number: 2
            })
            .build();

        collision_world.add_collision(
            &mut world,
            (HALF_WIDTH, HALF_WIDTH),
            FOOD_COLLISION_GROUP,
            apple,
            apple_transform
        );

        let snake_transform = Transform::from(Vector3::new(SPRITE_WIDTH * 3.0, SPRITE_WIDTH * 2.0, -1.0));
        let snake = world
            .create_entity()
            .with(SpriteRender {
                sprite_sheet: objects_sheet_handle.clone(),
                sprite_number: 5
            })
            .build();

        collision_world.add_collision(
            &mut world,
            (HALF_WIDTH, HALF_WIDTH),
            ENEMY_COLLISION_GROUP,
            snake,
            snake_transform
        );

        let exit_transform = Transform::from(Vector3::new(SPRITE_WIDTH * 12.0, SPRITE_WIDTH * -6.0, -1.0));
        let exit = world
            .create_entity()
            .with(SpriteRender {
                sprite_sheet: objects_sheet_handle.clone(),
                sprite_number: 0
            })
            .build();

        collision_world.add_collision(
            &mut world,
            (HALF_WIDTH, HALF_WIDTH),
            EXIT_COLLISION_GROUP,
            exit,
            exit_transform
        );

        // initialise HUD elements
        let font = world.read_resource::<Loader>().load(
            "font/TestFont.ttf",
            TtfFormat,
            (),
            &world.read_resource(),
        );

        // probably should choose different anchor, but time
        let hud_transform = UiTransform::new(
            "hud text".to_string(),
            Anchor::Middle,
            Anchor::Middle,
            -400.,
            290.,
            0.,
            2000.,
            500.,
        );

        let hud = world
            .create_entity()
            .with(hud_transform)
            .with(UiText::new(
                font.clone(),
                "".to_string(),
                [0.0, 0.0, 0.0, 1.0],
                30.,
            ))
            .build();

        world.insert(systems::hud::Hud { entity: hud });
        world.insert(tile_map);
        world.insert(Exiting {
            exiting: false
        });
        world.insert(collision_world);
    }

    fn update(&mut self, data: StateData<GameData<'a, 'b>>) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world);
        self.dispatcher.dispatch(&data.world);

        let exiting = data.world.write_resource::<Exiting>();
        /*if exiting.exiting {
            exiting.exiting = false;
            Trans::Switch(Box::new(level2::Level2))
        } else {*/
            Trans::None
        //}
    }
}

fn load_sprite_sheet(world: &mut World, png_path: PngPath, ron_path: RonPath) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(png_path.0, ImageFormat::default(), (), &texture_storage)
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        ron_path.0,
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn initialise_camera(world: &mut World, transform: Transform, camera: Camera) -> Entity {
    world
        .create_entity()
        .with(transform)
        .with(camera)
        .build()
}


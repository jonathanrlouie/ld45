use std::{
    error::Error,
    fmt,
    fs::File,
    path::Path
};
use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{
        math::Vector3,
        transform::Transform
    },
    ecs::{Entity, Builder, World, WorldExt},
    renderer::{
        formats::texture::ImageFormat,
        sprite::{Sprite, SpriteSheet, SpriteSheetHandle},
        Texture
    },
    tiles::{Tile, TileMap as AmethystTileMap}
};
use tiled;

// Example path: "./resources/desert.tmx"
pub struct TmxFilePath<'a>(pub &'a str);

#[derive(Debug)]
pub struct TilesetNotFoundError;

impl Error for TilesetNotFoundError {}

impl fmt::Display for TilesetNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No tilesets were found.")
    }
}

#[derive(Debug)]
pub struct TilesetImageNotFoundError;

impl Error for TilesetImageNotFoundError {}

impl fmt::Display for TilesetImageNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "No images for the tileset were found.")
    }
}

struct TileMapDimensions(u32, u32);
struct TileDimensions(u32, u32);

pub struct TileMap {
    tiled_map: tiled::Map
}

pub fn create_map<TileType: Tile>(
    tmx_file_path: TmxFilePath,
    world: &mut World
) -> Result<(AmethystTileMap<TileType>, TileMap), Box<dyn Error>> {
    let tile_map = TileMap::load_map(tmx_file_path)?;

    let TileMapDimensions(width, height) = tile_map.dimensions();
    let TileDimensions(tile_width, tile_height) = tile_map.tile_dimensions();
    let tileset_handle = tile_map.load_tileset(world)?;

    let map = AmethystTileMap::<TileType>::new(
        Vector3::new(width, height, 1),
        Vector3::new(tile_width, tile_height, 1),
        Some(tileset_handle),
    );

    Ok((map, tile_map))
}

impl TileMap {
    fn load_map(tmx_file_path: TmxFilePath) -> Result<Self, Box<dyn Error>> {
        let map_file = File::open(&Path::new(&tmx_file_path.0))?;
        let tiled_map = tiled::parse(map_file)?;
        Ok(TileMap {
            tiled_map
        })
    }

    pub fn tiles(&self) -> Option<&Vec<Vec<u32>>> {
        self.tiled_map.layers.get(0).map(|layer| &layer.tiles)
    }

    fn dimensions(&self) -> TileMapDimensions {
        TileMapDimensions(self.tiled_map.width, self.tiled_map.height)
    }

    fn tile_dimensions(&self) -> TileDimensions {
        TileDimensions(self.tiled_map.tile_width, self.tiled_map.tile_height)
    }

    fn load_tileset(&self, world: &mut World) -> Result<SpriteSheetHandle, Box<dyn Error>> {
        let tiled_image = self.get_tile_set_image()?;
        Ok(self.load_sprite_sheet(world, tiled_image))
    }

    fn get_tile_set_image(&self) -> Result<&tiled::Image, Box<dyn Error>> {
        // This will always use index 0 because we won't have more than 1 image per tileset
        // and we also won't have more than 1 tileset per tile map
        let tilesets = &self.tiled_map.tilesets;
        let first_tileset = tilesets.get(0).ok_or_else(|| TilesetNotFoundError)?;
        let first_image = first_tileset.images.get(0).ok_or_else(|| TilesetImageNotFoundError)?;
        Ok(first_image)
    }

    fn load_sprite_sheet(&self, world: &mut World, tiled_image: &tiled::Image) -> SpriteSheetHandle {
        let texture_handle: Handle<Texture> = {
            let loader = world.read_resource::<Loader>();
            let texture_storage = world.read_resource::<AssetStorage<Texture>>();
            loader.load(
                tiled_image.source.as_str(),
                ImageFormat::default(),
                (),
                &texture_storage,
            )
        };

        let sprite_sheet = self.create_sprite_sheet(texture_handle, tiled_image);
        let loader = world.read_resource::<Loader>();
        loader.load_from_data(
            sprite_sheet,
            (),
            &world.read_resource::<AssetStorage<SpriteSheet>>(),
        )
    }

    fn create_sprite_sheet(&self, texture: Handle<Texture>, tiled_image: &tiled::Image) -> SpriteSheet {
        let image_w = tiled_image.width as u32;
        let image_h = tiled_image.height as u32;
        let sprite_w = self.tiled_map.tile_width;
        let sprite_h = self.tiled_map.tile_height;
        let offsets = [0.0; 2];

        let column_count = image_w / sprite_w;
        let row_count = image_h / sprite_h;

        let sprite_count = column_count * row_count;

        let sprites = (0..sprite_count).map(move |index| {
            let offset_x = index % column_count * sprite_w;
            let offset_y = index / column_count * sprite_h;
            Sprite::from_pixel_values(
                image_w,
                image_h,
                sprite_w,
                sprite_h,
                offset_x,
                offset_y,
                offsets,
                false,
                false
            )
        }).collect::<Vec<Sprite>>();

        SpriteSheet {
            texture,
            sprites,
        }
    }
}

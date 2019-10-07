use amethyst::{
    core::math::Point3,
    ecs::{World, WorldExt},
    tiles::Tile
};
use crate::tile_map::TileMap;

#[derive(Default, Clone)]
pub struct TerrainTile;
impl Tile for TerrainTile {
    fn sprite(&self, point: Point3<u32>, world: &World) -> Option<usize> {
        let tile_map = world.read_resource::<TileMap>();
        tile_map.tiles().and_then(|tiles| {
            if tiles[point.y as usize][point.x as usize] == 0 {
                None
            } else {
                Some(tiles[point.y as usize][point.x as usize] as usize - 1)
            }
        })
    }
}

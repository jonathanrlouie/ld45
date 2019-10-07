use amethyst::{
    core::math::Vector2,
    ecs::prelude::{Component, VecStorage, DenseVecStorage}
};
use ncollide2d as nc;

pub struct HP {
    pub value: u32
}

impl Component for HP {
    type Storage = DenseVecStorage<Self>;
}

pub struct Power {
    pub value: u32,
}

impl Component for Power {
    type Storage = DenseVecStorage<Self>;
}

pub enum PlayerState {
    Exiting,
    Idle,
    Walking,
    Attacking,
    Climbing,
    Crouching
}

pub struct Player {
    pub snapback: Vector2<f32>,
    pub lr_input_state: f32,
    pub state: PlayerState,
    pub belly: u8,
}

impl Player {
    pub fn belly_max(&self) -> u8 { 10 }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone, Debug)]
pub struct Collider {
    pub slab_handle: nc::pipeline::object::CollisionObjectSlabHandle,
}

impl Component for Collider {
    type Storage = VecStorage<Self>;
}

pub struct Motion {
    pub velocity: Vector2<f32>,
    pub acceleration: Vector2<f32>,
}

impl Component for Motion {
    type Storage = VecStorage<Self>;
}

pub enum FoodType {
    Carrot,
    Apple,
    Blueberries,
    Clover
}

pub struct Food {
    fillingness: u8,
}

impl Food {
    pub fn new(food_type: FoodType) -> Self {
        match food_type {
            FoodType::Carrot => Food { fillingness: 3 },
            FoodType::Apple => Food { fillingness: 6 },
            FoodType::Blueberries => Food { fillingness: 1 },
            FoodType::Clover => Food { fillingness: 0 },
        }
    }

    pub fn fillingness(&self) -> u8 {
        self.fillingness
    }
}

impl Component for Food {
    type Storage = VecStorage<Self>;
}

pub struct Exit;

impl Component for Exit {
    type Storage = VecStorage<Self>;
}

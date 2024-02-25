use crate::{render::GameRenderObject, ARMOR_PICKUP_TILE, HEALTH_PICKUP_TILE};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PickupType {
    Health,
    Armor,
}

pub struct Pickup {
    position: [i32; 2],
    variant: PickupType,
}

impl GameRenderObject for Pickup {
    fn is_visible(&self) -> bool {
        true
    }

    fn get_frame(&self) -> &[f64; 4] {
        match self.variant {
            PickupType::Health => &HEALTH_PICKUP_TILE,
            PickupType::Armor => &ARMOR_PICKUP_TILE,
        }
    }

    fn get_position(&self) -> &[i32; 2] {
        &self.position
    }

    fn get_previous_position(&self) -> &[i32; 2] {
        &self.position
    }
}

impl Pickup {
    pub fn new(position: [i32; 2], variant: PickupType) -> Pickup {
        Pickup { position, variant }
    }

    pub fn set_position(&mut self, position: [i32; 2]) {
        self.position = position;
    }

    pub fn get_variant(&self) -> &PickupType {
        &self.variant
    }
}

pub struct PickupSpawnSystem {
    variant: PickupType,
    spawn_interval: f64,
    last_spawn_dt: f64,
}

impl PickupSpawnSystem {
    pub fn new(variant: PickupType, spawn_interval: f64) -> PickupSpawnSystem {
        PickupSpawnSystem {
            variant,
            spawn_interval,
            last_spawn_dt: 0.0,
        }
    }

    pub fn on_frame(&mut self, dt: f64) {
        self.last_spawn_dt += dt;
    }

    pub fn get_pickup_to_spawn(&self) -> Option<Pickup> {
        if self.last_spawn_dt > self.spawn_interval {
            let pickup = Pickup::new([0, 0], self.variant);

            Some(pickup)
        } else {
            None
        }
    }

    pub fn reset_spawn_timer(&mut self) {
        self.last_spawn_dt = 0.0;
    }
}

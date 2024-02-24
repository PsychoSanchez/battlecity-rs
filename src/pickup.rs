use crate::{ARMOR_PICKUP_TILE, HEALTH_PICKUP_TILE};

pub enum PickupType {
    Health,
    Armor,
}

pub struct Pickup {
    position: [i32; 2],
    variant: PickupType,
}

impl Pickup {
    pub fn new(position: [i32; 2], variant: PickupType) -> Pickup {
        Pickup { position, variant }
    }

    pub fn get_position(&self) -> &[i32; 2] {
        &self.position
    }

    pub fn set_position(&mut self, position: [i32; 2]) {
        self.position = position;
    }

    pub fn get_variant(&self) -> &PickupType {
        &self.variant
    }

    pub fn get_frame(&self) -> &[f64; 4] {
        match self.variant {
            PickupType::Health => &HEALTH_PICKUP_TILE,
            PickupType::Armor => &ARMOR_PICKUP_TILE,
        }
    }
}

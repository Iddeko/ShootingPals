use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use strum_macros::EnumIter;

use crate::player::guns::{revolver::create_revolver_pickup, sniper::create_sniper_pickup};
use crate::rendering::outline::Outline;
use crate::player::guns::GunBundle;
use super::pickup::{PickupBundle, PickupType};

#[derive(AssetCollection, Resource)]
pub struct GunAssets {
    #[asset(path = "guns/bullet.png")]
    pub marine_bullet: Handle<Image>,
    #[asset(path = "guns/marine_gun.png")]
    pub marine: Handle<Image>,
    #[asset(path = "guns/revolver.png")]
    pub revolver: Handle<Image>,
    #[asset(path = "guns/shotgun.png")]
    pub shotgun: Handle<Image>,
    #[asset(path = "guns/sniper.png")]
    pub sniper: Handle<Image>,
    #[asset(path = "guns/kalachnikov.png")]
    pub kalachnikov: Handle<Image>,
    #[asset(path = "guns/full_auto.png")]
    pub automatic: Handle<Image>,
    #[asset(path = "guns/flamethrower.png")]
    pub flame_thrower: Handle<Image>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, EnumIter, Reflect)]
pub enum Guns {
    Revolver,
    Sniper,
}

impl Guns {
    pub fn to_pickup(
        self,
        pos: Vec2,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<Outline>>,
        sprites: &Res<GunAssets>,
    ) -> PickupBundle {
        match self {
            Guns::Revolver => create_revolver_pickup(pos, meshes, materials, sprites),
            Guns::Sniper => create_sniper_pickup(pos, meshes, materials, sprites),
        }
    }

    pub fn to_gun_bundle(&self, assets: &Res<GunAssets>) -> GunBundle {
        match self {
            Guns::Revolver => GunBundle::revolver(assets),
            Guns::Sniper => GunBundle::sniper(assets),
        }
    }
}

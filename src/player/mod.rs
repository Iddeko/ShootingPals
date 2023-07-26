pub mod bullets;
pub mod input;
pub mod setup;
pub mod stats;
pub mod weapon;
pub mod assets;

use bevy::{prelude::*, window::PrimaryWindow};

use bevy_asset_loader::prelude::*;

use input::PlayerState;
use stats::PlayerStats;
use leafwing_input_manager::prelude::*;

use self::assets::{PlayerAssets, GunAssets};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PlayerStats>()
            .register_type::<PlayerState>()
            .init_collection::<assets::PlayerAssets>()
            .init_collection::<assets::GunAssets>()
            .add_plugins(InputManagerPlugin::<input::PlayerActions>::default())
            .add_systems(Startup, setup_players)
            .add_systems(Update, input::move_players)
            .add_systems(Update, input::shooting_system)
            .add_systems(Update, bullets::move_bullets)
            .add_systems(Update, bullets::detect_collision_bullets)
            .add_systems(PostUpdate, stats::player_death);
    }
}

fn setup_players(
    mut commands: Commands,
    window: Query<Entity, With<PrimaryWindow>>,
    assets: Res<PlayerAssets>,
    guns: Res<GunAssets>
) {
    setup::PlayerBundle::setup(
        &mut commands,
        &window,
        true,
        &assets,
        &guns,
    );
    setup::PlayerBundle::setup(
        &mut commands,
        &window,
        false,
        &assets,
        &guns,
    );
}

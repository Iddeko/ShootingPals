use bevy::{prelude::*, window::PrimaryWindow};
use leafwing_input_manager::{prelude::ActionStateDriver, InputManagerBundle};
use mouse::Mouse;

use crate::{
    animation::{AnimationFlip, AnimationIndices, AnimationState, AnimationStateMachine},
    mouse,
    physics::TesselatedCollider,
    rendering::{Offset, Position, Zindex},
};

use input::PlayerActions;

use super::{
    input::{self, IsController, PlayerState},
    stats::PlayerStats,
    weapon::{GunBundle, GunEntity}, assets::{PlayerAssets, GunAssets},
};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub name: Name,
    pub state: AnimationState,
    pub state_machine: AnimationStateMachine,
    pub sprite: SpriteSheetBundle,
    pub player: PlayerStats,
    pub player_action: InputManagerBundle<PlayerActions>,
    pub player_position: Position,
    pub zindex: Zindex,
    pub player_offset: Offset,
    pub current_gun: GunEntity,
    pub collider: TesselatedCollider,
}

impl PlayerBundle {
    pub fn setup(
        commands: &mut Commands,
        window: &Query<Entity, With<PrimaryWindow>>,
        controller: bool,
        assets: &Res<PlayerAssets>,
        guns_assets: &Res<GunAssets>
    ) {
        let mut state_machine = AnimationStateMachine::new();

        state_machine.insert(
            PlayerState::Idle,
            (
                assets.idle.clone(),
                AnimationIndices { first: 0, last: 3 },
                AnimationFlip::False,
            ),
        );
        state_machine.insert(
            PlayerState::LeftFront,
            (
                assets.side_front.clone(),
                AnimationIndices { first: 0, last: 5 },
                AnimationFlip::XAxis,
            ),
        );
        state_machine.insert(
            PlayerState::RightFront,
            (
                assets.side_back.clone(),
                AnimationIndices { first: 0, last: 5 },
                AnimationFlip::False,
            ),
        );
        state_machine.insert(
            PlayerState::LeftBack,
            (
                assets.side_back.clone(),
                AnimationIndices { first: 0, last: 5 },
                AnimationFlip::XAxis,
            ),
        );
        state_machine.insert(
            PlayerState::RightBack,
            (
                assets.back.clone(),
                AnimationIndices { first: 0, last: 5 },
                AnimationFlip::False,
            ),
        );
        state_machine.insert(
            PlayerState::Front,
            (
                assets.front.clone(),
                AnimationIndices { first: 0, last: 5 },
                AnimationFlip::False,
            ),
        );
        state_machine.insert(
            PlayerState::Back,
            (
                assets.back.clone(),
                AnimationIndices { first: 0, last: 5 },
                AnimationFlip::False,
            ),
        );

        let gun_id = commands.spawn(GunBundle::setup(guns_assets)).id();

        let player = PlayerBundle {
            name: bevy::core::Name::new("Player"),
            state: AnimationState::new(&PlayerState::Idle),
            sprite: SpriteSheetBundle {
                texture_atlas: assets.idle.clone(),
                sprite: TextureAtlasSprite {
                    index: 0,
                    anchor: bevy::sprite::Anchor::TopLeft,
                    ..default()
                },
                ..default()
            },
            state_machine,
            player: PlayerStats::default(),
            player_action: input::player_input_setup(),
            player_offset: Offset(Vec2::new(17. / 2., 25. / 2. + 8.)),
            zindex: Zindex(25.),
            player_position: Position(Vec2::ZERO),
            current_gun: GunEntity(gun_id),
            collider: TesselatedCollider {
                texture: assets.collider.clone(),
                offset: Vec2::ZERO,
            },
        };
        if controller {
            commands.spawn(player).insert(IsController);
        } else {
            let player_id = commands
                .spawn(player)
                .insert(InputManagerBundle::<Mouse>::default())
                .id();

            commands.entity(window.single()).insert(ActionStateDriver {
                action: crate::mouse::Mouse::MousePosition,
                targets: player_id.into(),
            });
        }
    }
}

use std::f32::INFINITY;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle, math::Vec3Swizzles};
use leafwing_input_manager::prelude::ActionState;
use rand::Rng;
use strum::IntoEnumIterator;

use crate::{
    rendering::outline::Outline,
    player::{
        input::PlayerActions,
        inventory::{inventory_manager::Inventory, item_manager::Items},
        stats::PlayerStats,
    },
    rendering::utils::Zindex,
};

use super::{assets::ItemsAssets, PickupEvent};

const PICKUP_RANGE: f32 = 25. * 1.5;

pub fn update_pickup(
    time: Res<Time>,
    mut ev_pickup: EventWriter<PickupEvent>,
    mut commands: Commands,
    mut materials: ResMut<Assets<Outline>>,
    mut pickups: Query<(
        Entity,
        &Handle<Outline>,
        &mut Transform,
        &mut Pickup,
        &mut Zindex,
        Without<PlayerStats>,
    )>,
    mut players: Query<(
        Entity,
        &mut Transform,
        &mut Inventory,
        &ActionState<PlayerActions>,
        With<PlayerStats>,
    )>,
) {
    for (_, outline, mut pos, pickup, mut zindex, _) in &mut pickups {
        let float = ((time.elapsed_seconds() + pickup.anim_offset) * 3.).sin() / 10.;
        pos.translation.y += float;
        zindex.0 = float + 5.;

        if let Some(material) = materials.get_mut(outline) {
            material.color = Color::WHITE.with_a(0.);
        }
    }

    for (entity, player_pos, mut inventory, actions, _) in &mut players {
        let mut nearest: Option<Entity> = None;
        let mut distance: f32 = INFINITY;

        for (entity, _, pos, _, _, _) in &mut pickups {
            let current_distance = pos.translation.xy().distance(player_pos.translation.xy());

            if current_distance < distance && current_distance < PICKUP_RANGE {
                distance = current_distance;
                nearest = Some(entity);
            }
        }

        if let Some(valid_pickup) = nearest {
            if let Ok((_, outline, _, pickup, _, _)) = pickups.get(valid_pickup) {
                if let Some(material) = materials.get_mut(outline) {
                    material.color = Color::WHITE;
                }
                if actions.just_pressed(PlayerActions::Pickup) {
                    match &pickup.pickup_type {
                        PickupType::Weapon => todo!(),
                        PickupType::Item(item) => {
                            ev_pickup.send(PickupEvent(*item, entity));
                            inventory.add(*item);
                        }
                    }
                    commands.entity(valid_pickup).despawn_recursive();
                }
            }
        }
    }
}

pub fn spawn_items(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Outline>>,
    assets: Res<ItemsAssets>,
) {
    let len = Items::iter().count();
    for (x, item) in Items::iter().enumerate() {
        for _ in 0..10 {
            commands.spawn(item.to_pickup(
                Vec2::new(-(len as f32 * 30.) / 2. + x as f32 * 30. + 15., 80.),
                &mut meshes,
                &mut materials,
                &assets,
            ));
        }
    }
}

pub enum PickupType {
    Weapon,
    Item(Items),
}

#[derive(Component)]
pub struct Pickup {
    pub anim_offset: f32,
    pub pickup_type: PickupType,
}

#[derive(Bundle)]
pub struct PickupBundle {
    pub name: bevy::core::Name,
    pub material: MaterialMesh2dBundle<Outline>,
    pub zindex: Zindex,
    pub pickup: Pickup,
}

impl PickupBundle {
    pub fn create(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<Outline>>,
        sprite: Handle<Image>,
        size: Vec2,
        name: String,
        pos: Vec2,
        item_type: Items,
    ) -> PickupBundle {
        let mut rng = rand::thread_rng();
        let place_rng = rng.gen::<f32>() * 100.;

        PickupBundle {
            name: bevy::core::Name::new(name),
            material: MaterialMesh2dBundle {
                transform: Transform::default()
                    .with_scale(size.extend(0.))
                    .with_translation(pos.floor().extend(0.)),
                mesh: meshes
                    .add(Mesh::from(shape::Quad::new(Vec2::splat(2.))))
                    .into(),
                material: materials.add(Outline {
                    color: Color::WHITE,
                    size,
                    thickness: 1.,
                    color_texture: sprite,
                }),
                ..default()
            },
            zindex: Zindex(0.),
            pickup: Pickup {
                anim_offset: place_rng,
                pickup_type: PickupType::Item(item_type),
            },
        }
    }
}

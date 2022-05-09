// Bevy
#![allow(clippy::type_complexity)]
#![feature(adt_const_params)]

use bevy::{
    prelude::*,
    render::{camera::WindowOrigin, view::RenderLayers},
    window::WindowMode,
};

use comp::{
    ai::{PlayerAI, RandomAI},
    beings::{self, BasicMonsterBundle},
    display::Renderable,
    entity::{ActiveTurn, Being, Mana, TurnTaker},
    Position, Walkable,
};

use crate::rng::Random;

pub mod comp;
pub mod event;
pub mod map;
pub mod rng;
pub mod util;

/// The current turn #
#[derive(Debug, Clone, Copy)]
struct TurnCount(pub u64);

fn main() {
    const PART_TURN_INIT: &str = "part_turn_init";
    const PART_TURN_PERFORM: &str = "part_turn_perform";

    App::new()
        .insert_resource(WindowDescriptor {
            width: 800.0,
            height: 600.0,
            title: "Thaumafunge".to_string(),
            resizable: false,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_plugins(DefaultPlugins)
        // Keep track of the current turn
        .insert_resource(TurnCount(0))
        .add_system(util::set_texture_filters_to_nearest)
        // Initialize everything
        .add_startup_system(setup)
        // Move the transform to their position within the grid
        .add_system(update_transform_from_position)
        // Perform part turn initialization
        .add_stage_before(
            CoreStage::PostUpdate,
            PART_TURN_INIT,
            SystemStage::single_threaded(),
        )
        // Remove active entity so that we can set the new one
        .add_system(remove_active_entity.label("remove_active_entity"))
        // Check if we need to start a new turn
        .add_system(
            check_init_new_turn
                .label("check_init_new_turn")
                .after("remove_active_entity"),
        )
        // Find the current active entity
        .add_system(
            set_active_entity
                .label("set_active_entity")
                .after("check_init_new_turn"),
        )
        // Perform part turn on the active entity
        // TODO: parallel?
        .add_stage_after(
            PART_TURN_INIT,
            PART_TURN_PERFORM,
            SystemStage::single_threaded(),
        )
        .add_system(update_player)
        .add_system(update_random_ai)
        .run();
}

// TODO: We might be able to use `Changed` to make this cheaper?
// TODO: Should we include a size parameter? Currently everything is 16x16 but we might want
// entities that are larger
fn update_transform_from_position(
    mut query: Query<(&mut Transform, &Position), Changed<Position>>,
) {
    for (mut transform, position) in query.iter_mut() {
        *transform = transform.with_translation(Vec3::new(
            position.0.x as f32 * 16.0 + 8.0,
            position.0.y as f32 * 16.0 + 8.0,
            0.0,
        ));
    }
}

fn remove_active_entity(mut commands: Commands, query: Query<Entity, With<ActiveTurn>>) {
    // TODO: is it correct to remove the ActiveTurn component here?
    // it might be re-added to the same entity in set_active_entity
    for entity in query.iter() {
        commands.entity(entity).remove::<ActiveTurn>();
    }
}

fn check_init_new_turn(mut turn: ResMut<TurnCount>, mut query: Query<&mut TurnTaker>) {
    let mut is_all_empty = true;
    for taker in query.iter() {
        if taker.energy > 0 {
            is_all_empty = false;
        }
    }

    if is_all_empty {
        turn.0 += 1;
        for mut taker in query.iter_mut() {
            taker.energy += taker.max_energy;
        }
    }
}

fn set_active_entity(mut commands: Commands, query: Query<(Entity, &TurnTaker)>) {
    for (entity, taker) in query.iter() {
        if taker.energy != 0 {
            commands.entity(entity).insert(ActiveTurn);
            break;
        }
    }
}

fn update_player(
    keys: Res<Input<KeyCode>>,
    mut query_player: Query<
        (Entity, &mut TurnTaker, &mut Position, &mut Mana),
        (With<PlayerAI>, With<ActiveTurn>),
    >,
    // mut other_query: Query<&Position, Without<Walkable>>,
) {
    if let Ok((_entity, mut turn_taker, mut position, _mana)) = query_player.get_single_mut() {
        if keys.any_pressed([KeyCode::W, KeyCode::S, KeyCode::A, KeyCode::D]) {
            if keys.pressed(KeyCode::W) {
                position.0.y += 1;
            }

            if keys.pressed(KeyCode::S) {
                position.0.y -= 1;
            }

            if keys.pressed(KeyCode::A) {
                position.0.x -= 1;
            }

            if keys.pressed(KeyCode::D) {
                position.0.x += 1;
            }

            turn_taker.energy = turn_taker.energy.saturating_sub(30);
        }
    }
}

fn update_random_ai(
    mut query: QuerySet<(
        QueryState<
            (
                Entity,
                &mut RandomAI,
                &mut TurnTaker,
                &mut Position,
                Option<&mut Mana>,
            ),
            With<ActiveTurn>,
        >,
        QueryState<(Entity, &Position), Without<Walkable>>,
    )>,
) {
    let mut offset_x = 0;
    let mut offset_y = 0;
    for (_entity, mut ai, mut turn_taker, mut position, _mana) in query.q0().iter_mut() {
        // We use turn count as very pseudo random number generator
        let rand: u8 = ai.rand.gen::<u8>() % 4;
        if rand == 0 {
            offset_y += 1;
        } else if rand == 1 {
            offset_y -= 1;
        } else if rand == 2 {
            offset_x += 1;
        } else if rand == 3 {
            offset_x -= 1;
        }
    }

    for (_, _, mut turn_taker, _, _) in query.q0().iter_mut() {
        turn_taker.energy = turn_taker.energy.saturating_sub(30);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Initialize the sprite sheet
    let texture_handle = asset_server.load("textures/Cheepicus_16x16.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle.clone(), Vec2::new(16.0, 16.0), 16, 16);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // Spawn a camera
    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    // Move origin to bottom left since that is easier to use
    camera_bundle.orthographic_projection.window_origin = WindowOrigin::BottomLeft;
    commands.spawn_bundle(camera_bundle);

    // Create the player
    let _player = commands
        .spawn()
        .insert(Position::new(1, 1))
        .insert(TurnTaker::new(60))
        .insert(Mana::new(100))
        .insert(Being {})
        .insert(PlayerAI)
        .insert_bundle(Renderable::from_index(
            texture_atlas_handle.clone(),
            64,
            Color::YELLOW,
        ))
        .id();

    // Create very dumb enemy
    let _enemy1 = commands
        .spawn_bundle(beings::new_zombie(
            texture_atlas_handle.clone(),
            Position::new(15, 15),
        ))
        .id();

    // Create a bunch of tiles
    for x in 0..10 {
        for y in 0..10 {
            commands
                .spawn()
                .insert(Position::new(x, y))
                .insert_bundle(Renderable::from_index(
                    texture_atlas_handle.clone(),
                    192 - 16,
                    Color::BLACK,
                ))
                .insert(Walkable);

            if x == 0 || y == 0 {
                commands
                    .spawn()
                    .insert(Position::new(x, y))
                    .insert_bundle(Renderable::from_index(
                        texture_atlas_handle.clone(),
                        35,
                        Color::GRAY,
                    ));
            }
        }
    }
}

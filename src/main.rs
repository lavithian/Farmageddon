use bevy::{prelude::*, transform::commands, window::PrimaryWindow, render::view::window};

fn main() {
  App::new()
    .add_plugins(
      DefaultPlugins
      .set(ImagePlugin::default_nearest())
      .set(WindowPlugin {
        primary_window: Some(Window {
          title: "Farmageddon".into(),
          resolution: (800.0, 800.0).into(),
          resizable: false,
          ..default()
        }),
        ..default()
      })
      .build(),
    )
    // .add_systems(Startup, setup)
    .add_systems(Startup, (spawn_camera, spawn_player))
    .add_systems(Update, (character_movement, confine_player_movement))
    .run();
}

// fn setup(
//   mut commands: Commands,
//   asset_server: Res<AssetServer>,
//   window_query: Query<&Window, With<PrimaryWindow>>,
// ) {
//   // let texture = asset_server.load("temp/char.png");
//   // let texture = asset_server.load("char.png");
//   let window = window_query.get_single().unwrap();

//   commands.spawn(
//     Camera2dBundle {
//     // camera_2d: Camera2d {
//     //   clear_color: ClearColorConfig::Custom(Color::GREEN),
//     // },
//       transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
//       ..default()
//     }
//   );

//   commands.spawn((
//     SpriteBundle {
//       sprite: Sprite {
//         custom_size: Some(Vec2::new(16.0, 16.0)),
//         ..default()
//       },
//       // where it spawns:
//       transform: Transform::from_xyz(window.width() /  2.0, window.height(), 0.0),
//       texture: asset_server.load("temp/char.png"),
//       ..default()
//     },
//     Player {speed: 100.0},
//   ));
//   println!("width: {}, height: {}", window.width(), window.height());
// }

fn spawn_camera(
  mut commands: Commands,
  window_query: Query<&Window, With<PrimaryWindow>>,
) {
  let window = window_query.get_single().unwrap();
  info!("width: {}, height: {}", window.width(), window.height());
  commands.spawn(
    Camera2dBundle {
    // camera_2d: Camera2d {
    //   clear_color: ClearColorConfig::Custom(Color::GREEN),
    // },
      transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
      ..default()
    }
  );
}

fn spawn_player(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  window_query: Query<&Window, With<PrimaryWindow>>,
) {

  let window = window_query.get_single().unwrap();
  info!("width: {}, height: {}", window.width(), window.height());
  commands.spawn((
    SpriteBundle {
      sprite: Sprite {
        custom_size: Some(Vec2::new(16.0, 16.0)),
        ..default()
      },
      // where it spawns:
      transform: Transform::from_xyz(window.width() /  2.0, window.height(), 0.0),
      texture: asset_server.load("temp/char.png"),
      ..default()
    },
    Player {speed: 100.0},
  ));
  println!("width: {}, height: {}", window.width(), window.height());
}

// fn character_movement(
//   mut characters: Query<(&mut Transform, &Player)>,
//   input: Res<Input<KeyCode>>,
//   time: Res<Time>,
// ) {
//   for (mut transform, player) in &mut characters {
//     let movement_amount = player.speed * time.delta_seconds();

//     if input.pressed(KeyCode::W) {
//         transform.translation.y += movement_amount;
//     }
//     if input.pressed(KeyCode::S) {
//         transform.translation.y -= movement_amount;
//     }
//     if input.pressed(KeyCode::D) {
//         transform.translation.x += movement_amount;
//     }
//     if input.pressed(KeyCode::A) {
//         transform.translation.x -= movement_amount;
//     }
//   }
// }

pub const PLAYER_SPEED: f32 = 100.0;
pub const PLAYER_SIZE: f32 = 16.0;

fn character_movement (
  mut player_query: Query<&mut Transform, With<Player>>,
  // mut player_speed: Query<&Player>,
  input: Res<Input<KeyCode>>,
  time: Res<Time>,
) {
  if let Ok(mut transform) = player_query.get_single_mut() {
    let mut direction = Vec3::ZERO;
    if input.pressed(KeyCode::W) || input.pressed(KeyCode::Up) {
      direction += Vec3::new(0.0, 1.0, 0.0)
    }
    if input.pressed(KeyCode::A) || input.pressed(KeyCode::Left) {
      direction += Vec3::new(-1.0, 0.0, 0.0)
    }
    if input.pressed(KeyCode::S) || input.pressed(KeyCode::Down) {
      direction += Vec3::new(0.0, -1.0, 0.0)
    }
    if input.pressed(KeyCode::D) || input.pressed(KeyCode::Right) {
      direction += Vec3::new(1.0, 0.0, 0.0)
    }
    if direction.length() > 0.0 {
      direction = direction.normalize();
    }

    transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    // transform.translation += direction * player_speed.speed * time.delta_seconds();
  }
}

pub fn confine_player_movement (
  mut player_query: Query<&mut Transform, With<Player>>,
  window_query: Query<&Window, With<PrimaryWindow>>,
) {
  if let Ok(mut player_transform) = player_query.get_single_mut() {
    let window = window_query.get_single().unwrap();

    let half_player_size: f32 = PLAYER_SIZE / 2.0;
    let x_min: f32 = 0.0 + half_player_size;
    let x_max: f32 = window.width() - half_player_size;
    let y_min: f32 = 0.0 + half_player_size;
    let y_max: f32 = window.height() - half_player_size;
    let mut translation = player_transform.translation;

    // Bound
    if translation.x < x_min {
      translation.x = x_min;
    } else if translation.x > x_max {
      translation.x = x_max;
    }

    if translation.y < y_min {
      translation.y = y_min;
    } else if translation.y > y_max {
      translation.y = y_max;
    }

    player_transform.translation = translation;
  }
}

#[derive(Component)]
pub struct Player {
  pub speed: f32,
}

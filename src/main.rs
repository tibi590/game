use bevy::prelude::*;

const SPEED: f32 = 200.;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("#141E46").unwrap()))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, movement)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::hex("#FFF5E0").unwrap(),
            custom_size: Some(Vec2::new(10.0, 10.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    },Player));
        
}

fn movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut sprite_position: Query<&mut Transform, With<Player>>,
    ) {
    for mut transform in &mut sprite_position {
        let mut direction: Vec3 = Vec3::ZERO;

        if keys.pressed(KeyCode::A) {
            direction += Vec3::new(-SPEED, 0., 0.);
        }
        if keys.pressed(KeyCode::W) {
            direction += Vec3::new(0., SPEED, 0.);
        }
        if keys.pressed(KeyCode::S) {
            direction += Vec3::new(0., -SPEED, 0.);
        }
        if keys.pressed(KeyCode::D) {
            direction += Vec3::new(SPEED, 0., 0.);
        }

        transform.translation += direction * time.delta_seconds();
    }
}

#[derive(Component)]
struct Player;

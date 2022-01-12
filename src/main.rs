use bevy::{
    prelude::*,
};
use rand::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(paddle_movement_system)
        .run();
}

#[derive(Component)]
struct Paddle {
    speed: f32,
}

fn setup(
    mut commands: Commands
) {
    // cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
    // paddle
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, -215.0, 0.0),
                scale: Vec3::new(120.0, 30.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Paddle { speed: 500.0 });
}

fn paddle_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Paddle, &mut Transform, &mut Sprite)>,
) {
    for (paddle, mut transform, mut sprite) in query.iter_mut() {
        let mut direction_x = 0.0;
        let mut direction_y = 0.0;

        if keyboard_input.pressed(KeyCode::Left) {
            direction_x -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction_x += 1.0;
        }

        if keyboard_input.pressed(KeyCode::Down) {
            direction_y -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            direction_y += 1.0;
        }

        if keyboard_input.just_pressed(KeyCode::Space) {
            let mut rng = rand::thread_rng();
            sprite.color = Color::rgb(rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0))
        }

        let translation = &mut transform.translation;
        // move the paddle horizontally
        translation.x += time.delta_seconds() * direction_x * paddle.speed;
        // bound the paddle
        translation.x = translation.x.min(380.0).max(-380.0);

        translation.y += time.delta_seconds() * direction_y * paddle.speed;
        translation.y = translation.y.min(200.0).max(-200.0);
        
    }
}
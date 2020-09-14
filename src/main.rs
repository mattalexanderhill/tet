use bevy::{
    prelude::*,
    render::pass::ClearColor,
    sprite::collide_aabb::{collide, Collision},
};
use tetronimo::Tetronimo;

mod tetronimo;

const BLOCK_WIDTH: f32 = 20.0;

const TICK_TIMERS_S: [f32; 9] = [
    2.0,
    1.8,
    1.6,
    1.4,
    1.2,
    1.0,
    0.8,
    0.6,
    0.2,
];

struct GameConfig {
    background: Color,
    foreground: Color,
    window: WindowDesciptor,
}

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig {
            background: Color::rgb(0.70, 0.45, 0.75),
            foreground: Color::rgb(0.75, 0.82, 0.92),
            window: WindowDesciptor {
                title: "Tet.rs",
                width: 240,
                height: 400,


(200.0, 360.0);


        WindowDescriptor {
            title: "bevy".to_string(),
            width: 1280,
            height: 720,
            vsync: true,
            resizable: true,
            mode: WindowMode::Windowed,
            __non_exhaustive: (),
        }
            }
        }
    }
}

struct GameTimer(Timer);

fn main() {
    let cfg = GameConfig::default();

    App::build()
        .add_resource()
        .add_default_plugins()
        .add_resource(Scoreboard { score: 0 })
        .add_resource(cfg.foreground)
        .add_startup_system(setup_sys.system())
        .add_system(move_sys.system())
        .add_system(score_sys.system())
        .add_system(collision_sys.system())
        .run();
}

struct Scoreboard {
    score: usize,
}

enum Collider {
    Solid,
    Scorable,
}

fn setup_sys(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let cfg = GameConfig::default();

    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        .spawn(TextComponents {
            text: Text {
                font: asset_server.load("assets/fonts/FiraSans-Bold.ttf").unwrap(),
                value: "Score:".to_string(),
                style: TextStyle {
                    color: Color::rgb(0.2, 0.2, 0.8),
                    font_size: 40.0,
                },
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        });

    let wall_material = materials.add(cfg.background.into());
    let wall_thickness = 10.0;
    let bounds = Vec2::new(200.0, 360.0);

    commands
        .spawn(SpriteComponents {
            material: wall_material,
            translation: Translation(Vec3::new(-bounds.x() / 2.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y() + wall_thickness)),
            ..Default::default()
        })
        .with(Collider::Solid)
        .spawn(SpriteComponents {
            material: wall_material,
            translation: Translation(Vec3::new(bounds.x() / 2.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y() + wall_thickness)),
            ..Default::default()
        })
        .with(Collider::Solid)
        .spawn(SpriteComponents {
            material: wall_material,
            translation: Translation(Vec3::new(0.0, -bounds.y() / 2.0, 0.0)),
            sprite: Sprite::new(Vec2::new(bounds.x() + wall_thickness, wall_thickness)),
            ..Default::default()
        })
        .with(Collider::Solid)
        .spawn(SpriteComponents {
            material: wall_material,
            translation: Translation(Vec3::new(0.0, bounds.y() / 2.0, 0.0)),
            sprite: Sprite::new(Vec2::new(bounds.x() + wall_thickness, wall_thickness)),
            ..Default::default()
        })
        .with(Collider::Solid);
}

fn move_sys(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Tetronimo, &mut Translation)>,
) {
    for (tetronimo, mut translation) in &mut query.iter() {
        let mut direction = 0.0;
        if keyboard_input.pressed(KeyCode::Left) {
            direction -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction += 1.0;
        }

        *translation.0.x_mut() += direction * BLOCK_WIDTH;

        // bound the paddle within the walls
        *translation.0.x_mut() = f32::max(-380.0, f32::min(380.0, translation.0.x()));
    }
}

// fn update() {
//             .spawn(SpriteComponents {
//             material: materials.add(new_tet.color().into()),
//             translation: Translation(Vec3::new(0.0, -215.0, 0.0)),
//             sprite: Sprite::new(Vec2::new(120.0, 30.0)),
//             ..Default::default()
//         })
//         .with(Tetronimo::random())
//         .with(Collider::Solid)
// }



fn score_sys(scoreboard: Res<Scoreboard>, mut query: Query<&mut Text>) {
    for mut text in &mut query.iter() {
        text.value = format!("Score: {}", scoreboard.score);
    }
}

// fn collision_sys(
//     mut commands: Commands,
//     mut scoreboard: ResMut<Scoreboard>,
//     mut ball_query: Query<(&mut Ball, &Translation, &Sprite)>,
//     mut collider_query: Query<(Entity, &Collider, &Translation, &Sprite)>,
// ) {
//     for (mut ball, ball_translation, sprite) in &mut ball_query.iter() {
//         let ball_size = sprite.size;
//         let velocity = &mut ball.velocity;

//         // check collision with walls
//         for (collider_entity, collider, translation, sprite) in &mut collider_query.iter() {
//             let collision = collide(ball_translation.0, ball_size, translation.0, sprite.size);
//             if let Some(collision) = collision {
//                 // scorable colliders should be despawned and increment the scoreboard on collision
//                 if let Collider::Scorable = *collider {
//                     scoreboard.score += 1;
//                     commands.despawn(collider_entity);
//                 }

//                 // reflect the ball when it collides
//                 let mut reflect_x = false;
//                 let mut reflect_y = false;

//                 // only reflect if the ball's velocity is going in the opposite direction of the collision
//                 match collision {
//                     Collision::Left => reflect_x = velocity.x() > 0.0,
//                     Collision::Right => reflect_x = velocity.x() < 0.0,
//                     Collision::Top => reflect_y = velocity.y() < 0.0,
//                     Collision::Bottom => reflect_y = velocity.y() > 0.0,
//                 }

//                 // reflect velocity on the x-axis if we hit something on the x-axis
//                 if reflect_x {
//                     *velocity.x_mut() = -velocity.x();
//                 }

//                 // reflect velocity on the y-axis if we hit something on the y-axis
//                 if reflect_y {
//                     *velocity.y_mut() = -velocity.y();
//                 }

//                 break;
//             }
//         }
//     }
// }

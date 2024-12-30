use bevy::prelude::*;
use avian2d::prelude::*;

#[derive(Component, Default)]
#[require(
    Sprite,
    RigidBody(|| RigidBody::Dynamic),
    LinearVelocity,
    AngularVelocity,
    Collider(|| Collider::circle(1.0)),
    GravityScale(|| 0.0)
)]
pub struct Ship {
    pub velocity: Vec2,
}

pub fn add_ship(
    mut commands: Commands,
    server: Res<AssetServer>,
)
{
    let image = server.load::<Image>("ship.png");

    commands.spawn(Ship::default())
        .insert(Sprite::from_image(image))
        .insert(Transform::from_scale(Vec3::new(0.1,0.1,0.1)));
}

pub fn ship_keyboard_move(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Transform, &mut LinearVelocity, &mut AngularVelocity), With<Ship>>,
    time: Res<Time>,
) {
    let mut movement = Vec2::default();
    if keys.pressed(KeyCode::KeyW)
        { movement.y += 1.0; }
    if keys.pressed(KeyCode::KeyS)
        { movement.y -= 1.0; }
    if keys.pressed(KeyCode::KeyA)
        { movement.x -= 1.0; }
    if keys.pressed(KeyCode::KeyD)
        { movement.x += 1.0; }

    //movement *= time.delta_secs();

    let (trns, mut linear, mut angular) = query.get_single_mut().unwrap();

    let direction = Vec2::from_angle(trns.rotation.z);
    println!("{}", Vec3::from(trns.rotation.to_euler(EulerRot::XYZ)));

    linear.0 += movement.y * direction;
    angular.0 += movement.x.to_radians();
}

pub struct ShipPlugin;

impl Plugin for ShipPlugin
{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_ship)
            .add_systems(Update, ship_keyboard_move);
    }
}
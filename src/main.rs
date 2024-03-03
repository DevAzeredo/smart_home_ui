//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(RapierConfiguration {
            gravity: Vec3::new(0., -10., 0.),
            ..default()
        })
        .add_systems(Startup, setup)
        .add_systems(Update, mouse_click_system)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let circular_base = PbrBundle {
        mesh: meshes.add(Circle::new(10.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    };
    let light = PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    };
    let camera = Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };

    let car = SceneBundle {
        scene: asset_server.load("free_1975_porsche_911_930_turbo.glb#Scene0"),
        ..default()
    };

    commands
        .spawn(RigidBody::Fixed)
        .insert(circular_base)
        .insert(Collider::cuboid(10., 10., 0.1));
    commands.spawn(light);
    commands.spawn(camera);
    //commands
      //  .spawn(car)
       // .insert(Collider::cuboid(1.3, 1., 1.3,))
        //.insert(RigidBody::Fixed);
}

fn mouse_click_system(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let car = SceneBundle {
            scene: asset_server.load("free_1975_porsche_911_930_turbo.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 2.0, 0.0),
            ..default()
        };

        commands
            .spawn(RigidBody::Dynamic)
            .insert(car)
            .insert(Collider::cuboid(1.3, 0.5, 1.3,));
        // Changes the scale of our gravity for this entity
        info!("RENDERIZANDO");
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        info!("left mouse just pressed");
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        info!("left mouse just released");
    }
}

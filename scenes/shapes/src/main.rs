//! A test scene that only uses primitive shapes.

use avian_navmesh::NavMeshPlugin;
use avian3d::prelude::*;
use bevy::{
    color::palettes::tailwind,
    prelude::*,
    remote::{RemotePlugin, http::RemoteHttpPlugin},
};

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((PhysicsPlugins::default(), PhysicsDebugPlugin::default()))
        .add_plugins((RemotePlugin::default(), RemoteHttpPlugin::default()))
        .add_plugins(NavMeshPlugin::default())
        .add_systems(Startup, setup)
        .add_observer(configure_camera)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material_gray = materials.add(Color::from(tailwind::GRAY_300));
    let material_red = materials.add(Color::from(tailwind::RED_500));
    let shape = Plane3d::new(Vec3::Y, Vec2::splat(25.0));
    commands.spawn((
        Name::new("Ground"),
        Mesh3d(meshes.add(shape)),
        RigidBody::Static,
        Collider::from(shape),
        MeshMaterial3d(material_gray.clone()),
        ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
    ));
    let shape = Cuboid::new(3.0, 2.0, 1.0);
    commands.spawn((
        Name::new("Cube"),
        Mesh3d(meshes.add(shape)),
        RigidBody::Static,
        Collider::from(shape),
        Transform::from_xyz(0.0, 1.0, 0.0),
        MeshMaterial3d(material_gray.clone()),
        ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
    ));
    let shape = Cuboid::new(1.0, 2.0, 3.0);
    commands.spawn((
        Name::new("Cube"),
        Mesh3d(meshes.add(shape)),
        RigidBody::Static,
        Collider::from(shape),
        Transform::from_xyz(-4.0, 1.0, 5.0),
        MeshMaterial3d(material_gray.clone()),
        ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
    ));

    let shape = Cuboid::new(5.0, 1.0, 5.0);
    commands.spawn((
        Name::new("Cube"),
        Mesh3d(meshes.add(shape)),
        RigidBody::Static,
        Collider::from(shape),
        Transform::from_xyz(10.0, 3.0, 3.0),
        MeshMaterial3d(material_red.clone()),
        ColliderConstructorHierarchy::new(ColliderConstructor::TrimeshFromMesh),
    ));
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::default().looking_to(Vec3::new(0.5, -1.0, 0.3), Vec3::Y),
    ));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(10.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn configure_camera(
    trigger: Trigger<OnAdd, Camera>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands
        .entity(trigger.target())
        .insert(EnvironmentMapLight {
            diffuse_map: asset_server.load("environment_maps/voortrekker_interior_1k_diffuse.ktx2"),
            specular_map: asset_server
                .load("environment_maps/voortrekker_interior_1k_specular.ktx2"),
            intensity: 2000.0,
            ..default()
        });
}

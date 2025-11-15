mod shared;
use bevy::prelude::*;
use bevy_virtual_cameras::prelude::*;


fn main() {
    let mut app = shared::get_app();
    app
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let (_red, blue) = shared::setup(&mut commands, &mut *meshes, &mut *materials);

    // 1️⃣ Spawn a camera entity
    let camera_entity = commands
        .spawn((
            Camera3d::default(),
            Transform::IDENTITY,
        ))
        .id();

    // 2️⃣ Spawn a director entity
    let director_entity = commands
        .spawn(Director::new(camera_entity))
        .id();

    // 3️⃣ Spawn a virtual camera that follows the blue target
    commands.spawn((
        VirtualCamera {
            director: director_entity,
            priority: 1,
            blend_in: CameraBlendDefinition::default(),
        },
        Transform::IDENTITY,
        Projection::Perspective(PerspectiveProjection { fov: 1., aspect_ratio: 1.5, near: 0.1, far: 100. }),
        FollowTarget::Single {
            target: blue,
            offset: Vec3::Z * 2.,
            damping: 10.0,
        },
    ));

}
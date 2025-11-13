mod shared;
use bevy::{input::mouse::{AccumulatedMouseMotion}, prelude::*};
use bevy_virtual_cameras::prelude::*;

// Import your virtual camera modules

fn main() {
    let mut app = shared::get_app();
    app
        .add_systems(Startup, setup)
        .add_systems(Update, input)
        .run();
}

fn input (
    mut cam: Query<&mut FreeLookCamera>,
    input: Res<AccumulatedMouseMotion>,
    time: Res<Time>,
) {
    let Ok(mut cam) = cam.single_mut() else { return };
    let delta = input.delta;
    const LOOK_SPEED: f32 = 0.1;
    cam.pitch += delta.y * time.delta_secs() * LOOK_SPEED;
    cam.yaw += delta.x * time.delta_secs() * LOOK_SPEED;
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let (_red, _blue) = shared::setup(&mut commands, &mut *meshes, &mut *materials);

    // 1️⃣ Spawn a camera entity
    let camera_entity = commands
        .spawn((
            Camera3d::default(),
            Transform::IDENTITY,
            GlobalTransform::default(),
        ))
        .id();

    // 2️⃣ Spawn a director entity
    let director_entity = commands
        .spawn(Director::new(camera_entity))
        .id();

    // 3️⃣ Spawn a virtual camera that looks at the red target
    commands.spawn((
        VirtualCamera {
            director: director_entity,
            priority: 1,
            blend_in: CameraBlendDefinition::default(),
        },
        Transform::IDENTITY,
        Projection::Perspective(PerspectiveProjection { fov: 1., aspect_ratio: 1.5, near: 0.1, far: 100. }),
        FreeLookCamera {
            pitch_limit: 1.5, // Little bit less than PI / 2 up and down
            ..default()
        },
    ));

}
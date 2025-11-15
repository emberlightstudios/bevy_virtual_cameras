mod shared;
use bevy::prelude::*;
use bevy_virtual_cameras::prelude::*;

fn main() {
    let mut app = shared::get_app();
    app
        .add_systems(Startup, setup)
        .add_systems(Update, input)
        .run();
}

fn input(
    input: Res<ButtonInput<KeyCode>>,
    mut orbit: Query<&mut OrbitCamera>,
    time: Res<Time>,
) {
    const ORBIT_SPEED: f32 = 2.0;
    let Ok(mut orbit) = orbit.single_mut() else { return };
    if input.pressed(KeyCode::KeyD) {
        orbit.yaw += time.delta_secs() * ORBIT_SPEED;
    }
    if input.pressed(KeyCode::KeyA) {
        orbit.yaw -= time.delta_secs() * ORBIT_SPEED;
    }
    if input.pressed(KeyCode::KeyW) {
        orbit.pitch += time.delta_secs() * ORBIT_SPEED;
    }
    if input.pressed(KeyCode::KeyS) {
        orbit.pitch -= time.delta_secs() * ORBIT_SPEED;
    }
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

    // 3️⃣ Spawn a virtual camera that orbits the blue target
    commands.spawn((
        VirtualCamera {
            director: director_entity,
            priority: 1,
            blend_in: CameraBlendDefinition::default(),
        },
        Transform::IDENTITY,
        Projection::Perspective(PerspectiveProjection { fov: 1., aspect_ratio: 1.5, near: 0.1, far: 100. }),
        OrbitCamera {
            target: blue,
            radius: 5.,
            offset: Vec3::X,
            ..default()
        }
    ));

}
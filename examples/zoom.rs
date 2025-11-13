mod shared;
use bevy::prelude::*;
use bevy_virtual_cameras::prelude::*;

// Import your virtual camera modules

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
    let (red, blue) = shared::setup(&mut commands, &mut *meshes, &mut *materials);

    // 1️⃣ Spawn a camera entity
    let camera_entity = commands
        .spawn((
            Camera3d::default(),
            Transform::from_translation(Vec3::new(0., 5., 2.)),
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
        Transform::from_translation(Vec3::Z * 5.),
        Projection::Perspective(PerspectiveProjection { fov: 1., aspect_ratio: 1.5, near: 0.1, far: 100. }),
        GroupZoom {
            targets: vec![red, blue],
            damping: 0.5,
            dead_zone: DeadZone { xmin: -0.5, xmax: 0.5, ymin: -0.5, ymax: 0.5 },
            min_scale: 2.,
            ..default()
        }
    ));

}
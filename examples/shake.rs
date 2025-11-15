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
    mut cam: Query<Entity, With<VirtualCamera>>,
    mut writer: MessageWriter<AddCameraShake>,
) {
    let Ok(cam) = cam.single_mut() else { return };
    if input.just_pressed(KeyCode::Space) {
        writer.write(AddCameraShake {
            vcam_entity: cam,
            camera_shake: CameraShake {
                timer: Timer::from_seconds(1.5, TimerMode::Once),
                translation_intensity: Vec3::new(0.1, 0.2, 0.3),
                rotation_intensity: Vec3::new(0., 0., 0.1),
                translation_frequency: Vec3::new(10., 8., 11.),
                rotation_frequency: Vec3::new(10., 9., 12.),
                damping: 1.0,
                ..default()
            }
        });
    }
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
        ))
        .id();

    // 2️⃣ Spawn a director entity
    let director_entity = commands
        .spawn(Director::new(camera_entity))
        .id();

    // 3️⃣ Spawn a virtual camera to shake
    commands.spawn((
        VirtualCamera {
            director: director_entity,
            priority: 1,
            blend_in: CameraBlendDefinition::default(),
        },
        Transform::from_translation(Vec3::Z * 2.),
        Projection::Perspective(PerspectiveProjection { fov: 1., aspect_ratio: 1.5, near: 0.1, far: 100. }),
    ));

}
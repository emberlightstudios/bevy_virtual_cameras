use bevy::prelude::*;
use bevy_virtual_cameras::prelude::*;

// Import your virtual camera modules

pub fn get_app() -> App {
    let mut app = App::new();
    app
        .add_plugins(DefaultPlugins)
        .add_plugins(VirtualCameraPlugin)
        .add_systems(Update, move_target);
    app
}

#[derive(Component)]
pub struct Target1;

#[derive(Component)]
pub struct Target2;

pub fn setup(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> (Entity, Entity) {
    // 1️⃣ Spawn a visible target
    let target_mesh = meshes.add(Cuboid::new(0.5, 0.5, 0.5).mesh());
    let target_material1 = materials.add(StandardMaterial {
        base_color: Color::LinearRgba(LinearRgba::RED),
        ..Default::default()
    });
    let target_material2 = materials.add(StandardMaterial {
        base_color: Color::LinearRgba(LinearRgba::BLUE),
        ..Default::default()
    });

    // circular base
    let mesh = meshes.add(Circle::new(3.0));
    let material = materials.add(Color::WHITE);

    commands.spawn((
        Mesh3d(mesh),
        MeshMaterial3d(material.clone()),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
            .with_translation(Vec3::NEG_Y),
    ));

    let capsule = meshes.add(Capsule3d::new(0.2, 1.0).mesh());
    commands.spawn((
        Mesh3d(capsule),
        MeshMaterial3d(material.clone()),
        Transform::from_translation(Vec3::NEG_Z * 3.),
    ));

    let target1 = commands
        .spawn((
            Transform::IDENTITY,
            GlobalTransform::default(),
            Mesh3d(target_mesh.clone()),
            MeshMaterial3d(target_material1),
            Target1,
        ))
        .id();

    let target2 = commands
        .spawn((
            Transform::IDENTITY,
            GlobalTransform::default(),
            Mesh3d(target_mesh),
            MeshMaterial3d(target_material2),
            Target2,
        ))
        .id();

    // 5️⃣ Optional: add some lighting
    commands.spawn((
        DirectionalLight {
            illuminance: 5000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -std::f32::consts::FRAC_PI_4,
            -std::f32::consts::FRAC_PI_4,
            0.0,
        )),
    ));
    (target1, target2)
}

// Simple movement for the target so the camera follows
fn move_target(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Target1>>,
    mut query2: Query<&mut Transform, (With<Target2>, Without<Target1>)>,
) {
    for mut t in &mut query {
        t.translation.x = (time.elapsed_secs() * 1.5).sin() * 7.0;
        t.translation.y = (time.elapsed_secs() * 1.8).cos() * 5.0;
        t.translation.z = (time.elapsed_secs() * 1.2).cos() * 5.0;
    }
    for mut t in &mut query2 {
        t.translation.x = (time.elapsed_secs() * 1.5).sin() * 2.0;
    }
}

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

#[derive(Resource)]
struct Cameras {
    cam2: Entity,
}

fn input (
    input: Res<ButtonInput<KeyCode>>,
    cameras: Res<Cameras>,
    mut vcams: Query<&mut VirtualCamera>,
) {
    if input.just_pressed(KeyCode::Space) {
        let mut vcam = vcams.get_mut(cameras.cam2).unwrap();
        if vcam.priority == 0 {
            info!("Enabling camera 2");
            vcam.priority = 2
        } else {
            info!("Disabling camera 2");
            vcam.priority = 0
        }
    }
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
            Transform::IDENTITY,
        ))
        .id();

    // 2️⃣ Spawn a director entity
    let director_entity = commands
        .spawn(Director::new(camera_entity))
        .id();

    // 3️⃣ Spawn a pair of virtual cameras to switch between
    let _cam1 = commands.spawn((
        VirtualCamera {
            director: director_entity,
            priority: 1,
            blend_in: CameraBlendDefinition {
                function: EaseFunction::CubicInOut,
                duration: std::time::Duration::from_secs(1),
            },
        },
        Transform::from_translation(Vec3::ZERO),
        Projection::Perspective(PerspectiveProjection::default()),
        FollowTarget { target: blue, offset: Vec3::Z, damping: 0. }
    )).id();

    let cam2 = commands.spawn((
        VirtualCamera {
            director: director_entity,
            priority: 0,
            blend_in: CameraBlendDefinition {
                function: EaseFunction::Elastic(20.),
                duration: std::time::Duration::from_secs(1),
            },
        },
        Transform::from_translation(Vec3::new(0., 5., 15.,)),
        Projection::Perspective(PerspectiveProjection::default()),
        LookAtTarget {
            target: red,
            offset: Vec3::ZERO,
            dead_zone: DeadZone { xmin: -0.5, ymin: -0.5, xmax: 0.5, ymax: 0.5 },
            damping: 1.
        } 
    )).id();

    commands.insert_resource(Cameras {cam2});
}
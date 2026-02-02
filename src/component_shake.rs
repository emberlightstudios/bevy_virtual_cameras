use bevy::prelude::*;

#[derive(Component, Debug, Clone, Default)]
pub struct Shake {
    /// Total duration of the shake
    pub timer: Timer,
    /// Maximum translation offset along each axis (world units)
    pub translation_intensity: Vec3,
    /// Maximum rotation offset along each axis (radians)
    pub rotation_intensity: Vec3,
    /// Frequency of translation shake (Hz)
    pub translation_frequency: Vec3,
    /// Frequency of rotation shake (Hz)
    pub rotation_frequency: Vec3,
    /// Damping factor (0 = instant stop, 1 = full fade over time)
    pub damping: f32,
    /// Optional seed for reproducible shake (affects phase)
    pub seed: f32,
    /// Transform when finished (cached automatically)
    pub original_transform: Option<Transform>,
}

#[derive(Message)]
pub struct AddCameraShake {
    pub vcam_entity: Entity,
    pub camera_shake: Shake,
}

pub(crate) fn add_shake(
    mut reader: MessageReader<AddCameraShake>,
    mut commands: Commands,
    mut query: Query<(&mut Transform, &mut Shake)>,
) {
    for AddCameraShake { vcam_entity, camera_shake } in reader.read() {
        if let Ok((mut transform, shake)) = query.get_mut(*vcam_entity) {
            if let Some(original) = shake.original_transform {
                *transform = original;
            }
            commands.entity(*vcam_entity).remove::<Shake>();
        }
        commands.entity(*vcam_entity).insert(camera_shake.clone());
    }
}

pub(crate) fn camera_shake_system(
    mut commands: Commands,
    time: Res<Time<Real>>,
    mut query: Query<(Entity, &mut Transform, &mut Shake)>,
) {
    for (entity, mut tf, mut shake) in query.iter_mut() {
        // Store original transform on first frame
        if shake.original_transform.is_none() {
            shake.original_transform = Some(*tf);
        }

        shake.timer.tick(time.delta());

        // Remove shake when timer finishes
        if shake.timer.is_finished() {
            if let Some(original) = shake.original_transform.take() {
                *tf = original;
            }
            commands.entity(entity).remove::<Shake>();
            continue;
        }

        let original = shake.original_transform.unwrap();
        let percent = shake.timer.fraction();
        let damping_factor = 1.0 - percent * shake.damping;

        // Time-based phase offset
        let elapsed = shake.timer.elapsed_secs() + shake.seed;

        // Translation offsets (sine waves)
        let trans_offset = Vec3::new(
            (elapsed * shake.translation_frequency.x * std::f32::consts::TAU).sin() * shake.translation_intensity.x,
            (elapsed * shake.translation_frequency.y * std::f32::consts::TAU).sin() * shake.translation_intensity.y,
            (elapsed * shake.translation_frequency.z * std::f32::consts::TAU).sin() * shake.translation_intensity.z,
        ) * damping_factor;

        // Rotation offsets (sine waves)
        let rot_offset = Quat::from_euler(
            EulerRot::XYZ,
            (elapsed * shake.rotation_frequency.x * std::f32::consts::TAU).sin() * shake.rotation_intensity.x,
            (elapsed * shake.rotation_frequency.y * std::f32::consts::TAU).sin() * shake.rotation_intensity.y,
            (elapsed * shake.rotation_frequency.z * std::f32::consts::TAU).sin() * shake.rotation_intensity.z,
        ) * damping_factor;

        // Apply shake on top of original transform
        tf.translation = original.translation + trans_offset;
        tf.rotation = rot_offset * original.rotation;
    }
}

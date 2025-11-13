use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct OrbitCamera {
    /// The entity to orbit around
    pub target: Entity,

    /// Distance from the target (radius)
    pub radius: f32,

    /// Offset in camera space
    pub offset: Vec3,

    /// Rotation around the target (in radians)
    pub yaw: f32,   // left/right
    pub pitch: f32, // up/down

    /// Optional damping (smooth movement)
    pub damping: f32,

    /// Clamps to prevent flipping over
    pub min_pitch: f32,
    pub max_pitch: f32,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self {
            target: Entity::PLACEHOLDER,
            offset: Vec3::ZERO,
            radius: 5.0,
            yaw: 0.0,
            pitch: 0.3,
            damping: 8.0,
            min_pitch: -1.4,
            max_pitch: 1.4,
        }
    }
}

pub fn orbit_camera_system(
    time: Res<Time>,
    target_transforms: Query<&GlobalTransform>,
    mut query: Query<(&mut Transform, &mut OrbitCamera)>,
) {
    let delta = time.delta_secs();

    for (mut transform, mut orbit) in query.iter_mut() {
        let Ok(target_tf) = target_transforms.get(orbit.target) else { continue; };
        let target_pos = target_tf.translation();

        // Clamp pitch to valid range
        orbit.pitch = orbit.pitch.clamp(orbit.min_pitch, orbit.max_pitch);

        // Compute desired position in spherical coordinates
        let dir = Vec3::new(
            orbit.yaw.cos() * orbit.pitch.cos(),
            orbit.pitch.sin(),
            orbit.yaw.sin() * orbit.pitch.cos(),
        );

        let desired_pos = target_pos + dir * orbit.radius;

        // Apply offset *in world space relative to camera axes*
        let offset_world = transform.rotation * orbit.offset;
        let desired_pos = desired_pos + offset_world;

        // Damping factor (smooth movement)
        let t = if orbit.damping > 0.0 { 1.0 - (-orbit.damping * delta).exp() } else { 1.0 };

        // Smoothly move camera
        transform.translation = transform.translation.lerp(desired_pos, t);

        // Always look at target
        transform.look_at(target_pos + offset_world, Vec3::Y);
    }
}

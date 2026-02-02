use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct OrbitArm {
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

impl Default for OrbitArm {
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
    time: Res<Time<Real>>,
    mut paramset: ParamSet<(
        Query<(Entity, &mut OrbitArm, &mut Transform)>,
        TransformHelper,
    )>,
) {
    let delta = time.delta_secs();

    let vcams = paramset
        .p0()
        .iter()
        .map(|(e, ..)| e)
        .collect::<Vec<_>>();

    for vcam in vcams {
        let q = paramset.p0();
        let Ok((_, orbit, _)) = q.get(vcam) else { continue };
        let target = orbit.target;

        let Ok(target_pos) = paramset.p1().compute_global_transform(target) else { continue };
        let target_pos = target_pos.translation();

        // Clamp pitch to valid range
        let mut q = paramset.p0();
        let Ok((_, mut orbit, mut transform)) = q.get_mut(vcam) else { continue };
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

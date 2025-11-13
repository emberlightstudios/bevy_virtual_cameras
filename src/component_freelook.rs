use bevy::prelude::*;


#[derive(Component, Debug, Clone, Default)]
pub struct FreeLookCamera {
    pub yaw: f32,          // Horizontal rotation (around Y axis)
    pub pitch: f32,        // Vertical rotation (around X axis)
    pub pitch_limit: f32,  // Maximum up/down rotation in radians
    pub invert_x: bool,    // Whether yaw is inverted
    pub invert_y: bool,    // Whether pitch is inverted
}

pub fn free_look_system(
    mut query: Query<(&mut Transform, &FreeLookCamera), With<FreeLookCamera>>,
) {
    for (mut cam_tf, freelook) in query.iter_mut() {
        // Apply inversion
        let yaw = if freelook.invert_x { freelook.yaw } else { -freelook.yaw };
        let pitch = if freelook.invert_y { freelook.pitch } else { -freelook.pitch };

        // Clamp pitch
        let pitch = pitch.clamp(-freelook.pitch_limit, freelook.pitch_limit);

        // Apply rotation
        *cam_tf = Transform {
            rotation: Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0),
            ..*cam_tf
        };
    }
}

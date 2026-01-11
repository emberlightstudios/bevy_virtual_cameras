use std::f32::consts::PI;

use bevy::prelude::*;


#[derive(Component, Debug, Clone)]
pub struct FreeLookCamera {
    pub yaw: f32,          // Horizontal rotation (around Y axis)
    pub pitch: f32,        // Vertical rotation (around X axis)
    pub pitch_limit: f32,  // Maximum up/down rotation in radians
}

impl Default for FreeLookCamera {
    fn default() -> Self {
        Self {
            yaw: 0.,
            pitch: 0.,
            pitch_limit: PI * 0.45,
        }
    }
}

pub fn free_look_system(
    mut query: Query<(&mut Transform, &mut FreeLookCamera), With<FreeLookCamera>>,
) {
    for (mut cam_tf, mut freelook) in query.iter_mut() {
        // Clamp pitch
        freelook.pitch = freelook.pitch.clamp(-freelook.pitch_limit, freelook.pitch_limit);

        // Apply rotation
        *cam_tf = Transform {
            rotation: Quat::from_euler(EulerRot::YXZ, freelook.yaw, freelook.pitch, 0.0),
            ..*cam_tf
        };
    }
}

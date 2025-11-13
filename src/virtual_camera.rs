use bevy::prelude::*;
use crate::{blend::CameraBlendDefinition, prelude::Director};


#[derive(Component)]
pub struct VirtualCamera {
    pub priority: i32,
    pub blend_in: CameraBlendDefinition,
    pub director: Entity,
}

pub(crate) fn camera_apply_system(
    directors: Query<&Director>,
    vcams: Query<(&Transform, &mut Projection), With<VirtualCamera>>,
    mut cameras: Query<(&mut Transform, &mut Projection), Without<VirtualCamera>>,
) {
    for director in directors.iter() {
        if director.blend.is_some() {
            return;
        }

        let active_vcam = match director.active {
            Some(e) => e,
            None => continue,
        };

        let Ok((vcam_tf, projection)) = vcams.get(active_vcam) else { continue };

        if let Ok((mut cam_tf, mut cam_proj)) = cameras.get_mut(director.camera_entity) {
            *cam_tf = *vcam_tf;
            *cam_proj = projection.clone();
        }
    }
}


use std::time::Duration;

use bevy::prelude::*;

use crate::{prelude::*, camera_state::CameraState};

#[derive(Clone, Debug)]
pub struct CameraBlendState {
    pub from: CameraState,
    pub to: Entity,
    pub t: f32, 
    pub(crate) definition: CameraBlendDefinition,
}

#[derive(Clone, Debug)]
pub struct CameraBlendDefinition {
    pub function: EaseFunction,
    pub duration: Duration,
}

impl Default for CameraBlendDefinition {
    fn default() -> Self {
        Self { function: EaseFunction::Linear, duration: Duration::from_secs(1) }
    }
}

impl CameraBlendDefinition {
    pub(crate) fn create(&self, from: CameraState, to: Entity) -> CameraBlendState {
        CameraBlendState {
            from,
            to,
            t: 0.,
            definition: self.clone()
        }
    }
}

pub(crate) fn camera_blend_update_system(
    mut directors: Query<&mut Director>,
    mut cameras: Query<(&mut Transform, &mut Projection), With<Camera3d>>,
    vcams: Query<(&Transform, &Projection), (With<VirtualCamera>, Without<Camera3d>)>,
    time: Res<Time<Real>>,
    mut message_writer: MessageWriter<FinishedCameraBlend>,
) {
    for mut director in directors.iter_mut() {
        let camera_entity = director.camera_entity;
        if let Some(blend) = &mut director.blend {

            // Advance blend
            let duration = blend.definition.duration.as_secs_f32();
            blend.t += time.delta_secs();
            let progress = (blend.t / duration).clamp(0.0, 1.0);
            let eased_t = blend.definition.function.sample(progress).unwrap();

            // Get camera states
            let Ok((to_transform, to_proj)) = vcams.get(blend.to) else { continue };

            // Interpolate state
            let interpolated_state = CameraState::interpolate(
                &blend.from,
                &CameraState { transform: to_transform.clone(), projection: to_proj.clone() },
                eased_t
            );

            // Apply to real camera
            if let Ok((mut camera, mut projection)) = cameras.get_mut(camera_entity) {
                *camera = interpolated_state.transform;
                *projection = interpolated_state.projection;
            }

            // Clean up finished blend
            if blend.t >= duration {
                director.blend = None;
                message_writer.write(FinishedCameraBlend {to: director.active.unwrap()});
            }
        }
    }
}

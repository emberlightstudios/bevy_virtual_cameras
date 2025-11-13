mod director;
mod camera_state;
mod virtual_camera;
mod blend;
mod component_lookat;
mod component_follow;
mod component_zoom;
mod component_orbit;
mod component_freelook;
mod component_shake;

use bevy::{camera::CameraProjection, prelude::*};


pub mod prelude {
    pub use crate::{
        VirtualCameraPlugin, DeadZone,
        component_follow::FollowTarget,
        component_lookat::LookAtTarget,
        component_zoom::GroupZoom,
        component_freelook::FreeLookCamera,
        component_orbit::OrbitCamera,
        component_shake::{CameraShake, AddCameraShake},
        director::{Director, StartedCameraBlend, FinishedCameraBlend},
        virtual_camera::VirtualCamera,
        blend::CameraBlendDefinition,
        camera_state::CameraState,
    };
}

#[derive(SystemSet, Eq, PartialEq, Hash, Debug, Copy, Clone)]
pub struct VirtualCameraSystems;

pub struct VirtualCameraPlugin;

impl Plugin for VirtualCameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_message::<component_shake::AddCameraShake>()
            .add_message::<director::StartedCameraBlend>()
            .add_message::<director::FinishedCameraBlend>()
            .add_systems(Update, 
                (
                    director::update_active_camera_system,
                    (
                        component_orbit::orbit_camera_system,
                        component_follow::follow_system,
                        component_lookat::look_at_system,
                        component_zoom::group_zoom_system,
                        component_freelook::free_look_system,
                        component_shake::add_shake,
                        component_shake::camera_shake_system,
                    )
                        .in_set(VirtualCameraSystems),
                    (
                        blend::camera_blend_update_system,
                        virtual_camera::camera_apply_system,
                    ),
                )
                    .chain()
                    .before(TransformSystems::Propagate)
        );
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct DeadZone {
    pub xmin: f32,
    pub xmax: f32,
    pub ymin: f32,
    pub ymax: f32,
}

impl DeadZone {
    pub const ZERO: DeadZone = DeadZone { xmin: 0., xmax: 0., ymin: 0., ymax: 0. };
}

pub(crate) fn world_to_ndc(world_pos: Vec3, camera_tf: &GlobalTransform, projection: &Projection) -> Vec2 {
    // View matrix
    let view = camera_tf.to_matrix().inverse();
    let clip = match projection {
        Projection::Perspective(p) => p.get_clip_from_view() * view * world_pos.extend(1.0),
        Projection::Orthographic(o) => o.get_clip_from_view() * view * world_pos.extend(1.0),
        Projection::Custom(c) => c.get_clip_from_view() * view * world_pos.extend(1.0),
    };
    let ndc = clip.truncate() / clip.w;      // [-1,1] range
    let ndc = ndc.xy();
    if ndc.is_finite() {
        ndc
    } else {
        Vec2::ZERO
    }
}



mod director;
mod camera_state;
mod virtual_camera;
mod blend;
mod component_lookat;
mod component_copy_rotation;
mod component_follow;
mod component_zoom;
mod component_orbit;
mod component_freelook;
mod component_shake;
mod debug;

use bevy::prelude::*;


pub mod prelude {
    pub use crate::{
        VirtualCameraPlugin, DeadZone,
        component_follow::{FollowTarget, FollowGroup},
        component_lookat::{LookAtTarget, LookAtGroup},
        component_copy_rotation::CopyRotation,
        component_zoom::GroupZoom,
        component_freelook::FreeLook,
        component_orbit::OrbitArm,
        component_shake::{Shake, AddCameraShake},
        director::{Director, StartedCameraBlend, FinishedCameraBlend},
        virtual_camera::VirtualCamera,
        blend::CameraBlendDefinition,
        camera_state::CameraState,
        debug::FrustumGizmo,
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
                    virtual_camera::sync_aspect_ratios,
                    virtual_camera::on_window_resize,
                )
            )
            .add_systems(PostUpdate, 
                (
                    director::update_active_camera_system,
                    (
                        component_follow::follow_target_system,
                        component_follow::follow_group_system,
                        component_zoom::group_zoom_system,
                        component_copy_rotation::copy_rotation_system,
                        component_lookat::look_at_system,
                        component_lookat::look_at_group_system,
                        component_freelook::free_look_system,
                        component_orbit::orbit_camera_system,
                        component_shake::add_shake,
                        component_shake::camera_shake_system,
                    )
                        .chain()
                        .in_set(VirtualCameraSystems),

                    blend::camera_blend_update_system,
                    virtual_camera::camera_apply_system,
                )
                    .chain()
            )
            .add_systems(
                PostUpdate,
                debug::draw_gizmos.after(TransformSystems::Propagate)
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


pub fn world_to_ndc(world_pos: Vec3, camera_tf: &Transform, projection: &Projection) -> Vec2 {
    // Compute view matrix (world -> camera space)
    let view = camera_tf.to_matrix().inverse();

    // Get clip (projection) matrix from projection component
    let clip_from_view = projection.get_clip_from_view();

    // Transform world -> clip space
    let clip = clip_from_view * view * world_pos.extend(1.0);

    // Perspective divide
    if clip.w.abs() > f32::EPSILON {
        let ndc = clip.truncate() / clip.w;
        let xy = ndc.xy();
        if xy.is_finite() {
            return xy; // [-1,1] range
        }
    }
    Vec2::ZERO
}

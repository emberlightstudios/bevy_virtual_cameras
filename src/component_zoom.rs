use bevy::prelude::*;

use crate::{DeadZone, world_to_ndc};

#[derive(Component, Debug, Clone, Default)]
pub struct GroupZoom {
    /// Entities to keep framed
    pub targets: Vec<Entity>,

    /// DeadZone 
    pub dead_zone: DeadZone,
 
    /// Optional smoothing factor (0 = instant)
    pub damping: f32,

    /// Minimum and maximum allowed zoom level.
    /// For perspective: distance limits.
    /// For orthographic: projection scale limits.
    pub min_scale: f32,
    pub max_scale: Option<f32>,
}

pub(crate) fn group_zoom_system(
    mut paramset: ParamSet<(
        Query<(Entity, &GroupZoom, &mut Transform, &mut Projection)>,
        TransformHelper,
    )>,
    time: Res<Time<Real>>,
) {
    let delta = time.delta_secs();

    let vcams = paramset
        .p0()
        .iter()
        .map(|(e, ..)| e)
        .collect::<Vec<_>>();

    for vcam in vcams {

        let q = paramset.p0();
        let Ok((_, zoom, _, _)) = q.get(vcam) else { continue };
        let valid_targets = zoom.targets.clone();
        let count = valid_targets.len();
        if count == 0 { continue }

        // Reference point (average position)
        let mut ref_point = Vec3::ZERO;
        let helper = paramset.p1();
        let mut positions = vec![];
        for target in valid_targets {
            let Ok(global) = helper.compute_global_transform(target) else { continue };
            positions.push(global.translation());
            ref_point += global.translation();
        }
        ref_point /= count as f32;

        // Camera forward vector (world-space)
        let mut q = paramset.p0();
        let Ok((_, zoom, mut transform, mut projection)) = q.get_mut(vcam) else { continue };
        let forward = transform.forward();

        // Check deadzone for transverse axes (optional)
        let mut breach = false;
        for position in positions.iter() {
            let ndc = world_to_ndc(*position, &transform, &*projection);
            if ndc.x < zoom.dead_zone.xmin
                || ndc.x > zoom.dead_zone.xmax
                || ndc.y < zoom.dead_zone.ymin
                || ndc.y > zoom.dead_zone.ymax
            {
                breach = true;
                break;
            }
        }

        match &mut *projection {
            Projection::Perspective(_) => {
                // Compute distance along forward axis to reference point
                let to_ref = ref_point - transform.translation;
                let current_dist = to_ref.dot(forward.into());

                // Prefer min_scale if group fits inside deadzone
                let desired = if !breach {
                    zoom.min_scale
                } else {
                    current_dist * 2.
                };

                // Clamp distance along forward
                let desired_dist = (desired)
                    .clamp(zoom.min_scale, zoom.max_scale.unwrap_or(f32::INFINITY));

                // Apply damping (scalar along forward)
                let damping = zoom.damping;
                let t = if damping > 0. {1.0 - (-zoom.damping * delta).exp() } else { 1. };
                let move_vec = forward * (current_dist - desired_dist) * t;

                // Move camera forward/back only
                transform.translation += move_vec;
            }

            Projection::Orthographic(o) => {
                // Adjust scale based on max forward offset
                let mut max_offset: f32 = 0.0;
                for position in positions {
                    let offset = (position - ref_point).dot(forward.into()).abs();
                    max_offset = max_offset.max(offset);
                }

                let mut desired_scale = max_offset.max(zoom.min_scale);
                if let Some(max_scale) = zoom.max_scale {
                    desired_scale = desired_scale.min(max_scale);
                }

                let t = 1.0 - (-zoom.damping * delta).exp();
                o.scale += (desired_scale - o.scale) * t;
            }

            Projection::Custom(_) => {}
        }
    }
}

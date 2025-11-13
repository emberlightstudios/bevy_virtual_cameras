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
    mut vcams: Query<(&GroupZoom, &GlobalTransform, &mut Transform, &mut Projection)>,
    targets: Query<&GlobalTransform, Without<GroupZoom>>,
    time: Res<Time>,
) {
    let delta = time.delta_secs();

    for (zoom, cam_global, mut cam_local, mut projection) in vcams.iter_mut() {
        // 1️⃣ Collect valid targets
        let valid_targets: Vec<&GlobalTransform> = zoom
            .targets
            .iter()
            .filter_map(|&e| targets.get(e).ok())
            .collect();

        if valid_targets.is_empty() {
            continue;
        }

        // 2️⃣ Reference point (average position)
        let ref_point: Vec3 = valid_targets
            .iter()
            .map(|t| t.translation())
            .sum::<Vec3>()
            / valid_targets.len() as f32;

        // 3️⃣ Camera forward vector (world-space)
        let forward = cam_global.forward();

        // 4️⃣ Check deadzone for transverse axes (optional)
        let mut breach = false;
        for tgt in &valid_targets {
            let ndc = world_to_ndc(tgt.translation(), cam_global, &*projection);
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
                // 5️⃣ Compute distance along forward axis to reference point
                let to_ref = ref_point - cam_global.translation();
                let current_dist = to_ref.dot(forward.into());

                // 6️⃣ Prefer min_scale if group fits inside deadzone
                let desired = if !breach {
                    zoom.min_scale
                } else {
                    current_dist * 2.
                };

                // 7️⃣ Clamp distance along forward
                let desired_dist = (desired)
                    .clamp(zoom.min_scale, zoom.max_scale.unwrap_or(f32::INFINITY));

                // 8️⃣ Apply damping (scalar along forward)
                let damping = zoom.damping;
                let t = if damping > 0. {1.0 - (-zoom.damping * delta).exp() } else { 1. };
                let move_vec = forward * (current_dist - desired_dist) * t;

                // 9️⃣ Move camera forward/back only
                cam_local.translation += move_vec;
            }

            Projection::Orthographic(o) => {
                // 10️⃣ Adjust scale based on max forward offset
                let mut max_offset: f32 = 0.0;
                for tgt in &valid_targets {
                    let offset = (tgt.translation() - ref_point).dot(forward.into()).abs();
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

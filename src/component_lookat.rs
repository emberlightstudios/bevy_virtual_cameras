use bevy::prelude::*;

use crate::{DeadZone, world_to_ndc};

#[derive(Component)]
#[allow(dead_code)]
pub struct LookAtTarget {
    pub target: Entity,
    pub offset: Vec3,
    pub dead_zone: DeadZone,
    pub damping: f32,
}

#[derive(Component)]
#[allow(dead_code)]
pub struct LookAtGroup {
    pub targets: Vec<Entity>,
    pub offset: Vec3,
    pub dead_zone: DeadZone,
    pub damping: f32,
}


pub(crate) fn look_at_system(
    mut paramset: ParamSet<(
        Query<(Entity, &LookAtTarget, &Projection, &mut Transform)>,
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
        // Get global target position
        let q = paramset.p0();
        let Ok((_, look_at, _, _)) = q.get(vcam) else { continue };
        let target = look_at.target;
        let offset = look_at.offset;

        let helper = paramset.p1();
        let Ok(target_pos) = helper.compute_global_transform(target) else { continue };
        let target_pos = target_pos.translation() + offset;

        let mut q = paramset.p0();
        let Ok((_, look_at, cam_proj, mut cam_tf)) = q.get_mut(vcam) else { continue };

        // 3) Compute screen-space position in [0,1] (X = 0 left -> 1 right, Y = 0 bottom -> 1 top)
        let screen_pos = match cam_proj {
            Projection::Perspective(_) | Projection::Custom(_) => {
                // world_to_ndc returns clip-space NDC in [-1,1]
                world_to_ndc(target_pos, &cam_tf, cam_proj)
            }
            Projection::Orthographic(o) => {
                // Use global transform (world space) for orthographic mapping.
                // right() and up() are world-space axes; dividing by o.scale gives approx [-1,1]
                let right = cam_tf.right();
                let up = cam_tf.up();
                let offset = target_pos - cam_tf.translation;
                Vec2::new(offset.dot(right.into()) / o.scale, offset.dot(up.into()) / o.scale)
            }
        };

        let dead_zone = look_at.dead_zone;
        // 5) If target inside dead zone -> do nothing
        if screen_pos.x >= dead_zone.xmin && screen_pos.x <= dead_zone.xmax &&
           screen_pos.y >= dead_zone.ymin && screen_pos.y <= dead_zone.ymax
        {
            continue;
        }

        // 6) Otherwise compute desired world rotation (look at target from vcam world pos)
        let desired_rot = Quat::look_at_rh(cam_tf.translation, target_pos, Vec3::Y).inverse();

        // 7) Apply damping (slerp in local space)
        let t = if look_at.damping > 0. { 1.0 - (-look_at.damping * delta).exp() } else { 1.0 };
        cam_tf.rotation = cam_tf.rotation.slerp(desired_rot, t);
    }
}

pub(crate) fn look_at_group_system(
    mut paramset: ParamSet<(
        Query<(Entity, &LookAtGroup, &Projection, &mut Transform)>,
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
        // Get global target position
        let q = paramset.p0();
        let Ok((_, look_at, _, _)) = q.get(vcam) else { continue };
        let offset = look_at.offset;

        let targets = look_at.targets.clone();
        let count = targets.len();
        if count == 0 { continue }

        let helper = paramset.p1();
        let mut sum = Vec3::ZERO;

        for target in targets {
            let Ok(target_pos) = helper.compute_global_transform(target) else { continue };
            sum += target_pos.translation();
        }
        let target_pos = sum / count as f32 + offset;

        let mut q = paramset.p0();
        let Ok((_, look_at, cam_proj, mut cam_tf)) = q.get_mut(vcam) else { continue };

        // 3) Compute screen-space position in [0,1] (X = 0 left -> 1 right, Y = 0 bottom -> 1 top)
        let screen_pos = match cam_proj {
            Projection::Perspective(_) | Projection::Custom(_) => {
                // world_to_ndc returns clip-space NDC in [-1,1]
                world_to_ndc(target_pos, &cam_tf, cam_proj)
            }
            Projection::Orthographic(o) => {
                // Use global transform (world space) for orthographic mapping.
                // right() and up() are world-space axes; dividing by o.scale gives approx [-1,1]
                let right = cam_tf.right();
                let up = cam_tf.up();
                let offset = target_pos - cam_tf.translation;
                Vec2::new(offset.dot(right.into()) / o.scale, offset.dot(up.into()) / o.scale)
            }
        };

        let dead_zone = look_at.dead_zone;
        // 5) If target inside dead zone -> do nothing
        if screen_pos.x >= dead_zone.xmin && screen_pos.x <= dead_zone.xmax &&
           screen_pos.y >= dead_zone.ymin && screen_pos.y <= dead_zone.ymax
        {
            continue;
        }

        // 6) Otherwise compute desired world rotation (look at target from vcam world pos)
        let desired_rot = Quat::look_at_rh(cam_tf.translation, target_pos, Vec3::Y).inverse();

        // 7) Apply damping (slerp in local space)
        let t = if look_at.damping > 0. { 1.0 - (-look_at.damping * delta).exp() } else { 1.0 };
        cam_tf.rotation = cam_tf.rotation.slerp(desired_rot, t);
    }
}

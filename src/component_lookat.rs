use bevy::prelude::*;

use crate::{DeadZone, world_to_ndc};

#[derive(Component)]
#[allow(dead_code)]
pub enum LookAtTarget {
    Single {
        target: Entity,
        offset: Vec3,
        dead_zone: DeadZone,
        damping: f32,
    },
    Group {
        targets: Vec<Entity>,
        offset: Vec3,
        dead_zone: DeadZone,
        damping: f32,
    },
}


pub(crate) fn look_at_system(
    mut vcams: Query<(&LookAtTarget, &GlobalTransform, &mut Transform, &Projection)>,
    target_transforms: Query<&Transform, Without<LookAtTarget>>,
    time: Res<Time>,
) {
    let delta = time.delta_secs();

    for (look_at, global_cam_tf, mut cam_tf, cam_proj) in vcams.iter_mut() {
        // 1) Compute target world position (single or group)
        let target_pos = match look_at {
            LookAtTarget::Single { target, offset, .. } => {
                if let Ok(t) = target_transforms.get(*target) {
                    t.translation + *offset
                } else {
                    continue;
                }
            }
            LookAtTarget::Group { targets, offset, .. } => {
                let mut sum = Vec3::ZERO;
                let mut count = 0;
                for &e in targets {
                    if let Ok(t) = target_transforms.get(e) {
                        sum += t.translation + *offset;
                        count += 1;
                    }
                }
                if count == 0 { continue; }
                sum / count as f32
            }
        };

        // 2) Dead zone and damping params
        let (dead_zone, damping) = match look_at {
            LookAtTarget::Single { dead_zone, damping, .. } => (*dead_zone, *damping),
            LookAtTarget::Group { dead_zone, damping, .. } => (*dead_zone, *damping),
        };

        // 3) Compute screen-space position in [0,1] (X = 0 left -> 1 right, Y = 0 bottom -> 1 top)
        let screen_pos = match cam_proj {
            Projection::Perspective(_) | Projection::Custom(_) => {
                // world_to_ndc returns clip-space NDC in [-1,1]
                world_to_ndc(target_pos, global_cam_tf, cam_proj)
            }
            Projection::Orthographic(o) => {
                // Use global transform (world space) for orthographic mapping.
                // right() and up() are world-space axes; dividing by o.scale gives approx [-1,1]
                let right = global_cam_tf.right();
                let up = global_cam_tf.up();
                let offset = target_pos - global_cam_tf.translation();
                Vec2::new(offset.dot(right.into()) / o.scale, offset.dot(up.into()) / o.scale)
            }
        };

        // 5) If target inside dead zone -> do nothing
        if screen_pos.x >= dead_zone.xmin && screen_pos.x <= dead_zone.xmax &&
           screen_pos.y >= dead_zone.ymin && screen_pos.y <= dead_zone.ymax
        {
            continue;
        }

        // 6) Otherwise compute desired world rotation (look at target from vcam world pos)
        let desired_rot = Transform::from_translation(global_cam_tf.translation())
            .looking_at(target_pos, Vec3::Y)
            .rotation;

        // 7) Apply damping (slerp in local space)
        let t = if damping > 0. { 1.0 - (-damping * delta).exp() } else { 1.0 };
        cam_tf.rotation = cam_tf.rotation.slerp(desired_rot, t);
    }
}

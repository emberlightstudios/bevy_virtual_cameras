use bevy::prelude::*;

#[derive(Component)]
#[allow(dead_code)]
pub enum FollowTarget {
    Single {
        target: Entity,
        offset: Vec3,
        damping: f32,
    },
    Group {
        targets: Vec<Entity>,
        offset: Vec3,
        damping: f32,
    },
}

pub(crate) fn follow_system(
    mut vcams: Query<(&FollowTarget, &mut Transform)>,
    target_transforms: Query<&Transform, Without<FollowTarget>>,
    time: Res<Time>,
) {
    let delta = time.delta_secs();

    for (follow, mut vcam_tf) in vcams.iter_mut() {
        // 1️⃣ Determine target world position
        let target_world = match follow {
            FollowTarget::Single { target, offset, .. } => {
                if let Ok(t) = target_transforms.get(*target) {
                    t.translation + *offset
                } else {
                    continue;
                }
            }
            FollowTarget::Group { targets, offset, .. } => {
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

        // 2️⃣ Compute damping factor
        let damping = match follow {
            FollowTarget::Single { damping, .. } => *damping,
            FollowTarget::Group { damping, .. } => *damping,
        };

        // ️3️⃣ Apply to local transform
        let t = if damping > 0. { 1.0 - (-damping * delta).exp()} else { 1.0 };
        vcam_tf.translation = vcam_tf.translation.lerp(target_world, t);
    }
}

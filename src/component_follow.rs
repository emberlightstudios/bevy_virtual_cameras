use bevy::prelude::*;
use smallvec::SmallVec;

#[derive(Component)]
#[allow(dead_code)]
pub struct FollowTarget {
    pub target: Entity,
    pub offset: Vec3,
    pub damping: f32,
}

#[derive(Component)]
#[allow(dead_code)]
pub struct FollowGroup {
    pub targets: SmallVec<[Entity; 8]>,
    pub offset: Vec3,
    pub damping: f32,
}

pub(crate) fn follow_target_system(
    mut paramset: ParamSet<(
        Query<(Entity, &FollowTarget, &mut Transform)>,
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
        // Determine target world position
        let q = paramset.p0();
        let Ok((_, follow, _)) = q.get(vcam) else { continue };
        let target = follow.target;

        let helper = paramset.p1();
        let Ok(target_tf) = helper.compute_global_transform(target) else { continue };

        let mut q = paramset.p0();
        let Ok((_, follow, mut vcam_tf)) = q.get_mut(vcam) else { continue };

        // Handle weirdness on target.  Otherwise follow is permanently broken
        if vcam_tf.translation.is_nan() || !vcam_tf.translation.is_finite() {
            vcam_tf.translation = target_tf.translation();
            continue;
        }

        // Apply to local transform
        let t = if follow.damping > 0. { 1.0 - (-follow.damping * delta).exp()} else { 1.0 };
        vcam_tf.translation = vcam_tf.translation.lerp(target_tf.translation() + vcam_tf.rotation * follow.offset, t);
    }
}

pub(crate) fn follow_group_system(
    mut paramset: ParamSet<(
        Query<(Entity, &FollowGroup, &mut Transform)>,
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
        // Determine target world position
        let q = paramset.p0();
        let Ok((_, follow, _)) = q.get(vcam) else { continue };

        let targets = follow.targets.clone();
        let count = targets.len();
        if count == 0 { return }

        let helper = paramset.p1();
        let mut sum = Vec3::ZERO;
        
        for target in targets {
            let Ok(target_pos) = helper.compute_global_transform(target) else { continue };
            sum += target_pos.translation();
        }
        let target_pos = sum / count as f32;

        let mut q = paramset.p0();
        let Ok((_, follow, mut vcam_tf)) = q.get_mut(vcam) else { continue };

        // Handle weirdness on target.  Otherwise follow is permanently broken
        if vcam_tf.translation.is_nan() || !vcam_tf.translation.is_finite() {
            vcam_tf.translation = target_pos;
            continue;
        }

        // Apply to local transform
        let t = if follow.damping > 0. { 1.0 - (-follow.damping * delta).exp()} else { 1.0 };
        vcam_tf.translation = vcam_tf.translation.lerp(target_pos + follow.offset, t);
    }
}

use bevy::prelude::*;

#[derive(Component)]
#[allow(dead_code)]
pub struct CopyRotation {
    pub target: Entity,
    pub damping: f32,
}

pub(crate) fn copy_rotation_system(
    mut paramset: ParamSet<(
        Query<(Entity, &CopyRotation, &Projection, &mut Transform)>,
        TransformHelper,
    )>,
    time: Res<Time>,
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
        let Ok((_, copy, _, _)) = q.get(vcam) else { continue };
        let target = copy.target;

        let helper = paramset.p1();
        let Ok(target_pos) = helper.compute_global_transform(target) else { continue };
        let target_rot = target_pos.rotation();

        let mut q = paramset.p0();
        let Ok((_, copy, _, mut cam_tf)) = q.get_mut(vcam) else { continue };

        // 7) Apply damping (slerp in local space)
        let t = if copy.damping > 0. { 1.0 - (-copy.damping * delta).exp() } else { 1.0 };
        cam_tf.rotation = cam_tf.rotation.slerp(target_rot, t);
    }
}

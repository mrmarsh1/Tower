use math::Vector3f;

pub fn raycast(
    origin: Vector3f,
    dir: Vector3f,
    vertices: &Vec<Vector3f>,
    indices: &Vec<u32>,
) -> Option<Vector3f> {
    for chunk in indices.chunks(3) {
        let v1 = vertices[chunk[0] as usize];
        let v2 = vertices[chunk[1] as usize];
        let v3 = vertices[chunk[2] as usize];
        if let Some(hit_point) = mti(origin, dir, v1, v2, v3) {
            return Some(hit_point);
        }
    }
    None
}

fn mti(
    origin: Vector3f,
    dir: Vector3f,
    v1: Vector3f,
    v2: Vector3f,
    v3: Vector3f,
) -> Option<Vector3f> {
    let e1 = v2 - v1;
    let e2 = v3 - v1;

    let ray_cross_e2 = dir.cross(&e2);
    let dot = e1.dot(&ray_cross_e2);

    if dot > -f32::EPSILON && dot < f32::EPSILON {
        return None;
    }

    let inv_dot = 1.0 / dot;
    let s = origin - v1;
    let u = inv_dot * s.dot(&ray_cross_e2);
    if u < 0.0 || u > 1.0 {
        return None;
    }

    let s_cross_e1 = s.cross(&e1);
    let v = inv_dot * dir.dot(&s_cross_e1);
    if u < 0.0 || v < 0.0 || u + v > 1.0 {
        return None;
    }

    let t = inv_dot * e2.dot(&s_cross_e1);

    if t > f32::EPSILON {
        let intersection_point = origin + dir * t;
        return Some(intersection_point);
    }

    None
}

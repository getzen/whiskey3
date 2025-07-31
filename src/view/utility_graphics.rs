/// Utility - Graphics
use macroquad::prelude::*;

#[allow(dead_code)]
pub fn draw_arc_lines(x: f32, y: f32, r: f32, start: f32, end: f32, stroke_width: f32, color: Color) {
    let center = vec2(x, y);
    let incr = (end - start) / 20.;
    for i in 0..20 {
        let a = center + polar_to_cartesian(r, start + i as f32 * incr);
        let b = center + polar_to_cartesian(r, start + (i + 1) as f32 * incr);
        draw_line(a.x, a.y, b.x, b.y, stroke_width, color);
    }
}

#[allow(dead_code)]
pub fn draw_arc(x: f32, y: f32, r: f32, start: f32, end: f32, color: Color) {
    let center = vec2(x, y);
    let incr = (end - start) / 20.;
    for i in 0..20 {
        let a = center + polar_to_cartesian(r, start + i as f32 * incr);
        let b = center + polar_to_cartesian(r, start + (i + 1) as f32 * incr);
        draw_triangle(center, a, b, color);
    }
}

/// Draws a polygon using the given vertices. A line is drawn between the last
/// point and the first.
#[allow(dead_code)]
pub fn draw_polygon_lines(vertices: &Vec<Vec2>, stroke_width: f32, color: Color) {
    let mut index = 0;

    while index < vertices.len() - 1 {
        let p = vertices[index];
        let q = vertices[index + 1];
        draw_line(p.x, p.y, q.x, q.y, stroke_width, color);
        index += 1;
    }
    let p = vertices.last().unwrap();
    let q = vertices.first().unwrap();
    draw_line(p.x, p.y, q.x, q.y, stroke_width, color);
}

#[allow(dead_code)]
/// Vertices must be in clockwise order and describe a simple polygon
/// (edges may not cross each other) with no holes and no colinear
/// vertices (a vertex that is exactly on a line formed by two others).
pub fn draw_polygon(vertices: &Vec<Vec2>, color: Color) {
    let triangles = triangulate_polygon(vertices);
    for (v1, v2, v3) in triangles {
        draw_triangle(v1, v2, v3, color);
    }
}

#[allow(dead_code)]
/// Assumes a 0,0 origin.
pub fn draw_rounded_rect_lines(width: f32, height: f32, radius: f32, stroke_width: f32, color: Color) {
    let mut start = std::f32::consts::PI;
    draw_arc_lines(
        radius,
        radius,
        radius,
        start,
        start + std::f32::consts::FRAC_PI_2,
        stroke_width,
        color,
    );
    start += std::f32::consts::FRAC_PI_2;
    draw_arc_lines(
        width - radius,
        radius,
        radius,
        start,
        start + std::f32::consts::FRAC_PI_2,
        stroke_width,
        color,
    );
    start += std::f32::consts::FRAC_PI_2;
    draw_arc_lines(
        width - radius,
        height - radius,
        radius,
        start,
        start + std::f32::consts::FRAC_PI_2,
        stroke_width,
        color,
    );
    start += std::f32::consts::FRAC_PI_2;
    draw_arc_lines(
        radius,
        height - radius,
        radius,
        start,
        start + std::f32::consts::FRAC_PI_2,
        stroke_width,
        color,
    );

    draw_line(radius, 0.0, width - radius, 0.0, stroke_width, color); // top
    draw_line(radius, height, width - radius, height, stroke_width, color); // bottom
    draw_line(0.0, radius, 0.0, height - radius, stroke_width, color); // left
    draw_line(width, radius, width, height - radius, stroke_width, color); // right
}

#[allow(dead_code)]
/// Assumes a 0,0 origin.
pub fn draw_rounded_rect(width: f32, height: f32, radius: f32, color: Color) {
    let mut start = std::f32::consts::PI;
    draw_arc(
        radius,
        radius,
        radius,
        start,
        start + std::f32::consts::FRAC_PI_2,
        color,
    );
    start += std::f32::consts::FRAC_PI_2;
    draw_arc(
        width - radius,
        radius,
        radius,
        start,
        start + std::f32::consts::FRAC_PI_2,
        color,
    );
    start += std::f32::consts::FRAC_PI_2;
    draw_arc(
        width - radius,
        height - radius,
        radius,
        start,
        start + std::f32::consts::FRAC_PI_2,
        color,
    );
    start += std::f32::consts::FRAC_PI_2;
    draw_arc(
        radius,
        height - radius,
        radius,
        start,
        start + std::f32::consts::FRAC_PI_2,
        color,
    );

    draw_rectangle(radius, 0.0, width - radius * 2.0, radius, color);
    draw_rectangle(radius, height - radius, width - radius * 2.0, radius, color);
    draw_rectangle(0.0, radius, width, height - radius * 2.0, color);
}

#[allow(dead_code)]
/// Determines if the given rect contains the given point.
pub fn rect_contains_point(origin: Vec2, size: Vec2, test_point: Vec2) -> bool {
    test_point.x >= origin.x
        && test_point.x <= origin.x + size.x
        && test_point.y >= origin.y
        && test_point.y <= origin.y + size.y
}

#[allow(dead_code)]
/// Determines if the given circle contains the given point.
pub fn circle_contains_point(origin: Vec2, radius: f32, test_point: Vec2) -> bool {
    origin.distance_squared(test_point) <= radius * radius
}

#[allow(dead_code)]
/// Returns true if the triangle a,b,c contains the given point.
/// https://www.youtube.com/watch?v=hTJFcHutls8
pub fn triangle_contains_point(a: Vec2, b: Vec2, c: Vec2, test_point: Vec2) -> bool {
    let p = test_point;
    let ab = b - a;
    let bc = c - b;
    let ca = a - c;

    let ap = p - a;
    let bp = p - b;
    let cp = p - c;

    let cross1 = cross(ab, ap);
    let cross2 = cross(bc, bp);
    let cross3 = cross(ca, cp);

    // Video says <= comparison should be used, but >= works instead:
    cross1 >= 0.0 && cross2 >= 0.0 && cross3 >= 0.0
}

#[allow(dead_code)]
/// Determines if the given polygon vertices contain the given point. Based on the
/// ray casting technique described here:
/// https://en.wikipedia.org/wiki/Point_in_polygon#Ray_casting_algorithm
pub fn polygon_contains_point(vertices: &Vec<Vec2>, test_point: Vec2) -> bool {
    let mut intersection_count = 0;

    // Use test_point as p1 and create a q1 some long distance away.
    let p1 = test_point;
    let q1 = vec2(100000.0, 100000.0);

    let mut index = 0;

    // Test each point pair from polygon.
    while index < vertices.len() - 1 {
        let p2 = vertices[index];
        let q2 = vertices[index + 1];
        if lines_intersect(p1, q1, p2, q2) {
            intersection_count += 1;
        }
        index += 1;
    }
    // Test the last point to the first point.
    let p2 = vertices.last().unwrap();
    let q2 = vertices.first().unwrap();
    if lines_intersect(p1, q1, *p2, *q2) {
        intersection_count += 1;
    }

    //println!("intersections: {}", intersection_count);

    // If the count is odd, the point is in the polygon.
    intersection_count % 2 != 0
}

#[allow(dead_code)]
/// # Returns true if the line segment 'p1q1' and 'p2q2' intersect.
/// https://www.geeksforgeeks.org/check-if-two-given-line-segments-intersect/
pub fn lines_intersect(p1: Vec2, q1: Vec2, p2: Vec2, q2: Vec2) -> bool {
    // Find the 4 orientations required for the general and special cases.
    let o1 = orientation(p1, q1, p2);
    let o2 = orientation(p1, q1, q2);
    let o3 = orientation(p2, q2, p1);
    let o4 = orientation(p2, q2, q1);

    //println!("{}, {}, {}, {}", o1, o2, o3, o4);
    // General case
    if o1 != o2 && o3 != o4 {
        return true;
    }

    // Special Cases
    // p1 , q1 and p2 are collinear and p2 lies on segment p1q1
    if o1 == 0 && on_segment(p1, p2, q1) {
        return true;
    }

    // p1 , q1 and q2 are collinear and q2 lies on segment p1q1
    if o2 == 0 && on_segment(p1, q2, q1) {
        return true;
    }

    // p2 , q2 and p1 are collinear and p1 lies on segment p2q2
    if o3 == 0 && on_segment(p2, p1, q2) {
        return true;
    }

    // p2 , q2 and q1 are collinear and q1 lies on segment p2q2
    if o4 == 0 && on_segment(p2, q1, q2) {
        return true;
    }

    false
}

#[allow(dead_code)]
/// Given three collinear points p, q, r, the function checks if
/// point q lies on line segment 'pr'. From:
/// https://www.geeksforgeeks.org/check-if-two-given-line-segments-intersect/
fn on_segment(p: Vec2, q: Vec2, r: Vec2) -> bool {
    q.x <= p.x.max(r.x) && q.x >= p.x.min(r.x) && q.y <= p.y.max(r.y) && q.y >= p.y.min(r.y)
}

#[allow(dead_code)]
/// To find the orientation of an ordered triplet (p,q,r). Returns the following values:
/// 0 : Collinear points
/// 1 : Clockwise points
/// 2 : Counterclockwise
/// https://www.geeksforgeeks.org/check-if-two-given-line-segments-intersect/
pub fn orientation(p: Vec2, q: Vec2, r: Vec2) -> u8 {
    let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);
    if val > 0.0 {
        return 1;
    }
    if val < 0.0 {
        return 2;
    }
    0
}

#[allow(dead_code)]
/// Vertices must be in clockwise order and describe a simple polygon
/// (edges may not cross each other) with no holes and no colinear
/// vertices (a vertex that is exactly on a line formed by two others).
/// https://www.youtube.com/watch?v=hTJFcHutls8
pub fn triangulate_polygon(vertices: &Vec<Vec2>) -> Vec<(Vec2, Vec2, Vec2)> {
    // Error checking here. vertices.len() > 2, etc.

    // Could check for simple polygon by calling lines_intersect()
    // on all the pairs.

    // Could also check for colinear edges by calling orientation().

    let mut indices = vec![];
    for i in 0..vertices.len() {
        indices.push(i);
    }

    let tri_count = vertices.len() - 2;
    let mut triangles = Vec::<(Vec2, Vec2, Vec2)>::with_capacity(tri_count);

    while indices.len() > 3 {
        for i in 0..indices.len() {
            let a = indices[i];
            let b_index = if i > 0 { i - 1 } else { indices.len() - 1 };
            let b = indices[b_index];
            let c_index = if i < indices.len() - 1 { i + 1 } else { 0 };
            let c = indices[c_index];

            // See if we have an ear.
            let va = vertices[a];
            let vb = vertices[b];
            let vc = vertices[c];

            let va_to_vb = vb - va;
            let va_to_vc = vc - va;

            // Is ear convex? Video says to use < comparison, but >
            // works instead.
            if cross(va_to_vb, va_to_vc) > 0.0 {
                continue; // reflex angle, not convex
            }

            let mut is_ear = true;

            // Are any of the other vertices inside this ear?
            for j in 0..vertices.len() {
                if j == a || j == b || j == c {
                    continue;
                }

                let p = vertices[j];
                if triangle_contains_point(vb, va, vc, p) {
                    is_ear = false;
                    break;
                }
            }

            if is_ear {
                triangles.push((vb, va, vc));
                indices.remove(i);
                break;
            }
        }
    }

    // Add final triangle
    let va = vertices[indices[0]];
    let vb = vertices[indices[1]];
    let vc = vertices[indices[2]];
    triangles.push((va, vb, vc));

    triangles
}

#[allow(dead_code)]
/// Cross product. Since it uses Vec2's, this should be faster
/// than converting to Vec3 and calling Vec3::cross().z.
pub fn cross(a: Vec2, b: Vec2) -> f32 {
    a.x * b.y - a.y * b.x
}

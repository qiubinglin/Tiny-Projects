struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    fn cmp(&self, rhs: &Point) -> std::cmp::Ordering {
        if self.x < rhs.x {
            std::cmp::Ordering::Less
        } else if self.x == rhs.x {
            std::cmp::Ordering::Equal
        } else {
            std::cmp::Ordering::Greater
        }
    }
}

struct Edge {
    p0: i32,
    p1: i32
}

struct Triangle {
    p0: i32,
    p1: i32,
    p2: i32,
    center: Point,
    radius_sqr: f64,
}

fn get_circumsphere(p0: &Point, p1: &Point, p2: &Point, center: &mut Point, radius_sqr: &mut f64) {
    let x1 = p1.x - p0.x;
    let y1 = p1.y - p0.y;
    let x2 = p2.x - p0.x;
    let y2 = p2.y - p0.y;

    // calc cross
    let mut det = x1 * y2 - y1 * x2;
    if det == 0f64 {
        center.x = p0.x;
        center.y = p0.y;
        center.z = p0.z;
        *radius_sqr = 0f64;
        return;
    }

    det = 0.5f64 /det;
    let len1 = x1*x1 + y1*y1;
    let len2 = x2*x2 + y2*y2;
    let cx = (len1 * y2 - len2 * y1) * det;
    let cy = (len2 * x1 - len1 * x2) * det;
    center.x = p0.x + cx;
	center.y = p0.y + cy;
	center.z = 0f64;
	*radius_sqr = cx * cx + cy * cy;
}

struct Delaunay2D {
    vertices: Vec<Point>,
    triangles: Vec<Triangle>,
}

fn circum_circle(vetx: &Point, tri: &Triangle) -> bool {
    let center = &tri.center;
    let radius_sqr = &tri.radius_sqr;
    if ((vetx.x - center.x).powi(2) + (vetx.y - center.y).powi(2)) <= *radius_sqr {
        true
    } else {
        false
    }
}

impl Delaunay2D {
    pub fn triangulate(&mut self, i_vertices: Vec<Point>) {
        // sort vertices
        self.vertices = i_vertices;
        self.vertices.sort_by(|a, b| a.cmp(b));

        // build super triangle, put it in unknown tris

        // unknown tris, corr tris, edge buf

        // traverse vertices
            // traverse unknown tris
                // if vertice is right outside tri, put tri in corr tris
                // if vertice is outside tri, continue
                // if vertice is inside tri
                    // tri's edge not in edge buf, put it in buf, else clear all same edge
            // let vertice and edges construct new triangles, then put them in unknown tris

        // merge unknown tris and corr tris
        // delete tris with super tri's edge
    }
}

fn main() {

}
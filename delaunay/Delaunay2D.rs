struct Point {
    x: f64,
    y: f64,
    // z: f64,
}

impl Point {
    fn cmp(&self, rhs: &Point) -> bool {
        self.x < rhs.x
    }
}

struct Edge {
    p1: Point,
    p2: Point,
}

struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
    center: Point,
    radius_sqr: f64,
}

struct Delaunay2D {
    vertices: Vec<Point>,
    triangles: Vec<Triangle>,
}

fn circum_circle(vetx: &Point, tri: &Triangle) -> bool {
    center = &tri.center;
    radius_sqr = &tri.radius_sqr;
    if ((vetx.x - center.x).powi(2) + (vetx.y - center.y).powi(2)) <= radius_sqr {
        true
    } else {
        false
    }
}

impl Delaunay2D {
    fn triangulate(i_vertices: mut Vec<Point>) {
        // sort vertices
        self.vertice = i_vertices;
        self.vertice.sort_by(|a, b| a.cmp(b));

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
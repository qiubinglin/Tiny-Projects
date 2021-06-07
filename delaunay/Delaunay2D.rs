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
    tri_completed: Vec<bool>,
    tri_count: usize,
    input_vertex_num: usize,
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
        self.input_vertex_num = i_vertices.len();
        self.vertices = i_vertices;
        self.vertices.sort_by(|a, b| a.cmp(b));

        // build super triangle, put it in unknown tris
        let mut xmin = self.vertices[0].x;
        let mut ymin = self.vertices[0].y;
        let mut xmax = xmin;
        let mut ymax = ymin;
        for i in 1..self.input_vertex_num {
            xmin = std::cmp::min(xmin, self.vertices[i].x);
            xmax = std::cmp::max(xmax, self.vertices[i].x);
            ymin = std::cmp::min(ymin, self.vertices[i].y);
            ymax = std::cmp::max(ymax, self.vertices[i].y);
        }
        let dx = xmax - xmin;
        let dy = ymax - ymin;
        let dmax = std::cmp::max(dx, dy);
        let xmid = (xmax + xmin) / 2.0;
        let ymid = (ymax + ymin) / 2.0;
        self.vertices.resize(self.input_vertex_num + 3, {0, 0, 0});
        self.vertices[self.input_vertex_num].x = xmid - 20 * dmax;
        self.vertices[self.input_vertex_num].y = ymid - dmax;
        self.vertices[self.input_vertex_num + 1].x = xmid;
        self.vertices[self.input_vertex_num + 1].y = ymid + 20 * dmax;
        self.vertices[self.input_vertex_num + 2].x = xmid + 20 * dmax;
        self.vertices[self.input_vertex_num + 2].y = ymid - dmax;

        self.add_triangle(self.input_vertex_num, self.input_vertex_num+1, self.input_vertex_num+2)

        // traverse vertices
        for ivtx in 0..self.input_vertex_num {
            // edges to build new unknown triangles
            let mut edges: Vec<Edge> = Vec<Edge>::new();

            // traverse unknown tris
            let mut itri = 0;
            while itri < self.tri_count {
                if (circum_circle(self.vertices[ivtx], self.triangles[itri])) {
                    // handle vertex inside tri circle
                    edges.push(Edge{p0: self.triangles[itri].p0, p1: self.triangles[itri].p1});
                    edges.push(Edge{p0: self.triangles[itri].p1, p1: self.triangles[itri].p2});
                    edges.push(Edge{p0: self.triangles[itri].p2, p1: self.triangles[itri].p0});

                    self.triangles[itri] = self.triangles[self.tri_count-1];
                    self.tri_completed[itri] = self.tri_completed[self.tri_count-1];
                    self.tri_count -= 1;
                } else {
                    if (self.triangles[itri].center.x + self.triangles[itri].radius_sqr.sqrt() < self.vertices[ivtx].x) {
                        // if vertice is right outside tri, put tri in corr tris
                        self.tri_completed[itri] = true;
                    } else {
                        // if vertice is outside tri, continue
                    }
                    itri += 1;
                }
            }
            // check and clear all same edge
            for i in 0..edges.len()-1 {
                for j in i+1..edges.len() {
                    if edges[i].p0 == edges[j].p1 && edges[i].p1 == edges[j].p0 {
                        edges[i].p0 = -1;
                        edges[i].p1 = -1;
                        edges[j].p0 = -1;
                        edges[j].p1 = -1;
                    }
                    if edges[i].p0 == edges[j].p0 && edges[i].p1 == edges[j].p1 {
                        edges[i].p0 = -1;
                        edges[i].p1 = -1;
                        edges[j].p0 = -1;
                        edges[j].p1 = -1;
                    }
                }
            }

            // let vertice and edges construct new triangles, then put them in unknown tris
            for i in 0..edges.len() {
                if edges[i].p0 < 0 || edges[i].p1 < 0 {
                    continue;
                }
                self.add_triangle(ivtx, edges[i].p0, edges[i].p1);
            }
        }
            
        // merge unknown tris and corr tris
        // delete tris with super tri's edge
        let mut i = 0;
        while i < self.tri_count {
            if self.triangles[i].p0 >= self.input_vertex_num as i32
               || self.triangles[i].p1 >= self.input_vertex_num as i32
               || self.triangles[i].p2 >= self.input_vertex_num as i32 {
                self.triangles[i] = self.triangles[tri_count-1];
                self.tri_completed[i] = self.tri_completed[tri_count-1];
                self.tri_count -= 1;
                i -= 1;
            }
            i += 1;
        }
    }

    fn add_triangle(&mut self, p0: i32, p1: i32, p2: i32) {
        let tri: Triangle = {
            p0: p0,
            p1: p1,
            p2: p2,
        };

        get_circumsphere(self.vertices[p0], self.vertices[p1], self.vertices[p2], tri.center, tri.radius_sqr);
        if self.tri_count < self.triangles.len() {
            self.triangles[self.tri_count] = tri;
            self.tri_completed[self.tri_count] = false;
        } else {
            self.triangles.push(tri);
            self.tri_completed.push(false);
        }
        tri_count += 1;
    }
}

fn main() {

}
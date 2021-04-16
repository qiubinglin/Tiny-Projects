struct Point {
    x: f64,
    y: f64,
    // z: f64,
}

struct Edge {
    p1: Point,
    p2: Point,
}

struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
}

struct Delaunay2D {
    vertices: Vec<Point>,
    triangles: Vec<Triangle>,
}

impl Delaunay2D {
    fn triangulate(vertices: Vec<Point>) {
        
    }
}
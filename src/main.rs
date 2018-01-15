#![feature(slice_patterns)]

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Vertex {
    x: f32,
    y: f32,
    z: f32,
}

impl std::ops::Sub for Vertex {
    type Output = Vertex;

    fn sub(self, rhs: Vertex) -> Self::Output {
        Vertex { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl std::ops::Add for Vertex {
    type Output = Vertex;

    fn add(self, rhs: Vertex) -> Self::Output {
        Vertex { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl Vertex {
    fn scale(&self, n: f32) -> Vertex {
        Vertex { x: self.x * n, y: self.y * n, z: self.z * n }
    }

    fn dot(&self, rhs: Vertex) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z + rhs.z
    }

    fn cross(&self, rhs: Vertex) -> Vertex {
        Vertex { x: self.y * rhs.z - self.z * rhs.y, y: self.z * rhs.x - self.x * rhs.z, z: self.x * rhs.y - self.y * rhs.x }
    }

    fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn unit(&self) -> Vertex {
        self.scale(1.0 / self.length())
    }
}

struct Vertices {
    vertices: Vec<Vertex>
}

impl Iterator for Vertices {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }
}
impl Vertices {
    fn max_length(&self) -> Option<f32> {
        self.vertices.iter()
            .map(Vertex::length)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
    }
}

struct Triangle {
    a: Vertex,
    b: Vertex,
    c: Vertex,
}

impl Triangle {
    fn unit(&self) -> Triangle {
        Triangle { a: self.a.unit(), b: self.b.unit(), c: self.c.unit() }
    }

    fn scale(&self, n: f32) -> Triangle {
        Triangle { a: self.a.scale(n), b: self.b.scale(n), c: self.c.scale(n) }
    }
}

struct Triangles {
    triangles: Vec<Triangle>
}

impl Vertex {
    fn parse3(x: &str, y: &str, z: &str) -> Vertex {
        Vertex { x: x.parse().unwrap(), y: y.parse().unwrap(), z: z.parse().unwrap() }
    }
    fn parse2(x: &str, y: &str) -> Vertex {
        Vertex { x: x.parse().unwrap(), y: y.parse().unwrap(), z: 0.0 }
    }
}

struct Face {
    v: (u16, u16, u16),
    t: (u16, u16, u16),
    n: (u16, u16, u16),
}

impl Face {
    fn parse(v: &str, t: &str, n: &str) -> Face {
        fn helper(triple: &str) -> (u16, u16, u16) {
            let triples: Vec<&str> = triple.split("/").collect();
            match triples.as_slice() {
                // FIXME need to subtract 1 from each here
                &[a, b, c] => (a.parse().unwrap(), b.parse().unwrap(), c.parse().unwrap()),
                _ => unreachable!()
            }
        }
        Face { v: helper(v), t: helper(t), n: helper(n) }
    }
}

struct Obj {
    vsv: Vec<Vertex>,
    vst: Vec<Vertex>,
    vsn: Vec<Vertex>,
    fs: Vec<Face>,
}

impl Obj {
    fn parse(file: File) -> Obj {
        let mut vsn: Vec<Vertex> = Vec::with_capacity(128);
        let mut vst: Vec<Vertex> = Vec::with_capacity(128);
        let mut vsv: Vec<Vertex> = Vec::with_capacity(128);
        let mut fs: Vec<Face> = Vec::with_capacity(128);
        let reader = BufReader::new(&file);
        for line in reader.lines() {
            let line = line.unwrap();
            let fields: Vec<&str> = line.split_whitespace().collect();
            match fields.as_slice() {
                &["vn", x, y, z] => vsn.push(Vertex::parse3(x, y, z)),
                &["vt", x, y, z] => vst.push(Vertex::parse3(x, y, z)),
                &["vt", x, y] => vst.push(Vertex::parse2(x, y)),
                &["v", x, y, z] => vsv.push(Vertex::parse3(x, y, z)),
                &["f", v, t, n] => fs.push(Face::parse(v, t, n)),
                &["#", ..] | &[] => {}
                _ => unreachable!()
            }
        }
        Obj { vsv: vsv, vst: vst, vsn: vsn, fs: fs }
    }
}

#[test]
fn test() {
    for file in vec!["model/anju.obj", "model/charizard.obj"] {
        let file = File::open(file).unwrap();
        Obj::parse(file);
    }
}

#[test]
fn test_max_length() {
    println!("{:?}", Vertices { vertices: vec![Vertex { x: 1.0, y: 1.0, z: 2.0 }] }.max_length());
    println!("{:?}", Vertices { vertices: vec![] }.max_length())
}

fn main() {
    println!("Hello, world!");
}

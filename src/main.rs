#![feature(slice_patterns)]

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Vertex {
    x: f32,
    y: f32,
    z: f32,
}

impl Vertex {
    fn parse3(x: &str, y: &str, z: &str) -> Vertex {
        Vertex { x: x.parse().unwrap(), y: y.parse().unwrap(), z: z.parse().unwrap() }
    }
    fn parse2(x: &str, y: &str) -> Vertex {
        Vertex { x: x.parse().unwrap(), y: y.parse().unwrap(), z: 0.0f32 }
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
                &["vn", x, y, z] =>
                    vsn.push(Vertex::parse3(x, y, z)),
                &["vt", x, y, z] =>
                    vst.push(Vertex::parse3(x, y, z)),
                &["vt", x, y] =>
                    vst.push(Vertex::parse2(x, y)),
                &["v", x, y, z] =>
                    vsv.push(Vertex::parse3(x, y, z)),
                &["f", v, t, n] => fs.push(Face::parse(v, t, n)),
                &["#", ..] => {}
                _ => unreachable!()
            }
        }
        Obj { vsv: vsv, vst: vst, vsn: vsn, fs: fs }
    }
}

#[test]
fn test() {
    let file = File::open("model/anju.obj").unwrap();
    Obj::parse(file);
}

fn main() {
    println!("Hello, world!");
}

extern crate stl_io;
use std::fs::OpenOptions;
use std::fs::File;
use std::io::Write;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[clap(parse(from_os_str))]
    input_path: std::path::PathBuf,
    #[clap(parse(from_os_str))]
    #[clap(short = 'o', long = "output")]
    output_path: std::path::PathBuf,
}

/*
nTriangle
verticies
v0.x
v0.y
v0.z
...
triangles
v0
v1
v2
...
normals (opt.)
v0.x normal
v0.y normal
v0.z normal
...

*/

fn main() {
    let args = Cli::parse();

    let mut input_file = OpenOptions::new().read(true).open(args.input_path).unwrap();

    let mesh = stl_io::read_stl(&mut input_file).unwrap();

    let mut output_file = File::create(args.output_path).unwrap();

    writeln!(&mut output_file, "{}", mesh.vertices.len()).unwrap();
    writeln!(output_file, "{}", mesh.faces.len()).unwrap();

    writeln!(output_file, "vertices").unwrap();
    for vertex in mesh.vertices.iter() {
        writeln!(output_file, "{:.06}", vertex[0]).unwrap();
        writeln!(output_file, "{:.06}", vertex[2]).unwrap();
        writeln!(output_file, "{:.06}", vertex[1]).unwrap();
    }

    writeln!(output_file, "triangles").unwrap();
    for triangle in mesh.faces.iter() {
        writeln!(output_file, "{}", triangle.vertices[0]).unwrap();
        writeln!(output_file, "{}", triangle.vertices[2]).unwrap();
        writeln!(output_file, "{}", triangle.vertices[1]).unwrap();
    }

    writeln!(output_file, "normals").unwrap();
    for triangle in mesh.faces.iter() {
        writeln!(output_file, "{:.16}", triangle.normal[0]).unwrap();
        writeln!(output_file, "{:.16}", triangle.normal[2]).unwrap();
        writeln!(output_file, "{:.16}", triangle.normal[1]).unwrap();
    }
}

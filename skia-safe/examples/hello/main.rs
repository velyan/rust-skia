extern crate skia_safe;

mod canvas;
use canvas::Canvas;

use std::fs;
use std::io::Write;
use skia_safe::interop::stream::MemoryStream;
use skia_safe::experimental::SVGDom;
use skia_safe::{Data, Size};
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // canvas.scale(1.2, 1.2);
    // canvas.move_to(36.0, 48.0);
    // canvas.quad_to(660.0, 880.0, 1200.0, 360.0);
    // canvas.translate(10.0, 10.0);
    // canvas.set_line_width(20.0);
    // canvas.stroke();
    // canvas.save();
    // canvas.move_to(30.0, 90.0);
    // canvas.line_to(110.0, 20.0);
    // canvas.line_to(240.0, 130.0);
    // canvas.line_to(60.0, 130.0);
    // canvas.line_to(190.0, 20.0);
    // canvas.line_to(270.0, 90.0);
    // canvas.fill();

//    compareAll();
   render();
}

pub fn render() {
 let paths = fs::read_dir("../svg/canva-svg/").unwrap();

    for path in paths {
        let p = path.unwrap();
        let name = &p.file_name();
        let n = name.to_str().unwrap();
        let name_str = String::from(n);

        if name_str.contains("svg") {
            use std::io::Read;

            let path_str = &p.path();

            let mut file = fs::File::open(path_str).unwrap();
            let mut bytes = vec![];
            file.read_to_end(&mut bytes).unwrap();

            println!("size of svg bytes {} for file {}", bytes.len(), &name_str);
            let mut data = Data::new_copy(&bytes);
            let mut stream = MemoryStream::from_data(&mut data);
            let mut svg_dom = SVGDom::from_stream(&mut stream);
            let svg_size = svg_dom.container_size();
            let mut canvas = Canvas::new(1024, 1024);
            svg_dom.set_container_size(&Size::new(450.0, 540.0));
            // if svg_size.is_zero() || svg_size.is_empty() {
            //     canvas = Canvas::new(500, 500);
            //     println!("setting default size");
            // }
            // else {
            //     let i_size = svg_size.to_floor();
            //     canvas = Canvas::new(i_size.width, i_size.height);
            // }

            svg_dom.render(canvas.canvas());
                
            let d = canvas.data();
            let split_name: Vec<&str> = name_str.split(".svg").collect();
            let name = split_name.first().unwrap();
            let file_name = String::from(*name) + &".png";

            let path = PathBuf::from("../svg/output/skia").join(&file_name);
            let mut file = fs::File::create(path).unwrap();
            let bytes = d.bytes();
            file.write_all(bytes).unwrap();
        }
    } 
}

pub fn compareAll() {
    let paths = fs::read_dir("../svg/output/skia").unwrap();
    for path in paths {
        let p = path.unwrap();
        let name = &p.file_name();
        let n = name.to_str().unwrap();
        let name_str = String::from(n);

        let split_name: Vec<&str> = name_str.split(".png").collect();
        let name = split_name.first().unwrap();
        compare(&String::from(*name));
    }
}

fn compare(file_name: &String) {
    let name = file_name.as_str();
    let png_file = String::from(name) + &".png";
    let ppm_file = String::from(name) + &".ppm";

    let skia_out = PathBuf::from("../svg/output/skia").join(&png_file);
    let resvg_out = PathBuf::from("../svg/output/resvg").join(&png_file);
    let diff = PathBuf::from("../svg/output/diff").join(&ppm_file);

    let cmd_output = Command::new("sh")
            .arg("./run.sh")
            .arg(skia_out.to_str().unwrap())
            .arg(resvg_out.to_str().unwrap())
            .arg(diff.to_str().unwrap())
            .output();
    // cmd_output;
    // println!("compare result {}", cmd_output);
}

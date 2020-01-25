use std::io::Read;
use std::f64;
use std::time::Instant;
use std::env;

extern crate bmp;
use bmp::{Image, Pixel};

const HELP: &str = "
binary-to-bmp24-converter converts any file given as input to a 24bit depth BMP image.
Usage:
binary-to-bmp24-converter FILE
";

fn main() -> std::io::Result<()> {
    let now = Instant::now();
    let mut buf : Vec<u8> = Vec::new();
    let mut size : usize = 0;
    let args: Vec<_> = env::args().collect();

    if args.len() == 2 {
        println!("Converting file {} to bmp.", args[1]);
        let filename = &args[1];

        read_file(&filename, &mut size, &mut buf).unwrap();

        let mut dim : u32 = ( (size as f64) / 3f64 ) as u32; // Round and loose some bytes.
        dim = (dim as f64).sqrt() as u32; // Round again and loose some bytes.

        let mut img = Image::new(dim, dim);
        let mut i:usize = 0;

        for (x, y) in img.coordinates() {
            let pix = Pixel{
                r: buf[i],
                g: buf[i+1],
                b: buf[i+2]
            };
            img.set_pixel(x, y, pix);
            i = i + 3;
        }
        img.save("foo.bmp").expect("Problem saving the image."); // TODO: implement dynamic name.

        println!("Image converted in {} ms.", now.elapsed().as_micros());
    }
    else{
        print!("{}",HELP);
        // TODO: return an error.
    }
    Ok(())
}

fn read_file( filename : &String, size : &mut usize , buf : &mut Vec<u8> ) -> std::io::Result<()> {
    let mut file = std::fs::File::open(filename).expect("Problem opening the file.");
    *size = file.read_to_end(buf).expect("Problem reading the file");
    Ok(())
}

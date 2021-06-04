use image::io::Reader as ImageReader;
use image::{GenericImage, GenericImageView, Pixel};

use std::env;
use std::collections::HashMap;
use std::cmp;

use std::path::Path;

fn main() {

    // first, parse the command line args again (namely, the picture to process)
    // will also need some 'level' of pixelation (size of pixel groups, maybe?)
    // perhaps also allow the pixelation metric used
    let args = env::args().collect();
    let parsed = parse_cmd_line(&args);
    
    let file = parsed.get("-f").unwrap();
    let level = parsed.get("-l").unwrap();

    let file = String::from(format!("./{}", &file));
    let level = level.parse::<u32>().unwrap();

    basic_pixelate(&Path::new(&file), level)

}


/// Parses the command line arguments into a HashMap
fn parse_cmd_line(args: &Vec<String>) -> HashMap<String,&str> {

    let mut parsed_args = HashMap::new();

    if (args.len() % 2) != 1 {
        panic!("Invalid input! Key without a pair identified.");
    }

    for i in (1..args.len()-1).step_by(2) {
        parsed_args.insert(args[i].clone(), args[i+1].as_ref()); // insert into map
    }

    parsed_args

}



fn basic_pixelate(path: &Path, level: u32) {
    // takes an imagebuffer and transforms it according a basic 

    let mut img = ImageReader::open(path).unwrap().decode().unwrap();

    let (width,height) = img.dimensions();
    let area_size = cmp::max(width,height) / level; // the side-length of each new pixelated area

    for i in 0..(width / area_size) {

        for j in 0..(height / area_size) {

            let (x,y) = (i * area_size, j * area_size);

            let (clampw,clamph) = (cmp::min(area_size, width - (area_size * i)), cmp::min(area_size, height - (area_size * j)));

            let area = img.view(x,y, clampw, clamph);

            // for this, we will use the average color of the area!
            let (w1,h1) = area.dimensions();

            let mut f = area.get_pixel(0, 0);

            for a in 0..w1 {
                for b in 0..h1 {
                    if a == 0 && b == 0 {
                        continue;
                    }

                    let pixel = area.get_pixel(a,b);
                    f.blend(&pixel);
                }
            }

            for a in 0..w1 {
                for b in 0..h1 {
                    img.put_pixel(x + a,y + b, f.to_rgba());
                }
            }

        }



    }


    // read an area of the image, collecting data abt pixels local to it
    // then choose the color to "paint" the area
    // save the image!
    //let mut output_path = PathBuf::from(path); // create path from input
    let filename = path.file_stem().unwrap().to_str().unwrap();
    let new_name = format!("{}-pixels.png", filename);
    img.save_with_format(new_name, image::ImageFormat::Png).unwrap();
    

}


fn pixelate_palette() {
    // will pixelate according to a pallete
}

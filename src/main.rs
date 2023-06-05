enum Color {
    Blue = 11,
    BlueDark = 12,
    Pink = 4,
    PinkDark = 14,
    White = 0,
    WhiteDark = 1
}

fn load_image() -> image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>> {
    let args: Vec<String> = std::env::args().collect();

    let image_path = args.get(1);
    if image_path.is_none() {
        panic!("Failed to get filename from args!");
    }
    let image_path = image_path.unwrap();
    let image_path = std::path::Path::new(image_path);
    if !image_path.exists() {
        panic!("File {} does not exist!", image_path.display());
    }
    else if !image_path.is_file() {
        panic!("{} is not a file!", image_path.display());
    }
    let image = image::io::Reader::open(image_path);
    if let Err(err) = image {
        panic!("Failed to open file {}! Is another program using it? {}", image_path.display(), err);
    }
    let image = image.unwrap().with_guessed_format();
    if let Err(err) = image {
        panic!("Failed to read file {}! {}", image_path.display(), err);
    }
    let image = image.unwrap().decode();
    if let Err(err) = image {
        panic!("Failed to decode image {}! {}", image_path.display(), err);
    }
    image.unwrap().into_rgb8()
}

fn main() {
    
    let image: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> = load_image();
    
    
    
    
}
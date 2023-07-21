fn make_thumbnail(image_path: &str, thumbnail_path: &str) -> anyhow::Result<()> {
    let image_bytes: Vec<u8> = std::fs::read(image_path)?;
    let image = if let Ok(format) = image::guess_format(&image_bytes) {
        image::load_from_memory_with_format(&image_bytes, format)?
    } else {
        image::load_from_memory(&image_bytes)?
    };
    let thumbnail = image.thumbnail(100, 100);
    thumbnail.save(thumbnail_path)?;
    Ok(())
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        println!("Usage: thumbnailer <image> <thumbnail>");
    } else {
        match make_thumbnail(&args[1], &args[2]) {
            Ok(_) => println!("Thumbnail created"),
            Err(err) => println!("Error: {}", err),
        }
    }
}

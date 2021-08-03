
use hex_coords::*;

use image::{ImageBuffer, RgbImage};

fn hexagonalize_image(img: &RgbImage, size: f32) -> RgbImage
{
    let (w, h) = img.dimensions();

    let mut grid = HexGrid::<([u32; 3], u32)>::new();

    for x in 0..w
    {
        for y in 0..h
        {
            let pix = PixelCoord(x as i32, y as i32);
            let hex = pix.to_hex(size);

            let val: [u8; 3] = img[(x, y)].0;

            let val: [u32; 3] = [val[0] as u32,
                                 val[1] as u32,
                                 val[2] as u32];
            if let Some(mut thing) = grid.get_mut(&hex)
            {
                thing.0[0] += val[0];
                thing.0[1] += val[1];
                thing.0[2] += val[2];
                thing.1 += 1;
            }
            else
            {
                grid.set(hex, (val, 1));
            }
            
        }
    }
    
    ImageBuffer::from_fn(w, h, |x, y| {

        let pix = PixelCoord(x as i32, y as i32);
        let hex = pix.to_hex(size);

        let (val, n) = grid.get(&hex).unwrap();

        let r = (val[0]/n) as u8;
        let g = (val[1]/n) as u8;
        let b = (val[2]/n) as u8;
        
        
        image::Rgb([r, g, b])
    })
    
}

fn main() -> Result<(), image::ImageError>
{
    let img: RgbImage = image::open("/home/clement/Wallpapers/39905312_p0.jpg")?.to_rgb8();
    
    let img2 = hexagonalize_image(&img, 10.);
    img2.save("hexagonalized.png")?;
    Ok(())
}

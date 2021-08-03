const BASE03: [u8; 3] = [0, 43, 54];
const BASE01: [u8; 3] = [88, 110, 117];
const BASE00: [u8; 3] = [101, 123, 131];
const BASE0: [u8; 3] = [131, 148, 150];
const BASE1: [u8; 3] = [147, 161, 161];
const BASE3: [u8; 3] = [253, 246, 227];
const YELLOW: [u8; 3] = [181, 137, 0];
const ORANGE: [u8; 3] = [203, 75, 22];
const RED: [u8; 3] = [220, 50, 47];
const BLUE: [u8; 3] = [38, 139, 210];
const CYAN: [u8; 3] = [42, 161, 152];
const GREEN: [u8; 3] = [133, 153, 0];


use hex_coords::*;

use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};

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

fn main() {
     use std::collections::HashMap;
    println!("{:?}", HexCoord(1, -1, 0));

    //let size = 6.0;
    let size = 20.0;
    {

        // Construct a new RGB ImageBuffer with the specified width and height.
        let img: RgbImage = ImageBuffer::new(512, 512);

        // Construct a new by repeated calls to the supplied closure.
        let ampl = 8;

        let palette = vec![
            BASE03, BASE01, BASE0, BASE1 
        ];
        
        let mut ids = HashMap::new();
        
        let mut img = ImageBuffer::from_fn(1000, 1000, |x, y| {
            let pix = PixelCoord(x as i32, y as i32);
            let hex = pix.to_hex(size);
            if !ids.contains_key(&hex)
            {
                let id: usize = rand::random();
                ids.insert(hex.clone(), id);
            }

            image::Rgb(palette[ids[&hex] % palette.len()])
        });

        img.save("test.png").unwrap();
    }
    {
        let coords = vec![PixelCoord(5, 50),
                          PixelCoord(5, 51),
        ];

        let hexs: Vec<_> = coords.iter().map(|pix| pix.to_hex(size)).collect();
        for (pix, hex) in coords.iter().zip(hexs.iter())
        {
            println!("{:?} -> {:?} -> {:?}", pix, hex, hex.to_pixel(size));
        }
    }


    {
        let img: RgbImage = image::open("teto.jpg").unwrap().to_rgb();
        
        let img2 = hexagonalize_image(&img, 20.);
        img2.save("hexagonalized.png");
    }
}

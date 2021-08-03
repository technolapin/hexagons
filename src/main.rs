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




#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum HexCoord
{
    Offset(i32, i32),
    Cube(i32, i32, i32),
    Axial(i32, i32),
    Doubled(i32, i32)
}
impl HexCoord
{
    fn to_cube(&self) -> Self
    {
        match self
        {
            Self::Cube(..) => *self,
            Self::Axial(q, r) => Self::Cube(*q, *r, -q-r),
            Self::Offset(col, row) => 
            {
                let x = col - (row - (col & 1)) / 2;
                let y = *row;
                let z = -x-y;
                Self::Cube(x, y, z)
            },
            Self::Doubled(col, row) => 
            {
                let x = *col;
                let z = (row + col)/2;
                let y = -x-z;
                Self::Cube(x, y, z)
            }
        }
    }
    fn to_axial(&self) -> Self
    {
        if let HexCoord::Cube(x, y, z) = self.to_cube()
        {
            HexCoord::Axial(x, z)}
        else
        {
            unreachable!();
        }
    }
    fn to_pixel(&self, size: f32) -> PixelCoord
    {
        if let HexCoord::Cube(q, _, r) = self.to_cube()
        {
            let (q, r) = (q as f32, r as f32);

            let x = size * (3f32).sqrt() * (q + r/2.0);
            let y = size * 3.0/2.0 * r;

            PixelCoord(x as i32, y as i32)
        }
        else
        {
            unreachable!();
        }
    }
    fn cube_from_float(x: f32, y: f32, z: f32) -> Self
    {
        let mut rx = x.round();
        let mut ry = y.round();
        let mut rz = z.round();

        let x_diff = (rx - x).abs();
        let y_diff = (ry - y).abs();
        let z_diff = (rz - z).abs();

        if x_diff > y_diff && x_diff > z_diff
        {
            rx = -ry-rz;
        }
        else if y_diff > z_diff
        {
            ry = -rx-rz;
        }
        else
        {
            rz = -rx-ry;
        }

        HexCoord::Cube(rx as i32, ry as i32, rz as i32)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct PixelCoord(i32, i32);

impl PixelCoord
{
    fn to_hex(&self, size: f32) -> HexCoord
    {
        let sqrt3 = 3f32.sqrt();
        let (x, y) = (self.0 as f32, self.1 as f32);
        let q = (sqrt3/3.0*x - y/3.0) / size;
        let r = y*2.0/3.0/size;

        
        HexCoord::cube_from_float(q, -q-r, r)

    }
}


fn main() {
    use std::collections::HashMap;
    println!("{:?}", HexCoord::Cube(1, -1, 0));

    //let size = 6.0;
    let size = 20.0;
    {
        use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};

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
        let coords = vec![PixelCoord(5, 14),
                          PixelCoord(5, 15),
        ];

        let hexs: Vec<_> = coords.iter().map(|pix| pix.to_hex(size)).collect();
        for (pix, hex) in coords.iter().zip(hexs.iter())
        {
            println!("{:?} -> {:?}", pix, hex);
        }
    }
}

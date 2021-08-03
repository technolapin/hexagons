#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HexCoord(pub i32, pub i32, pub i32);

impl HexCoord
{
    // untested
    pub fn to_pixel(&self, size: f32) -> PixelCoord
    {
        if let HexCoord(q, _, r) = *self
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

    pub fn from_floats(x: f32, y: f32, z: f32) -> Self
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

        HexCoord(rx as i32, ry as i32, rz as i32)
    }
    
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PixelCoord(pub i32, pub i32);

impl PixelCoord
{
    pub fn to_hex(&self, size: f32) -> HexCoord
    {
        let sqrt3 = 3f32.sqrt();
        let (x, y) = (self.0 as f32, self.1 as f32);
        let q = (sqrt3/3.0*x - y/3.0) / size;
        let r = y*2.0/3.0/size;

        
        HexCoord::from_floats(q, -q-r, r)

    }
}

macro_rules! impl_op
{
    ($trait: ident, $foo: ident, $op:tt) =>
    {
        impl $trait for HexCoord
        {
            type Output = HexCoord;
            fn $foo(self, other: Self) -> Self::Output
            {
                HexCoord(self.0 $op other.0,
                         self.1 $op other.1,
                         self.2 $op other.2)
            }
        }

        impl $trait for &HexCoord
        {
            type Output = HexCoord;
            fn $foo(self, other: Self) -> Self::Output
            {
                HexCoord(self.0 $op other.0,
                         self.1 $op other.1,
                         self.2 $op other.2)
            }
        }
    }

}

use std::ops::*;

impl_op!(Add, add, +);
impl_op!(Sub, sub, -);
impl_op!(Mul, mul, *);
impl_op!(Div, div, /);

impl Neg for HexCoord
{
    type Output = Self;
    fn neg(self) -> Self::Output
    {
        Self(-self.0, -self.1, -self.2)
    }
}

use std::collections::HashMap;

use crate::*;

pub struct HexGrid<T>(HashMap<HexCoord, T>);

const NGH_DIR: [HexCoord; 6] = [ HexCoord(-1, 1, 0),
                                 HexCoord(1, -1, 0),
                                 HexCoord(0, -1, 1),
                                 HexCoord(0, 1, -1),
                                 HexCoord(-1, 0, 1),
                                 HexCoord(1, 0, -1) ];


impl<T> HexGrid<T>
{
    pub fn new() -> Self
    {
        Self(HashMap::new())
    }
    
    pub fn get(&self, pos: &HexCoord) -> Option<&T>
    {
        self.0.get(pos)
    }
    pub fn get_mut(&mut self, pos: &HexCoord) -> Option<&mut T>
    {
        self.0.get_mut(pos)
    }
    pub fn set(&mut self, pos: HexCoord, val: T)
    {
        self.0.insert(pos, val);
    }
    pub fn contains(&self, pos: &HexCoord) -> bool
    {
        self.0.contains_key(pos)
    }
    
    pub fn iter_neighbors<'a, 'b>(&'a self, pos: &'b HexCoord) -> impl std::iter::Iterator + 'b
    where
        'a: 'b
    {
        NGH_DIR.iter().map(move |dir| self.get(&(dir+pos)))
    }
}


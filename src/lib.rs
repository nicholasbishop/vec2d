extern crate num;

use std::ops;

/// 2D grid coordinate
#[derive(Clone, Copy, Eq, Debug, PartialEq)]
pub struct Coord<T> {
    pub x: T,
    pub y: T
}

impl<T> Coord<T> {
    /// Create a grid coordinate at (x, y)
    pub fn new(x: T, y: T) -> Coord<T> {
        Coord {
            x: x,
            y: y
        }
    }
}

/// Rectangle defined by inclusive minimum and maximum coordinates
#[derive(Clone, Copy, Eq, Debug, PartialEq)]
pub struct Rect<T: Copy> {
    /// Minimum coordinate (inclusive)
    pub min_coord: Coord<T>,

    /// Maximum coordinate (inclusive)
    pub max_coord: Coord<T>
}

impl<T: Copy + PartialOrd> Rect<T> {
    /// Create a new Rect defined by inclusive minimum and maximum
    /// coordinates. If min_coord is greater than max_coord on either
    /// axis then None is returned.
    pub fn new(min_coord: Coord<T>, max_coord: Coord<T>) -> Option<Rect<T>> {
        if min_coord.x <= max_coord.x && min_coord.y <= max_coord.y {
            Some(Rect {
                min_coord: min_coord,
                max_coord: max_coord
            })
        }
        else {
            None
        }
    }

    pub fn iter(&self) -> RectIter<T> {
        RectIter {
            rect: *self,
            cur_coord: self.min_coord
        }
    }
}

pub struct RectIter<T: Copy> {
    rect: Rect<T>,
    cur_coord: Coord<T>
}

impl<T: Copy + Ord + ops::Add<Output=T> + num::One> Iterator for RectIter<T> {
    type Item = Coord<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_coord.y <= self.rect.max_coord.y {
            let result = Some(self.cur_coord);
            self.cur_coord.x = self.cur_coord.x + T::one();
            if self.cur_coord.x > self.rect.max_coord.x {
                self.cur_coord.x = self.rect.min_coord.x;
                self.cur_coord.y = self.cur_coord.y + T::one();
            }
            result
        }
        else {
            None
        }
    }
}

#[test]
fn test_rect_iter() {
    let rect = Rect::new(Coord::new(1, 2), Coord::new(3, 4)).unwrap();
    let coords: Vec<Coord<u8>> = rect.iter().collect();
    assert_eq!(coords, [
        Coord::new(1, 2), Coord::new(2, 2), Coord::new(3, 2),
        Coord::new(1, 3), Coord::new(2, 3), Coord::new(3, 3),
        Coord::new(1, 4), Coord::new(2, 4), Coord::new(3, 4)]);
}

// pub struct RectIter<'s, S: 's, T> {
//     data: &'s [S],
//     cur_elem: *const S,
//     cur_coord: 
//     full: Rect<T>,
//     part: Rect<T>
// }

// impl<'s, S: 's, T> Iterator for RectIter<'s, S, T> {
//     type Item = (Coord<T>, &'s S);

//     fn next(&mut self) -> Option<Self::Item> {
//         if cur {
            
//         }
//     }
// }

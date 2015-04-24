/// 2D grid coordinate
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Coord {
    pub x: usize,
    pub y: usize
}

/// 2D grid dimensions
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Size {
    pub width: usize,
    pub height: usize
}

impl Coord {
    /// Create a grid coordinate at (x, y)
    pub fn new(x: usize, y: usize) -> Coord {
        Coord {
            x: x,
            y: y
        }
    }
}

impl Size {
    /// Create a grid size of (width, height)
    pub fn new(width: usize, height: usize) -> Size {
        Size {
            width: width,
            height: height
        }
    }

    /// Return true if the coordinate fits within self's width and
    /// height, false otherwise.
    pub fn contains_coord(&self, coord: Coord) -> bool {
        coord.x < self.width && coord.y < self.height
    }
}

/// Rectangle defined by inclusive minimum and maximum coordinates
#[derive(Clone, Copy, Eq, Debug, PartialEq)]
pub struct Rect {
    /// Minimum coordinate (inclusive)
    min_coord: Coord,

    /// Maximum coordinate (inclusive)
    max_coord: Coord
}

impl Rect {
    fn width(&self) -> usize {
        return self.max_coord.x - self.min_coord.x + 1;
    }
}

pub struct GridMut<'a, Elem: 'a> {
    elems: &'a mut [Elem],
    size: Size
}

impl<'a, Elem> GridMut<'a, Elem> {
    pub fn new(elems: &'a mut [Elem], size: Size) -> Option<GridMut<Elem>> {
        if size.width * size.height == elems.len() {
            Some(GridMut {
                elems: elems,
                size: size
            })
        }
        else {
            None
        }
    }

    pub fn rect_iter_mut(&'a mut self, rect: Rect) -> Option<RectIterMut<'a, Elem>> {
        if self.size.contains_coord(rect.max_coord) {
            Some(RectIterMut {
                stride: (self.size.width - rect.width()) as isize,
                cur_elem: self.elems.as_mut_ptr(),
                grid: self,
                rect: rect,
                cur_coord: rect.min_coord
            })
        }
        else {
            None
        }
    }
}

impl<'a, Elem> Iterator for RectIterMut<'a, Elem> {
    type Item = (Coord, &'a mut Elem);

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_coord.y <= self.rect.max_coord.y {
            let result = (self.cur_coord, unsafe { &mut *self.cur_elem });

            self.cur_coord.x += 1;
            if self.cur_coord.x <= self.rect.max_coord.x {
                unsafe { self.cur_elem.offset(1); }
            }
            else {
                self.cur_coord.x = self.rect.min_coord.x;
                self.cur_coord.y += 1;
                unsafe { self.cur_elem.offset(self.stride); }
            }
            Some(result)
        }
        else {
            None
        }
    }
}

pub struct RectIterMut<'a, Elem: 'a> {
    grid: &'a mut GridMut<'a, Elem>,
    rect: Rect,
    cur_elem: *mut Elem,
    cur_coord: Coord,
    stride: isize
}

#[test]
fn test_rect_iter_mut() {
    let mut elems = [0, 1, 2, 3];
    let mut grid = GridMut::new(&mut elems, Size::new(2, 2)).unwrap();
    let rect = Rect::new(Coord::new(0, 0), Coord::new(1, 1)).unwrap();

    let mut actual_coords = Vec::new();
    for (coord, elem) in grid.rect_iter_mut(rect).unwrap() {
        *elem = -(*elem);
        actual_coords.push((coord.x, coord.y));
    }
    assert_eq!(actual_coords, [(0, 0), (1, 0), (0, 1), (1, 1)]);
}

impl Rect {
    /// Create a new Rect defined by inclusive minimum and maximum
    /// coordinates. If min_coord is greater than max_coord on either
    /// axis then None is returned.
    pub fn new(min_coord: Coord, max_coord: Coord) -> Option<Rect> {
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

}

// pub struct RectIter<usize: Copy + Unsigned> {
//     rect: Rect,
//     cur_coord: Coord
// }

// impl<usize: Copy + Ord + Unsigned + Add<Output=usize> + num::One> Iterator for RectIter {
//     type Item = Coord;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.cur_coord.y <= self.rect.max_coord.y {
//             let result = Some(self.cur_coord);
//             self.cur_coord.x = self.cur_coord.x + usize::one();
//             if self.cur_coord.x > self.rect.max_coord.x {
//                 self.cur_coord.x = self.rect.min_coord.x;
//                 self.cur_coord.y = self.cur_coord.y + usize::one();
//             }
//             result
//         }
//         else {
//             None
//         }
//     }
// }

// #[test]
// fn test_rect_iter() {
//     let rect = Rect::new(Coord::new(1, 2), Coord::new(3, 4)).unwrap();
//     let coords: Vec<Coord<u8>> = rect.iter().collect();
//     assert_eq!(coords, [
//         Coord::new(1, 2), Coord::new(2, 2), Coord::new(3, 2),
//         Coord::new(1, 3), Coord::new(2, 3), Coord::new(3, 3),
//         Coord::new(1, 4), Coord::new(2, 4), Coord::new(3, 4)]);
// }

// pub struct DataRectIter<'s, S: 's, usize: Copy + Unsigned> {
//     data: &'s [S],
//     cur_elem: *const S,
//     cur_coord: Coord,
//     full: Rect,
//     part: Rect
// }

// impl<'s, S: 's, usize: Copy + Unsigned> Iterator for DataRectIter<'s, S, usize> {
//     type Item = (Coord, &'s S);

//     fn next(&mut self) -> Option<Self::Item> {
//         unsafe {
//             self.cur_elem = self.cur_elem.offset(1);
//         }
//         None
//     }
// }


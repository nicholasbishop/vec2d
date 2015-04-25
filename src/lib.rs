// Copyright 2015 Nicholas Bishop
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Vec2D is a very simple 2D container for storing rectangular data

#![deny(missing_docs)]

/// 2D coordinate
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Coord {
    /// X component
    pub x: usize,

    /// Y component
    pub y: usize
}

/// Rectangle defined by inclusive minimum and maximum coordinates
#[derive(Clone, Copy, Eq, Debug, PartialEq)]
pub struct Rect {
    /// Minimum coordinate (inclusive)
    min_coord: Coord,

    /// Maximum coordinate (inclusive)
    max_coord: Coord
}

/// Rectangle dimensions
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Size {
    /// Width of rectangle
    pub width: usize,
    /// Height of rectangle
    pub height: usize
}

/// Container for 2D data
#[derive(Clone, Debug)]
pub struct Vec2D<T> {
    elems: Vec<T>,
    size: Size
}

/// Mutable iterator over a Vec2D
pub struct RectIterMut<'a, Elem: 'a> {
    grid: std::marker::PhantomData<&'a mut Vec2D<Elem>>,

    rect: Rect,
    cur_elem: *mut Elem,
    cur_coord: Coord,
    stride: isize
}

impl Coord {
    /// Create a coordinate at (x, y)
    pub fn new(x: usize, y: usize) -> Coord {
        Coord {
            x: x,
            y: y
        }
    }
}

impl Rect {
    /// Calculate rectangle width
    pub fn width(&self) -> usize {
        return self.max_coord.x - self.min_coord.x + 1;
    }

    /// Calculate rectangle height
    pub fn height(&self) -> usize {
        return self.max_coord.y - self.min_coord.y + 1;
    }

    /// Calculate rectangle size
    pub fn size(&self) -> Size {
        Size::new(self.width(), self.height())
    }
}

impl Size {
    /// Create a 2D size of (width, height)
    pub fn new(width: usize, height: usize) -> Size {
        Size {
            width: width,
            height: height
        }
    }

    /// width * height
    pub fn area(&self) -> usize {
        self.width * self.height
    }

    /// Return true if the coordinate fits within self's width and
    /// height, false otherwise.
    pub fn contains_coord(&self, coord: Coord) -> bool {
        coord.x < self.width && coord.y < self.height
    }
}

impl<Elem: Copy> Vec2D<Elem> {
    /// Create a Vec2D with the given `size`. All elements are
    /// initialized as copies of the `example` element.
    pub fn from_example(size: Size, example: &Elem) -> Vec2D<Elem> {
        Vec2D {
            elems: vec![*example; size.area()],
            size: size
        }
    }

    /// Create a Vec2D with the given `size`. The contents are set to
    /// `src`. None is returned if the `size` does not match the
    /// length of `src`.
    pub fn from_vec(size: Size, src: Vec<Elem>) -> Option<Vec2D<Elem>> {
        if size.area() == src.len() {
            Some(Vec2D {
                elems: src,
                size: size
            })
        }
        else {
            None
        }
    }

    /// Create a mutable iterator over a rectangular region of the
    /// Vec2D. None is returned if the given `rect` does not fit
    /// entirely within the Vec2D.
    pub fn rect_iter_mut<'a>(&'a mut self, rect: Rect) -> Option<RectIterMut<'a, Elem>> {
        if self.size.contains_coord(rect.max_coord) {
            Some(RectIterMut {
                grid: std::marker::PhantomData,
                stride: (self.size.width - rect.width() + 1) as isize,
                cur_elem: self.elems.as_mut_ptr(),
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
                unsafe { self.cur_elem = self.cur_elem.offset(1); }
            }
            else {
                self.cur_coord.x = self.rect.min_coord.x;
                self.cur_coord.y += 1;
                unsafe { self.cur_elem = self.cur_elem.offset(self.stride); }
            }
            Some(result)
        }
        else {
            None
        }
    }
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_coord() {
        let coord = Coord::new(1, 2);
        assert_eq!(coord.x, 1);
        assert_eq!(coord.y, 2);
    }

    #[test]
    fn test_rect() {
        let rect = Rect::new(Coord::new(1, 2), Coord::new(5, 3)).unwrap();
        assert_eq!(rect.width(), 5);
        assert_eq!(rect.height(), 2);

        assert_eq!(rect.width(), rect.size().width);
        assert_eq!(rect.height(), rect.size().height);
    }

    #[test]
    fn test_bad_rect() {
        assert_eq!(Rect::new(Coord::new(2, 1), Coord::new(1, 1)).is_none(),
                   true);
        assert_eq!(Rect::new(Coord::new(1, 2), Coord::new(1, 1)).is_none(),
                   true);
    }

    #[test]
    fn test_rect_iter_mut() {
        let elems = vec![1, 2, 3, 4];
        let mut grid = Vec2D::from_vec(Size::new(2, 2), elems).unwrap();
        let rect = Rect::new(Coord::new(0, 0), Coord::new(1, 1)).unwrap();

        let mut actual_coords = Vec::new();
        for (coord, elem) in grid.rect_iter_mut(rect).unwrap() {
            *elem = -(*elem);
            actual_coords.push((coord.x, coord.y));
        }
        assert_eq!(actual_coords, [(0, 0), (1, 0), (0, 1), (1, 1)]);
        assert_eq!(grid.elems, [-1, -2, -3, -4]);
    }
}

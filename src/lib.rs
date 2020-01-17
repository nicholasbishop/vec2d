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
    pub y: usize,
}

/// Rectangle defined by inclusive minimum and maximum coordinates
#[derive(Clone, Copy, Eq, Debug, PartialEq)]
pub struct Rect {
    /// Minimum coordinate (inclusive)
    min_coord: Coord,

    /// Maximum coordinate (inclusive)
    max_coord: Coord,
}

/// Rectangle dimensions
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Size {
    /// Width of rectangle
    pub width: usize,
    /// Height of rectangle
    pub height: usize,
}

/// Container for 2D data
#[derive(Clone, Debug)]
pub struct Vec2D<T> {
    elems: Vec<T>,
    size: Size,
}

/// Iterator over a rectangle within a Vec2D
pub struct RectIter<'a, Elem: 'a> {
    grid: std::marker::PhantomData<&'a Vec2D<Elem>>,

    rect: Rect,
    cur_elem: *const Elem,
    cur_coord: Coord,
    stride: isize,
}

/// Mutable iterator over a rectangle within a Vec2D
pub struct RectIterMut<'a, Elem: 'a> {
    grid: std::marker::PhantomData<&'a mut Vec2D<Elem>>,

    rect: Rect,
    cur_elem: *mut Elem,
    cur_coord: Coord,
    stride: isize,
}

impl Coord {
    /// Create a coordinate at (x, y)
    pub fn new(x: usize, y: usize) -> Coord {
        Coord { x: x, y: y }
    }
}

impl std::ops::Add for Coord {
    type Output = Coord;

    fn add(self, other: Coord) -> Coord {
        Coord::new(self.x + other.x, self.y + other.y)
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

    /// Return true if the coordinate is between `min_coord` and
    /// `max_coord` (inclusive).
    pub fn contains_coord(&self, coord: Coord) -> bool {
        (coord.x >= self.min_coord.x
            && coord.x <= self.max_coord.x
            && coord.y >= self.min_coord.y
            && coord.y <= self.max_coord.y)
    }
}

impl Size {
    /// Create a 2D size of (width, height)
    pub fn new(width: usize, height: usize) -> Size {
        Size {
            width: width,
            height: height,
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

    /// Create a rectangle starting at (0, 0) with `self`'s size.
    pub fn rect(&self) -> Rect {
        Rect {
            min_coord: Coord::new(0, 0),
            max_coord: Coord::new(self.width - 1, self.height - 1),
        }
    }
}

impl<Elem: Clone> Vec2D<Elem> {
    /// Create a Vec2D with the given `size`. All elements are
    /// initialized as copies of the `example` element.
    ///
    /// ```
    /// # use vec2d::{Vec2D, Size};
    /// let vector = Vec2D::from_example(Size::new(10, 10), &42);
    /// for (_coord, &item) in vector.iter() {
    ///     assert_eq!(item, 42);
    /// }
    /// ```
    pub fn from_example(size: Size, example: &Elem) -> Vec2D<Elem> {
        Vec2D {
            elems: vec![example.clone(); size.area()],
            size: size,
        }
    }

    /// Resize in-place so that `size()` is equal to `new_size`
    pub fn resize(&mut self, new_size: Size, value: Elem) {
        self.elems.resize(new_size.area(), value);
        self.size = new_size;
    }
}

impl<Elem> Vec2D<Elem> {
    /// Create a Vec2D with the given `size`. The contents are set to
    /// `src`. None is returned if the `size` does not match the
    /// length of `src`.
    pub fn from_vec(size: Size, src: Vec<Elem>) -> Option<Vec2D<Elem>> {
        if size.area() == src.len() {
            Some(Vec2D {
                elems: src,
                size: size,
            })
        } else {
            None
        }
    }

    /// Returns element at the given coord or `None` if the coord is
    /// outside the Vec2D
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate vec2d; use vec2d::*;
    /// # fn main () {
    /// let v = Vec2D::from_vec (
    ///   Size { width: 3, height: 3 },
    ///   vec!['a','b','c','d','e','f','g','h','i']
    /// ).unwrap();
    /// assert_eq!(v.get (Coord { x: 1, y: 0 }), Some(&'b'));
    /// assert_eq!(v.get (Coord { x: 1, y: 2 }), Some(&'h'));
    /// assert_eq!(v.get (Coord { x: 3, y: 0 }), None);
    /// # }
    /// ```
    pub fn get(&self, coord: Coord) -> Option<&Elem> {
        if self.size.contains_coord(coord) {
            // column major coords
            let i = coord.y * self.size.width + coord.x;
            return Some(&self.elems[i]);
        }
        None
    }

    /// Returns a mutable reference to the element at the given coord or
    /// `None` if the coord is outside the Vec2D
    ///
    /// # Example
    ///
    /// ```
    /// # extern crate vec2d; use vec2d::*;
    /// # fn main () {
    /// let mut v = Vec2D::from_vec (
    ///   Size { width: 3, height: 3 },
    ///   vec!['a','b','c','d','e','f','g','h','i']
    /// ).unwrap();
    /// assert_eq!(v.get_mut (Coord { x: 1, y: 0 }), Some(&mut 'b'));
    /// assert_eq!(v.get_mut (Coord { x: 1, y: 2 }), Some(&mut 'h'));
    /// assert_eq!(v.get_mut (Coord { x: 3, y: 0 }), None);
    /// # }
    /// ```
    pub fn get_mut(&mut self, coord: Coord) -> Option<&mut Elem> {
        if self.size.contains_coord(coord) {
            // column major coords
            let i = coord.y * self.size.width + coord.x;
            return Some(&mut self.elems[i]);
        }
        None
    }

    /// Shortcut for self.size.rect()
    pub fn rect(&self) -> Rect {
        self.size.rect()
    }

    /// Width and height
    pub fn size(&self) -> Size {
        self.size
    }

    fn stride(&self, rect: &Rect) -> isize {
        (self.size.width + 1 - rect.width()) as isize
    }

    /// Calculate pointer offset for `start` element.
    fn start_offset(&self, start: Coord) -> isize {
        debug_assert_eq!(self.size.contains_coord(start), true);
        (start.y * self.size.width + start.x) as isize
    }

    /// Iterator over the entire Vec2D.
    pub fn iter<'a>(&'a self) -> RectIter<'a, Elem> {
        self.rect_iter(self.size.rect()).unwrap()
    }

    /// Create an iterator over a rectangular region of the
    /// Vec2D. None is returned if the given `rect` does not fit
    /// entirely within the Vec2D.
    pub fn rect_iter<'a>(&'a self, rect: Rect) -> Option<RectIter<'a, Elem>> {
        self.rect_iter_at(rect, rect.min_coord)
    }

    /// Create an iterator over a rectangular region of the Vec2D with
    /// the `start` coord. None is returned if the given `rect` does
    /// not fit entirely within the Vec2D or if the `start` coord is
    /// not within `rect`.
    pub fn rect_iter_at<'a>(&'a self, rect: Rect, start: Coord) -> Option<RectIter<'a, Elem>> {
        if self.size.contains_coord(rect.max_coord) && rect.contains_coord(start) {
            Some(RectIter {
                grid: std::marker::PhantomData,
                stride: self.stride(&rect),
                cur_elem: unsafe { self.elems.as_ptr().offset(self.start_offset(start)) },
                rect: rect,
                cur_coord: start,
            })
        } else {
            None
        }
    }

    /// Mutable iterater over the entire Vec2D.
    pub fn iter_mut<'a>(&'a mut self) -> RectIterMut<'a, Elem> {
        let rect = self.size.rect();
        self.rect_iter_mut(rect).unwrap()
    }

    /// Create a mutable iterator over a rectangular region of the
    /// Vec2D. None is returned if the given `rect` does not fit
    /// entirely within the Vec2D.
    pub fn rect_iter_mut<'a>(&'a mut self, rect: Rect) -> Option<RectIterMut<'a, Elem>> {
        self.rect_iter_mut_at(rect, rect.min_coord)
    }

    /// Create a mutable iterator over a rectangular region of the
    /// Vec2D with the `start` coord. None is returned if the given
    /// `rect` does not fit entirely within the Vec2D or if the
    /// `start` coord is not within `rect`.
    pub fn rect_iter_mut_at<'a>(
        &'a mut self,
        rect: Rect,
        start: Coord,
    ) -> Option<RectIterMut<'a, Elem>> {
        if self.size.contains_coord(rect.max_coord) && rect.contains_coord(start) {
            Some(RectIterMut {
                grid: std::marker::PhantomData,
                stride: self.stride(&rect),
                cur_elem: unsafe { self.elems.as_mut_ptr().offset(self.start_offset(start)) },
                rect: rect,
                cur_coord: start,
            })
        } else {
            None
        }
    }
}

impl<'a, Elem> Iterator for RectIter<'a, Elem> {
    type Item = (Coord, &'a Elem);

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_coord.y <= self.rect.max_coord.y {
            let result = (self.cur_coord, unsafe { &*self.cur_elem });

            self.cur_coord.x += 1;
            if self.cur_coord.x <= self.rect.max_coord.x {
                unsafe {
                    self.cur_elem = self.cur_elem.offset(1);
                }
            } else {
                self.cur_coord.x = self.rect.min_coord.x;
                self.cur_coord.y += 1;
                unsafe {
                    self.cur_elem = self.cur_elem.offset(self.stride);
                }
            }
            Some(result)
        } else {
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
                unsafe {
                    self.cur_elem = self.cur_elem.offset(1);
                }
            } else {
                self.cur_coord.x = self.rect.min_coord.x;
                self.cur_coord.y += 1;
                unsafe {
                    self.cur_elem = self.cur_elem.offset(self.stride);
                }
            }
            Some(result)
        } else {
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
                max_coord: max_coord,
            })
        } else {
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
    fn test_coord_add() {
        let a = Coord::new(1, 2);
        let b = Coord::new(5, 9);
        assert_eq!(a + b, Coord::new(6, 11));
    }

    #[test]
    fn test_rect() {
        let rect = Rect::new(Coord::new(1, 2), Coord::new(5, 3)).unwrap();
        assert_eq!(rect.width(), 5);
        assert_eq!(rect.height(), 2);

        assert_eq!(rect.width(), rect.size().width);
        assert_eq!(rect.height(), rect.size().height);

        assert_eq!(rect.contains_coord(Coord::new(0, 0)), false);
        assert_eq!(rect.contains_coord(Coord::new(4, 3)), true);
    }

    #[test]
    fn test_bad_rect() {
        assert_eq!(
            Rect::new(Coord::new(2, 1), Coord::new(1, 1)).is_none(),
            true
        );
        assert_eq!(
            Rect::new(Coord::new(1, 2), Coord::new(1, 1)).is_none(),
            true
        );
    }

    #[test]
    fn test_size() {
        let size = Size::new(3, 2);
        assert_eq!(size.width, 3);
        assert_eq!(size.height, 2);

        assert_eq!(size.area(), 6);

        assert_eq!(size.contains_coord(Coord::new(1, 1)), true);
        assert_eq!(size.contains_coord(Coord::new(4, 1)), false);
        assert_eq!(size.contains_coord(Coord::new(1, 3)), false);

        let rect = size.rect();
        assert_eq!(rect.min_coord, Coord::new(0, 0));
        assert_eq!(rect.max_coord, Coord::new(2, 1));
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

    #[test]
    fn test_two_iterators() {
        let size = Size::new(2, 1);
        let v = Vec2D::from_vec(size, vec![0, 1]).unwrap();

        let iter1 = v.rect_iter(size.rect()).unwrap();
        let iter2 = v.rect_iter(size.rect()).unwrap();

        for ((coord1, elem1), (coord2, elem2)) in iter1.zip(iter2) {
            assert_eq!(coord1, coord2);
            assert_eq!(elem1, elem2);
        }
    }

    #[test]
    fn test_rect_iter_at() {
        let size = Size::new(1, 2);
        let v = Vec2D::from_vec(size, vec![0, 1]).unwrap();

        let start = Coord::new(0, 1);
        let mut iter = v.rect_iter_at(size.rect(), start).unwrap();
        let (coord, elem) = iter.next().unwrap();
        assert_eq!((coord, *elem), (start, 1));
        assert_eq!(iter.next().is_none(), true);
    }
}

pub mod template;

use core::str;
use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Write;
use std::sync::LazyLock;

use forward_ref::{forward_ref_binop, forward_ref_op_assign};

// Use this file to add helper functions and additional modules.

#[derive(Clone)]
pub struct Matrix<T> {
    pub rows: isize,
    pub cols: isize,
    pub data: Vec<T>,
}

impl<T: Clone> Matrix<T> {
    pub fn empty(rows: usize, cols: usize, filler: T) -> Matrix<T> {
        Matrix {
            cols: cols as isize,
            rows: rows as isize,
            data: vec![filler; rows * cols],
        }
    }
}

impl<T: PartialEq> Matrix<T> {
    pub fn from(rows: usize, cols: usize, data: Vec<T>) -> Matrix<T> {
        assert_eq!(data.len(), rows * cols);
        Matrix {
            cols: cols as isize,
            rows: rows as isize,
            data,
        }
    }

    pub fn get(&self, point: &Point) -> Option<&T> {
        if point.0 < 0 || point.1 < 0 || point.0 >= self.rows || point.1 >= self.cols {
            return None;
        }

        let pos = point.0 * self.cols + point.1;

        let chr = self.data.get(pos as usize).expect("Checked");
        Some(chr)
    }

    pub fn get_mut(&mut self, point: &Point) -> Option<&mut T> {
        if point.0 < 0 || point.1 < 0 || point.0 >= self.rows || point.1 >= self.cols {
            return None;
        }

        let pos = point.0 * self.cols + point.1;

        let chr = self.data.get_mut(pos as usize).expect("Checked");
        Some(chr)
    }

    pub fn update(&mut self, point: &Point, chr: T) -> Option<T> {
        if point.0 < 0 || point.1 < 0 || point.0 >= self.rows || point.1 >= self.cols {
            return None;
        }

        let pos = (point.0 * self.cols + point.1) as usize;
        let old = self.data.splice(pos..=pos, [chr]).last();
        old
    }

    pub fn as_points<'a>(&self) -> Box<dyn Iterator<Item = Point> + 'a> {
        let rows = self.rows;
        let cols = self.cols;
        Box::from((0..rows).flat_map(move |x| (0..cols).map(move |y| Point(x, y))))
    }

    pub fn find(&self, needle: &T) -> Option<Point> {
        for p in self.as_points() {
            if let Some(chr) = self.get(&p) {
                if *chr == *needle {
                    return Some(p);
                }
            }
        }
        None
    }
}

impl Display for Matrix<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in 0..self.rows {
            if x > 0 {
                f.write_char('\n')?;
            }
            for y in 0..self.cols {
                let value = self.get(&Point(x, y)).expect("Checked");
                f.write_str(str::from_utf8(&[*value]).unwrap()).unwrap();
            }
        }
        Ok(())
    }
}

impl<T: Display + PartialEq> Debug for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in 0..self.rows {
            if x > 0 {
                f.write_char('\n')?;
            }
            for y in 0..self.cols {
                let p = Point(x, y);
                let value = self.get(&p).expect("Checked");
                f.write_str(value.to_string().as_str()).unwrap();
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point(pub isize, pub isize);

impl Point {
    pub fn distance(&self, other: Self) -> isize {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.0, self.1))
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

forward_ref_binop!(impl Add, add for Point, Point);

impl AddAssign for Point {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
    }
}

forward_ref_op_assign!(impl AddAssign, add_assign for Point, Point);

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

forward_ref_binop!(impl Sub, sub for Point, Point);

impl SubAssign for Point {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
        self.1 -= other.1;
    }
}

forward_ref_op_assign!(impl SubAssign, sub_assign for Point, Point);

impl Mul<isize> for Point {
    type Output = Self;

    fn mul(self, factor: isize) -> Self::Output {
        Self(self.0 * factor, self.1 * factor)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up(Point),
    Down(Point),
    Left(Point),
    Right(Point),
}

impl Direction {
    pub const fn as_point(self) -> Point {
        match self {
            Direction::Up(p) => p,
            Direction::Down(p) => p,
            Direction::Left(p) => p,
            Direction::Right(p) => p,
        }
    }

    pub const fn opposite_point(self) -> Point {
        match self {
            Direction::Up(_) => DOWN,
            Direction::Down(_) => UP,
            Direction::Left(_) => RIGHT,
            Direction::Right(_) => LEFT,
        }
        .as_point()
    }

    pub const fn opposite_direction(self) -> Self {
        match self {
            Direction::Up(_) => DOWN,
            Direction::Down(_) => UP,
            Direction::Left(_) => RIGHT,
            Direction::Right(_) => LEFT,
        }
    }

    pub fn from_point(point: Point) -> Self {
        match point {
            Point(-1, 0) => UP,
            Point(1, 0) => DOWN,
            Point(0, -1) => LEFT,
            Point(0, 1) => RIGHT,
            _ => panic!("This is not a direction: {}", point),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Direction::Up(_) => '^',
            Direction::Down(_) => 'v',
            Direction::Left(_) => '<',
            Direction::Right(_) => '>',
        })
    }
}

impl Add for Direction {
    type Output = Point;

    fn add(self, other: Self) -> Point {
        self.as_point() + other.as_point()
    }
}

forward_ref_binop!(impl Add, add for Direction, Direction);

pub static UP: Direction = Direction::Up(Point(-1, 0));
pub static DOWN: Direction = Direction::Down(Point(1, 0));
pub static LEFT: Direction = Direction::Left(Point(0, -1));
pub static RIGHT: Direction = Direction::Right(Point(0, 1));

pub static ALL_4_DIRECTIONS: [Direction; 4] = [UP, DOWN, LEFT, RIGHT];
pub static ALL_8_POINTS: LazyLock<[Point; 8]> = std::sync::LazyLock::new(|| {
    [
        UP.as_point(),
        UP + RIGHT,
        RIGHT.as_point(),
        RIGHT + DOWN,
        DOWN.as_point(),
        DOWN + LEFT,
        LEFT.as_point(),
        LEFT + UP,
    ]
});

impl Add<Direction> for Point {
    type Output = Self;

    fn add(self, other: Direction) -> Self {
        Self(self.0 + other.as_point().0, self.1 + other.as_point().1)
    }
}

impl AddAssign<Direction> for Point {
    #[inline]
    fn add_assign(&mut self, other: Direction) {
        self.0 += other.as_point().0;
        self.1 += other.as_point().1;
    }
}

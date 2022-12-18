use num::PrimInt;

use crate::math::Vec2D;

pub fn min_coordinates<T: PrimInt + std::fmt::Debug>(fst: Vec2D<T>, snd: Vec2D<T>) -> Vec2D<T> {
    let mut min: Vec2D<T> = fst;

    if snd.x < min.x { min.x = snd.x }
    if snd.y < min.y { min.y = snd.y }

    min
}

pub fn max_coordinates<T: PrimInt + std::fmt::Debug>(fst: Vec2D<T>, snd: Vec2D<T>) -> Vec2D<T> {
    let mut max: Vec2D<T> = fst;

    if snd.x > max.x { max.x = snd.x }
    if snd.y > max.y { max.y = snd.y }

    max
}

pub fn min_max_coordinates<T: PrimInt + std::fmt::Debug>(list: Vec<Vec2D<T>>) -> (Vec2D<T>, Vec2D<T>) {
    let mut min_coordinate: Vec2D<T> = Vec2D::new(T::max_value(), T::max_value());
    let mut max_coordinate: Vec2D<T> = Vec2D::new(T::min_value(), T::min_value());

    for item in list {
        if item.x < min_coordinate.x { min_coordinate.x = item.x }
        if item.y < min_coordinate.y { min_coordinate.y = item.y }
        if item.x > max_coordinate.x { max_coordinate.x = item.x }
        if item.y > max_coordinate.y { max_coordinate.y = item.y }
    }

    (min_coordinate, max_coordinate)
}

// method cannot be called on `std::ops::RangeInclusive<T>` due to unsatisfied trait bounds
// use of unstable library feature 'step_trait': recently redesigned
pub fn display_i64(
    min: Vec2D<i64>,
    max: Vec2D<i64>,
    show: impl Fn(Vec2D<i64>) -> char,
) -> Vec<String>
{
    (min.y..=max.y).map(|y| {
        (min.x..=max.x).map(|x| {
            let pos = Vec2D::new(x, y);
            show(pos)
        }).collect::<String>()
    }).collect::<Vec<String>>()
}

#[cfg(test)]
mod tests {
    use crate::math::Vec2D;

    #[test]
    fn min_coordinates() {
        assert_eq!(
            super::min_coordinates(Vec2D::new(10, 10), Vec2D::new(15, -3)),
            Vec2D::new(10, -3),
        )
    }

    #[test]
    fn max_coordinates() {
        assert_eq!(
            super::max_coordinates(Vec2D::new(10, 10), Vec2D::new(15, -3)),
            Vec2D::new(15, 10),
        )
    }

    #[test]
    fn min_max_coordinates() {
        let input = vec![
            Vec2D::new(10, 10),
            Vec2D::new(15, -3),
            Vec2D::new(-3, 5),
            Vec2D::new(21, -17),
        ];
        let expected_min = Vec2D::new(-3, -17);
        let expected_max = Vec2D::new(21, 10);

        assert_eq!(super::min_max_coordinates(input), (expected_min, expected_max));
    }

    #[test]
    fn display_i64() {
        let output = super::display_i64(
            Vec2D::new(-2, -2),
            Vec2D::new(2, 2),
            |pos| if pos.x == 0 || pos.y == 0 { '#' } else { '.' },
        );
        let expected = vec![
            "..#..",
            "..#..",
            "#####",
            "..#..",
            "..#..",
        ].iter().map(|f| f.to_string()).collect::<Vec<String>>();

        assert_eq!(output, expected);
    }
}
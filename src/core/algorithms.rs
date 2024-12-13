//! Miscelanious algorithms for grids.

use crate::lib::*;

/// Error for flood_fill
#[derive(Debug)]
pub enum FFError {
    /// Denotes an invalid seed point was provided.
    InvalidSeed,
}

impl Display for FFError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FFError::InvalidSeed => {
                write!(f, "provided seed is out of bounds of the provided array")
            }
        }
    }
}

/// Flood fill aglorithm, fills a bounded area.
///
/// The `seed` parameter **must** be a valid index of `in_arr` else an [`FFError::InvalidSeed`] error will be returned.
///
/// The `pred` functor takes in two references, this is in the order `Fn(&element, &target) -> bool`: Element is the
/// actual value at the index being compared, and target is the value selected by the seed. The functor should return
/// true if the coordinate should be filled, false if not.
///
/// # Example
/// ```
/// use ndarray::array;
/// use gridava::core::algorithms::flood_fill;
///
/// let mut arr = array![
///     [0, 1, 0],
///     [0, 1, 0],
///     [0, 1, 0]];
///
/// // Seed points to index 0,0 of the array so e2 will always be 0 in the predicate.
/// flood_fill(&mut arr, (0, 0), 3, |e1: &i32, e2: &i32| { e1 == e2 });
///
/// assert_eq!(arr, array![
///     [3, 1, 0],
///     [3, 1, 0],
///     [3, 1, 0]]);
/// ```
/// Uses the Span-filling algorithm
#[cfg(any(feature = "std", feature = "alloc"))]
pub fn flood_fill<T, F>(
    in_arr: &mut Array2<T>,
    seed: (i32, i32),
    value: T,
    mut pred: F,
) -> Result<(), FFError>
where
    T: Clone,
    F: FnMut(&T, &T) -> bool,
{
    let (seed_x, seed_y) = seed;

    // Bounds check
    if in_arr.get((seed_x as usize, seed_y as usize)).is_none() {
        return Err(FFError::InvalidSeed);
    }

    // Target value to fill
    let target = &in_arr[[seed_x as usize, seed_y as usize]].clone();

    // Inside function, does a bounds check and asks the predicate whether or not to fill
    let mut inside = |x, y, arr: &Array2<T>| {
        if let Some(ele) = arr.get((x as usize, y as usize)) {
            pred(ele, target)
        } else {
            false
        }
    };

    // Working stack.
    let mut stack = vec![
        (seed_x, seed_x, seed_y, 1),
        (seed_x, seed_x, seed_y - 1, -1),
    ];

    // Process the stack
    while let Some((mut x1, x2, y, dy)) = stack.pop() {
        let mut x = x1;

        // Expand to the left.
        if inside(x, y, in_arr) {
            while inside(x - 1, y, in_arr) {
                in_arr[[(x - 1) as usize, y as usize]] = value.clone();
                x -= 1;
            }
            if x < x1 {
                stack.push((x, x1 - 1, y - dy, -dy));
            }
        }

        // Expand to the right and handle spans.
        while x1 <= x2 {
            while inside(x1, y, in_arr) {
                in_arr[[x1 as usize, y as usize]] = value.clone();
                x1 += 1;
            }
            if x1 > x {
                stack.push((x, x1 - 1, y + dy, dy));
            }
            if x1 - 1 > x2 {
                stack.push((x2 + 1, x1 - 1, y - dy, -dy));
            }

            x1 += 1;
            while x1 < x2 && !inside(x1, y, in_arr) {
                x1 += 1;
            }
            x = x1;
        }
    }
    Ok(())
}

#[cfg(all(test, any(feature = "std", feature = "alloc")))]
mod tests {
    use alloc::format;

    use super::*;

    #[test]
    fn fmt() {
        let err = FFError::InvalidSeed;
        assert!(format!("{err}") == "provided seed is out of bounds of the provided array")
    }

    #[test]
    fn flood_fill() {
        let arr = array![
            [0, 0, 0, 3, 0, 0, 1, 0],
            [0, 0, 0, 3, 0, 0, 1, 0],
            [0, 0, 0, 3, 0, 0, 1, 0],
            [2, 2, 2, 3, 0, 0, 1, 0],
            [0, 0, 0, 3, 1, 0, 1, 0],
            [0, 0, 0, 3, 0, 1, 1, 0],
            [0, 0, 0, 3, 0, 0, 1, 0],
            [0, 0, 0, 3, 0, 0, 1, 0]
        ];

        let eq_pred = |e1: &i32, e2: &i32| e1 == e2;

        // Invalid index, should error
        assert!(super::flood_fill(&mut arr.clone(), (-1, -1), 3, eq_pred).is_err());

        let mut arr1 = arr.clone();
        assert!(super::flood_fill(&mut arr1, (0, 0), 1, eq_pred).is_ok());
        assert_eq!(
            arr1,
            array![
                [1, 1, 1, 3, 0, 0, 1, 0],
                [1, 1, 1, 3, 0, 0, 1, 0],
                [1, 1, 1, 3, 0, 0, 1, 0],
                [2, 2, 2, 3, 0, 0, 1, 0],
                [0, 0, 0, 3, 1, 0, 1, 0],
                [0, 0, 0, 3, 0, 1, 1, 0],
                [0, 0, 0, 3, 0, 0, 1, 0],
                [0, 0, 0, 3, 0, 0, 1, 0]
            ]
        );

        let lt_pred = |e1: &i32, _e2: &i32| *e1 < 3;

        let mut arr2 = arr.clone();
        assert!(super::flood_fill(&mut arr2, (0, 0), 3, lt_pred).is_ok());
        assert_eq!(
            arr2,
            array![
                [3, 3, 3, 3, 0, 0, 1, 0],
                [3, 3, 3, 3, 0, 0, 1, 0],
                [3, 3, 3, 3, 0, 0, 1, 0],
                [3, 3, 3, 3, 0, 0, 1, 0],
                [3, 3, 3, 3, 1, 0, 1, 0],
                [3, 3, 3, 3, 0, 1, 1, 0],
                [3, 3, 3, 3, 0, 0, 1, 0],
                [3, 3, 3, 3, 0, 0, 1, 0]
            ]
        );

        const RANGE: i32 = 2;

        let range_pred = |e1: &i32, e2: &i32| {
            let upper = *e1 + RANGE;
            let lower = *e1 - RANGE;
            *e2 <= upper && *e2 >= lower
        };

        let mut arr3 = array![
            [0, 4, 0, 3, 0, 0, 1, 0],
            [0, 4, 0, 3, 0, 0, 1, 0],
            [0, 4, 0, 3, 0, 0, 1, 0],
            [2, 2, 4, 3, 0, 0, 1, 0],
            [0, 0, 0, 4, 1, 0, 1, 0],
            [0, 0, 0, 4, 0, 1, 1, 0],
            [0, 0, 0, 4, 0, 0, 1, 0],
            [0, 0, 0, 4, 0, 0, 1, 0]
        ];
        assert!(super::flood_fill(&mut arr3, (0, 0), 1, range_pred).is_ok());
        assert_eq!(
            arr3,
            array![
                [1, 4, 0, 3, 0, 0, 1, 0],
                [1, 4, 0, 3, 0, 0, 1, 0],
                [1, 4, 0, 3, 0, 0, 1, 0],
                [1, 1, 4, 3, 0, 0, 1, 0],
                [1, 1, 1, 4, 1, 0, 1, 0],
                [1, 1, 1, 4, 0, 1, 1, 0],
                [1, 1, 1, 4, 0, 0, 1, 0],
                [1, 1, 1, 4, 0, 0, 1, 0]
            ]
        );

        let mut arr4 = arr.clone();
        assert!(super::flood_fill(&mut arr4, (4, 5), 1, eq_pred).is_ok());
        assert_eq!(
            arr4,
            array![
                [0, 0, 0, 3, 1, 1, 1, 0],
                [0, 0, 0, 3, 1, 1, 1, 0],
                [0, 0, 0, 3, 1, 1, 1, 0],
                [2, 2, 2, 3, 1, 1, 1, 0],
                [0, 0, 0, 3, 1, 1, 1, 0],
                [0, 0, 0, 3, 0, 1, 1, 0],
                [0, 0, 0, 3, 0, 0, 1, 0],
                [0, 0, 0, 3, 0, 0, 1, 0]
            ]
        );
    }
}

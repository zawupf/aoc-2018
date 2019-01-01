const GRID_SIZE: usize = 300;

type Grid = [Row; GRID_SIZE];
type Row = [i32; GRID_SIZE];

fn make_grid(serial_number: i32) -> Grid {
    let mut grid: Grid = [[0; GRID_SIZE]; GRID_SIZE];
    grid.iter_mut().enumerate().for_each(|(y, row)| {
        row.iter_mut().enumerate().for_each(|(x, cell)| {
            let coordinates = (x as i32 + 1, y as i32 + 1);
            *cell = power_level(coordinates, serial_number);
        });
    });
    grid
}

fn power_level((x, y): (i32, i32), grid_serial_number: i32) -> i32 {
    let rack_id = x + 10;
    hundreds_digit((rack_id * y + grid_serial_number) * rack_id) - 5
}

fn hundreds_digit(number: i32) -> i32 {
    (number % 1000) / 100
}

fn total_power(grid: &Grid, (x, y): (usize, usize), size: usize) -> i32 {
    grid[y..y + size]
        .iter()
        .map(|row| row[x..x + size].iter())
        .flatten()
        .sum::<i32>()
}

pub fn largest_total_power_coords(serial_number: i32, size: usize) -> ((i32, i32), i32) {
    let grid = make_grid(serial_number);

    let total_power_level = |(_coords, total_power_level): &((i32, i32), i32)| *total_power_level;

    (0..=(GRID_SIZE - size))
        .map(|y| {
            (0..=(GRID_SIZE - size))
                .map(|x| {
                    (
                        (x as i32 + 1, y as i32 + 1),
                        total_power(&grid, (x, y), size),
                    )
                })
                .max_by_key(total_power_level)
                .unwrap()
        })
        .max_by_key(total_power_level)
        .unwrap()
}

pub fn largest_total_power_coords_any_size(serial_number: i32) -> ((i32, i32, i32), i32) {
    (1..=GRID_SIZE)
        .map(|size| {
            let ((x, y), power) = largest_total_power_coords(serial_number, size);
            ((x, y, size as i32), power)
        })
        .max_by_key(|(_, power)| *power)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cell_new() {
        assert_eq!(4, power_level((3, 5), 8));
        assert_eq!(-5, power_level((122, 79), 57));
        assert_eq!(0, power_level((217, 196), 39));
        assert_eq!(4, power_level((101, 153), 71));
    }

    #[test]
    fn largest_total_power_coords() {
        use super::largest_total_power_coords;
        assert_eq!(((33, 45), 29), largest_total_power_coords(18, 3));
        assert_eq!(((21, 61), 30), largest_total_power_coords(42, 3));
    }

    #[test]
    fn largest_total_power_coords_any_size() {
        use super::largest_total_power_coords_any_size;
        assert_eq!(
            ((90, 269, 16), 113),
            largest_total_power_coords_any_size(18)
        );
        assert_eq!(
            ((232, 251, 12), 119),
            largest_total_power_coords_any_size(42)
        );
    }
}

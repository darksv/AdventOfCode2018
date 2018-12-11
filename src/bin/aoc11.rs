type Grid = [[i32; 300]; 300];

fn main() {
    let grid = create_grid(9110);
    println!("Best 3x3 = {:?}", best3x3(&grid).unwrap());
    println!("ID of best square = {}", {
        let (x, y, size) = best_ever(&grid).unwrap();
        format!("{},{},{}", x, y, size)
    });
}

fn create_grid(serial: u32) -> Grid {
    let mut grid = [[0; 300]; 300];
    for x in 1..=300 {
        for y in 1..=300 {
            grid[x - 1][y - 1] = power(x as u32, y as u32, serial);
        }
    }
    grid
}

#[inline(always)]
fn power_of_square(grid: &Grid, x: usize, y: usize, size: usize) -> i32 {
    grid
        .iter()
        .skip(x)
        .take(size)
        .flat_map(|column| column
            .iter()
            .skip(y)
            .take(size))
        .sum()
}

fn best3x3(grid: &Grid) -> Option<(usize, usize)> {
    let mut max = (i32::min_value(), None);
    for x in 1..=298 {
        for y in 1..=298 {
            let i = x - 1;
            let j = y - 1;

            let sum = power_of_square(grid, i, j, 3);
            if sum > max.0 {
                max = (sum, Some((x, y)));
            }
        }
    }
    max.1
}

fn best_ever(grid: &Grid) -> Option<(usize, usize, usize)> {
    let mut max = (i32::min_value(), None);

    for size in 1..=300 {
        for x in 1..=(300 + 1 - size) {
            for y in 1..=(300 + 1 - size) {
                let i = x - 1;
                let j = y - 1;
                let power = power_of_square(grid, i, j, size);
                if power > max.0 {
                    max = (power, Some((x, y, size)));
                }
            }
        }

//        eprintln!("size = {:#?}", size);
    }
    max.1
}

fn power(x: u32, y: u32, serial: u32) -> i32 {
    let rack_id = x + 10;
    let starts_at = rack_id * y;
    let a = starts_at + serial;
    let b = a * rack_id;
    let h = (b / 100) % 10;
    h as i32 - 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(power(122, 79, 57), -5);
        assert_eq!(power(217, 196, 39), 0);
        assert_eq!(power(101, 153, 71), 4);
    }
}
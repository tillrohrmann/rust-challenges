use aoc_common::GenericResult;
use std::iter::Enumerate;
use std::slice::Iter;

#[derive(Debug)]
struct Map<T> {
    width: usize,
    height: usize,
    map: Vec<T>,
}

impl Map<u8> {
    fn parse(input: &Vec<String>) -> GenericResult<Map<u8>> {
        let height = input.len();
        let width = input.get(0).map(|line| line.len()).unwrap_or(0);

        let map = input
            .iter()
            .flat_map(|line| line.trim().chars().map(|digit| (digit as u8) - ('0' as u8)))
            .collect();

        Ok(Map { width, height, map })
    }
}

impl<T> Map<T>
where
    T: Default + Clone,
{
    fn new(width: usize, height: usize) -> Map<T> {
        let map = vec![T::default(); width * height];

        Map { width, height, map }
    }
    fn get(&self, x: isize, y: isize) -> Option<&T> {
        self.index(x, y).map(|index| self.map.get(index)).flatten()
    }

    fn index(&self, x: isize, y: isize) -> Option<usize> {
        if x < 0 || x >= self.width as isize || y < 0 || y >= self.height as isize {
            None
        } else {
            Some((x + y * self.width as isize) as usize)
        }
    }

    pub fn set(&mut self, x: isize, y: isize, value: T) {
        self.index(x, y).map(|index| self.map[index] = value);
    }

    fn entry_iter(&self) -> EntryIterator<T> {
        EntryIterator {
            iter: self.map.iter().enumerate(),
            width: self.width,
        }
    }

    fn size(&self) -> usize {
        self.width * self.height
    }

    fn height(&self) -> usize {
        self.height
    }

    fn width(&self) -> usize {
        self.width
    }
}

struct EntryIterator<'a, T> {
    iter: Enumerate<Iter<'a, T>>,
    width: usize,
}

impl<'a, T> Iterator for EntryIterator<'a, T>
where
    T: Copy + Clone,
{
    type Item = (usize, usize, T);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(idx, value)| (idx % self.width, idx / self.width, *value))
    }
}

pub fn find_danger_points(input: &Vec<String>) -> GenericResult<u32> {
    let map = Map::parse(input)?;

    let mut danger_points = Vec::new();

    let positions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    for (x, y, value) in map.entry_iter() {
        let danger_point = positions
            .iter()
            .flat_map(|&(diff_x, diff_y)| map.get(diff_x + x as isize, diff_y + y as isize))
            .all(|&map_value| map_value > value);

        if danger_point {
            danger_points.push((value + 1) as u32);
        }
    }

    Ok(danger_points.iter().sum())
}

pub fn find_largest_basins(input: &Vec<String>) -> GenericResult<u32> {
    let map = Map::parse(input)?;
    let mut basin_map: Map<bool> = Map::new(map.width(), map.height());
    let mut basin_sizes = Vec::new();

    let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

    for (x, y, value) in map.entry_iter() {
        if is_free_basin_field(x as isize, y as isize, &map, &basin_map) {
            let mut basin_size = 0;
            let mut positions = vec![(x as isize, y as isize)];

            while let Some((x, y)) = positions.pop() {
                if is_free_basin_field(x, y, &map, &basin_map) {
                    basin_size += 1;
                    basin_map.set(x, y, true);

                    directions
                        .iter()
                        .map(|&(x_diff, y_diff)| (x + x_diff, y + y_diff))
                        .filter(|&(x, y)| is_free_basin_field(x, y, &map, &basin_map))
                        .for_each(|position| positions.push(position));
                }
            }

            basin_sizes.push(basin_size);
        }
    }

    basin_sizes.sort_by(|a, b| b.cmp(a));

    let result = basin_sizes.iter().take(3).fold(1, |a, b| a * b);

    Ok(result)
}

fn is_free_basin_field(x: isize, y: isize, map: &Map<u8>, basin_map: &Map<bool>) -> bool {
    map.get(x, y).map(|&value| value < 9).unwrap_or(false)
        && !basin_map.get(x, y).cloned().unwrap_or(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_example() {
        let input: Vec<String> = get_input();
        assert_eq!(find_danger_points(&input).unwrap(), 15);
    }

    fn get_input() -> Vec<String> {
        "2199943210
3987894921
9856789892
8767896789
9899965678"
            .split('\n')
            .map(|line| line.to_string())
            .collect()
    }

    #[test]
    fn basin_example() {
        let input = get_input();
        assert_eq!(find_largest_basins(&input).unwrap(), 1134)
    }
}

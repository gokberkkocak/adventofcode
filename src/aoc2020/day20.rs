use std::collections::{HashMap, HashSet};

pub fn run() {
    let input = crate::util::get_puzzle_input(2020, 20);
    let p1 = part1(&input);
    println!("p1 {}", p1);
}

fn parse(input: &str) -> Vec<Tile> {
    input
        .split("\n\n")
        .map(|tile_info| {
            let tile_id = tile_info
                .lines()
                .next()
                .unwrap()
                .split_ascii_whitespace()
                .skip(1)
                .next()
                .unwrap()
                .strip_suffix(":")
                .unwrap()
                .parse::<u16>()
                .unwrap();
            let tile_matrix = tile_info
                .lines()
                .skip(1)
                .map(|x| x.chars().map(|c| c == '#').collect::<Vec<_>>())
                .collect::<Vec<_>>();
            Tile::new(tile_id, tile_matrix)
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let v = parse(&input);
    println!("{}", v.len());
    let image = WholeImage::construct(parse(&input));
    let c = image
        .tiles
        .iter()
        .filter(|(&(x, y), _)| {
            (x == image.x_min || x == image.x_max) && (y == image.y_min || y == image.y_max)
        })
        .map(|(_, t)| t.id as usize)
        .collect::<Vec<_>>();
    println!("{:?}", c);
    c.iter().product::<usize>()
}

struct WholeImage {
    tiles: HashMap<(isize, isize), Tile>,
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
}

impl WholeImage {
    fn construct(mut tiles: Vec<Tile>) -> Self {
        fn solve(
            x_min: isize,
            x_max: isize,
            y_min: isize,
            y_max: isize,
            image: &mut HashMap<(isize, isize), Tile>,
            tiles: &Vec<Tile>,
            all_v: &Vec<Tile>,
            seen: &mut HashSet<u16>,
            len: u8,
        ) -> bool {
            if tiles.len() != seen.len() {
                for t in all_v {
                    if !seen.contains(&t.id) {
                        seen.insert(t.id);
                        for x in x_min..=x_max {
                            for y in y_min..=y_max {
                                if image.contains_key(&(x, y)) {
                                    let n = is_neighbour(image.get(&(x, y)).unwrap(), &t);
                                    if n.up
                                    // && (y + 1isize <= y_max || y_max - y_min + 1 < len as isize)
                                    // && !image.contains_key(&(x, y + 1))
                                    {
                                        // let mut new_t = t.clone();
                                        // new_t.neighbourhood.down = true;
                                        // image.get_mut(&(x, y)).unwrap().neighbourhood.up = true;
                                        image.insert((x, y + 1), t.clone());
                                        let new_y_max = y_max.max(y + 1);
                                        if solve(
                                            x_min, x_max, y_min, new_y_max, image, tiles, all_v,
                                            seen, len,
                                        ) {
                                            return true;
                                        }
                                        //revert
                                        image.remove(&(x, y + 1)).unwrap();
                                    }
                                    if n.down
                                    // && (y - 1isize >= y_min || y_max - y_min + 1 < len as isize)
                                    // && !image.contains_key(&(x, y - 1))
                                    {
                                        // let mut new_t = t.clone();
                                        // new_t.neighbourhood.up = true;
                                        // image.get_mut(&(x, y)).unwrap().neighbourhood.down = true;
                                        image.insert((x, y - 1), t.clone());
                                        let new_y_min = y_min.min(y - 1);
                                        if solve(
                                            x_min, x_max, new_y_min, y_max, image, tiles, all_v,
                                            seen, len,
                                        ) {
                                            return true;
                                        }
                                        //revert
                                        image.remove(&(x, y - 1)).unwrap();
                                    }
                                    if n.left
                                    // && (x - 1isize >= x_min || x_max - x_min + 1 < len as isize)
                                    // && !image.contains_key(&(x - 1, y))
                                    {
                                        // let mut new_t = t.clone();
                                        // new_t.neighbourhood.right = true;
                                        // image.get_mut(&(x, y)).unwrap().neighbourhood.left = true;
                                        image.insert((x - 1, y), t.clone());
                                        let new_x_min = x_min.min(x - 1);
                                        if solve(
                                            new_x_min, x_max, y_min, y_max, image, tiles, all_v,
                                            seen, len,
                                        ) {
                                            return true;
                                        }
                                        //revert
                                        image.remove(&(x - 1, y)).unwrap();
                                    }
                                    if n.right
                                    // && (x + 1isize <= x_min || x_max - x_min + 1 < len as isize)
                                    // && !image.contains_key(&(x + 1, y))
                                    {
                                        // let mut new_t = t.clone();
                                        // new_t.neighbourhood.left = true;
                                        // image.get_mut(&(x, y)).unwrap().neighbourhood.right = true;
                                        image.insert((x + 1, y), t.clone());
                                        let new_x_max = x_max.max(x + 1);
                                        if solve(
                                            x_min, new_x_max, y_min, y_max, image, tiles, all_v,
                                            seen, len,
                                        ) {
                                            return true;
                                        }
                                        //revert
                                        image.remove(&(x + 1, y)).unwrap();
                                    }
                                }
                            }
                        }
                        seen.remove(&t.id);
                    }
                }
                return false;
            } else {
                return true;
            }
        }
        let len = (tiles.len() as f64).sqrt() as u8;
        let mut image = HashMap::new();
        let mut seen = HashSet::new();
        let mut all_v = vec![];
        let id = tiles[0].id;
        for i in tiles.clone() {
            for j in TileVariationIterator::new(i) {
                all_v.push(j);
            }
        }
        seen.insert(id);
        image.insert((0, 0), tiles[0].clone());
        let ret = solve(0, 0, 0, 0, &mut image, &mut tiles, &all_v, &mut seen, len);
        debug_assert!(ret);
        let x_min = image.iter().map(|(&(x, _y), _)| x).min().unwrap();
        let x_max = image.iter().map(|(&(x, _y), _)| x).max().unwrap();
        let y_min = image.iter().map(|(&(_x, y), _)| y).min().unwrap();
        let y_max = image.iter().map(|(&(_x, y), _)| y).max().unwrap();
        Self {
            tiles: image,
            x_min,
            x_max,
            y_min,
            y_max,
        }
    }
}

#[derive(Debug, Clone)]
struct Tile {
    id: u16,
    image: Vec<Vec<bool>>,
    hashes: TileHash,
    len: usize,
    neighbourhood: Neighbourhood,
    status: u8,
}
#[derive(Debug, Clone, Copy)]
struct Neighbourhood {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl Neighbourhood {
    fn new() -> Self {
        Self {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }
}

impl Tile {
    fn new(id: u16, image: Vec<Vec<bool>>) -> Self {
        let hashes = TileHash::calculate_hashes(&image);
        let len = image.len();
        Self {
            id,
            image,
            hashes,
            len,
            neighbourhood: Neighbourhood::new(),
            status: 0,
        }
    }

    fn flip_vertical(&mut self) {
        for i in 0..self.image.len() / 2 {
            let mut temp = self.image[i].clone();
            std::mem::swap(&mut temp, &mut self.image[self.len - 1 - i]);
            std::mem::swap(&mut temp, &mut self.image[i]);
        }
        std::mem::swap(&mut self.hashes.up_hash, &mut self.hashes.down_hash);
        u16::flip_bits(&mut self.hashes.left_hash);
        u16::flip_bits(&mut self.hashes.right_hash);
    }

    fn flip_horizontal(&mut self) {
        for i in 0..self.image.len() {
            for j in 0..self.image.len() / 2 {
                let mut temp = self.image[i][j];
                std::mem::swap(&mut temp, &mut self.image[i][self.len - 1 - j]);
                std::mem::swap(&mut temp, &mut self.image[i][j]);
            }
        }
        std::mem::swap(&mut self.hashes.left_hash, &mut self.hashes.right_hash);
        u16::flip_bits(&mut self.hashes.up_hash);
        u16::flip_bits(&mut self.hashes.down_hash);
    }

    fn transpose(&mut self) {
        for i in 0..self.image.len() {
            for j in 0..self.image.len() {
                let mut temp = self.image[i][j];
                std::mem::swap(&mut temp, &mut self.image[j][i]);
                std::mem::swap(&mut temp, &mut self.image[i][j]);
            }
        }
        std::mem::swap(&mut self.hashes.up_hash, &mut self.hashes.left_hash);
        std::mem::swap(&mut self.hashes.down_hash, &mut self.hashes.right_hash);
    }
}

fn is_neighbour(tile: &Tile, other: &Tile) -> Neighbourhood {
    let mut n = Neighbourhood::new();
    if !tile.neighbourhood.up && tile.hashes.up_hash == other.hashes.down_hash {
        n.up = true;
    }
    if !tile.neighbourhood.right && tile.hashes.right_hash == other.hashes.left_hash {
        n.right = true;
    }
    if !tile.neighbourhood.left && tile.hashes.left_hash == other.hashes.right_hash {
        n.left = true;
    }
    if !tile.neighbourhood.down && tile.hashes.down_hash == other.hashes.up_hash {
        n.down = true;
    }
    n
}

struct TileVariationIterator {
    tile: Tile,
    status: u8,
}

impl TileVariationIterator {
    fn new(tile: Tile) -> Self {
        Self { tile, status: 0 }
    }
}

impl Iterator for TileVariationIterator {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        let mut cloned = self.tile.clone();
        match self.status {
            0 => (),
            1 => cloned.flip_horizontal(),
            2 => cloned.flip_vertical(),
            3 => {
                cloned.flip_vertical();
                cloned.flip_horizontal();
            }
            4 => cloned.transpose(),
            5 => {
                cloned.transpose();
                cloned.flip_horizontal();
            }
            6 => {
                cloned.transpose();
                cloned.flip_vertical();
            }
            7 => {
                cloned.transpose();
                cloned.flip_horizontal();
                cloned.flip_vertical();
            }
            _ => return None,
        }
        cloned.status = self.status;
        self.status += 1;
        Some(cloned)
    }
}

trait BitFlipper {
    fn flip_bits(num: &mut Self);
}

impl BitFlipper for u16 {
    fn flip_bits(num: &mut u16) {
        *num = u16::from_str_radix(
            &format!("{:010b}", *num).chars().rev().collect::<String>(),
            2,
        )
        .unwrap();
    }
}

#[derive(Debug, Clone, Copy)]
struct TileHash {
    up_hash: u16,
    down_hash: u16,
    left_hash: u16,
    right_hash: u16,
}

impl TileHash {
    fn new() -> Self {
        Self {
            up_hash: 0,
            down_hash: 0,
            left_hash: 0,
            right_hash: 0,
        }
    }

    fn calculate_hashes(image: &Vec<Vec<bool>>) -> Self {
        let mut tile_hash = TileHash::new();
        for i in 0..image.len() {
            tile_hash.up_hash += image[0][i] as u16 * u16::pow(2, i as u32);
            tile_hash.down_hash += image[image.len() - 1][i] as u16 * u16::pow(2, i as u32);
            tile_hash.left_hash += image[i][0] as u16 * u16::pow(2, i as u32);
            tile_hash.right_hash += image[i][image.len() - 1] as u16 * u16::pow(2, i as u32);
        }
        tile_hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        let input = crate::util::read_file("inputs/2020_20_test.in");
        let p1 = part1(&input);
        assert_eq!(p1, 20899048083289);
    }
}

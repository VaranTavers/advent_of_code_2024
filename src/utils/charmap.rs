use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Clone)]
pub struct CharMap {
    pub map: Vec<Vec<char>>,
}

impl CharMap {
    #[must_use]
    pub fn parse_map_string(reader: Vec<String>) -> CharMap {
        let map = reader
            .iter()
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();

        CharMap { map }
    }
    #[must_use]
    pub fn parse_map(reader: BufReader<File>) -> CharMap {
        let map = reader
            .lines()
            .map_while(Result::ok)
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();

        CharMap { map }
    }

    #[must_use]
    pub fn parse_maps(reader: BufReader<File>) -> Vec<CharMap> {
        let mut res = Vec::new();
        let mut map = Vec::new();

        for line in reader.lines().map_while(Result::ok) {
            if line.is_empty() {
                if !map.is_empty() {
                    res.push(CharMap { map });
                    map = Vec::new();
                }
            } else {
                map.push(line.chars().collect());
            }
        }

        if !map.is_empty() {
            res.push(CharMap { map });
        }

        res
    }

    pub fn map_to_val<T: Copy>(&self, val: T) -> Vec<Vec<T>> {
        vec![vec![val; self.map[0].len()]; self.map.len()]
    }

    pub fn clone_to_val<T: Clone>(&self, val: T) -> Vec<Vec<T>> {
        vec![vec![val.clone(); self.map[0].len()]; self.map.len()]
    }

    pub fn map_to<F, T>(&self, f: F) -> Vec<Vec<T>>
    where
        F: Fn(&char) -> T,
    {
        let mut res = Vec::new();

        for line in &self.map {
            res.push(line.iter().map(&f).collect::<Vec<T>>());
        }

        res
    }

    #[must_use]
    pub fn find_first(&self, needle: char) -> Option<(usize, usize)> {
        for (i, row) in self.map.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if *c == needle {
                    return Some((i, j));
                }
            }
        }

        None
    }

    #[must_use]
    pub fn find_all(&self, needle: char) -> Vec<(usize, usize)> {
        let mut res = Vec::new();
        for (i, row) in self.map.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                if *c == needle {
                    res.push((i, j));
                }
            }
        }

        res
    }

    #[must_use]
    pub fn is_valid_coords(&self, (row, col): (usize, usize)) -> bool {
        row < self.map.len() && col < self.map[row].len()
    }

    #[must_use]
    pub fn get(&self, (row, col): (usize, usize)) -> Option<char> {
        self.map.get(row).and_then(|x| x.get(col)).copied()
    }

    #[must_use]
    pub fn iter(&self) -> CharMapIterator {
        CharMapIterator::new(self)
    }
}

impl<'a> IntoIterator for &'a CharMap {
    type Item = (usize, usize, char);

    type IntoIter = CharMapIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl Display for CharMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.map {
            for c in line {
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

pub struct CharMapIterator<'a> {
    row: usize,
    col: usize,
    cmap: &'a CharMap,
}

impl<'a> CharMapIterator<'a> {
    pub fn new(cmap: &'a CharMap) -> Self {
        CharMapIterator {
            row: 0,
            col: 0,
            cmap,
        }
    }
}

impl Iterator for CharMapIterator<'_> {
    type Item = (usize, usize, char);

    fn next(&mut self) -> Option<Self::Item> {
        self.col += 1;
        if self.col >= self.cmap.map[self.row].len() {
            self.col = 0;
            self.row += 1;
        }
        if self.row >= self.cmap.map.len() {
            return None;
        }

        Some((self.row, self.col, self.cmap.map[self.row][self.col]))
    }
}

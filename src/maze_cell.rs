#![allow(dead_code)]

use std::collections::BTreeMap;
#[derive(Debug)]
struct MazeCell {
    links: BTreeMap<Id, bool>,
    row: i32,
    col: i32,
    configuration: Configuration,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Id {
    row: i32,
    col: i32,
}

#[derive(Debug, Default)]
struct Configuration {
    north: Option<Id>,
    south: Option<Id>,
    east: Option<Id>,
    west: Option<Id>,
}

impl MazeCell {
    fn new(row: i32, col: i32) -> Self {
        Self {
            row,
            col,
            links: BTreeMap::new(),
            configuration: Default::default(),
        }
    }

    fn configure(&mut self, configuration: Configuration) {
        self.configuration = configuration;
    }

    fn id(&self) -> Id {
        Id {
            row: self.row,
            col: self.col,
        }
    }

    fn neighbors(&self) -> Vec<Id> {
        [
            self.configuration.north,
            self.configuration.south,
            self.configuration.east,
            self.configuration.west,
        ]
        .iter()
        .flatten()
        .copied()
        .collect()
    }

    fn is_linked(&self, other: &MazeCell) -> bool {
        *self.links.get(&other.id()).unwrap_or(&false)
    }

    fn link(&mut self, other: &mut MazeCell) {
        self.links.insert(other.id(), true);
        other.links.insert(self.id(), true);
    }

    fn links(&self) -> Vec<Id> {
        self.links.keys().copied().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cells_can_be_linked() {
        let mut cell1 = MazeCell::new(1, 1);
        let mut cell2 = MazeCell::new(1, 2);
        assert!(!cell1.is_linked(&cell2));
        assert!(!cell2.is_linked(&cell1));
        cell1.link(&mut cell2);
        assert!(cell1.is_linked(&cell2));
        assert!(cell2.is_linked(&cell1));
    }

    #[test]
    fn can_get_list_of_linked() {
        let mut cell1 = MazeCell::new(1, 1);
        let mut cell2 = MazeCell::new(1, 2);
        let mut cell3 = MazeCell::new(1, 3);
        let mut cell4 = MazeCell::new(1, 4);
        cell2.link(&mut cell1);
        cell2.link(&mut cell3);
        cell3.link(&mut cell4);
        assert_eq!(2, cell2.links().len());
        assert!(cell2.links().contains(&cell1.id()));
        assert!(cell2.links().contains(&cell3.id()));
    }

    #[test]
    fn starts_with_no_neighbors() {
        let cell = MazeCell::new(1, 2);
        assert!(cell.neighbors().is_empty());
    }

    #[test]
    fn knows_its_neighbors() {
        let mut cell1 = MazeCell::new(1, 2);
        let cell2 = MazeCell::new(1, 3);
        let cell3 = MazeCell::new(1, 1);
        cell1.configure(Configuration {
            north: Some(cell2.id()),
            south: Some(cell3.id()),
            ..Default::default()
        });
        assert_eq!(2, cell1.neighbors().len());
        assert!(cell1.neighbors().contains(&cell2.id()));
        assert!(cell1.neighbors().contains(&cell3.id()));
    }
}

use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct MazeCell {
    links: BTreeMap<Id, bool>,
    row: i32,
    col: i32,
    configuration: Configuration,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Id {
    row: i32,
    col: i32,
}

#[derive(Debug, Default)]
pub struct Configuration {
    pub north: Option<Id>,
    pub south: Option<Id>,
    pub east: Option<Id>,
    pub west: Option<Id>,
}

impl MazeCell {
    pub fn new(row: i32, col: i32) -> Self {
        Self {
            row,
            col,
            links: BTreeMap::new(),
            configuration: Default::default(),
        }
    }

    pub fn configure(&mut self, configuration: Configuration) {
        self.configuration = configuration;
    }

    pub fn id(&self) -> Id {
        Id {
            row: self.row,
            col: self.col,
        }
    }

    pub fn neighbors(&self) -> Vec<Id> {
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

    pub fn north(&self) -> Option<Id> {
        self.configuration.north
    }

    pub fn south(&self) -> Option<Id> {
        self.configuration.south
    }

    pub fn east(&self) -> Option<Id> {
        self.configuration.east
    }
    pub fn west(&self) -> Option<Id> {
        self.configuration.west
    }

    pub fn is_linked(&self, other: &Id) -> bool {
        *self.links.get(other).unwrap_or(&false)
    }

    pub fn link(&mut self, other: &mut MazeCell) {
        self.links.insert(other.id(), true);
        other.links.insert(self.id(), true);
    }

    pub fn links(&self) -> Vec<Id> {
        self.links.keys().copied().collect()
    }

    #[allow(clippy::match_like_matches_macro)] // In this case I find the match more readable
    pub fn has_south_wall(&self) -> bool {
        match self.south() {
            Some(south) if self.is_linked(&south) => false,
            _ => true,
        }
    }

    #[allow(clippy::match_like_matches_macro)] // In this case I find the match more readable
    pub fn has_east_wall(&self) -> bool {
        match self.east() {
            Some(east) if self.is_linked(&east) => false,
            _ => true,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cells_can_be_linked() {
        let mut cell1 = MazeCell::new(1, 1);
        let mut cell2 = MazeCell::new(1, 2);
        assert!(!cell1.is_linked(&cell2.id()));
        assert!(!cell2.is_linked(&cell1.id()));
        cell1.link(&mut cell2);
        assert!(cell1.is_linked(&cell2.id()));
        assert!(cell2.is_linked(&cell1.id()));
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

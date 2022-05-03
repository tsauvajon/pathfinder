use crate::node::{Node, State};

#[derive(Clone)]
pub struct Matrix {
    inner: Vec<Vec<Node>>,

    active_cells: Vec<(usize, usize)>,
}

impl Matrix {
    pub fn new(height: usize, width: usize) -> Self {
        let mut matrix = vec![];
        for i in 0..height {
            let mut row: Vec<Node> = vec![];
            for j in 0..width {
                row.push(Node::new(i, j));
            }
            matrix.push(row);
        }
        Self {
            inner: matrix,
            active_cells: vec![],
        }
    }
}

impl IntoIterator for Matrix {
    type Item = Vec<Node>;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

pub enum Solve {
    Continue,
    Stop,
}

impl Matrix {
    pub fn neighbours(&self, i: usize, j: usize) -> Vec<Node> {
        let mut nb: Vec<Node> = vec![];

        let possible_nb: Vec<(isize, isize)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

        for p in possible_nb {
            let nb_i = (i as isize + p.0) as usize;
            let nb_j = (j as isize + p.1) as usize;
            if nb_i < self.height() && nb_j < self.width() {
                nb.push(self.inner[nb_i][nb_j].clone());
            }
        }

        nb
    }

    pub fn width(&self) -> usize {
        self.inner[0].len()
    }

    pub fn height(&self) -> usize {
        self.inner.len()
    }

    pub fn step(&mut self) -> Solve {
        let mut active = vec![];

        for (x, y) in self.active_cells.clone() {
            self.inner[x][y].active = false;

            let neighbours = self.neighbours(x, y);
            for neighbour in neighbours {
                if neighbour.is_target() {
                    self.mark_path(x, y);
                    return Solve::Stop;
                } else if neighbour.is_empty() {
                    active.push((neighbour.x, neighbour.y));
                    self.inner[neighbour.x][neighbour.y].visit(x, y);
                }
            }
        }

        self.active_cells = active;

        return if self.active_cells.len() > 0 {
            Solve::Continue
        } else {
            Solve::Stop
        };
    }

    pub fn set_active(&mut self, x: usize, y: usize) {
        self.active_cells.push((x, y));
        self.inner[x][y].active = true;
    }

    pub fn set_inactive(&mut self, x: usize, y: usize) {
        self.active_cells = self
            .active_cells
            .iter()
            .filter(|(a, b)| a != &x || b != &y)
            .map(|&(a, b)| (a, b))
            .collect();
        self.inner[x][y].active = false;
    }

    pub fn clear_parent(&mut self, x: usize, y: usize) {
        self.inner[x][y].parent_coords = None;
    }

    pub fn set_state(&mut self, x: usize, y: usize, state: State) {
        self.inner[x][y].state = state;
    }

    pub fn state(&self, x: usize, y: usize) -> State {
        self.inner[x][y].state
    }

    fn mark_path(&mut self, x: usize, y: usize) {
        let mut x = x;
        let mut y = y;
        loop {
            match self.inner[x][y].mark_path() {
                Some((nx, ny)) => {
                    x = nx;
                    y = ny;
                }
                None => break,
            }
        }
    }
}

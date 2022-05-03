use crate::node::Node;

#[derive(Clone)]
pub struct Matrix {
    pub inner: Vec<Vec<Node>>,
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
        Self { inner: matrix }
    }
}

impl IntoIterator for Matrix {
    type Item = Vec<Node>;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

type More = bool;

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

    pub fn step(&mut self) -> More {
        let mut active_nb = 0;

        for (i, r) in self.clone().into_iter().enumerate() {
            for (j, n) in r.iter().enumerate() {
                if !n.active {
                    continue;
                };

                active_nb += 1;

                self.inner[n.x][n.y].active = false;

                let nbs = self.neighbours(n.x, n.y);
                for nb in nbs {
                    if nb.is_target() {
                        self.mark_path(n);
                        return false;
                    } else if nb.is_empty() {
                        self.inner[nb.x][nb.y].visit(i, j);
                    }
                }
            }
        }

        // stop if there's no solution
        return active_nb > 0;
    }

    fn mark_path(&mut self, n: &Node) {
        let mut x = n.x;
        let mut y = n.y;

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

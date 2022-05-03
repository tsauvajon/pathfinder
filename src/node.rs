#[derive(Copy, Clone, PartialEq)]
pub enum State {
    Empty,
    Start,
    Visited,
    Path,
    Target,
    Wall,
}

pub fn state_class(state: State) -> String {
    match state {
        State::Path => String::from("path pop"),
        State::Empty => String::from("empty"),
        State::Start => String::from("start pop"),
        State::Wall => String::from("wall pop"),
        State::Visited => String::from("visited"),
        State::Target => String::from("target pop"),
    }
}

#[derive(Clone)]
pub struct Node {
    pub x: usize,
    pub y: usize,
    pub active: bool,
    pub state: State,
    pub parent_coords: Option<(usize, usize)>,
}

impl Node {
    pub fn new(x: usize, y: usize) -> Node {
        Node {
            x,
            y,
            state: State::Empty,
            active: false,
            parent_coords: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self.state {
            State::Empty => true,
            _ => false,
        }
    }

    pub fn is_target(&self) -> bool {
        match self.state {
            State::Target => true,
            _ => false,
        }
    }

    pub fn is_visited(&self) -> bool {
        match self.state {
            State::Visited => true,
            _ => false,
        }
    }

    pub fn visit(&mut self, p_x: usize, p_y: usize) {
        match self.state {
            State::Empty => (),
            _ => return,
        }
        self.state = State::Visited;
        self.active = true;
        self.parent_coords = Some((p_x, p_y))
    }

    pub fn mark_path(&mut self) -> Option<(usize, usize)> {
        if self.is_visited() {
            self.state = State::Path;
            self.parent_coords
        } else {
            None
        }
    }
}

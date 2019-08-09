#[derive(Default,Debug)]
pub struct Tree {
    pub data: Option<char>,
    pub left: Option<Box<Tree>>,
    pub right: Option<Box<Tree>>,
}

impl Tree {
    pub fn blank() -> Tree {
        Tree { ..Default::default()}
    }

    pub fn new(data: char) -> Tree {
        Tree {
            data: Some(data),
            ..Default::default()
        }
    }

    pub fn set_left(mut self, child: Tree) -> Tree {
        self.left = Some(Box::new(child));
        self
    }

    pub fn set_right(mut self, child: Tree) -> Tree {
        self.right = Some(Box::new(child));
        self
    }
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl Eq for Tree {}

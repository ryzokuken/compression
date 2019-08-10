use std::cmp::Ordering;

#[derive(Default, Debug)]
pub struct Tree {
    pub data: Option<char>,
    pub left: Option<Box<Tree>>,
    pub right: Option<Box<Tree>>,
}

impl Tree {
    pub fn blank() -> Tree {
        Tree {
            ..Default::default()
        }
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

impl PartialOrd for Tree {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Tree {
    fn cmp(&self, _other: &Self) -> Ordering {
        Ordering::Equal
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::Tree;
        let tree = Tree::new('a')
            .set_left(Tree::new('b'))
            .set_right(Tree::new('c'));
        assert_eq!(tree.data, Some('a'));
        assert_eq!(tree.left.unwrap().data, Some('b'));
        assert_eq!(tree.right.unwrap().data, Some('c'));
    }
}

use std::cmp::max;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct BinTreeNode<T> {
    pub(crate) value: Option<T>,
    pub(crate) left: Option<Box<BinTreeNode<T>>>,
    pub(crate) right: Option<Box<BinTreeNode<T>>>,
}

impl<T> BinTreeNode<T> {
    pub(crate) fn new() -> BinTreeNode<T> {
        BinTreeNode {
            value: None,
            left: None,
            right: None,
        }
    }

    pub fn init_left(&mut self) {
        if self.left.is_none() {
            self.left = Some(Box::new(BinTreeNode::new()));
        }
    }

    pub fn init_right(&mut self) {
        if self.right.is_none() {
            self.right = Some(Box::new(BinTreeNode::new()));
        }
    }

    fn of(value: T) -> BinTreeNode<T> {
        BinTreeNode {
            value: Some(value),
            left: None,
            right: None,
        }
    }

    #[allow(dead_code)]
    pub fn from(
        value: Option<T>,
        left: Option<Box<BinTreeNode<T>>>,
        right: Option<Box<BinTreeNode<T>>>,
    ) -> BinTreeNode<T> {
        BinTreeNode { value, left, right }
    }

    pub(crate) fn max_depth(&self) -> i32 {
        let left_depth = match &self.left {
            Some(node) => node.max_depth(),
            None => 0,
        };
        let right_depth = match &self.right {
            Some(node) => node.max_depth(),
            None => 0,
        };
        max(left_depth, right_depth) + 1
    }

    pub(crate) fn insert(&mut self, value: T)
    where
        T: PartialOrd,
    {
        if self.value.is_none() {
            self.value = Some(value);
        } else {
            if value <= *self.value.as_ref().unwrap() {
                match &mut self.left {
                    Some(node) => node.insert(value),
                    None => {
                        let new_node = Box::new(BinTreeNode::of(value));
                        self.left = Some(new_node);
                    }
                }
            } else {
                match &mut self.right {
                    Some(node) => node.insert(value),
                    None => {
                        let new_node = Box::new(BinTreeNode::of(value));
                        self.right = Some(new_node);
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct BinTree<T> {
    pub(crate) root: BinTreeNode<T>,
}

#[allow(dead_code)]
impl<T> BinTree<T>
where
    T: Display + Clone,
{
    pub fn get_root(&self) -> &BinTreeNode<T> {
        &self.root
    }

    pub fn new() -> BinTree<T> {
        BinTree {
            root: BinTreeNode::<T>::new(),
        }
    }

    pub fn get_max_depth(&self) -> i32 {
        self.root.max_depth()
    }

    pub fn clear(&mut self) {
        self.root = BinTreeNode::<T>::new();
    }

    pub fn insert(&mut self, value: T)
    where
        T: PartialOrd,
    {
        self.root.insert(value);
    }

    pub fn insert_many(&mut self, values: &Vec<T>)
    where
        T: PartialOrd,
    {
        for value in values {
            self.insert((*value).clone());
        }
    }
}

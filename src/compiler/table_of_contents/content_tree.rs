use std::convert::Infallible;

use getset::{Getters, MutGetters, Setters};


#[derive(Debug, Clone)]
pub struct ContentTree<T> {
    nodes: Vec<ContentTreeNode<T>>
}

impl<T> ContentTree<T> {

    pub fn new_empty() -> Self {
        Self {
            nodes: Vec::new()
        }
    }

    pub fn nodes(&self) -> &Vec<ContentTreeNode<T>> {
        &self.nodes
    }
}


#[derive(Debug, Clone, Getters, MutGetters, Setters)]
pub struct ContentTreeNode<T> {

    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    content: T,

    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    sub_nodes: Vec<ContentTreeNode<T>>,
}

impl<T> ContentTreeNode<T> {
    pub fn new(content: T, sub_nodes: Vec<ContentTreeNode<T>>) -> Self {
        Self {
            content,
            sub_nodes
        }
    }

    pub fn new_leaf(content: T) -> Self {
        Self::new(content, vec![])
    }

    /// Return true is a leaf (`sub_contents.len() == 0`), else false
    pub fn is_leaf(&self) -> bool {
        self.sub_nodes.len() == 0
    }
}

impl<T: Clone> ContentTreeNode<T> {

    /// Walk tree using depth first approach.
    /// 
    /// Return cloned contents Vector
    pub fn walk_depth_first(&self) -> Vec<T> {
        let mut contents: Vec<T> = Vec::new();

        Self::walk_depth_first_rec(&self, &mut |node: &ContentTreeNode<T>, _current_lv: usize| -> Result<(), Infallible> {
            contents.push(node.content().clone());

            Ok(())
        }, 0).unwrap();

        contents
    }

    fn walk_depth_first_rec<O, E>(node: &ContentTreeNode<T>, f: &mut dyn FnMut(&ContentTreeNode<T>, usize) -> Result<O, E>, current_lv: usize) -> Result<(), E> {

        f(node, current_lv)?;

        if node.is_leaf() {
            return Ok(());
        }

        for sub_node in node.sub_nodes() {

            Self::walk_depth_first_rec(sub_node, f, current_lv + 1)?;
        }

        Ok(())
    }

    pub fn walk_depth_first_applying<O, E>(&self, f: &mut dyn FnMut(&ContentTreeNode<T>, usize) -> Result<O, E>) -> Result<(), E> {
        Self::walk_depth_first_rec(&self, f, 0)?;

        Ok(())
    }
}


#[cfg(test)]
mod test {
    use super::ContentTreeNode;


    fn get_content_tree_to_test() -> ContentTreeNode<u32> {
        let tree = ContentTreeNode::new(
            1,
            vec![
                ContentTreeNode::new(
                    2,
                    vec![
                        ContentTreeNode::new_leaf(3),
                        ContentTreeNode::new_leaf(4),
                        ContentTreeNode::new_leaf(5),
                    ]
                ),
                ContentTreeNode::new_leaf(6),
                ContentTreeNode::new_leaf(7),
            ]
        );

        tree
    }

    #[test]
    fn walk_depth_first() {

        let tree = get_content_tree_to_test();
        let output: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7];

        assert_eq!(tree.walk_depth_first(), output);
    }
}
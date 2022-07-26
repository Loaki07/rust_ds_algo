use std::fmt::Debug;

type Tree<T> = Box<Node<T>>;

#[derive(Debug)]
pub struct Node<T> {
    data: T,
    parent: Option<Tree<T>>,
    children: Vec<Tree<T>>,
    is_locked: Option<u32>,
}

impl<T: Clone + Clone> Node<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            parent: None,
            children: Vec::<Tree<T>>::new(),
            is_locked: None,
        }
    }
    
    fn update_parent(&mut self, new_parent: Option<Tree<T>>) {
        self.parent = new_parent;
    }
    
    fn update_children(&mut self, new_child: Tree<T>) {
        self.children.push(new_child);
    }
    
    fn check_children_for_lock(node: &Tree<T>) -> bool {
        for child in node.children.iter() {
            if child.is_locked.is_some() {
                return true;
            }
        }
        false
    }
}

#[derive(Debug)]
pub struct MTree<T> {
    root: Option<Tree<T>>,
}

impl<T: Clone + std::cmp::PartialEq> MTree<T> {
    pub fn new() -> Self {
        Self {
            root: None,
        }
    }
    
    pub fn add_node(&mut self, data: T, ref_parent: T) {
        match self.root {
            Some(ref n) => {
                let parent_node = self.find_node(n, &ref_parent);
                let mut new_node = Node::new(data);
                new_node.update_parent(parent_node);
                let mut parent_node = self.find_node(n, &ref_parent);
                if let Some(ref mut p) = parent_node {
                    p.update_children(Box::new(new_node));
                }
            },
            None => {
                self.root = Some(Box::new(Node::new(data)));
            }
        }
    }
    
    pub fn find_node(&self, node: &Tree<T>, key: &T) -> Option<Tree<T>> {
        // to_be_fixed
        // if node.data == *key {
        //     return Some(node)
        // }
        let mut return_node = None;
        for child in node.children.iter() {
            return_node = self.find_node(child, key);
        }
        return return_node;
    }
    
    pub fn lock(&mut self, key: &T, uid: u32) -> bool {
        let mut found_node = self.find_node(self.root.as_ref().unwrap(), &key);
        
        if let Some(ref mut fnn) = found_node {
            let parent_node = &fnn.parent;
            
            if parent_node.as_ref().unwrap().is_locked.is_some() {
                return false;
            }
            
            if fnn.is_locked.is_some() {
                return false;
            }
            
            if fnn.is_locked.is_none() && parent_node.as_ref().unwrap().is_locked.is_none() {
                for child in fnn.children.iter() {
                    // need to check every child if its locked
                    if Node::check_children_for_lock(child) {
                        return false;
                    }
                    if child.is_locked.is_none() {
                        continue;
                    } else {
                        return false; 
                    }
                }
                fnn.is_locked = Some(uid);
                return true;
            }
        }
        false
    }
    
    pub fn unlock(&mut self, key: T, uid: u32) -> bool {
        let mut found_node = self.find_node(self.root.as_ref().unwrap(), &key);
        
        if let Some(ref mut fnn) = found_node {
            if let Some(locked_uid) = fnn.is_locked {
                if locked_uid == uid {
                    fnn.is_locked = None;
                    return true;
                }
            }
        }
        false
    }
    
    pub fn upgrade(&mut self, key: T, uid: u32) -> bool {
        let mut found_node = self.find_node(self.root.as_ref().unwrap(), &key);
        
        if let Some(ref mut fnn) = found_node {
            for child in fnn.children.iter() {
                if let Some(locked_uid) = child.is_locked {
                    if locked_uid == uid {
                        continue;
                    } else {
                        return false;
                    }
                }
            }
            fnn.is_locked = Some(uid);
            for child in fnn.children.iter_mut() {
                child.is_locked = None
            }
            return true;
        }
        false
    }
}


fn main() {
    

}

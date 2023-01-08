use std::borrow::Borrow;
use std::rc::{
    Rc,
    Weak
};
use std::cell::RefCell;
#[derive(Debug)]
pub enum FsNode {
    Dir(Rc<DirNode>),
    File(Rc<FileNode>)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NodeType {
    Dir,
    File
}

#[derive(Debug)]
pub struct DirNode {
    size: RefCell<usize>,
    name: String,
    children: RefCell<Vec<Rc<FsNode>>>,
    node_type: NodeType,
    parent: Option<Weak<DirNode>>
}

impl DirNode {
    pub fn new(name: &str, parent: Option<Rc<DirNode>>) -> Self {
        let parent_node = match parent {
            Some(dir_node)=> Some(Rc::downgrade(&dir_node)),
            None => None
        };
        DirNode { size: RefCell::new(0),
                  name: String::from(name),
                  children: RefCell::new(Vec::new()),
                  node_type: NodeType::Dir,
                  parent: parent_node}
    }

    pub fn get_size(&self) -> usize {
        self.size.borrow().clone()
    }
    
    pub fn update_size(&self) -> usize {
        let mut total_size = 0;
        for child in &(*(self.children.borrow()))[..] {
            match &(**child) {
                FsNode::Dir(dir_node) => {
                    total_size += dir_node.update_size();
                }
                FsNode::File(file_node) => { 
                    total_size += file_node.get_size();
                }
            }
        }
        *(self.size.borrow_mut()) = total_size;
        return total_size;
    }
    
    pub fn get_name(&self) -> &String {
        return &self.name
    }
    
    pub fn get_parent(&self) -> Option<Rc<DirNode>> {
        match &self.parent {
            Some(parent_node) => match parent_node.upgrade() {
                Some(parent_pointer) => Some(parent_pointer),
                None => panic!("Couldn't upgrade parent pointer")
            },
            None => None
        }
    }

    pub fn add_child(&self, child: Rc<FsNode>) {
        self.children.borrow_mut().push(child);
    }

    pub fn search_children(&self, pattern_str: &str) -> Option<Weak<DirNode>>{
        for child in &(*(self.children.borrow()))[..] {
            match &(**child) {
                FsNode::Dir(dir_node) => {
                    if dir_node.get_name() == pattern_str {
                        return Some(Rc::downgrade(dir_node));
                    }
                },
                FsNode::File(_) => continue
            }
        }
        None
    }

    pub fn filter_child_dirs(&self, filter: fn(usize, usize) -> bool, threshold: usize) -> Vec<Rc<DirNode>> {
        let mut dir_list: Vec<Rc<DirNode>> = Vec::new();
        for child in &(*(self.children.borrow()))[..] {
            match &(**child) {
                FsNode::Dir(dir_node) => {
                    let dir_size = *(dir_node.size.borrow());
                    if (&filter)(dir_size, threshold) {
                        dir_list.push(Rc::clone(dir_node));
                    }
                    let mut children = (*dir_node).filter_child_dirs(filter, threshold);
                    dir_list.append(&mut children);
                }
                FsNode::File(_) => {
                    // don't check anything in this case
                    continue;
                }
            }
        }
        return dir_list;
    }
}

#[derive(Debug)]
pub struct FileNode {
    size: usize,
    name: String,
    node_type: NodeType,
    parent: Weak<DirNode>
}

impl FileNode {
    pub fn new(name: &str, size: usize, parent: Weak<DirNode>) -> Self {
        FileNode { size: size, 
            name: String::from(name),
            parent: Weak::clone(&parent),
            node_type: NodeType::File}
    }
    
    pub fn get_size(&self) -> usize {
        self.size
    }
    
    pub fn get_type(&self) -> NodeType {
        self.node_type
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_parent(&self) -> Option<Weak<DirNode>> {
        Some(Weak::clone(&(self.parent)))
    }
}
    

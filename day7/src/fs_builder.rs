use std::rc::{
    Rc,
    Weak
};
use std::io::{
    BufRead, 
    BufReader
};
use std::fs::File;

use crate::fs_helpers::{
    FsNode, 
    DirNode,
    FileNode
};

enum FsBuilderState {
    FSMove,
    LSParse
}

struct FsBuilder {
    pub root_dir: Rc<DirNode>,
    pub curr_dir: Rc<DirNode>,
    pub state: FsBuilderState
}

pub fn build_fs (reader: &mut BufReader<File>) -> Rc<DirNode> {
    // construct the root node
    let root_node = Rc::new(DirNode::new("/", None));
    let mut builder = FsBuilder {
        root_dir: Rc::clone(&root_node),
        curr_dir: Rc::clone(&root_node),
        state: FsBuilderState::FSMove
    };

    let mut iter = reader.lines().peekable();

    loop {
        let line = iter.peek();
        match line {
            Some(line_res) => { 
                match line_res {
                    Ok(actual_line) => {
                        let tokens: Vec<&str> = actual_line.split_whitespace().collect();
                        match builder.state {
                            FsBuilderState::FSMove => {
                                match tokens[0] {
                                    "$" => {
                                        parse_nav_cmd(&mut builder, &tokens[1..]);
                                    }
                                    &_ => {
                                        panic!("We're in a command state, but didn't get a command")
                                    }
                                }
                                // consume the command
                                iter.next();
                            },
                            FsBuilderState::LSParse => {
                                // if we see the command token, don't advance the iterator, but
                                // kick back to the command state
                                match tokens[0] {
                                    "$" => {
                                        builder.state = FsBuilderState::FSMove;
                                        continue;
                                    }
                                    &_ => {
                                        parse_ls_line(&builder, &tokens[..]);
                                        // consume the line
                                        iter.next();
                                    }
                                }
                            }
                        };
                    },
                    Err(_) => {panic!("couldn't read the line")}
                }
            },
            None => break,
        };
    }


    return root_node
}

fn parse_nav_cmd(fs_builder: &mut FsBuilder, tokens: &[&str]) {
    let cmd_string = tokens[0];
    match cmd_string {
        "cd" => {
            let cd_arg = tokens[1];
            match cd_arg {
                // just set the curr dir to root
                "/" => fs_builder.curr_dir = Rc::clone(&fs_builder.root_dir), 
                // go up a directory
                ".." => {
                    match (*(fs_builder.curr_dir)).get_parent() {
                        Some(parent_node) => {
                            fs_builder.curr_dir = parent_node;
                        }
                        None => panic!("Tried to go up in the tree, but there is no parent")
                    }
                },
                &_ => {
                    match (*(fs_builder.curr_dir)).search_children(cd_arg) {
                        Some(child_node) => {
                            fs_builder.curr_dir = child_node.upgrade().expect("Couldn't upgrade pointer")
                        },
                        None => panic!("Couldn't find directory {cd_arg:}"),
                    }
                }
            }
        }
        "ls" => {
            fs_builder.state = FsBuilderState::LSParse;
        }
        &_ => panic!("Unexpected command"),
    }
}

fn parse_ls_line(fs_builder: &FsBuilder, tokens: &[&str]) {
    let info_token = tokens[0];
    let name_token = tokens[1];
    match info_token {
        // create a directory node child
        "dir" => {
            let new_dir_node = Rc::new(DirNode::new(name_token, Some(Rc::clone(&(fs_builder.curr_dir)))));
            let new_fs_node = Rc::new(FsNode::Dir(new_dir_node));

            (*(fs_builder.curr_dir)).add_child(new_fs_node);
        },
        &_ => {
            let size: usize = info_token.parse().unwrap();
            let new_file_node = Rc::new(FileNode::new(name_token, size, Rc::downgrade(&fs_builder.curr_dir)));
            let new_fs_node = Rc::new(FsNode::File(new_file_node));
            (*(fs_builder.curr_dir)).add_child(new_fs_node);
        }
    }
}
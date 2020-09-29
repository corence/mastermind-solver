
use crate::attempt::*;
use crate::code::*;
use crate::random_index::*;
use std::fmt;

#[derive(Debug)]
enum NodeChildren {
    Expanded(Vec<Node>),
    NoChildren,
    Unexpanded,
}
use NodeChildren::*;

pub struct Node {
    code: Code,
    code_count: usize,
    children: NodeChildren,
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
         .field("code", &self.code)
         .field("children", &self.children)
         .finish()
    }
}

pub enum SelectionResult {
    Selection(Code),
    DeadBranch,
    Surrender(usize),
}
pub use SelectionResult::*;

impl Node {
    pub fn new(code: Code) -> Self {
        Node {
            code,
            children: Unexpanded,
            code_count: 1,
        }
    }

    pub fn code_count(&self) -> usize {
        self.code_count
    }

    pub fn select_code(&mut self, previous_guesses: &Vec<Attempt>, available_colors: &Vec<Color>, surrender_depth_on_dead_branch: usize) -> SelectionResult {
        // given a node,
        //   a) does it contradict any guesses? if so:
        //     delete it from its parent
        //     set the parent as the current node
        //     goto e (or if easier, just goto a)
        //   b) has it been expanded? if not, expand it now
        //   c) does it have children? 
        //     d) no: return its code
        //     e) yes: select a random child and make that the new node

        if previous_guesses.iter().any(|guess| !guess.matches(&self.code)) {
            return DeadBranch;
        }

        self.expand(available_colors);

        match &mut self.children {
            NoChildren => Selection(self.code.clone()),
            Unexpanded => panic!("ok but what"),
            Expanded(children) => {
                while children.len() > 0 {
                    let index = select_random_index(&children).unwrap();
                    self.code_count -= children[index].code_count();
                    let result = children[index].select_code(previous_guesses, available_colors, surrender_depth_on_dead_branch);

                    match result {
                        Selection(_) => {
                            self.code_count += children[index].code_count();
                            return result;
                        },
                        DeadBranch => {
                            children.remove(index);
                            if surrender_depth_on_dead_branch > 0 {
                                return Surrender(surrender_depth_on_dead_branch - 1);
                            }
                        },
                        Surrender(0) => {
                            self.code_count += children[index].code_count();
                        }
                        Surrender(depth) => {
                            self.code_count += children[index].code_count();
                            return Surrender(depth - 1);
                        }
                    }
                }
                DeadBranch
            },
        }
    }

    fn expand(&mut self, available_colors: &Vec<Color>) {
        if let Unexpanded = self.children {
            self.children = match select_random_matching_index(&self.code.code, '.') {
                None => NoChildren,
                Some(index) => {
                    let mut children = Vec::new();
                    for color in available_colors {
                        let mut new_code = self.code.clone();
                        new_code.code[index] = *color;
                        children.push(Node::new(new_code));
                        self.code_count += 1;
                    }
                    Expanded(children)
                },
            };
        }
    }
}

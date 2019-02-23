pub struct Tree<T> {
    value: Option<T>,
    left_child: NodeOption<T>,
    right_child: NodeOption<T>,
}

enum NodeOption<T> {
    Some(Box<Tree<T>>),
    None,
}

impl<T: Ord> Tree<T> {
    /// Creates an empty tree
    pub fn new() -> Self {
        Tree { value: None, left_child: NodeOption::None, right_child: NodeOption::None }
    }

    pub fn new_with_value(key: T) -> Self {
        Tree { value: Some(key), left_child: NodeOption::None, right_child: NodeOption::None }
    }

    /// Returns `false` if `key` already exists in the tree, and `true` otherwise.
    pub fn insert(&mut self, key: T) -> bool {
        match self.value {
            Option::None => {
                 self.value = Some(key);
            },
            Option::Some(ref curr_value) => {
                if key == *curr_value {
                    return false
                } else if key < *curr_value {
                    match self.left_child {
                        NodeOption::Some(ref mut left_node) => {
                            let check = left_node.insert(key);
                            return check
                        }
                        NodeOption::None => {
                            let new_node = Box::new(Tree::new_with_value(key));
                            self.left_child = NodeOption::Some(new_node);
                        },
                    }
                } else if key > *curr_value {
                    match self.right_child {
                        NodeOption::Some(ref mut right_node) => {
                            let check = right_node.insert(key);
                            return check
                        }
                        NodeOption::None => {
                            let new_node = Box::new(Tree::new_with_value(key));
                            self.right_child = NodeOption::Some(new_node);
                        },
                    }
                }
            },
        }
        return true
    }

    /// Returns `true` if `key` exists in the tree, and `false` otherwise.
    pub fn find(&self, key: &T) -> bool {
        match self.value {
            Option::None => {
              return false
            },
            Option::Some(ref curr_value) => {
              if *key == *curr_value {
                  return true
              } else if *key < *curr_value {
                  match self.left_child {
                      NodeOption::Some(ref left_node) => {
                          let check = left_node.find(key);
                          return check
                      }
                      NodeOption::None => {
                          return false
                      },
                  }
              } else {
                  match self.right_child {
                      NodeOption::Some(ref right_node) => {
                          let check = right_node.find(key);
                          return check
                      }
                      NodeOption::None => {
                          return false
                      },
                  }
              }
            },
        }
    }

    /// Returns the preorder traversal of the tree.
    pub fn preorder(&self) -> Vec<&T> {
        match self.value {
            Option::None => {
                let return_vec: Vec<&T> = vec![];
                return return_vec
            },
            Option::Some(ref curr_value) => {
                let mut return_vec: Vec<&T> = vec![];
                return_vec.push(curr_value);

                match self.left_child {
                    NodeOption::Some(ref left_node) => {
                        let left_child_vec = left_node.preorder();
                        for each in left_child_vec {
                            return_vec.push(each);
                        }
                    },
                    NodeOption::None => {},
                }

                match self.right_child {
                    NodeOption::Some(ref right_node) => {
                        let right_child_vec = right_node.preorder();
                        for each in right_child_vec {
                            return_vec.push(each);
                        }
                    },
                    NodeOption::None => {},
                }
                return return_vec
            },
        }
    }

    /// Returns the inorder traversal of the tree.
    pub fn inorder(&self) -> Vec<&T> {
        match self.value {
            Option::None => {
                let return_vec: Vec<&T> = vec![];
                return return_vec
            },
            Option::Some(ref curr_value) => {
                let mut return_vec: Vec<&T> = vec![];

                match self.left_child {
                    NodeOption::Some(ref left_node) => {
                        let left_child_vec = left_node.inorder();
                        for each in left_child_vec {
                            return_vec.push(each);
                        }
                    },
                    NodeOption::None => {},
                }

                return_vec.push(curr_value);

                match self.right_child {
                    NodeOption::Some(ref right_node) => {
                        let right_child_vec = right_node.inorder();
                        for each in right_child_vec {
                            return_vec.push(each);
                        }
                    },
                    NodeOption::None => {},
                }
                return return_vec
            },
        }
    }

    /// Returns the postorder traversal of the tree.
    pub fn postorder(&self) -> Vec<&T> {
        match self.value {
            Option::None => {
                let return_vec: Vec<&T> = vec![];
                return return_vec
            },
            Option::Some(ref curr_value) => {
                let mut return_vec: Vec<&T> = vec![];

                match self.left_child {
                    NodeOption::Some(ref left_node) => {
                        let left_child_vec = left_node.postorder();
                        for each in left_child_vec {
                            return_vec.push(each);
                        }
                    },
                    NodeOption::None => {},
                }

                match self.right_child {
                    NodeOption::Some(ref right_node) => {
                        let right_child_vec = right_node.postorder();
                        for each in right_child_vec {
                            return_vec.push(each);
                        }
                    },
                    NodeOption::None => {},
                }

                return_vec.push(curr_value);

                return return_vec
            },
        }
    }
}
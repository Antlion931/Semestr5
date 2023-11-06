use std::{ mem::swap, ptr};

#[derive(Debug, Clone)]
pub enum Node{
    NYT,
    INTERMEDIATE {
        counter: u64,
        index: u32,
        left: Box<Node>,
        right: Box<Node>
    },
    VALUE {
        counter: u64,
        index: u32,
        value: u8,
    },
}

impl Node {
    fn new_with_value(value: u8, index: u32) -> Self {
        Self::VALUE { value, counter: 1, index }
    }

    fn new_intermediate(left: Node, right: Node, index: u32) -> Self {
        let mut counter = 0;

        match left {
            Node::INTERMEDIATE { counter: c, ..} => counter += c,
            Node::VALUE { counter: c, ..} => counter += c,
            Node::NYT => {}
        }

        match right {
            Node::INTERMEDIATE { counter: c, ..} => counter += c,
            Node::VALUE { counter: c, ..} => counter += c,
            Node::NYT => {}
        }

        Self::INTERMEDIATE { left: Box::new(left), right: Box::new(right), index, counter }
    }

    fn add_one(&mut self) {
        match self {
            Node::NYT => (),
            Node::VALUE { counter, ..} => *counter += 1,
            Node::INTERMEDIATE { counter, .. } => *counter +=1,
        }
    }
    
    fn get_counter(&self) -> Option<u64> {
        match self {
            Node::VALUE { counter, ..} => Some(*counter),
            Node::INTERMEDIATE { counter, .. } => Some(*counter),
            Node::NYT => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Code {
    body: u32,
    length: u8,
}

impl Code {
    fn new() -> Self {
        Self { body: 0, length: 0}
    }

    fn left(&self) -> Self {
        Self { body: self.body << 1, length: self.length + 1 }
    }

    fn right(&self) -> Self {
        Self { body: (self.body << 1) + 1, length: self.length + 1 }
    }

    fn back(&self) -> Self {
        Self { body: self.body >> 1, length: self.length - 1 }
    }
}

pub struct HuffmanTree {
    root: Node,
    nyt_counter: u32,
}

impl HuffmanTree {
    pub fn new() -> Self {
        HuffmanTree { root: Node::NYT, nyt_counter: u32::MAX }
    }

    pub fn code_of(&self, byte: u8) -> Option<Code> {
        HuffmanTree::code_of_recursion(&self.root, byte, Code::new())
    }

    fn code_of_recursion(actual: &Node, byte: u8, code: Code) -> Option<Code> {
        match actual {
            Node::VALUE { value: x, .. } if *x == byte => Some(code),
            Node::INTERMEDIATE { ref left, ref right , ..} => HuffmanTree::code_of_recursion(left, byte, code.left()).or_else(|| HuffmanTree::code_of_recursion(right, byte, code.right())),
            _ => None,
        }
    }

    pub fn nyt_code(&self) -> Code {
        HuffmanTree::nyt_code_recursion(&self.root, Code::new()).expect("There is always NYT")
    }

    fn nyt_code_recursion(actual: &Node, code: Code) -> Option<Code> {
        match actual {
            Node::NYT => Some(code),
            Node::INTERMEDIATE { ref left, ref right, .. } => HuffmanTree::nyt_code_recursion(left, code.left()).or_else(|| HuffmanTree::nyt_code_recursion(right, code.right())),
            _ => None,
        }
    }

    pub fn decode(&self, code: Code) -> Option<&Node> {
        let mut actual = &self.root;

        for i in (0..code.length).rev() {
            match (actual, code.body & 1<<i) {
                (Node::INTERMEDIATE { ref left, ..}, 0) => actual = left,
                (Node::INTERMEDIATE { ref right, .. }, _) => actual = right,
                _ => return None,
            }
        }

        Some(&actual)
    }

    fn decode_mut(&mut self, code: Code) -> Option<&mut Node> {
        let mut actual = &mut self.root;

        for i in (0..code.length).rev() {
            match (actual, code.body & 1<<i) {
                (Node::INTERMEDIATE { left, ..}, 0) => actual = left,
                (Node::INTERMEDIATE { right, .. }, _) => actual = right,
                _ => return None,
            }
        }

        Some(actual)
    }

    fn code_of_with_index(&self, index: u32) -> Option<Code> {
        HuffmanTree::code_of_with_index_recursion(&self.root, index, Code::new())
    }

    fn code_of_with_index_recursion(actual: &Node, index: u32, code: Code) -> Option<Code> {
        match actual {
            Node::VALUE { index: x, .. } if *x == index => Some(code),
            Node::INTERMEDIATE { ref left, ref right , ..} => HuffmanTree::code_of_with_index_recursion(left, index, code.left()).or_else(|| HuffmanTree::code_of_with_index_recursion(right, index, code.right())),
            _ => None,
        }
    }

    fn biggest_index_in_block(&self, count: u64) -> Option<u32> {
        HuffmanTree::biggest_index_in_block_recursion(&self.root, count)
    }

    fn biggest_index_in_block_recursion(actual: &Node, count: u64) -> Option<u32> {
        match actual {
            Node::VALUE { counter, index, ..} if *counter == count => Some(*index),
            Node::INTERMEDIATE { counter, index, .. } if *counter == count => Some(*index),
            Node::INTERMEDIATE { left, right, .. } => [left, right].iter().filter_map(|x| HuffmanTree::biggest_index_in_block_recursion(x, count)).max(),
            _ => None,
        }
    }

    pub fn update(&mut self, byte: u8) {
        if let Some(code) = self.code_of(byte) {
            self.repair_and_add_one(code);
        } else {
            let nyt_counter = self.nyt_counter;
            let nyt_code = self.nyt_code();
            let node = self.decode_mut(nyt_code).expect("There is always NYT");
            *node = Node::new_intermediate(Node::NYT, Node::new_with_value(byte, nyt_counter - 1), nyt_counter - 2);

            self.nyt_counter -= 2;
            self.repair_and_add_one(nyt_code.right())
        }
    }

    fn repair_and_add_one(&mut self, code: Code) {
        let mut buffer = code;
        while buffer.length > 0 {
            self.decode_mut(buffer).unwrap().add_one();
            buffer = buffer.back();
        }

        let mut buffer = code;
        while buffer.length > 0 {
            if let Some(Node::INTERMEDIATE { left, right, .. }) = self.decode_mut(code.back()) {
                if let (Some(l), Some(r)) = (left.get_counter(), right.get_counter()) {
                    if l > r {
                        swap(left, right)
                    }
                }
                
            } else {
                panic!("HELP");
            }

            let node = self.decode(code).unwrap();
            match node {
                Node::VALUE { counter, index, ..} => {
                    let biggest = self.biggest_index_in_block(*counter).unwrap(); 
                    if *index == biggest {
                        buffer = buffer.back();
                    } else {
                        
                    }
                }
                Node::INTERMEDIATE { counter, index, ..} => {
                    let biggest = self.biggest_index_in_block(*counter).unwrap(); 
                    if *index == biggest {
                        buffer = buffer.back();
                    } else {
                        //swap
                    }

                }
                _ => {}
            }


        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_is_computed_correctly() {
        let code = Code::new();
        let code = code.left().right().left().left().right();

        assert_eq!(code.length, 5);
        assert_eq!(code.body, 0b01001);
    }

    #[test]
    fn first_nyt_have_code_zero() {
        let tree = HuffmanTree::new();
        assert_eq!(tree.nyt_code(), Code::new())
    }

    #[test]
    fn simple_update() {
        let mut tree = HuffmanTree::new();
        tree.update(b'a');
        assert_eq!(tree.nyt_code(), Code::new().left());
        assert_eq!(tree.code_of(b'a').unwrap(), Code::new().right());
    }
}

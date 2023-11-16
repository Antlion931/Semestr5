const AMOUNT_OF_DIFFERENT_BYTES: usize = 256;
type ByteType = u8;
const MAX_SIZE_OF_DICTIONARY: usize = u16::MAX as usize + 1;
type DictionaryType = u16;

pub fn decode(message: impl AsRef<[DictionaryType]>) -> Vec<ByteType> {
    let mut dictionary = Vec::with_capacity(MAX_SIZE_OF_DICTIONARY);

    for x in 0..AMOUNT_OF_DIFFERENT_BYTES {
        dictionary.push(vec![x as ByteType]);
        assert!(dictionary.last().unwrap().capacity() == 1);
    }

    let mut result = Vec::new();
    let mut is_first = true;

    for i in message.as_ref() {
        let index = *i as usize;

        // update last value in dictionary by byte that blocked
        if !is_first {
            let byte_that_blocked = dictionary[index][0];
            dictionary
                .last_mut()
                .expect("It is not first")
                .push(byte_that_blocked);
        }

        is_first = false;

        // add bytes from word to result, and to new word with out byte that blocked
        for byte in &dictionary[index] {
            result.push(*byte);
        }

        // if capacity will be reached, reset dictionary, otherwise add new word
        if dictionary.len() + 1 >= MAX_SIZE_OF_DICTIONARY {
            dictionary.clear();

            for x in 0..AMOUNT_OF_DIFFERENT_BYTES {
                dictionary.push(vec![x as ByteType]);
                assert!(dictionary.last().unwrap().capacity() == 1);
            }

            is_first = true;
        } else {
            let mut new_word = Vec::with_capacity(dictionary[index].capacity() + 1);

            for byte in &dictionary[index] {
                new_word.push(*byte);
            }

            dictionary.push(new_word);
        }
    }

    result
}

#[derive(Debug, Clone)]
struct TreeNode {
    value: DictionaryType,
    next_nodes: [Option<Box<TreeNode>>; AMOUNT_OF_DIFFERENT_BYTES],
}

impl TreeNode {
    fn new(value: DictionaryType) -> Self {
        Self {
            value,
            next_nodes: vec![None; AMOUNT_OF_DIFFERENT_BYTES]
                .try_into()
                .expect("Size matches"),
        }
    }

    fn update(
        &mut self,
        size: usize,
        message: &[ByteType],
        new_index: DictionaryType,
    ) -> (DictionaryType, usize) {
        if size >= message.len() {
            (self.value, size)
        } else if let Some(next_node) = self.next_nodes[message[size] as usize].as_mut() {
            next_node.update(size + 1, message, new_index)
        } else {
            self.next_nodes[message[size] as usize] = Some(Box::new(TreeNode::new(new_index)));

            (self.value, size)
        }
    }
}

#[derive(Debug)]
struct TreeRoot {
    size: usize,
    next_nodes: [Option<Box<TreeNode>>; AMOUNT_OF_DIFFERENT_BYTES],
}

impl TreeRoot {
    fn new() -> Self {
        let mut next_nodes = Vec::with_capacity(AMOUNT_OF_DIFFERENT_BYTES);

        for byte in 0..AMOUNT_OF_DIFFERENT_BYTES {
            next_nodes.push(Some(Box::new(TreeNode::new(byte as DictionaryType))));
        }

        Self {
            size: AMOUNT_OF_DIFFERENT_BYTES,
            next_nodes: next_nodes.try_into().expect("Size matches"),
        }
    }

    // If message not empty, reads message, until it doesn't have tree node that represent given readed fragment, add
    // readed fragment plus byte that blocked as tree node or resets if capacity reached, returns value of readed fragment and
    // size of readed fragment
    fn update(&mut self, message: &[ByteType]) -> Option<(DictionaryType, usize)> {
        // Doesn't work for empty
        if message.is_empty() {
            return None;
        }

        let result = self.next_nodes[message[0] as usize]
            .as_mut()
            .expect("Every byte is covered")
            .update(1, message, self.size as u16);

        if self.size + 1 >= MAX_SIZE_OF_DICTIONARY {
            *self = TreeRoot::new();
        } else {
            self.size += 1;
        }

        Some(result)
    }
}

pub fn encode(message: impl AsRef<[u8]>) -> Vec<u16> {
    let message = message.as_ref();
    let mut dictionary = TreeRoot::new();

    let mut size = 0;
    let mut result = Vec::new();

    while size < message.len() {
        let (code, readed_size) = dictionary
            .update(&message[size..])
            .expect("It will not be empty");
        result.push(code);
        size += readed_size;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_decode_test() {
        assert_eq!(decode(vec![97, 98, 256, 258]), b"abababa");
    }

    #[test]
    fn simple_encode_test() {
        assert_eq!(encode(b"abababa"), vec![97, 98, 256, 258]);
    }
}

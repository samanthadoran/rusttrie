use std::str::Chars;

#[derive(Default)]
struct TrieNode{
    count: i32,
    children: [Option<Box<TrieNode>>; 26],
}

impl TrieNode {
    //Internal recursive function to determine if a trie contains a given word
    fn insert_internal(&mut self, mut word: Chars) {
        match word.next() {
            //If we have reached the end of the word and have always hit initialized Nodes, increment count
            None => self.count += 1,
            //If there is still more left to the word
            Some(c) => {
                //Find the index
                let index = (c as usize) - ('a' as usize);
                //And determine whether or not that child is initialized yet
                match self.children[index]{
                    //If not, init and rematch
                    None => {
                        self.children[index] = Some(Box::new(Default::default()));
                        //Gross double match to work around E0508?
                        match self.children[index]{
                            Some(ref mut c) => TrieNode::insert_internal(&mut *c, word),
                            None => {},
                        }
                    },
                    //Otherwise, continue recursively
                    Some(ref mut c) => TrieNode::insert_internal(&mut *c, word),
                }
            }
        }
    }

    fn insert(&mut self, word: &String) {
        //Transform word to a character iterator
        let word = word.to_lowercase();
        let mut word = word.chars();

        //Match against the word
        match word.next() {
            //We managed to insert nothing, good job.
            None => return,
            //We have a character to work with
            Some(c) => {
                //Find the index and match against it
                let index = (c as usize) - ('a' as usize);
                match self.children[index]{
                    //Init the node.
                    None => {
                        self.children[index] = Some(Box::new(Default::default()));
                        //Gross double match to wrk around E0508?
                        match self.children[index]{
                            Some(ref mut c) => TrieNode::insert_internal(&mut *c, word),
                            None => {},
                        }
                    },
                    //Node is already initialized, continue recursively
                    Some(ref mut c) => TrieNode::insert_internal(&mut *c, word),
                }
            }
        }
    }

    fn contains_internal(&self, mut word: Chars) -> bool {
        match word.next() {
            //If there are still characters in the string...
            Some(c) => {
                //Find the index and match against that node
                let index = (c as usize) - ('a' as usize);
                match self.children[index] {
                    //If it exists, continue looking
                    Some(ref c) => c.contains_internal(word),
                    //If it doesn't, we don't have the word
                    None => false,
                }
            },
            //If we have no characters and this node's count is non-zero, we have it.
            None => self.count != 0,
        }
    }

    fn contains(&self, word: &String) -> bool{
        let word = word.to_lowercase();
        self.contains_internal(word.chars())
    }
}

fn main() {
    //Examples
    let mut root: TrieNode = Default::default();
    let other_item = "not stuff".to_string();
    root.insert(&"stuff".to_string());
    root.insert(&"Samantha".to_string());
    println!("Contains {}: {}", other_item, root.contains(&other_item));
    println!("Contains {}: {}", "Samantha", root.contains(&"Samantha".to_string()));
}


use std::io::prelude::*;
use std::fs::File;
use std::fmt;

fn index<'a>(string: &'a mut String) -> Vec<Item<'a>> {
    let splitter = string.split(' ');

    let mut res = Vec::new();

    for s in splitter {
        if s.len() < 3 {continue};
        res.push(Item {key: s, value: 1});
    }

    res
}
/*
fn from_file<'a>(mut file: File) -> Vec<Item<'a>> {
    let mut buffer: String = String::new();
    file.read_to_string(&mut buffer);
    index(&mut buffer)
}
*/

struct Item<'a> {
    key: &'a str,
    value: usize,
}

impl<'a> PartialEq for Item<'a> {
    fn eq(&self, other: &Item) -> bool {
        self.key == other.key
    }
}

impl<'a> fmt::Debug for Item<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Item ({}: {} occurences)", self.key, self.value)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn simple_index() {
        let mut text = String::from("Hello, world!");
        let index1 = index(&mut text);
        let index2 = vec!(Item{key: "Hello,", value: 1}, Item{key: "world!", value: 1});
        assert_eq!(index1, index2);
    }
}

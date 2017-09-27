
use std::io::prelude::*;
use std::fs::File;
use std::fmt;

fn index<'a>(string: &'a mut String) -> Vec<Item<'a>> {
    let splitter = string.split(' ');

    let mut res: Vec<Item> = Vec::new();

    for s in splitter {
        if s.len() < 3 {continue};

        let mut temp_item = Item {key: s, value: get_value(s)};
        let mut exists: bool = false;
        for item in &mut res {
            if item == &temp_item {
                item.increase_value();
                exists = true;
                break;
            }
        }

        if !exists {
            res.push(temp_item);
        }


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

fn get_value(string: &str) -> usize {

    // The initial value is the length of the str.
    let mut value: usize = string.len();

    // Constants increase the string value.
    for c in string.chars() {
        match c {
            'a' => value +=1,
            'e' => value +=1,
            'i' => value +=1,
            'o' => value +=1,
            'u' => value +=1,
            _ => continue
        };
    }

    value
}

struct Item<'a> {
    key: &'a str,
    value: usize,
}

impl<'a> Item<'a> {
    fn increase_value(&mut self) {
        self.value += self.key.len();
    }

    fn get_value(&self) -> usize {
        self.value
    }
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

    #[test]
    fn simple_get_value() {
        assert_eq!(get_value("testing"), 9);
    }

    #[test]
    fn simple_item_increase() {
        let mut string: String = "testing if this works, by testing".to_string();
        let mut vec = index(&mut string);
        let item = Item {key: "testing", value: 16};
        assert_eq!(vec.get(0).unwrap().get_value(), item.get_value());
    }

}

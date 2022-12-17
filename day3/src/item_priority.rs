use std::f32::consts::E;
use std::vec::Vec;
use std::collections::HashMap;
use log::debug;

pub fn get_char_priority(c: char) -> u32 {
    let num = c as u32;

    // if it's lowercase
    if c.is_ascii_lowercase() {
        num - ('a' as u32) + 1
    }
    else if c.is_ascii_uppercase() {
        num - ('A' as u32) + 27
    }
    else {
        panic!("We've gotten a non-letter character")
    }
}

pub struct Rucksack {
    cptmts: Vec<Compartment>
}

pub struct Compartment {
    items: String,
    items_map: HashMap<char, u32>
}
impl Compartment {
    pub fn new(items: &str) -> Self {
        let mut new_struct = Compartment {items: items.to_string(), 
                                                   items_map: HashMap::new()};
        for c in items.chars() {
            if new_struct.items_map.contains_key(&c) {
                let last_total = new_struct.items_map.get_mut(&c).unwrap();
                *last_total = *last_total + 1;
            }
            else {
                new_struct.items_map.insert(c, 1);
            }
        }                                           
        return new_struct;
    }
}

impl Rucksack {
    pub fn new() -> Self {
        let new_struct = Self {cptmts: Vec::new()};

        new_struct
    }

    pub fn add_compartment(&mut self, contents: &str) {
        self.cptmts.push(Compartment::new(contents));
    }

    pub fn calc_contents_prio(&self) -> u32 {
        let cmpt0 = self.cptmts.get(0).unwrap();



        let mut total_prio = 0;
        for (c, tot) in cmpt0.items_map.iter() {
            // for each character, check if it exists in the other compartments
            for i in 1..self.cptmts.len() {
                let other = self.cptmts.get(i).unwrap();
                if other.items_map.contains_key(c) {
                    if i == (self.cptmts.len() - 1) {
                        println!("common char {c}");
                        let prio = get_char_priority(*c);
                        println!("priority: {prio}");
                        total_prio += prio;
                    }
                }
                else {
                    break;
                }
            }
        }
        return total_prio;
    }
}
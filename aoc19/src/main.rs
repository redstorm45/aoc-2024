use core::hash;
use std::fs;
use std::env;
use std::collections::HashMap;
use std::path;
use std::sync::atomic::AtomicUsize;
//use regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
        .expect("Should have been able to read the file");
    let mut contents_it = contents.split("\n\n");

    let available = parse_available(contents_it.next().unwrap());
    let validator = Validator::from(&available);
    let possible = contents_it.next().unwrap().split('\n')
        .filter(|s| validator.is_match(s))
        .count();

    println!("Result: {}", possible);
}

static COMPLETION_DICT_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug)]
struct CompletionDict<T>
where T: hash::Hash
{
    is_complete: bool,
    id: usize,
    possibilities: HashMap<T,Box<CompletionDict<T>>>,
}

impl<T> CompletionDict<T>
where T: hash::Hash,
      T: std::cmp::Eq
{
    fn new() -> CompletionDict<T> {
        CompletionDict{
            possibilities: HashMap::new(),
            is_complete: false,
            id: COMPLETION_DICT_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        }
    }
    fn insert(&mut self, it: &mut dyn Iterator<Item=T>) {
        if let Some(elem) = it.next() {
            self.possibilities
                .entry(elem)
                .or_insert(Box::new(CompletionDict::new()))
                .insert(it);
        } else {
            self.is_complete = true;
        }
    }
    fn get(&self, item: &T) -> Option<&CompletionDict<T>> {
        self.possibilities.get(item).map(|boxed| boxed.as_ref())
    }
}

impl<T> PartialEq for CompletionDict<T>
where T: hash::Hash
{
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

struct Validator {
    completions: CompletionDict<char>,
}

impl From<&[&str]> for Validator {
    fn from(value: &[&str]) -> Self {
        let mut res = Validator{completions: CompletionDict::new()};
        for &item in value {
            res.completions.insert(&mut item.chars());
        }
        res
    }
}

impl From<&Vec<&str>> for Validator {
    fn from(value: &Vec<&str>) -> Self {
        Self::from(&value[..])
    }
}

impl Validator {
    fn is_match(&self, s: &str) -> bool {
        let arr= self.count_arrangements(s);
        println!("Test match {} gives {}", s, arr);
        arr > 0
    }

    fn count_arrangements(&self, s: &str) -> usize {
        // number of ways to validate s
        let mut heads: HashMap<usize,(&CompletionDict<char>,usize)> = HashMap::new();
        heads.insert(self.completions.id, (&self.completions,1));
        for c in s.chars() {
            //println!("Match step {}", c);
            let mut new_heads = HashMap::new();
            for (head, path_count) in heads.values() {
                if let Some(next) = head.get(&c) {
                    if next.is_complete {
                        new_heads.entry(next.id).or_insert((&self.completions,0)).1 += path_count;
                    }
                    new_heads.entry(next.id).or_insert((next,0)).1 += path_count;
                }
            }
            heads = new_heads;
        }
        heads.values()
            .filter(|(d,_)| *d==&self.completions)
            .map(|(_,count)| *count)
            .next()
            .unwrap_or_default()
    }
}

fn parse_available(s: &str) -> Vec<&str>{
    s.split(", ").collect()
}

/*
fn build_validator(patterns: &[&str]) -> regex::Regex {
    let match_group = patterns.join("|");
    let pat = format!("^(?:{})+$", match_group);
    regex::Regex::new(pat.as_str()).unwrap()
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let available = parse_available("r, wr, b, g, bwu, rb, gb, br");
        let validator = Validator::from(&available);

        println!("{:?}", validator.completions);

        let possible = "brwrr\nbggr\ngbbr\nrrbgbr\nubwu\nbwurrg\nbrgr\nbbrgwb".split('\n')
            .filter(|s| validator.is_match(s))
            .count();

        assert_eq!(possible, 6);
    }
}
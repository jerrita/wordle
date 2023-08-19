use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

struct Wordle {
    sz: usize,
    vocab: Vec<String>,
    res: BTreeMap<u32, String>,
}

impl Wordle {
    fn new(sz: usize) -> Self {
        let mut vocab: Vec<String> = Vec::new();

        let file = File::open("res/words.txt").expect("File not found");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(word) = line {
                if word.is_ascii() && word.len() == sz {
                    vocab.push(word);
                }
            }
        }

        println!("vocab: {:?}", vocab.len());

        Wordle {
            sz,
            vocab,
            res: BTreeMap::new(),
        }
    }

    fn calc_entropy(&mut self) {
        for word in &self.vocab {
            let patterns = 3_u32.pow(self.sz as u32);
            let mut distribution = vec![0; patterns as usize];
            for other in &self.vocab {
                // 0: not match
                // 1: match, but not in the same position
                // 2: match, and in the same position
                // pattern: ternary arithmetic

                let mut pattern = 0;
                for (i, c) in word.chars().enumerate() {
                    if c == other.chars().nth(i).unwrap() {
                        pattern += 2 * 3_u32.pow(i as u32);
                    } else if other.contains(c) {
                        pattern += 3_u32.pow(i as u32);
                    }
                }

                distribution[pattern as usize] += 1;
            }

            let mut entropy = 0.0;
            for count in distribution {
                if count > 0 {
                    let p = count as f32 / self.vocab.len() as f32;
                    entropy -= p * p.log2();
                }
            }

            self.res.insert((entropy * 100.0) as u32, word.clone());
        }
    }

    fn filter_pattern(&self) -> Vec<String> {
        let mut word = String::new();
        print!("Input word: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut word).unwrap();

        let mut pattern_str = String::new();
        print!("Input pattern: ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut pattern_str).unwrap();

        // pattern_str: 0 1 2 2 1 ...
        let mut pattern: Vec<u32> = Vec::new();
        for c in pattern_str.trim().split_whitespace() {
            pattern.push(c.parse::<u32>().unwrap());
        }

        if pattern.len() != self.sz {
            println!("Invalid pattern");
            return Vec::new();
        }

        self.vocab.iter().filter(|other| {
            for (i, c) in word.trim().chars().enumerate() {
                if pattern[i] == 0 && other.contains(c) {
                    return false;
                }
                if pattern[i] == 1 && (!other.contains(c) || other.chars().nth(i).unwrap() == c) {
                    return false;
                }
                if pattern[i] == 2 && (!other.contains(c) || other.chars().nth(i).unwrap() != c) {
                    return false;
                }
            }
            true
        }).cloned().collect::<Vec<String>>()
    }

    fn update(&mut self, vocab: Vec<String>) {
        self.vocab = vocab;
    }
}

fn main() {
    let mut sz_str = String::new();
    print!("Input word size: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut sz_str).unwrap();
    let sz: usize = sz_str.trim().parse().unwrap();

    let mut wordle = Wordle::new(sz);
    while wordle.vocab.len() >= 1 {
        wordle.calc_entropy();

        // print top 5 words
        let mut cnt = 0;
        for (entropy, word) in wordle.res.iter().rev() {
            println!("{}: {}", entropy, word);
            cnt += 1;
            if cnt == 5 {
                break;
            }
        }

        let filtered = wordle.filter_pattern();
        if filtered.len() == 0 {
            println!("No word found");
            continue;
        }

        println!("vocab: {:?}", filtered.len());

        wordle.update(filtered);
        wordle.res.clear();
    }
}
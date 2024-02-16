use std::{
    borrow::Borrow,
    collections::{BinaryHeap, HashSet},
    fs,
    path::PathBuf,
};

use anyhow::Result;

///////////////////////////////////////////////////////////////////////////////

/*

*/

///////////////////////////////////////////////////////////////////////////////

#[inline]
fn t<T>(c: bool, a: T, b: T) -> T {
    if c {
        a
    } else {
        b
    }
}

#[inline]
fn min<T>(a: T, b: T, c: T) -> T
where
    T: Ord,
{
    a.min(b).min(c)
}

///////////////////////////////////////////////////////////////////////////////

fn levenshtein_distance(first: &str, second: &str) -> usize {
    // iterative single row based on [] and []

    let mut row: Vec<usize> = (0..second.len() + 1).into_iter().collect();

    let mut previous_diagonal;
    let mut previous_above = 0;

    println!("{:#?}", row);

    for i in 0..first.len() {
        row[0] = i + 1;

        for j in 0..second.len() {
            let indicator = t(first[i..i + 1] != second[j..j + 1], 1, 0);
            previous_diagonal = previous_above;
            previous_above = row[j + 1];
            row[j + 1] = min(
                previous_above + 1,
                row[j] + 1,
                previous_diagonal + indicator,
            );
        }
    }

    row[second.len()]
}
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Eq, PartialOrd)]
struct RankedWord {
    ranking: usize,
    word: String,
}

//---------------------------------------------------------------------------//

impl PartialEq for RankedWord {
    fn eq(&self, other: &Self) -> bool {
        self.ranking == other.ranking && self.word == other.word
    }
}

impl Ord for RankedWord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ranking.cmp(&other.ranking)
    }
}

///////////////////////////////////////////////////////////////////////////////

pub struct SpellChecker {
    dictionary: HashSet<String>,
}

//---------------------------------------------------------------------------//

impl SpellChecker {
    /// Returns a spell checker with no known words
    pub fn new() -> SpellChecker {
        SpellChecker {
            dictionary: HashSet::new(),
        }
    }

    //.......................................................................//

    /// Returns true if the word is in the dictionary, otherwise false
    pub fn check(&self, word: &str) -> bool {
        self.dictionary.contains(word)
    }

    /// Returns a sorted list of similar words
    pub fn suggest(&self, word: &str) -> Vec<String> {
        let mut suggestions = BinaryHeap::with_capacity(self.dictionary.len());

        for other in &self.dictionary {
            suggestions.push(RankedWord {
                ranking: levenshtein_distance(word, &other),
                word: other.to_string(),
            })
        }

        let suggestions = suggestions.into_sorted_vec();

        println!("{:#?}", suggestions);

        suggestions.iter().map(|c| c.word.to_owned()).collect()
    }

    //.......................................................................//

    /// Adds the given word to the dictionary
    pub fn add_word(&mut self, word: &str) {
        self.dictionary.insert(word.to_owned());
    }

    /// Returns all known words
    pub fn words(&self) -> Vec<&String> {
        self.dictionary.borrow().into_iter().collect()
    }

    /// Returns all known words in sorted order
    pub fn sorted_words(&self) -> Vec<&String> {
        let mut words = self.words();
        words.sort();
        words
    }

    //.......................................................................//

    /// Returns a formatted string of all known words in sorted order
    pub fn to_string(&self) -> String {
        self.sorted_words()
            .into_iter()
            .fold("".to_owned(), |acc, elem| acc + &elem + "\n")
    }

    //.......................................................................//

    /// Returns a spell checker with words from the provided dictionary file
    pub fn from_file(path: &PathBuf) -> Result<Self> {
        let mut context = Self::new();

        let contents = fs::read_to_string(path)?;

        for line in contents.lines() {
            let word = line.split("/").next().unwrap_or_default();
            context.add_word(word);
        }

        Ok(context)
    }

    /// Dumps all known words to the given file.
    /// Any existing file contents are overwritten.
    pub fn to_file(&self, path: &str) -> Result<()> {
        Ok(fs::write(path, self.to_string())?)
    }
}

///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn levenshtein_distance_tests() {
        assert_eq!(levenshtein_distance("INTENTION", "EXECUTION"), 5);
    }

    #[test]
    fn in_memory_tests() {
        let mut spell_checker = SpellChecker::new();

        assert!(!spell_checker.check("test"));
        assert_eq!(spell_checker.words(), Vec::<&String>::new());
        assert_eq!(spell_checker.to_string(), "");

        spell_checker.add_word("a");

        assert!(!spell_checker.check("test"));
        assert!(spell_checker.check("a"));
        assert_eq!(spell_checker.words(), vec!["a"]);
        assert_eq!(spell_checker.to_string(), "a\n");

        spell_checker.add_word("test");

        assert!(spell_checker.check("test"));
        assert!(spell_checker.check("a"));
        assert_eq!(spell_checker.sorted_words(), vec!["a", "test"]);
        assert_eq!(spell_checker.to_string(), "a\ntest\n");

        assert_eq!(spell_checker.suggest("tes"), vec!["test", "a"]);

        spell_checker.add_word("the");
        spell_checker.add_word("was");
        spell_checker.add_word("it");
        spell_checker.add_word("programming");
        spell_checker.add_word("cat");

        assert!(spell_checker.check("test"));
        assert!(spell_checker.check("a"));
        assert_eq!(
            spell_checker.sorted_words(),
            vec!["a", "cat", "it", "programming", "test", "the", "was"]
        );
        assert_eq!(
            spell_checker.to_string(),
            "a\ncat\nit\nprogramming\ntest\nthe\nwas\n"
        );
    }
}

///////////////////////////////////////////////////////////////////////////////

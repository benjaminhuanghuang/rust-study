use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
  let mut set = HashSet::new();
  let lowered_word = word.to_lowercase();
  let sorted_word = sorted_str(&lowered_word);

  possible_anagrams
    .iter()
    .filter(|&&anagram| {
      let lowered_anagram = anagram.to_lowercase();
      lowered_word != lowered_anagram && sorted_word == sorted_str(&lowered_anagram)
    })
    .for_each(|&anagram| {
      set.insert(anagram);
    });
  // for &anagram in possible_anagrams {
  //     let lowered_anagram = anagram.to_lowercase();
  //     if lowered_word != lowered_anagram &&
  //        sorted_word == sorted_str(&lowered_anagram) {
  //         set.insert(anagram);
  //     }
  // }
  set
}

pub fn anagrams_for2<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
  let lowered_word = word.to_lowercase();
  let sorted_word = sorted_str(&lowered_word);

  possible_anagrams
    .iter()
    .filter(|&&anagram| {
      let lowered_anagram = anagram.to_lowercase();
      lowered_word != lowered_anagram && sorted_word == sorted_str(&lowered_anagram)
    })
    .cloned()
    .collect() // convert the iterator to a HashSet
}

fn sorted_str(s: &str) -> String {
  let mut word = s.chars().collect::<Vec<_>>();
  word.sort();
  word.into_iter().collect()
}

/*
    .entry(key).or_insert() : avoids the need to manually check whether the key exists.
    The similar code in typescript:
    frequencyMap.set(currentCount, (frequencyMap.get(currentCount) || 0) - 1);

*/
use std::collections::HashMap;

// A structure to store the goal details of a team.
#[derive(Default)]
struct TeamScores {
  goals_scored: u8,
  goals_conceded: u8,
}

fn build_scores_table(results: &str) -> HashMap<&str, TeamScores> {
  // The name of the team is the key and its associated struct is the value.
  let mut scores = HashMap::<&str, TeamScores>::new();

  for line in results.lines() {
    let mut split_iterator = line.split(',');
    // NOTE: We use `unwrap` because we didn't deal with error handling yet.
    let team_1_name = split_iterator.next().unwrap();
    let team_2_name = split_iterator.next().unwrap();
    let team_1_score: u8 = split_iterator.next().unwrap().parse().unwrap();
    let team_2_score: u8 = split_iterator.next().unwrap().parse().unwrap();

    // TODO: Populate the scores table with the extracted details.
    // Keep in mind that goals scored by team 1 will be the number of goals
    // conceded by team 2. Similarly, goals scored by team 2 will be the
    // number of goals conceded by team 1.
    // Update team 1 stats
    let entry = scores.entry(team_1_name).or_insert(TeamScores {
      goals_scored: 0,
      goals_conceded: 0,
    });
    entry.goals_scored += team_1_score; // Goals scored
    entry.goals_conceded += team_2_score; // Goals conceded

    // Update team 2 stats
    let entry: &mut TeamScores = scores.entry(team_2_name).or_insert(TeamScores {
      goals_scored: 0,
      goals_conceded: 0,
    });
    entry.goals_scored += team_2_score; // Goals scored
    entry.goals_conceded += team_1_score; // Goals conceded
  }

  scores
}

fn main() {
  // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
  use super::*;

  const RESULTS: &str = "England,France,4,2
France,Italy,3,1
Poland,Spain,2,0
Germany,England,2,1
England,Spain,1,0";

  #[test]
  fn build_scores() {
    let scores = build_scores_table(RESULTS);

    assert!(["England", "France", "Germany", "Italy", "Poland", "Spain"]
      .into_iter()
      .all(|team_name| scores.contains_key(team_name)));
  }

  #[test]
  fn validate_team_score_1() {
    let scores = build_scores_table(RESULTS);
    let team = scores.get("England").unwrap();
    assert_eq!(team.goals_scored, 6);
    assert_eq!(team.goals_conceded, 4);
  }

  #[test]
  fn validate_team_score_2() {
    let scores = build_scores_table(RESULTS);
    let team = scores.get("Spain").unwrap();
    assert_eq!(team.goals_scored, 0);
    assert_eq!(team.goals_conceded, 3);
  }
}

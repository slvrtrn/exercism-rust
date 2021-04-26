use std::collections::HashMap;

enum MatchResult {
    Win,
    Loss,
    Draw,
}

impl MatchResult {
    pub fn new(input: &str) -> Self {
        match input {
            "win" => MatchResult::Win,
            "loss" => MatchResult::Loss,
            "draw" => MatchResult::Draw,
            s => panic!("Unrecognized MatchResult input: {}", s),
        }
    }
    pub fn opposite(&self) -> MatchResult {
        match self {
            MatchResult::Win => MatchResult::Loss,
            MatchResult::Loss => MatchResult::Win,
            MatchResult::Draw => MatchResult::Draw,
        }
    }
    pub fn points(&self) -> u32 {
        match self {
            MatchResult::Win => 3,
            MatchResult::Loss => 0,
            MatchResult::Draw => 1,
        }
    }
}

struct TeamStats {
    wins: u32,
    losses: u32,
    draws: u32,
    matches_played: u32,
    points: u32,
}

impl TeamStats {
    pub fn new(result: &MatchResult) -> Self {
        let points = result.points();
        match result {
            MatchResult::Win => Self { wins: 1, losses: 0, draws: 0, matches_played: 1, points },
            MatchResult::Loss => Self { wins: 0, losses: 1, draws: 0, matches_played: 1, points },
            MatchResult::Draw => Self { wins: 0, losses: 0, draws: 1, matches_played: 1, points },
        }
    }
    pub fn update_with_match_result(&mut self, result: &MatchResult) -> () {
        self.matches_played += 1;
        self.points += result.points();
        match result {
            MatchResult::Win => self.wins += 1,
            MatchResult::Loss => self.losses += 1,
            MatchResult::Draw => self.draws += 1,
        }
    }
}

fn process_team_result(
    team_stats: &mut HashMap<String, TeamStats>,
    match_result: &MatchResult,
    team: &str,
) -> () {
    return match team_stats.get_mut(team) {
        Some(ts) => {
            ts.update_with_match_result(&match_result);
        }
        _ => {
            team_stats.insert(team.to_owned(), TeamStats::new(&match_result));
        }
    };
}

pub fn tally(match_results: &str) -> String {
    let mut team_stats: HashMap<String, TeamStats> = HashMap::new();
    for line in match_results.lines() {
        let parsed: Vec<_> = line.split(';').map(|s| s.to_string()).collect();
        match parsed.as_slice() {
            [team1, team2, result] => {
                let match_result = MatchResult::new(result);
                process_team_result(&mut team_stats, &match_result, team1);
                process_team_result(&mut team_stats, &match_result.opposite(), team2);
            }
            _ => panic!("invalid line: {}", line)
        }
    }

    let mut result = vec![format!("{:<30} | MP |  W |  D |  L |  P", "Team")];
    let mut vec: Vec<_> = team_stats.into_iter().collect();
    vec.sort_by(|(team1, ts1), (team2, ts2)| {
        let points_ord = ts2.points.cmp(&ts1.points);
        if points_ord == std::cmp::Ordering::Equal {
            team1.cmp(team2)
        } else {
            points_ord
        }
    });

    for (team, stats) in vec {
        result.push(
            format!("{team:<30} | {mp:>2} | {w:>2} | {d:>2} | {l:>2} | {p:>2}",
                    team = team,
                    mp = stats.matches_played,
                    w = stats.wins,
                    d = stats.draws,
                    l = stats.losses,
                    p = stats.points)
        )
    }
    result.join("\n")
}

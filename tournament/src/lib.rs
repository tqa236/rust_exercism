use std::collections::HashMap;

#[derive(Default)]
struct TeamStats {
    matches_played: u32,
    wins: u32,
    draws: u32,
    losses: u32,
    points: u32,
}

impl TeamStats {
    fn add_win(&mut self) {
        self.matches_played += 1;
        self.wins += 1;
        self.points += 3;
    }

    fn add_loss(&mut self) {
        self.matches_played += 1;
        self.losses += 1;
        // No points for a loss
    }

    fn add_draw(&mut self) {
        self.matches_played += 1;
        self.draws += 1;
        self.points += 1;
    }
}

pub fn tally(match_results: &str) -> String {
    let header = "Team                           | MP |  W |  D |  L |  P";

    if match_results.is_empty() {
        return header.to_string();
    }

    let mut team_stats: HashMap<String, TeamStats> = HashMap::new();

    for line in match_results.lines() {
        let parts: Vec<&str> = line.split(';').collect();
        if parts.len() != 3 {
            continue;
        }

        let team1 = parts[0].to_string();
        let team2 = parts[1].to_string();
        let result = parts[2];

        team_stats.entry(team1.clone()).or_default();
        team_stats.entry(team2.clone()).or_default();

        match result {
            "win" => {
                team_stats.get_mut(&team1).unwrap().add_win();
                team_stats.get_mut(&team2).unwrap().add_loss();
            }
            "loss" => {
                team_stats.get_mut(&team1).unwrap().add_loss();
                team_stats.get_mut(&team2).unwrap().add_win();
            }
            "draw" => {
                team_stats.get_mut(&team1).unwrap().add_draw();
                team_stats.get_mut(&team2).unwrap().add_draw();
            }
            _ => {}
        }
    }

    let mut teams: Vec<(String, &TeamStats)> = team_stats
        .iter()
        .map(|(team, stats)| (team.clone(), stats))
        .collect();

    // Sort teams by points (descending) and then alphabetically in case of ties
    teams.sort_by(|a, b| b.1.points.cmp(&a.1.points).then_with(|| a.0.cmp(&b.0)));

    let mut result = vec![header.to_string()];

    for (team, stats) in teams {
        let row = format!(
            "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            team, stats.matches_played, stats.wins, stats.draws, stats.losses, stats.points
        );
        result.push(row);
    }

    result.join("\n")
}

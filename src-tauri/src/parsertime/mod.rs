use regex::Regex;

pub struct Request {
    pub file: String,
    pub source: String,
    pub compatibility: String,
}

struct Schemas {
    defensive_assist: Regex,
    dva_remech: Regex,
    echo_duplicate_start: Regex,
    echo_duplicate_end: Regex,
    hero_spawn: Regex,
    hero_swap: Regex,
    kill: Regex,
    match_end: Regex,
    match_start: Regex,
    mercy_rez: Regex,
    objective_captured: Regex,
    objective_updated: Regex,
    offensive_assist: Regex,
    payload_progress: Regex,
    player_stat: Regex,
    point_progress: Regex,
    remech_charged: Regex,
    round_end: Regex,
    round_start: Regex,
    setup_complete: Regex,
    ultimate_charged: Regex,
    ultimate_end: Regex,
    ultimate_start: Regex,
}

impl Schemas {
    fn new() -> Self {
        Schemas {
            defensive_assist: Regex::new(r"^\[\d{2}:\d{2}:\d{2}\] ,defensive_assist,\d{1,5}\.\d{2},[\p{L}\p{N}\s]+,[\p{L}\p{N}\s]+,[\p{L}\p{N}\.\s]+,(0|[\p{L}\p{N}\.\s]+)$").unwrap(),
            dva_remech: Regex::new(r"^\[\d{2}:\d{2}:\d{2}\] ,dva_remech,\d{1,5}\.\d{2},[\p{L}\p{N}\s]+,[\p{L}\p{N}\s]+,[\p{L}\p{N}\.\s]+,\d+$").unwrap(),
            echo_duplicate_start: Regex::new(r"^\[\d{2}:\d{2}:\d{2}\] ,echo_duplicate_start,\d{1,5}\.\d{2},[\p{L}\p{N}\s]+,[\p{L}\p{N}\s]+,[\p{L}\p{N}\.\s]+,[\p{L}\p{N}\.\s]+,\d+$").unwrap(),
            echo_duplicate_end: Regex::new(r"^\[\d{2}:\d{2}:\d{2}\] ,echo_duplicate_end,\d{1,5}\.\d{2},[\p{L}\p{N}\s]+,[\p{L}\p{N}\s]+,[\p{L}\p{N}\.\s]+,\d+$").unwrap(),
            hero_spawn: Regex::new(r"^\[\d{2}:\d{2}:\d{2}\] ,hero_spawn,(0|\d{1,5}\.\d{2}),[\p{L}\p{N}\s]+,[\p{L}\p{N}\s]+,[\p{L}\p{N}\.\s]+,(|[\p{L}\p{N}\.\s]+),\d+$").unwrap(),
            hero_swap: Regex::new(r"^\[\d{2}:\d{2}:\d{2}\] ,hero_swap,(0|\d{1,5}\.\d{2}),[\p{L}\p{N}\s]+,[\p{L}\p{N}\s]+,[\p{L}\p{N}\.\s]+,[\p{L}\p{N}\.\s]+,(0|\d{1,5}\.\d{2})$").unwrap(),
            kill: Regex::new(r"^\[\d{2}:\d{2}:\d{2}\] ,kill,\d{1,5}\.\d{2},[\p{L}\p{N}\s]+,[\p{L}\p{N}\s]+,[\p{L}\p{N}\.\s]+,[\p{L}\p{N}\.\s]+,[\p{L}\p{N}\.\s]+,[\p{L}\p{N}\.\s]+,(0|[\p{L}\p{N}\.\s]+),(\d{1,5}|\d{1,5}\.\d{2}),(True|0),(True|0)$").unwrap(),
            match_end: Regex::new(r"^\[\d{2}:\d{2}:\d{2}\] ,match_end,\d{1,5}\.\d{2},\d+,\d+,\d+$").unwrap(),
            match_start: Regex::new(r"^\[\d{2}:\d{2}:\d{2}\] ,match_start,\d+,[\p{L}\p{N}\s]+,[\p{L}\p{N}\s]+,[\p{L}\p{N}\s]+,[\p{L}\p{N}\s]+$").unwrap(),
            mercy_rez: Regex::new(r"^\[\d{2}:\d{2}:\d{2}\] ,mercy_rez,\d{1,5}\.\d{2},[\p{L}\p{N}\s]+,[\p{L}\p{N}\s]+,[\p{L}\p{N}\.\s]+,[\p{L}\p{N}\.\s]+,[\p{L}\p{N}\.\s]+,[\p{L}\p{N}\.\s]+$").unwrap(),
            objective_captured: Regex::new(r"^\[\d{2}:\d{2}:\d{2}\] ,objective_captured,\d{1,5}\.\d{2},\d+,[\p{L}\p{N}\s]+,\d+,(0|\d{1,5}\.\d{2}),(0|\d{1,5}\.\d{2}),(\d+|\d{1,5}\.\d{2})$").unwrap(),
            objective_updated: Regex::new(r"^\[\d{2}:\d{2}:\d{2}\] ,objective_updated,(\d{1,5}|\d{1,5}\.\d{2}),\d+,\d+,\d+$").unwrap(),
            offensive_assist: Regex::new(r"^\[\d{2}:\d{2}:\d{2}\] ,offensive_assist,\d{1,5}\.\d{2},[\p{L}\p{N}\s]+,[\p{L}\p{N}\s]+,[\p{L}\p{N}\.\s]+,(0|[\p{L}\p{N}\.\s]+)$").unwrap(),
            payload_progress: Regex::new(r"^\[\d{2}:\d{2}:\d{2}\] ,payload_progress,\d{1,5}\.\d{2},\d+,[\p{L}\p{N}\s]+,\d+,\d{1,5}\.\d{2}$").unwrap(),
            player_stat: Regex::new(r"^\[\d{2}:\d{2}:\d{2}\] ,player_stat,\d{1,5}\.\d{2},\d+,[\p{L}\p{N}\s]+,[\p{L}\p{N}\s]+,[\p{L}\p{N}\.\s]+,\d+,\d+,\d+,\d{1,7}(\.\d{2})?,\d{1,7}(\.\d{2})?,\d{1,7}(\.\d{2})?,\d{1,7}(\.\d{2})?,\d{1,7}(\.\d{2})?,\d{1,7}(\.\d{2})?,\d{1,7}(\.\d{2})?,\d{1,7}(\.\d{2})?,\d+,\d+,\d+,\d+,\d+,\d+,\d+,\d+,\d+,\d+,\d+,\d+(\.\d{2})?,\d+(\.\d{2})?,\d+(\.\d{2})?,\d+,\d+,\d+,\d+,\d+,\d{1,2}(\.\d{2})?,(0|\d{1,7}\.\d{2}),(0|\d{1,7}\.\d{2})$").unwrap(),
            point_progress: Regex::new(r"^\[\d{2}:\d{2}:\d{2}\] ,point_progress,\d{1,5}\.\d{2},\d+,[\p{L}\p{N}\s]+,\d+,\d{1,5}\.\d{2}$").unwrap(),
            remech_charged: Regex::new(r"^\[\d{2}:\d{2}:\d{2}\] ,remech_charged,\d{1,5}\.\d{2},[\p{L}\p{N}\s]+,[\p{L}\p{N}\.\s]+,[\p{L}\p{N}\.\s]+,(0|[\p{L}\p{N}\.\s]+),(\d{1,5}|\d{1,5}\.\d{2})$").unwrap(),
            round_end: Regex::new(r"^\[\d{2}:\d{2}:\d{2}\] ,round_end,\d{1,5}\.\d{2},\d+,[\p{L}\p{N}\s]+,\d+,\d+,\d+,(\d{1,5}|\d{1,5}\.\d{2}),(\d{1,5}|\d{1,5}\.\d{2}),(\d{1,5}|\d{1,5}\.\d{2})$").unwrap(),
            round_start: Regex::new(r"^\[\d{2}:\d{2}:\d{2}\] ,round_start,(0|\d{1,5}\.\d{2}),\d+,[\p{L}\p{N}\s]+,\d+,\d+,\d+").unwrap(),
            setup_complete: Regex::new(r"^\[\d{2}:\d{2}:\d{2}\] ,setup_complete,(0|\d{1,5}\.\d{2}),\d+,(0|\d{1,5}\.\d{2})$").unwrap(),
            ultimate_charged: Regex::new(r"^\[\d{2}:\d{2}:\d{2}\] ,ultimate_charged,\d{1,5}\.\d{2},[\p{L}\p{N}\s]+,[\p{L}\p{N}\.\s]+,[\p{L}\p{N}\.\s]+,(0|[\p{L}\p{N}\.\s]+),\d+$").unwrap(),
            ultimate_end: Regex::new(r"^\[\d{2}:\d{2}:\d{2}\] ,ultimate_end,\d{1,5}\.\d{2},[\p{L}\p{N}\s]+,[\p{L}\p{N}\.\s]+,[\p{L}\p{N}\.\s]+,(0|[\p{L}\p{N}\.\s]+),\d+$").unwrap(),
            ultimate_start: Regex::new(r"^\[\d{2}:\d{2}:\d{2}\] ,ultimate_start,\d{1,5}\.\d{2},[\p{L}\p{N}\s]+,[\p{L}\p{N}\.\s]+,[\p{L}\p{N}\.\s]+,(0|[\p{L}\p{N}\.\s]+),\d+$").unwrap(),
        }
    }
}

pub fn validate(request: Request) -> Result<String, String> {
    let file_text = request.file;

    let schemas = Schemas::new();

    let lines = file_text.lines();

    println!("Validating {} lines", lines.clone().count());

    for line in lines {
        match validate_line(line, &schemas) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
    }

    Ok("Validation successful".to_string())
}

fn validate_line(line: &str, schemas: &Schemas) -> Result<(), String> {
    let parts: Vec<&str> = line.split(',').collect();
    println!("parts: {:?}", parts);
    if parts.len() < 2 {
        return Err(format!("Invalid line: {}", line));
    }

    match parts[1] {
        "defensive_assist" => {
            if schemas.defensive_assist.is_match(line) {
                println!("Defensive assist: {}", line);
                Ok(())
            } else {
                println!("Invalid defensive_assist line: {}", line);
                Err(format!("Invalid defensive_assist line: {}", line))
            }
        }
        "dva_remech" => {
            if schemas.dva_remech.is_match(line) {
                println!("Dva remech: {}", line);
                Ok(())
            } else {
                println!("Invalid dva_remech line: {}", line);
                Err(format!("Invalid dva_remech line: {}", line))
            }
        }
        "echo_duplicate_start" => {
            if schemas.echo_duplicate_start.is_match(line) {
                println!("Echo duplicate start: {}", line);
                Ok(())
            } else {
                println!("Invalid echo_duplicate_start line: {}", line);
                Err(format!("Invalid echo_duplicate_start line: {}", line))
            }
        }
        "echo_duplicate_end" => {
            if schemas.echo_duplicate_end.is_match(line) {
                println!("Echo duplicate end: {}", line);
                Ok(())
            } else {
                println!("Invalid echo_duplicate_end line: {}", line);
                Err(format!("Invalid echo_duplicate_end line: {}", line))
            }
        }
        "hero_spawn" => {
            if schemas.hero_spawn.is_match(line) {
                println!("Hero spawn: {}", line);
                Ok(())
            } else {
                println!("Invalid hero_spawn line: {}", line);
                Err(format!("Invalid hero_spawn line: {}", line))
            }
        }
        "hero_swap" => {
            if schemas.hero_swap.is_match(line) {
                println!("Hero swap: {}", line);
                Ok(())
            } else {
                println!("Invalid hero_swap line: {}", line);
                Err(format!("Invalid hero_swap line: {}", line))
            }
        }
        "kill" => {
            if schemas.kill.is_match(line) {
                println!("Kill: {}", line);
                Ok(())
            } else {
                println!("Invalid kill line: {}", line);
                Err(format!("Invalid kill line: {}", line))
            }
        }
        "match_end" => {
            if schemas.match_end.is_match(line) {
                println!("Match end: {}", line);
                Ok(())
            } else {
                println!("Invalid match_end line: {}", line);
                Err(format!("Invalid match_end line: {}", line))
            }
        }
        "match_start" => {
            if schemas.match_start.is_match(line) {
                println!("Match start: {}", line);
                Ok(())
            } else {
                println!("Invalid match_start line: {}", line);
                Err(format!("Invalid match_start line: {}", line))
            }
        }
        "mercy_rez" => {
            if schemas.mercy_rez.is_match(line) {
                println!("Mercy rez: {}", line);
                Ok(())
            } else {
                println!("Invalid mercy_rez line: {}", line);
                Err(format!("Invalid mercy_rez line: {}", line))
            }
        }
        "objective_captured" => {
            if schemas.objective_captured.is_match(line) {
                println!("Objective captured: {}", line);
                Ok(())
            } else {
                println!("Invalid objective_captured line: {}", line);
                Err(format!("Invalid objective_captured line: {}", line))
            }
        }
        "objective_updated" => {
            if schemas.objective_updated.is_match(line) {
                println!("Objective updated: {}", line);
                Ok(())
            } else {
                println!("Invalid objective_updated line: {}", line);
                Err(format!("Invalid objective_updated line: {}", line))
            }
        }
        "offensive_assist" => {
            if schemas.offensive_assist.is_match(line) {
                println!("Offensive assist: {}", line);
                Ok(())
            } else {
                println!("Invalid offensive_assist line: {}", line);
                Err(format!("Invalid offensive_assist line: {}", line))
            }
        }
        "payload_progress" => {
            if schemas.payload_progress.is_match(line) {
                println!("Payload progress: {}", line);
                Ok(())
            } else {
                println!("Invalid payload_progress line: {}", line);
                Err(format!("Invalid payload_progress line: {}", line))
            }
        }
        "player_stat" => {
            if schemas.player_stat.is_match(line) {
                println!("Player stat: {}", line);
                Ok(())
            } else {
                println!("Invalid player_stat line: {}", line);
                Err(format!("Invalid player_stat line: {}", line))
            }
        }
        "point_progress" => {
            if schemas.point_progress.is_match(line) {
                println!("Point progress: {}", line);
                Ok(())
            } else {
                println!("Invalid point_progress line: {}", line);
                Err(format!("Invalid point_progress line: {}", line))
            }
        }
        "remech_charged" => {
            if schemas.remech_charged.is_match(line) {
                println!("Remech charged: {}", line);
                Ok(())
            } else {
                println!("Invalid remech_charged line: {}", line);
                Err(format!("Invalid remech_charged line: {}", line))
            }
        }
        "round_end" => {
            if schemas.round_end.is_match(line) {
                println!("Round end: {}", line);
                Ok(())
            } else {
                println!("Invalid round_end line: {}", line);
                Err(format!("Invalid round_end line: {}", line))
            }
        }
        "round_start" => {
            if schemas.round_start.is_match(line) {
                println!("Round start: {}", line);
                Ok(())
            } else {
                println!("Invalid round_start line: {}", line);
                Err(format!("Invalid round_start line: {}", line))
            }
        }
        "setup_complete" => {
            if schemas.setup_complete.is_match(line) {
                println!("Setup complete: {}", line);
                Ok(())
            } else {
                println!("Invalid setup_complete line: {}", line);
                Err(format!("Invalid setup_complete line: {}", line))
            }
        }
        "ultimate_charged" => {
            if schemas.ultimate_charged.is_match(line) {
                println!("Ultimate charged: {}", line);
                Ok(())
            } else {
                println!("Invalid ultimate_charged line: {}", line);
                Err(format!("Invalid ultimate_charged line: {}", line))
            }
        }
        "ultimate_end" => {
            if schemas.ultimate_end.is_match(line) {
                println!("Ultimate end: {}", line);
                Ok(())
            } else {
                println!("Invalid ultimate_end line: {}", line);
                Err(format!("Invalid ultimate_end line: {}", line))
            }
        }
        "ultimate_start" => {
            if schemas.ultimate_start.is_match(line) {
                println!("Ultimate start: {}", line);
                Ok(())
            } else {
                println!("Invalid ultimate_start line: {}", line);
                Err(format!("Invalid ultimate_start line: {}", line))
            }
        }
        _ => {
            println!("Unknown event type: {}", parts[1]);
            Err(format!("Unknown event type: {}", parts[1]))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defensive_assist_regex() {
        // valid defensive_assist line
        let line = "[00:15:45] ,defensive_assist,783.75,Team 1,YAI,Baptiste,0";
        let schemas = Schemas::new();
        assert!(schemas.defensive_assist.is_match(line));

        // invalid defensive_assist line
        let line = "[00:15:45] ,defensive_assist,783.75,Team 1,YAI,Baptiste,";
        assert!(!schemas.defensive_assist.is_match(line));
    }

    #[test]
    fn test_dva_remech_regex() {
        // valid dva_remech line
        let line = "[00:15:46] ,dva_remech,784.18,Team 1,Jinhyeok,D.Va,6";
        let schemas = Schemas::new();
        assert!(schemas.dva_remech.is_match(line));

        // invalid dva_remech line
        let line = "[00:15:46] ,dva_remech,784.18,Team 1,,D.Va,6";
        assert!(!schemas.dva_remech.is_match(line));
    }

    #[test]
    fn test_echo_duplicate_start_regex() {
        // valid echo_duplicate_start line
        let line = "[00:03:13] ,echo_duplicate_start,92.04,Team 2,BoneSaw,Echo,Orisa,1";
        let schemas = Schemas::new();
        assert!(schemas.echo_duplicate_start.is_match(line));

        // invalid echo_duplicate_start line
        let line = "[00:03:13] ,echo_duplicate_start,92.04,Team 2,BoneSaw,,Orisa,1";
        assert!(!schemas.echo_duplicate_start.is_match(line));
    }

    #[test]
    fn test_echo_duplicate_end_regex() {
        // valid echo_duplicate_end line
        let line = "[00:03:22] ,echo_duplicate_end,101.58,Team 2,BoneSaw,Echo,1";
        let schemas = Schemas::new();
        assert!(schemas.echo_duplicate_end.is_match(line));

        // invalid echo_duplicate_end line
        let line = "[00:03:22] ,echo_duplicate_end,101.58,Team 2,BoneSaw,Echo,";
        assert!(!schemas.echo_duplicate_end.is_match(line));
    }

    #[test]
    fn test_hero_spawn_regex() {
        // valid hero_spawn line
        let line = "[00:00:08] ,hero_spawn,0,Team 2,Shocker,Mauga,0,0";
        let schemas = Schemas::new();
        assert!(schemas.hero_spawn.is_match(line));

        // invalid hero_spawn line
        let line = "[00:00:08] ,hero_spawn,0,Team 2,,Mauga,0,0";
        assert!(!schemas.hero_spawn.is_match(line));
    }

    #[test]
    fn test_hero_swap_regex() {
        // valid hero_swap line
        let line = "[00:16:38] ,hero_swap,756.04,Team 1,グレイシー,Tracer,Symmetra,3.26";
        let schemas = Schemas::new();
        assert!(schemas.hero_swap.is_match(line));

        // invalid hero_swap line
        let line = "[00:16:38] ,hero_swap,756.04,Team 1,グレイシー,Tracer,,3.26";
        assert!(!schemas.hero_swap.is_match(line));
    }

    #[test]
    fn test_kill_regex() {
        // valid kill line
        let line = "[00:17:01] ,kill,778.77,Team 1,グレイシー,Sojourn,Team 2,BoneSaw,Echo,Secondary Fire,106.37,0,0";
        let schemas = Schemas::new();
        assert!(schemas.kill.is_match(line));

        // invalid kill line
        let line = "[00:17:01] ,kill,778.77,Team 1,Spingar,Sojourn,Team 2,BoneSaw,Echo,Secondary Fire,106.37,False,0";
        assert!(!schemas.kill.is_match(line));
    }

    #[test]
    fn test_match_end_regex() {
        // valid match_end line
        let line = "[00:17:32] ,match_end,809.08,3,1,2";
        let schemas = Schemas::new();
        assert!(schemas.match_end.is_match(line));

        // invalid match_end line
        let line = "[00:17:32] ,match_end,809.08,3,,2";
        assert!(!schemas.match_end.is_match(line));
    }

    #[test]
    fn test_match_start_regex() {
        // valid match_start line
        let line = "[00:00:00] ,match_start,0,0,0,0,0";
        let schemas = Schemas::new();
        assert!(schemas.match_start.is_match(line));

        // invalid match_start line
        let line = "[00:00:00] ,match_start,0,0,0,0";
        assert!(!schemas.match_start.is_match(line));
    }

    #[test]
    fn test_mercy_rez_regex() {
        // valid mercy_rez line
        let line = "[00:15:05] ,mercy_rez,631.04,Team 2,Algarra,Mercy,Team 2,Suzuka,Echo";
        let schemas = Schemas::new();
        assert!(schemas.mercy_rez.is_match(line));

        // invalid mercy_rez line
        let line = "[00:13:07] ,mercy_rez,513.31,Team 2,,Mercy,Team 2,Suzuka,Echo";
        assert!(!schemas.mercy_rez.is_match(line));
    }

    #[test]
    fn test_objective_captured_regex() {
        // valid objective_captured line
        let line = "[00:02:53] ,objective_captured,38.91,1,Team 1,2,0.01,0,0";
        let schemas = Schemas::new();
        assert!(schemas.objective_captured.is_match(line));

        // invalid objective_captured line
        let line = "[00:02:53] ,objective_captured,38.91,1,Team 1,2,0.01,0,";
        assert!(!schemas.objective_captured.is_match(line));
    }

    #[test]
    fn test_objective_updated_regex() {
        // valid objective_updated line
        let line = "[00:00:00] ,objective_updated,0,1,0,2";
        let schemas = Schemas::new();
        assert!(schemas.objective_updated.is_match(line));

        // invalid objective_updated line
        let line = "[00:00:00] ,objective_updated,,1,0,2";
        assert!(!schemas.objective_updated.is_match(line));
    }

    #[test]
    fn test_offensive_assist_regex() {
        // valid offensive_assist line
        let line = "[00:03:23] ,offensive_assist,69.12,Team 2,Toad,Orisa,0";
        let schemas = Schemas::new();
        assert!(schemas.offensive_assist.is_match(line));

        // invalid offensive_assist line
        let line = "[00:03:23] ,offensive_assist,69.12,Team 2,,Orisa,0";
        assert!(!schemas.offensive_assist.is_match(line));
    }

    #[test]
    fn test_payload_progress_regex() {
        // valid payload_progress line
        let line = "[00:03:18] ,payload_progress,82.64,1,Team 2,0,10.02";
        let schemas = Schemas::new();
        assert!(schemas.payload_progress.is_match(line));

        // invalid payload_progress line
        let line = "[00:03:18] ,payload_progress,82.64,,Team 2,0,10.02";
        assert!(!schemas.payload_progress.is_match(line));
    }

    #[test]
    fn test_player_stat_regex() {
        // valid player_stat line
        let line = "[00:05:46] ,player_stat,211.72,1,Team 1,PISSLORANDOM,Tracer,3,1,1,2204.43,0,2204.43,0,489.32,251.76,1163.58,0,0,0,2,1,0,0,0,1,0,0,37,0.09,0,0,0,1390,411,898,0,0,0.31,208.89";
        let schemas = Schemas::new();
        assert!(schemas.player_stat.is_match(line));

        // invalid player_stat line
        let line = "[00:05:46] ,player_stat,211.72,1,Team 1,PISSLORANDOM,Tracer,3,1,1,2204.43,0,2204.43,0,489.32,251.76,1163.58,0,0,0,2,1,0,0,0,1,0,0,37,0.09,0,0,0,1390,411,898,0,0,0.31,208.89,";
        assert!(!schemas.player_stat.is_match(line));
    }

    #[test]
    fn test_point_progress_regex() {
        // valid point_progress line
        let line = "[00:03:39] ,point_progress,85.50,1,Team 2,2,66.66";
        let schemas = Schemas::new();
        assert!(schemas.point_progress.is_match(line));

        // invalid point_progress line
        let line = "[00:02:53] ,point_progress,38.91,1,Team 1,0,";
        assert!(!schemas.point_progress.is_match(line));
    }

    #[test]
    fn test_remech_charged_regex() {
        // valid remech_charged line
        let line = "[00:15:43] ,remech_charged,781.77,Team 1,Jinhyeok,D.Va,0,6";
        let schemas = Schemas::new();
        assert!(schemas.remech_charged.is_match(line));

        // invalid remech_charged line
        let line = "[00:15:46] ,remech_charged,784.18,Team 1,Jinhyeok,D.Va,,6";
        assert!(!schemas.remech_charged.is_match(line));
    }

    #[test]
    fn test_round_end_regex() {
        // valid round_end line
        let line = "[00:15:58] ,round_end,796.49,2,Team 1,3,3,3,0,0,190.40";
        let schemas = Schemas::new();
        assert!(schemas.round_end.is_match(line));

        // invalid round_end line
        let line = "[00:17:32] ,round_end,809.08,3,Team 1,1,0,0,";
        assert!(!schemas.round_end.is_match(line));
    }

    #[test]
    fn test_round_start_regex() {
        // valid round_start line
        let line = "[00:00:00] ,round_start,0,1,Team 2,0,0,0";
        let schemas = Schemas::new();
        assert!(schemas.round_start.is_match(line));

        // invalid round_start line
        let line = "[00:00:00] ,round_start,0,1,Team 2,0,0,";
        assert!(!schemas.round_start.is_match(line));
    }

    #[test]
    fn test_setup_complete_regex() {
        // valid setup_complete line
        let line = "[00:01:30] ,setup_complete,0,1,239.99";
        let schemas = Schemas::new();
        assert!(schemas.setup_complete.is_match(line));

        // invalid setup_complete line
        let line = "[00:00:00] ,setup_complete,0,1,";
        assert!(!schemas.setup_complete.is_match(line));
    }

    #[test]
    fn test_ultimate_charged_regex() {
        // valid ultimate_charged line
        let line = "[00:03:13] ,ultimate_charged,102.95,Team 1,Spingar,Widowmaker,0,1";
        let schemas = Schemas::new();
        assert!(schemas.ultimate_charged.is_match(line));

        // invalid ultimate_charged line
        let line = "[00:15:46] ,ultimate_charged,784.18,Team 1,Jinhyeok,D.Va,,6";
        assert!(!schemas.ultimate_charged.is_match(line));
    }

    #[test]
    fn test_ultimate_end_regex() {
        // valid ultimate_end line
        let line = "[00:03:42] ,ultimate_end,131.77,Team 1,YAI,Baptiste,0,1";
        let schemas = Schemas::new();
        assert!(schemas.ultimate_end.is_match(line));

        // invalid ultimate_end line
        let line = "[00:03:42] ,ultimate_end,131.77,,YAI,Baptiste,0,1";
        assert!(!schemas.ultimate_end.is_match(line));
    }

    #[test]
    fn test_ultimate_start_regex() {
        // valid ultimate_start line
        let line = "[00:03:29] ,ultimate_start,119.63,Team 1,Spingar,Widowmaker,0,1";
        let schemas = Schemas::new();
        assert!(schemas.ultimate_start.is_match(line));

        // invalid ultimate_start line
        let line = "[00:03:29] ,ultimate_start,119.63,Team 1,,Widowmaker,0,1";
        assert!(!schemas.ultimate_start.is_match(line));
    }
}

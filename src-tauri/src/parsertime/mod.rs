use regex::Regex;

pub struct Request {
    pub file: String,
    pub source: String,
    pub compatibility: String,
}

struct Schemas {
    // defensive_assist: Regex,
    // dva_remech: Regex,
    // echo_duplicate_start: Regex,
    // echo_duplicate_end: Regex,
    // hero_spawn: Regex,
    // hero_swap: Regex,
    // kill: Regex,
    // match_end: Regex,
    match_start: Regex,
    mercy_rez: Regex,
    // objective_captured: Regex,
    // objective_updated: Regex,
    // offensive_assist: Regex,
    // payload_progress: Regex,
    // player_stat: Regex,
    // point_progress: Regex,
    // remech_charged: Regex,
    // round_end: Regex,
    // round_start: Regex,
    // setup_complete: Regex,
    // ultimate_charged: Regex,
    // ultimate_end: Regex,
    // ultimate_start: Regex,
}

impl Schemas {
    fn new() -> Self {
        Schemas {
            match_start: Regex::new(
                r"^\[\d{2}:\d{2}:\d{2}\] ,match_start,\d+,[\w\s]+,[\w\s]+,[\w\s]+,[\w\s]+$",
            )
            .unwrap(),
            mercy_rez: Regex::new(
                r"^\[\d{2}:\d{2}:\d{2}\] ,mercy_rez,\d{1,5}\.\d{2},[A-Za-z0-9\s]+,[A-Za-z0-9\s]+,[A-Za-z0-9\s]+,[A-Za-z0-9\s]+,[A-Za-z0-9\s]+,[A-Za-z0-9\s]+$"
            )
            .unwrap(),
        }
    }
}

pub fn validate(request: Request) -> Result<(), String> {
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

    Ok(())
}

fn validate_line(line: &str, schemas: &Schemas) -> Result<(), String> {
    let parts: Vec<&str> = line.split(',').collect();
    println!("parts: {:?}", parts);
    if parts.len() < 2 {
        return Err(format!("Invalid line: {}", line));
    }

    match parts[1] {
        "match_start" => {
            if schemas.match_start.is_match(line) {
                println!("Match start: {}", line);
                Ok(())
            } else {
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
        _ => Err(format!("Unknown event type: {}", parts[1])),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

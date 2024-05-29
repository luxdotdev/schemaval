use regex::Regex;

pub struct Request {
    pub file: String,
    pub source: String,
    pub compatibility: String,
}

struct Schemas {
    match_start: Regex,
    objective_updated: Regex,
    round_start: Regex,
    hero_spawn: Regex,
}

impl Schemas {
    fn new() -> Self {
        Schemas {
            match_start: Regex::new(
                r"^\[\d{2}:\d{2}:\d{2}\] ,match_start,\d+,[\w\s]+,[\w\s]+,[\w\s]+,[\w\s]+$",
            )
            .unwrap(),
            objective_updated: Regex::new(
                r"^\[\d{2}:\d{2}:\d{2}\] ,objective_updated,\d+,\d+,\d+,\d+$",
            )
            .unwrap(),
            round_start: Regex::new(
                r"^\[\d{2}:\d{2}:\d{2}\] ,round_start,\d+,\d+,\d+,\d+,\d+,\d+$",
            )
            .unwrap(),
            hero_spawn: Regex::new(
                r"^\[\d{2}:\d{2}:\d{2}\] ,hero_spawn,\d+,[\w\s]+,[\w\s]+,[\w\s]+,\d+,\d+$",
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
        "objective_updated" => {
            if schemas.objective_updated.is_match(line) {
                Ok(())
            } else {
                Err(format!("Invalid objective_updated line: {}", line))
            }
        }
        "round_start" => {
            if schemas.round_start.is_match(line) {
                Ok(())
            } else {
                Err(format!("Invalid round_start line: {}", line))
            }
        }
        "hero_spawn" => {
            if schemas.hero_spawn.is_match(line) {
                Ok(())
            } else {
                Err(format!("Invalid hero_spawn line: {}", line))
            }
        }
        _ => Err(format!("Unknown event type: {}", parts[1])),
    }
}

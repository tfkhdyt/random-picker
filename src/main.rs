use directories::BaseDirs;
use rand::Rng;
use std::collections::HashSet;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get user directories in a cross-platform manner.
    let base_dirs = BaseDirs::new().expect("Could not determine user directories");

    // Define the config directory and file for the games list.
    let config_dir = base_dirs.config_local_dir().join("game-picker");
    let games_file_path = config_dir.join("games.txt");

    // Define the cache directory and file for the played games list.
    let cache_dir = base_dirs.cache_dir().join("game-picker");
    let played_file_path = cache_dir.join("played-games.txt");

    // Ensure the configuration and cache directories exist.
    fs::create_dir_all(&config_dir)?;
    fs::create_dir_all(&cache_dir)?;

    // If the games file doesn't exist, create it with a default list.
    if !games_file_path.exists() {
        let default_games = ["Balatro"];
        let mut file = fs::File::create(&games_file_path)?;
        for game in &default_games {
            writeln!(file, "{}", game)?;
        }
    }

    // Read the games list from the config file.
    let games_content = fs::read_to_string(&games_file_path)?;
    let games: Vec<String> = games_content
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();

    if games.is_empty() {
        eprintln!("The games list is empty in {:?}", games_file_path);
        return Ok(());
    }

    // Ensure the played games file exists.
    if !played_file_path.exists() {
        fs::File::create(&played_file_path)?;
    }

    // Handle command-line arguments.
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "list" => {
                print_played_games(&played_file_path)?;
                return Ok(());
            }
            "reset" => {
                reset_played_games(&played_file_path)?;
                return Ok(());
            }
            _ => {} // Continue with main logic if no recognized subcommand.
        }
    }

    // Read played games into a HashSet for quick lookup.
    let mut played_set = HashSet::new();
    let file = fs::File::open(&played_file_path)?;
    for line in io::BufReader::new(file).lines() {
        let line = line?;
        if !line.trim().is_empty() {
            played_set.insert(line);
        }
    }

    // Filter available games (games not yet played).
    let mut available: Vec<&String> = games
        .iter()
        .filter(|game| !played_set.contains(*game))
        .collect();

    // If all games have been played, reset the list.
    if available.is_empty() {
        println!("All games have been played. Resetting the list.");
        reset_played_games(&played_file_path)?;
        available = games.iter().collect();
    }

    // Select a random game from the available list.
    let mut rng = rand::rng();
    let chosen = available[rng.random_range(0..available.len())];

    // Display the chosen game.
    println!("You should play: {}", chosen);

    // Append the chosen game to the played games file.
    let mut file = fs::OpenOptions::new()
        .append(true)
        .open(&played_file_path)?;
    writeln!(file, "{}", chosen)?;

    Ok(())
}

fn print_played_games<P: AsRef<Path>>(
    played_file_path: P,
) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(played_file_path)?;
    if content.trim().is_empty() {
        println!("No games have been played yet.");
    } else {
        println!("Played games:");
        print!("{}", content);
    }
    Ok(())
}

fn reset_played_games<P: AsRef<Path>>(
    played_file_path: P,
) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(played_file_path, "")?;
    println!("Played games list has been reset.");
    Ok(())
}

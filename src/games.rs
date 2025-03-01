use std::collections::HashSet;
use std::fs;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::{fs::File, path::PathBuf};

use anyhow::{Context, Ok};
use rand::Rng;

pub fn create_default_list_of_games_file(games_file_path: &PathBuf) -> anyhow::Result<()> {
    let default_games = ["Balatro"];
    let mut file = File::create(games_file_path).with_context(|| {
        format!(
            "Failed to create list of games file in {:?}",
            games_file_path
        )
    })?;
    for game in &default_games {
        writeln!(file, "{}", game).with_context(|| {
            format!(
                "Failed to write to list of games file in {:?}",
                games_file_path
            )
        })?;
    }

    Ok(())
}

fn list_games(games_file_path: &PathBuf) -> anyhow::Result<Vec<String>> {
    let games_content = fs::read_to_string(games_file_path)
        .with_context(|| format!("Failed to read list of games in {:?}", games_file_path))?;
    let games: Vec<String> = games_content
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();

    Ok(games)
}

fn list_played_games(played_file_path: &PathBuf) -> anyhow::Result<HashSet<String>> {
    let mut played_set = HashSet::new();
    let file = File::open(played_file_path)
        .with_context(|| format!("Failed to open played games file in {:?}", played_file_path))?;
    for line in BufReader::new(file).lines() {
        let line = line?;
        if !line.trim().is_empty() {
            played_set.insert(line);
        }
    }

    Ok(played_set)
}

fn list_available_games(
    games_file_path: &PathBuf,
    played_file_path: &PathBuf,
) -> anyhow::Result<Vec<String>> {
    let games = list_games(games_file_path)?;
    let played_games = list_played_games(played_file_path)?;

    if games.is_empty() {
        anyhow::bail!("The games list is empty in {:?}", games_file_path);
    }

    let available_games: Vec<String> = games
        .into_iter()
        .filter(|game| !played_games.contains(game))
        .collect();

    Ok(available_games)
}

pub fn append_chosen_to_played_games(
    played_file_path: &PathBuf,
    chosen: &String,
) -> anyhow::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .open(played_file_path)
        .with_context(|| format!("Failed to open played games file in {:?}", played_file_path))?;
    writeln!(file, "{}", chosen).with_context(|| {
        format!(
            "Failed to write into played games file in {:?}",
            played_file_path
        )
    })?;

    Ok(())
}

pub fn choose_random_game(
    games_file_path: &PathBuf,
    played_file_path: &PathBuf,
) -> anyhow::Result<String> {
    let games = list_games(games_file_path)?;
    if games.is_empty() {
        anyhow::bail!("The games list is empty in {:?}", games_file_path);
    }

    // Filter available games (games not yet played).
    let mut available = list_available_games(games_file_path, played_file_path)?;

    // If all games have been played, reset the list.
    if available.is_empty() {
        println!("All games have been played. Resetting the list.");
        reset_played_games(played_file_path)?;
        available = games.to_vec();
    }

    let mut rng = rand::rng();
    let chosen = &available[rng.random_range(0..available.len())];

    Ok(chosen.clone())
}

pub fn print_played_games(played_file_path: &PathBuf) -> anyhow::Result<()> {
    let content = fs::read_to_string(played_file_path)
        .with_context(|| format!("Failed to read played games file in {:?}", played_file_path))?;
    if content.trim().is_empty() {
        println!("No games have been played yet.");
    } else {
        println!("Played games:");
        print!("{}", content);
    }
    Ok(())
}

pub fn reset_played_games(played_file_path: &PathBuf) -> anyhow::Result<()> {
    fs::write(played_file_path, "").with_context(|| {
        format!(
            "Failed to clear played games file in {:?}",
            played_file_path
        )
    })?;
    println!("Played games list has been reset.");
    Ok(())
}

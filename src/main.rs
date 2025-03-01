mod dirs;
mod games;

use std::fs;

fn main() -> anyhow::Result<()> {
    // Define the config directory and file for the games list.
    let app_config_dir = dirs::get_app_config_dir();
    let games_file_path = app_config_dir.join("games.txt");

    // Define the cache directory and file for the played games list.
    let app_cache_dir = dirs::get_app_cache_dir();
    let played_file_path = app_cache_dir.join("played-games.txt");

    // Ensure the configuration and cache directories exist.
    fs::create_dir_all(&app_config_dir)?;
    fs::create_dir_all(&app_cache_dir)?;

    // If the games file doesn't exist, create it with a default list.
    if !games_file_path.exists() {
        games::create_default_list_of_games_file(&games_file_path)?;
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
                games::print_played_games(&played_file_path)?;
                return Ok(());
            }
            "reset" => {
                games::reset_played_games(&played_file_path)?;
                return Ok(());
            }
            _ => {} // Continue with main logic if no recognized subcommand.
        }
    }

    // Select a random game from the available list.
    let chosen = games::choose_random_game(&games_file_path, &played_file_path)?;

    // Display the chosen game.
    println!("You should play: {}", chosen);

    // Append the chosen game to the played games file.
    games::append_chosen_to_played_games(&played_file_path, &chosen)?;

    Ok(())
}

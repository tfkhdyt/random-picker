mod dirs;
mod item;

use std::fs;

fn main() -> anyhow::Result<()> {
    // Define the config directory and file for the items list.
    let app_config_dir = dirs::get_app_config_dir();
    let items_file_path = app_config_dir.join("items.txt");

    // Define the cache directory and file for the choosed items list.
    let app_cache_dir = dirs::get_app_cache_dir();
    let choosed_file_path = app_cache_dir.join("choosed-items.txt");

    // Ensure the configuration and cache directories exist.
    fs::create_dir_all(&app_config_dir)?;
    fs::create_dir_all(&app_cache_dir)?;

    // If the items file doesn't exist, create it with a default list.
    if !items_file_path.exists() {
        item::create_default_list_of_items_file(&items_file_path)?;
    }

    // Ensure the choosed items file exists.
    if !choosed_file_path.exists() {
        fs::File::create(&choosed_file_path)?;
    }

    // Handle command-line arguments.
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "list" => {
                item::print_choosed_item(&choosed_file_path)?;
                return Ok(());
            }
            "reset" => {
                item::reset_choosed_items(&choosed_file_path)?;
                return Ok(());
            }
            _ => {} // Continue with main logic if no recognized subcommand.
        }
    }

    // Select a random item from the available list.
    let chosen = item::choose_random_item(&items_file_path, &choosed_file_path)?;

    // Display the chosen item.
    println!("Item chosen: {}", chosen);

    // Append the chosen item to the choosed items file.
    item::append_chosen_to_choosed_items(&choosed_file_path, &chosen)?;

    Ok(())
}

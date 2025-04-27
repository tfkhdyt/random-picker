use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};

use anyhow::Ok;
use rand::Rng;

use crate::dirs;
use crate::setting::AppSetting;

fn read_cache_file(group_name: &String) -> anyhow::Result<File> {
    let app_cache_dir = dirs::get_app_cache_dir();
    let cache_file_path = app_cache_dir.join(format!("{}.txt", group_name));

    let file = OpenOptions::new()
        .read(true)
        .create(true)
        .append(true)
        .open(cache_file_path)?;

    Ok(file)
}

fn list_cache_items(group_name: &String) -> anyhow::Result<Vec<String>> {
    let file = read_cache_file(group_name)?; // Assuming read_cache_file handles the file opening
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line?; // Propagate any error that occurs while reading the line
        lines.push(line);
    }
    Ok(lines)
}

fn list_available_items(items: &[String], group_name: &String) -> anyhow::Result<Vec<String>> {
    let choosed_items = list_cache_items(group_name)?;

    if items.is_empty() {
        anyhow::bail!("The {} items list is empty", group_name);
    }

    let available_items: Vec<String> = items
        .iter()
        .filter(|&item| !choosed_items.contains(item))
        .cloned()
        .collect();

    Ok(available_items)
}

pub fn append_chosen_to_cache_file(group_name: &String, chosen: &String) -> anyhow::Result<()> {
    let mut file = read_cache_file(group_name)?;

    writeln!(file, "{}", chosen).expect("Failed to write into cache file");

    Ok(())
}

pub fn choose_random_item(items: &[String], group_name: &String) -> anyhow::Result<String> {
    if items.is_empty() {
        anyhow::bail!("The items list is empty");
    }

    // Filter available items (items not yet choosed).
    let mut available = list_available_items(items, group_name)?;

    // If all items have been chosen, reset the list.
    if available.is_empty() {
        println!("All items have been chosen. Resetting the list.");
        reset_cache(group_name)?;
        available = list_available_items(items, group_name)?;
    }

    let mut rng = rand::rng();
    let chosen = &available[rng.random_range(0..available.len())];

    Ok(chosen.clone())
}

pub fn print_choosed_item(group_name: &String) -> anyhow::Result<()> {
    let choosed_items = list_cache_items(group_name)?;
    if choosed_items.is_empty() {
        println!("No items have been chosen yet.");
    } else {
        choosed_items.iter().for_each(|item| println!("{}", item));
    }

    Ok(())
}

pub fn print_all_items(app_setting: &AppSetting, group_name: String) -> anyhow::Result<()> {
    let found_group = app_setting
        .groups
        .iter()
        .find(|item| item.name == group_name);

    if let Some(gr) = found_group {
        gr.items.iter().for_each(|item| {
            println!("{}", *item);
        });
    } else {
        println!("No items in this group is configured")
    };

    Ok(())
}

pub fn print_unchoosed_item(items: &[String], group_name: &String) -> anyhow::Result<()> {
    let available = list_available_items(items, group_name)?;
    if available.is_empty() {
        println!("No items have been chosen yet.");
    } else {
        available.iter().for_each(|a| println!("{}", a));
    }
    Ok(())
}

pub fn reset_cache(group_name: &String) -> anyhow::Result<()> {
    let app_cache_dir = dirs::get_app_cache_dir();
    let cache_file_path = app_cache_dir.join(format!("{}.txt", group_name));

    // Read all lines from the file
    let file = OpenOptions::new().read(true).open(&cache_file_path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    // Get the last line, if any
    let last_line = lines.last().cloned().unwrap_or_default();

    // Overwrite the file with only the last line (or empty if no lines)
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&cache_file_path)?;
    if !last_line.is_empty() {
        writeln!(file, "{}", last_line)?;
    }

    println!("Choosed items list has been reset.");
    Ok(())
}

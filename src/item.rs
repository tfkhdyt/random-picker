use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};

use anyhow::Ok;
use rand::Rng;

use crate::dirs;
use crate::setting::AppSetting;

// Cache for storing file handles
use std::collections::HashMap;
use std::sync::Mutex;
lazy_static::lazy_static! {
    static ref FILE_CACHE: Mutex<HashMap<String, File>> = Mutex::new(HashMap::new());
}

fn read_cache_file(group_name: &str) -> anyhow::Result<File> {
    let mut cache = FILE_CACHE.lock().unwrap();

    if let Some(file) = cache.get(group_name) {
        // Clone the file handle
        return Ok(file.try_clone()?);
    }

    let app_cache_dir = dirs::get_app_cache_dir();
    let cache_file_path = app_cache_dir.join(format!("{}.txt", group_name));

    let file = OpenOptions::new()
        .read(true)
        .create(true)
        .append(true)
        .open(cache_file_path)?;

    cache.insert(group_name.to_string(), file.try_clone()?);
    Ok(file)
}

fn list_cache_items(group_name: &str) -> anyhow::Result<Vec<String>> {
    let file = read_cache_file(group_name)?;
    let reader = BufReader::new(file);

    let mut lines = Vec::new();
    for line in reader.lines() {
        let line = line?;
        lines.push(line);
    }
    Ok(lines)
}

fn list_available_items(items: &[String], group_name: &str) -> anyhow::Result<Vec<String>> {
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

pub fn append_chosen_to_cache_file(group_name: &str, chosen: &str) -> anyhow::Result<()> {
    let mut file = read_cache_file(group_name)?;
    writeln!(file, "{}", chosen)?;
    Ok(())
}

pub fn choose_random_item(items: &[String], group_name: &str) -> anyhow::Result<String> {
    if items.is_empty() {
        anyhow::bail!("The items list is empty");
    }

    let mut available = list_available_items(items, group_name)?;

    if available.is_empty() {
        println!("All items have been chosen. Resetting the list.");
        reset_cache(group_name)?;
        available = list_available_items(items, group_name)?;
    }

    let mut rng = rand::rng();
    let chosen = &available[rng.random_range(0..available.len())];

    Ok(chosen.clone())
}

pub fn print_choosed_item(group_name: &str) -> anyhow::Result<()> {
    let choosed_items = list_cache_items(group_name)?;
    if choosed_items.is_empty() {
        println!("No items have been chosen yet.");
    } else {
        for item in choosed_items {
            println!("{}", item);
        }
    }
    Ok(())
}

pub fn print_all_items(app_setting: &AppSetting, group_name: &str) -> anyhow::Result<()> {
    if let Some(gr) = app_setting
        .groups
        .iter()
        .find(|item| item.name == group_name)
    {
        for item in &gr.items {
            println!("{}", item);
        }
    } else {
        println!("No items in this group is configured");
    }
    Ok(())
}

pub fn print_unchoosed_item(items: &[String], group_name: &str) -> anyhow::Result<()> {
    let available = list_available_items(items, group_name)?;
    if available.is_empty() {
        println!("No items have been chosen yet.");
    } else {
        for item in available {
            println!("{}", item);
        }
    }
    Ok(())
}

pub fn reset_cache(group_name: &str) -> anyhow::Result<()> {
    let app_cache_dir = dirs::get_app_cache_dir();
    let cache_file_path = app_cache_dir.join(format!("{}.txt", group_name));

    // Clear the file cache for this group
    FILE_CACHE.lock().unwrap().remove(group_name);

    // Read the file content
    let content = fs::read_to_string(&cache_file_path)?;
    let lines: Vec<&str> = content.lines().collect();

    // Keep only the last 2 lines if there are at least 2 lines
    // Otherwise clear the file completely
    let lines_to_keep = if lines.len() >= 2 {
        lines[lines.len() - 2..].to_vec()
    } else {
        Vec::new() // Return empty vector to clear the file
    };

    // Write back only the last 2 lines or empty string to clear
    let content_to_write = if !lines_to_keep.is_empty() {
        format!("{}\n", lines_to_keep.join("\n"))
    } else {
        String::new()
    };
    fs::write(&cache_file_path, content_to_write)?;

    println!("Choosed items list has been reset.");
    Ok(())
}

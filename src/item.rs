use std::collections::HashSet;
use std::fs;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::{fs::File, path::PathBuf};

use anyhow::{Context, Ok};
use rand::Rng;

pub fn create_default_list_of_items_file(items_file_path: &PathBuf) -> anyhow::Result<()> {
    let default_item = ["Balatro"];
    let mut file = File::create(items_file_path).with_context(|| {
        format!(
            "Failed to create list of items file in {:?}",
            items_file_path
        )
    })?;
    for item in &default_item {
        writeln!(file, "{}", item).with_context(|| {
            format!(
                "Failed to write to list of items file in {:?}",
                items_file_path
            )
        })?;
    }

    Ok(())
}

fn list_items(items_file_path: &PathBuf) -> anyhow::Result<Vec<String>> {
    let items_content = fs::read_to_string(items_file_path)
        .with_context(|| format!("Failed to read list of items in {:?}", items_file_path))?;
    let items: Vec<String> = items_content
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();

    Ok(items)
}

fn list_choosed_items(choosed_file_path: &PathBuf) -> anyhow::Result<HashSet<String>> {
    let mut choosed_set = HashSet::new();
    let file = File::open(choosed_file_path).with_context(|| {
        format!(
            "Failed to open choosed items file in {:?}",
            choosed_file_path
        )
    })?;
    for line in BufReader::new(file).lines() {
        let line = line?;
        if !line.trim().is_empty() {
            choosed_set.insert(line);
        }
    }

    Ok(choosed_set)
}

fn list_available_items(
    items_file_path: &PathBuf,
    choosed_file_path: &PathBuf,
) -> anyhow::Result<Vec<String>> {
    let items = list_items(items_file_path)?;
    let choosed_items = list_choosed_items(choosed_file_path)?;

    if items.is_empty() {
        anyhow::bail!("The items list is empty in {:?}", items_file_path);
    }

    let available_items: Vec<String> = items
        .into_iter()
        .filter(|item| !choosed_items.contains(item))
        .collect();

    Ok(available_items)
}

pub fn append_chosen_to_choosed_items(
    choosed_file_path: &PathBuf,
    chosen: &String,
) -> anyhow::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .open(choosed_file_path)
        .with_context(|| {
            format!(
                "Failed to open choosed items file in {:?}",
                choosed_file_path
            )
        })?;
    writeln!(file, "{}", chosen).with_context(|| {
        format!(
            "Failed to write into choosed items file in {:?}",
            choosed_file_path
        )
    })?;

    Ok(())
}

pub fn choose_random_item(
    items_file_path: &PathBuf,
    choosed_file_path: &PathBuf,
) -> anyhow::Result<String> {
    let items = list_items(items_file_path)?;
    if items.is_empty() {
        anyhow::bail!("The items list is empty in {:?}", items_file_path);
    }

    // Filter available items (items not yet choosed).
    let mut available = list_available_items(items_file_path, choosed_file_path)?;

    // If all items have been chosen, reset the list.
    if available.is_empty() {
        println!("All items have been chosen. Resetting the list.");
        reset_choosed_items(choosed_file_path)?;
        available = items.to_vec();
    }

    let mut rng = rand::rng();
    let chosen = &available[rng.random_range(0..available.len())];

    Ok(chosen.clone())
}

pub fn print_choosed_item(choosed_file_path: &PathBuf) -> anyhow::Result<()> {
    let content = fs::read_to_string(choosed_file_path).with_context(|| {
        format!(
            "Failed to read choosed items file in {:?}",
            choosed_file_path
        )
    })?;
    if content.trim().is_empty() {
        println!("No items have been chosen yet.");
    } else {
        println!("Choosed items:");
        print!("{}", content);
    }
    Ok(())
}

pub fn reset_choosed_items(choosed_file_path: &PathBuf) -> anyhow::Result<()> {
    fs::write(choosed_file_path, "").with_context(|| {
        format!(
            "Failed to clear choosed items file in {:?}",
            choosed_file_path
        )
    })?;
    println!("Choosed items list has been reset.");
    Ok(())
}

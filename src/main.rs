mod dirs;
mod group;
mod item;
mod setting;

use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use clap_complete::Shell;
use setting::AppSetting;
use std::{fs, io};

#[derive(Parser)]
#[command(name = "random-picker")]
#[command(version, about = "Random item picker", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long, value_parser = group::parse_group, help = "Group name")]
    group: Option<String>,
}

#[derive(ValueEnum, Clone, Debug)]
enum ListType {
    All = 0,
    Chosen = 1,
    Unchosen = 2,
}

#[derive(ValueEnum, Clone)]
enum ResetType {
    All = 0,
    Chosen = 1,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "List configured items")]
    List {
        #[arg(value_enum, default_value_t = ListType::Chosen)]
        list_type: ListType,
    },

    #[command(about = "Reset configured items")]
    Reset,

    #[command(about = "Generate shell completion")]
    Completion { shell: Shell },
}

fn main() -> anyhow::Result<()> {
    let app_setting = AppSetting::get_instance();

    let app_cache_dir = dirs::get_app_cache_dir();
    fs::create_dir_all(&app_cache_dir)?;
    // let choosed_file_path = app_cache_dir.join("choosed-items.txt");

    let cli = Cli::parse();

    let group_name = cli.group.or(app_setting.default_group.clone());
    let Some(group_name) = group_name else {
        anyhow::bail!("No group provided");
    };

    let group = app_setting.groups.iter().find(|g| g.name == group_name);
    let Some(group) = group else {
        anyhow::bail!("Group item not found")
    };

    match &cli.command {
        Some(Commands::List { list_type }) => match list_type {
            ListType::All => item::print_all_items(&app_setting, group_name)?,
            ListType::Chosen => item::print_choosed_item(&group_name)?,
            ListType::Unchosen => item::print_unchoosed_item(&group.items, &group_name)?,
        },
        Some(Commands::Reset) => item::reset_cache(&group_name)?,
        Some(Commands::Completion { shell }) => {
            let mut cmd = Cli::command();
            clap_complete::generate(*shell, &mut cmd, "rp", &mut io::stdout());
        }
        None => {
            let chosen = item::choose_random_item(&group.items, &group_name)?;

            println!("Item chosen: {}", chosen);

            item::append_chosen_to_cache_file(&group_name, &chosen)?;
        }
    }
    Ok(())
}

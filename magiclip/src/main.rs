use anyhow::Result;
use clipboard::{ClipboardContext, ClipboardProvider};
use colored::*;
use console::{style, Style};
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use magiclip::{ClipboardClient, DaemonClient};

#[tokio::main]
async fn main() -> Result<()> {
    let discovered_services = DaemonClient::new("127.0.0.1", 6061)?
        .fetch_discovered_services()
        .await?;

    if discovered_services.is_empty() {
        eprintln("no discovered services");
        return Ok(());
    }

    let mut theme = ColorfulTheme::default();
    theme.active_item_prefix = style("▸".to_string()).cyan();
    theme.active_item_style = Style::new().cyan().underlined();

    let mut select = Select::with_theme(&theme);

    let items = &discovered_services
        .iter()
        .map(|s| s.name())
        .collect::<Vec<&String>>();

    select.items(&items).default(0);

    let selected_name = items[select.interact()?];

    let service = discovered_services
        .iter()
        .find(|s| s.name() == selected_name)
        .unwrap();

    let contents = ClipboardClient::new(service.address(), 6060)?
        .fetch_clipboard()
        .await?;

    match contents {
        Some(c) => {
            let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
            clipboard.set_contents(c.clone()).unwrap();
            println!("{} {}", "Copied to clipboard:".green(), c.bold());
        }
        None => eprintln("clipboard is empty"),
    };

    Ok(())
}

fn eprintln(message: &str) {
    eprintln!("{} {}", "error:".bright_red().bold(), message.bold());
}

use anyhow::{Context, Result};
use colored::*;
use console::{style, Style};
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use magiclip::{ClipboardClient, DaemonClient};

static DAEMON_HOST: &'static str = "127.0.0.1";
static DAEMON_PORT: u16 = 6061;
static CLIPBOARD_PORT: u16 = 6060;

#[tokio::main]
async fn main() -> Result<()> {
    let daemon = DaemonClient::new(DAEMON_HOST, DAEMON_PORT)?;
    let discovered_services = daemon.list_discovered_services().await?;

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

    let contents = ClipboardClient::new(service.address(), CLIPBOARD_PORT)?
        .fetch_clipboard()
        .await?;

    match contents {
        Some(c) => {
            daemon
                .set_clipboard(&c)
                .await
                .context("could not push clipboard to daemon")?;

            println!("{} {}", "Copied to clipboard:".green(), c.bold());
        }
        None => eprintln("clipboard is empty"),
    };

    Ok(())
}

fn eprintln(message: &str) {
    eprintln!("{} {}", "error:".bright_red().bold(), message.bold());
}

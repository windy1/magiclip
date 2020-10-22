use anyhow::{Context, Result};
use clipboard::{ClipboardContext, ClipboardProvider};
use std::env;
use std::fs;

pub fn push_clipboard() -> Result<()> {
    let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
    let contents = clipboard.get_contents().unwrap_or_else(|_| String::new());
    let fname = format!("{}/.magiclip/clipboard.txt", env::var("HOME")?);
    fs::write(&fname, &contents).context("could not write to clipboard file")
}

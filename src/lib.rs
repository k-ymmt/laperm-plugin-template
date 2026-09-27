//! A minimal Laperm Plugin: one Command that inserts today's date at the caret.
//!
//! Rename the crate, edit `manifest.json`, and replace `InsertDate` with your own logic.

use laperm_plugin::{clock, editor, settings, Plugin, Result};
use serde::Deserialize;

/// The Plugin Settings declared in `manifest.json` (`settings::get` fills in the defaults).
#[derive(Deserialize)]
struct Settings {
    format: String,
}

struct InsertDate;

impl Plugin for InsertDate {
    fn new() -> Result<Self> {
        Ok(InsertDate)
    }

    fn run_command(&mut self, command: &str) -> Result<()> {
        match command {
            "insert-date" => insert_date(),
            other => Err(format!("unknown command {other}").into()),
        }
    }
}

fn insert_date() -> Result<()> {
    let Some(state) = editor::get()? else {
        return Err("Open a Note first.".into());
    };
    let settings: Settings = settings::get()?;
    let date = clock::now()?.format(&settings.format);
    state.replace_selection(&date)?;
    Ok(())
}

laperm_plugin::export_plugin!(InsertDate);

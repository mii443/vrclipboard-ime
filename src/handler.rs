use clipboard::{ClipboardContext, ClipboardProvider};
use clipboard_master::{ClipboardHandler, CallbackResult};
use crate::{config::Config, conversion::Conversion};
use anyhow::Result;

pub struct ConversionHandler {
    pub config: Config,
    conversion: Conversion,
    clipboard_ctx: ClipboardContext,
    last_text: String,
}

impl ConversionHandler {
    pub fn new(config: Config) -> Result<Self> {
        let conversion = Conversion::new(config.clone())?;
        let clipboard_ctx = ClipboardProvider::new().unwrap();

        Ok(Self { config, conversion, clipboard_ctx, last_text: String::new() })
    }
}

impl ClipboardHandler for ConversionHandler {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        if let Ok(mut contents) = self.clipboard_ctx.get_contents() {
            if contents != self.last_text {
                if contents.starts_with(&self.config.prefix) {
                    let parsed_contents = contents.split_off(1);
                    let converted = self.conversion.convert_text(&parsed_contents).unwrap();

                    self.last_text = converted.clone();
                    self.clipboard_ctx.set_contents(converted).unwrap();
                } else {
                    self.last_text = contents;
                }
            }
        }
        CallbackResult::Next
    }
}

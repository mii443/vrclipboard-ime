use crate::{config::Config, converter::converter::{get_custom_converter, Converter}};
use anyhow::Result;

pub struct ConversionBlock {
    pub text: String,
    pub converter: Box<dyn Converter>,
}

pub struct Conversion {
    pub config: Config,
}

impl Conversion {
    pub fn new(config: Config) -> Result<Self> {
        Ok(Self { config })
    }

    pub fn convert_text(&self, text: &str) -> Result<String> {
        let blocks = self.split_text(text)?;
        self.convert_blocks(blocks)
    }

    pub fn convert_blocks(&self, blocks: Vec<ConversionBlock>) -> Result<String> {
        let mut result = String::new();
        for block in blocks {
            let converted = self.convert_block(&block)?;
            result.push_str(&converted);
        }
        Ok(result)
    }

    pub fn convert_block(&self, block: &ConversionBlock) -> Result<String> {
        block.converter.convert(&block.text)
    }

    pub fn split_text(&self, text: &str) -> Result<Vec<ConversionBlock>> {
        let mut text = text.to_string();
        let mut blocks = Vec::new();
        let mut current_converter = 'r';

        if text.starts_with(&self.config.command) {
            text = text.split_off(1);
            current_converter = text.chars().next().unwrap();
            text = text.split_off(1);
        }

        for (i, command_splitted) in text.split(&self.config.command).enumerate() {
            let mut command_splitted = command_splitted.to_string();
            if i != 0 {
                current_converter = command_splitted.chars().next().unwrap();
                command_splitted = command_splitted.split_off(1);
            }

            for splitted in command_splitted.split(&self.config.split) {
                blocks.push(ConversionBlock {
                    text: splitted.to_string(),
                    converter: get_custom_converter(current_converter)
                });
            }
        }

        Ok(blocks)
    }
}

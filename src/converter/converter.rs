use anyhow::Result;

use super::{calculator::CalculatorConverter, hiragana::HiraganaConverter, none_converter::NoneConverter, roman_to_kanji::RomanToKanjiConverter};

pub trait Converter {
    fn convert(&self, text: &str) -> Result<String>;
}

pub fn get_custom_converter(prefix: char) -> Box<dyn Converter> {
    match prefix {
        'r' => {
            Box::new(RomanToKanjiConverter) as Box<dyn Converter>
        }
        'h' => {
            Box::new(HiraganaConverter) as Box<dyn Converter>
        }
        'c' => {
            Box::new(CalculatorConverter) as Box<dyn Converter>
        }
        'n' => {
            Box::new(NoneConverter) as Box<dyn Converter>
        }
        _ => panic!("Unknown prefix: {}", prefix),
    }
}

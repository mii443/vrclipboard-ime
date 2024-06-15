mod com;
mod felanguage;
mod handler;
mod conversion;
mod config;
mod converter;

use anyhow::Result;
use clipboard_master::Master;
use com::Com;
use config::Config;
use handler::ConversionHandler;

fn main() -> Result<()> {
    let _com = Com::new()?;
    
    let config = Config::load()?;
    let conversion_handler = ConversionHandler::new(config)?;

    let mut master = Master::new(conversion_handler);

    master.run()?;

    Ok(())
}

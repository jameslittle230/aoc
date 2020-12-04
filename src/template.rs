use crate::{AOCOutput, Variant};
use std::error::Error;

pub fn main(buffer: &String, variant: Variant) -> Result<AOCOutput, Box<dyn Error>> {

    let stderr = format!("Standard Error");
    let stdout = format!("Standard Output");

    Ok(AOCOutput { stderr, stdout })
}

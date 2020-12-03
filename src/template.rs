use std::{
    error::Error,
};

use super::AOCOutput;

pub fn main(buffer: &String) -> Result<AOCOutput, Box<dyn Error>> {

    let stderr = format!("Standard Error");
    let stdout = format!("Standard Output");

    Ok(AOCOutput { stderr, stdout })
}

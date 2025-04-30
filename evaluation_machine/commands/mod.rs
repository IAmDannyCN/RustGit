pub mod test1;
pub mod test1s;
pub mod test2;
pub mod test2s;
pub mod test3;
pub mod test3s;
pub mod test4;
pub mod test5;

use std::error::Error;

pub type CommandResult = Result<(), Box<dyn Error>>;

pub trait Command {
    fn execute(&self) -> CommandResult;
}
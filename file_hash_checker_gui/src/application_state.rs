use std::env;
use druid::{Data, Lens};
use file_hash_checker::file_hash::FileHash;

#[derive(Clone, Data, Lens)]
pub(crate) struct ApplicationState {
    pub(crate) file_hash: Result<String, &'static str>,
}

impl ApplicationState {
    pub(crate) fn update_file_hash_from_command_args(&mut self) {
        self.file_hash = (|| -> Result<String, &'static str> {
            if env::args().count() < 2 {
                return Err("No filename given");
            }
            Ok(format!("{}", FileHash::new(&env::args().nth(1).ok_or("Unexpected error in command args")?)?.get_hash()?))
        })();
    }
}
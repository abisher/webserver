use super::base::Base;
use super::super::enums::TaskStatus;

use super::super::traits::get::Get;
use super::super::traits::edit::Edit;
use super::super::traits::create::Create;


pub struct Pending {
    pub super_struct: Base,
}

impl Pending {
    pub fn new(input_file: &str) -> Self {
        let base = Base {
            title: input_file.to_string(),
            status: TaskStatus::PENDING,
        };

        Self { super_struct: base }
    }
}

impl Get for Pending {}

impl Edit for Pending {}

impl Create for Pending {}
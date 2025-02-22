use crate::database_setup;

use database_setup::Fields;

impl database_setup::Database {
    pub fn insertData(&self, database_name: &String, table_name: &String, data: Vec<Fields>) {
        todo!("Implement insertData");
    }
}
use std::fs::{
    self, 
    OpenOptions
    };
// use std::io::{Read, Write, stdin};
use std::path::Path;
use std::io::Write;
use serde_json;



pub struct Database{
    root_path:String,
}

pub struct Fields {
    pub name: String,
    pub data_type: DataType,
    pub primary_key: bool,
    pub size: u32,
}
#[derive(Debug)]
pub enum DataType {
    String,
    Integer,
    Float,
    Boolean,
}

impl std::fmt::Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::String => write!(f, "String"),
            DataType::Integer => write!(f, "Integer"),
            DataType::Float => write!(f, "Float"),
            DataType::Boolean => write!(f, "Boolean"),
        }
    }
}


impl Database{
    pub fn init()->Database{
        let dir_path = "config/AdatA";
        if !Path::new(dir_path).exists(){
            fs::create_dir_all(dir_path).expect("Failed to create config directory");
        }
        Database{
            root_path:dir_path.to_string(),
        }
    }
    pub fn new(&self, db_name:&str){
        let database_path = format!("{}/{}", self.root_path, db_name);
        if Path::new(&database_path).exists(){
            println!("Database already exists");
            return;
        }
        fs::create_dir(self.root_path.to_string()+"/"+db_name).expect("Failed to create database");
    }
    pub fn list_databases(){
        let dir_path = "config/AdatA/";
        Self::format_display(dir_path);
    }
    fn format_display(database_dir: &str){
        match fs::read_dir(database_dir) {
            Ok(entries) => {
                println!("Available Databases:\n--------------------");
                for (i, entry) in entries.filter_map(Result::ok).enumerate() {
                    if let Some(filename) = entry.file_name().to_str() {
                        println!("{}. {}", i + 1, filename);
                    }
                }
            }
            Err(_) => println!("❌ Error: Database directory '{}' not found.", database_dir),
        }
    }
    pub fn create_table(&self,database_name:&str,table_name:&str,infromation:Vec<Fields>){
        /*
        table information will be stored in a file named table_config.json
        for example:
        {
            "table_name": "users",
            "columns":[ 
                {
                    "name": "id",
                    "type": "integer",
                    "primary_key": true,
                    "size": 32bit -> 32 char
                },
                {
                    "name": "name",
                    "type": "String",
                    "primary_key": false,
                    "size": 100 -> 100 char
                },
                {
                    "name": "email",
                    "type": "String",
                    "primary_key": false,
                    "size": 100 -> 100 char
                },
                {
                    "name": "password",
                    "type": "String",
                    "primary_key": false,
                    "size": 100 -> 100 char
                } 
            ]
        }
        
        for example string (100) -> 100 char, integer (32) -> 32 char, float (32) -> 32 char, boolean (1) -> 1 char

        format of the storing is 
        <----id---->|<----name---->|<----email---->|<----password---->|
        key -> id 
         */

        let dir_path = format!("{}/{}",self.root_path,database_name);
        match fs::read_dir(&dir_path){
            Ok(_) => {
                let base_path = dir_path;
                let file_path = format!("{}/{}.txt", base_path, table_name);
                // ToDo check if table already exists should update table configuration or not
                if std::path::Path::new(&file_path).exists() {
                    println!("Table already exists");
                    return;
                }
                fs::File::create(&file_path).expect("Failed to create table");
                fs::File::create(format!("{}/.table_config.json",base_path))
                    .expect("Failed to create table config");
                self.create_table_configuration(table_name, infromation);
            }
            Err(_)=>{
                println!("❌ Error: Database directory '{}' not found.", dir_path);
            }
        }
    }
    fn create_table_configuration(&self, table_name:&str, information:Vec<Fields>){
        let file_path = format!("{}/.table_config.json", format!("{}/{}", self.root_path, table_name));
        match fs::File::create(file_path) {
            Ok(mut file) => {
            let columns: Vec<_> = information
                .iter()
                .map(|field| {
                    serde_json::json!({
                        "name": field.name.clone(),
                        "data_type": field.data_type.to_string(),
                        "size": field.size,
                        "primary_key": field.primary_key
                    })
                })
                .collect();
            let table_config = serde_json::json!({
                "table_name": table_name,
                "columns": columns
            });
            let json_data =
                serde_json::to_string_pretty(&table_config).expect("Failed to serialize table configuration");
            file.write_all(json_data.as_bytes())
                .expect("Failed to write JSON to file");
            println!("Table configuration created successfully.");
            }
            Err(_) => {
            todo!("Handle error");
            }
        }
    }
}
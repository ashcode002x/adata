mod database_data;
mod database_setup;
mod bitcode;

use std::vec;

use bitcode::Value;
use database_setup::Fields;


fn main() {
    
    let db = database_setup::Database::init();
    database_setup::Database::list_databases();
    db.new("test");
    let mut information:Vec<Fields> = Vec::new();
    information.push(Fields{
        name:"name".to_string(),
        data_type:database_setup::DataType::String,
        primary_key:true,
        size:100,
    });
    information.push(Fields{
        name:"age".to_string(),
        data_type:database_setup::DataType::Integer,
        primary_key:false,
        size:8,
    });
    db.create_table("test","test",information);

    let data: Vec<Fields> = Vec::new();

    db.insertData(&"test".to_string(), &"test".to_string(), data);

    let temp = Value::new(10);
    println!("{:?}", temp.data());
    let temp = Value::new("Hello");
    println!("{:?}", temp.data());

}

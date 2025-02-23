use crate::database_setup;
use std::fs::File;
use std::io::{BufWriter, Write, BufReader, Read, Seek, SeekFrom};
use std::fs::OpenOptions;

impl database_setup::Database {
    pub fn insert_data(&self, database_name: &String, table_name: &String, data:&Vec<Vec<u8>>) {
        let table_path = format!("{}/{}/{}.bin", self.root_path, database_name, table_name);
        for d in data{
            match self.write_to_bin_file(&table_path, &d) {
                Ok(_) => println!("Data inserted successfully"),
                Err(e) => println!("Error inserting data: {}", e),
            }
        }
    }
    pub fn select_data(&self, database_name: &String, table_name: &String) -> Vec<Vec<u8>> {
        let table_path = format!("{}/{}/{}.bin", self.root_path, database_name, table_name);
        let mut ans = Vec::new();
        let mut count = 0;
        loop {
            match self.read_from_bin_file(&table_path, 20, count) {
                Ok(data) => {
                    ans.push(data);
                    count += 1;
                },
                Err(e) => {
                    println!("Error reading data: {}", e);
                    break;
                },
            }
        }
        ans
    }
    // Function to write binary data to a .bin file
    fn write_to_bin_file(&self,filename: &str, data: &Vec<u8>) -> std::io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(filename)?;
        // let file = File::create(filename)?;
        // let mut writer = BufWriter::new(file);
        let mut writer = BufWriter::new(file);
        writer.write_all(data)?;
        writer.flush()?;
        Ok(())
    }
    fn read_from_bin_file(&self, filename: &str, block_size: usize, seek_index: usize) -> std::io::Result<Vec<u8>> {
        let mut file = File::open(filename)?; // Open the file
        let mut reader = BufReader::new(file);
    
        // Seek to the required block position
        let seek_position = seek_index * block_size;
        reader.seek(SeekFrom::Start(seek_position as u64))?;
    
        // Read the required block
        let mut buffer = vec![0; block_size];
        reader.read_exact(&mut buffer)?;
    
        Ok(buffer)
    }
}
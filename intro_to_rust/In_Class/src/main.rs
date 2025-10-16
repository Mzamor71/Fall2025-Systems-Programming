use std::fs;
use std::io::{self, Write};
use std::path::Path;

enum FileOperation {
    Create(String),
    Rename(String, String),
    Delete(String),
}

fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::Create(filename) => {
            // TODO: Implement file creation logic
            let mut file = fs::File::create(&filename).expect("Failed to create file");

            println!("File '{}' created successfully.", filename);
        }
        FileOperation::Rename(old_name, new_name) => {
            // TODO: Implement file renaming logic
            fs::rename(&old_name, &new_name).expect("Failed to rename file");
            println!("File renamed from '{}' to '{}' successfully.", old_name, new_name);
        }
        FileOperation::Delete(filename) => {
            // TODO: Implement file deletion logic
            fs::remove_file(&filename).expect("Failed to delete file");
            println!("File '{}' deleted successfully.", filename);
        }
    }
}

fn main() {
    while true {
        println!("Choose an operation:");
        println!("1. Create a new file");
        println!("2. Rename an existing file");
        println!("3. Delete a file");
        println!("4. Exit");
        

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                // TODO: Prompt for new filename and call perform_operation, if file exitsts, do not overwrite
                let mut filename = String::new();
                print!("Enter the name of the new file: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut filename).unwrap();
                let filename = filename.trim().to_string();
                if Path::new(&filename).exists() {
                    println!("File '{}' already exists. Not overwriting.", filename);
                } 
                else {
                    perform_operation(FileOperation::Create(filename));
                }
                
            }
            "2" => {
                // TODO: Prompt for old and new filenames and call perform_operation
                // If old file does not exist, print error message
                let mut old_name = String::new();
                let mut new_name = String::new();
                print!("Enter the current name of the file: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut old_name).unwrap();
                let old_name = old_name.trim().to_string();
                if !Path::new(&old_name).exists() {
                    println!("File '{}' does not exist. Cannot rename.", old_name); 
                }  
                else {
                    print!("Enter the new name for the file: ");
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut new_name).unwrap();
                    let new_name = new_name.trim().to_string();
                    perform_operation(FileOperation::Rename(old_name, new_name));
                }
            }
            "3" => {
                // TODO: Prompt for filename to delete and call perform_operation
                let mut filename = String::new();
                print!("Enter the name of the file to delete: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut filename).unwrap();
                let filename = filename.trim().to_string();
                perform_operation(FileOperation::Delete(filename));
            }
            "4" => {
                break;
            }
            _ => println!("Invalid choice"),
        }
    }
}
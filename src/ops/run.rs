use std::collections::hash_map::DefaultHasher;
use std::hash::{Hasher, Hash};

use crate::engines::EngineImpl;
use crate::load;

pub async fn run<T>(migrations_location: &str, engine: &T) -> anyhow::Result<()> where T: EngineImpl{
    let dir_files = load::get_dir_files(&migrations_location).await?; 
    println!("Migration Count: {:?}", dir_files.len());
   
    let mut hasher = DefaultHasher::new();

    for file in dir_files {
        println!("Running migration {:?}", file.name);
        let mut file_handle = load::load_file_data(&[migrations_location, &"/", &file.name].join("")).await?;
        let mut file_buffer = Vec::new();
        
        file_handle.read_to_end(&mut file_buffer)?;
        std::mem::drop(file_handle);

        let file_data_converted = String::from_utf8_lossy(&file_buffer).to_string();
        file_data_converted.hash(&mut hasher);

        let hash = hasher.finish();

        engine.build_migration_table().await?;
        
        let exists = engine.hash_exists(&hash).await?;
        println!("Exists Flag: {:?}", (if exists { "Exists - Skipping" } else {"Does Not Exist"}));
        if exists == true {
            continue;
        }

        engine.migrate(&file_data_converted).await?;
        engine.add_hash(&hash, &file.name).await?;
        println!("Migration finished {:?}", file.name);
    }

    return Ok(());
}
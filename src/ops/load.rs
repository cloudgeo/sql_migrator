use std::cmp::Ordering;
use std::io::BufRead;

use generic_filesystem;
use generic_filesystem::FileProvider;
use regex::Regex;

// our program will run program sequentially.
// migration    ----|
//                  | 0000001_[name].sql
//                  | 0000002_[name].sql
//                  | 0000003_[name].sql
//                  | 0000004_[name].sql
//                  | ....
const SORT_ORDER: std::cmp::Ordering = std::cmp::Ordering::Less;

pub async fn get_dir_files(folder_path: &str) -> anyhow::Result<Vec<generic_filesystem::FileEntry>>{
    let provider = generic_filesystem::LocalFileProvider {base: folder_path.to_string()};

    let mut files: Vec<generic_filesystem::FileEntry> = provider.read_dir("").await?;
    
    let re = Regex::new(r"[0-9]").unwrap();

    files = files.into_iter().filter(|file| {
        re.is_match(&file.name)
    }).collect();
    
    match SORT_ORDER {
        Ordering::Less => {
            files.sort_by(|a, b| {
                a.name.split_at(7).0.to_string().parse::<i32>().unwrap().cmp(&b.name.split_at(7).0.to_string().parse::<i32>().unwrap())
            });
        },
        _ => unimplemented!()
    }

    return Ok(files);
}

pub async fn load_file_data(path: &str) -> anyhow::Result<Box<dyn BufRead>> {
    let provider = generic_filesystem::LocalFileProvider {base: path.to_string()};

    let buf = provider.read_file_buffer("").await?;

    return Ok(buf);
}
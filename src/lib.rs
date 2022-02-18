#![feature(proc_macro_hygiene, new_uninit)]


#[macro_use]
extern crate lazy_static;
use crate::keyboard::ShowKeyboardArg;

use std::{fs, io::{Error, ErrorKind, Result}, path::{Path, PathBuf}, collections::HashMap, sync::Mutex};
use rand::Rng;
use walkdir::WalkDir;
use arcropolis_api::*;

mod keyboard;

const RANDOMIZE_PATH: &str = "rom:/ClassicModeSelector_FilesToCatch/";
const FILECHOICE_PATH: &str = "rom:/ClassicModeSelector";

lazy_static! {
    static ref FILE_HOLDER: Mutex<HashMap<u64, PathBuf>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };    
}

pub fn random_file_select(directory: &Path) -> Result<Vec<String>>{
    let mut rng = rand::thread_rng();






	
	let mut folders = HashMap::new();
	for entry in fs::read_dir(FILECHOICE_PATH)?
	{
		
		let entry = entry?;
		let path = entry.path();
		if path.is_dir() {
			folders.insert(folders.len(), format!("{}", path.display()));
		}
		
	}
	let mut folder_choice = 0;
	if folders.len() > 1
	{
		println!("{}","Many Folders.");
		let folder_text = format!("Enter Folder ID (0-{})", (folders.len()-1).to_string());
		folder_choice = (ShowKeyboardArg::new().header_text(&folder_text)
						.show().unwrap_or("0".to_string())).parse::<usize>().unwrap();
	}
	else if folders.len() <= 0
	{
		println!("{}","No folders");
		//println!("{}","No folders");
		return Err(Error::new(ErrorKind::Other, "No Folders Found! Please add some to ClassicModeSelector!"))
	}

	if folder_choice >= folders.len()
	{
		folder_choice = 0;
	}

	println!("{}",folder_choice);
	println!("{}",folders.get(&folder_choice).unwrap().to_string());


	let mut files = HashMap::new();
	//println!("{}",directory.display());
	
	
	
	
	
	
	
	for entry in fs::read_dir(/*directory*/folders.get(&folder_choice).unwrap().to_string())? {
		let entry = entry?;
		let path = entry.path();
		if !&path.is_dir() {
			files.insert(files.len(), format!("{}", path.display()));
		}
	}

	let count = files.len();

	if count <= 0 {
		return Err(Error::new(ErrorKind::Other, "No Files Found!"))
	}
	
	
	let display = format!("Enter Classic Mode ID (0-{})", (count-1).to_string());
	
	
	let mut user_input = ShowKeyboardArg::new().header_text(&display).show().unwrap_or("0".to_string());
	
	
	if user_input == "R" || user_input == "r"
	{
		println!("{}","Random");
		user_input = rng.gen_range(0..count).to_string();
	}
	
	let mut random_result = user_input.parse::<usize>().unwrap();
	//let random_result = rng.gen_range(0..count);
	
	if random_result >= files.len()
	{
		//return Err(Error::new(ErrorKind::Other, "File chosen outside range!"))
		random_result = 0;
	}
	
	println!("{}",files.get(&random_result).unwrap().to_string());
	Ok(vec![files.get(&random_result).unwrap().to_string(),directory.display().to_string()])
}

#[arc_callback]
fn arc_file_callback(hash: u64, data: &mut [u8]) -> Option<usize>{
	
	//let ogFile = ;
	
    match random_file_select(FILE_HOLDER.lock().unwrap().get(&hash).unwrap()){
        Ok(col) => {
			let s = &col[0];
			let d = &col[1];
			println!("{}",s.to_string());
			println!("{}",d.to_string());
			
            let file = fs::read(s).unwrap();
			//let file2 = fs::read(d).unwrap();
			
			println!("{:?}",..file.len());
			//println!("{:?}",..file.len());
			//println!("{:?}",..file2.len());
            
            // Shoutouts to Genwald
            data[..file.len()].copy_from_slice(&file);

            Some(file.len())
        },
        Err(_err) => None
    }
}

/*
#[stream_callback]
fn stream_file_callback(hash: u64) -> Option<String>{    
    match random_file_select(FILE_HOLDER.lock().unwrap().get(&hash).unwrap()){
        Ok(s) => Some(s),
        Err(_err) => None
    }
}
*/


fn get_biggest_size_from_path(path: &Path) -> usize{
    let mut biggest_size: usize = 0;

    for entry in fs::read_dir(path).unwrap() {
		
		let size;
		let entry_inner = entry.unwrap();
		
		//let entry = entry?;
		let path = entry_inner.path();
		if path.is_dir() {
			size = get_biggest_size_from_path(&path);
		}
		else
		{
			size = entry_inner.metadata().unwrap().len() as usize;
		
			
		}
        if size > biggest_size {
            biggest_size = size;
        }
    };

    biggest_size
}

#[skyline::main(name = "arc-randomizer")]
pub fn main() {
    if !Path::new(RANDOMIZE_PATH).exists(){
        return;
    }

    for entry in WalkDir::new(&RANDOMIZE_PATH) {
        let entry = entry.unwrap();

        if entry.path().is_dir() && format!("{}", &entry.path().display()).contains("."){

            let path = &format!("{}", &entry.path().display())[RANDOMIZE_PATH.len()..].replace(";", ":").replace(".mp4", ".webm");
            
            let hash = hash40(path);
            
            FILE_HOLDER.lock().unwrap().insert(hash.as_u64(), entry.path().to_path_buf());
            
            if path.contains("stream"){//None of these should be stream files.
                //stream_file_callback::install(hash);
            }else{
				let path = Path::new(FILECHOICE_PATH);
                arc_file_callback::install(hash, get_biggest_size_from_path(/*&entry.path()*/path));
            }

        }
    }
}
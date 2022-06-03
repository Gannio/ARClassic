#![feature(proc_macro_hygiene, new_uninit)]


#[macro_use]
extern crate lazy_static;

use std::{fs, io::{Error, ErrorKind, Result}, path::{Path, PathBuf}, collections::HashMap, sync::Mutex};
use rand::Rng;
use walkdir::WalkDir;
use arcropolis_api::*;

mod webmenu;

const RANDOMIZE_PATH: &str = "rom:/ARClassic_FilesToCatch/";
const FILECHOICE_PATH: &str = "sd:/ultimate/ClassicRoutes";




lazy_static! {
    static ref FILE_HOLDER: Mutex<HashMap<u64, PathBuf>> = {
        let m = HashMap::new();
        Mutex::new(m)
    };  
}

lazy_static! {
    static ref VANILLA_HOLDER: Mutex<Vec<String>> = Mutex::new(vec![]);
}


pub fn classic_file_select(directory: &Path) -> Result<Vec<String>>{
    let mut rng = rand::thread_rng();

	let mut folders = HashMap::new();//Firstly, go through the folders in the route folder to see if we need to perform a folder selection.
	let mut folders_text = vec![];//Used so that the webapp can actually get the name of the file.
	
	for entry in fs::read_dir(FILECHOICE_PATH)?
	{
		
		let entry = entry?;
		let path = entry.path();
		if path.is_dir() {
			folders.insert(folders.len(), format!("{}", path.display()));
			folders_text.insert(folders_text.len(),format!("{}",path.display()));
		}
		
	}
	let mut folder_choice = 0;
	let mut search_internal = false;
	let mut ishub = true;//Currently, this is just to have the HTML page know whether to change the name of the default button or not.
	if folders.len() > 1//If more than a single folder, prompt the user for multiple folders using the wep applet UI.
	{
		println!("[ARClassic]{}","Many Folders.");

		let mut webpage = webmenu::Routes{
			routes: folders_text, ishub
		};
		
		let web_out = webpage.get_file_index();//SKINS.lock().unwrap().get_file_index();
		//println!("{}",web_out.to_string());
		
		if web_out == "*Default"//Use * for special return values as they can never be in a normal file system.
		{
			folder_choice = folders.len();
			search_internal = true;//folder_choice = -2;//Special case to read from the in-game files.
		}
		else if web_out == "*Random"
		{
			folder_choice = rng.gen_range(0..folders.len());
		}
		else
		{
			let output = webpage.routes.iter().position(|s| s == &web_out).unwrap();//Set the folder choice to the index of the selected choice in the array.
			folder_choice = output;
		}
	}
	else if folders.len() <= 0
	{
		println!("[ARClassic]{}","No folders");
		return Err(Error::new(ErrorKind::Other, "No Folders Found! Please add some to \"sd:/ultimate/ClassicRoutes\"!"))
	}
	else
	{
		println!("[ARClassic]{}","Single Folder.");//Assume we are only using the first folder's contents if there's only one.
	}

	if folder_choice >= folders.len()
	{
		folder_choice = 0;
	}

	println!("[ARClassic]{}",folder_choice);
	println!("[ARClassic]{}",folders.get(&folder_choice).unwrap().to_string());


	//Below is pretty much the same as above, but for the files within folders instead of the folders themselves, so follow the comments above for below.
	let mut files = HashMap::new();//Getting files within the folder now.
	let mut files_text = vec![];
	//println!("{}",directory.display());
	
	
	
	
	
	
	
	

	if search_internal//Read from internal files, only accept those with proper naming format.
	{
			let folder_to_read = &String::from(format!("arc:/"));
			//println!("{}",folder_to_read); //Seems to crash right after this.
			//Need to do two runarounds: One for all base-game characters + Pirahana Plant, and another for DLC characters.
			let vanilla = &VANILLA_HOLDER.lock().unwrap();
			
			let mut i = 0;
			while i < vanilla.len()
			{
				let value = format!("arc:/0x{:x}",hash40(&vanilla[i]).as_u64());
				//println!("{};{}",vanilla[i], value);//mak
				files.insert(files.len(), value);
				files_text.insert(files_text.len(), format!("{}", vanilla[i]));
				i = i+1;
			}
	}
	else
	{
		for entry in fs::read_dir(/*directory*/&folders.get(&folder_choice).unwrap().to_string())? {
			let entry = entry?;
			let path = entry.path();
			if !&path.is_dir() {
				files.insert(files.len(), format!("{}", path.display()));
				files_text.insert(files_text.len(), format!("{}", path.display()));
			}
		}
	}
	println!("[ARClassic]File acquisition finished.");
	let count = files.len();

	if count <= 0 {
		return Err(Error::new(ErrorKind::Other, "No Files Found!"))
	}
	ishub = false;
	let mut webpage = webmenu::Routes{
			routes: files_text,ishub
		};
	let mut user_input = webpage.get_file_index();
	println!("[ARClassic]{}",user_input.to_string());
	
	if user_input == "*Default"
	{
		return Err(Error::new(ErrorKind::Other, "Default Route"));
	}
	else if user_input == "*Random"
	{
		user_input = rng.gen_range(0..count).to_string();
	}
	else
	{
		let output = webpage.routes.iter().position(|s| s == &user_input).unwrap();
		user_input = output.to_string();
	}
	
	
	let mut file_result = user_input.parse::<usize>().unwrap();
	
	if file_result >= files.len()
	{
		file_result = 0;
	}
	
	println!("[ARClassic]{}",files.get(&file_result).unwrap().to_string());
	Ok(vec![files.get(&file_result).unwrap().to_string(),directory.display().to_string()])
}

#[arc_callback]
fn arc_file_callback(hash: u64, data: &mut [u8]) -> Option<usize>{
	
	//let ogFile = ;
	
    match classic_file_select(FILE_HOLDER.lock().unwrap().get(&hash).unwrap()){
        Ok(col) => {
			let s = &col[0];
//			let d = &col[1];
			
            let file = fs::read(s).unwrap();
			
			println!("[ARClassic]{:?}",..file.len());
            
            // Shoutouts to Genwald
            data[..file.len()].copy_from_slice(&file);

            Some(file.len())
        },
        Err(_err) => None
    }
}


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


#[skyline::main(name = "arclassic")]
pub fn main() {
    if !Path::new(RANDOMIZE_PATH).exists(){
        return;
    }
	//println!("Starting");
	let optionsPath = Path::new(FILECHOICE_PATH);
	let maxSize = get_biggest_size_from_path(/*&entry.path()*/optionsPath);
	let mut i = 0;
    for entry in WalkDir::new(&RANDOMIZE_PATH) {
        let entry = entry.unwrap();
        if entry.path().is_dir() && format!("{}", &entry.path().display()).contains("."){
            let path = &format!("{}", &entry.path().display())[RANDOMIZE_PATH.len()..];//.replace(";", ":").replace(".mp4", ".webm"); Last bits we don't need as we aren't using streaming.
            //println!("Adding {}",path);
            let hash = hash40(path);
            
            FILE_HOLDER.lock().unwrap().insert(hash.as_u64(), entry.path().to_path_buf());
            
			let convToVanilla = str::replace(&entry.path().display().to_string(),RANDOMIZE_PATH,"");
			
			VANILLA_HOLDER.lock().unwrap().insert(i,convToVanilla);
			i = i+1;
			arc_file_callback::install(hash, maxSize);
			

        }
    }
	//println!("Done");
}
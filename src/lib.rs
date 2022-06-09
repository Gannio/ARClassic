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
	if folders.len() > 0//If more than a single folder, prompt the user for multiple folders using the wep applet UI.
	{
		println!("[ARClassic]{}","At least 1 folder.");

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
			folder_choice = rng.gen_range(0..folders.len()+1);
			if folder_choice >= folders.len()//Treat the final folder as vanilla.
			{
				folder_choice = folders.len();
				search_internal = true;
			}
		}
		else
		{
			let output = webpage.routes.iter().position(|s| s == &web_out).unwrap();//Set the folder choice to the index of the selected choice in the array.
			folder_choice = output;
		}
	}
	else if folders.len() == 0
	{
		println!("[ARClassic]{}","No folders.");//Assume we are only using the vanilla folder's contents if there's only one.
		search_internal = true;
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
	
	
	
	
	
	
	
	

	if search_internal//Read from internal files.
	{
			let vanilla = &VANILLA_HOLDER.lock().unwrap();
			
			let mut i = 0;
			while i < vanilla.len()
			{
				let mut value = format!("mods:/{}",&vanilla[i]);
				if (!Path::new(&value).exists())//Switch to vanilla mount if it doesn't exist in arc mount. We have to do this backward from what you expect otherwise 3.4.0 will crash.
				{//I should also note this is kind of a 'hope and pray' until we can get 4.0.0 to double check but can't check for exists rn so...eh.
					println!("[ARClassic]Switch to vanilla.");
					value = format!("arc:/0x{:x}",hash40(&vanilla[i]).as_u64());
					files.insert(files.len(), value);
					files_text.insert(files_text.len(), format!("{}", vanilla[i]));
				}
				else
				{
					println!("[ARClassic]{};{}",vanilla[i], value);//mak
					//Todo: fix this to include the new files when 4.0.0 comes out. For now, exclude internal files as we use thie dumb auto-write workaround.
				}
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
			println!("[ARClassic]Reading...{}",s);
			
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
#[repr(C)]
#[derive(Copy, Clone)]
pub enum Event {
	ArcFilesystemMounted,
	ModFilesystemMounted,
}

pub type EventCallbackFn = extern "C" fn(Event);

extern "C" {
	fn arcrop_register_event_callback(ty: Event, callback: EventCallbackFn);
}



pub extern "C" fn init(event: Event) {
	
	if !Path::new(RANDOMIZE_PATH).exists(){
        return;
    }
	//println!("Starting");
	let options_path = Path::new(FILECHOICE_PATH);
	let mut max_size = get_biggest_size_from_path(/*&entry.path()*/options_path);
	let mut i = 0;
	
	let mut vanilla_max_size = 0;//Slightly larger than Steve's max size: 8192 bytes.
	
	//Todo: Figure out how to search the files for max size (Does arc:/ not mount until after main is done?).
	for entry in WalkDir::new(&RANDOMIZE_PATH) {//Set things up to walk the vanilla path.
        let entry = entry.unwrap();
		
        if entry.path().is_dir() && format!("{}", &entry.path().display()).contains("."){
			
			let convert_to_vanilla = str::replace(&entry.path().display().to_string(),RANDOMIZE_PATH,"");
			
			
			//Mod file setup.
			
			
			let stupid = &str::replace(&entry.path().display().to_string(),RANDOMIZE_PATH,"");
			//hash = format!("mods:/0x{:x}",hash40(&stupid).as_u64());
			let mut hash = format!("mods:/{}",stupid);
			//str::replace(&hash,"arc:/","mods:/");//format!("mods:/{}",hash40(&convert_to_vanilla).as_u64());//str::replace(&entry.path().display().to_string(),RANDOMIZE_PATH,"");
			let mut path = Path::new(&hash);
			//mod file setup end.
			
			
			
			
			
			
			
			println!("{}",entry.path().display());
			if !path.exists()
			{
				//Vanilla file setup.
				hash = format!("arc:/0x{:x}",hash40(&convert_to_vanilla).as_u64());
				VANILLA_HOLDER.lock().unwrap().insert(i,convert_to_vanilla);
				i = i+1;
				path = Path::new(&hash);//VANILLA_HOLDER.lock().unwrap()[i].as_str());
				//path = Path::new(&VANILLA_HOLDER.lock().unwrap()[i].as_str());
				//end of vanilla setup.
				//continue;
			}
			else
			{
				
				VANILLA_HOLDER.lock().unwrap().insert(i,stupid.to_string());
				//let file = fs::read(VANILLA_HOLDER.lock().unwrap()[i]);
				
				
				//Todo: Remove this part, it's a workaround until 4.0.0 comes out.
				let dumbFileName = format!("mods:/{}",&VANILLA_HOLDER.lock().unwrap()[i]);
				let dumbFile = fs::read(&dumbFileName).unwrap();
				let dumbOutputFile = format!("{}{}","sd:/ultimate/ClassicRoutes/Added Routes (Temp)/",Path::new(&dumbFileName).file_name().unwrap().to_os_string().into_string().unwrap());
				println!("{}",&dumbOutputFile);
				
				//fs::File::create(&dumbOutputFile).expect("Couldn't create file");
				//fs::write("sd:/ultimate/ClassicRoutes/Mod Routes/standard_route_koopag.prc","").expect("Dummy write failed.");
				fs::create_dir_all("sd:/ultimate/ClassicRoutes/Added Routes (Temp)/");
				std::fs::write(&dumbOutputFile,dumbFile).expect("Couldn't copy files! Is the SD card full?");
				
				
				//End of Todo
				i = i+1;
			}
			println!("{}",hash);
			//println!("[ARClassic]{} {} {}",VANILLA_HOLDER.lock().unwrap()[i].as_str(),path.display(),size);
			if !path.exists()
			{
				println!("[ARClassic] ERROR! NoExist!{}{}",hash,path.display());
				continue;
			}
			
			let file = fs::read(path).unwrap();
			let size = file.len() as usize;
			
			
			
			if size > vanilla_max_size
			{
				vanilla_max_size = size;
				println!("[ARClassic]{}",size);
			}
			//let this_file = fs::read_dir(&hash);//Path::new(&hash);
			
		}
	}
	/*for entry in WalkDir::new("arc:/"){//This is gross but it's crashing if I try it up above by rawly reading the file. Post note: As you can see, this also crashed.
		let entry = entry.unwrap();
		if VANILLA_HOLDER.lock().unwrap().contains(&entry.path().display().to_string())
		{
			println!("{}",&entry.path().display().to_string());
			
			let size = entry.path().metadata().unwrap().len() as usize;
			if size > vanilla_max_size
			{
				vanilla_max_size = size;
				println!("[ARClassic]{}",size);
			}
		}
		
	}*/
	
	println!("[ARClassic]Max Size Compare: {}, {}", vanilla_max_size,max_size);
	if vanilla_max_size > max_size
	{
		max_size = vanilla_max_size;
		println!("[ARClassic]Vanilla size chosen");
	}
    for entry in WalkDir::new(&RANDOMIZE_PATH) {
        let entry = entry.unwrap();
        if entry.path().is_dir() && format!("{}", &entry.path().display()).contains("."){
            let path = &format!("{}", &entry.path().display())[RANDOMIZE_PATH.len()..];//.replace(";", ":").replace(".mp4", ".webm"); Last bits we don't need as we aren't using streaming.
            //println!("Adding {}",path);
            let hash = hash40(path);
			
			FILE_HOLDER.lock().unwrap().insert(hash.as_u64(), entry.path().to_path_buf());
			
			//let convert_to_vanilla = str::replace(&entry.path().display().to_string(),RANDOMIZE_PATH,"");
			//VANILLA_HOLDER.lock().unwrap().insert(i,convert_to_vanilla);
			
			arc_file_callback::install(hash, max_size);
			

        }
    }
	//println!("Done");
	return;
}
pub extern "C" fn readRoute(event: Event) {
	

}

#[skyline::main(name = "arclassic")]
pub fn main() {
    unsafe {
        arcrop_register_event_callback(Event::ModFilesystemMounted, init);
    }
}
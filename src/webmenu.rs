use std::fs;
use std::path::{Path, PathBuf};

//use image::DynamicImage;
use skyline_web::Webpage;
use ramhorns::{Template, Content};//This might not actually be needed idk yet as of writing this lol.
use percent_encoding::percent_decode_str;//Same with this though it *does* look more needed.

//use crate::minecraft_api::*;
use crate::keyboard::ShowKeyboardArg;
//use crate::modern_skin::convert_to_modern_skin;

const LOCALHOST: &str = "http://localhost/";

const ROUTE_DIR: &str = "rom:/ClassicModeSelector";//"sd:/atmosphere/contents/01006A800016E000/romfs/minecraft_skins";

const CACHE_VER: &str = "version=0";//because the switch apparently saves cache, use this to effectively make them useless during testing.


#[derive(Default)]
pub struct Skins {
    pub skins: Vec<String>,
//    skin_files: Vec<PathBuf>,
}

#[derive(Content)]
struct SkinIcon<'a> {
    path: &'a str,
    left: isize,
    top: isize,
    button_left: isize,
    button_top: isize,
}

#[derive(Content)]
struct Rendered<'a> {
    skins: Vec<SkinIcon<'a>>,
    add_left: isize,
    add_top: isize,
    add_button_left: isize,
    add_button_top: isize,
}

#[derive(Debug, Clone, PartialEq)]
enum Skin {
    Default,
    Custom(String),
    Random,
}


impl Skins {
	fn render(&self) -> Rendered {
        let mut skins = vec![];

        let mut i = 1;
		
        for skin in &self.skins {
            let (left, top) = index_to_image_x_y(i);
            let (button_left, button_top) = index_to_button_x_y(i);
            skins.push(SkinIcon {
                path: &skin,
                left,
                top,
                button_left,
                button_top,
            });

            i += 1;
        }

        let (add_left, add_top) = index_to_image_x_y(i);
        let (add_button_left, add_button_top) = index_to_button_x_y(i);

        Rendered { skins, add_top, add_left, add_button_left, add_button_top }
    }
	
	fn to_html(&self) -> String {
		let tpl = Template::new(include_str!("popup/index.html")).unwrap();
		tpl.render(&self.render())
	}

	fn show_menu(&self) -> Skin {
		let response = Webpage::new()
			.file("index.html", &self.to_html())
			//.file("steve.png", STEVE_PNG)
			//.file("plus_skin.png", &include_bytes!("popup/plus_skin.png")[..])
			/*.files(
				skin_files
					.iter()
					.zip(self.skins.iter())
					.filter_map(|(path, skin)| Some((&skin[..], fix_png(&path)?)))
					.collect::<Vec<(&str, Vec<u8>)>>()
			)*/
			.background(skyline_web::Background::BlurredScreenshot)
			.boot_display(skyline_web::BootDisplay::BlurredScreenshot)
			.open()
			.unwrap();

		match response.get_last_url().unwrap() {
			"http://localhost/default" => Skin::Default,
			"http://localhost/random" => Skin::Random,
			url if !url.starts_with(LOCALHOST) => Skin::Default,
			url => Skin::Custom(percent_decode_str(&url[LOCALHOST.len()..]).decode_utf8_lossy().into_owned())
		}
	}
	pub fn get_file_index(&mut self) -> /*Option<PathBuf>*/String {
        loop {
            match self.show_menu() {
				Skin::Default => return "*Default".to_string(),/*None,*/
				Skin::Custom(custom) => return custom.to_string(),/*Some(Path::new(ROUTE_DIR).join(custom))*/
				Skin::Random => return "*Random".to_string(),
			}
		}
	}

}

const BUTTONPERROW: &isize = &3;//&6;
const BUTTONSEPERATIONX: &isize = &425;
const BUTTONSEPERATIONY: &isize = &125;
const BUTTONSIZEX: &isize = &400;//&200;
const BUTTONSIZEY: &isize = &100;//&200;

fn index_to_button_x(i: isize) -> isize {
    (i % BUTTONPERROW) * BUTTONSEPERATIONX
}

fn index_to_button_y(i: isize) -> isize {
    (i / BUTTONPERROW) * BUTTONSEPERATIONY
}

fn index_to_button_x_y(i: isize) -> (isize, isize) {
	println!("{}{}",index_to_button_x(i).to_string(), index_to_button_y(i).to_string());
    (index_to_button_x(i), index_to_button_y(i))
}

fn index_to_image_x(i: isize) -> isize {
    ((i % BUTTONPERROW) * BUTTONSEPERATIONX) - BUTTONSIZEX
}

fn index_to_image_y(i: isize) -> isize {
    ((i / BUTTONPERROW) * BUTTONSEPERATIONY) - BUTTONSIZEY
}

fn index_to_image_x_y(i: isize) -> (isize, isize) {
    (index_to_image_x(i), index_to_image_y(i))
}
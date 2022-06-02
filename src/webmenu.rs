//use image::DynamicImage;
use skyline_web::Webpage;
use ramhorns::{Template, Content};//This might not actually be needed idk yet as of writing this lol.
use percent_encoding::percent_decode_str;//Same with this though it *does* look more needed.

//use crate::minecraft_api::*;
//use crate::modern_route::convert_to_modern_route;

const LOCALHOST: &str = "http://localhost/";

//const ROUTE_DIR: &str = "rom:/ClassicModeSelector";//"sd:/atmosphere/contents/01006A800016E000/romfs/minecraft_routes";

//const CACHE_VER: &str = "version=0";//because the switch apparently saves cache, use this to effectively make them useless during testing.


#[derive(Default)]
pub struct Routes {
    pub routes: Vec<String>,
//    route_files: Vec<PathBuf>,
}

#[derive(Content)]
struct RouteIcon<'a> {
    path: &'a str,
    button_left: isize,
    button_top: isize,
}

#[derive(Content)]
struct Rendered<'a> {
    routes: Vec<RouteIcon<'a>>,
    random_button_left: isize,
    random_button_top: isize,
}

#[derive(Debug, Clone, PartialEq)]
enum Route {
    Default,
	Random,
    Custom(String),
}


impl Routes {
	fn render(&self) -> Rendered {
        let mut routes = vec![];

        let mut i = 1;
		
        let (random_button_left, random_button_top) = index_to_button_x_y(i);
		
		i += 1;
		
        for route in &self.routes {
            let (button_left, button_top) = index_to_button_x_y(i);
            routes.push(RouteIcon {
                path: &route,
                button_left,
                button_top,
            });

            i += 1;
        }

        

        Rendered { routes, random_button_left, random_button_top }
    }
	
	fn to_html(&self) -> String {
		let tpl = Template::new(include_str!("popup/index.html")).unwrap();
		tpl.render(&self.render())
	}

	fn show_menu(&self) -> Route {
		let response = Webpage::new()
			.file("index.html", &self.to_html())
			//.file("steve.png", STEVE_PNG)
			//.file("plus_route.png", &include_bytes!("popup/plus_route.png")[..])
			/*.files(
				route_files
					.iter()
					.zip(self.routes.iter())
					.filter_map(|(path, route)| Some((&route[..], fix_png(&path)?)))
					.collect::<Vec<(&str, Vec<u8>)>>()
			)*/
			.background(skyline_web::Background::BlurredScreenshot)
			.boot_display(skyline_web::BootDisplay::BlurredScreenshot)
			.open()
			.unwrap();

		match response.get_last_url().unwrap() {
			"http://localhost/default" => Route::Default,
			"http://localhost/random" => Route::Random,
			url if !url.starts_with(LOCALHOST) => Route::Default,
			url => Route::Custom(percent_decode_str(&url[LOCALHOST.len()..]).decode_utf8_lossy().into_owned())
		}
	}
	pub fn get_file_index(&mut self) -> /*Option<PathBuf>*/String {
        loop {
            match self.show_menu() {
				Route::Default => return "*Default".to_string(),/*None,*/
				Route::Random => return "*Random".to_string(),
				Route::Custom(custom) => return custom.to_string(),/*Some(Path::new(ROUTE_DIR).join(custom))*/
				
			}
		}
	}

}

const BUTTONPERROW: &isize = &3;//&6;
const BUTTONSEPERATIONX: &isize = &425;
const BUTTONSEPERATIONY: &isize = &125;
const BORDEROFFSETX: &isize = &16;
const BORDEROFFSETY: &isize = &25;

fn index_to_button_x(i: isize) -> isize {
    (i % BUTTONPERROW) * BUTTONSEPERATIONX + (BORDEROFFSETX)
}

fn index_to_button_y(i: isize) -> isize {
    (i / BUTTONPERROW) * BUTTONSEPERATIONY + (BORDEROFFSETY)
}

fn index_to_button_x_y(i: isize) -> (isize, isize) {
	println!("{}{}",index_to_button_x(i).to_string(), index_to_button_y(i).to_string());
    (index_to_button_x(i), index_to_button_y(i))
}

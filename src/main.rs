#![feature(get_type_id)]

extern crate plygui;
extern crate plygui_image;
extern crate image;

use plygui::*;
use plygui_image::*;

use std::fs::*;
use std::io::BufReader;

fn create_image() -> Box<Control> {
	let img = image::load(BufReader::new(File::open("res/lulz.png").unwrap()), image::PNG).unwrap();
	
	let mut i = plygui_image::imp::Image::with_content(img);
	i.set_layout_width(layout::Size::MatchParent);
    i.set_layout_height(layout::Size::MatchParent);
    
	i.into_control()
}

fn main() {
    let mut application = plygui::imp::Application::with_name("Plygui test");

    let mut window = application.new_window("plygui!!", WindowStartSize::Exact(200, 200), WindowMenu::None);

    window.on_resize(Some(
        (|_: &mut Member, w: u16, h: u16| {
             println!("win resized to {}/{}", w, h);
         }).into(),
    ));

    window.set_child(Some(create_image()));
	
	application.start();
    
    println!("Exiting");
}

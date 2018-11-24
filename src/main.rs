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
	i.set_scale(plygui_image::ScalePolicy::CropCenter);
	i.set_layout_width(layout::Size::MatchParent);
    i.set_layout_height(layout::Size::MatchParent);
    
	i.into_control()
}
fn create_frame() -> Box<Control> {
	let mut frame = plygui::imp::Frame::with_label("Horizontal Frame");    
    
    frame.set_child(Some(create_image()));
	frame.set_layout_width(layout::Size::MatchParent);
    frame.set_layout_height(layout::Size::WrapContent);
    
	frame.into_control()
}

fn create_button(name: &str) -> Box<Control> {
	let mut button = plygui::imp::Button::with_label(name);
    button.set_layout_width(layout::Size::WrapContent);
    button.set_layout_height(layout::Size::WrapContent);
    button.into_control()
}

fn create_vertical_layout() -> Box<Control> {
	let mut vb = plygui::imp::LinearLayout::with_orientation(layout::Orientation::Vertical);
    vb.on_resize(Some(
        (|_: &mut Member, w: u16, h: u16| {
             println!("wb resized to {}/{}", w, h);
         }).into(),
    ));

	let mut button = plygui::imp::Button::with_label("Butt1");
    let butt1_id = button.id();
    //button.set_layout_params(layout::Params::WrapContent, layout::Params::MatchParent);
    button.on_click(Some(
        (|b: &mut Clickable| {
             let b = b.as_any_mut().downcast_mut::<plygui::imp::Button>().unwrap();
    	
        	 println!("button clicked: {}", b.label());
             b.set_visibility(Visibility::Gone);
             //b.set_visibility(Visibility::Invisible);
             
             let parent = b.is_control_mut().unwrap().parent_mut().unwrap().is_container_mut().unwrap().is_multi_mut().unwrap();
             
             if parent.len() < 3 {
             	parent.push_child(create_frame());
             	//parent.push_child(create_button("Buuu"));
             	//parent.push_child(create_vertical_layout());
             } else {
             	parent.pop_child();
             }
         }).into(),
    ));
    button.on_resize(Some(
        (|_: &mut Member, w: u16, h: u16| {
             println!("button resized too to {}/{}", w, h);
         }).into(),
    ));
    vb.push_child(button.into_control());

    let mut button = plygui::imp::Button::with_label("Butt2");
    //button.set_layout_params(layout::Params::WrapContent, layout::Params::MatchParent);
    button.on_click(Some(
        (move |b: &mut Clickable| {
            let b = b.as_any_mut().downcast_mut::<plygui::imp::Button>().unwrap();
    	
        	println!("button clicked: {} / {:?}", b.label(), b.as_control().id());
            {
            	let parent = b.parent().unwrap();
                let parent_member_id = parent.as_any().get_type_id();
                println!("parent is {:?}", parent_member_id);

                let parent: &Container = parent.is_container().unwrap();

                println!(
                    "clicked is {:?}",
                    parent
                        .find_control_by_id(b.id())
                        .unwrap()
                        .as_any()
                        .get_type_id()
                );
            }
            let root = b.root_mut().unwrap();
            let root_member_id = root.as_any().get_type_id();
            println!("root is {:?}", root_member_id);

            let root: &mut Container = root.is_container_mut().unwrap();

            let butt1 = root.find_control_by_id_mut(butt1_id).unwrap();
            butt1.set_visibility(Visibility::Visible);
        }).into(),
    ));
    button.on_resize(Some(
        (|_: &mut Member, w: u16, h: u16| {
             println!("button resized too to {}/{}", w, h);
         }).into(),
    ));
    vb.push_child(button.into_control());
    vb.push_child(create_image());
    vb.into_control()
}

fn main() {
    let mut application = plygui::imp::Application::with_name("Plygui test");

    let mut window = application.new_window("plygui!!", WindowStartSize::Exact(200, 200), WindowMenu::None);

    window.on_resize(Some(
        (|_: &mut Member, w: u16, h: u16| {
             println!("win resized to {}/{}", w, h);
         }).into(),
    ));

    window.set_child(Some(create_vertical_layout()));
	
	application.start();
    
    println!("Exiting");
}

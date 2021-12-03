extern crate num;
#[macro_use]
extern crate num_derive;

mod database;

use database::{Key, KeyboardEvent, Modifier, MouseEvent, EventDatabase, MouseButton, Coordinates};
use std::fs::File;
use std::io::{Read, Write};
//```rust
use fltk::{app, button, enums::Color, frame::Frame, image, prelude::*, window::Window};
use fltk::text::{TextBuffer,TextDisplay};
//use std::io;
use winput::{Input, Vk, Action, MouseMotion, Button};
use winput::message_loop;
use std::time::{SystemTime, Duration};



fn main() {
    let app = app::App::default().with_scheme(app::AppScheme::Gtk);

    let mut window = Window::new(0, 0, 400, 300, "Input Data");
        window.set_color(Color::White);

    let mut play_button = button::Button::default()
        .with_size(50,50)
        .with_label("Play")
        .with_pos(10, 10);

    let mut stop_button = button::Button::default()
        .with_size(50,50)
        .with_label("Stop")
        //.with_pos(50,50)
        .right_of(&play_button,10);

    let mut graph_button = button::Button::new(0, 0, 50, 50, "Graph")
        .right_of(&stop_button, 10);

    window.end();
    window.show();    
    
    let mut dispwindow = Window::new(0, 0, 400, 300, "Input Data");
        dispwindow.set_color(Color::White);

    let mut disp = TextDisplay::new(5,5,350,200,None);

    let mut buf = TextBuffer::default();


    disp.set_buffer(buf.clone());

    

      //play_button.set_callback(move |_| stop_button.set_color(Color::Red));
      play_button.clone().set_callback(move |_| play_button.set_label("Recording"));

      graph_button.set_callback( move |b|{
        dispwindow.end();
        dispwindow.show();
          let displayData =EventDatabase::load_database("database.db".to_string());
          let stringdata =  serde_json::to_string(&displayData).unwrap();
          let mut split =stringdata.split("},{");
          for s in split{
              buf.append(s);
              buf.append("\n");
          }
        //  dispwindow.end();
      //    dispwindow.show();
        //  dispwindow.make_current();
        
      }

      );

      app.run().unwrap();
      while app.wait() {
      }


    startListen();


}

fn startListen(){
    let mut event_database = EventDatabase::load_database("database.db".to_string());
    let receiver = message_loop::start().unwrap();
    let mut start_time_key = SystemTime::now();
    let mut start_time_mouse = SystemTime::now();
    let mut mouse_holding = false;
    let mut key_holding = false;
    loop {
        match receiver.next_event() {
            message_loop::Event::Keyboard {
                vk,
                action: Action::Release,
                ..
            } => {
                if vk == Vk::Escape {
                    break;
                } else {key_holding = false;
                    //Testing time
                    let new_time = SystemTime::now();
                    let difference = new_time.duration_since(start_time_key)
                    .expect("Clock may have gone backwards");
                    println!("Time was {:?}",difference);

                    event_database.add_keyboard_event(
                        KeyboardEvent{
                            key: Key::from(vk),
                            modifier: Modifier::Release,
                            event_time: difference
                        }
                    );
                    println!("{:?} was release!", vk);
                    //sys_time = SystemTime::now();
                    //Goes into database
                }
            },

            message_loop::Event::Keyboard {
                vk,
                action: Action::Press,
                ..
            } => {
                if vk == Vk::Escape {
                    break;
                } else {
                    if key_holding == false {
                        start_time_key = SystemTime::now();
                    }
                    key_holding = true;

                    let placeholder = Duration::new(0, 0);
                    event_database.add_keyboard_event(
                        KeyboardEvent{
                            // key: Key::new(vk),
                            key: Key::from(vk),
                            modifier: Modifier::Press,
                            event_time: placeholder
                        }
                    );
                    //let sys_time = SystemTime::now();
                    println!("{:?} was pressed!", vk);
                }
            },
            message_loop::Event::MouseButton{
                button,
                action: Action::Press,
            } => {
                if mouse_holding == false {
                start_time_mouse = SystemTime::now();
            }
            mouse_holding = true;
            println!("{:?} was clicked", button);

            let placeholder = Duration::new(0, 0);
            
            event_database.add_mouse_event(
                MouseEvent{
                    button: MouseButton::new(button),
                    modifier: Modifier::Press,
                    event_time: placeholder,
                    event_coordinate: Coordinates { x: 0.0, y: 0.0 }
                }
            );
                println!("{:?} was clicked", button);
            },

            message_loop::Event::MouseButton{
                button,
                action: Action::Release,
            } => {mouse_holding = false;
                println!("{:?} was clicked", button);

                //Testing time
                let new_time = SystemTime::now();
                let difference = new_time.duration_since(start_time_mouse)
                .expect("Clock may have gone backwards");
                println!("Time was {:?}",difference);

                event_database.add_mouse_event(
                    MouseEvent{
                        button: MouseButton::new(button),
                        modifier: Modifier::Release,
                        event_time: difference,
                        event_coordinate: Coordinates { x: 0.0, y: 0.0 }
                    }
                );
                println!("{:?} was released! Time was", button);
            },
            message_loop::Event::MouseMoveAbsolute{
                x,
                y,
                ..
            } => {

                let placeholder = Duration::new(0, 0);
                event_database.add_mouse_event(
                    MouseEvent{
                        button: MouseButton::Left,
                        modifier: Modifier::Press,
                        event_time: placeholder,
                        event_coordinate: Coordinates { x, y }
                    }
                );
                println!("Mouse is located at X:{:?} and Y: {:?}", x, y);
            }
            _=> (),

        }
        event_database.save_database("database.db".to_string());
    }
}//```
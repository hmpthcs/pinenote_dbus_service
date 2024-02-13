use input::{Libinput, LibinputInterface};
use libc::{O_RDONLY, O_RDWR, O_WRONLY};
use std::fs::{File, OpenOptions};
use std::os::unix::{fs::OpenOptionsExt, io::OwnedFd};
use std::path::Path;
use std::time::Duration;

use input::event::keyboard::KeyboardEventTrait;

use input::Event;
use input::Event::Tablet;
use input::event::tablet_tool::{TabletToolProximityEvent, TabletToolEventTrait};
use input::event::TabletToolEvent::Proximity;

use input::Event::Touch;
use input::event::TouchEvent;
use input::event::touch::TouchEventTrait;
use input::event::TouchEvent::{Down, Up};

mod sys_handler;

//use input::event::tablet_tool::TabletToolEventTrait;
//use input::tablet_tool::TabletToolProximityEvent;
//use input::event::tablet_tool::TabletTool;

struct Interface;

impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<OwnedFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .read(/*(flags & O_RDONLY != 0) |*/ (flags & O_RDWR != 0))
            .write((flags & O_WRONLY != 0) | (flags & O_RDWR != 0))
            .open(path)
            .map(|file| file.into())
            .map_err(|err| err.raw_os_error().unwrap())
    }
    fn close_restricted(&mut self, fd: OwnedFd) {
        drop(File::from(fd));
    }
}


fn handle_tablet_event(event:TabletToolProximityEvent){
    let proximity = event.proximity_state();
    match proximity {
         input::event::tablet_tool::ProximityState::In => {
//            println!("Proximity IN {:?}", proximity);
            sys_handler::set_bw_mode(2);
            sys_handler::set_default_waveform(2)
        }   
        input::event::tablet_tool::ProximityState::Out => {
//            println!("Proximity OUT {:?}", proximity);
            sys_handler::set_bw_mode(0);
            sys_handler::set_default_waveform(7)

        }
        _ => {}
    
    //    let down = proximity_state > 0.0;
    }
}

fn handle_touch_event(event:TouchEvent) {  
        match event {
//            TouchEvent::Down(_) => {
            TouchEvent::Motion(_) => {
//                println!("Touch was MOTION");
                sys_handler::set_bw_mode(2);
                sys_handler::set_default_waveform(2)
            }
            TouchEvent::Up(_) => {
//                TouchEvent::Frame(_) => {
//              println!("Touch was UP");
//                sys_handler::set_bw_mode(0);
                sys_handler::set_default_waveform(7);
                sys_handler::set_bw_mode(0)

            }
            _ => {}
        }
    }  

fn main() {
    let mut input = Libinput::new_with_udev(Interface);
    input.udev_assign_seat("seat0").unwrap();
    loop {
        input.dispatch().unwrap();
        for event in &mut input {
            match event {
                Event::Tablet(Proximity(event)) => {
                    handle_tablet_event(event);
                },
                Event::Touch(event) => {
                    handle_touch_event(event);
                },
                _ => {}
            }    
        }
        std::thread::sleep(Duration::from_millis(5));
    }
}
// //            if let Tablet(Proximity(event)) = event {
// //             handle_tablet_event(event);
// //               }
//            if let Touch(event) = event {
//                handle_touch_event(event);
//                }
//            }
//        }
//    }   
/*
fn handle_tablet_event(event:TabletToolProximityEvent){
    let proximity = event.proximity_state();
    match proximity {
         input::event::tablet_tool::ProximityState::In => {
            println!("Proximity IN {:?}", proximity);
        }   
        input::event::tablet_tool::ProximityState::Out => {
            println!("Proximity OUT {:?}", proximity);
        }
        _ => {}
    
    //    let down = proximity_state > 0.0;
    }
}

fn handle_touch_event(event:TouchEvent) {  
        match event {
            TouchEvent::Down(_) => {
                println!("Touch was DOWN");
                sys_handler::set_bw_mode(2);
                sys_handler::set_default_waveform(2)
            }
            TouchEvent::Up(_) => {
                println!("Touch was UP");
                sys_handler::set_bw_mode(0);
                sys_handler::set_default_waveform(7)
            }
            _ => {}
        }
    }  
*/
//        println!("\nTouchDown {:?}", touch);

//            match event {
//                input::Event::Tablet(proximity) => {
//                    println!("\nProximity {} {:?}\n",
//                    &proximity.proximity(),
//                    proximity.Proximity_state()
//                    )
//                },
//                input::Event::Keyboard(key) => {
//                    println!("\nKey {} {:?}\n",
//                    &key.key(),
//                    key.key_state()
//                    )
//                },
//                _ => println!("wtf")
//            }
//        }
//    }
//}

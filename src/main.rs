use std::error::Error;
use ash::{vk, prelude::*};
use winit::{window::WindowBuilder, event_loop::EventLoop, event::{Event, WindowEvent, KeyEvent, ElementState}, keyboard::{Key, NamedKey}, raw_window_handle::{HasWindowHandle, HasDisplayHandle}};
use std::ffi::CStr;

use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};


fn main() -> Result<(), Box<dyn Error>>{
    let window_width = 800;
    let window_height = 600;



    let entry = ash::Entry::linked();

    let event_loop = EventLoop::new()?;

    let display_handle = event_loop.raw_display_handle();

    let window = WindowBuilder::new()
        .with_title("vulcan ig")
        .with_resizable(false)
        .with_inner_size(winit::dpi::LogicalSize::new(
            f64::from(window_width),
            f64::from(window_height),
        ))
        .build(&event_loop)?;

    let window_handle = window.raw_window_handle();

    unsafe{

        let application_info = vk::ApplicationInfo::builder()
            .engine_name(CStr::from_bytes_with_nul_unchecked(b"No Engine\0"))
            .application_name(CStr::from_bytes_with_nul_unchecked(b"vulcan ig\0"))
            .api_version(vk::API_VERSION_1_0);

        let required_extensions = ash_window::enumerate_required_extensions(display_handle)?;

        vk::InstanceCreateInfo::builder()
            .application_info(&application_info);


    }





    event_loop.run(|e, w|{
        w.set_control_flow(winit::event_loop::ControlFlow::Poll);
        match e {
            Event::WindowEvent { 
                event: 
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput { 
                        event: KeyEvent { 
                            state: ElementState::Pressed, 
                            logical_key: Key::Named(NamedKey::Escape), 
                            .. 
                        },
                        .. 
                    },
                ..

            } => {
                w.exit();
            },
            _ => {}
        }
    })?;
    Ok(())
}

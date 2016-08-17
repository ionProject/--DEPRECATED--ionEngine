/*===============================================================================================*/
// Copyright 2016 Kyle Finlay
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
/*===============================================================================================*/

extern crate ion_core;
extern crate sdl2;

use self::ion_core::renderer::traits::WindowBackend;
use self::ion_core::renderer::window::{WindowConfig, WindowState};
use self::ion_core::util::traits::AsAny;
use self::sdl2::event::Event;

use std::any::Any;
use std::os::raw::c_void;

/*===============================================================================================*/
/*------WINDOW BACKEND SDL2 STRUCT---------------------------------------------------------------*/
/*===============================================================================================*/

/// Allows for the creation and handling of a SDL2 window.
pub struct WindowBackendSDL2 {

    // Public
    /// Stores the current window state.
    pub window_state:    WindowState,
    /// Stores the sdl2 context.
    pub sdl2_context:    Option<sdl2::Sdl>,
    /// Stores the video subsystem.
    pub sdl2_video:      Option<sdl2::VideoSubsystem>,
    /// Stores the sdl2 window.
    pub sdl2_window:     Option<sdl2::video::Window>,
    /// Stores the sdl2 event pump.
    pub sdl2_event_pump: Option<sdl2::EventPump>
}

/*===============================================================================================*/
/*------WINDOW BACKEND SDL2 PUBLIC METHODS-------------------------------------------------------*/
/*===============================================================================================*/

impl AsAny for WindowBackendSDL2 {

    fn as_any (&self) -> &Any {
        self
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl WindowBackend for WindowBackendSDL2 {

    fn init (&mut self, config: &WindowConfig) {

        // Initialize sdl and video subsystem
        let sdl2_context = sdl2::init ().unwrap ();
        let sdl2_video   = sdl2_context.video ().unwrap ();

        // Create new sdl2 window builder
        let mut sdl2_window_builder = sdl2_video.window (&config.window_title,
            config.window_size.x as u32,
            config.window_size.y as u32);

        // Enable OpenGL and set window position
        sdl2_window_builder.opengl ()
            .position (config.window_pos.x as i32, config.window_pos.y as i32);

        // Set fullscreen and resizable state
        if config.window_is_fullscreen {sdl2_window_builder.fullscreen ();}
        if config.window_is_resizable  {sdl2_window_builder.resizable  ();}

        // Create the sdl2 window and event pump
        let sdl2_window     = sdl2_window_builder.build ().unwrap ();
        let sdl2_event_pump = sdl2_context.event_pump   ().unwrap ();

        // Set window backend variables
        self.sdl2_context    = Some (sdl2_context);
        self.sdl2_video      = Some (sdl2_video);
        self.sdl2_event_pump = Some (sdl2_event_pump);
        self.sdl2_window     = Some (sdl2_window);
        self.window_state    = WindowState::Active;
    }

/*-----------------------------------------------------------------------------------------------*/

    fn get_window_state (&self) -> WindowState {
        self.window_state
    }

/*-----------------------------------------------------------------------------------------------*/

    fn on_pre_render (&mut self) {

        for event in self.sdl2_event_pump.as_mut ().unwrap ().poll_iter () {

            if let Event::Quit {..} = event {
                self.window_state = WindowState::Closed
            }
        }
    }

/*-----------------------------------------------------------------------------------------------*/

    fn on_render      (&mut self) {}
    fn on_post_render (&mut self) {}

/*-----------------------------------------------------------------------------------------------*/

    unsafe fn get_platform_display (&self) -> Result<*mut c_void, ()> {
        unimplemented! ()
    }

/*-----------------------------------------------------------------------------------------------*/

    unsafe fn get_platform_window (&self) -> Result<*mut c_void, ()> {
        unimplemented! ()
    }
}

/*===============================================================================================*/
/*------WINDOW BACKEND SDL2 PUBLIC STATIC METHODS------------------------------------------------*/
/*===============================================================================================*/

impl WindowBackendSDL2 {

    /// Returns a new `WindowBackendSDL2` instance.
    pub fn new () -> WindowBackendSDL2 {

        WindowBackendSDL2 {

            window_state:    WindowState::Closed,
            sdl2_context:    None,
            sdl2_video:      None,
            sdl2_event_pump: None,
            sdl2_window:     None,
        }
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Default for WindowBackendSDL2 {

    fn default () -> WindowBackendSDL2 {
        WindowBackendSDL2::new ()
    }
}

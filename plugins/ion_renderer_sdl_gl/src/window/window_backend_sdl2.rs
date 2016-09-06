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
use self::ion_core::renderer::window::WindowConfig;
use self::ion_core::util::traits::AsAny;
use self::ion_core::util::math::Vec2;
use self::sdl2::event::{Event, WindowEventId};

use std::any::Any;

/*===============================================================================================*/
/*------WINDOW BACKEND SDL2 STRUCT---------------------------------------------------------------*/
/*===============================================================================================*/

/// Allows for the creation and handling of a SDL2 window.
pub struct WindowBackendSDL2 {

    // Public
    /// Stores the sdl2 context.
    pub sdl2_context:    Option<sdl2::Sdl>,
    /// Stores the video subsystem.
    pub sdl2_video:      Option<sdl2::VideoSubsystem>,
    /// Stores the sdl2 window.
    pub sdl2_window:     Option<sdl2::video::Window>,
    /// Stores the sdl2 event pump.
    pub sdl2_event_pump: Option<sdl2::EventPump>,

    // Private
    _create_callback: Box<Fn ()>,
    _move_callback:   Box<Fn (Vec2)>,
    _resize_callback: Box<Fn (Vec2)>,
    _close_callback:  Box<Fn ()>,
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

        // Call the window creation callback
        (self._create_callback) ();
    }

/*-----------------------------------------------------------------------------------------------*/

    fn set_create_callback (&mut self, callback: Box<Fn ()>) {
        self._create_callback = callback;
    }

/*-----------------------------------------------------------------------------------------------*/

    fn set_move_callback (&mut self, callback: Box<Fn (Vec2)>) {
        self._move_callback = callback;
    }

/*-----------------------------------------------------------------------------------------------*/

    fn set_resize_callback (&mut self, callback: Box<Fn (Vec2)>) {
        self._resize_callback = callback;
    }

/*-----------------------------------------------------------------------------------------------*/

    fn set_close_callback (&mut self, callback: Box<Fn ()>) {
        self._close_callback = callback;
    }

/*-----------------------------------------------------------------------------------------------*/

    #[allow (unused_variables)]
    fn process_events (&mut self) {

        // Loop through all events
        for event in self.sdl2_event_pump.as_mut ().unwrap ().poll_iter () {

            if let Event::Window {timestamp, window_id, win_event_id, data1, data2} = event {

                match win_event_id {

                    // Window moved event
                    WindowEventId::Moved => {
                        (self._move_callback) (Vec2 {x: data1 as f32, y: data2 as f32})
                    }

                    // Window resize event
                    WindowEventId::Resized => {
                        (self._resize_callback) (Vec2 {x: data1 as f32, y: data2 as f32})
                    }

                    // Window closed event
                    WindowEventId::Close => {
                        (self._close_callback) ()
                    }

                    // Default
                    _ => {}
                }
            }
        }
    }
}

/*===============================================================================================*/
/*------WINDOW BACKEND SDL2 PUBLIC STATIC METHODS------------------------------------------------*/
/*===============================================================================================*/

impl WindowBackendSDL2 {

    /// Returns a new `WindowBackendSDL2` instance.
    pub fn new () -> WindowBackendSDL2 {

        WindowBackendSDL2 {

            sdl2_context:     None,
            sdl2_video:       None,
            sdl2_event_pump:  None,
            sdl2_window:      None,
            _create_callback: Box::new (||  {}),
            _move_callback:   Box::new (|_| {}),
            _resize_callback: Box::new (|_| {}),
            _close_callback:  Box::new (||  {}),
        }
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Default for WindowBackendSDL2 {

    fn default () -> WindowBackendSDL2 {
        WindowBackendSDL2::new ()
    }
}

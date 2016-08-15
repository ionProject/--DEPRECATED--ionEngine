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

    // Private
    _window_state:    WindowState,
    _sdl2_context:    Option<sdl2::Sdl>,
    _sdl2_video:      Option<sdl2::VideoSubsystem>,
    _sdl2_renderer:   Option<sdl2::render::Renderer<'static >>,
    _sdl2_event_pump: Option<sdl2::EventPump>
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

        let sdl2_context = sdl2::init ().unwrap ();
        let sdl2_video   = sdl2_context.video ().unwrap ();

        let mut sdl2_window = {

            if config.window_is_resizable {

                sdl2_video.window (&config.window_title,
                config.window_size.x as u32,
                config.window_size.y as u32)
                .resizable ()
                .build     ()
                .unwrap    ()
            }

            else {

                sdl2_video.window (&config.window_title,
                config.window_size.x as u32,
                config.window_size.y as u32)
                .build     ()
                .unwrap    ()
            }
        };

        if config.window_is_fullscreen {
            sdl2_window.set_fullscreen (sdl2::video::FullscreenType::True).unwrap ();
        }

        let sdl2_event_pump = sdl2_context.event_pump ().unwrap ();
        let sdl2_renderer   = sdl2_window.renderer ().build ().unwrap ();

        self._sdl2_context    = Some (sdl2_context);
        self._sdl2_video      = Some (sdl2_video);
        self._sdl2_event_pump = Some (sdl2_event_pump);
        self._sdl2_renderer   = Some (sdl2_renderer);
        self._window_state    = WindowState::Active;
    }

/*-----------------------------------------------------------------------------------------------*/

    fn get_window_state (&self) -> WindowState {
        self._window_state
    }

/*-----------------------------------------------------------------------------------------------*/

    fn on_pre_render (&mut self) {

        for event in self._sdl2_event_pump.as_mut ().unwrap ().poll_iter () {

            if let Event::Quit {..} = event {
                self._window_state = WindowState::Closed
            }
        }

        self._sdl2_renderer.as_mut ().unwrap ().clear   ();
        self._sdl2_renderer.as_mut ().unwrap ().present ();
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

            _window_state:    WindowState::Closed,
            _sdl2_context:    None,
            _sdl2_video:      None,
            _sdl2_event_pump: None,
            _sdl2_renderer:   None
        }
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Default for WindowBackendSDL2 {

    fn default () -> WindowBackendSDL2 {
        WindowBackendSDL2::new ()
    }
}

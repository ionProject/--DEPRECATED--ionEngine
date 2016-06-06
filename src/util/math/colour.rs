/*================================================================================================*/
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
/*================================================================================================*/

use ::util::math::Util;

/*================================================================================================*/
/*------COLOUR STRUCT-----------------------------------------------------------------------------*/
/*================================================================================================*/

/// The Colour struct
///
/// Used to represent RGBA colours.
/// 32-bit floats are used for the values.
#[derive (Copy, Clone, Default)]
pub struct Colour {

    // Public
    /// Red component
    pub r: f32,
    /// Green component
    pub g: f32,
    /// Blue component
    pub b: f32,
    /// Alpha component
    pub a: f32
}

/*================================================================================================*/
/*------COLOUR PUBLIC METHODS---------------------------------------------------------------------*/
/*================================================================================================*/

impl Colour {

    /// Formats the colour as a string.
    ///
    /// # Return value
    /// The colour formatted as a string.
    ///
    /// # Examples
    /// ```
    /// use ion_utils::math::Colour;
    ///
    /// let colour = Colour {r : 0.5, g : 1.0, b : 0.0, a : 1.0};
    /// println! ("Colour = {}", colour.to_string ());
    /// ```
    /// ```c
    /// Output : Colour = 0.5, 1.0, 0.0, 1.0
    pub fn to_string (&self) -> String {

        format! ("{}, {}, {}, {}", self.r, self.g, self.b, self.a)
    }

/*================================================================================================*/
/*------COLOUR PUBLIC STATIC METHODS--------------------------------------------------------------*/
/*================================================================================================*/

    /// Creates a colour with default values.
    ///
    /// # Return value
    /// A new colour instance.
    ///
    /// # Examples
    /// ```
    /// use ion_utils::math::Colour;
    ///
    /// let col = Colour::new (); // Resulting colour is white
    pub fn new () -> Colour {

        Colour {r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0}
    }

    /// Creates a new colour from an RGBA value.
    ///
    /// Each colour channel has a 0-255 range.
    ///
    /// # Arguments
    /// * `r` - Red colour channel.
    /// * `g` - Green colour channel.
    /// * `b` - Blue colour channel.
    /// * `a` - Aplha colour channel.
    ///
    /// # Return value
    /// A new colour instance.
    pub fn from_rgba (r: i32, g: i32, b: i32, a: i32) -> Colour {

        Colour {r: Util::clamp (r as f32, 0.0, 255.0) / 255.0,
                g: Util::clamp (g as f32, 0.0, 255.0) / 255.0,
                b: Util::clamp (b as f32, 0.0, 255.0) / 255.0,
                a: Util::clamp (a as f32, 0.0, 255.0) / 255.0}
    }

    /// Creates a new colour from a hexadecimal value.
    ///
    /// # Arguments
    /// * `hex_value` - A string containing a hex colour value.
    ///
    /// # Return value
    /// A new colour instance.
    ///
    /// # Examples
    /// ```
    /// use ion_utils::math::Colour;
    ///
    /// let col = Colour::from_hex ("20b2aaff");
    /// println! ("{}", col.to_string ());
    /// ```
    /// ```c
    /// Output : 0.1234902, 0.69803923, 0.6666667, 1
    pub fn from_hex (hex_value: &str) -> Colour {

        assert! (hex_value.len () == 8, "Hex colour value must be 8 characters long");

        Colour {r: (i32::from_str_radix (&hex_value [0..2], 16).unwrap () as f32) / 255.0,
                g: (i32::from_str_radix (&hex_value [2..4], 16).unwrap () as f32) / 255.0,
                b: (i32::from_str_radix (&hex_value [4..6], 16).unwrap () as f32) / 255.0,
                a: (i32::from_str_radix (&hex_value [6..8], 16).unwrap () as f32) / 255.0}
    }

    /// Creates a new colour from HSV (hue, saturation, value).
    ///
    /// # Arguments
    /// * `hue` - The HSV hue value.
    /// * `saturation` - The HSV saturation value.
    /// * `value` - The HSV colour value.
    ///
    /// # Return value
    /// A new colour instance.
    ///
    /// # Examples
    /// ```
    /// use ion_utils::math::Colour;
    ///
    /// let col = Colour::from_hsv (197, 43, 92);
    /// println! ("{}", col.to_string ());
    /// ```
    /// ```c
    /// Output : 0.5244, 0.80791336, 0.92, 1
    pub fn from_hsv (hue: i32, saturation: i32, value: i32) -> Colour {

        // Clamp all values to respective ranges
        let h_clamped = Util::clamp (hue as f32, 0.0, 360.0);
        let s_clamped = Util::clamp (saturation as f32, 0.0, 100.0) / 100.0;
        let v_clamped = Util::clamp (value as f32, 0.0, 100.0) / 100.0;

        // Set the chroma, Hdash, x, and min values
        let c = v_clamped * s_clamped;
        let h = h_clamped / 60.0;
        let x = c * (1.0 - (h % 2.0 - 1.0));
        let m = v_clamped - c;

        // Get the raw rgb values
        let rgb = match h {

            _ if h < 1.0 => (c, x, 0.0),
            _ if h < 2.0 => (x, c, 0.0),
            _ if h < 3.0 => (0.0, c, x),
            _ if h < 4.0 => (0.0, x, c),
            _ if h < 5.0 => (x, 0.0, c),
            _ if h < 6.0 => (c, 0.0, x),
            _ => (0.0, 0.0, 0.0)
        };

        // Return the final colour (rgb + min)
        Colour {r: rgb.0 + m,
                g: rgb.1 + m,
                b: rgb.2 + m,
                a: 1.0}
    }

    /// Outputs the colour as an RGBA value.
    ///
    /// It returns a tuple, containing four i32 values.
    ///
    /// # Arguments
    /// * `colour` - The colour to convert to a rgba value.
    ///
    /// # Return value
    /// A tuple containing the `r`, `g`, `b`, and `a` values.
    pub fn to_rgba (colour: &Colour) -> (i32, i32, i32, i32) {

        ((Util::clamp (colour.r * 255.0, 0.0, 255.0)) as i32,
         (Util::clamp (colour.g * 255.0, 0.0, 255.0)) as i32,
         (Util::clamp (colour.b * 255.0, 0.0, 255.0)) as i32,
         (Util::clamp (colour.a * 255.0, 0.0, 255.0)) as i32)
    }

    /// Outputs the colour as a hex value.
    ///
    /// # Arguments
    /// * `colour` - The colour to convert to a hex value.
    ///
    /// # Return value
    /// A String containing a hex colour value.
    ///
    /// # Examples
    /// ```
    /// use ion_utils::math::Colour;
    ///
    /// let col = Colour {r : 0.5, g : 0.2, b : 0.9, a : 1.0};
    /// println! ("{}", Colour::to_hex (&col));
    /// ```
    /// ```c
    /// Output : 7f33e5ff
    pub fn to_hex (colour: &Colour) -> String {

        format! ("{:x}{:x}{:x}{:x}", (colour.r * 255.0) as i32,
                                     (colour.g * 255.0) as i32,
                                     (colour.b * 255.0) as i32,
                                     (colour.a * 255.0) as i32)
    }

    /// Outputs the colour as hsv
    ///
    /// # Arguments
    /// * `colour` - The colour to convert to a hsv value
    ///
    /// # Return value
    /// A tuple containing a HSV colour.
    pub fn to_hsv (colour: &Colour) -> (i32, i32, i32) {

        // Get the min, max, and delta colour values
        let min = Util::min (Util::min (colour.r, colour.g), colour.b);
        let max = Util::max (Util::max (colour.r, colour.g), colour.b);
        let del = max - min;

        // Set initial hsv value
        let mut hsv = (0.0, (del / max) * 100.0, max * 100.0);

        // If delta is not 0, calc h value
        if del != 0.0 {

            if colour.r >= max {

                hsv.0 = (colour.g - colour.b) / del;

                if hsv.0 < 0.0 {
                    hsv.0 += 6.0;
                }
            }

            else if colour.g >= max {
                hsv.0 = ((colour.b - colour.r) / del) + 2.0;
            }

            else if colour.b >= max {
                hsv.0 = ((colour.r - colour.g) / del) + 4.0;
            }

            // Set final h value
            hsv.0 *= 60.0;
        }

        (hsv.0 as i32, hsv.1 as i32, hsv.2 as i32)
    }

    /// Returns the colour red (1.0, 0.0, 0.0, 1.0),
    ///
    /// # Return value
    /// A new colour instance representing red.
    pub fn red () -> Colour {

        Colour {r: 1.0,
                g: 0.0,
                b: 0.0,
                a: 1.0}
    }

    /// Returns the colour Green (0.0, 1.0, 0.0, 1.0).
    ///
    /// # Return value
    /// A new colour instance representing green.
    pub fn green () -> Colour {

        Colour {r: 0.0,
                g: 1.0,
                b: 0.0,
                a: 1.0}
    }

    /// Returns the colour Blue (0.0, 0.0, 1.0, 1.0).
    ///
    /// # Return value
    /// A new colour instance representing blue.
    pub fn blue () -> Colour {

        Colour {r: 0.0,
                g: 0.0,
                b: 1.0,
                a: 1.0}
    }

    /// Returns the colour Black (0.0, 0.0, 0.0, 1.0).
    ///
    /// # Return value
    /// A new colour instance representing black.
    pub fn black () -> Colour {

        Colour {r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0}
    }

    /// Returns the colour white (1.0, 1.0, 1.0, 1.0).
    ///
    /// # Return value
    /// A new colour instance representing white.
    pub fn white () -> Colour {

        Colour {r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0}
    }

    /// Returns the colour yellow (1.0, 1.0, 0.0, 1.0).
    ///
    /// # Return value
    /// A new colour instance representing yellow.
    pub fn yellow () -> Colour {

        Colour {r: 1.0,
                g: 1.0,
                b: 0.0,
                a: 1.0}
    }

    /// Returns the colour magenta (1.0, 0.0, 1.0, 1.0).
    ///
    /// # Return value
    /// A new colour instance representing magenta.
    pub fn magenta () -> Colour {

        Colour {r: 1.0,
                g: 0.0,
                b: 1.0,
                a: 1.0}
    }

    /// Returns the colour cyan (0.0, 1.0, 1.0, 1.0).
    ///
    /// # Return value
    /// A new colour instance representing cyan.
    pub fn cyan () -> Colour {

        Colour {r: 0.0,
                g: 1.0,
                b: 1.0,
                a: 1.0}
    }

    /// Returns the colour light grey (0.75, 0.75, 0.75, 1.0).
    ///
    /// # Return value
    /// A new colour instance representing light grey.
    pub fn light_grey () -> Colour {

        Colour {r: 0.75,
                g: 0.75,
                b: 0.75,
                a: 1.0}
    }

    /// Returns the colour grey (0.5, 0.5, 0.5, 1.0).
    ///
    /// # Return value
    /// A new colour instance representing grey.
    pub fn grey () -> Colour {

        Colour {r: 0.5,
                g: 0.5,
                b: 0.5,
                a: 1.0}
    }

    /// Returns the colour dark grey.
    ///
    /// # Return value
    /// A new colour instance representing dark grey.
    pub fn dark_grey () -> Colour {

        Colour {r: 0.25,
                g: 0.25,
                b: 0.25,
                a: 1.0}
    }

    /// Returns a completely transparent colour (0, 0, 0, 0).
    ///
    /// # Return value
    /// A new colour instance representing pure transparency.
    pub fn clear () -> Colour {

        Colour {r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 1.0}
    }

    // TODO: Add example to documentation
    /// Linearly interpolates between two colours.
    ///
    /// # Arguments
    /// * `start` - The starting colour.
    /// * `end` - The end colour.
    /// * `percentage` - How far between `start` and `end` the resulting colour will be.
    ///
    /// # Return value
    /// A new colour instance.
    pub fn lerp (start: &Colour, end: &Colour, percentage: f32) -> Colour {

        Colour {r: Util::lerp (start.r, end.r, percentage),
                g: Util::lerp (start.g, end.g, percentage),
                b: Util::lerp (start.b, end.b, percentage),
                a: Util::lerp (start.a, end.a, percentage)}
    }

    /// Linearly interpolates between two colours without clamping.
    ///
    /// # Arguments
    /// * `start` - The starting colour.
    /// * `end` - The end colour.
    /// * `percentage` - How far between `start` and `end` the resulting colour will be.
    ///
    /// # Return value
    /// A new colour instance.
    pub fn lerp_unclamped (start: &Colour, end: &Colour, percentage: f32) -> Colour {

        Colour {r: Util::lerp_unclamped (start.r, end.r, percentage),
                g: Util::lerp_unclamped (start.g, end.g, percentage),
                b: Util::lerp_unclamped (start.b, end.b, percentage),
                a: Util::lerp_unclamped (start.a, end.a, percentage)}
    }
}

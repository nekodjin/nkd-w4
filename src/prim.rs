use core::fmt;
use core::marker::PhantomData;
use core::mem;
use core::ops;

use spin::Mutex;

use nkd_w4_prim as w4;

pub const SCREEN_SIZE: u32 = w4::SCREEN_SIZE;

pub struct Palette(PhantomData<()>);

pub static PALETTE: Mutex<Palette> = Mutex::new(Palette(PhantomData));

#[doc(alias = "PaletteColour")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaletteColor {
    #[doc(alias = "Colour1")]
    Color1,
    #[doc(alias = "Colour2")]
    Color2,
    #[doc(alias = "Colour3")]
    Color3,
    #[doc(alias = "Colour4")]
    Color4,
}

impl PaletteColor {
    fn as_palette_index(self) -> usize {
        use PaletteColor::*;

        match self {
            Color1 => 0,
            Color2 => 1,
            Color3 => 2,
            Color4 => 3,
        }
    }
}

impl fmt::Debug for Palette {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use PaletteColor::*;

        f.debug_struct("Palette")
            .field("Color1", &self[Color1])
            .field("Color2", &self[Color2])
            .field("Color3", &self[Color3])
            .field("Color4", &self[Color4])
            .finish()
    }
}

impl ops::Index<PaletteColor> for Palette {
    type Output = Color;

    fn index(&self, c: PaletteColor) -> &Color {
        let index = c.as_palette_index();

        // A `Palette` instance can only be obtained by locking the PALETTE
        // static mutex; therefore, this Palette instance is unique, meaning
        // it is safe to assume that the aliasing state of self is identical
        // to that of the primitive palette binding.
        let palette = &unsafe { *w4::PALETTE };
        let color = &palette[index];

        // `Color` is representationally transparent with u32, and maintains
        // no invariants on the value of the u32 therein, making this reference
        // transmutation safe.
        unsafe { mem::transmute(color) }
    }
}

impl ops::IndexMut<PaletteColor> for Palette {
    fn index_mut(&mut self, c: PaletteColor) -> &mut Color {
        let index = c.as_palette_index();

        // A `Palette` instance can only be obtained by locking the PALETTE
        // static mutex; therefore, this Palette instance is unique, meaning
        // it is safe to assume that the aliasing state of self is identical
        // to that of the primitive palette binding.
        let palette = &mut unsafe { *w4::PALETTE };
        let color = &mut palette[index];

        // `Color` is representationally transparent with u32, and maintains
        // no invariants on the value of the u32 therein, making this reference
        // transmutation safe.
        unsafe { mem::transmute(color) }
    }
}

#[doc(alias = "Colour")]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Color(u32);

impl Color {
    pub const fn rgb(red: u8, green: u8, blue: u8) -> Self {
        let red = red as u32;
        let green = green as u32;
        let blue = blue as u32;

        let red = red << 16;
        let green = green << 8;
        let blue = blue << 0;

        let color = red | green | blue;

        Color(color)
    }

    #[doc(alias = "greyscale")]
    pub const fn grayscale(value: u8) -> Self {
        Self::rgb(value, value, value)
    }

    pub const fn with_red(mut self, red: u8) -> Self {
        let red = red as u32;

        self.0 &= 0xff00ffff;
        self.0 |= red << 16;

        self
    }

    pub const fn with_green(mut self, green: u8) -> Self {
        let green = green as u32;

        self.0 &= 0xffff00ff;
        self.0 |= green << 8;

        self
    }

    pub const fn with_blue(mut self, blue: u8) -> Self {
        let blue = blue as u32;

        self.0 &= 0xffffff00;
        self.0 |= blue << 0;

        self
    }

    pub const BLACK: Color = Color::rgb(0, 0, 0);
    pub const SILVER: Color = Color::rgb(192, 192, 192);
    #[doc(alias = "GREY")]
    pub const GRAY: Color = Color::rgb(128, 128, 128);
    pub const WHITE: Color = Color::rgb(255, 255, 255);
    pub const MAROON: Color = Color::rgb(128, 0, 0);
    pub const RED: Color = Color::rgb(255, 0, 0);
    pub const PURPLE: Color = Color::rgb(128, 0, 128);
    pub const FUCHSIA: Color = Color::rgb(255, 0, 255);
    pub const GREEN: Color = Color::rgb(0, 128, 0);
    pub const LIME: Color = Color::rgb(0, 255, 0);
    pub const OLIVE: Color = Color::rgb(128, 128, 0);
    pub const YELLOW: Color = Color::rgb(255, 255, 0);
    pub const NAVY: Color = Color::rgb(0, 0, 128);
    pub const BLUE: Color = Color::rgb(0, 0, 255);
    pub const TEAL: Color = Color::rgb(0, 128, 128);
    pub const AQUA: Color = Color::rgb(0, 255, 255);
    pub const ALICE_BLUE: Color = Color::rgb(240, 248, 255);
    pub const ANTIQUE_WHITE: Color = Color::rgb(250, 235, 215);
    pub const AQUAMARINE: Color = Color::rgb(127, 255, 212);
    pub const AZURE: Color = Color::rgb(240, 255, 255);
    pub const BEIGE: Color = Color::rgb(245, 245, 220);
    pub const BISQUE: Color = Color::rgb(255, 228, 196);
    pub const BLANCHED_ALMOND: Color = Color::rgb(255, 235, 205);
    pub const BLUE_VIOLET: Color = Color::rgb(128, 43, 226);
    pub const BROWN: Color = Color::rgb(165, 42, 42);
    pub const BURLYWOOD: Color = Color::rgb(222, 184, 135);
    pub const CADET_BLUE: Color = Color::rgb(95, 158, 160);
    pub const CHARTREUSE: Color = Color::rgb(127, 255, 0);
    pub const CHOCOLATE: Color = Color::rgb(210, 105, 30);
    pub const CORAL: Color = Color::rgb(255, 127, 80);
    pub const CORNFLOWER_BLUE: Color = Color::rgb(100, 149, 237);
    pub const CORNSILK: Color = Color::rgb(255, 248, 220);
    pub const CRIMSON: Color = Color::rgb(220, 20, 60);
    pub const DARK_CYAN: Color = Color::rgb(0, 139, 139);
    pub const DARK_GOLDENROD: Color = Color::rgb(184, 134, 11);
    #[doc(alias = "DARK_GREY")]
    pub const DARK_GRAY: Color = Color::rgb(169, 169, 169);
    pub const DARK_GREEN: Color = Color::rgb(0, 100, 0);
    pub const DARK_KHAKI: Color = Color::rgb(189, 183, 107);
    pub const DARK_MAGENTA: Color = Color::rgb(139, 0, 139);
    pub const DARK_OLIVE_GREEN: Color = Color::rgb(85, 107, 47);
    pub const DARK_ORANGE: Color = Color::rgb(255, 140, 0);
    pub const DARK_ORCHID: Color = Color::rgb(153, 50, 204);
    pub const DARK_RED: Color = Color::rgb(139, 0, 0);
    pub const DARK_SALMON: Color = Color::rgb(233, 150, 122);
    pub const DARK_SEA_GREEN: Color = Color::rgb(143, 188, 143);
    pub const DARK_SLATE_BLUE: Color = Color::rgb(72, 61, 139);
    pub const DARK_SLATE_GRAY: Color = Color::rgb(47, 79, 79);
    pub const DARK_TURQUOISE: Color = Color::rgb(0, 206, 209);
    pub const DARK_VIOLET: Color = Color::rgb(148, 0, 211);
    pub const DEEP_PINK: Color = Color::rgb(255, 20, 147);
    pub const DEEP_SKY_BLUE: Color = Color::rgb(0, 191, 255);
    #[doc(alias = "DIM_GREY")]
    pub const DIM_GRAY: Color = Color::rgb(105, 105, 105);
    pub const DODGER_BLUE: Color = Color::rgb(30, 144, 255);
    pub const FIREBRICK: Color = Color::rgb(178, 34, 34);
    pub const FLORAL_WHITE: Color = Color::rgb(255, 250, 240);
    pub const FOREST_GREEN: Color = Color::rgb(34, 139, 34);
    pub const GAINSBORO: Color = Color::rgb(220, 220, 220);
    pub const GHOST_WHITE: Color = Color::rgb(248, 248, 255);
    pub const GOLD: Color = Color::rgb(255, 215, 0);
    pub const GOLDENROD: Color = Color::rgb(218, 165, 32);
    pub const GREEN_YELLOW: Color = Color::rgb(173, 255, 47);
    pub const HONEYDEW: Color = Color::rgb(240, 255, 240);
    pub const HOT_PINK: Color = Color::rgb(255, 105, 180);
    pub const INDIAN_RED: Color = Color::rgb(205, 92, 92);
    pub const INDIGO: Color = Color::rgb(75, 0, 130);
    pub const IVORY: Color = Color::rgb(255, 255, 240);
    pub const KHAKI: Color = Color::rgb(240, 230, 140);
    pub const LAVENDER: Color = Color::rgb(230, 230, 250);
    pub const LAVENDER_BUSH: Color = Color::rgb(255, 240, 245);
    pub const LAWN_GREEN: Color = Color::rgb(124, 252, 0);
    pub const LEMON_CHIFFON: Color = Color::rgb(255, 250, 205);
    pub const LIGHT_BLUE: Color = Color::rgb(173, 216, 230);
    pub const LIGHT_CORAL: Color = Color::rgb(240, 128, 128);
    pub const LIGHT_CYAN: Color = Color::rgb(224, 255, 255);
    pub const LIGHT_GOLDENROD_YELLOW: Color = Color::rgb(250, 250, 210);
    #[doc(alias = "LIGHT_GREY")]
    pub const LIGHT_GRAY: Color = Color::rgb(211, 211, 211);
    pub const LIGHT_GREEN: Color = Color::rgb(144, 238, 144);
    pub const LIGHT_PINK: Color = Color::rgb(255, 182, 193);
    pub const LIGHT_SALMON: Color = Color::rgb(255, 160, 122);
    pub const LIGHT_SEA_GREEN: Color = Color::rgb(32, 178, 170);
    pub const LIGHT_SKY_BLUE: Color = Color::rgb(135, 206, 250);
    #[doc(alias = "LIGHT_SLATE_GREY")]
    pub const LIGHT_SLATE_GRAY: Color = Color::rgb(119, 136, 153);
    pub const LIGHT_STEEL_BLUE: Color = Color::rgb(176, 196, 222);
    pub const LIGHT_YELLOW: Color = Color::rgb(255, 255, 224);
    pub const LIME_GREEN: Color = Color::rgb(50, 205, 50);
    pub const LINEN: Color = Color::rgb(250, 240, 230);
    pub const MEDIUM_AQUAMARINE: Color = Color::rgb(102, 205, 170);
    pub const MEDIUM_BLUE: Color = Color::rgb(0, 0, 205);
    pub const MEDIUM_ORCHID: Color = Color::rgb(186, 85, 211);
    pub const MEDIUM_PURPLE: Color = Color::rgb(147, 112, 219);
    pub const MEDIUM_SEA_GREEN: Color = Color::rgb(60, 179, 113);
    pub const MEDIUM_SLATE_BLUE: Color = Color::rgb(123, 104, 238);
    pub const MEDIUM_SPRING_GREEN: Color = Color::rgb(0, 250, 154);
    pub const MEDIUM_TURQUOISE: Color = Color::rgb(72, 209, 204);
    pub const MEDIUM_VIOLET_RED: Color = Color::rgb(199, 21, 133);
    pub const MEDIUM_NIGHT_BLUE: Color = Color::rgb(25, 25, 112);
    pub const MINT_CREAM: Color = Color::rgb(245, 255, 250);
    pub const MISTY_ROSE: Color = Color::rgb(255, 228, 225);
    pub const MOCCASIN: Color = Color::rgb(255, 228, 181);
    pub const NAVAJO_WHITE: Color = Color::rgb(255, 222, 173);
    pub const OLD_LACE: Color = Color::rgb(253, 245, 230);
    pub const OLIVE_DRAB: Color = Color::rgb(107, 142, 35);
    pub const ORANGE: Color = Color::rgb(255, 165, 0);
    pub const ORANGE_RED: Color = Color::rgb(255, 69, 0);
    pub const ORCHID: Color = Color::rgb(218, 112, 214);
    pub const PALE_GOLDENROD: Color = Color::rgb(238, 232, 170);
    pub const PALE_GREEN: Color = Color::rgb(152, 251, 152);
    pub const PALE_TURQUOISE: Color = Color::rgb(175, 238, 238);
    pub const PALE_VIOLET_RED: Color = Color::rgb(219, 112, 147);
    pub const PAPAYA_WHIP: Color = Color::rgb(255, 239, 213);
    pub const PEACH_PUFF: Color = Color::rgb(255, 218, 185);
    pub const PERU: Color = Color::rgb(205, 133, 63);
    pub const PINK: Color = Color::rgb(255, 192, 203);
    pub const PLUM: Color = Color::rgb(221, 160, 221);
    pub const POWDER_BLUE: Color = Color::rgb(176, 224, 230);
    pub const ROSY_BROWN: Color = Color::rgb(188, 143, 143);
    pub const ROYAL_BLUE: Color = Color::rgb(65, 105, 225);
    pub const SADDLE_BROWN: Color = Color::rgb(139, 69, 19);
    pub const SALMON: Color = Color::rgb(250, 128, 114);
    pub const SANDY_BROWN: Color = Color::rgb(244, 164, 96);
    pub const SEA_GREEN: Color = Color::rgb(46, 139, 87);
    pub const SEASHELL: Color = Color::rgb(255, 245, 238);
    pub const SIENNA: Color = Color::rgb(160, 82, 45);
    pub const SKY_BLUE: Color = Color::rgb(135, 206, 235);
    pub const SLATE_BLUE: Color = Color::rgb(106, 90, 205);
    #[doc(alias = "SLATE_GREY")]
    pub const SLATE_GRAY: Color = Color::rgb(112, 128, 144);
    pub const SNOW: Color = Color::rgb(255, 250, 250);
    pub const SPRING_GREEN: Color = Color::rgb(0, 255, 127);
    pub const STEEL_BLUE: Color = Color::rgb(70, 130, 180);
    pub const TAN: Color = Color::rgb(210, 180, 140);
    pub const THISTLE: Color = Color::rgb(216, 191, 216);
    pub const TOMATO: Color = Color::rgb(255, 99, 71);
    pub const VIOLET: Color = Color::rgb(238, 130, 238);
    pub const WHEAT: Color = Color::rgb(245, 222, 179);
    pub const WHITE_SMOKE: Color = Color::rgb(245, 245, 245);
    pub const YELLOW_GREEN: Color = Color::rgb(154, 205, 50);
}

impl Default for Color {
    fn default() -> Self {
        Color::BLACK
    }
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use ColorChannel::*;

        f.debug_struct("Color")
            .field("R", &self[Red])
            .field("G", &self[Green])
            .field("B", &self[Blue])
            .finish()
    }
}

impl ops::Index<ColorChannel> for Color {
    type Output = u8;

    fn index(&self, channel: ColorChannel) -> &u8 {
        #[cfg(target_endian = "little")]
        // must never exceed 3
        let index = match channel {
            ColorChannel::Red => 2,
            ColorChannel::Green => 1,
            ColorChannel::Blue => 0,
        };
        #[cfg(target_endian = "big")]
        // must never exceed 3
        let index = match channel {
            ColorChannel::Red => 1,
            ColorChannel::Green => 2,
            ColorChannel::Blue => 3,
        };

        // `Self` is representationally transparent with u32. pointer cast
        // reinterpretation of u32 as [u8; 4] is valid, and reinterpretation
        // as a non-padded lower-aligned type is valid. `index` will never
        // exceed 3, so the `.add` invocation remains within bounds.
        unsafe { &*(self as *const _ as *const u8).add(index) }
    }
}

impl ops::IndexMut<ColorChannel> for Color {
    fn index_mut(&mut self, channel: ColorChannel) -> &mut u8 {
        #[cfg(target_endian = "little")]
        // must never exceed 3
        let index = match channel {
            ColorChannel::Red => 2,
            ColorChannel::Green => 1,
            ColorChannel::Blue => 0,
        };
        #[cfg(target_endian = "big")]
        // must never exceed 3
        let index = match channel {
            ColorChannel::Red => 1,
            ColorChannel::Green => 2,
            ColorChannel::Blue => 3,
        };

        // `Self` is representationally transparent with u32. pointer cast
        // reinterpretation of u32 as [u8; 4] is valid, and reinterpretation
        // as a non-padded lower-aligned type is valid. `index` will never
        // exceed 3, so the `.add` invocation remains within bounds.
        unsafe { &mut *(self as *mut _ as *mut u8).add(index) }
    }
}

impl From<u32> for Color {
    fn from(source: u32) -> Color {
        Color(source)
    }
}

impl From<Color> for u32 {
    fn from(source: Color) -> u32 {
        source.0
    }
}

#[doc(alias = "ColourChannel")]
pub enum ColorChannel {
    Red,
    Green,
    Blue,
}

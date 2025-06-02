use ratzilla::ratatui::style::Color;

pub const CATPPUCCIN: Catppuccin = Catppuccin::new();

#[allow(unused)]
pub struct Catppuccin {
    pub rosewater: Color,
    pub flamingo: Color,
    pub pink: Color,
    pub mauve: Color,
    pub red: Color,
    pub maroon: Color,
    pub peach: Color,
    pub yellow: Color,
    pub green: Color,
    pub teal: Color,
    pub sky: Color,
    pub sapphire: Color,
    pub blue: Color,
    pub lavender: Color,
    pub text: Color,
    pub subtext1: Color,
    pub subtext0: Color,
    pub overlay2: Color,
    pub overlay1: Color,
    pub overlay0: Color,
    pub surface2: Color,
    pub surface1: Color,
    pub surface0: Color,
    pub base: Color,
    pub mantle: Color,
    pub crust: Color,
}

impl Catppuccin {
    pub const fn new() -> Self {
        Self {
            rosewater: Color::from_u32(0xf2d5cf),
            flamingo: Color::from_u32(0xeebebe),
            pink: Color::from_u32(0xf4b8e4),
            mauve: Color::from_u32(0xca9ee6),
            red: Color::from_u32(0xe78284),
            maroon: Color::from_u32(0xea999c),
            peach: Color::from_u32(0xef9f76),
            yellow: Color::from_u32(0xe5c890),
            green: Color::from_u32(0xa6d189),
            teal: Color::from_u32(0x81c8be),
            sky: Color::from_u32(0x99d1db),
            sapphire: Color::from_u32(0x85c1dc),
            blue: Color::from_u32(0x8caaee),
            lavender: Color::from_u32(0xbabbf1),
            text: Color::from_u32(0xc6d0f5),
            subtext1: Color::from_u32(0xb5bfe2),
            subtext0: Color::from_u32(0xa5adce),
            overlay2: Color::from_u32(0x949cbb),
            overlay1: Color::from_u32(0x838ba7),
            overlay0: Color::from_u32(0x737994),
            surface2: Color::from_u32(0x626880),
            surface1: Color::from_u32(0x51576d),
            surface0: Color::from_u32(0x414559),
            base: Color::from_u32(0x303446),
            mantle: Color::from_u32(0x292c3c),
            crust: Color::from_u32(0x232634),
        }
    }
}

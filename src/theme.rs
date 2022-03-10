//default theme layout for the news website, mainly color scheme
#[allow(unused_imports)]
use crossterm::style::{Color::*, Attribute::*};
#[allow(unused_imports)]
use termimad::{rgb, MadSkin, StyledChar};

pub fn default() -> MadSkin {
    let mut skin = MadSkin::default();
    skin.bold.set_fg(Grey);
    skin.italic.set_bg(Rgb {
        r: 44,
        g: 44,
        b: 44,
    });
    skin.bullet = StyledChar::from_fg_char(Yellow, '♣');
    skin.set_headers_fg(Rgb {
        r: 89,
        g: 141,
        b: 212,
    });
    skin.quote_mark = StyledChar::from_fg_char(Yellow, '♤');
    skin.quote_mark.set_fg(Rgb {
        r: 89,
        g: 141,
        b: 212,
    });
    skin.inline_code.set_fg(Rgb {
        r: 33,
        g: 47,
        b: 82,
    });
    skin.italic.set_fg(Rgb {
        r: 89,
        g: 141,
        b: 212,
    });
    skin
}
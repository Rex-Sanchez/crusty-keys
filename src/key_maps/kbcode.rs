use x11_dl::{keysym, xlib};

#[derive(Debug)]
pub enum KbModifier {
    Alt,
    Ctrl,
    Super,
    Shift,
}
impl KbModifier {
    pub fn to_code(&self) -> u32 {
        match self {
            KbModifier::Alt => xlib::Mod1Mask,
            KbModifier::Ctrl => xlib::ControlMask,
            KbModifier::Super => xlib::Mod4Mask,
            KbModifier::Shift => xlib::ShiftMask,
        }
    }
}

pub enum KbCode {
    Enter,
    Space,
    Backspace,
    Esc,

    End,
    Home,
    Insert,
    Del,

    Down,
    Up,
    Left,
    Right,

    PgUp,
    PgDown,

    Tab,

    Minus,
    Equals,

    Quote,
    Backtick,
    Backslash,
    Slash,
    Semicolon,

    Comma,
    Period,

    F(u8),
    Char(char),
}

impl KbCode {
    pub fn to_code(&self) -> u32 {
        match self {
            KbCode::Enter => keysym::XK_Return,
            KbCode::Space => keysym::XK_space,
            KbCode::Backspace => keysym::XK_BackSpace,
            KbCode::Esc => keysym::XK_Escape,
            KbCode::End => keysym::XK_End,
            KbCode::Home => keysym::XK_Home,
            KbCode::Insert => keysym::XK_Insert,
            KbCode::Del => keysym::XK_Delete,
            KbCode::Down => keysym::XK_Down,
            KbCode::Up => keysym::XK_Up,
            KbCode::Left => keysym::XK_Left,
            KbCode::Right => keysym::XK_Right,
            KbCode::PgUp => keysym::XK_Page_Up,
            KbCode::PgDown => keysym::XK_Page_Down,
            KbCode::Tab => keysym::XK_Tab,
            KbCode::Minus => keysym::XK_minus,
            KbCode::Equals => keysym::XK_equal,
            KbCode::Quote => keysym::XK_quoteleft,
            KbCode::Backtick => keysym::XK_quoteright,
            KbCode::Backslash => keysym::XK_backslash,
            KbCode::Slash => keysym::XK_slash,
            KbCode::Semicolon => keysym::XK_semicolon,
            KbCode::Comma => keysym::XK_comma,
            KbCode::Period => keysym::XK_period,
            KbCode::F(f) => match f {
                1 => keysym::XK_F1,
                2 => keysym::XK_F2,
                3 => keysym::XK_F3,
                4 => keysym::XK_F4,
                5 => keysym::XK_F5,
                6 => keysym::XK_F6,
                7 => keysym::XK_F7,
                8 => keysym::XK_F8,
                9 => keysym::XK_F9,
                10 => keysym::XK_F10,
                11 => keysym::XK_F11,
                12 => keysym::XK_F12,
                _ => unreachable!(),
            },
            KbCode::Char(c) => match c {
                'a' => keysym::XK_a,
                'b' => keysym::XK_b,
                'c' => keysym::XK_c,
                'd' => keysym::XK_d,
                'e' => keysym::XK_e,
                'f' => keysym::XK_f,
                'g' => keysym::XK_g,
                'h' => keysym::XK_h,
                'i' => keysym::XK_i,
                'j' => keysym::XK_j,
                'k' => keysym::XK_k,
                'l' => keysym::XK_l,
                'n' => keysym::XK_n,
                'o' => keysym::XK_o,
                'p' => keysym::XK_p,
                'q' => keysym::XK_q,
                'r' => keysym::XK_r,
                's' => keysym::XK_s,
                't' => keysym::XK_t,
                'u' => keysym::XK_u,
                'v' => keysym::XK_v,
                'w' => keysym::XK_w,
                'x' => keysym::XK_x,
                'y' => keysym::XK_y,
                'z' => keysym::XK_x,
                '1' => keysym::XK_1,
                '2' => keysym::XK_2,
                '3' => keysym::XK_3,
                '4' => keysym::XK_4,
                '5' => keysym::XK_5,
                '6' => keysym::XK_6,
                '7' => keysym::XK_7,
                '8' => keysym::XK_8,
                '9' => keysym::XK_9,
                '0' => keysym::XK_0,
                _ => unreachable!(),
            },
        }
    }
}

pub enum KeyKind {
    Mod(KbModifier),
    Code(KbCode),
    Unknown,
}

impl From<KbModifier> for KeyKind {
    fn from(value: KbModifier) -> Self {
        Self::Mod(value)
    }
}
impl From<KbCode> for KeyKind {
    fn from(value: KbCode) -> Self {
        Self::Code(value)
    }
}

impl From<&str> for KeyKind {
    fn from(value: &str) -> Self {
        let value = value.to_lowercase();
        match value.as_str() {
            "<super>" => KbModifier::Super.into(),
            "<ctrl>" => KbModifier::Ctrl.into(),
            "<alt>" => KbModifier::Alt.into(),
            "<shift>" => KbModifier::Shift.into(),

            "<enter>" => KbCode::Enter.into(),
            "<space>" => KbCode::Space.into(),
            "<backspace>" => KbCode::Backspace.into(),
            "<esc>" => KbCode::Esc.into(),

            "<end>" => KbCode::End.into(),
            "<home>" => KbCode::Home.into(),
            "<insert>" => KbCode::Insert.into(),
            "<del>" => KbCode::Del.into(),

            "<up>" => KbCode::Up.into(),
            "<down>" => KbCode::Down.into(),
            "<right>" => KbCode::Right.into(),
            "<left>" => KbCode::Left.into(),

            "<pgdown>" => KbCode::PgDown.into(),
            "<pgup>" => KbCode::PgUp.into(),

            "<tab>" => KbCode::Tab.into(),

            "<equals>" => KbCode::Equals.into(),
            "<minus>" => KbCode::Minus.into(),

            "<quote>" => KbCode::Quote.into(),
            "<backtick>" => KbCode::Backtick.into(),
            "<backslash>" => KbCode::Backslash.into(),
            "<slash>" => KbCode::Slash.into(),
            "<semicolon>" => KbCode::Semicolon.into(),

            "<comma>" => KbCode::Comma.into(),
            "<period>" => KbCode::Period.into(),

            "<f1>" => KbCode::F(1).into(),
            "<f2>" => KbCode::F(2).into(),
            "<f3>" => KbCode::F(3).into(),
            "<f4>" => KbCode::F(4).into(),
            "<f5>" => KbCode::F(5).into(),
            "<f6>" => KbCode::F(6).into(),
            "<f7>" => KbCode::F(7).into(),
            "<f8>" => KbCode::F(8).into(),
            "<f9>" => KbCode::F(9).into(),
            "<f10>" => KbCode::F(10).into(),
            "<f11>" => KbCode::F(11).into(),
            "<f12>" => KbCode::F(12).into(),
            _ => {
                if value.len() == 1
                    && let Some(char) = value.chars().next()
                    && char.is_ascii_alphanumeric()
                {
                    KbCode::Char(char).into()
                } else {
                    KeyKind::Unknown
                }
            }
        }
    }
}

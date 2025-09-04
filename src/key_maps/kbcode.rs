use x11_dl::{keysym, xlib};

#[derive(Debug)]
pub enum KbModifierCode {
    Alt,
    Ctrl,
    Super,
    Shift,
}
impl KbModifierCode {
    pub fn to_code(&self) -> u32 {
        match self {
            KbModifierCode::Alt => xlib::Mod1Mask,
            KbModifierCode::Ctrl => xlib::ControlMask,
            KbModifierCode::Super => xlib::Mod4Mask,
            KbModifierCode::Shift => xlib::ShiftMask,
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

    KP0,
    KP1,
    KP2,
    KP3,
    KP4,
    KP5,
    KP6,
    KP7,
    KP8,
    KP9,

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
            KbCode::KP0 => keysym::XK_KP_0,
            KbCode::KP1 => keysym::XK_KP_1,
            KbCode::KP2 => keysym::XK_KP_2,
            KbCode::KP3 => keysym::XK_KP_3,
            KbCode::KP4 => keysym::XK_KP_4,
            KbCode::KP5 => keysym::XK_KP_5,
            KbCode::KP6 => keysym::XK_KP_6,
            KbCode::KP7 => keysym::XK_KP_7,
            KbCode::KP8 => keysym::XK_KP_8,
            KbCode::KP9 => keysym::XK_KP_9,
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
                13 => keysym::XK_F14,
                14 => keysym::XK_F15,
                15 => keysym::XK_F16,
                16 => keysym::XK_F17,
                17 => keysym::XK_F18,
                18 => keysym::XK_F19,
                19 => keysym::XK_F20,
                20 => keysym::XK_F21,
                21 => keysym::XK_F21,
                22 => keysym::XK_F22,
                23 => keysym::XK_F23,
                24 => keysym::XK_F24,
                25 => keysym::XK_F25,
                26 => keysym::XK_F26,
                27 => keysym::XK_F27,
                28 => keysym::XK_F28,
                29 => keysym::XK_F29,
                30 => keysym::XK_F30,
                31 => keysym::XK_F31,
                32 => keysym::XK_F32,
                33 => keysym::XK_F33,
                34 => keysym::XK_F34,
                35 => keysym::XK_F35,
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
                'm' => keysym::XK_m,
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
    Mod(KbModifierCode),
    Code(KbCode),
    Unknown,
}

impl From<KbModifierCode> for KeyKind {
    fn from(value: KbModifierCode) -> Self {
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
            "<super>" => KbModifierCode::Super.into(),
            "<ctrl>" => KbModifierCode::Ctrl.into(),
            "<alt>" => KbModifierCode::Alt.into(),
            "<shift>" => KbModifierCode::Shift.into(),

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
            "<f13>" => KbCode::F(13).into(),
            "<f14>" => KbCode::F(14).into(),
            "<f15>" => KbCode::F(15).into(),
            "<f16>" => KbCode::F(16).into(),
            "<f17>" => KbCode::F(17).into(),
            "<f18>" => KbCode::F(18).into(),
            "<f19>" => KbCode::F(19).into(),
            "<f20>" => KbCode::F(20).into(),
            "<f21>" => KbCode::F(21).into(),
            "<f22>" => KbCode::F(22).into(),
            "<f23>" => KbCode::F(23).into(),
            "<f24>" => KbCode::F(24).into(),
            "<f25>" => KbCode::F(25).into(),
            "<f26>" => KbCode::F(26).into(),
            "<f27>" => KbCode::F(27).into(),
            "<f28>" => KbCode::F(28).into(),
            "<f29>" => KbCode::F(29).into(),
            "<f30>" => KbCode::F(30).into(),
            "<f31>" => KbCode::F(31).into(),
            "<f32>" => KbCode::F(32).into(),
            "<f33>" => KbCode::F(33).into(),
            "<f34>" => KbCode::F(34).into(),
            "<f35>" => KbCode::F(35).into(),

            "<KP0>" => KbCode::KP0.into(),
            "<KP1>" => KbCode::KP1.into(),
            "<KP2>" => KbCode::KP2.into(),
            "<KP3>" => KbCode::KP3.into(),
            "<KP4>" => KbCode::KP4.into(),
            "<KP5>" => KbCode::KP5.into(),
            "<KP6>" => KbCode::KP6.into(),
            "<KP7>" => KbCode::KP7.into(),
            "<KP8>" => KbCode::KP8.into(),
            "<KP9>" => KbCode::KP9.into(),

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

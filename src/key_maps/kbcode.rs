use x11_dl::{
    keysym,
    xlib::{self},
};

#[derive(Debug)]
pub enum KbModifierCode {
    Alt,
    Ctrl,
    Super,
    Shift,
}
pub struct KbCode(u32);

impl KbCode {
    pub fn to_code(&self) -> u32 {
        self.0
    }
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

pub enum KbSym {
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

    XF86AudioMute,
    XF86AudioMicMute,
    XF86AudioNext,
    XF86AudioPrev,
    XF86AudioStop,
    XF86AudioPlay,
    XF86AudioLowerVolue,
    XF86AudioRaiseVolume,

    F(u8),
    Char(char),
}

impl KbSym {
    pub fn to_code(&self) -> u32 {
        match self {
            KbSym::Enter => keysym::XK_Return,
            KbSym::Space => keysym::XK_space,
            KbSym::Backspace => keysym::XK_BackSpace,
            KbSym::Esc => keysym::XK_Escape,
            KbSym::End => keysym::XK_End,
            KbSym::Home => keysym::XK_Home,
            KbSym::Insert => keysym::XK_Insert,
            KbSym::Del => keysym::XK_Delete,
            KbSym::Down => keysym::XK_Down,
            KbSym::Up => keysym::XK_Up,
            KbSym::Left => keysym::XK_Left,
            KbSym::Right => keysym::XK_Right,
            KbSym::PgUp => keysym::XK_Page_Up,
            KbSym::PgDown => keysym::XK_Page_Down,
            KbSym::Tab => keysym::XK_Tab,
            KbSym::Minus => keysym::XK_minus,
            KbSym::Equals => keysym::XK_equal,
            KbSym::Quote => keysym::XK_quoteleft,
            KbSym::Backtick => keysym::XK_quoteright,
            KbSym::Backslash => keysym::XK_backslash,
            KbSym::Slash => keysym::XK_slash,
            KbSym::Semicolon => keysym::XK_semicolon,
            KbSym::Comma => keysym::XK_comma,
            KbSym::Period => keysym::XK_period,

            KbSym::KP0 => keysym::XK_KP_0,
            KbSym::KP1 => keysym::XK_KP_1,
            KbSym::KP2 => keysym::XK_KP_2,
            KbSym::KP3 => keysym::XK_KP_3,
            KbSym::KP4 => keysym::XK_KP_4,
            KbSym::KP5 => keysym::XK_KP_5,
            KbSym::KP6 => keysym::XK_KP_6,
            KbSym::KP7 => keysym::XK_KP_7,
            KbSym::KP8 => keysym::XK_KP_8,
            KbSym::KP9 => keysym::XK_KP_9,

            KbSym::XF86AudioMute => keysym::XF86XK_AudioMute,
            KbSym::XF86AudioMicMute => keysym::XF86XK_AudioMicMute,
            KbSym::XF86AudioNext => keysym::XF86XK_AudioNext,
            KbSym::XF86AudioPrev => keysym::XF86XK_AudioPrev,
            KbSym::XF86AudioStop => keysym::XF86XK_AudioStop,
            KbSym::XF86AudioPlay => keysym::XF86XK_AudioPlay,
            KbSym::XF86AudioLowerVolue => keysym::XF86XK_AudioLowerVolume,
            KbSym::XF86AudioRaiseVolume => keysym::XF86XK_AudioRaiseVolume,

            KbSym::F(f) => match f {
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
            KbSym::Char(c) => match c {
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
                'z' => keysym::XK_z,
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
    KeySym(KbSym),
    KeyCode(KbCode),
    Unknown,
}

impl From<KbModifierCode> for KeyKind {
    fn from(value: KbModifierCode) -> Self {
        Self::Mod(value)
    }
}
impl From<KbSym> for KeyKind {
    fn from(value: KbSym) -> Self {
        Self::KeySym(value)
    }
}
impl From<KbCode> for KeyKind {
    fn from(value: KbCode) -> Self {
        Self::KeyCode(value)
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

            "<enter>" => KbSym::Enter.into(),
            "<space>" => KbSym::Space.into(),
            "<backspace>" => KbSym::Backspace.into(),
            "<esc>" => KbSym::Esc.into(),

            "<end>" => KbSym::End.into(),
            "<home>" => KbSym::Home.into(),
            "<insert>" => KbSym::Insert.into(),
            "<del>" => KbSym::Del.into(),

            "<up>" => KbSym::Up.into(),
            "<down>" => KbSym::Down.into(),
            "<right>" => KbSym::Right.into(),
            "<left>" => KbSym::Left.into(),

            "<pgdown>" => KbSym::PgDown.into(),
            "<pgup>" => KbSym::PgUp.into(),

            "<tab>" => KbSym::Tab.into(),

            "<equals>" => KbSym::Equals.into(),
            "<minus>" => KbSym::Minus.into(),

            "<quote>" => KbSym::Quote.into(),
            "<backtick>" => KbSym::Backtick.into(),
            "<backslash>" => KbSym::Backslash.into(),
            "<slash>" => KbSym::Slash.into(),
            "<semicolon>" => KbSym::Semicolon.into(),

            "<comma>" => KbSym::Comma.into(),
            "<period>" => KbSym::Period.into(),

            "<f1>" => KbSym::F(1).into(),
            "<f2>" => KbSym::F(2).into(),
            "<f3>" => KbSym::F(3).into(),
            "<f4>" => KbSym::F(4).into(),
            "<f5>" => KbSym::F(5).into(),
            "<f6>" => KbSym::F(6).into(),
            "<f7>" => KbSym::F(7).into(),
            "<f8>" => KbSym::F(8).into(),
            "<f9>" => KbSym::F(9).into(),
            "<f10>" => KbSym::F(10).into(),
            "<f11>" => KbSym::F(11).into(),
            "<f12>" => KbSym::F(12).into(),
            "<f13>" => KbSym::F(13).into(),
            "<f14>" => KbSym::F(14).into(),
            "<f15>" => KbSym::F(15).into(),
            "<f16>" => KbSym::F(16).into(),
            "<f17>" => KbSym::F(17).into(),
            "<f18>" => KbSym::F(18).into(),
            "<f19>" => KbSym::F(19).into(),
            "<f20>" => KbSym::F(20).into(),
            "<f21>" => KbSym::F(21).into(),
            "<f22>" => KbSym::F(22).into(),
            "<f23>" => KbSym::F(23).into(),
            "<f24>" => KbSym::F(24).into(),
            "<f25>" => KbSym::F(25).into(),
            "<f26>" => KbSym::F(26).into(),
            "<f27>" => KbSym::F(27).into(),
            "<f28>" => KbSym::F(28).into(),
            "<f29>" => KbSym::F(29).into(),
            "<f30>" => KbSym::F(30).into(),
            "<f31>" => KbSym::F(31).into(),
            "<f32>" => KbSym::F(32).into(),
            "<f33>" => KbSym::F(33).into(),
            "<f34>" => KbSym::F(34).into(),
            "<f35>" => KbSym::F(35).into(),

            "<KP0>" => KbSym::KP0.into(),
            "<KP1>" => KbSym::KP1.into(),
            "<KP2>" => KbSym::KP2.into(),
            "<KP3>" => KbSym::KP3.into(),
            "<KP4>" => KbSym::KP4.into(),
            "<KP5>" => KbSym::KP5.into(),
            "<KP6>" => KbSym::KP6.into(),
            "<KP7>" => KbSym::KP7.into(),
            "<KP8>" => KbSym::KP8.into(),
            "<KP9>" => KbSym::KP9.into(),

            "<XF86AudioMute>" => KbSym::XF86AudioMute.into(),
            "<XF86AudioMicMute>" => KbSym::XF86AudioMicMute.into(),
            "<XF86AudioNext>" => KbSym::XF86AudioNext.into(),
            "<XF86AudioPrev>" => KbSym::XF86AudioPrev.into(),
            "<XF86AudioStop>" => KbSym::XF86AudioStop.into(),
            "<XF86AudioPlay>" => KbSym::XF86AudioPlay.into(),
            "<XF86AudioLowerVolume>" => KbSym::XF86AudioLowerVolue.into(),
            "<XF86AudioRaiseVolume>" => KbSym::XF86AudioRaiseVolume.into(),

            _ => {
                if value.len() == 1
                    && let Some(char) = value.chars().next()
                    && char.is_ascii_alphanumeric()
                {
                    KbSym::Char(char).into()
                } else {
                    if let Some(code) = extract_keycode(&value) {
                        return KbCode(code).into();
                    }
                    KeyKind::Unknown
                }
            }
        }
    }
}

fn extract_keycode(code: &str) -> Option<u32> {
    regex::Regex::new(r#"\[(.*)\]"#)
        .ok()?
        .captures_iter(code)
        .map(|c| c.extract())
        .map(|(_, [value])| value.parse::<u32>().ok())
        .next()
        .flatten()
}

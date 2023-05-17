#[allow(unused)]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum KeyCode {
    /// Empty
    Emp = 0x0,
    A = 0x04,
    B = 0x05,
    C = 0x06,
    D = 0x07,
    E = 0x08,
    F = 0x09,
    G = 0x0A,
    H = 0x0B,
    I = 0x0C,
    J = 0x0D,
    K = 0x0E,
    L = 0x0F,
    M = 0x10,
    N = 0x11,
    O = 0x12,
    P = 0x13,
    Q = 0x14,
    R = 0x15,
    S = 0x16,
    T = 0x17,
    U = 0x18,
    V = 0x19,
    W = 0x1A,
    X = 0x1B,
    Y = 0x1C,
    Z = 0x1D,
    N1 = 0x1E,
    N2 = 0x1F,
    N3 = 0x20,
    N4 = 0x21,
    N5 = 0x22,
    N6 = 0x23,
    N7 = 0x24,
    N8 = 0x25,
    N9 = 0x26,
    N0 = 0x27,
    /// Enter
    Ent = 0x28,
    /// Escape
    Esc = 0x29,
    /// Backspace
    Bsp = 0x2A,
    /// Tab
    Tab = 0x2B,
    /// Space
    Spc = 0x2C,
    /// Minus
    Min = 0x2D,
    /// Equals
    Equ = 0x2E,
    /// Left Square Bracket
    LSB = 0x2F,
    /// Right Square Bracket
    RSB = 0x30,
    /// Backslash
    BS = 0x31,
    /// Semicolon
    Scln = 0x33,
    /// Single Quote
    SQt = 0x34,
    /// Tilde
    Til = 0x35,
    /// Comma
    Com = 0x36,
    /// Period
    Per = 0x37,
    /// Forward Slash
    FS = 0x38,
    /// Capslock
    Caps = 0x39,
    F1 = 0x3A,
    F2 = 0x3B,
    F3 = 0x3C,
    F4 = 0x3D,
    F5 = 0x3E,
    F6 = 0x3F,
    F7 = 0x40,
    F8 = 0x41,
    F9 = 0x42,
    F10 = 0x43,
    F11 = 0x44,
    F12 = 0x45,

    /// Right
    Rgt = 0x4F,
    /// Left
    Lft = 0x50,
    /// Down
    Dwn = 0x51,
    /// Up
    Up = 0x52,

    /// Home
    Hom = 0x4A,
    /// PageUp
    PgUp = 0x4B,
    /// Delete
    Del = 0x4C,
    /// End
    End = 0x4D,
    /// PageDown
    PgDn = 0x4E,

    // Media Keys
    /// Volume Mute
    VlMt = 0x7F,
    /// Volume Up
    VlUp = 0x80,
    /// Volume Down
    VlDn = 0x81,

    // Keypad keys
    /// Left Paren
    LftPar = 0xB6,
    /// Right Paren
    RgtPar = 0xB7,

    // Modifier keys
    Fn = 0xF0,
    /// Left Shift
    LftSft = 0xF1,
    /// Left Control
    LftCtl = 0xF2,
    /// Left Alt
    LftAlt = 0xF3,
    /// Left Command
    LftCmd = 0xF4,
    /// Right Command
    RgtCmd = 0xF5,
    /// Right Alt
    RgtAlt = 0xF6,
    /// Right Alt
    RgtCtl = 0xF7,
    /// Right Shift
    RgtSft = 0xF8,
}

impl KeyCode {
    pub fn modifier_bitmask(&self) -> Option<u8> {
        match *self {
            KeyCode::LftCtl => Some(1 << 0),
            KeyCode::LftSft => Some(1 << 1),
            KeyCode::LftAlt => Some(1 << 2),
            KeyCode::LftCmd => Some(1 << 3),
            KeyCode::RgtCtl => Some(1 << 4),
            KeyCode::RgtSft => Some(1 << 5),
            KeyCode::RgtAlt => Some(1 << 6),
            KeyCode::RgtCmd => Some(1 << 7),
            _ => None,
        }
    }

    pub fn is_modifier(&self) -> bool {
        *self == KeyCode::Fn || self.modifier_bitmask().is_some()
    }
}

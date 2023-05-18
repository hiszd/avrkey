#[allow(unused)]
#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum KeyCode {
    /// Empty
    Emptyzzz = 0x0,
    LtrAzzzz = 0x04,
    LtrBzzzz = 0x05,
    LtrCzzzz = 0x06,
    LtrDzzzz = 0x07,
    LtrEzzzz = 0x08,
    LtrFzzzz = 0x09,
    LtrGzzzz = 0x0A,
    LtrHzzzz = 0x0B,
    LtrIzzzz = 0x0C,
    LtrJzzzz = 0x0D,
    LtrKzzzz = 0x0E,
    LtrLzzzz = 0x0F,
    LtrMzzzz = 0x10,
    LtrNzzzz = 0x11,
    LtrOzzzz = 0x12,
    LtrPzzzz = 0x13,
    LtrQzzzz = 0x14,
    LtrRzzzz = 0x15,
    LtrSzzzz = 0x16,
    LtrTzzzz = 0x17,
    LtrUzzzz = 0x18,
    LtrVzzzz = 0x19,
    LtrWzzzz = 0x1A,
    LtrXzzzz = 0x1B,
    LtrYzzzz = 0x1C,
    LtrZzzzz = 0x1D,
    Num1zzzz = 0x1E,
    Num2zzzz = 0x1F,
    Num3zzzz = 0x20,
    Num4zzzz = 0x21,
    Num5zzzz = 0x22,
    Num6zzzz = 0x23,
    Num7zzzz = 0x24,
    Num8zzzz = 0x25,
    Num9zzzz = 0x26,
    Num0zzzz = 0x27,
    /// Enter
    FunEntzz = 0x28,
    /// Escape
    FunEsczz = 0x29,
    /// Backspace
    FunBkspz = 0x2A,
    /// Tab
    FunTabzz = 0x2B,
    /// Space
    FunSpczz = 0x2C,
    /// Minus
    SymMinzz = 0x2D,
    /// Equals
    SymEquzz = 0x2E,
    /// Left Square Bracket
    SymLBrkz = 0x2F,
    /// Right Square Bracket
    SymRBrkz = 0x30,
    /// Backslash
    SymBszzz = 0x31,
    /// Semicolon
    SymSclnz = 0x33,
    /// Single Quote
    SymSQuot = 0x34,
    /// Tilde
    SymTilde = 0x35,
    /// Comma
    SymComma = 0x36,
    /// Period
    SymPerdz = 0x37,
    /// Forward Slash
    SymFSlaz = 0x38,
    /// Capslock
    FunCapsz = 0x39,
    FunF1zzz = 0x3A,
    FunF2zzz = 0x3B,
    FunF3zzz = 0x3C,
    FunF4zzz = 0x3D,
    FunF5zzz = 0x3E,
    FunF6zzz = 0x3F,
    FunF7zzz = 0x40,
    FunF8zzz = 0x41,
    FunF9zzz = 0x42,
    FunF10zz = 0x43,
    FunF11zz = 0x44,
    FunF12zz = 0x45,

    /// Right
    ArwRight = 0x4F,
    /// Left
    ArwLeftz = 0x50,
    /// Down
    ArwDownz = 0x51,
    /// Up
    ArwUpzzz = 0x52,

    /// Home
    FunHomez = 0x4A,
    /// PageUp
    FunPgUpz = 0x4B,
    /// Delete
    FunDelet = 0x4C,
    /// End
    FunEndzz = 0x4D,
    /// PageDown
    FunPgDwn = 0x4E,

    // Media Keys
    /// Volume Mute
    VolMutez = 0x7F,
    /// Volume Up
    VolUpzzz = 0x80,
    /// Volume Down
    VolDownz = 0x81,

    // Keypad keys
    /// Left Paren
    SymLParn = 0xB6,
    /// Right Paren
    SymRParn = 0xB7,

    // Modifier keys
    ModLay01 = 0xF0,
    /// Left Shift
    ModLShft = 0xF1,
    /// Left Control
    ModLCtrl = 0xF2,
    /// Left Alt
    ModLAltz = 0xF3,
    /// Left Command
    ModLCmdz = 0xF4,
    /// Right Command
    ModRCmdz = 0xF5,
    /// Right Alt
    ModRAltz = 0xF6,
    /// Right Ctrl
    ModRCtrl = 0xF7,
    /// Right Shift
    ModRShft = 0xF8,
}

impl KeyCode {
    pub fn modifier_bitmask(&self) -> Option<u8> {
        match *self {
            KeyCode::ModLCtrl => Some(1 << 0),
            KeyCode::ModLShft => Some(1 << 1),
            KeyCode::ModLAltz => Some(1 << 2),
            KeyCode::ModLCmdz => Some(1 << 3),
            KeyCode::ModRCtrl => Some(1 << 4),
            KeyCode::ModRShft => Some(1 << 5),
            KeyCode::ModRAltz => Some(1 << 6),
            KeyCode::ModRCmdz => Some(1 << 7),
            _ => None,
        }
    }

    pub fn is_modifier(&self) -> bool {
        *self == KeyCode::ModLay01 || self.modifier_bitmask().is_some()
    }
}

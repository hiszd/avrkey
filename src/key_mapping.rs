use crate::key_codes::KeyCode as KC;
use crate::key_codes::KeyCode::*;

#[rustfmt::skip]
pub const NORMAL_LAYER_MAPPING: [[KC; 16]; 5] = [
    [FunEsczz, Num1zzzz, Num2zzzz, Num3zzzz, Num4zzzz, Num5zzzz, Num6zzzz, Num7zzzz, Num8zzzz, Num9zzzz, Num0zzzz, SymMinzz, SymEquzz, FunBkspz, Emptyzzz, FunDelet],
    [FunTabzz, LtrQzzzz, LtrWzzzz, LtrDzzzz, LtrFzzzz, LtrZzzzz, Emptyzzz, SymSclnz, LtrUzzzz, LtrKzzzz, LtrYzzzz, LtrPzzzz, SymLBrkz, SymRBrkz, SymBszzz, FunPgUpz],
    [ModLCtrl, LtrAzzzz, LtrSzzzz, LtrEzzzz, LtrRzzzz, LtrTzzzz, Emptyzzz, LtrHzzzz, LtrNzzzz, LtrIzzzz, LtrOzzzz, LtrLzzzz, SymSQuot, FunBkspz, Emptyzzz, FunPgDwn],
    [ModLShft, LtrGzzzz, LtrXzzzz, LtrCzzzz, LtrVzzzz, SymFSlaz, Emptyzzz, LtrBzzzz, LtrJzzzz, LtrMzzzz, SymComma, SymPerdz, ModRShft, LtrGzzzz, ArwUpzzz, LtrGzzzz],
    [ModLCtrl, Emptyzzz, ModLCmdz, FunSpczz, Emptyzzz, Emptyzzz, SymMinzz, FunEntzz, ModRAltz, Emptyzzz, Emptyzzz, Emptyzzz, Emptyzzz, ArwLeftz, ArwDownz, ArwRight],
];

#[rustfmt::skip]
pub const FN_LAYER_MAPPING: [[KC; 16]; 5] = [
    [LtrAzzzz, LtrBzzzz, LtrCzzzz, LtrDzzzz, LtrEzzzz, LtrFzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz],
    [LtrAzzzz, LtrBzzzz, LtrCzzzz, LtrDzzzz, LtrEzzzz, LtrFzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz],
    [LtrAzzzz, LtrBzzzz, LtrCzzzz, LtrDzzzz, LtrEzzzz, LtrFzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz],
    [LtrAzzzz, LtrBzzzz, LtrCzzzz, LtrDzzzz, LtrEzzzz, LtrFzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz],
    [LtrAzzzz, LtrBzzzz, LtrCzzzz, LtrDzzzz, LtrEzzzz, LtrFzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz],
];

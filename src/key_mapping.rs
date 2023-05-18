use crate::key_codes::KeyCode as KC;
use crate::key_codes::KeyCode::*;

#[rustfmt::skip]
pub const NORMAL_LAYER_MAPPING: [[KC; 16]; 5] = [
    [FunEsczz, Num1zzzz, Num2zzzz, Num3zzzz, Num4zzzz, Num5zzzz, Num6zzzz, Num7zzzz, Num8zzzz, Num9zzzz, Num0zzzz, SymMinzz, SymEquzz, FunBkspz, FunDelet,LtrGzzzz],
    [FunTabzz, LtrQzzzz, LtrWzzzz, LtrDzzzz, LtrFzzzz, LtrZzzzz, SymSclnz, LtrUzzzz, LtrKzzzz, LtrYzzzz, LtrPzzzz, SymLBrkz, SymRBrkz, SymBszzz, FunPgUpz, LtrGzzzz],
    [ModLCtrl, LtrAzzzz, LtrSzzzz, LtrEzzzz, LtrRzzzz, LtrTzzzz, LtrHzzzz, LtrNzzzz, LtrIzzzz, LtrOzzzz, LtrLzzzz, SymSQuot, FunBkspz, FunPgDwn, LtrGzzzz, LtrGzzzz],
    [ModLShft, LtrGzzzz, LtrXzzzz, LtrCzzzz, LtrVzzzz, SymFSlaz, LtrBzzzz, LtrJzzzz, LtrMzzzz, SymComma, SymPerdz, ModRShft, ArwUpzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz],
    [ModLCtrl, ModLCmdz, FunSpczz, SymMinzz, FunEntzz, ModRAltz, ArwLeftz, ArwDownz, ArwRight, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz],
];

#[rustfmt::skip]
pub const FN_LAYER_MAPPING: [[KC; 16]; 5] = [
    [LtrAzzzz, LtrBzzzz,LtrCzzzz, LtrDzzzz, LtrEzzzz,LtrFzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz],
    [LtrAzzzz, LtrBzzzz,LtrCzzzz, LtrDzzzz, LtrEzzzz,LtrFzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz],
    [LtrAzzzz, LtrBzzzz,LtrCzzzz, LtrDzzzz, LtrEzzzz,LtrFzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz],
    [LtrAzzzz, LtrBzzzz,LtrCzzzz, LtrDzzzz, LtrEzzzz,LtrFzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz],
    [LtrAzzzz, LtrBzzzz,LtrCzzzz, LtrDzzzz, LtrEzzzz,LtrFzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz, LtrGzzzz],
];

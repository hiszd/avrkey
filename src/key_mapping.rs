use crate::key_codes::KeyCode as KC;
use crate::key_codes::KeyCode::*;

// Maybe instead of keycodes we store functions that return keycodes.
// This way we end up making them expandable by nature.

#[rustfmt::skip]
pub const NORMAL_LAYER_MAPPING: [[KC; 16]; 5] = [
    [Fun_Escz, Num_1zzz, Num_2zzz, Num_3zzz, Num_4zzz, Num_5zzz, Num_6zzz, Num_7zzz, Num_8zzz, Num_9zzz, Num_0zzz, Sym_Minz, Sym_Equz, Fun_Bksp, ________, Fun_Delz],
    [Fun_Tabz, Ltr_Qzzz, Ltr_Wzzz, Ltr_Dzzz, Ltr_Fzzz, Ltr_Zzzz, ________, Sym_Scln, Ltr_Uzzz, Ltr_Kzzz, Ltr_Yzzz, Ltr_Pzzz, Sym_LBrk, Sym_RBrk, Sym_Bszz, Fun_PgUp],
    [Mod_LCtl, Ltr_Azzz, Ltr_Szzz, Ltr_Ezzz, Ltr_Rzzz, Ltr_Tzzz, ________, Ltr_Hzzz, Ltr_Nzzz, Ltr_Izzz, Ltr_Ozzz, Ltr_Lzzz, Sym_SQut, Fun_Bksp, ________, Fun_PgDn],
    [Mod_LSft, Ltr_Gzzz, Ltr_Xzzz, Ltr_Czzz, Ltr_Vzzz, Sym_FSla, ________, Ltr_Bzzz, Ltr_Jzzz, Ltr_Mzzz, Sym_Coma, Sym_Perd, Mod_RSft, Ltr_Gzzz, Arw_Upzz, Ltr_Gzzz],
    [Mod_LCtl, ________, Mod_LCmd, Fun_Spcz, ________, ________, Sym_Minz, Fun_Entz, Mod_RAlt, ________, ________, ________, ________, Arw_Left, Arw_Down, Arw_Rght],
];

#[rustfmt::skip]
pub const FN_LAYER_MAPPING: [[KC; 16]; 5] = [
    [Ltr_Azzz, Ltr_Bzzz, Ltr_Czzz, Ltr_Dzzz, Ltr_Ezzz, Ltr_Fzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz],
    [Ltr_Azzz, Ltr_Bzzz, Ltr_Czzz, Ltr_Dzzz, Ltr_Ezzz, Ltr_Fzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz],
    [Ltr_Azzz, Ltr_Bzzz, Ltr_Czzz, Ltr_Dzzz, Ltr_Ezzz, Ltr_Fzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz],
    [Ltr_Azzz, Ltr_Bzzz, Ltr_Czzz, Ltr_Dzzz, Ltr_Ezzz, Ltr_Fzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz],
    [Ltr_Azzz, Ltr_Bzzz, Ltr_Czzz, Ltr_Dzzz, Ltr_Ezzz, Ltr_Fzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz, Ltr_Gzzz],
];

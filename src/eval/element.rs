use std::fmt::{Debug, Display};

use super::traits::Weighable;

const HEAVY_LETTERS: &str = "nubtqphsoe";

#[derive(Clone)]
pub enum Element {
    Heavy(String),
    H,
    He,
    Li,
    Be,
    B,
    C,
    N,
    O,
    F,
    Ne,
    Na,
    Mg,
    Al,
    Si,
    P,
    S,
    Cl,
    Ar,
    K,
    Ca,
    Sc,
    Ti,
    V,
    Cr,
    Mn,
    Fe,
    Co,
    Ni,
    Cu,
    Zn,
    Ga,
    Ge,
    As,
    Se,
    Br,
    Kr,
    Rb,
    Sr,
    Y,
    Zr,
    Nb,
    Mo,
    Tc,
    Ru,
    Rh,
    Pd,
    Ag,
    Cd,
    In,
    Sn,
    Sb,
    Te,
    I,
    Xe,
    Cs,
    Ba,
    La,
    Ce,
    Pr,
    Nd,
    Pm,
    Sm,
    Eu,
    Gd,
    Tb,
    Dy,
    Ho,
    Er,
    Tm,
    Yb,
    Lu,
    Hf,
    Ta,
    W,
    Re,
    Os,
    Ir,
    Pt,
    Au,
    Hg,
    Tl,
    Pb,
    Bi,
    Po,
    At,
    Rn,
    Fr,
    Ra,
    Ac,
    Th,
    Pa,
    U,
    Np,
    Pu,
    Am,
    Cm,
    Bk,
    Cf,
    Es,
    Fm,
    Md,
    No,
    Lr,
    Rf,
    Db,
    Sg,
    Bh,
    Hs,
    Mt,
    Ds,
    Rg,
    Cn,
    Nh,
    Fl,
    Mc,
    Lv,
    Ts,
    Og,
}

impl Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Heavy(arg0) => f.debug_tuple("Heavy").field(arg0).finish(),
            Self::H => write!(f, "H"),
            Self::He => write!(f, "He"),
            Self::Li => write!(f, "Li"),
            Self::Be => write!(f, "Be"),
            Self::B => write!(f, "B"),
            Self::C => write!(f, "C"),
            Self::N => write!(f, "N"),
            Self::O => write!(f, "O"),
            Self::F => write!(f, "F"),
            Self::Ne => write!(f, "Ne"),
            Self::Na => write!(f, "Na"),
            Self::Mg => write!(f, "Mg"),
            Self::Al => write!(f, "Al"),
            Self::Si => write!(f, "Si"),
            Self::P => write!(f, "P"),
            Self::S => write!(f, "S"),
            Self::Cl => write!(f, "Cl"),
            Self::Ar => write!(f, "Ar"),
            Self::K => write!(f, "K"),
            Self::Ca => write!(f, "Ca"),
            Self::Sc => write!(f, "Sc"),
            Self::Ti => write!(f, "Ti"),
            Self::V => write!(f, "V"),
            Self::Cr => write!(f, "Cr"),
            Self::Mn => write!(f, "Mn"),
            Self::Fe => write!(f, "Fe"),
            Self::Co => write!(f, "Co"),
            Self::Ni => write!(f, "Ni"),
            Self::Cu => write!(f, "Cu"),
            Self::Zn => write!(f, "Zn"),
            Self::Ga => write!(f, "Ga"),
            Self::Ge => write!(f, "Ge"),
            Self::As => write!(f, "As"),
            Self::Se => write!(f, "Se"),
            Self::Br => write!(f, "Br"),
            Self::Kr => write!(f, "Kr"),
            Self::Rb => write!(f, "Rb"),
            Self::Sr => write!(f, "Sr"),
            Self::Y => write!(f, "Y"),
            Self::Zr => write!(f, "Zr"),
            Self::Nb => write!(f, "Nb"),
            Self::Mo => write!(f, "Mo"),
            Self::Tc => write!(f, "Tc"),
            Self::Ru => write!(f, "Ru"),
            Self::Rh => write!(f, "Rh"),
            Self::Pd => write!(f, "Pd"),
            Self::Ag => write!(f, "Ag"),
            Self::Cd => write!(f, "Cd"),
            Self::In => write!(f, "In"),
            Self::Sn => write!(f, "Sn"),
            Self::Sb => write!(f, "Sb"),
            Self::Te => write!(f, "Te"),
            Self::I => write!(f, "I"),
            Self::Xe => write!(f, "Xe"),
            Self::Cs => write!(f, "Cs"),
            Self::Ba => write!(f, "Ba"),
            Self::La => write!(f, "La"),
            Self::Ce => write!(f, "Ce"),
            Self::Pr => write!(f, "Pr"),
            Self::Nd => write!(f, "Nd"),
            Self::Pm => write!(f, "Pm"),
            Self::Sm => write!(f, "Sm"),
            Self::Eu => write!(f, "Eu"),
            Self::Gd => write!(f, "Gd"),
            Self::Tb => write!(f, "Tb"),
            Self::Dy => write!(f, "Dy"),
            Self::Ho => write!(f, "Ho"),
            Self::Er => write!(f, "Er"),
            Self::Tm => write!(f, "Tm"),
            Self::Yb => write!(f, "Yb"),
            Self::Lu => write!(f, "Lu"),
            Self::Hf => write!(f, "Hf"),
            Self::Ta => write!(f, "Ta"),
            Self::W => write!(f, "W"),
            Self::Re => write!(f, "Re"),
            Self::Os => write!(f, "Os"),
            Self::Ir => write!(f, "Ir"),
            Self::Pt => write!(f, "Pt"),
            Self::Au => write!(f, "Au"),
            Self::Hg => write!(f, "Hg"),
            Self::Tl => write!(f, "Tl"),
            Self::Pb => write!(f, "Pb"),
            Self::Bi => write!(f, "Bi"),
            Self::Po => write!(f, "Po"),
            Self::At => write!(f, "At"),
            Self::Rn => write!(f, "Rn"),
            Self::Fr => write!(f, "Fr"),
            Self::Ra => write!(f, "Ra"),
            Self::Ac => write!(f, "Ac"),
            Self::Th => write!(f, "Th"),
            Self::Pa => write!(f, "Pa"),
            Self::U => write!(f, "U"),
            Self::Np => write!(f, "Np"),
            Self::Pu => write!(f, "Pu"),
            Self::Am => write!(f, "Am"),
            Self::Cm => write!(f, "Cm"),
            Self::Bk => write!(f, "Bk"),
            Self::Cf => write!(f, "Cf"),
            Self::Es => write!(f, "Es"),
            Self::Fm => write!(f, "Fm"),
            Self::Md => write!(f, "Md"),
            Self::No => write!(f, "No"),
            Self::Lr => write!(f, "Lr"),
            Self::Rf => write!(f, "Rf"),
            Self::Db => write!(f, "Db"),
            Self::Sg => write!(f, "Sg"),
            Self::Bh => write!(f, "Bh"),
            Self::Hs => write!(f, "Hs"),
            Self::Mt => write!(f, "Mt"),
            Self::Ds => write!(f, "Ds"),
            Self::Rg => write!(f, "Rg"),
            Self::Cn => write!(f, "Cn"),
            Self::Nh => write!(f, "Nh"),
            Self::Fl => write!(f, "Fl"),
            Self::Mc => write!(f, "Mc"),
            Self::Lv => write!(f, "Lv"),
            Self::Ts => write!(f, "Ts"),
            Self::Og => write!(f, "Og"),
        }
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Heavy(arg0) => f.debug_tuple("Heavy").field(arg0).finish(),
            Self::H => write!(f, "H"),
            Self::He => write!(f, "He"),
            Self::Li => write!(f, "Li"),
            Self::Be => write!(f, "Be"),
            Self::B => write!(f, "B"),
            Self::C => write!(f, "C"),
            Self::N => write!(f, "N"),
            Self::O => write!(f, "O"),
            Self::F => write!(f, "F"),
            Self::Ne => write!(f, "Ne"),
            Self::Na => write!(f, "Na"),
            Self::Mg => write!(f, "Mg"),
            Self::Al => write!(f, "Al"),
            Self::Si => write!(f, "Si"),
            Self::P => write!(f, "P"),
            Self::S => write!(f, "S"),
            Self::Cl => write!(f, "Cl"),
            Self::Ar => write!(f, "Ar"),
            Self::K => write!(f, "K"),
            Self::Ca => write!(f, "Ca"),
            Self::Sc => write!(f, "Sc"),
            Self::Ti => write!(f, "Ti"),
            Self::V => write!(f, "V"),
            Self::Cr => write!(f, "Cr"),
            Self::Mn => write!(f, "Mn"),
            Self::Fe => write!(f, "Fe"),
            Self::Co => write!(f, "Co"),
            Self::Ni => write!(f, "Ni"),
            Self::Cu => write!(f, "Cu"),
            Self::Zn => write!(f, "Zn"),
            Self::Ga => write!(f, "Ga"),
            Self::Ge => write!(f, "Ge"),
            Self::As => write!(f, "As"),
            Self::Se => write!(f, "Se"),
            Self::Br => write!(f, "Br"),
            Self::Kr => write!(f, "Kr"),
            Self::Rb => write!(f, "Rb"),
            Self::Sr => write!(f, "Sr"),
            Self::Y => write!(f, "Y"),
            Self::Zr => write!(f, "Zr"),
            Self::Nb => write!(f, "Nb"),
            Self::Mo => write!(f, "Mo"),
            Self::Tc => write!(f, "Tc"),
            Self::Ru => write!(f, "Ru"),
            Self::Rh => write!(f, "Rh"),
            Self::Pd => write!(f, "Pd"),
            Self::Ag => write!(f, "Ag"),
            Self::Cd => write!(f, "Cd"),
            Self::In => write!(f, "In"),
            Self::Sn => write!(f, "Sn"),
            Self::Sb => write!(f, "Sb"),
            Self::Te => write!(f, "Te"),
            Self::I => write!(f, "I"),
            Self::Xe => write!(f, "Xe"),
            Self::Cs => write!(f, "Cs"),
            Self::Ba => write!(f, "Ba"),
            Self::La => write!(f, "La"),
            Self::Ce => write!(f, "Ce"),
            Self::Pr => write!(f, "Pr"),
            Self::Nd => write!(f, "Nd"),
            Self::Pm => write!(f, "Pm"),
            Self::Sm => write!(f, "Sm"),
            Self::Eu => write!(f, "Eu"),
            Self::Gd => write!(f, "Gd"),
            Self::Tb => write!(f, "Tb"),
            Self::Dy => write!(f, "Dy"),
            Self::Ho => write!(f, "Ho"),
            Self::Er => write!(f, "Er"),
            Self::Tm => write!(f, "Tm"),
            Self::Yb => write!(f, "Yb"),
            Self::Lu => write!(f, "Lu"),
            Self::Hf => write!(f, "Hf"),
            Self::Ta => write!(f, "Ta"),
            Self::W => write!(f, "W"),
            Self::Re => write!(f, "Re"),
            Self::Os => write!(f, "Os"),
            Self::Ir => write!(f, "Ir"),
            Self::Pt => write!(f, "Pt"),
            Self::Au => write!(f, "Au"),
            Self::Hg => write!(f, "Hg"),
            Self::Tl => write!(f, "Tl"),
            Self::Pb => write!(f, "Pb"),
            Self::Bi => write!(f, "Bi"),
            Self::Po => write!(f, "Po"),
            Self::At => write!(f, "At"),
            Self::Rn => write!(f, "Rn"),
            Self::Fr => write!(f, "Fr"),
            Self::Ra => write!(f, "Ra"),
            Self::Ac => write!(f, "Ac"),
            Self::Th => write!(f, "Th"),
            Self::Pa => write!(f, "Pa"),
            Self::U => write!(f, "U"),
            Self::Np => write!(f, "Np"),
            Self::Pu => write!(f, "Pu"),
            Self::Am => write!(f, "Am"),
            Self::Cm => write!(f, "Cm"),
            Self::Bk => write!(f, "Bk"),
            Self::Cf => write!(f, "Cf"),
            Self::Es => write!(f, "Es"),
            Self::Fm => write!(f, "Fm"),
            Self::Md => write!(f, "Md"),
            Self::No => write!(f, "No"),
            Self::Lr => write!(f, "Lr"),
            Self::Rf => write!(f, "Rf"),
            Self::Db => write!(f, "Db"),
            Self::Sg => write!(f, "Sg"),
            Self::Bh => write!(f, "Bh"),
            Self::Hs => write!(f, "Hs"),
            Self::Mt => write!(f, "Mt"),
            Self::Ds => write!(f, "Ds"),
            Self::Rg => write!(f, "Rg"),
            Self::Cn => write!(f, "Cn"),
            Self::Nh => write!(f, "Nh"),
            Self::Fl => write!(f, "Fl"),
            Self::Mc => write!(f, "Mc"),
            Self::Lv => write!(f, "Lv"),
            Self::Ts => write!(f, "Ts"),
            Self::Og => write!(f, "Og"),
        }
    }
}

impl Weighable for Element {
    fn atomic_numbers(&self) -> i64 {
        match self {
            Element::H => 1,
            Element::He => 2,
            Element::Li => 3,
            Element::Be => 4,
            Element::B => 5,
            Element::C => 6,
            Element::N => 7,
            Element::O => 8,
            Element::F => 9,
            Element::Ne => 10,
            Element::Na => 11,
            Element::Mg => 12,
            Element::Al => 13,
            Element::Si => 14,
            Element::P => 15,
            Element::S => 16,
            Element::Cl => 17,
            Element::Ar => 18,
            Element::K => 19,
            Element::Ca => 20,
            Element::Sc => 21,
            Element::Ti => 22,
            Element::V => 23,
            Element::Cr => 24,
            Element::Mn => 25,
            Element::Fe => 26,
            Element::Co => 27,
            Element::Ni => 28,
            Element::Cu => 29,
            Element::Zn => 30,
            Element::Ga => 31,
            Element::Ge => 32,
            Element::As => 33,
            Element::Se => 34,
            Element::Br => 35,
            Element::Kr => 36,
            Element::Rb => 37,
            Element::Sr => 38,
            Element::Y => 39,
            Element::Zr => 40,
            Element::Nb => 41,
            Element::Mo => 42,
            Element::Tc => 43,
            Element::Ru => 44,
            Element::Rh => 45,
            Element::Pd => 46,
            Element::Ag => 47,
            Element::Cd => 48,
            Element::In => 49,
            Element::Sn => 50,
            Element::Sb => 51,
            Element::Te => 52,
            Element::I => 53,
            Element::Xe => 54,
            Element::Cs => 55,
            Element::Ba => 56,
            Element::La => 57,
            Element::Ce => 58,
            Element::Pr => 59,
            Element::Nd => 60,
            Element::Pm => 61,
            Element::Sm => 62,
            Element::Eu => 63,
            Element::Gd => 64,
            Element::Tb => 65,
            Element::Dy => 66,
            Element::Ho => 67,
            Element::Er => 68,
            Element::Tm => 69,
            Element::Yb => 70,
            Element::Lu => 71,
            Element::Hf => 72,
            Element::Ta => 73,
            Element::W => 74,
            Element::Re => 75,
            Element::Os => 76,
            Element::Ir => 77,
            Element::Pt => 78,
            Element::Au => 79,
            Element::Hg => 80,
            Element::Tl => 81,
            Element::Pb => 82,
            Element::Bi => 83,
            Element::Po => 84,
            Element::At => 85,
            Element::Rn => 86,
            Element::Fr => 87,
            Element::Ra => 88,
            Element::Ac => 89,
            Element::Th => 90,
            Element::Pa => 91,
            Element::U => 92,
            Element::Np => 93,
            Element::Pu => 94,
            Element::Am => 95,
            Element::Cm => 96,
            Element::Bk => 97,
            Element::Cf => 98,
            Element::Es => 99,
            Element::Fm => 100,
            Element::Md => 101,
            Element::No => 102,
            Element::Lr => 103,
            Element::Rf => 104,
            Element::Db => 105,
            Element::Sg => 106,
            Element::Bh => 107,
            Element::Hs => 108,
            Element::Mt => 109,
            Element::Ds => 110,
            Element::Rg => 111,
            Element::Cn => 112,
            Element::Nh => 113,
            Element::Fl => 114,
            Element::Mc => 115,
            Element::Lv => 116,
            Element::Ts => 117,
            Element::Og => 118,
            Element::Heavy(s) => {
                s
                    // lowercase the string
                    .to_lowercase()
                    .chars()
                    // map each char to a digit
                    .map(|c| format!("{}", HEAVY_LETTERS.find(|d| d == c).unwrap()))
                    .collect::<String>()
                    // parse into an int
                    .parse::<i64>()
                    .unwrap()
            }
        }
    }
}

macro_rules! from_for_element {
    ($t:ty) => {
        impl From<$t> for Element {
            fn from(n: $t) -> Element {
                match n {
                    1 => Element::H,
                    2 => Element::He,
                    3 => Element::Li,
                    4 => Element::Be,
                    5 => Element::B,
                    6 => Element::C,
                    7 => Element::N,
                    8 => Element::O,
                    9 => Element::F,
                    10 => Element::Ne,
                    11 => Element::Na,
                    12 => Element::Mg,
                    13 => Element::Al,
                    14 => Element::Si,
                    15 => Element::P,
                    16 => Element::S,
                    17 => Element::Cl,
                    18 => Element::Ar,
                    19 => Element::K,
                    20 => Element::Ca,
                    21 => Element::Sc,
                    22 => Element::Ti,
                    23 => Element::V,
                    24 => Element::Cr,
                    25 => Element::Mn,
                    26 => Element::Fe,
                    27 => Element::Co,
                    28 => Element::Ni,
                    29 => Element::Cu,
                    30 => Element::Zn,
                    31 => Element::Ga,
                    32 => Element::Ge,
                    33 => Element::As,
                    34 => Element::Se,
                    35 => Element::Br,
                    36 => Element::Kr,
                    37 => Element::Rb,
                    38 => Element::Sr,
                    39 => Element::Y,
                    40 => Element::Zr,
                    41 => Element::Nb,
                    42 => Element::Mo,
                    43 => Element::Tc,
                    44 => Element::Ru,
                    45 => Element::Rh,
                    46 => Element::Pd,
                    47 => Element::Ag,
                    48 => Element::Cd,
                    49 => Element::In,
                    50 => Element::Sn,
                    51 => Element::Sb,
                    52 => Element::Te,
                    53 => Element::I,
                    54 => Element::Xe,
                    55 => Element::Cs,
                    56 => Element::Ba,
                    57 => Element::La,
                    58 => Element::Ce,
                    59 => Element::Pr,
                    60 => Element::Nd,
                    61 => Element::Pm,
                    62 => Element::Sm,
                    63 => Element::Eu,
                    64 => Element::Gd,
                    65 => Element::Tb,
                    66 => Element::Dy,
                    67 => Element::Ho,
                    68 => Element::Er,
                    69 => Element::Tm,
                    70 => Element::Yb,
                    71 => Element::Lu,
                    72 => Element::Hf,
                    73 => Element::Ta,
                    74 => Element::W,
                    75 => Element::Re,
                    76 => Element::Os,
                    77 => Element::Ir,
                    78 => Element::Pt,
                    79 => Element::Au,
                    80 => Element::Hg,
                    81 => Element::Tl,
                    82 => Element::Pb,
                    83 => Element::Bi,
                    84 => Element::Po,
                    85 => Element::At,
                    86 => Element::Rn,
                    87 => Element::Fr,
                    88 => Element::Ra,
                    89 => Element::Ac,
                    90 => Element::Th,
                    91 => Element::Pa,
                    92 => Element::U,
                    93 => Element::Np,
                    94 => Element::Pu,
                    95 => Element::Am,
                    96 => Element::Cm,
                    97 => Element::Bk,
                    98 => Element::Cf,
                    99 => Element::Es,
                    100 => Element::Fm,
                    101 => Element::Md,
                    102 => Element::No,
                    103 => Element::Lr,
                    104 => Element::Rf,
                    105 => Element::Db,
                    106 => Element::Sg,
                    107 => Element::Bh,
                    108 => Element::Hs,
                    109 => Element::Mt,
                    110 => Element::Ds,
                    111 => Element::Rg,
                    112 => Element::Cn,
                    113 => Element::Nh,
                    114 => Element::Fl,
                    115 => Element::Mc,
                    116 => Element::Lv,
                    117 => Element::Ts,
                    118 => Element::Og,
                    n => {
                        let n_str = n.to_string();
                        let mut digits = n_str
                            .chars()
                            // get digits
                            .map(|d| d.to_digit(10).unwrap())
                            // map digit to letter
                            .map(|d| {
                                HEAVY_LETTERS
                                    .chars()
                                    .nth(d as usize)
                                    .unwrap_or(char::REPLACEMENT_CHARACTER)
                            });
                        let heavy = digits
                            .next()
                            .map(|first_letter| first_letter.to_uppercase())
                            .into_iter()
                            .flatten()
                            .chain(digits)
                            .collect::<String>();
                        Element::Heavy(heavy)
                    }
                }
            }
        }
    };
}

from_for_element!(u8);
from_for_element!(u16);
from_for_element!(u32);
from_for_element!(u64);
from_for_element!(u128);
from_for_element!(i8);
from_for_element!(i16);
from_for_element!(i32);
from_for_element!(i64);
from_for_element!(i128);

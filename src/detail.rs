//! The AGA8 DETAIL equation of state.

use crate::composition::{Composition, CompositionError};
use crate::DensityError;

pub(crate) const NC: usize = 21;
const MAXFLDS: usize = 21;
const NTERMS: usize = 58;
const EPSILON: f64 = 1e-15;
const RDETAIL: f64 = 8.31451;

// Molar masses (g/mol)
const MMI: [f64; 21] = [
    16.043,  // Methane
    28.0135, // Nitrogen
    44.01,   // Carbon dioxide
    30.07,   // Ethane
    44.097,  // Propane
    58.123,  // Isobutane
    58.123,  // n-Butane
    72.15,   // Isopentane
    72.15,   // n-Pentane
    86.177,  // Hexane
    100.204, // Heptane
    114.231, // Octane
    128.258, // Nonane
    142.285, // Decane
    2.0159,  // Hydrogen
    31.9988, // Oxygen
    28.01,   // Carbon monoxide
    18.0153, // Water
    34.082,  // Hydrogen sulfide
    4.0026,  // Helium
    39.948,  // Argon
];

// Coefficients of the equation of state
const AN: [f64; NTERMS] = [
    0.153_832_6,
    1.341_953_,
    -2.998_583_,
    -0.048_312_28,
    0.375_796_5,
    -1.589_575_,
    -0.053_588_47,
    0.886_594_63,
    -0.710_237_04,
    -1.471_722_,
    1.321_850_35,
    -0.786_659_25,
    0.000_000_002_291_29,
    0.157_672_4,
    -0.436_386_4,
    -0.044_081_59,
    -0.003_433_888,
    0.032_059_05,
    0.024_873_55,
    0.073_322_79,
    -0.001_600_573,
    0.642_470_6,
    -0.416_260_1,
    -0.066_899_57,
    0.279_179_5,
    -0.696_605_1,
    -0.002_860_589,
    -0.008_098_836,
    3.150_547_,
    0.007_224_479,
    -0.705_752_9,
    0.534_979_2,
    -0.079_314_91,
    -1.418_465_,
    -5.99905E-17,
    0.105_840_2,
    0.034_317_29,
    -0.007_022_847,
    0.024_955_87,
    0.042_968_18,
    0.746_545_3,
    -0.291_961_3,
    7.294_616_,
    -9.936_757_,
    -0.005_399_808,
    -0.243_256_7,
    0.049_870_16,
    0.003_733_797,
    1.874_951_,
    0.002_168_144,
    -0.658_716_4,
    0.000_205_518,
    0.009_776_195,
    -0.020_487_08,
    0.015_573_22,
    0.006_862_415,
    -0.001_226_752,
    0.002_850_908,
];

// Density exponents
const BN: [usize; NTERMS] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3,
    3, 3, 3, 3, 3, 4, 4, 4, 4, 4, 4, 4, 5, 5, 5, 5, 5, 6, 6, 7, 7, 8, 8, 8, 9, 9,
];

// Exponents on density in EXP[-cn*D^kn] part
// The cn part in this term is not included in this program since it is 1 when kn<>0][and 0 otherwise
const KN: [usize; NTERMS] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 2, 2, 2, 4, 4, 0, 0, 2, 2, 2, 4, 4, 4, 4, 0, 1, 1, 2, 2,
    3, 3, 4, 4, 4, 0, 0, 2, 2, 2, 4, 4, 0, 2, 2, 4, 4, 0, 2, 0, 2, 1, 2, 2, 2, 2,
];

// Temperature exponents
const UN: [f64; NTERMS] = [
    0.0, 0.5, 1.0, 3.5, -0.5, 4.5, 0.5, 7.5, 9.5, 6.0, 12.0, 12.5, -6.0, 2.0, 3.0, 2.0, 2.0, 11.0,
    -0.5, 0.5, 0.0, 4.0, 6.0, 21.0, 23.0, 22.0, -1.0, -0.5, 7.0, -1.0, 6.0, 4.0, 1.0, 9.0, -13.0,
    21.0, 8.0, -0.5, 0.0, 2.0, 7.0, 9.0, 22.0, 23.0, 1.0, 9.0, 3.0, 8.0, 23.0, 1.5, 5.0, -0.5, 4.0,
    7.0, 3.0, 0.0, 1.0, 0.0,
];

// Flags
// fn[13] = 1; fn[27] = 1; fn[30] = 1; fn[35] = 1;
const FN: [i32; NTERMS] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0,
    0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

// gn[5] = 1; gn[6] = 1; gn[25] = 1; gn[29] = 1; gn[32] = 1;
// gn[33] = 1; gn[34] = 1; gn[51] = 1; gn[54] = 1; gn[56] = 1;
const GN: [i32; NTERMS] = [
    0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1,
    1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0,
];

// qn[7] = 1; qn[16] = 1; qn[26] = 1; qn[28] = 1; qn[37] = 1;
// qn[42] = 1; qn[47] = 1; qn[49] = 1; qn[52] = 1; qn[58] = 1;
const QN: [i32; NTERMS] = [
    0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0,
    0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1,
];

// sn[8] = 1; sn[9] = 1;
const SN: [i32; NTERMS] = [
    0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

// wn[10] = 1; wn[11] = 1; wn[12] = 1;
const WN: [i32; NTERMS] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

// Energy parameters
const EI: [f64; MAXFLDS] = [
    151.318_3,
    99.737_78,
    241.960_6,
    244.166_7,
    298.118_3,
    324.068_9,
    337.638_9,
    365.599_9,
    370.682_3,
    402.636_293,
    427.722_63,
    450.325_022,
    470.840_891,
    489.558_373,
    26.957_94,
    122.766_7,
    105.534_8,
    514.015_6,
    296.355,
    2.610_111,
    119.629_9,
];
// Size parameters
const KI: [f64; MAXFLDS] = [
    0.461_925_5,
    0.447_915_3,
    0.455_748_9,
    0.527_920_9,
    0.583_749_,
    0.640_693_7,
    0.634_142_3,
    0.673_857_7,
    0.679_830_7,
    0.717_511_8,
    0.752_518_9,
    0.784_955,
    0.815_273_1,
    0.843_782_6,
    0.351_491_6,
    0.418_695_4,
    0.453_389_4,
    0.382_586_8,
    0.461_826_3,
    0.358_988_8,
    0.421_655_1,
];

// Orientation parameters
const GI: [f64; MAXFLDS] = [
    0.0, 0.027_815, 0.189_065, 0.079_3, 0.141_239, 0.256_692, 0.281_835, 0.332_267, 0.366_911,
    0.289_731, 0.337_542, 0.383_381, 0.427_354, 0.469_659, 0.034_369, 0.021, 0.038_953, 0.332_5,
    0.088_5, 0.0, 0.0,
];

// Quadrupole parameters
const QI: [f64; MAXFLDS] = [
    0.0, 0.0, 0.69, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.067_75,
    0.633_276, 0.0, 0.0,
];

const FI: [f64; MAXFLDS] = [
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    1.0, // High temperature parameter
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
];

const SI: [f64; MAXFLDS] = [
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    1.5822, // Dipole parameter
    0.39,   // Dipole parameter
    0.0, 0.0,
];

const WI: [f64; MAXFLDS] = [
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    1.0, // Association parameter
    0.0, 0.0, 0.0,
];

// Energy parameters
const EIJ: [[f64; MAXFLDS]; MAXFLDS] = [
    [
        1.0, 0.97164, 0.960_644, 1.0, 0.994_635, 1.019_53, 0.989_844, 1.002_35, 0.999_268,
        1.107_274, 0.880_88, 0.880_973, 0.881_067, 0.881_161, 1.170_52, 1.0, 0.990_126, 0.708_218,
        0.931_484, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.02274, 0.97012, 0.945_939, 0.946_914, 0.973_384, 0.95934, 0.94552, 1.0, 1.0,
        1.0, 1.0, 1.0, 1.08632, 1.021, 1.00571, 0.746_954, 0.902_271, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 0.925_053, 0.960_237, 0.906_849, 0.897_362, 0.726_255, 0.859_764, 0.855_134,
        0.831_229, 0.80831, 0.786_323, 0.765_171, 1.28179, 1.0, 1.5, 0.849_408, 0.955_052, 1.0,
        1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.02256, 1.0, 1.01306, 1.0, 1.00532, 1.0, 1.0, 1.0, 1.0, 1.0, 1.16446,
        1.0, 1.0, 0.693_168, 0.946_871, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0049, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.034_787, 1.0,
        1.0, 1.0, 1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.3, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.3, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.008_692, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.010_126, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.011_501, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.012_821, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.014_089, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.1, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
];

const UIJ: [[f64; MAXFLDS]; MAXFLDS] = [
    [
        1.0, 0.886_106, 0.963_827, 1.0, 0.990_877, 1.0, 0.992_291, 1.0, 1.003_67, 1.302_576,
        1.191_904, 1.205_769, 1.219_634, 1.233_498, 1.15639, 1.0, 1.0, 1.0, 0.736_833, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 0.835_058, 0.816_431, 0.915_502, 1.0, 0.993_556, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 0.408_838, 1.0, 1.0, 1.0, 0.993_476, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 0.969_87, 1.0, 1.0, 1.0, 1.0, 1.0, 1.066_638, 1.077_634, 1.088_178,
        1.098_291, 1.108_021, 1.0, 1.0, 0.9, 1.0, 1.045_29, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.065_173, 1.25, 1.25, 1.25, 1.25, 1.0, 1.0, 1.0, 1.0, 1.0, 1.616_66,
        1.0, 1.0, 1.0, 0.971_926, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.028_973, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.033_754, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.038_338, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.042_735, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.046_966, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
];

const KIJ: [[f64; MAXFLDS]; MAXFLDS] = [
    [
        1.0, 1.00363, 0.995_933, 1.0, 1.007_619, 1.0, 0.997_596, 1.0, 1.002_529, 0.982_962,
        0.983_565, 0.982_707, 0.981_849, 0.980_991, 1.023_26, 1.0, 1.0, 1.0, 1.000_08, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 0.982_361, 1.00796, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.03227,
        1.0, 1.0, 1.0, 0.942_596, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.00851, 1.0, 1.0, 1.0, 1.0, 1.0, 0.910_183, 0.895_362, 0.881_152, 0.86752,
        0.854_406, 1.0, 1.0, 1.0, 1.0, 1.00779, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 0.986_893, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.02034, 1.0,
        1.0, 1.0, 0.999_969, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        0.96813, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        0.96287, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        0.957_828, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        0.952_441, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        0.948_338, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
];

const GIJ: [[f64; MAXFLDS]; MAXFLDS] = [
    [
        1.0, 1.0, 0.807_653, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.957_31, 1.0,
        1.0, 1.0, 1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 0.982_746, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 0.370_296, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.67309, 1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
    [
        1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0,
    ],
];

const TH0I: [[f64; 7]; MAXFLDS] = [
    [0.0, 0.0, 0.0, 820.659, 178.41, 1_062.82, 1_090.53],
    [0.0, 0.0, 0.0, 662.738, 680.562, 1740.06, 0.0],
    [0.0, 0.0, 0.0, 919.306, 865.07, 483.553, 341.109],
    [0.0, 0.0, 0.0, 559.314, 223.284, 1_031.38, 1_071.29],
    [0.0, 0.0, 0.0, 479.856, 200.893, 955.312, 1027.29],
    [0.0, 0.0, 0.0, 438.27, 198.018, 1_905.02, 893.765],
    [0.0, 0.0, 0.0, 468.27, 183.636, 1_914.1, 903.185],
    [0.0, 0.0, 0.0, 292.503, 910.237, 1_919.37, 0.0],
    [0.0, 0.0, 0.0, 178.67, 840.538, 1_774.25, 0.0],
    [0.0, 0.0, 0.0, 182.326, 859.207, 1_826.59, 0.0],
    [0.0, 0.0, 0.0, 169.789, 836.195, 1_760.46, 0.0],
    [0.0, 0.0, 0.0, 158.922, 815.064, 1_693.07, 0.0],
    [0.0, 0.0, 0.0, 156.854, 814.882, 1_693.79, 0.0],
    [0.0, 0.0, 0.0, 164.947, 836.264, 1_750.24, 0.0],
    [0.0, 0.0, 0.0, 228.734, 326.843, 1_651.71, 1671.69],
    [0.0, 0.0, 0.0, 2_235.71, 1_116.69, 0.0, 0.0],
    [0.0, 0.0, 0.0, 1_550.45, 704.525, 0.0, 0.0],
    [0.0, 0.0, 0.0, 268.795, 1_141.41, 2_507.37, 0.0],
    [0.0, 0.0, 0.0, 1_833.63, 847.181, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
];

/// Implements the DETAIL equation of state described in
/// AGA Report No. 8, Part 1, Third Edition, April 2017.
///
/// # Example
///
/// ```
/// use aga8::detail::Detail;
///
/// let mut aga8_test: Detail = Detail::new();
///
/// // Set the gas composition in mol fraction
/// // The sum of all the components must be 1.0
/// aga8_test.x = [
///     0.778_240, // Methane
///     0.020_000, // Nitrogen
///     0.060_000, // Carbon dioxide
///     0.080_000, // Ethane
///     0.030_000, // Propane
///     0.001_500, // Isobutane
///     0.003_000, // n-Butane
///     0.000_500, // Isopentane
///     0.001_650, // n-Pentane
///     0.002_150, // Hexane
///     0.000_880, // Heptane
///     0.000_240, // Octane
///     0.000_150, // Nonane
///     0.000_090, // Decane
///     0.004_000, // Hydrogen
///     0.005_000, // Oxygen
///     0.002_000, // Carbon monoxide
///     0.000_100, // Water
///     0.002_500, // Hydrogen sulfide
///     0.007_000, // Helium
///     0.001_000, // Argon
/// ];
/// // Set pressure in kPA
/// aga8_test.p = 50_000.0;
/// // Set temperature in K
/// aga8_test.t = 400.0;
/// // Run density_detail to calculate the density in mol/l
/// aga8_test.density();
/// // Run properties_detail to calculate all of the
/// // output properties mentioned below
/// aga8_test.properties();
///
/// assert!((12.807_924_036_488_01 - aga8_test.d).abs() < 1.0e-10);
/// ```
pub struct Detail {
    // Calculated in the Pressure subroutine,
    // but not included as an argument since it
    // is only used internally in the density algorithm.
    dp_dd_save: f64,

    /// Temperature in K
    pub t: f64,
    /// Pressure in kPa
    pub p: f64,
    /// Molar concentration in mol/l
    pub d: f64,
    /// Compressibility factor
    pub z: f64,
    /// Molar mass in g/mol
    pub mm: f64,
    /// First derivative of pressure with respect
    /// to density at constant temperature in kPa/(mol/l)
    pub dp_dd: f64,
    /// Second derivative of pressure with respect
    /// to density at constant temperature in kPa/(mol/l)^2
    pub d2p_dd2: f64,
    /// Second derivative of pressure with respect to
    /// temperature and density in kPa/(mol/l)/K (currently not calculated)
    pub d2p_dtd: f64,
    /// First derivative of pressure with respect to
    /// temperature at constant density in kPa/K
    pub dp_dt: f64,
    /// Internal energy in J/mol
    pub u: f64,
    /// Enthalpy in J/mol
    pub h: f64,
    /// Entropy in J/(mol-K)
    pub s: f64,
    /// Isochoric heat capacity in J/(mol-K)
    pub cv: f64,
    /// Isobaric heat capacity in J/(mol-K)
    pub cp: f64,
    /// Speed of sound in m/s
    pub w: f64,
    /// Gibbs energy in J/mol
    pub g: f64,
    /// Joule-Thomson coefficient in K/kPa
    pub jt: f64,
    /// Isentropic Exponent
    pub kappa: f64,
    /// Composition mole fractions
    pub x: [f64; NC],

    xold: [f64; MAXFLDS],
    told: f64,
    ki25: [f64; MAXFLDS],
    ei25: [f64; MAXFLDS],
    bsnij2: [[[f64; 18]; MAXFLDS]; MAXFLDS],
    bs: [f64; 18],
    kij5: [[f64; MAXFLDS]; MAXFLDS],
    uij5: [[f64; MAXFLDS]; MAXFLDS],
    gij5: [[f64; MAXFLDS]; MAXFLDS],
    k3: f64,
    csn: [f64; NTERMS],
    a0: [f64; 3],
    ar: [[f64; 4]; 4],
    tun: [f64; NTERMS],
    n0i: [[f64; 7]; MAXFLDS],
}

impl Default for Detail {
    fn default() -> Self {
        Detail {
            dp_dd_save: 0.0,
            x: [0.0; NC],
            t: 0.0,
            p: 0.0,
            d: 0.0,
            z: 0.0,
            mm: 0.0,
            dp_dd: 0.0,
            d2p_dd2: 0.0,
            d2p_dtd: 0.0,
            dp_dt: 0.0,
            u: 0.0,
            h: 0.0,
            s: 0.0,
            cv: 0.0,
            cp: 0.0,
            w: 0.0,
            g: 0.0,
            jt: 0.0,
            kappa: 0.0,
            xold: [0.0; MAXFLDS],
            told: 0.0,
            ki25: [0.0; MAXFLDS],
            ei25: [0.0; MAXFLDS],
            bsnij2: [[[0.0; 18]; MAXFLDS]; MAXFLDS],
            bs: [0.0; 18],
            kij5: [[0.0; MAXFLDS]; MAXFLDS],
            uij5: [[0.0; MAXFLDS]; MAXFLDS],
            gij5: [[0.0; MAXFLDS]; MAXFLDS],
            k3: 0.0,
            a0: [0.0; 3],
            ar: [[0.0; 4]; 4],
            csn: [0.0; NTERMS],
            tun: [0.0; NTERMS],
            n0i: [[0.0; 7]; MAXFLDS],
        }
    }
}

impl Detail {
    /// Constructs a new Detail struct
    pub fn new() -> Self {
        let mut item: Self = Default::default();
        item.setup();
        item
    }

    /// Initialize all the constants and parameters in the DETAIL model.
    fn setup(&mut self) {
        for i in 0..MAXFLDS {
            self.ki25[i] = KI[i].powf(2.5);
            self.ei25[i] = EI[i].powf(2.5);
        }

        self.n0i[0][2] = 4.00088;
        self.n0i[0][3] = 0.76315;
        self.n0i[0][4] = 0.0046;
        self.n0i[0][5] = 8.74432;
        self.n0i[0][6] = -4.46921;
        self.n0i[0][0] = 29.83843397;
        self.n0i[0][1] = -15999.69151;
        self.n0i[1][2] = 3.50031;
        self.n0i[1][3] = 0.13732;
        self.n0i[1][4] = -0.1466;
        self.n0i[1][5] = 0.90066;
        self.n0i[1][6] = 0.0;
        self.n0i[1][0] = 17.56770785;
        self.n0i[1][1] = -2801.729072;
        self.n0i[2][2] = 3.50002;
        self.n0i[2][3] = 2.04452;
        self.n0i[2][4] = -1.06044;
        self.n0i[2][5] = 2.03366;
        self.n0i[2][6] = 0.01393;
        self.n0i[2][0] = 20.65844696;
        self.n0i[2][1] = -4902.171516;
        self.n0i[3][2] = 4.00263;
        self.n0i[3][3] = 4.33939;
        self.n0i[3][4] = 1.23722;
        self.n0i[3][5] = 13.1974;
        self.n0i[3][6] = -6.01989;
        self.n0i[3][0] = 36.73005938;
        self.n0i[3][1] = -23639.65301;
        self.n0i[4][2] = 4.02939;
        self.n0i[4][3] = 6.60569;
        self.n0i[4][4] = 3.197;
        self.n0i[4][5] = 19.1921;
        self.n0i[4][6] = -8.37267;
        self.n0i[4][0] = 44.70909619;
        self.n0i[4][1] = -31236.63551;
        self.n0i[5][2] = 4.06714;
        self.n0i[5][3] = 8.97575;
        self.n0i[5][4] = 5.25156;
        self.n0i[5][5] = 25.1423;
        self.n0i[5][6] = 16.1388;
        self.n0i[5][0] = 34.30180349;
        self.n0i[5][1] = -38525.50276;
        self.n0i[6][2] = 4.33944;
        self.n0i[6][3] = 9.44893;
        self.n0i[6][4] = 6.89406;
        self.n0i[6][5] = 24.4618;
        self.n0i[6][6] = 14.7824;
        self.n0i[6][0] = 36.53237783;
        self.n0i[6][1] = -38957.80933;
        self.n0i[7][2] = 4.0;
        self.n0i[7][3] = 11.7618;
        self.n0i[7][4] = 20.1101;
        self.n0i[7][5] = 33.1688;
        self.n0i[7][6] = 0.0;
        self.n0i[7][0] = 43.17218626;
        self.n0i[7][1] = -51198.30946;
        self.n0i[8][2] = 4.0;
        self.n0i[8][3] = 8.95043;
        self.n0i[8][4] = 21.836;
        self.n0i[8][5] = 33.4032;
        self.n0i[8][6] = 0.0;
        self.n0i[8][0] = 42.67837089;
        self.n0i[8][1] = -45215.83;
        self.n0i[9][2] = 4.0;
        self.n0i[9][3] = 11.6977;
        self.n0i[9][4] = 26.8142;
        self.n0i[9][5] = 38.6164;
        self.n0i[9][6] = 0.0;
        self.n0i[9][0] = 46.99717188;
        self.n0i[9][1] = -52746.83318;
        self.n0i[10][2] = 4.0;
        self.n0i[10][3] = 13.7266;
        self.n0i[10][4] = 30.4707;
        self.n0i[10][5] = 43.5561;
        self.n0i[10][6] = 0.0;
        self.n0i[10][0] = 52.07631631;
        self.n0i[10][1] = -57104.81056;
        self.n0i[11][2] = 4.0;
        self.n0i[11][3] = 15.6865;
        self.n0i[11][4] = 33.8029;
        self.n0i[11][5] = 48.1731;
        self.n0i[11][6] = 0.0;
        self.n0i[11][0] = 57.25830934;
        self.n0i[11][1] = -60546.76385;
        self.n0i[12][2] = 4.0;
        self.n0i[12][3] = 18.0241;
        self.n0i[12][4] = 38.1235;
        self.n0i[12][5] = 53.3415;
        self.n0i[12][6] = 0.0;
        self.n0i[12][0] = 62.09646901;
        self.n0i[12][1] = -66600.12837;
        self.n0i[13][2] = 4.0;
        self.n0i[13][3] = 21.0069;
        self.n0i[13][4] = 43.4931;
        self.n0i[13][5] = 58.3657;
        self.n0i[13][6] = 0.0;
        self.n0i[13][0] = 65.93909154;
        self.n0i[13][1] = -74131.45483;
        self.n0i[14][2] = 2.47906;
        self.n0i[14][3] = 0.95806;
        self.n0i[14][4] = 0.45444;
        self.n0i[14][5] = 1.56039;
        self.n0i[14][6] = -1.3756;
        self.n0i[14][0] = 13.07520288;
        self.n0i[14][1] = -5836.943696;
        self.n0i[15][2] = 3.50146;
        self.n0i[15][3] = 1.07558;
        self.n0i[15][4] = 1.01334;
        self.n0i[15][5] = 0.0;
        self.n0i[15][6] = 0.0;
        self.n0i[15][0] = 16.8017173;
        self.n0i[15][1] = -2318.32269;
        self.n0i[16][2] = 3.50055;
        self.n0i[16][3] = 1.02865;
        self.n0i[16][4] = 0.00493;
        self.n0i[16][5] = 0.0;
        self.n0i[16][6] = 0.0;
        self.n0i[16][0] = 17.45786899;
        self.n0i[16][1] = -2635.244116;
        self.n0i[17][2] = 4.00392;
        self.n0i[17][3] = 0.01059;
        self.n0i[17][4] = 0.98763;
        self.n0i[17][5] = 3.06904;
        self.n0i[17][6] = 0.0;
        self.n0i[17][0] = 21.57882705;
        self.n0i[17][1] = -7766.733078;
        self.n0i[18][2] = 4.0;
        self.n0i[18][3] = 3.11942;
        self.n0i[18][4] = 1.00243;
        self.n0i[18][5] = 0.0;
        self.n0i[18][6] = 0.0;
        self.n0i[18][0] = 21.5830944;
        self.n0i[18][1] = -6069.035869;
        self.n0i[19][2] = 2.5;
        self.n0i[19][3] = 0.0;
        self.n0i[19][4] = 0.0;
        self.n0i[19][5] = 0.0;
        self.n0i[19][6] = 0.0;
        self.n0i[19][0] = 10.04639507;
        self.n0i[19][1] = -745.375;
        self.n0i[20][2] = 2.5;
        self.n0i[20][3] = 0.0;
        self.n0i[20][4] = 0.0;
        self.n0i[20][5] = 0.0;
        self.n0i[20][6] = 0.0;
        self.n0i[20][0] = 10.04639507;
        self.n0i[20][1] = -745.375;

        let mut bsnij: f64;

        for i in 0..MAXFLDS {
            for j in 0..MAXFLDS {
                for n in 0..18 {
                    bsnij = 1.0;
                    if GN[n] == 1 {
                        bsnij = GIJ[i][j] * (GI[i] + GI[j]) / 2.0;
                    }
                    if QN[n] == 1 {
                        bsnij = bsnij * QI[i] * QI[j];
                    }
                    if FN[n] == 1 {
                        bsnij = bsnij * FI[i] * FI[j];
                    }
                    if SN[n] == 1 {
                        bsnij = bsnij * SI[i] * SI[j];
                    }
                    if WN[n] == 1 {
                        bsnij = bsnij * WI[i] * WI[j];
                    }
                    self.bsnij2[i][j][n] = AN[n]
                        * (EIJ[i][j] * (EI[i] * EI[j]).sqrt()).powf(UN[n])
                        * (KI[i] * KI[j]).powf(1.5)
                        * bsnij;
                }
                self.kij5[i][j] = (KIJ[i][j].powi(5) - 1.0) * self.ki25[i] * self.ki25[j];
                self.uij5[i][j] = (UIJ[i][j].powi(5) - 1.0) * self.ei25[i] * self.ei25[j];
                self.gij5[i][j] = (GIJ[i][j] - 1.0) * (GI[i] + GI[j]) / 2.0;
            }
        }

        // Ideal gas terms
        const D0: f64 = 101.325 / RDETAIL / 298.15;

        for i in 0..MAXFLDS {
            self.n0i[i][2] -= 1.0;
            self.n0i[i][0] -= D0.ln();
        }
    }

    /// Sets the composition
    pub fn set_composition(&mut self, comp: &Composition) -> Result<(), CompositionError> {
        comp.check()?;

        self.x[0] = comp.methane;
        self.x[1] = comp.nitrogen;
        self.x[2] = comp.carbon_dioxide;
        self.x[3] = comp.ethane;
        self.x[4] = comp.propane;
        self.x[5] = comp.isobutane;
        self.x[6] = comp.n_butane;
        self.x[7] = comp.isopentane;
        self.x[8] = comp.n_pentane;
        self.x[9] = comp.hexane;
        self.x[10] = comp.heptane;
        self.x[11] = comp.octane;
        self.x[12] = comp.nonane;
        self.x[13] = comp.decane;
        self.x[14] = comp.hydrogen;
        self.x[15] = comp.oxygen;
        self.x[16] = comp.carbon_monoxide;
        self.x[17] = comp.water;
        self.x[18] = comp.hydrogen_sulfide;
        self.x[19] = comp.helium;
        self.x[20] = comp.argon;

        Ok(())
    }

    /// Calculates molar mass of the gas composition
    ///
    /// ## Returns:
    /// - mm - Molar mass (g/mol)
    pub fn molar_mass(&mut self) -> f64 {
        let mut mm = 0.0;
        for (i, item) in MMI.iter().enumerate() {
            mm += self.x[i] * item;
        }
        self.mm = mm;
        mm
    }

    // Calculate terms dependent only on composition
    fn x_terms(&mut self) {
        let mut xij: f64;
        let mut xi2: f64;

        // Check to see if a component fraction has changed.  If x is the same as the previous call, then exit.
        let mut icheck = 0;

        for (i, x) in self.x.iter().enumerate() {
            if (x - self.xold[i]).abs() > 0.000_000_1 {
                icheck = 1;
            }
            self.xold[i] = *x;
        }
        if icheck == 0 {
            return;
        }

        self.k3 = 0.0;
        let mut u = 0.0;
        let mut g = 0.0;
        let mut q = 0.0;
        let mut f = 0.0;
        for n in 0..18 {
            self.bs[n] = 0.0;
        }

        // Calculate pure fluid contributions
        for (i, x) in self.x.iter().enumerate() {
            if x > &0.0 {
                xi2 = x.powi(2);
                self.k3 += x * self.ki25[i]; // K, U, and G are the sums of a pure fluid contribution and a
                u += x * self.ei25[i]; // binary pair contribution
                g += x * GI[i];
                q += x * QI[i]; // Q and F depend only on the pure fluid parts
                f += xi2 * FI[i];

                for n in 0..18 {
                    self.bs[n] += xi2 * self.bsnij2[i][i][n]; // Pure fluid contributions to second virial coefficient
                }
            }
        }
        self.k3 = self.k3.powi(2);
        u = u.powi(2);

        // Binary pair contributions
        for (i, xi) in self.x.iter().enumerate() {
            if xi > &0.0 {
                for (j, xj) in self.x.iter().enumerate().skip(i + 1) {
                    if xj > &0.0 {
                        xij = 2.0 * xi * xj;
                        self.k3 += xij * self.kij5[i][j];
                        u += xij * self.uij5[i][j];
                        g += xij * self.gij5[i][j];

                        for n in 0..18 {
                            self.bs[n] += xij * self.bsnij2[i][j][n]; // Second virial coefficients of mixture
                        }
                    }
                }
            }
        }
        self.k3 = self.k3.powf(0.6);
        u = u.powf(0.2);

        // Third virial and higher coefficients
        let q2 = q.powi(2);
        for n in 12..58 {
            self.csn[n] = AN[n] * u.powf(UN[n]);
            if GN[n] == 1 {
                self.csn[n] *= g;
            }
            if QN[n] == 1 {
                self.csn[n] *= q2;
            }
            if FN[n] == 1 {
                self.csn[n] *= f;
            }
        }
    }

    fn alpha0_detail(&mut self) {
        // Calculate the ideal gas Helmholtz energy and its derivatives with respect to T and D.
        // This routine is not needed when only P (or Z) is calculated.

        // Inputs:
        //      T - Temperature (K)
        //      D - Density (mol/l)
        //    x() - Composition (mole fraction)

        // Outputs:
        // a0(0) - Ideal gas Helmholtz energy (J/mol)
        // a0(1) -   partial  (a0)/partial(T) [J/(mol-K)]
        // a0(2) - T*partial^2(a0)/partial(T)^2 [J/(mol-K)]

        let mut loghyp: f64;
        let mut th0t: f64;
        let mut logxd: f64;

        let mut sumhyp0: f64;
        let mut sumhyp1: f64;
        let mut sumhyp2: f64;

        let mut em: f64;
        let mut ep: f64;
        let mut hcn: f64;
        let mut hsn: f64;

        self.a0[0] = 0.0;
        self.a0[1] = 0.0;
        self.a0[2] = 0.0;
        let logd = if self.d > EPSILON {
            self.d.ln()
        } else {
            EPSILON.ln()
        };
        let logt = self.t.ln();

        for (i, x) in self.x.iter().enumerate() {
            if x > &0.0 {
                logxd = logd + x.ln();
                sumhyp0 = 0.0;
                sumhyp1 = 0.0;
                sumhyp2 = 0.0;

                for j in 3..7 {
                    if TH0I[i][j] > 0.0 {
                        th0t = TH0I[i][j] / self.t;
                        ep = th0t.exp();
                        em = 1.0 / ep;
                        hsn = (ep - em) / 2.0;
                        hcn = (ep + em) / 2.0;

                        if j == 3 || j == 5 {
                            loghyp = hsn.abs().ln();
                            sumhyp0 += self.n0i[i][j] * loghyp;
                            sumhyp1 += self.n0i[i][j] * (loghyp - th0t * hcn / hsn);
                            sumhyp2 += self.n0i[i][j] * (th0t / hsn).powi(2);
                        } else {
                            loghyp = hcn.abs().ln();
                            sumhyp0 += -self.n0i[i][j] * loghyp;
                            sumhyp1 += -self.n0i[i][j] * (loghyp - th0t * hsn / hcn);
                            sumhyp2 += self.n0i[i][j] * (th0t / hcn).powi(2);
                        }
                    }
                }
                self.a0[0] += x
                    * (logxd + self.n0i[i][0] + self.n0i[i][1] / self.t - self.n0i[i][2] * logt
                        + sumhyp0);
                self.a0[1] +=
                    x * (logxd + self.n0i[i][0] - self.n0i[i][2] * (1.0 + logt) + sumhyp1);
                self.a0[2] += -x * (self.n0i[i][2] + sumhyp2);
            }
        }
        self.a0[0] = self.a0[0] * RDETAIL * self.t;
        self.a0[1] *= RDETAIL;
        self.a0[2] *= RDETAIL;
    }

    fn alphar(&mut self, itau: i32, _idel: i32) {
        // Calculate the derivatives of the residual Helmholtz energy (ar) with respect to T and D.
        // itau and idel are inputs that contain the highest derivatives needed.
        // Outputs are returned in the array ar.
        // Subroutine xTerms must be called before this routine if x has changed

        // Inputs:
        //  itau - Set this to 1 to calculate "ar" derivatives with respect to T [i.e., ar(1,0), ar(1,1), and ar(2,0)], otherwise set it to 0.
        //  idel - Currently not used, but kept as an input for future use in specifing the highest density derivative needed.
        //     T - Temperature (K)
        //     D - Density (mol/l)

        // Outputs:
        // ar(0,0) - Residual Helmholtz energy (J/mol)
        // ar(0,1) -   D*partial  (ar)/partial(D) (J/mol)
        // ar(0,2) - D^2*partial^2(ar)/partial(D)^2 (J/mol)
        // ar(0,3) - D^3*partial^3(ar)/partial(D)^3 (J/mol)
        // ar(1,0) -     partial  (ar)/partial(T) [J/(mol-K)]
        // ar(1,1) -   D*partial^2(ar)/partial(D)/partial(T) [J/(mol-K)]
        // ar(2,0) -   T*partial^2(ar)/partial(T)^2 [J/(mol-K)]

        let mut ckd;
        let mut bkd;

        let mut sum;
        let mut s0;
        let mut s1;
        let mut s2;
        let mut s3;

        let mut sum0: [f64; NTERMS] = [0.0; NTERMS];
        let mut sumb: [f64; NTERMS] = [0.0; NTERMS];
        let mut dknn: [f64; 10] = [0.0; 10];
        let mut expn: [f64; 5] = [0.0; 5];

        let mut coefd1: [f64; NTERMS] = [0.0; NTERMS];
        let mut coefd2: [f64; NTERMS] = [0.0; NTERMS];
        let mut coefd3: [f64; NTERMS] = [0.0; NTERMS];

        let mut coeft1: [f64; NTERMS] = [0.0; NTERMS];
        let mut coeft2: [f64; NTERMS] = [0.0; NTERMS];

        for i in 0..4 {
            for j in 0..4 {
                self.ar[i][j] = 0.0;
            }
        }
        if (self.t - self.told).abs() > 0.000_000_1 {
            for (i, item) in UN.iter().enumerate() {
                self.tun[i] = self.t.powf(-item);
            }
        }
        self.told = self.t;

        // Precalculation of common powers and exponents of density
        let dred = self.k3 * self.d;
        dknn[0] = 1.0;

        for n in 1..10 {
            dknn[n] = dred * dknn[n - 1];
        }
        expn[0] = 1.0;

        for n in 1..5 {
            expn[n] = (-dknn[n]).exp();
        }
        let rt = RDETAIL * self.t;

        for n in 0..58 {
            // Contributions to the Helmholtz energy and its derivatives with respect to temperature
            coeft1[n] = RDETAIL * (UN[n] - 1.0);
            coeft2[n] = coeft1[n] * UN[n];
            // Contributions to the virial coefficients
            sumb[n] = 0.0;
            sum0[n] = 0.0;
            if n <= 17 {
                sum = self.bs[n] * self.d;
                if n >= 12 {
                    sum += -self.csn[n] * dred;
                }
                sumb[n] = sum * self.tun[n];
            }
            if n >= 12 {
                // Contributions to the residual part of the Helmholtz energy
                sum0[n] = self.csn[n] * dknn[BN[n]] * self.tun[n] * expn[KN[n]];
                // Contributions to the derivatives of the Helmholtz energy with respect to density
                bkd = BN[n] as f64 - KN[n] as f64 * dknn[KN[n]];
                ckd = KN[n] as f64 * KN[n] as f64 * dknn[KN[n]];
                coefd1[n] = bkd;
                coefd2[n] = bkd * (bkd - 1.0) - ckd;
                coefd3[n] = (bkd - 2.0) * coefd2[n] + ckd * (1.0 - KN[n] as f64 - 2.0 * bkd);
            } else {
                coefd1[n] = 0.0;
                coefd2[n] = 0.0;
                coefd3[n] = 0.0;
            }
        }

        for n in 0..58 {
            // Density derivatives
            s0 = sum0[n] + sumb[n];
            s1 = sum0[n] * coefd1[n] + sumb[n];
            s2 = sum0[n] * coefd2[n];
            s3 = sum0[n] * coefd3[n];
            self.ar[0][0] += rt * s0;
            self.ar[0][1] += rt * s1;
            self.ar[0][2] += rt * s2;
            self.ar[0][3] += rt * s3;
            // Temperature derivatives
            if itau > 0 {
                self.ar[1][1] -= coeft1[n] * s1;
                self.ar[1][0] -= coeft1[n] * s0;
                self.ar[2][0] += coeft2[n] * s0;
                //The following are not used, but fully functional
                //ar(1, 2) = ar(1, 2) - CoefT1(n) * s2;
                //ar(1, 3) = ar(1, 3) - CoefT1(n) * s3;
                //ar(2, 1) = ar(2, 1) + CoefT2(n) * s1;
                //ar(2, 2) = ar(2, 2) + CoefT2(n) * s2;
                //ar(2, 3) = ar(2, 3) + CoefT2(n) * s3;
            }
        }
    }

    /// Calculate density as a function of temperature and pressure.
    ///
    /// This is an iterative routine that calls PressureDetail
    /// to find the correct state point. Generally only 6 iterations at most are required.
    /// If the iteration fails to converge, the ideal gas density and an error message are returned.
    ///
    /// No checks are made to determine the phase boundary, which would have guaranteed that the output is in the gas phase.
    /// It is up to the user to locate the phase boundary, and thus identify the phase of the T and P inputs.
    /// If the state point is 2-phase, the output density will represent a metastable state.
    pub fn density(&mut self) -> Result<(), DensityError> {
        let mut dpdlv: f64;
        let mut vdiff: f64;
        let mut p2: f64;

        if self.p.abs() < EPSILON {
            self.d = 0.0;
            return Err(DensityError::PressureTooLow);
        }
        const TOLR: f64 = 0.000_000_1;
        if self.d > -EPSILON {
            self.d = self.p / RDETAIL / self.t; // Ideal gas estimate
        } else {
            self.d = self.d.abs(); // If D<0, then use as initial estimate
        }
        let plog = self.p.ln();
        let mut vlog = -self.d.ln();
        for _it in 0..20 {
            if !(-7.0..=100.0).contains(&vlog) {
                //ierr = 1; herr = "Calculation failed to converge in DETAIL method, ideal gas density returned.";
                self.d = self.p / RDETAIL / self.t;
                return Err(DensityError::IterationFail);
            }
            self.d = (-vlog).exp();
            p2 = self.pressure();
            if self.dp_dd_save < EPSILON || p2 < EPSILON {
                vlog += 0.1;
            } else {
                // Find the next density with a first order Newton's type iterative scheme, with
                // log(P) as the known variable and log(v) as the unknown property.
                // See AGA 8 publication for further information.
                dpdlv = -self.d * self.dp_dd_save; // d(p)/d[log(v)]
                vdiff = (p2.ln() - plog) * p2 / dpdlv;
                vlog -= vdiff;
                if vdiff.abs() < TOLR {
                    self.d = (-vlog).exp();
                    return Ok(()); // Iteration converged
                }
            }
        }
        //ierr = 1; herr = "Calculation failed to converge in DETAIL method, ideal gas density returned.";
        self.d = self.p / RDETAIL / self.t;
        Err(DensityError::IterationFail)
    }

    /// Calculate pressure as a function of temperature and density.
    ///
    /// The derivative d(P)/d(D) is also calculated
    /// for use in the iterative DensityDetail subroutine (and is only returned as a common variable).
    pub fn pressure(&mut self) -> f64 {
        self.x_terms();
        self.alphar(0, 2);
        self.z = 1.0 + self.ar[0][1] / RDETAIL / self.t; // ar(0,1) is the first derivative of alpha(r) with respect to density
        let p = self.d * RDETAIL * self.t * self.z;
        self.dp_dd_save = RDETAIL * self.t + 2.0 * self.ar[0][1] + self.ar[0][2]; // d(P)/d(D) for use in density iteration
        p
    }

    /// Calculate thermodynamic properties as a function of temperature and density.
    ///
    /// Calls are made to the subroutines
    /// Molarmass, Alpha0Detail, and AlpharDetail.
    ///
    /// If the density is not known, call subroutine DensityDetail first
    /// with the known values of pressure and temperature.
    pub fn properties(&mut self) {
        let mm = self.molar_mass();
        self.x_terms();

        // Calculate the ideal gas Helmholtz energy, and its first and second derivatives with respect to temperature.
        self.alpha0_detail();

        // Calculate the real gas Helmholtz energy, and its derivatives with respect to temperature and/or density.
        self.alphar(2, 3);

        let rt = RDETAIL * self.t;
        self.z = 1.0 + self.ar[0][1] / rt;
        self.p = self.d * rt * self.z;
        self.dp_dd = rt + 2.0 * self.ar[0][1] + self.ar[0][2];
        self.dp_dt = self.d * RDETAIL + self.d * self.ar[1][1];
        let a = self.a0[0] + self.ar[0][0];
        self.s = -self.a0[1] - self.ar[1][0];
        self.u = a + self.t * self.s;
        self.cv = -(self.a0[2] + self.ar[2][0]);
        if self.d > EPSILON {
            self.h = self.u + self.p / self.d;
            self.g = a + self.p / self.d;
            self.cp = self.cv + self.t * (self.dp_dt / self.d).powi(2) / self.dp_dd;
            self.d2p_dd2 = (2.0 * self.ar[0][1] + 4.0 * self.ar[0][2] + self.ar[0][3]) / self.d;
            self.jt = (self.t / self.d * self.dp_dt / self.dp_dd - 1.0) / self.cp / self.d;
        } else {
            self.h = self.u + rt;
            self.g = a + rt;
            self.cp = self.cv + RDETAIL;
            self.d2p_dd2 = 0.0;
            self.jt = 1.0E+20; //=(dB/dT*T-B)/Cp for an ideal gas, but dB/dT is not calculated here
        }
        self.w = 1000.0 * self.cp / self.cv * self.dp_dd / mm;
        if self.w < 0.0 {
            self.w = 0.0;
        }
        self.w = self.w.sqrt();
        self.kappa = self.w * self.w * mm / (rt * 1000.0 * self.z);
        self.d2p_dtd = 0.0;
    }
}

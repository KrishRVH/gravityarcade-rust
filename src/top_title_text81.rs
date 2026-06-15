// Generated from gravity_arcade.swf DefineText 81 and DefineFont 54.
// Coordinates are stored as SWF text-local centipixels; divide by 100 for pixels.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopTitleSegment {
    Line([i16; 2]),
    Quad { control: [i16; 2], to: [i16; 2] },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TopTitleContour {
    pub(super) start: [i16; 2],
    pub(super) segments: &'static [TopTitleSegment],
}

pub const TEXT: &str = "Gravity";
pub const FONT_ID: u16 = 54;
pub const DEFINE_TEXT_ID: u16 = 81;
pub const BOUNDS_CENTIPX: [i16; 4] = [50, 4605, 305, 1425];

const CONTOUR_0: [TopTitleSegment; 20] = [
    TopTitleSegment::Line([664, 514]),
    TopTitleSegment::Line([580, 466]),
    TopTitleSegment::Quad {
        control: [524, 443],
        to: [483, 443],
    },
    TopTitleSegment::Quad {
        control: [357, 443],
        to: [284, 528],
    },
    TopTitleSegment::Quad {
        control: [210, 614],
        to: [210, 758],
    },
    TopTitleSegment::Quad {
        control: [210, 895],
        to: [283, 977],
    },
    TopTitleSegment::Quad {
        control: [354, 1059],
        to: [477, 1059],
    },
    TopTitleSegment::Quad {
        control: [559, 1059],
        to: [612, 1015],
    },
    TopTitleSegment::Line([612, 846]),
    TopTitleSegment::Line([492, 846]),
    TopTitleSegment::Line([492, 716]),
    TopTitleSegment::Line([764, 716]),
    TopTitleSegment::Line([764, 1100]),
    TopTitleSegment::Quad {
        control: [709, 1145],
        to: [624, 1171],
    },
    TopTitleSegment::Quad {
        control: [538, 1195],
        to: [453, 1195],
    },
    TopTitleSegment::Quad {
        control: [269, 1195],
        to: [161, 1075],
    },
    TopTitleSegment::Quad {
        control: [52, 953],
        to: [52, 755],
    },
    TopTitleSegment::Quad {
        control: [52, 555],
        to: [170, 431],
    },
    TopTitleSegment::Quad {
        control: [287, 307],
        to: [487, 307],
    },
    TopTitleSegment::Quad {
        control: [628, 307],
        to: [727, 391],
    },
];

const CONTOUR_1: [TopTitleSegment; 11] = [
    TopTitleSegment::Quad {
        control: [1213, 663],
        to: [1166, 663],
    },
    TopTitleSegment::Quad {
        control: [1115, 663],
        to: [1076, 710],
    },
    TopTitleSegment::Quad {
        control: [1036, 756],
        to: [1036, 823],
    },
    TopTitleSegment::Line([1036, 1180]),
    TopTitleSegment::Line([890, 1180]),
    TopTitleSegment::Line([890, 553]),
    TopTitleSegment::Line([1036, 553]),
    TopTitleSegment::Line([1036, 610]),
    TopTitleSegment::Quad {
        control: [1097, 541],
        to: [1199, 541],
    },
    TopTitleSegment::Quad {
        control: [1274, 541],
        to: [1314, 564],
    },
    TopTitleSegment::Line([1252, 689]),
];

const CONTOUR_2: [TopTitleSegment; 19] = [
    TopTitleSegment::Quad {
        control: [1717, 541],
        to: [1783, 607],
    },
    TopTitleSegment::Quad {
        control: [1850, 671],
        to: [1850, 854],
    },
    TopTitleSegment::Line([1850, 988]),
    TopTitleSegment::Quad {
        control: [1850, 1113],
        to: [1901, 1146],
    },
    TopTitleSegment::Quad {
        control: [1883, 1178],
        to: [1861, 1185],
    },
    TopTitleSegment::Line([1809, 1192]),
    TopTitleSegment::Quad {
        control: [1778, 1192],
        to: [1753, 1168],
    },
    TopTitleSegment::Quad {
        control: [1727, 1145],
        to: [1719, 1118],
    },
    TopTitleSegment::Quad {
        control: [1699, 1151],
        to: [1650, 1172],
    },
    TopTitleSegment::Quad {
        control: [1599, 1192],
        to: [1545, 1192],
    },
    TopTitleSegment::Quad {
        control: [1445, 1192],
        to: [1386, 1141],
    },
    TopTitleSegment::Quad {
        control: [1329, 1091],
        to: [1329, 997],
    },
    TopTitleSegment::Quad {
        control: [1329, 888],
        to: [1411, 827],
    },
    TopTitleSegment::Quad {
        control: [1492, 765],
        to: [1643, 765],
    },
    TopTitleSegment::Line([1704, 775]),
    TopTitleSegment::Quad {
        control: [1704, 663],
        to: [1563, 663],
    },
    TopTitleSegment::Quad {
        control: [1481, 663],
        to: [1425, 691],
    },
    TopTitleSegment::Line([1393, 578]),
    TopTitleSegment::Quad {
        control: [1469, 541],
        to: [1574, 541],
    },
];

const CONTOUR_3: [TopTitleSegment; 5] = [
    TopTitleSegment::Quad {
        control: [1475, 878],
        to: [1475, 991],
    },
    TopTitleSegment::Quad {
        control: [1475, 1076],
        to: [1572, 1076],
    },
    TopTitleSegment::Quad {
        control: [1705, 1076],
        to: [1705, 943],
    },
    TopTitleSegment::Line([1705, 885]),
    TopTitleSegment::Line([1649, 878]),
];

const CONTOUR_4: [TopTitleSegment; 7] = [
    TopTitleSegment::Line([2270, 1192]),
    TopTitleSegment::Line([2217, 1192]),
    TopTitleSegment::Line([1934, 553]),
    TopTitleSegment::Line([2094, 553]),
    TopTitleSegment::Line([2244, 936]),
    TopTitleSegment::Line([2405, 553]),
    TopTitleSegment::Line([2560, 553]),
];

const CONTOUR_5: [TopTitleSegment; 8] = [
    TopTitleSegment::Quad {
        control: [2764, 309],
        to: [2800, 309],
    },
    TopTitleSegment::Quad {
        control: [2835, 309],
        to: [2859, 335],
    },
    TopTitleSegment::Quad {
        control: [2884, 360],
        to: [2884, 395],
    },
    TopTitleSegment::Quad {
        control: [2884, 430],
        to: [2859, 455],
    },
    TopTitleSegment::Quad {
        control: [2835, 479],
        to: [2800, 479],
    },
    TopTitleSegment::Quad {
        control: [2764, 479],
        to: [2740, 455],
    },
    TopTitleSegment::Quad {
        control: [2714, 430],
        to: [2714, 395],
    },
    TopTitleSegment::Quad {
        control: [2714, 360],
        to: [2740, 335],
    },
];

const CONTOUR_6: [TopTitleSegment; 6] = [
    TopTitleSegment::Line([2872, 1180]),
    TopTitleSegment::Line([2723, 1180]),
    TopTitleSegment::Line([2723, 673]),
    TopTitleSegment::Line([2644, 673]),
    TopTitleSegment::Line([2644, 553]),
    TopTitleSegment::Line([2872, 553]),
];

const CONTOUR_7: [TopTitleSegment; 18] = [
    TopTitleSegment::Line([3067, 425]),
    TopTitleSegment::Line([3213, 371]),
    TopTitleSegment::Line([3213, 553]),
    TopTitleSegment::Line([3386, 553]),
    TopTitleSegment::Line([3386, 670]),
    TopTitleSegment::Line([3213, 670]),
    TopTitleSegment::Line([3213, 946]),
    TopTitleSegment::Quad {
        control: [3213, 1012],
        to: [3235, 1041],
    },
    TopTitleSegment::Quad {
        control: [3256, 1069],
        to: [3308, 1069],
    },
    TopTitleSegment::Quad {
        control: [3361, 1069],
        to: [3407, 1039],
    },
    TopTitleSegment::Line([3407, 1174]),
    TopTitleSegment::Quad {
        control: [3355, 1192],
        to: [3261, 1192],
    },
    TopTitleSegment::Quad {
        control: [3168, 1192],
        to: [3117, 1139],
    },
    TopTitleSegment::Quad {
        control: [3067, 1086],
        to: [3067, 988],
    },
    TopTitleSegment::Line([3067, 670]),
    TopTitleSegment::Line([2994, 670]),
    TopTitleSegment::Line([2994, 553]),
    TopTitleSegment::Line([3067, 553]),
];

const CONTOUR_8: [TopTitleSegment; 11] = [
    TopTitleSegment::Line([3797, 1281]),
    TopTitleSegment::Quad {
        control: [3774, 1344],
        to: [3697, 1385],
    },
    TopTitleSegment::Quad {
        control: [3619, 1426],
        to: [3517, 1426],
    },
    TopTitleSegment::Line([3517, 1297]),
    TopTitleSegment::Quad {
        control: [3686, 1297],
        to: [3686, 1212],
    },
    TopTitleSegment::Quad {
        control: [3686, 1155],
        to: [3639, 1039],
    },
    TopTitleSegment::Line([3444, 553]),
    TopTitleSegment::Line([3595, 553]),
    TopTitleSegment::Line([3766, 987]),
    TopTitleSegment::Line([3920, 553]),
    TopTitleSegment::Line([4072, 553]),
];

pub const CONTOURS: [TopTitleContour; 9] = [
    TopTitleContour {
        start: [727, 391],
        segments: &CONTOUR_0,
    },
    TopTitleContour {
        start: [1252, 689],
        segments: &CONTOUR_1,
    },
    TopTitleContour {
        start: [1574, 541],
        segments: &CONTOUR_2,
    },
    TopTitleContour {
        start: [1649, 878],
        segments: &CONTOUR_3,
    },
    TopTitleContour {
        start: [2560, 553],
        segments: &CONTOUR_4,
    },
    TopTitleContour {
        start: [2740, 335],
        segments: &CONTOUR_5,
    },
    TopTitleContour {
        start: [2872, 553],
        segments: &CONTOUR_6,
    },
    TopTitleContour {
        start: [3067, 553],
        segments: &CONTOUR_7,
    },
    TopTitleContour {
        start: [4072, 553],
        segments: &CONTOUR_8,
    },
];

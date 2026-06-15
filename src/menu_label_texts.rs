// Generated from gravity_arcade.swf menu DefineText tags and DefineFont 26.
// Coordinates are stored as SWF text-local centipixels; divide by 100 for pixels.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuLabelSegment {
    Line([i16; 2]),
    Quad { control: [i16; 2], to: [i16; 2] },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MenuLabelContour {
    pub(super) start: [i16; 2],
    pub(super) segments: &'static [MenuLabelSegment],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MenuLabelDefinition {
    pub(super) text: &'static str,
    pub(super) define_text_id: u16,
    pub(super) font_id: u16,
    pub(super) color_rgb: [u8; 3],
    pub(super) bounds_centipx: [i16; 4],
    pub(super) contours: &'static [MenuLabelContour],
}

pub const FONT_ID: u16 = 26;

const HELP_CONTOUR_0: [MenuLabelSegment; 16] = [
    MenuLabelSegment::Line([3270, 756]),
    MenuLabelSegment::Quad {
        control: [3299, 709],
        to: [3365, 680],
    },
    MenuLabelSegment::Quad {
        control: [3431, 648],
        to: [3501, 648],
    },
    MenuLabelSegment::Quad {
        control: [3634, 648],
        to: [3710, 736],
    },
    MenuLabelSegment::Quad {
        control: [3785, 823],
        to: [3785, 975],
    },
    MenuLabelSegment::Line([3785, 1500]),
    MenuLabelSegment::Line([3637, 1500]),
    MenuLabelSegment::Line([3637, 975]),
    MenuLabelSegment::Quad {
        control: [3637, 881],
        to: [3590, 827],
    },
    MenuLabelSegment::Quad {
        control: [3545, 773],
        to: [3460, 773],
    },
    MenuLabelSegment::Quad {
        control: [3407, 773],
        to: [3352, 805],
    },
    MenuLabelSegment::Quad {
        control: [3298, 836],
        to: [3270, 878],
    },
    MenuLabelSegment::Line([3270, 1500]),
    MenuLabelSegment::Line([3121, 1500]),
    MenuLabelSegment::Line([3121, 320]),
    MenuLabelSegment::Line([3270, 320]),
];

const HELP_CONTOUR_1: [MenuLabelSegment; 6] = [
    MenuLabelSegment::Quad {
        control: [4098, 1395],
        to: [4320, 1395],
    },
    MenuLabelSegment::Quad {
        control: [4424, 1395],
        to: [4484, 1311],
    },
    MenuLabelSegment::Quad {
        control: [4542, 1227],
        to: [4542, 1080],
    },
    MenuLabelSegment::Quad {
        control: [4542, 769],
        to: [4320, 769],
    },
    MenuLabelSegment::Quad {
        control: [4218, 769],
        to: [4159, 852],
    },
    MenuLabelSegment::Quad {
        control: [4098, 934],
        to: [4098, 1080],
    },
];

const HELP_CONTOUR_2: [MenuLabelSegment; 8] = [
    MenuLabelSegment::Quad {
        control: [4149, 648],
        to: [4320, 648],
    },
    MenuLabelSegment::Quad {
        control: [4499, 648],
        to: [4599, 762],
    },
    MenuLabelSegment::Quad {
        control: [4698, 877],
        to: [4698, 1080],
    },
    MenuLabelSegment::Quad {
        control: [4698, 1283],
        to: [4596, 1400],
    },
    MenuLabelSegment::Quad {
        control: [4495, 1516],
        to: [4320, 1516],
    },
    MenuLabelSegment::Quad {
        control: [4142, 1516],
        to: [4042, 1398],
    },
    MenuLabelSegment::Quad {
        control: [3942, 1280],
        to: [3942, 1080],
    },
    MenuLabelSegment::Quad {
        control: [3942, 886],
        to: [4046, 767],
    },
];

const HELP_CONTOUR_3: [MenuLabelSegment; 13] = [
    MenuLabelSegment::Line([5630, 1516]),
    MenuLabelSegment::Line([5591, 1516]),
    MenuLabelSegment::Line([5345, 947]),
    MenuLabelSegment::Line([5100, 1516]),
    MenuLabelSegment::Line([5061, 1516]),
    MenuLabelSegment::Line([4762, 661]),
    MenuLabelSegment::Line([4920, 661]),
    MenuLabelSegment::Line([5100, 1211]),
    MenuLabelSegment::Line([5322, 661]),
    MenuLabelSegment::Line([5361, 661]),
    MenuLabelSegment::Line([5591, 1211]),
    MenuLabelSegment::Line([5783, 661]),
    MenuLabelSegment::Line([5930, 661]),
];

const HELP_CONTOUR_4: [MenuLabelSegment; 18] = [
    MenuLabelSegment::Line([6579, 489]),
    MenuLabelSegment::Line([6728, 431]),
    MenuLabelSegment::Line([6728, 664]),
    MenuLabelSegment::Line([6958, 664]),
    MenuLabelSegment::Line([6958, 781]),
    MenuLabelSegment::Line([6728, 781]),
    MenuLabelSegment::Line([6728, 1197]),
    MenuLabelSegment::Quad {
        control: [6728, 1302],
        to: [6764, 1347],
    },
    MenuLabelSegment::Quad {
        control: [6798, 1391],
        to: [6878, 1391],
    },
    MenuLabelSegment::Quad {
        control: [6934, 1391],
        to: [6995, 1362],
    },
    MenuLabelSegment::Line([7017, 1492]),
    MenuLabelSegment::Line([6815, 1516]),
    MenuLabelSegment::Quad {
        control: [6715, 1516],
        to: [6648, 1442],
    },
    MenuLabelSegment::Quad {
        control: [6579, 1369],
        to: [6579, 1256],
    },
    MenuLabelSegment::Line([6579, 781]),
    MenuLabelSegment::Line([6482, 781]),
    MenuLabelSegment::Line([6482, 664]),
    MenuLabelSegment::Line([6579, 664]),
];

const HELP_CONTOUR_5: [MenuLabelSegment; 6] = [
    MenuLabelSegment::Quad {
        control: [7263, 1395],
        to: [7485, 1395],
    },
    MenuLabelSegment::Quad {
        control: [7589, 1395],
        to: [7649, 1311],
    },
    MenuLabelSegment::Quad {
        control: [7707, 1227],
        to: [7707, 1080],
    },
    MenuLabelSegment::Quad {
        control: [7707, 769],
        to: [7485, 769],
    },
    MenuLabelSegment::Quad {
        control: [7383, 769],
        to: [7324, 852],
    },
    MenuLabelSegment::Quad {
        control: [7263, 934],
        to: [7263, 1080],
    },
];

const HELP_CONTOUR_6: [MenuLabelSegment; 8] = [
    MenuLabelSegment::Quad {
        control: [7314, 648],
        to: [7485, 648],
    },
    MenuLabelSegment::Quad {
        control: [7664, 648],
        to: [7764, 762],
    },
    MenuLabelSegment::Quad {
        control: [7863, 877],
        to: [7863, 1080],
    },
    MenuLabelSegment::Quad {
        control: [7863, 1283],
        to: [7761, 1400],
    },
    MenuLabelSegment::Quad {
        control: [7660, 1516],
        to: [7485, 1516],
    },
    MenuLabelSegment::Quad {
        control: [7307, 1516],
        to: [7207, 1398],
    },
    MenuLabelSegment::Quad {
        control: [7107, 1280],
        to: [7107, 1080],
    },
    MenuLabelSegment::Quad {
        control: [7107, 886],
        to: [7211, 767],
    },
];

const HELP_CONTOUR_7: [MenuLabelSegment; 12] = [
    MenuLabelSegment::Line([8650, 733]),
    MenuLabelSegment::Quad {
        control: [8734, 648],
        to: [8853, 648],
    },
    MenuLabelSegment::Quad {
        control: [9031, 648],
        to: [9131, 759],
    },
    MenuLabelSegment::Quad {
        control: [9229, 870],
        to: [9229, 1084],
    },
    MenuLabelSegment::Quad {
        control: [9229, 1275],
        to: [9129, 1395],
    },
    MenuLabelSegment::Quad {
        control: [9029, 1516],
        to: [8840, 1516],
    },
    MenuLabelSegment::Quad {
        control: [8787, 1516],
        to: [8728, 1497],
    },
    MenuLabelSegment::Quad {
        control: [8667, 1478],
        to: [8650, 1455],
    },
    MenuLabelSegment::Line([8650, 1828]),
    MenuLabelSegment::Line([8501, 1828]),
    MenuLabelSegment::Line([8501, 664]),
    MenuLabelSegment::Line([8650, 664]),
];

const HELP_CONTOUR_8: [MenuLabelSegment; 8] = [
    MenuLabelSegment::Line([8650, 1331]),
    MenuLabelSegment::Quad {
        control: [8664, 1353],
        to: [8709, 1372],
    },
    MenuLabelSegment::Quad {
        control: [8754, 1391],
        to: [8797, 1391],
    },
    MenuLabelSegment::Quad {
        control: [9073, 1391],
        to: [9073, 1078],
    },
    MenuLabelSegment::Quad {
        control: [9073, 920],
        to: [9008, 847],
    },
    MenuLabelSegment::Quad {
        control: [8942, 773],
        to: [8798, 773],
    },
    MenuLabelSegment::Quad {
        control: [8767, 773],
        to: [8722, 795],
    },
    MenuLabelSegment::Line([8650, 844]),
];

const HELP_CONTOUR_9: [MenuLabelSegment; 7] = [
    MenuLabelSegment::Line([9691, 1516]),
    MenuLabelSegment::Quad {
        control: [9402, 1516],
        to: [9402, 1264],
    },
    MenuLabelSegment::Line([9402, 320]),
    MenuLabelSegment::Line([9551, 320]),
    MenuLabelSegment::Line([9551, 1239]),
    MenuLabelSegment::Quad {
        control: [9551, 1306],
        to: [9590, 1345],
    },
    MenuLabelSegment::Quad {
        control: [9629, 1383],
        to: [9691, 1383],
    },
];

const HELP_CONTOUR_10: [MenuLabelSegment; 18] = [
    MenuLabelSegment::Quad {
        control: [10303, 648],
        to: [10386, 731],
    },
    MenuLabelSegment::Quad {
        control: [10467, 814],
        to: [10467, 994],
    },
    MenuLabelSegment::Line([10467, 1294]),
    MenuLabelSegment::Quad {
        control: [10467, 1405],
        to: [10533, 1441],
    },
    MenuLabelSegment::Line([10533, 1516]),
    MenuLabelSegment::Quad {
        control: [10442, 1516],
        to: [10399, 1489],
    },
    MenuLabelSegment::Quad {
        control: [10353, 1464],
        to: [10333, 1405],
    },
    MenuLabelSegment::Quad {
        control: [10244, 1516],
        to: [10061, 1516],
    },
    MenuLabelSegment::Quad {
        control: [9963, 1516],
        to: [9891, 1445],
    },
    MenuLabelSegment::Quad {
        control: [9817, 1373],
        to: [9817, 1267],
    },
    MenuLabelSegment::Quad {
        control: [9817, 1139],
        to: [9930, 1052],
    },
    MenuLabelSegment::Quad {
        control: [10041, 964],
        to: [10213, 964],
    },
    MenuLabelSegment::Line([10319, 984]),
    MenuLabelSegment::Quad {
        control: [10319, 781],
        to: [10138, 781],
    },
    MenuLabelSegment::Quad {
        control: [9999, 781],
        to: [9924, 856],
    },
    MenuLabelSegment::Line([9861, 731]),
    MenuLabelSegment::Quad {
        control: [9903, 697],
        to: [9978, 673],
    },
    MenuLabelSegment::Quad {
        control: [10053, 648],
        to: [10121, 648],
    },
];

const HELP_CONTOUR_11: [MenuLabelSegment; 6] = [
    MenuLabelSegment::Quad {
        control: [10108, 1073],
        to: [10038, 1131],
    },
    MenuLabelSegment::Quad {
        control: [9966, 1189],
        to: [9966, 1269],
    },
    MenuLabelSegment::Quad {
        control: [9966, 1398],
        to: [10121, 1398],
    },
    MenuLabelSegment::Quad {
        control: [10233, 1398],
        to: [10319, 1292],
    },
    MenuLabelSegment::Line([10319, 1089]),
    MenuLabelSegment::Line([10221, 1073]),
];

const HELP_CONTOUR_12: [MenuLabelSegment; 13] = [
    MenuLabelSegment::Line([11020, 1642]),
    MenuLabelSegment::Quad {
        control: [10992, 1722],
        to: [10900, 1775],
    },
    MenuLabelSegment::Quad {
        control: [10806, 1828],
        to: [10692, 1828],
    },
    MenuLabelSegment::Line([10692, 1695]),
    MenuLabelSegment::Quad {
        control: [10786, 1695],
        to: [10851, 1653],
    },
    MenuLabelSegment::Quad {
        control: [10918, 1609],
        to: [10918, 1547],
    },
    MenuLabelSegment::Quad {
        control: [10918, 1478],
        to: [10893, 1411],
    },
    MenuLabelSegment::Line([10831, 1245]),
    MenuLabelSegment::Line([10604, 664]),
    MenuLabelSegment::Line([10756, 664]),
    MenuLabelSegment::Line([11003, 1311]),
    MenuLabelSegment::Line([11223, 664]),
    MenuLabelSegment::Line([11375, 664]),
];

const HELP_CONTOURS: [MenuLabelContour; 13] = [
    MenuLabelContour {
        start: [3270, 320],
        segments: &HELP_CONTOUR_0,
    },
    MenuLabelContour {
        start: [4098, 1080],
        segments: &HELP_CONTOUR_1,
    },
    MenuLabelContour {
        start: [4046, 767],
        segments: &HELP_CONTOUR_2,
    },
    MenuLabelContour {
        start: [5930, 661],
        segments: &HELP_CONTOUR_3,
    },
    MenuLabelContour {
        start: [6579, 664],
        segments: &HELP_CONTOUR_4,
    },
    MenuLabelContour {
        start: [7263, 1080],
        segments: &HELP_CONTOUR_5,
    },
    MenuLabelContour {
        start: [7211, 767],
        segments: &HELP_CONTOUR_6,
    },
    MenuLabelContour {
        start: [8650, 664],
        segments: &HELP_CONTOUR_7,
    },
    MenuLabelContour {
        start: [8650, 844],
        segments: &HELP_CONTOUR_8,
    },
    MenuLabelContour {
        start: [9691, 1383],
        segments: &HELP_CONTOUR_9,
    },
    MenuLabelContour {
        start: [10121, 648],
        segments: &HELP_CONTOUR_10,
    },
    MenuLabelContour {
        start: [10221, 1073],
        segments: &HELP_CONTOUR_11,
    },
    MenuLabelContour {
        start: [11375, 664],
        segments: &HELP_CONTOUR_12,
    },
];

pub const HELP: MenuLabelDefinition = MenuLabelDefinition {
    text: "how to play",
    define_text_id: 53,
    font_id: FONT_ID,
    color_rgb: [255, 255, 255],
    bounds_centipx: [3120, 12085, 320, 1830],
    contours: &HELP_CONTOURS,
};

const POLARISATION_CONTOUR_0: [MenuLabelSegment; 32] = [
    MenuLabelSegment::Line([3170, 769]),
    MenuLabelSegment::Quad {
        control: [3228, 842],
        to: [3228, 962],
    },
    MenuLabelSegment::Quad {
        control: [3228, 1089],
        to: [3148, 1175],
    },
    MenuLabelSegment::Quad {
        control: [3070, 1261],
        to: [2942, 1273],
    },
    MenuLabelSegment::Line([2818, 1286]),
    MenuLabelSegment::Line([2760, 1303]),
    MenuLabelSegment::Quad {
        control: [2723, 1317],
        to: [2723, 1341],
    },
    MenuLabelSegment::Quad {
        control: [2723, 1372],
        to: [2799, 1372],
    },
    MenuLabelSegment::Line([2904, 1361]),
    MenuLabelSegment::Line([3010, 1348]),
    MenuLabelSegment::Quad {
        control: [3134, 1348],
        to: [3202, 1408],
    },
    MenuLabelSegment::Quad {
        control: [3271, 1466],
        to: [3271, 1570],
    },
    MenuLabelSegment::Quad {
        control: [3271, 1686],
        to: [3168, 1758],
    },
    MenuLabelSegment::Quad {
        control: [3065, 1828],
        to: [2906, 1828],
    },
    MenuLabelSegment::Quad {
        control: [2824, 1828],
        to: [2735, 1800],
    },
    MenuLabelSegment::Quad {
        control: [2645, 1770],
        to: [2590, 1730],
    },
    MenuLabelSegment::Line([2671, 1611]),
    MenuLabelSegment::Quad {
        control: [2801, 1697],
        to: [2910, 1697],
    },
    MenuLabelSegment::Quad {
        control: [3010, 1697],
        to: [3070, 1662],
    },
    MenuLabelSegment::Quad {
        control: [3128, 1628],
        to: [3128, 1577],
    },
    MenuLabelSegment::Quad {
        control: [3128, 1475],
        to: [2981, 1475],
    },
    MenuLabelSegment::Line([2890, 1488]),
    MenuLabelSegment::Line([2787, 1500]),
    MenuLabelSegment::Quad {
        control: [2609, 1500],
        to: [2609, 1366],
    },
    MenuLabelSegment::Quad {
        control: [2609, 1323],
        to: [2651, 1291],
    },
    MenuLabelSegment::Quad {
        control: [2693, 1256],
        to: [2754, 1242],
    },
    MenuLabelSegment::Quad {
        control: [2578, 1159],
        to: [2578, 955],
    },
    MenuLabelSegment::Quad {
        control: [2578, 823],
        to: [2670, 736],
    },
    MenuLabelSegment::Quad {
        control: [2760, 648],
        to: [2895, 648],
    },
    MenuLabelSegment::Quad {
        control: [3018, 648],
        to: [3088, 698],
    },
    MenuLabelSegment::Line([3162, 609]),
    MenuLabelSegment::Line([3259, 702]),
];

const POLARISATION_CONTOUR_1: [MenuLabelSegment; 8] = [
    MenuLabelSegment::Quad {
        control: [2828, 767],
        to: [2779, 822],
    },
    MenuLabelSegment::Quad {
        control: [2731, 877],
        to: [2731, 955],
    },
    MenuLabelSegment::Quad {
        control: [2731, 1042],
        to: [2778, 1100],
    },
    MenuLabelSegment::Quad {
        control: [2824, 1158],
        to: [2906, 1158],
    },
    MenuLabelSegment::Quad {
        control: [2984, 1158],
        to: [3029, 1102],
    },
    MenuLabelSegment::Quad {
        control: [3073, 1045],
        to: [3073, 955],
    },
    MenuLabelSegment::Quad {
        control: [3073, 877],
        to: [3026, 822],
    },
    MenuLabelSegment::Quad {
        control: [2978, 767],
        to: [2906, 767],
    },
];

const POLARISATION_CONTOUR_2: [MenuLabelSegment; 11] = [
    MenuLabelSegment::Quad {
        control: [3833, 773],
        to: [3783, 773],
    },
    MenuLabelSegment::Quad {
        control: [3704, 773],
        to: [3645, 845],
    },
    MenuLabelSegment::Quad {
        control: [3586, 919],
        to: [3586, 1020],
    },
    MenuLabelSegment::Line([3586, 1500]),
    MenuLabelSegment::Line([3437, 1500]),
    MenuLabelSegment::Line([3437, 664]),
    MenuLabelSegment::Line([3586, 664]),
    MenuLabelSegment::Line([3586, 797]),
    MenuLabelSegment::Quad {
        control: [3667, 648],
        to: [3828, 648],
    },
    MenuLabelSegment::Line([3942, 662]),
    MenuLabelSegment::Line([3881, 806]),
];

const POLARISATION_CONTOUR_3: [MenuLabelSegment; 18] = [
    MenuLabelSegment::Quad {
        control: [4488, 648],
        to: [4571, 731],
    },
    MenuLabelSegment::Quad {
        control: [4653, 814],
        to: [4653, 994],
    },
    MenuLabelSegment::Line([4653, 1294]),
    MenuLabelSegment::Quad {
        control: [4653, 1405],
        to: [4718, 1441],
    },
    MenuLabelSegment::Line([4718, 1516]),
    MenuLabelSegment::Quad {
        control: [4628, 1516],
        to: [4584, 1489],
    },
    MenuLabelSegment::Quad {
        control: [4538, 1464],
        to: [4518, 1405],
    },
    MenuLabelSegment::Quad {
        control: [4429, 1516],
        to: [4246, 1516],
    },
    MenuLabelSegment::Quad {
        control: [4148, 1516],
        to: [4076, 1445],
    },
    MenuLabelSegment::Quad {
        control: [4003, 1373],
        to: [4003, 1267],
    },
    MenuLabelSegment::Quad {
        control: [4003, 1139],
        to: [4115, 1052],
    },
    MenuLabelSegment::Quad {
        control: [4226, 964],
        to: [4398, 964],
    },
    MenuLabelSegment::Line([4504, 984]),
    MenuLabelSegment::Quad {
        control: [4504, 781],
        to: [4323, 781],
    },
    MenuLabelSegment::Quad {
        control: [4184, 781],
        to: [4109, 856],
    },
    MenuLabelSegment::Line([4046, 731]),
    MenuLabelSegment::Quad {
        control: [4088, 697],
        to: [4163, 673],
    },
    MenuLabelSegment::Quad {
        control: [4238, 648],
        to: [4306, 648],
    },
];

const POLARISATION_CONTOUR_4: [MenuLabelSegment; 6] = [
    MenuLabelSegment::Quad {
        control: [4293, 1073],
        to: [4223, 1131],
    },
    MenuLabelSegment::Quad {
        control: [4151, 1189],
        to: [4151, 1269],
    },
    MenuLabelSegment::Quad {
        control: [4151, 1398],
        to: [4306, 1398],
    },
    MenuLabelSegment::Quad {
        control: [4418, 1398],
        to: [4504, 1292],
    },
    MenuLabelSegment::Line([4504, 1089]),
    MenuLabelSegment::Line([4406, 1073]),
];

const POLARISATION_CONTOUR_5: [MenuLabelSegment; 7] = [
    MenuLabelSegment::Line([5188, 1516]),
    MenuLabelSegment::Line([5149, 1516]),
    MenuLabelSegment::Line([4789, 661]),
    MenuLabelSegment::Line([4952, 661]),
    MenuLabelSegment::Line([5174, 1247]),
    MenuLabelSegment::Line([5399, 661]),
    MenuLabelSegment::Line([5555, 661]),
];

const POLARISATION_CONTOUR_6: [MenuLabelSegment; 8] = [
    MenuLabelSegment::Quad {
        control: [5862, 347],
        to: [5890, 375],
    },
    MenuLabelSegment::Quad {
        control: [5917, 402],
        to: [5917, 439],
    },
    MenuLabelSegment::Quad {
        control: [5917, 477],
        to: [5890, 505],
    },
    MenuLabelSegment::Quad {
        control: [5862, 531],
        to: [5824, 531],
    },
    MenuLabelSegment::Quad {
        control: [5787, 531],
        to: [5760, 505],
    },
    MenuLabelSegment::Quad {
        control: [5732, 477],
        to: [5732, 439],
    },
    MenuLabelSegment::Quad {
        control: [5732, 400],
        to: [5759, 373],
    },
    MenuLabelSegment::Quad {
        control: [5785, 347],
        to: [5824, 347],
    },
];

const POLARISATION_CONTOUR_7: [MenuLabelSegment; 6] = [
    MenuLabelSegment::Line([5892, 1500]),
    MenuLabelSegment::Line([5743, 1500]),
    MenuLabelSegment::Line([5743, 789]),
    MenuLabelSegment::Line([5628, 789]),
    MenuLabelSegment::Line([5628, 664]),
    MenuLabelSegment::Line([5892, 664]),
];

const POLARISATION_CONTOUR_8: [MenuLabelSegment; 18] = [
    MenuLabelSegment::Line([6179, 489]),
    MenuLabelSegment::Line([6328, 431]),
    MenuLabelSegment::Line([6328, 664]),
    MenuLabelSegment::Line([6557, 664]),
    MenuLabelSegment::Line([6557, 781]),
    MenuLabelSegment::Line([6328, 781]),
    MenuLabelSegment::Line([6328, 1197]),
    MenuLabelSegment::Quad {
        control: [6328, 1302],
        to: [6364, 1347],
    },
    MenuLabelSegment::Quad {
        control: [6398, 1391],
        to: [6478, 1391],
    },
    MenuLabelSegment::Quad {
        control: [6534, 1391],
        to: [6595, 1362],
    },
    MenuLabelSegment::Line([6617, 1492]),
    MenuLabelSegment::Line([6415, 1516]),
    MenuLabelSegment::Quad {
        control: [6315, 1516],
        to: [6248, 1442],
    },
    MenuLabelSegment::Quad {
        control: [6179, 1369],
        to: [6179, 1256],
    },
    MenuLabelSegment::Line([6179, 781]),
    MenuLabelSegment::Line([6082, 781]),
    MenuLabelSegment::Line([6082, 664]),
    MenuLabelSegment::Line([6179, 664]),
];

const POLARISATION_CONTOUR_9: [MenuLabelSegment; 13] = [
    MenuLabelSegment::Line([7080, 1642]),
    MenuLabelSegment::Quad {
        control: [7052, 1722],
        to: [6960, 1775],
    },
    MenuLabelSegment::Quad {
        control: [6866, 1828],
        to: [6752, 1828],
    },
    MenuLabelSegment::Line([6752, 1695]),
    MenuLabelSegment::Quad {
        control: [6846, 1695],
        to: [6911, 1653],
    },
    MenuLabelSegment::Quad {
        control: [6978, 1609],
        to: [6978, 1547],
    },
    MenuLabelSegment::Quad {
        control: [6978, 1478],
        to: [6953, 1411],
    },
    MenuLabelSegment::Line([6891, 1245]),
    MenuLabelSegment::Line([6664, 664]),
    MenuLabelSegment::Line([6816, 664]),
    MenuLabelSegment::Line([7063, 1311]),
    MenuLabelSegment::Line([7283, 664]),
    MenuLabelSegment::Line([7435, 664]),
];

const POLARISATION_CONTOUR_10: [MenuLabelSegment; 24] = [
    MenuLabelSegment::Quad {
        control: [9148, 802],
        to: [9148, 941],
    },
    MenuLabelSegment::Line([9148, 1500]),
    MenuLabelSegment::Line([9000, 1500]),
    MenuLabelSegment::Line([9000, 970]),
    MenuLabelSegment::Quad {
        control: [9000, 773],
        to: [8828, 773],
    },
    MenuLabelSegment::Quad {
        control: [8775, 773],
        to: [8728, 806],
    },
    MenuLabelSegment::Quad {
        control: [8681, 839],
        to: [8664, 881],
    },
    MenuLabelSegment::Line([8664, 1500]),
    MenuLabelSegment::Line([8516, 1500]),
    MenuLabelSegment::Line([8516, 906]),
    MenuLabelSegment::Quad {
        control: [8516, 844],
        to: [8469, 809],
    },
    MenuLabelSegment::Quad {
        control: [8422, 773],
        to: [8345, 773],
    },
    MenuLabelSegment::Quad {
        control: [8302, 773],
        to: [8252, 808],
    },
    MenuLabelSegment::Quad {
        control: [8200, 842],
        to: [8180, 883],
    },
    MenuLabelSegment::Line([8180, 1500]),
    MenuLabelSegment::Line([8031, 1500]),
    MenuLabelSegment::Line([8031, 664]),
    MenuLabelSegment::Line([8128, 664]),
    MenuLabelSegment::Line([8177, 761]),
    MenuLabelSegment::Quad {
        control: [8262, 648],
        to: [8392, 648],
    },
    MenuLabelSegment::Quad {
        control: [8572, 648],
        to: [8644, 759],
    },
    MenuLabelSegment::Quad {
        control: [8669, 712],
        to: [8736, 680],
    },
    MenuLabelSegment::Quad {
        control: [8805, 648],
        to: [8877, 648],
    },
    MenuLabelSegment::Quad {
        control: [9006, 648],
        to: [9077, 725],
    },
];

const POLARISATION_CONTOUR_11: [MenuLabelSegment; 6] = [
    MenuLabelSegment::Quad {
        control: [9463, 1395],
        to: [9685, 1395],
    },
    MenuLabelSegment::Quad {
        control: [9789, 1395],
        to: [9849, 1311],
    },
    MenuLabelSegment::Quad {
        control: [9907, 1227],
        to: [9907, 1080],
    },
    MenuLabelSegment::Quad {
        control: [9907, 769],
        to: [9685, 769],
    },
    MenuLabelSegment::Quad {
        control: [9583, 769],
        to: [9524, 852],
    },
    MenuLabelSegment::Quad {
        control: [9463, 934],
        to: [9463, 1080],
    },
];

const POLARISATION_CONTOUR_12: [MenuLabelSegment; 8] = [
    MenuLabelSegment::Quad {
        control: [9514, 648],
        to: [9685, 648],
    },
    MenuLabelSegment::Quad {
        control: [9864, 648],
        to: [9964, 762],
    },
    MenuLabelSegment::Quad {
        control: [10063, 877],
        to: [10063, 1080],
    },
    MenuLabelSegment::Quad {
        control: [10063, 1283],
        to: [9961, 1400],
    },
    MenuLabelSegment::Quad {
        control: [9860, 1516],
        to: [9685, 1516],
    },
    MenuLabelSegment::Quad {
        control: [9507, 1516],
        to: [9407, 1398],
    },
    MenuLabelSegment::Quad {
        control: [9307, 1280],
        to: [9307, 1080],
    },
    MenuLabelSegment::Quad {
        control: [9307, 886],
        to: [9411, 767],
    },
];

const POLARISATION_CONTOUR_13: [MenuLabelSegment; 11] = [
    MenuLabelSegment::Line([10899, 320]),
    MenuLabelSegment::Line([10899, 1500]),
    MenuLabelSegment::Line([10751, 1500]),
    MenuLabelSegment::Line([10751, 1438]),
    MenuLabelSegment::Quad {
        control: [10674, 1516],
        to: [10526, 1516],
    },
    MenuLabelSegment::Quad {
        control: [10370, 1516],
        to: [10271, 1403],
    },
    MenuLabelSegment::Quad {
        control: [10174, 1291],
        to: [10174, 1103],
    },
    MenuLabelSegment::Quad {
        control: [10174, 914],
        to: [10287, 781],
    },
    MenuLabelSegment::Quad {
        control: [10399, 648],
        to: [10554, 648],
    },
    MenuLabelSegment::Quad {
        control: [10684, 648],
        to: [10751, 709],
    },
    MenuLabelSegment::Line([10751, 320]),
];

const POLARISATION_CONTOUR_14: [MenuLabelSegment; 7] = [
    MenuLabelSegment::Quad {
        control: [10738, 1347],
        to: [10751, 1322],
    },
    MenuLabelSegment::Line([10751, 858]),
    MenuLabelSegment::Quad {
        control: [10695, 773],
        to: [10598, 773],
    },
    MenuLabelSegment::Quad {
        control: [10477, 773],
        to: [10404, 862],
    },
    MenuLabelSegment::Quad {
        control: [10331, 952],
        to: [10331, 1089],
    },
    MenuLabelSegment::Quad {
        control: [10331, 1391],
        to: [10606, 1391],
    },
    MenuLabelSegment::Quad {
        control: [10640, 1391],
        to: [10690, 1369],
    },
];

const POLARISATION_CONTOUR_15: [MenuLabelSegment; 5] = [
    MenuLabelSegment::Quad {
        control: [11357, 773],
        to: [11289, 838],
    },
    MenuLabelSegment::Quad {
        control: [11225, 898],
        to: [11216, 989],
    },
    MenuLabelSegment::Line([11680, 989]),
    MenuLabelSegment::Quad {
        control: [11680, 898],
        to: [11624, 839],
    },
    MenuLabelSegment::Quad {
        control: [11561, 773],
        to: [11455, 773],
    },
];

const POLARISATION_CONTOUR_16: [MenuLabelSegment; 14] = [
    MenuLabelSegment::Quad {
        control: [11364, 1391],
        to: [11475, 1391],
    },
    MenuLabelSegment::Quad {
        control: [11603, 1391],
        to: [11688, 1317],
    },
    MenuLabelSegment::Line([11750, 1423]),
    MenuLabelSegment::Quad {
        control: [11716, 1458],
        to: [11646, 1483],
    },
    MenuLabelSegment::Quad {
        control: [11557, 1516],
        to: [11447, 1516],
    },
    MenuLabelSegment::Quad {
        control: [11289, 1516],
        to: [11178, 1409],
    },
    MenuLabelSegment::Quad {
        control: [11057, 1291],
        to: [11057, 1092],
    },
    MenuLabelSegment::Quad {
        control: [11057, 884],
        to: [11182, 759],
    },
    MenuLabelSegment::Quad {
        control: [11294, 648],
        to: [11449, 648],
    },
    MenuLabelSegment::Quad {
        control: [11627, 648],
        to: [11728, 748],
    },
    MenuLabelSegment::Quad {
        control: [11827, 845],
        to: [11827, 1006],
    },
    MenuLabelSegment::Quad {
        control: [11827, 1055],
        to: [11816, 1097],
    },
    MenuLabelSegment::Line([11213, 1097]),
    MenuLabelSegment::Quad {
        control: [11213, 1244],
        to: [11292, 1322],
    },
];

const POLARISATION_CONTOURS: [MenuLabelContour; 17] = [
    MenuLabelContour {
        start: [3259, 702],
        segments: &POLARISATION_CONTOUR_0,
    },
    MenuLabelContour {
        start: [2906, 767],
        segments: &POLARISATION_CONTOUR_1,
    },
    MenuLabelContour {
        start: [3881, 806],
        segments: &POLARISATION_CONTOUR_2,
    },
    MenuLabelContour {
        start: [4306, 648],
        segments: &POLARISATION_CONTOUR_3,
    },
    MenuLabelContour {
        start: [4406, 1073],
        segments: &POLARISATION_CONTOUR_4,
    },
    MenuLabelContour {
        start: [5555, 661],
        segments: &POLARISATION_CONTOUR_5,
    },
    MenuLabelContour {
        start: [5824, 347],
        segments: &POLARISATION_CONTOUR_6,
    },
    MenuLabelContour {
        start: [5892, 664],
        segments: &POLARISATION_CONTOUR_7,
    },
    MenuLabelContour {
        start: [6179, 664],
        segments: &POLARISATION_CONTOUR_8,
    },
    MenuLabelContour {
        start: [7435, 664],
        segments: &POLARISATION_CONTOUR_9,
    },
    MenuLabelContour {
        start: [9077, 725],
        segments: &POLARISATION_CONTOUR_10,
    },
    MenuLabelContour {
        start: [9463, 1080],
        segments: &POLARISATION_CONTOUR_11,
    },
    MenuLabelContour {
        start: [9411, 767],
        segments: &POLARISATION_CONTOUR_12,
    },
    MenuLabelContour {
        start: [10751, 320],
        segments: &POLARISATION_CONTOUR_13,
    },
    MenuLabelContour {
        start: [10690, 1369],
        segments: &POLARISATION_CONTOUR_14,
    },
    MenuLabelContour {
        start: [11455, 773],
        segments: &POLARISATION_CONTOUR_15,
    },
    MenuLabelContour {
        start: [11292, 1322],
        segments: &POLARISATION_CONTOUR_16,
    },
];

pub const POLARISATION: MenuLabelDefinition = MenuLabelDefinition {
    text: "gravity mode",
    define_text_id: 49,
    font_id: FONT_ID,
    color_rgb: [255, 255, 255],
    bounds_centipx: [2580, 12580, 320, 1830],
    contours: &POLARISATION_CONTOURS,
};

const MATCHES_CONTOUR_0: [MenuLabelSegment; 24] = [
    MenuLabelSegment::Quad {
        control: [4353, 802],
        to: [4353, 941],
    },
    MenuLabelSegment::Line([4353, 1500]),
    MenuLabelSegment::Line([4205, 1500]),
    MenuLabelSegment::Line([4205, 970]),
    MenuLabelSegment::Quad {
        control: [4205, 773],
        to: [4033, 773],
    },
    MenuLabelSegment::Quad {
        control: [3980, 773],
        to: [3933, 806],
    },
    MenuLabelSegment::Quad {
        control: [3886, 839],
        to: [3869, 881],
    },
    MenuLabelSegment::Line([3869, 1500]),
    MenuLabelSegment::Line([3721, 1500]),
    MenuLabelSegment::Line([3721, 906]),
    MenuLabelSegment::Quad {
        control: [3721, 844],
        to: [3674, 809],
    },
    MenuLabelSegment::Quad {
        control: [3627, 773],
        to: [3550, 773],
    },
    MenuLabelSegment::Quad {
        control: [3507, 773],
        to: [3457, 808],
    },
    MenuLabelSegment::Quad {
        control: [3405, 842],
        to: [3385, 883],
    },
    MenuLabelSegment::Line([3385, 1500]),
    MenuLabelSegment::Line([3236, 1500]),
    MenuLabelSegment::Line([3236, 664]),
    MenuLabelSegment::Line([3333, 664]),
    MenuLabelSegment::Line([3382, 761]),
    MenuLabelSegment::Quad {
        control: [3467, 648],
        to: [3597, 648],
    },
    MenuLabelSegment::Quad {
        control: [3777, 648],
        to: [3849, 759],
    },
    MenuLabelSegment::Quad {
        control: [3874, 712],
        to: [3941, 680],
    },
    MenuLabelSegment::Quad {
        control: [4010, 648],
        to: [4082, 648],
    },
    MenuLabelSegment::Quad {
        control: [4211, 648],
        to: [4282, 725],
    },
];

const MATCHES_CONTOUR_1: [MenuLabelSegment; 18] = [
    MenuLabelSegment::Quad {
        control: [5008, 648],
        to: [5091, 731],
    },
    MenuLabelSegment::Quad {
        control: [5172, 814],
        to: [5172, 994],
    },
    MenuLabelSegment::Line([5172, 1294]),
    MenuLabelSegment::Quad {
        control: [5172, 1405],
        to: [5238, 1441],
    },
    MenuLabelSegment::Line([5238, 1516]),
    MenuLabelSegment::Quad {
        control: [5147, 1516],
        to: [5104, 1489],
    },
    MenuLabelSegment::Quad {
        control: [5058, 1464],
        to: [5038, 1405],
    },
    MenuLabelSegment::Quad {
        control: [4949, 1516],
        to: [4766, 1516],
    },
    MenuLabelSegment::Quad {
        control: [4668, 1516],
        to: [4596, 1445],
    },
    MenuLabelSegment::Quad {
        control: [4522, 1373],
        to: [4522, 1267],
    },
    MenuLabelSegment::Quad {
        control: [4522, 1139],
        to: [4635, 1052],
    },
    MenuLabelSegment::Quad {
        control: [4746, 964],
        to: [4918, 964],
    },
    MenuLabelSegment::Line([5024, 984]),
    MenuLabelSegment::Quad {
        control: [5024, 781],
        to: [4843, 781],
    },
    MenuLabelSegment::Quad {
        control: [4704, 781],
        to: [4629, 856],
    },
    MenuLabelSegment::Line([4566, 731]),
    MenuLabelSegment::Quad {
        control: [4608, 697],
        to: [4683, 673],
    },
    MenuLabelSegment::Quad {
        control: [4758, 648],
        to: [4826, 648],
    },
];

const MATCHES_CONTOUR_2: [MenuLabelSegment; 6] = [
    MenuLabelSegment::Quad {
        control: [4813, 1073],
        to: [4743, 1131],
    },
    MenuLabelSegment::Quad {
        control: [4671, 1189],
        to: [4671, 1269],
    },
    MenuLabelSegment::Quad {
        control: [4671, 1398],
        to: [4826, 1398],
    },
    MenuLabelSegment::Quad {
        control: [4938, 1398],
        to: [5024, 1292],
    },
    MenuLabelSegment::Line([5024, 1089]),
    MenuLabelSegment::Line([4926, 1073]),
];

const MATCHES_CONTOUR_3: [MenuLabelSegment; 18] = [
    MenuLabelSegment::Line([5459, 489]),
    MenuLabelSegment::Line([5608, 431]),
    MenuLabelSegment::Line([5608, 664]),
    MenuLabelSegment::Line([5838, 664]),
    MenuLabelSegment::Line([5838, 781]),
    MenuLabelSegment::Line([5608, 781]),
    MenuLabelSegment::Line([5608, 1197]),
    MenuLabelSegment::Quad {
        control: [5608, 1302],
        to: [5644, 1347],
    },
    MenuLabelSegment::Quad {
        control: [5678, 1391],
        to: [5758, 1391],
    },
    MenuLabelSegment::Quad {
        control: [5814, 1391],
        to: [5875, 1362],
    },
    MenuLabelSegment::Line([5897, 1492]),
    MenuLabelSegment::Line([5695, 1516]),
    MenuLabelSegment::Quad {
        control: [5595, 1516],
        to: [5528, 1442],
    },
    MenuLabelSegment::Quad {
        control: [5459, 1369],
        to: [5459, 1256],
    },
    MenuLabelSegment::Line([5459, 781]),
    MenuLabelSegment::Line([5362, 781]),
    MenuLabelSegment::Line([5362, 664]),
    MenuLabelSegment::Line([5459, 664]),
];

const MATCHES_CONTOUR_4: [MenuLabelSegment; 16] = [
    MenuLabelSegment::Quad {
        control: [6637, 703],
        to: [6673, 731],
    },
    MenuLabelSegment::Line([6599, 836]),
    MenuLabelSegment::Quad {
        control: [6576, 814],
        to: [6519, 794],
    },
    MenuLabelSegment::Line([6405, 773]),
    MenuLabelSegment::Quad {
        control: [6285, 773],
        to: [6213, 858],
    },
    MenuLabelSegment::Quad {
        control: [6143, 942],
        to: [6143, 1091],
    },
    MenuLabelSegment::Quad {
        control: [6143, 1238],
        to: [6215, 1314],
    },
    MenuLabelSegment::Quad {
        control: [6288, 1391],
        to: [6416, 1391],
    },
    MenuLabelSegment::Quad {
        control: [6516, 1391],
        to: [6618, 1314],
    },
    MenuLabelSegment::Line([6677, 1439]),
    MenuLabelSegment::Quad {
        control: [6557, 1516],
        to: [6380, 1516],
    },
    MenuLabelSegment::Quad {
        control: [6210, 1516],
        to: [6098, 1402],
    },
    MenuLabelSegment::Quad {
        control: [5987, 1286],
        to: [5987, 1091],
    },
    MenuLabelSegment::Quad {
        control: [5987, 891],
        to: [6102, 769],
    },
    MenuLabelSegment::Quad {
        control: [6218, 648],
        to: [6419, 648],
    },
    MenuLabelSegment::Line([6560, 675]),
];

const MATCHES_CONTOUR_5: [MenuLabelSegment; 16] = [
    MenuLabelSegment::Line([6980, 756]),
    MenuLabelSegment::Quad {
        control: [7009, 709],
        to: [7075, 680],
    },
    MenuLabelSegment::Quad {
        control: [7141, 648],
        to: [7211, 648],
    },
    MenuLabelSegment::Quad {
        control: [7344, 648],
        to: [7420, 736],
    },
    MenuLabelSegment::Quad {
        control: [7495, 823],
        to: [7495, 975],
    },
    MenuLabelSegment::Line([7495, 1500]),
    MenuLabelSegment::Line([7347, 1500]),
    MenuLabelSegment::Line([7347, 975]),
    MenuLabelSegment::Quad {
        control: [7347, 881],
        to: [7300, 827],
    },
    MenuLabelSegment::Quad {
        control: [7255, 773],
        to: [7170, 773],
    },
    MenuLabelSegment::Quad {
        control: [7117, 773],
        to: [7062, 805],
    },
    MenuLabelSegment::Quad {
        control: [7008, 836],
        to: [6980, 878],
    },
    MenuLabelSegment::Line([6980, 1500]),
    MenuLabelSegment::Line([6831, 1500]),
    MenuLabelSegment::Line([6831, 320]),
    MenuLabelSegment::Line([6980, 320]),
];

const MATCHES_CONTOUR_6: [MenuLabelSegment; 18] = [
    MenuLabelSegment::Line([8239, 489]),
    MenuLabelSegment::Line([8388, 431]),
    MenuLabelSegment::Line([8388, 664]),
    MenuLabelSegment::Line([8618, 664]),
    MenuLabelSegment::Line([8618, 781]),
    MenuLabelSegment::Line([8388, 781]),
    MenuLabelSegment::Line([8388, 1197]),
    MenuLabelSegment::Quad {
        control: [8388, 1302],
        to: [8424, 1347],
    },
    MenuLabelSegment::Quad {
        control: [8458, 1391],
        to: [8538, 1391],
    },
    MenuLabelSegment::Quad {
        control: [8594, 1391],
        to: [8655, 1362],
    },
    MenuLabelSegment::Line([8677, 1492]),
    MenuLabelSegment::Line([8475, 1516]),
    MenuLabelSegment::Quad {
        control: [8375, 1516],
        to: [8308, 1442],
    },
    MenuLabelSegment::Quad {
        control: [8239, 1369],
        to: [8239, 1256],
    },
    MenuLabelSegment::Line([8239, 781]),
    MenuLabelSegment::Line([8142, 781]),
    MenuLabelSegment::Line([8142, 664]),
    MenuLabelSegment::Line([8239, 664]),
];

const MATCHES_CONTOUR_7: [MenuLabelSegment; 13] = [
    MenuLabelSegment::Line([9140, 1642]),
    MenuLabelSegment::Quad {
        control: [9112, 1722],
        to: [9020, 1775],
    },
    MenuLabelSegment::Quad {
        control: [8926, 1828],
        to: [8812, 1828],
    },
    MenuLabelSegment::Line([8812, 1695]),
    MenuLabelSegment::Quad {
        control: [8906, 1695],
        to: [8971, 1653],
    },
    MenuLabelSegment::Quad {
        control: [9038, 1609],
        to: [9038, 1547],
    },
    MenuLabelSegment::Quad {
        control: [9038, 1478],
        to: [9013, 1411],
    },
    MenuLabelSegment::Line([8951, 1245]),
    MenuLabelSegment::Line([8724, 664]),
    MenuLabelSegment::Line([8876, 664]),
    MenuLabelSegment::Line([9123, 1311]),
    MenuLabelSegment::Line([9343, 664]),
    MenuLabelSegment::Line([9495, 664]),
];

const MATCHES_CONTOUR_8: [MenuLabelSegment; 12] = [
    MenuLabelSegment::Line([9760, 733]),
    MenuLabelSegment::Quad {
        control: [9844, 648],
        to: [9963, 648],
    },
    MenuLabelSegment::Quad {
        control: [10141, 648],
        to: [10241, 759],
    },
    MenuLabelSegment::Quad {
        control: [10339, 870],
        to: [10339, 1084],
    },
    MenuLabelSegment::Quad {
        control: [10339, 1275],
        to: [10239, 1395],
    },
    MenuLabelSegment::Quad {
        control: [10139, 1516],
        to: [9950, 1516],
    },
    MenuLabelSegment::Quad {
        control: [9897, 1516],
        to: [9838, 1497],
    },
    MenuLabelSegment::Quad {
        control: [9777, 1478],
        to: [9760, 1455],
    },
    MenuLabelSegment::Line([9760, 1828]),
    MenuLabelSegment::Line([9611, 1828]),
    MenuLabelSegment::Line([9611, 664]),
    MenuLabelSegment::Line([9760, 664]),
];

const MATCHES_CONTOUR_9: [MenuLabelSegment; 8] = [
    MenuLabelSegment::Line([9760, 1331]),
    MenuLabelSegment::Quad {
        control: [9774, 1353],
        to: [9819, 1372],
    },
    MenuLabelSegment::Quad {
        control: [9864, 1391],
        to: [9907, 1391],
    },
    MenuLabelSegment::Quad {
        control: [10183, 1391],
        to: [10183, 1078],
    },
    MenuLabelSegment::Quad {
        control: [10183, 920],
        to: [10118, 847],
    },
    MenuLabelSegment::Quad {
        control: [10052, 773],
        to: [9908, 773],
    },
    MenuLabelSegment::Quad {
        control: [9877, 773],
        to: [9832, 795],
    },
    MenuLabelSegment::Line([9760, 844]),
];

const MATCHES_CONTOUR_10: [MenuLabelSegment; 5] = [
    MenuLabelSegment::Quad {
        control: [10747, 773],
        to: [10679, 838],
    },
    MenuLabelSegment::Quad {
        control: [10615, 898],
        to: [10606, 989],
    },
    MenuLabelSegment::Line([11070, 989]),
    MenuLabelSegment::Quad {
        control: [11070, 898],
        to: [11014, 839],
    },
    MenuLabelSegment::Quad {
        control: [10951, 773],
        to: [10845, 773],
    },
];

const MATCHES_CONTOUR_11: [MenuLabelSegment; 14] = [
    MenuLabelSegment::Quad {
        control: [10754, 1391],
        to: [10865, 1391],
    },
    MenuLabelSegment::Quad {
        control: [10993, 1391],
        to: [11078, 1317],
    },
    MenuLabelSegment::Line([11140, 1423]),
    MenuLabelSegment::Quad {
        control: [11106, 1458],
        to: [11036, 1483],
    },
    MenuLabelSegment::Quad {
        control: [10947, 1516],
        to: [10837, 1516],
    },
    MenuLabelSegment::Quad {
        control: [10679, 1516],
        to: [10568, 1409],
    },
    MenuLabelSegment::Quad {
        control: [10447, 1291],
        to: [10447, 1092],
    },
    MenuLabelSegment::Quad {
        control: [10447, 884],
        to: [10572, 759],
    },
    MenuLabelSegment::Quad {
        control: [10684, 648],
        to: [10839, 648],
    },
    MenuLabelSegment::Quad {
        control: [11017, 648],
        to: [11118, 748],
    },
    MenuLabelSegment::Quad {
        control: [11217, 845],
        to: [11217, 1006],
    },
    MenuLabelSegment::Quad {
        control: [11217, 1055],
        to: [11206, 1097],
    },
    MenuLabelSegment::Line([10603, 1097]),
    MenuLabelSegment::Quad {
        control: [10603, 1244],
        to: [10682, 1322],
    },
];

const MATCHES_CONTOURS: [MenuLabelContour; 12] = [
    MenuLabelContour {
        start: [4282, 725],
        segments: &MATCHES_CONTOUR_0,
    },
    MenuLabelContour {
        start: [4826, 648],
        segments: &MATCHES_CONTOUR_1,
    },
    MenuLabelContour {
        start: [4926, 1073],
        segments: &MATCHES_CONTOUR_2,
    },
    MenuLabelContour {
        start: [5459, 664],
        segments: &MATCHES_CONTOUR_3,
    },
    MenuLabelContour {
        start: [6560, 675],
        segments: &MATCHES_CONTOUR_4,
    },
    MenuLabelContour {
        start: [6980, 320],
        segments: &MATCHES_CONTOUR_5,
    },
    MenuLabelContour {
        start: [8239, 664],
        segments: &MATCHES_CONTOUR_6,
    },
    MenuLabelContour {
        start: [9495, 664],
        segments: &MATCHES_CONTOUR_7,
    },
    MenuLabelContour {
        start: [9760, 664],
        segments: &MATCHES_CONTOUR_8,
    },
    MenuLabelContour {
        start: [9760, 844],
        segments: &MATCHES_CONTOUR_9,
    },
    MenuLabelContour {
        start: [10845, 773],
        segments: &MATCHES_CONTOUR_10,
    },
    MenuLabelContour {
        start: [10682, 1322],
        segments: &MATCHES_CONTOUR_11,
    },
];

pub const MATCHES: MenuLabelDefinition = MenuLabelDefinition {
    text: "match type",
    define_text_id: 51,
    font_id: FONT_ID,
    color_rgb: [255, 255, 255],
    bounds_centipx: [3235, 11970, 320, 1830],
    contours: &MATCHES_CONTOURS,
};

const GRAVITY_CONTOUR_0: [MenuLabelSegment; 32] = [
    MenuLabelSegment::Line([2165, 769]),
    MenuLabelSegment::Quad {
        control: [2222, 842],
        to: [2222, 962],
    },
    MenuLabelSegment::Quad {
        control: [2222, 1089],
        to: [2143, 1175],
    },
    MenuLabelSegment::Quad {
        control: [2065, 1261],
        to: [1937, 1273],
    },
    MenuLabelSegment::Line([1813, 1286]),
    MenuLabelSegment::Line([1755, 1303]),
    MenuLabelSegment::Quad {
        control: [1718, 1317],
        to: [1718, 1341],
    },
    MenuLabelSegment::Quad {
        control: [1718, 1372],
        to: [1794, 1372],
    },
    MenuLabelSegment::Line([1899, 1361]),
    MenuLabelSegment::Line([2005, 1348]),
    MenuLabelSegment::Quad {
        control: [2129, 1348],
        to: [2198, 1408],
    },
    MenuLabelSegment::Quad {
        control: [2266, 1466],
        to: [2266, 1570],
    },
    MenuLabelSegment::Quad {
        control: [2266, 1686],
        to: [2163, 1758],
    },
    MenuLabelSegment::Quad {
        control: [2060, 1828],
        to: [1901, 1828],
    },
    MenuLabelSegment::Quad {
        control: [1819, 1828],
        to: [1730, 1800],
    },
    MenuLabelSegment::Quad {
        control: [1640, 1770],
        to: [1585, 1730],
    },
    MenuLabelSegment::Line([1666, 1611]),
    MenuLabelSegment::Quad {
        control: [1796, 1697],
        to: [1905, 1697],
    },
    MenuLabelSegment::Quad {
        control: [2005, 1697],
        to: [2065, 1662],
    },
    MenuLabelSegment::Quad {
        control: [2122, 1628],
        to: [2122, 1577],
    },
    MenuLabelSegment::Quad {
        control: [2122, 1475],
        to: [1976, 1475],
    },
    MenuLabelSegment::Line([1885, 1488]),
    MenuLabelSegment::Line([1782, 1500]),
    MenuLabelSegment::Quad {
        control: [1604, 1500],
        to: [1604, 1366],
    },
    MenuLabelSegment::Quad {
        control: [1604, 1323],
        to: [1646, 1291],
    },
    MenuLabelSegment::Quad {
        control: [1688, 1256],
        to: [1749, 1242],
    },
    MenuLabelSegment::Quad {
        control: [1572, 1159],
        to: [1572, 955],
    },
    MenuLabelSegment::Quad {
        control: [1572, 823],
        to: [1665, 736],
    },
    MenuLabelSegment::Quad {
        control: [1755, 648],
        to: [1890, 648],
    },
    MenuLabelSegment::Quad {
        control: [2013, 648],
        to: [2083, 698],
    },
    MenuLabelSegment::Line([2157, 609]),
    MenuLabelSegment::Line([2254, 702]),
];

const GRAVITY_CONTOUR_1: [MenuLabelSegment; 8] = [
    MenuLabelSegment::Quad {
        control: [1823, 767],
        to: [1774, 822],
    },
    MenuLabelSegment::Quad {
        control: [1726, 877],
        to: [1726, 955],
    },
    MenuLabelSegment::Quad {
        control: [1726, 1042],
        to: [1773, 1100],
    },
    MenuLabelSegment::Quad {
        control: [1819, 1158],
        to: [1901, 1158],
    },
    MenuLabelSegment::Quad {
        control: [1979, 1158],
        to: [2024, 1102],
    },
    MenuLabelSegment::Quad {
        control: [2068, 1045],
        to: [2068, 955],
    },
    MenuLabelSegment::Quad {
        control: [2068, 877],
        to: [2021, 822],
    },
    MenuLabelSegment::Quad {
        control: [1973, 767],
        to: [1901, 767],
    },
];

const GRAVITY_CONTOUR_2: [MenuLabelSegment; 11] = [
    MenuLabelSegment::Quad {
        control: [2828, 773],
        to: [2778, 773],
    },
    MenuLabelSegment::Quad {
        control: [2699, 773],
        to: [2640, 845],
    },
    MenuLabelSegment::Quad {
        control: [2581, 919],
        to: [2581, 1020],
    },
    MenuLabelSegment::Line([2581, 1500]),
    MenuLabelSegment::Line([2432, 1500]),
    MenuLabelSegment::Line([2432, 664]),
    MenuLabelSegment::Line([2581, 664]),
    MenuLabelSegment::Line([2581, 797]),
    MenuLabelSegment::Quad {
        control: [2662, 648],
        to: [2823, 648],
    },
    MenuLabelSegment::Line([2937, 662]),
    MenuLabelSegment::Line([2876, 806]),
];

const GRAVITY_CONTOUR_3: [MenuLabelSegment; 18] = [
    MenuLabelSegment::Quad {
        control: [3483, 648],
        to: [3566, 731],
    },
    MenuLabelSegment::Quad {
        control: [3648, 814],
        to: [3648, 994],
    },
    MenuLabelSegment::Line([3648, 1294]),
    MenuLabelSegment::Quad {
        control: [3648, 1405],
        to: [3713, 1441],
    },
    MenuLabelSegment::Line([3713, 1516]),
    MenuLabelSegment::Quad {
        control: [3622, 1516],
        to: [3579, 1489],
    },
    MenuLabelSegment::Quad {
        control: [3533, 1464],
        to: [3513, 1405],
    },
    MenuLabelSegment::Quad {
        control: [3424, 1516],
        to: [3241, 1516],
    },
    MenuLabelSegment::Quad {
        control: [3143, 1516],
        to: [3071, 1445],
    },
    MenuLabelSegment::Quad {
        control: [2998, 1373],
        to: [2998, 1267],
    },
    MenuLabelSegment::Quad {
        control: [2998, 1139],
        to: [3110, 1052],
    },
    MenuLabelSegment::Quad {
        control: [3221, 964],
        to: [3393, 964],
    },
    MenuLabelSegment::Line([3499, 984]),
    MenuLabelSegment::Quad {
        control: [3499, 781],
        to: [3318, 781],
    },
    MenuLabelSegment::Quad {
        control: [3179, 781],
        to: [3104, 856],
    },
    MenuLabelSegment::Line([3041, 731]),
    MenuLabelSegment::Quad {
        control: [3083, 697],
        to: [3158, 673],
    },
    MenuLabelSegment::Quad {
        control: [3233, 648],
        to: [3301, 648],
    },
];

const GRAVITY_CONTOUR_4: [MenuLabelSegment; 6] = [
    MenuLabelSegment::Quad {
        control: [3288, 1073],
        to: [3218, 1131],
    },
    MenuLabelSegment::Quad {
        control: [3146, 1189],
        to: [3146, 1269],
    },
    MenuLabelSegment::Quad {
        control: [3146, 1398],
        to: [3301, 1398],
    },
    MenuLabelSegment::Quad {
        control: [3413, 1398],
        to: [3499, 1292],
    },
    MenuLabelSegment::Line([3499, 1089]),
    MenuLabelSegment::Line([3401, 1073]),
];

const GRAVITY_CONTOUR_5: [MenuLabelSegment; 7] = [
    MenuLabelSegment::Line([4183, 1516]),
    MenuLabelSegment::Line([4144, 1516]),
    MenuLabelSegment::Line([3784, 661]),
    MenuLabelSegment::Line([3947, 661]),
    MenuLabelSegment::Line([4169, 1247]),
    MenuLabelSegment::Line([4394, 661]),
    MenuLabelSegment::Line([4550, 661]),
];

const GRAVITY_CONTOUR_6: [MenuLabelSegment; 8] = [
    MenuLabelSegment::Quad {
        control: [4857, 347],
        to: [4885, 375],
    },
    MenuLabelSegment::Quad {
        control: [4912, 402],
        to: [4912, 439],
    },
    MenuLabelSegment::Quad {
        control: [4912, 477],
        to: [4885, 505],
    },
    MenuLabelSegment::Quad {
        control: [4857, 531],
        to: [4819, 531],
    },
    MenuLabelSegment::Quad {
        control: [4782, 531],
        to: [4755, 505],
    },
    MenuLabelSegment::Quad {
        control: [4727, 477],
        to: [4727, 439],
    },
    MenuLabelSegment::Quad {
        control: [4727, 400],
        to: [4754, 373],
    },
    MenuLabelSegment::Quad {
        control: [4780, 347],
        to: [4819, 347],
    },
];

const GRAVITY_CONTOUR_7: [MenuLabelSegment; 6] = [
    MenuLabelSegment::Line([4887, 1500]),
    MenuLabelSegment::Line([4738, 1500]),
    MenuLabelSegment::Line([4738, 789]),
    MenuLabelSegment::Line([4622, 789]),
    MenuLabelSegment::Line([4622, 664]),
    MenuLabelSegment::Line([4887, 664]),
];

const GRAVITY_CONTOUR_8: [MenuLabelSegment; 18] = [
    MenuLabelSegment::Line([5174, 489]),
    MenuLabelSegment::Line([5323, 431]),
    MenuLabelSegment::Line([5323, 664]),
    MenuLabelSegment::Line([5552, 664]),
    MenuLabelSegment::Line([5552, 781]),
    MenuLabelSegment::Line([5323, 781]),
    MenuLabelSegment::Line([5323, 1197]),
    MenuLabelSegment::Quad {
        control: [5323, 1302],
        to: [5359, 1347],
    },
    MenuLabelSegment::Quad {
        control: [5393, 1391],
        to: [5473, 1391],
    },
    MenuLabelSegment::Quad {
        control: [5529, 1391],
        to: [5590, 1362],
    },
    MenuLabelSegment::Line([5612, 1492]),
    MenuLabelSegment::Line([5410, 1516]),
    MenuLabelSegment::Quad {
        control: [5310, 1516],
        to: [5243, 1442],
    },
    MenuLabelSegment::Quad {
        control: [5174, 1369],
        to: [5174, 1256],
    },
    MenuLabelSegment::Line([5174, 781]),
    MenuLabelSegment::Line([5078, 781]),
    MenuLabelSegment::Line([5078, 664]),
    MenuLabelSegment::Line([5174, 664]),
];

const GRAVITY_CONTOUR_9: [MenuLabelSegment; 13] = [
    MenuLabelSegment::Line([6075, 1642]),
    MenuLabelSegment::Quad {
        control: [6047, 1722],
        to: [5955, 1775],
    },
    MenuLabelSegment::Quad {
        control: [5861, 1828],
        to: [5747, 1828],
    },
    MenuLabelSegment::Line([5747, 1695]),
    MenuLabelSegment::Quad {
        control: [5841, 1695],
        to: [5906, 1653],
    },
    MenuLabelSegment::Quad {
        control: [5973, 1609],
        to: [5973, 1547],
    },
    MenuLabelSegment::Quad {
        control: [5973, 1478],
        to: [5948, 1411],
    },
    MenuLabelSegment::Line([5886, 1245]),
    MenuLabelSegment::Line([5659, 664]),
    MenuLabelSegment::Line([5811, 664]),
    MenuLabelSegment::Line([6058, 1311]),
    MenuLabelSegment::Line([6278, 664]),
    MenuLabelSegment::Line([6430, 664]),
];

const GRAVITY_CONTOUR_10: [MenuLabelSegment; 22] = [
    MenuLabelSegment::Line([7436, 844]),
    MenuLabelSegment::Quad {
        control: [7348, 773],
        to: [7259, 773],
    },
    MenuLabelSegment::Quad {
        control: [7206, 773],
        to: [7170, 798],
    },
    MenuLabelSegment::Quad {
        control: [7132, 823],
        to: [7132, 861],
    },
    MenuLabelSegment::Quad {
        control: [7132, 942],
        to: [7225, 983],
    },
    MenuLabelSegment::Line([7331, 1031]),
    MenuLabelSegment::Quad {
        control: [7428, 1077],
        to: [7473, 1133],
    },
    MenuLabelSegment::Quad {
        control: [7517, 1191],
        to: [7517, 1277],
    },
    MenuLabelSegment::Quad {
        control: [7517, 1389],
        to: [7439, 1453],
    },
    MenuLabelSegment::Quad {
        control: [7359, 1516],
        to: [7220, 1516],
    },
    MenuLabelSegment::Quad {
        control: [7087, 1516],
        to: [6972, 1450],
    },
    MenuLabelSegment::Line([7023, 1309]),
    MenuLabelSegment::Quad {
        control: [7148, 1391],
        to: [7223, 1391],
    },
    MenuLabelSegment::Quad {
        control: [7361, 1391],
        to: [7361, 1275],
    },
    MenuLabelSegment::Quad {
        control: [7361, 1192],
        to: [7228, 1133],
    },
    MenuLabelSegment::Quad {
        control: [7126, 1086],
        to: [7090, 1062],
    },
    MenuLabelSegment::Quad {
        control: [7054, 1038],
        to: [7029, 1008],
    },
    MenuLabelSegment::Quad {
        control: [7003, 977],
        to: [6990, 942],
    },
    MenuLabelSegment::Quad {
        control: [6976, 906],
        to: [6976, 867],
    },
    MenuLabelSegment::Quad {
        control: [6976, 764],
        to: [7051, 706],
    },
    MenuLabelSegment::Quad {
        control: [7126, 648],
        to: [7248, 648],
    },
    MenuLabelSegment::Quad {
        control: [7339, 648],
        to: [7478, 706],
    },
];

const GRAVITY_CONTOUR_11: [MenuLabelSegment; 18] = [
    MenuLabelSegment::Line([7729, 489]),
    MenuLabelSegment::Line([7878, 431]),
    MenuLabelSegment::Line([7878, 664]),
    MenuLabelSegment::Line([8107, 664]),
    MenuLabelSegment::Line([8107, 781]),
    MenuLabelSegment::Line([7878, 781]),
    MenuLabelSegment::Line([7878, 1197]),
    MenuLabelSegment::Quad {
        control: [7878, 1302],
        to: [7914, 1347],
    },
    MenuLabelSegment::Quad {
        control: [7948, 1391],
        to: [8028, 1391],
    },
    MenuLabelSegment::Quad {
        control: [8084, 1391],
        to: [8145, 1362],
    },
    MenuLabelSegment::Line([8167, 1492]),
    MenuLabelSegment::Line([7965, 1516]),
    MenuLabelSegment::Quad {
        control: [7865, 1516],
        to: [7798, 1442],
    },
    MenuLabelSegment::Quad {
        control: [7729, 1369],
        to: [7729, 1256],
    },
    MenuLabelSegment::Line([7729, 781]),
    MenuLabelSegment::Line([7632, 781]),
    MenuLabelSegment::Line([7632, 664]),
    MenuLabelSegment::Line([7729, 664]),
];

const GRAVITY_CONTOUR_12: [MenuLabelSegment; 11] = [
    MenuLabelSegment::Quad {
        control: [8717, 773],
        to: [8667, 773],
    },
    MenuLabelSegment::Quad {
        control: [8589, 773],
        to: [8530, 845],
    },
    MenuLabelSegment::Quad {
        control: [8471, 919],
        to: [8471, 1020],
    },
    MenuLabelSegment::Line([8471, 1500]),
    MenuLabelSegment::Line([8322, 1500]),
    MenuLabelSegment::Line([8322, 664]),
    MenuLabelSegment::Line([8471, 664]),
    MenuLabelSegment::Line([8471, 797]),
    MenuLabelSegment::Quad {
        control: [8552, 648],
        to: [8713, 648],
    },
    MenuLabelSegment::Line([8827, 662]),
    MenuLabelSegment::Line([8766, 806]),
];

const GRAVITY_CONTOUR_13: [MenuLabelSegment; 5] = [
    MenuLabelSegment::Quad {
        control: [9177, 773],
        to: [9109, 838],
    },
    MenuLabelSegment::Quad {
        control: [9045, 898],
        to: [9036, 989],
    },
    MenuLabelSegment::Line([9500, 989]),
    MenuLabelSegment::Quad {
        control: [9500, 898],
        to: [9444, 839],
    },
    MenuLabelSegment::Quad {
        control: [9381, 773],
        to: [9275, 773],
    },
];

const GRAVITY_CONTOUR_14: [MenuLabelSegment; 14] = [
    MenuLabelSegment::Quad {
        control: [9184, 1391],
        to: [9295, 1391],
    },
    MenuLabelSegment::Quad {
        control: [9423, 1391],
        to: [9508, 1317],
    },
    MenuLabelSegment::Line([9570, 1423]),
    MenuLabelSegment::Quad {
        control: [9536, 1458],
        to: [9466, 1483],
    },
    MenuLabelSegment::Quad {
        control: [9377, 1516],
        to: [9267, 1516],
    },
    MenuLabelSegment::Quad {
        control: [9109, 1516],
        to: [8998, 1409],
    },
    MenuLabelSegment::Quad {
        control: [8877, 1291],
        to: [8877, 1092],
    },
    MenuLabelSegment::Quad {
        control: [8877, 884],
        to: [9002, 759],
    },
    MenuLabelSegment::Quad {
        control: [9114, 648],
        to: [9269, 648],
    },
    MenuLabelSegment::Quad {
        control: [9447, 648],
        to: [9548, 748],
    },
    MenuLabelSegment::Quad {
        control: [9647, 845],
        to: [9647, 1006],
    },
    MenuLabelSegment::Quad {
        control: [9647, 1055],
        to: [9636, 1097],
    },
    MenuLabelSegment::Line([9033, 1097]),
    MenuLabelSegment::Quad {
        control: [9033, 1244],
        to: [9112, 1322],
    },
];

const GRAVITY_CONTOUR_15: [MenuLabelSegment; 14] = [
    MenuLabelSegment::Line([10469, 1500]),
    MenuLabelSegment::Line([10320, 1500]),
    MenuLabelSegment::Line([10320, 1014]),
    MenuLabelSegment::Quad {
        control: [10320, 880],
        to: [10281, 827],
    },
    MenuLabelSegment::Quad {
        control: [10241, 773],
        to: [10145, 773],
    },
    MenuLabelSegment::Quad {
        control: [10095, 773],
        to: [10039, 803],
    },
    MenuLabelSegment::Quad {
        control: [9984, 834],
        to: [9955, 878],
    },
    MenuLabelSegment::Line([9955, 1500]),
    MenuLabelSegment::Line([9806, 1500]),
    MenuLabelSegment::Line([9806, 664]),
    MenuLabelSegment::Line([9908, 664]),
    MenuLabelSegment::Line([9955, 772]),
    MenuLabelSegment::Quad {
        control: [10028, 648],
        to: [10194, 648],
    },
    MenuLabelSegment::Quad {
        control: [10469, 648],
        to: [10469, 983],
    },
];

const GRAVITY_CONTOUR_16: [MenuLabelSegment; 32] = [
    MenuLabelSegment::Line([11230, 769]),
    MenuLabelSegment::Quad {
        control: [11287, 842],
        to: [11287, 962],
    },
    MenuLabelSegment::Quad {
        control: [11287, 1089],
        to: [11208, 1175],
    },
    MenuLabelSegment::Quad {
        control: [11130, 1261],
        to: [11002, 1273],
    },
    MenuLabelSegment::Line([10878, 1286]),
    MenuLabelSegment::Line([10820, 1303]),
    MenuLabelSegment::Quad {
        control: [10783, 1317],
        to: [10783, 1341],
    },
    MenuLabelSegment::Quad {
        control: [10783, 1372],
        to: [10859, 1372],
    },
    MenuLabelSegment::Line([10964, 1361]),
    MenuLabelSegment::Line([11070, 1348]),
    MenuLabelSegment::Quad {
        control: [11194, 1348],
        to: [11262, 1408],
    },
    MenuLabelSegment::Quad {
        control: [11331, 1466],
        to: [11331, 1570],
    },
    MenuLabelSegment::Quad {
        control: [11331, 1686],
        to: [11228, 1758],
    },
    MenuLabelSegment::Quad {
        control: [11125, 1828],
        to: [10966, 1828],
    },
    MenuLabelSegment::Quad {
        control: [10884, 1828],
        to: [10795, 1800],
    },
    MenuLabelSegment::Quad {
        control: [10705, 1770],
        to: [10650, 1730],
    },
    MenuLabelSegment::Line([10731, 1611]),
    MenuLabelSegment::Quad {
        control: [10861, 1697],
        to: [10970, 1697],
    },
    MenuLabelSegment::Quad {
        control: [11070, 1697],
        to: [11130, 1662],
    },
    MenuLabelSegment::Quad {
        control: [11187, 1628],
        to: [11187, 1577],
    },
    MenuLabelSegment::Quad {
        control: [11187, 1475],
        to: [11041, 1475],
    },
    MenuLabelSegment::Line([10950, 1488]),
    MenuLabelSegment::Line([10847, 1500]),
    MenuLabelSegment::Quad {
        control: [10669, 1500],
        to: [10669, 1366],
    },
    MenuLabelSegment::Quad {
        control: [10669, 1323],
        to: [10711, 1291],
    },
    MenuLabelSegment::Quad {
        control: [10753, 1256],
        to: [10814, 1242],
    },
    MenuLabelSegment::Quad {
        control: [10637, 1159],
        to: [10637, 955],
    },
    MenuLabelSegment::Quad {
        control: [10637, 823],
        to: [10730, 736],
    },
    MenuLabelSegment::Quad {
        control: [10820, 648],
        to: [10955, 648],
    },
    MenuLabelSegment::Quad {
        control: [11078, 648],
        to: [11148, 698],
    },
    MenuLabelSegment::Line([11222, 609]),
    MenuLabelSegment::Line([11319, 702]),
];

const GRAVITY_CONTOUR_17: [MenuLabelSegment; 8] = [
    MenuLabelSegment::Quad {
        control: [10887, 767],
        to: [10839, 822],
    },
    MenuLabelSegment::Quad {
        control: [10791, 877],
        to: [10791, 955],
    },
    MenuLabelSegment::Quad {
        control: [10791, 1042],
        to: [10837, 1100],
    },
    MenuLabelSegment::Quad {
        control: [10884, 1158],
        to: [10966, 1158],
    },
    MenuLabelSegment::Quad {
        control: [11044, 1158],
        to: [11089, 1102],
    },
    MenuLabelSegment::Quad {
        control: [11133, 1045],
        to: [11133, 955],
    },
    MenuLabelSegment::Quad {
        control: [11133, 877],
        to: [11086, 822],
    },
    MenuLabelSegment::Quad {
        control: [11037, 767],
        to: [10966, 767],
    },
];

const GRAVITY_CONTOUR_18: [MenuLabelSegment; 16] = [
    MenuLabelSegment::Line([11635, 756]),
    MenuLabelSegment::Quad {
        control: [11664, 709],
        to: [11730, 680],
    },
    MenuLabelSegment::Quad {
        control: [11796, 648],
        to: [11866, 648],
    },
    MenuLabelSegment::Quad {
        control: [11999, 648],
        to: [12075, 736],
    },
    MenuLabelSegment::Quad {
        control: [12150, 823],
        to: [12150, 975],
    },
    MenuLabelSegment::Line([12150, 1500]),
    MenuLabelSegment::Line([12002, 1500]),
    MenuLabelSegment::Line([12002, 975]),
    MenuLabelSegment::Quad {
        control: [12002, 881],
        to: [11955, 827],
    },
    MenuLabelSegment::Quad {
        control: [11910, 773],
        to: [11825, 773],
    },
    MenuLabelSegment::Quad {
        control: [11772, 773],
        to: [11717, 805],
    },
    MenuLabelSegment::Quad {
        control: [11663, 836],
        to: [11635, 878],
    },
    MenuLabelSegment::Line([11635, 1500]),
    MenuLabelSegment::Line([11486, 1500]),
    MenuLabelSegment::Line([11486, 320]),
    MenuLabelSegment::Line([11635, 320]),
];

const GRAVITY_CONTOUR_19: [MenuLabelSegment; 18] = [
    MenuLabelSegment::Line([12414, 489]),
    MenuLabelSegment::Line([12563, 431]),
    MenuLabelSegment::Line([12563, 664]),
    MenuLabelSegment::Line([12792, 664]),
    MenuLabelSegment::Line([12792, 781]),
    MenuLabelSegment::Line([12563, 781]),
    MenuLabelSegment::Line([12563, 1197]),
    MenuLabelSegment::Quad {
        control: [12563, 1302],
        to: [12599, 1347],
    },
    MenuLabelSegment::Quad {
        control: [12633, 1391],
        to: [12713, 1391],
    },
    MenuLabelSegment::Quad {
        control: [12769, 1391],
        to: [12830, 1362],
    },
    MenuLabelSegment::Line([12852, 1492]),
    MenuLabelSegment::Line([12650, 1516]),
    MenuLabelSegment::Quad {
        control: [12550, 1516],
        to: [12483, 1442],
    },
    MenuLabelSegment::Quad {
        control: [12414, 1369],
        to: [12414, 1256],
    },
    MenuLabelSegment::Line([12414, 781]),
    MenuLabelSegment::Line([12317, 781]),
    MenuLabelSegment::Line([12317, 664]),
    MenuLabelSegment::Line([12414, 664]),
];

const GRAVITY_CONTOURS: [MenuLabelContour; 20] = [
    MenuLabelContour {
        start: [2254, 702],
        segments: &GRAVITY_CONTOUR_0,
    },
    MenuLabelContour {
        start: [1901, 767],
        segments: &GRAVITY_CONTOUR_1,
    },
    MenuLabelContour {
        start: [2876, 806],
        segments: &GRAVITY_CONTOUR_2,
    },
    MenuLabelContour {
        start: [3301, 648],
        segments: &GRAVITY_CONTOUR_3,
    },
    MenuLabelContour {
        start: [3401, 1073],
        segments: &GRAVITY_CONTOUR_4,
    },
    MenuLabelContour {
        start: [4550, 661],
        segments: &GRAVITY_CONTOUR_5,
    },
    MenuLabelContour {
        start: [4819, 347],
        segments: &GRAVITY_CONTOUR_6,
    },
    MenuLabelContour {
        start: [4887, 664],
        segments: &GRAVITY_CONTOUR_7,
    },
    MenuLabelContour {
        start: [5174, 664],
        segments: &GRAVITY_CONTOUR_8,
    },
    MenuLabelContour {
        start: [6430, 664],
        segments: &GRAVITY_CONTOUR_9,
    },
    MenuLabelContour {
        start: [7478, 706],
        segments: &GRAVITY_CONTOUR_10,
    },
    MenuLabelContour {
        start: [7729, 664],
        segments: &GRAVITY_CONTOUR_11,
    },
    MenuLabelContour {
        start: [8766, 806],
        segments: &GRAVITY_CONTOUR_12,
    },
    MenuLabelContour {
        start: [9275, 773],
        segments: &GRAVITY_CONTOUR_13,
    },
    MenuLabelContour {
        start: [9112, 1322],
        segments: &GRAVITY_CONTOUR_14,
    },
    MenuLabelContour {
        start: [10469, 983],
        segments: &GRAVITY_CONTOUR_15,
    },
    MenuLabelContour {
        start: [11319, 702],
        segments: &GRAVITY_CONTOUR_16,
    },
    MenuLabelContour {
        start: [10966, 767],
        segments: &GRAVITY_CONTOUR_17,
    },
    MenuLabelContour {
        start: [11635, 320],
        segments: &GRAVITY_CONTOUR_18,
    },
    MenuLabelContour {
        start: [12414, 664],
        segments: &GRAVITY_CONTOUR_19,
    },
];

pub const GRAVITY: MenuLabelDefinition = MenuLabelDefinition {
    text: "gravity strenght",
    define_text_id: 65,
    font_id: FONT_ID,
    color_rgb: [255, 255, 255],
    bounds_centipx: [1575, 13590, 320, 1830],
    contours: &GRAVITY_CONTOURS,
};

const SPEED_CONTOUR_0: [MenuLabelSegment; 27] = [
    MenuLabelSegment::Quad {
        control: [2697, 747],
        to: [2726, 770],
    },
    MenuLabelSegment::Quad {
        control: [2756, 795],
        to: [2847, 839],
    },
    MenuLabelSegment::Line([2940, 883]),
    MenuLabelSegment::Quad {
        control: [3058, 939],
        to: [3104, 1016],
    },
    MenuLabelSegment::Quad {
        control: [3151, 1092],
        to: [3151, 1211],
    },
    MenuLabelSegment::Quad {
        control: [3151, 1341],
        to: [3048, 1430],
    },
    MenuLabelSegment::Quad {
        control: [2945, 1520],
        to: [2772, 1520],
    },
    MenuLabelSegment::Quad {
        control: [2618, 1520],
        to: [2511, 1448],
    },
    MenuLabelSegment::Line([2568, 1305]),
    MenuLabelSegment::Quad {
        control: [2612, 1336],
        to: [2678, 1358],
    },
    MenuLabelSegment::Quad {
        control: [2742, 1380],
        to: [2793, 1380],
    },
    MenuLabelSegment::Quad {
        control: [2886, 1380],
        to: [2940, 1330],
    },
    MenuLabelSegment::Quad {
        control: [2997, 1280],
        to: [2997, 1202],
    },
    MenuLabelSegment::Quad {
        control: [2997, 1144],
        to: [2965, 1095],
    },
    MenuLabelSegment::Quad {
        control: [2934, 1045],
        to: [2811, 986],
    },
    MenuLabelSegment::Line([2718, 944]),
    MenuLabelSegment::Quad {
        control: [2601, 889],
        to: [2554, 814],
    },
    MenuLabelSegment::Quad {
        control: [2508, 739],
        to: [2508, 633],
    },
    MenuLabelSegment::Quad {
        control: [2508, 505],
        to: [2598, 420],
    },
    MenuLabelSegment::Quad {
        control: [2689, 336],
        to: [2831, 336],
    },
    MenuLabelSegment::Quad {
        control: [3022, 336],
        to: [3095, 398],
    },
    MenuLabelSegment::Line([3050, 534]),
    MenuLabelSegment::Quad {
        control: [3018, 511],
        to: [2954, 491],
    },
    MenuLabelSegment::Line([2836, 469]),
    MenuLabelSegment::Quad {
        control: [2756, 469],
        to: [2711, 514],
    },
    MenuLabelSegment::Quad {
        control: [2664, 559],
        to: [2664, 631],
    },
    MenuLabelSegment::Quad {
        control: [2664, 675],
        to: [2681, 711],
    },
];

const SPEED_CONTOUR_1: [MenuLabelSegment; 12] = [
    MenuLabelSegment::Line([3470, 733]),
    MenuLabelSegment::Quad {
        control: [3554, 648],
        to: [3673, 648],
    },
    MenuLabelSegment::Quad {
        control: [3851, 648],
        to: [3951, 759],
    },
    MenuLabelSegment::Quad {
        control: [4049, 870],
        to: [4049, 1084],
    },
    MenuLabelSegment::Quad {
        control: [4049, 1275],
        to: [3949, 1395],
    },
    MenuLabelSegment::Quad {
        control: [3849, 1516],
        to: [3660, 1516],
    },
    MenuLabelSegment::Quad {
        control: [3607, 1516],
        to: [3548, 1497],
    },
    MenuLabelSegment::Quad {
        control: [3487, 1478],
        to: [3470, 1455],
    },
    MenuLabelSegment::Line([3470, 1828]),
    MenuLabelSegment::Line([3321, 1828]),
    MenuLabelSegment::Line([3321, 664]),
    MenuLabelSegment::Line([3470, 664]),
];

const SPEED_CONTOUR_2: [MenuLabelSegment; 8] = [
    MenuLabelSegment::Line([3470, 1331]),
    MenuLabelSegment::Quad {
        control: [3484, 1353],
        to: [3529, 1372],
    },
    MenuLabelSegment::Quad {
        control: [3574, 1391],
        to: [3617, 1391],
    },
    MenuLabelSegment::Quad {
        control: [3893, 1391],
        to: [3893, 1078],
    },
    MenuLabelSegment::Quad {
        control: [3893, 920],
        to: [3828, 847],
    },
    MenuLabelSegment::Quad {
        control: [3762, 773],
        to: [3618, 773],
    },
    MenuLabelSegment::Quad {
        control: [3587, 773],
        to: [3542, 795],
    },
    MenuLabelSegment::Line([3470, 844]),
];

const SPEED_CONTOUR_3: [MenuLabelSegment; 5] = [
    MenuLabelSegment::Quad {
        control: [4457, 773],
        to: [4389, 838],
    },
    MenuLabelSegment::Quad {
        control: [4325, 898],
        to: [4316, 989],
    },
    MenuLabelSegment::Line([4780, 989]),
    MenuLabelSegment::Quad {
        control: [4780, 898],
        to: [4724, 839],
    },
    MenuLabelSegment::Quad {
        control: [4661, 773],
        to: [4555, 773],
    },
];

const SPEED_CONTOUR_4: [MenuLabelSegment; 14] = [
    MenuLabelSegment::Quad {
        control: [4464, 1391],
        to: [4575, 1391],
    },
    MenuLabelSegment::Quad {
        control: [4703, 1391],
        to: [4788, 1317],
    },
    MenuLabelSegment::Line([4850, 1423]),
    MenuLabelSegment::Quad {
        control: [4816, 1458],
        to: [4746, 1483],
    },
    MenuLabelSegment::Quad {
        control: [4657, 1516],
        to: [4547, 1516],
    },
    MenuLabelSegment::Quad {
        control: [4389, 1516],
        to: [4278, 1409],
    },
    MenuLabelSegment::Quad {
        control: [4157, 1291],
        to: [4157, 1092],
    },
    MenuLabelSegment::Quad {
        control: [4157, 884],
        to: [4282, 759],
    },
    MenuLabelSegment::Quad {
        control: [4394, 648],
        to: [4549, 648],
    },
    MenuLabelSegment::Quad {
        control: [4727, 648],
        to: [4828, 748],
    },
    MenuLabelSegment::Quad {
        control: [4927, 845],
        to: [4927, 1006],
    },
    MenuLabelSegment::Quad {
        control: [4927, 1055],
        to: [4916, 1097],
    },
    MenuLabelSegment::Line([4313, 1097]),
    MenuLabelSegment::Quad {
        control: [4313, 1244],
        to: [4392, 1322],
    },
];

const SPEED_CONTOUR_5: [MenuLabelSegment; 5] = [
    MenuLabelSegment::Quad {
        control: [5332, 773],
        to: [5264, 838],
    },
    MenuLabelSegment::Quad {
        control: [5200, 898],
        to: [5191, 989],
    },
    MenuLabelSegment::Line([5655, 989]),
    MenuLabelSegment::Quad {
        control: [5655, 898],
        to: [5599, 839],
    },
    MenuLabelSegment::Quad {
        control: [5536, 773],
        to: [5430, 773],
    },
];

const SPEED_CONTOUR_6: [MenuLabelSegment; 14] = [
    MenuLabelSegment::Quad {
        control: [5339, 1391],
        to: [5450, 1391],
    },
    MenuLabelSegment::Quad {
        control: [5578, 1391],
        to: [5663, 1317],
    },
    MenuLabelSegment::Line([5725, 1423]),
    MenuLabelSegment::Quad {
        control: [5691, 1458],
        to: [5621, 1483],
    },
    MenuLabelSegment::Quad {
        control: [5532, 1516],
        to: [5422, 1516],
    },
    MenuLabelSegment::Quad {
        control: [5264, 1516],
        to: [5153, 1409],
    },
    MenuLabelSegment::Quad {
        control: [5032, 1291],
        to: [5032, 1092],
    },
    MenuLabelSegment::Quad {
        control: [5032, 884],
        to: [5157, 759],
    },
    MenuLabelSegment::Quad {
        control: [5269, 648],
        to: [5424, 648],
    },
    MenuLabelSegment::Quad {
        control: [5602, 648],
        to: [5703, 748],
    },
    MenuLabelSegment::Quad {
        control: [5802, 845],
        to: [5802, 1006],
    },
    MenuLabelSegment::Quad {
        control: [5802, 1055],
        to: [5791, 1097],
    },
    MenuLabelSegment::Line([5188, 1097]),
    MenuLabelSegment::Quad {
        control: [5188, 1244],
        to: [5268, 1322],
    },
];

const SPEED_CONTOUR_7: [MenuLabelSegment; 11] = [
    MenuLabelSegment::Line([6639, 320]),
    MenuLabelSegment::Line([6639, 1500]),
    MenuLabelSegment::Line([6491, 1500]),
    MenuLabelSegment::Line([6491, 1438]),
    MenuLabelSegment::Quad {
        control: [6414, 1516],
        to: [6266, 1516],
    },
    MenuLabelSegment::Quad {
        control: [6110, 1516],
        to: [6011, 1403],
    },
    MenuLabelSegment::Quad {
        control: [5914, 1291],
        to: [5914, 1103],
    },
    MenuLabelSegment::Quad {
        control: [5914, 914],
        to: [6027, 781],
    },
    MenuLabelSegment::Quad {
        control: [6139, 648],
        to: [6294, 648],
    },
    MenuLabelSegment::Quad {
        control: [6424, 648],
        to: [6491, 709],
    },
    MenuLabelSegment::Line([6491, 320]),
];

const SPEED_CONTOUR_8: [MenuLabelSegment; 7] = [
    MenuLabelSegment::Quad {
        control: [6478, 1347],
        to: [6491, 1322],
    },
    MenuLabelSegment::Line([6491, 858]),
    MenuLabelSegment::Quad {
        control: [6435, 773],
        to: [6338, 773],
    },
    MenuLabelSegment::Quad {
        control: [6218, 773],
        to: [6144, 862],
    },
    MenuLabelSegment::Quad {
        control: [6071, 952],
        to: [6071, 1089],
    },
    MenuLabelSegment::Quad {
        control: [6071, 1391],
        to: [6346, 1391],
    },
    MenuLabelSegment::Quad {
        control: [6380, 1391],
        to: [6430, 1369],
    },
];

const SPEED_CONTOUR_9: [MenuLabelSegment; 19] = [
    MenuLabelSegment::Quad {
        control: [7473, 477],
        to: [7359, 477],
    },
    MenuLabelSegment::Quad {
        control: [7179, 477],
        to: [7076, 603],
    },
    MenuLabelSegment::Quad {
        control: [6972, 730],
        to: [6972, 939],
    },
    MenuLabelSegment::Quad {
        control: [6972, 1138],
        to: [7075, 1258],
    },
    MenuLabelSegment::Quad {
        control: [7176, 1380],
        to: [7351, 1380],
    },
    MenuLabelSegment::Quad {
        control: [7476, 1380],
        to: [7554, 1305],
    },
    MenuLabelSegment::Line([7554, 1036]),
    MenuLabelSegment::Line([7395, 1036]),
    MenuLabelSegment::Line([7395, 903]),
    MenuLabelSegment::Line([7711, 903]),
    MenuLabelSegment::Line([7711, 1402]),
    MenuLabelSegment::Quad {
        control: [7648, 1455],
        to: [7536, 1488],
    },
    MenuLabelSegment::Quad {
        control: [7425, 1520],
        to: [7318, 1520],
    },
    MenuLabelSegment::Quad {
        control: [7082, 1520],
        to: [6945, 1359],
    },
    MenuLabelSegment::Quad {
        control: [6807, 1198],
        to: [6807, 933],
    },
    MenuLabelSegment::Quad {
        control: [6807, 667],
        to: [6959, 502],
    },
    MenuLabelSegment::Quad {
        control: [7109, 336],
        to: [7364, 336],
    },
    MenuLabelSegment::Quad {
        control: [7543, 336],
        to: [7656, 436],
    },
    MenuLabelSegment::Line([7590, 564]),
];

const SPEED_CONTOUR_10: [MenuLabelSegment; 11] = [
    MenuLabelSegment::Quad {
        control: [8338, 773],
        to: [8288, 773],
    },
    MenuLabelSegment::Quad {
        control: [8209, 773],
        to: [8150, 845],
    },
    MenuLabelSegment::Quad {
        control: [8091, 919],
        to: [8091, 1020],
    },
    MenuLabelSegment::Line([8091, 1500]),
    MenuLabelSegment::Line([7942, 1500]),
    MenuLabelSegment::Line([7942, 664]),
    MenuLabelSegment::Line([8091, 664]),
    MenuLabelSegment::Line([8091, 797]),
    MenuLabelSegment::Quad {
        control: [8172, 648],
        to: [8333, 648],
    },
    MenuLabelSegment::Line([8447, 662]),
    MenuLabelSegment::Line([8386, 806]),
];

const SPEED_CONTOUR_11: [MenuLabelSegment; 18] = [
    MenuLabelSegment::Quad {
        control: [8993, 648],
        to: [9076, 731],
    },
    MenuLabelSegment::Quad {
        control: [9157, 814],
        to: [9157, 994],
    },
    MenuLabelSegment::Line([9157, 1294]),
    MenuLabelSegment::Quad {
        control: [9157, 1405],
        to: [9223, 1441],
    },
    MenuLabelSegment::Line([9223, 1516]),
    MenuLabelSegment::Quad {
        control: [9132, 1516],
        to: [9089, 1489],
    },
    MenuLabelSegment::Quad {
        control: [9043, 1464],
        to: [9023, 1405],
    },
    MenuLabelSegment::Quad {
        control: [8934, 1516],
        to: [8751, 1516],
    },
    MenuLabelSegment::Quad {
        control: [8653, 1516],
        to: [8581, 1445],
    },
    MenuLabelSegment::Quad {
        control: [8507, 1373],
        to: [8507, 1267],
    },
    MenuLabelSegment::Quad {
        control: [8507, 1139],
        to: [8620, 1052],
    },
    MenuLabelSegment::Quad {
        control: [8731, 964],
        to: [8903, 964],
    },
    MenuLabelSegment::Line([9009, 984]),
    MenuLabelSegment::Quad {
        control: [9009, 781],
        to: [8828, 781],
    },
    MenuLabelSegment::Quad {
        control: [8689, 781],
        to: [8614, 856],
    },
    MenuLabelSegment::Line([8551, 731]),
    MenuLabelSegment::Quad {
        control: [8593, 697],
        to: [8668, 673],
    },
    MenuLabelSegment::Quad {
        control: [8743, 648],
        to: [8811, 648],
    },
];

const SPEED_CONTOUR_12: [MenuLabelSegment; 6] = [
    MenuLabelSegment::Quad {
        control: [8798, 1073],
        to: [8728, 1131],
    },
    MenuLabelSegment::Quad {
        control: [8656, 1189],
        to: [8656, 1269],
    },
    MenuLabelSegment::Quad {
        control: [8656, 1398],
        to: [8811, 1398],
    },
    MenuLabelSegment::Quad {
        control: [8923, 1398],
        to: [9009, 1292],
    },
    MenuLabelSegment::Line([9009, 1089]),
    MenuLabelSegment::Line([8911, 1073]),
];

const SPEED_CONTOUR_13: [MenuLabelSegment; 7] = [
    MenuLabelSegment::Line([9693, 1516]),
    MenuLabelSegment::Line([9654, 1516]),
    MenuLabelSegment::Line([9294, 661]),
    MenuLabelSegment::Line([9457, 661]),
    MenuLabelSegment::Line([9679, 1247]),
    MenuLabelSegment::Line([9904, 661]),
    MenuLabelSegment::Line([10060, 661]),
];

const SPEED_CONTOUR_14: [MenuLabelSegment; 8] = [
    MenuLabelSegment::Quad {
        control: [10367, 347],
        to: [10395, 375],
    },
    MenuLabelSegment::Quad {
        control: [10422, 402],
        to: [10422, 439],
    },
    MenuLabelSegment::Quad {
        control: [10422, 477],
        to: [10395, 505],
    },
    MenuLabelSegment::Quad {
        control: [10367, 531],
        to: [10329, 531],
    },
    MenuLabelSegment::Quad {
        control: [10292, 531],
        to: [10265, 505],
    },
    MenuLabelSegment::Quad {
        control: [10237, 477],
        to: [10237, 439],
    },
    MenuLabelSegment::Quad {
        control: [10237, 400],
        to: [10264, 373],
    },
    MenuLabelSegment::Quad {
        control: [10290, 347],
        to: [10329, 347],
    },
];

const SPEED_CONTOUR_15: [MenuLabelSegment; 6] = [
    MenuLabelSegment::Line([10397, 1500]),
    MenuLabelSegment::Line([10248, 1500]),
    MenuLabelSegment::Line([10248, 789]),
    MenuLabelSegment::Line([10132, 789]),
    MenuLabelSegment::Line([10132, 664]),
    MenuLabelSegment::Line([10397, 664]),
];

const SPEED_CONTOUR_16: [MenuLabelSegment; 18] = [
    MenuLabelSegment::Line([10684, 489]),
    MenuLabelSegment::Line([10833, 431]),
    MenuLabelSegment::Line([10833, 664]),
    MenuLabelSegment::Line([11062, 664]),
    MenuLabelSegment::Line([11062, 781]),
    MenuLabelSegment::Line([10833, 781]),
    MenuLabelSegment::Line([10833, 1197]),
    MenuLabelSegment::Quad {
        control: [10833, 1302],
        to: [10869, 1347],
    },
    MenuLabelSegment::Quad {
        control: [10903, 1391],
        to: [10983, 1391],
    },
    MenuLabelSegment::Quad {
        control: [11039, 1391],
        to: [11100, 1362],
    },
    MenuLabelSegment::Line([11122, 1492]),
    MenuLabelSegment::Line([10920, 1516]),
    MenuLabelSegment::Quad {
        control: [10820, 1516],
        to: [10753, 1442],
    },
    MenuLabelSegment::Quad {
        control: [10684, 1369],
        to: [10684, 1256],
    },
    MenuLabelSegment::Line([10684, 781]),
    MenuLabelSegment::Line([10588, 781]),
    MenuLabelSegment::Line([10588, 664]),
    MenuLabelSegment::Line([10684, 664]),
];

const SPEED_CONTOUR_17: [MenuLabelSegment; 13] = [
    MenuLabelSegment::Line([11585, 1642]),
    MenuLabelSegment::Quad {
        control: [11557, 1722],
        to: [11465, 1775],
    },
    MenuLabelSegment::Quad {
        control: [11371, 1828],
        to: [11257, 1828],
    },
    MenuLabelSegment::Line([11257, 1695]),
    MenuLabelSegment::Quad {
        control: [11351, 1695],
        to: [11416, 1653],
    },
    MenuLabelSegment::Quad {
        control: [11483, 1609],
        to: [11483, 1547],
    },
    MenuLabelSegment::Quad {
        control: [11483, 1478],
        to: [11458, 1411],
    },
    MenuLabelSegment::Line([11396, 1245]),
    MenuLabelSegment::Line([11169, 664]),
    MenuLabelSegment::Line([11321, 664]),
    MenuLabelSegment::Line([11568, 1311]),
    MenuLabelSegment::Line([11788, 664]),
    MenuLabelSegment::Line([11940, 664]),
];

const SPEED_CONTOURS: [MenuLabelContour; 18] = [
    MenuLabelContour {
        start: [2681, 711],
        segments: &SPEED_CONTOUR_0,
    },
    MenuLabelContour {
        start: [3470, 664],
        segments: &SPEED_CONTOUR_1,
    },
    MenuLabelContour {
        start: [3470, 844],
        segments: &SPEED_CONTOUR_2,
    },
    MenuLabelContour {
        start: [4555, 773],
        segments: &SPEED_CONTOUR_3,
    },
    MenuLabelContour {
        start: [4392, 1322],
        segments: &SPEED_CONTOUR_4,
    },
    MenuLabelContour {
        start: [5430, 773],
        segments: &SPEED_CONTOUR_5,
    },
    MenuLabelContour {
        start: [5268, 1322],
        segments: &SPEED_CONTOUR_6,
    },
    MenuLabelContour {
        start: [6491, 320],
        segments: &SPEED_CONTOUR_7,
    },
    MenuLabelContour {
        start: [6430, 1369],
        segments: &SPEED_CONTOUR_8,
    },
    MenuLabelContour {
        start: [7590, 564],
        segments: &SPEED_CONTOUR_9,
    },
    MenuLabelContour {
        start: [8386, 806],
        segments: &SPEED_CONTOUR_10,
    },
    MenuLabelContour {
        start: [8811, 648],
        segments: &SPEED_CONTOUR_11,
    },
    MenuLabelContour {
        start: [8911, 1073],
        segments: &SPEED_CONTOUR_12,
    },
    MenuLabelContour {
        start: [10060, 661],
        segments: &SPEED_CONTOUR_13,
    },
    MenuLabelContour {
        start: [10329, 347],
        segments: &SPEED_CONTOUR_14,
    },
    MenuLabelContour {
        start: [10397, 664],
        segments: &SPEED_CONTOUR_15,
    },
    MenuLabelContour {
        start: [10684, 664],
        segments: &SPEED_CONTOUR_16,
    },
    MenuLabelContour {
        start: [11940, 664],
        segments: &SPEED_CONTOUR_17,
    },
];

pub const SPEED: MenuLabelDefinition = MenuLabelDefinition {
    text: "SpeedGravity",
    define_text_id: 67,
    font_id: FONT_ID,
    color_rgb: [255, 255, 255],
    bounds_centipx: [2510, 12650, 320, 1830],
    contours: &SPEED_CONTOURS,
};

const START_CONTOUR_0: [MenuLabelSegment; 22] = [
    MenuLabelSegment::Line([3861, 844]),
    MenuLabelSegment::Quad {
        control: [3773, 773],
        to: [3684, 773],
    },
    MenuLabelSegment::Quad {
        control: [3631, 773],
        to: [3595, 798],
    },
    MenuLabelSegment::Quad {
        control: [3558, 823],
        to: [3558, 861],
    },
    MenuLabelSegment::Quad {
        control: [3558, 942],
        to: [3650, 983],
    },
    MenuLabelSegment::Line([3756, 1031]),
    MenuLabelSegment::Quad {
        control: [3853, 1077],
        to: [3898, 1133],
    },
    MenuLabelSegment::Quad {
        control: [3942, 1191],
        to: [3942, 1277],
    },
    MenuLabelSegment::Quad {
        control: [3942, 1389],
        to: [3864, 1453],
    },
    MenuLabelSegment::Quad {
        control: [3784, 1516],
        to: [3645, 1516],
    },
    MenuLabelSegment::Quad {
        control: [3512, 1516],
        to: [3397, 1450],
    },
    MenuLabelSegment::Line([3448, 1309]),
    MenuLabelSegment::Quad {
        control: [3573, 1391],
        to: [3648, 1391],
    },
    MenuLabelSegment::Quad {
        control: [3786, 1391],
        to: [3786, 1275],
    },
    MenuLabelSegment::Quad {
        control: [3786, 1192],
        to: [3653, 1133],
    },
    MenuLabelSegment::Quad {
        control: [3551, 1086],
        to: [3515, 1062],
    },
    MenuLabelSegment::Quad {
        control: [3479, 1038],
        to: [3454, 1008],
    },
    MenuLabelSegment::Quad {
        control: [3428, 977],
        to: [3415, 942],
    },
    MenuLabelSegment::Quad {
        control: [3401, 906],
        to: [3401, 867],
    },
    MenuLabelSegment::Quad {
        control: [3401, 764],
        to: [3476, 706],
    },
    MenuLabelSegment::Quad {
        control: [3551, 648],
        to: [3673, 648],
    },
    MenuLabelSegment::Quad {
        control: [3764, 648],
        to: [3903, 706],
    },
];

const START_CONTOUR_1: [MenuLabelSegment; 18] = [
    MenuLabelSegment::Line([4154, 489]),
    MenuLabelSegment::Line([4303, 431]),
    MenuLabelSegment::Line([4303, 664]),
    MenuLabelSegment::Line([4532, 664]),
    MenuLabelSegment::Line([4532, 781]),
    MenuLabelSegment::Line([4303, 781]),
    MenuLabelSegment::Line([4303, 1197]),
    MenuLabelSegment::Quad {
        control: [4303, 1302],
        to: [4339, 1347],
    },
    MenuLabelSegment::Quad {
        control: [4373, 1391],
        to: [4453, 1391],
    },
    MenuLabelSegment::Quad {
        control: [4509, 1391],
        to: [4570, 1362],
    },
    MenuLabelSegment::Line([4592, 1492]),
    MenuLabelSegment::Line([4390, 1516]),
    MenuLabelSegment::Quad {
        control: [4290, 1516],
        to: [4223, 1442],
    },
    MenuLabelSegment::Quad {
        control: [4154, 1369],
        to: [4154, 1256],
    },
    MenuLabelSegment::Line([4154, 781]),
    MenuLabelSegment::Line([4058, 781]),
    MenuLabelSegment::Line([4058, 664]),
    MenuLabelSegment::Line([4154, 664]),
];

const START_CONTOUR_2: [MenuLabelSegment; 18] = [
    MenuLabelSegment::Quad {
        control: [5178, 648],
        to: [5261, 731],
    },
    MenuLabelSegment::Quad {
        control: [5342, 814],
        to: [5342, 994],
    },
    MenuLabelSegment::Line([5342, 1294]),
    MenuLabelSegment::Quad {
        control: [5342, 1405],
        to: [5408, 1441],
    },
    MenuLabelSegment::Line([5408, 1516]),
    MenuLabelSegment::Quad {
        control: [5318, 1516],
        to: [5274, 1489],
    },
    MenuLabelSegment::Quad {
        control: [5228, 1464],
        to: [5208, 1405],
    },
    MenuLabelSegment::Quad {
        control: [5119, 1516],
        to: [4936, 1516],
    },
    MenuLabelSegment::Quad {
        control: [4838, 1516],
        to: [4766, 1445],
    },
    MenuLabelSegment::Quad {
        control: [4692, 1373],
        to: [4692, 1267],
    },
    MenuLabelSegment::Quad {
        control: [4692, 1139],
        to: [4805, 1052],
    },
    MenuLabelSegment::Quad {
        control: [4916, 964],
        to: [5088, 964],
    },
    MenuLabelSegment::Line([5194, 984]),
    MenuLabelSegment::Quad {
        control: [5194, 781],
        to: [5013, 781],
    },
    MenuLabelSegment::Quad {
        control: [4874, 781],
        to: [4799, 856],
    },
    MenuLabelSegment::Line([4736, 731]),
    MenuLabelSegment::Quad {
        control: [4778, 697],
        to: [4853, 673],
    },
    MenuLabelSegment::Quad {
        control: [4928, 648],
        to: [4996, 648],
    },
];

const START_CONTOUR_3: [MenuLabelSegment; 6] = [
    MenuLabelSegment::Quad {
        control: [4983, 1073],
        to: [4913, 1131],
    },
    MenuLabelSegment::Quad {
        control: [4841, 1189],
        to: [4841, 1269],
    },
    MenuLabelSegment::Quad {
        control: [4841, 1398],
        to: [4996, 1398],
    },
    MenuLabelSegment::Quad {
        control: [5108, 1398],
        to: [5194, 1292],
    },
    MenuLabelSegment::Line([5194, 1089]),
    MenuLabelSegment::Line([5096, 1073]),
];

const START_CONTOUR_4: [MenuLabelSegment; 11] = [
    MenuLabelSegment::Quad {
        control: [5982, 773],
        to: [5932, 773],
    },
    MenuLabelSegment::Quad {
        control: [5854, 773],
        to: [5795, 845],
    },
    MenuLabelSegment::Quad {
        control: [5736, 919],
        to: [5736, 1020],
    },
    MenuLabelSegment::Line([5736, 1500]),
    MenuLabelSegment::Line([5587, 1500]),
    MenuLabelSegment::Line([5587, 664]),
    MenuLabelSegment::Line([5736, 664]),
    MenuLabelSegment::Line([5736, 797]),
    MenuLabelSegment::Quad {
        control: [5817, 648],
        to: [5978, 648],
    },
    MenuLabelSegment::Line([6092, 662]),
    MenuLabelSegment::Line([6031, 806]),
];

const START_CONTOUR_5: [MenuLabelSegment; 18] = [
    MenuLabelSegment::Line([6249, 489]),
    MenuLabelSegment::Line([6398, 431]),
    MenuLabelSegment::Line([6398, 664]),
    MenuLabelSegment::Line([6628, 664]),
    MenuLabelSegment::Line([6628, 781]),
    MenuLabelSegment::Line([6398, 781]),
    MenuLabelSegment::Line([6398, 1197]),
    MenuLabelSegment::Quad {
        control: [6398, 1302],
        to: [6434, 1347],
    },
    MenuLabelSegment::Quad {
        control: [6468, 1391],
        to: [6548, 1391],
    },
    MenuLabelSegment::Quad {
        control: [6604, 1391],
        to: [6665, 1362],
    },
    MenuLabelSegment::Line([6687, 1492]),
    MenuLabelSegment::Line([6485, 1516]),
    MenuLabelSegment::Quad {
        control: [6385, 1516],
        to: [6318, 1442],
    },
    MenuLabelSegment::Quad {
        control: [6249, 1369],
        to: [6249, 1256],
    },
    MenuLabelSegment::Line([6249, 781]),
    MenuLabelSegment::Line([6153, 781]),
    MenuLabelSegment::Line([6153, 664]),
    MenuLabelSegment::Line([6249, 664]),
];

const START_CONTOUR_6: [MenuLabelSegment; 32] = [
    MenuLabelSegment::Line([7860, 769]),
    MenuLabelSegment::Quad {
        control: [7918, 842],
        to: [7918, 962],
    },
    MenuLabelSegment::Quad {
        control: [7918, 1089],
        to: [7838, 1175],
    },
    MenuLabelSegment::Quad {
        control: [7760, 1261],
        to: [7632, 1273],
    },
    MenuLabelSegment::Line([7508, 1286]),
    MenuLabelSegment::Line([7450, 1303]),
    MenuLabelSegment::Quad {
        control: [7413, 1317],
        to: [7413, 1341],
    },
    MenuLabelSegment::Quad {
        control: [7413, 1372],
        to: [7489, 1372],
    },
    MenuLabelSegment::Line([7594, 1361]),
    MenuLabelSegment::Line([7700, 1348]),
    MenuLabelSegment::Quad {
        control: [7824, 1348],
        to: [7892, 1408],
    },
    MenuLabelSegment::Quad {
        control: [7961, 1466],
        to: [7961, 1570],
    },
    MenuLabelSegment::Quad {
        control: [7961, 1686],
        to: [7858, 1758],
    },
    MenuLabelSegment::Quad {
        control: [7755, 1828],
        to: [7596, 1828],
    },
    MenuLabelSegment::Quad {
        control: [7514, 1828],
        to: [7425, 1800],
    },
    MenuLabelSegment::Quad {
        control: [7335, 1770],
        to: [7280, 1730],
    },
    MenuLabelSegment::Line([7361, 1611]),
    MenuLabelSegment::Quad {
        control: [7491, 1697],
        to: [7600, 1697],
    },
    MenuLabelSegment::Quad {
        control: [7700, 1697],
        to: [7760, 1662],
    },
    MenuLabelSegment::Quad {
        control: [7818, 1628],
        to: [7818, 1577],
    },
    MenuLabelSegment::Quad {
        control: [7818, 1475],
        to: [7671, 1475],
    },
    MenuLabelSegment::Line([7580, 1488]),
    MenuLabelSegment::Line([7477, 1500]),
    MenuLabelSegment::Quad {
        control: [7299, 1500],
        to: [7299, 1366],
    },
    MenuLabelSegment::Quad {
        control: [7299, 1323],
        to: [7341, 1291],
    },
    MenuLabelSegment::Quad {
        control: [7383, 1256],
        to: [7444, 1242],
    },
    MenuLabelSegment::Quad {
        control: [7268, 1159],
        to: [7268, 955],
    },
    MenuLabelSegment::Quad {
        control: [7268, 823],
        to: [7360, 736],
    },
    MenuLabelSegment::Quad {
        control: [7450, 648],
        to: [7585, 648],
    },
    MenuLabelSegment::Quad {
        control: [7708, 648],
        to: [7778, 698],
    },
    MenuLabelSegment::Line([7852, 609]),
    MenuLabelSegment::Line([7949, 702]),
];

const START_CONTOUR_7: [MenuLabelSegment; 8] = [
    MenuLabelSegment::Quad {
        control: [7518, 767],
        to: [7469, 822],
    },
    MenuLabelSegment::Quad {
        control: [7421, 877],
        to: [7421, 955],
    },
    MenuLabelSegment::Quad {
        control: [7421, 1042],
        to: [7468, 1100],
    },
    MenuLabelSegment::Quad {
        control: [7514, 1158],
        to: [7596, 1158],
    },
    MenuLabelSegment::Quad {
        control: [7674, 1158],
        to: [7719, 1102],
    },
    MenuLabelSegment::Quad {
        control: [7763, 1045],
        to: [7763, 955],
    },
    MenuLabelSegment::Quad {
        control: [7763, 877],
        to: [7716, 822],
    },
    MenuLabelSegment::Quad {
        control: [7668, 767],
        to: [7596, 767],
    },
];

const START_CONTOUR_8: [MenuLabelSegment; 18] = [
    MenuLabelSegment::Quad {
        control: [8558, 648],
        to: [8641, 731],
    },
    MenuLabelSegment::Quad {
        control: [8722, 814],
        to: [8722, 994],
    },
    MenuLabelSegment::Line([8722, 1294]),
    MenuLabelSegment::Quad {
        control: [8722, 1405],
        to: [8788, 1441],
    },
    MenuLabelSegment::Line([8788, 1516]),
    MenuLabelSegment::Quad {
        control: [8698, 1516],
        to: [8654, 1489],
    },
    MenuLabelSegment::Quad {
        control: [8608, 1464],
        to: [8588, 1405],
    },
    MenuLabelSegment::Quad {
        control: [8499, 1516],
        to: [8316, 1516],
    },
    MenuLabelSegment::Quad {
        control: [8218, 1516],
        to: [8146, 1445],
    },
    MenuLabelSegment::Quad {
        control: [8072, 1373],
        to: [8072, 1267],
    },
    MenuLabelSegment::Quad {
        control: [8072, 1139],
        to: [8185, 1052],
    },
    MenuLabelSegment::Quad {
        control: [8296, 964],
        to: [8468, 964],
    },
    MenuLabelSegment::Line([8574, 984]),
    MenuLabelSegment::Quad {
        control: [8574, 781],
        to: [8393, 781],
    },
    MenuLabelSegment::Quad {
        control: [8254, 781],
        to: [8179, 856],
    },
    MenuLabelSegment::Line([8116, 731]),
    MenuLabelSegment::Quad {
        control: [8158, 697],
        to: [8233, 673],
    },
    MenuLabelSegment::Quad {
        control: [8308, 648],
        to: [8376, 648],
    },
];

const START_CONTOUR_9: [MenuLabelSegment; 6] = [
    MenuLabelSegment::Quad {
        control: [8363, 1073],
        to: [8293, 1131],
    },
    MenuLabelSegment::Quad {
        control: [8221, 1189],
        to: [8221, 1269],
    },
    MenuLabelSegment::Quad {
        control: [8221, 1398],
        to: [8376, 1398],
    },
    MenuLabelSegment::Quad {
        control: [8488, 1398],
        to: [8574, 1292],
    },
    MenuLabelSegment::Line([8574, 1089]),
    MenuLabelSegment::Line([8476, 1073]),
];

const START_CONTOUR_10: [MenuLabelSegment; 24] = [
    MenuLabelSegment::Quad {
        control: [10073, 802],
        to: [10073, 941],
    },
    MenuLabelSegment::Line([10073, 1500]),
    MenuLabelSegment::Line([9925, 1500]),
    MenuLabelSegment::Line([9925, 970]),
    MenuLabelSegment::Quad {
        control: [9925, 773],
        to: [9753, 773],
    },
    MenuLabelSegment::Quad {
        control: [9700, 773],
        to: [9653, 806],
    },
    MenuLabelSegment::Quad {
        control: [9606, 839],
        to: [9589, 881],
    },
    MenuLabelSegment::Line([9589, 1500]),
    MenuLabelSegment::Line([9441, 1500]),
    MenuLabelSegment::Line([9441, 906]),
    MenuLabelSegment::Quad {
        control: [9441, 844],
        to: [9394, 809],
    },
    MenuLabelSegment::Quad {
        control: [9347, 773],
        to: [9270, 773],
    },
    MenuLabelSegment::Quad {
        control: [9227, 773],
        to: [9177, 808],
    },
    MenuLabelSegment::Quad {
        control: [9125, 842],
        to: [9105, 883],
    },
    MenuLabelSegment::Line([9105, 1500]),
    MenuLabelSegment::Line([8956, 1500]),
    MenuLabelSegment::Line([8956, 664]),
    MenuLabelSegment::Line([9053, 664]),
    MenuLabelSegment::Line([9102, 761]),
    MenuLabelSegment::Quad {
        control: [9188, 648],
        to: [9317, 648],
    },
    MenuLabelSegment::Quad {
        control: [9497, 648],
        to: [9569, 759],
    },
    MenuLabelSegment::Quad {
        control: [9594, 712],
        to: [9661, 680],
    },
    MenuLabelSegment::Quad {
        control: [9730, 648],
        to: [9802, 648],
    },
    MenuLabelSegment::Quad {
        control: [9931, 648],
        to: [10002, 725],
    },
];

const START_CONTOUR_11: [MenuLabelSegment; 5] = [
    MenuLabelSegment::Quad {
        control: [10532, 773],
        to: [10464, 838],
    },
    MenuLabelSegment::Quad {
        control: [10400, 898],
        to: [10391, 989],
    },
    MenuLabelSegment::Line([10855, 989]),
    MenuLabelSegment::Quad {
        control: [10855, 898],
        to: [10799, 839],
    },
    MenuLabelSegment::Quad {
        control: [10736, 773],
        to: [10630, 773],
    },
];

const START_CONTOUR_12: [MenuLabelSegment; 14] = [
    MenuLabelSegment::Quad {
        control: [10539, 1391],
        to: [10650, 1391],
    },
    MenuLabelSegment::Quad {
        control: [10778, 1391],
        to: [10863, 1317],
    },
    MenuLabelSegment::Line([10925, 1423]),
    MenuLabelSegment::Quad {
        control: [10891, 1458],
        to: [10821, 1483],
    },
    MenuLabelSegment::Quad {
        control: [10732, 1516],
        to: [10622, 1516],
    },
    MenuLabelSegment::Quad {
        control: [10464, 1516],
        to: [10353, 1409],
    },
    MenuLabelSegment::Quad {
        control: [10232, 1291],
        to: [10232, 1092],
    },
    MenuLabelSegment::Quad {
        control: [10232, 884],
        to: [10357, 759],
    },
    MenuLabelSegment::Quad {
        control: [10469, 648],
        to: [10624, 648],
    },
    MenuLabelSegment::Quad {
        control: [10802, 648],
        to: [10903, 748],
    },
    MenuLabelSegment::Quad {
        control: [11002, 845],
        to: [11002, 1006],
    },
    MenuLabelSegment::Quad {
        control: [11002, 1055],
        to: [10991, 1097],
    },
    MenuLabelSegment::Line([10388, 1097]),
    MenuLabelSegment::Quad {
        control: [10388, 1244],
        to: [10467, 1322],
    },
];

const START_CONTOURS: [MenuLabelContour; 13] = [
    MenuLabelContour {
        start: [3903, 706],
        segments: &START_CONTOUR_0,
    },
    MenuLabelContour {
        start: [4154, 664],
        segments: &START_CONTOUR_1,
    },
    MenuLabelContour {
        start: [4996, 648],
        segments: &START_CONTOUR_2,
    },
    MenuLabelContour {
        start: [5096, 1073],
        segments: &START_CONTOUR_3,
    },
    MenuLabelContour {
        start: [6031, 806],
        segments: &START_CONTOUR_4,
    },
    MenuLabelContour {
        start: [6249, 664],
        segments: &START_CONTOUR_5,
    },
    MenuLabelContour {
        start: [7949, 702],
        segments: &START_CONTOUR_6,
    },
    MenuLabelContour {
        start: [7596, 767],
        segments: &START_CONTOUR_7,
    },
    MenuLabelContour {
        start: [8376, 648],
        segments: &START_CONTOUR_8,
    },
    MenuLabelContour {
        start: [8476, 1073],
        segments: &START_CONTOUR_9,
    },
    MenuLabelContour {
        start: [10002, 725],
        segments: &START_CONTOUR_10,
    },
    MenuLabelContour {
        start: [10630, 773],
        segments: &START_CONTOUR_11,
    },
    MenuLabelContour {
        start: [10467, 1322],
        segments: &START_CONTOUR_12,
    },
];

pub const START: MenuLabelDefinition = MenuLabelDefinition {
    text: "start game",
    define_text_id: 40,
    font_id: FONT_ID,
    color_rgb: [255, 255, 255],
    bounds_centipx: [3395, 11755, 430, 1830],
    contours: &START_CONTOURS,
};

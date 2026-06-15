// Generated from gravity_arcade.swf sprite-162 announcement DefineText tags.
// Coordinates are stored as SWF text-local centipixels; divide by 100 for pixels.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnnounceTextSegment {
    Line([i16; 2]),
    Quad { control: [i16; 2], to: [i16; 2] },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnnounceTextContour {
    pub(super) start: [i16; 2],
    pub(super) segments: &'static [AnnounceTextSegment],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AnnounceTextDefinition {
    pub(super) text: &'static str,
    pub(super) define_text_id: u16,
    pub(super) font_ids: &'static [u16],
    pub(super) color_rgb: [u8; 3],
    pub(super) bounds_centipx: [i16; 4],
    pub(super) contours: &'static [AnnounceTextContour],
}

const BLUE_WINS_FONT_IDS: [u16; 1] = [26];

const BLUE_WINS_CONTOUR_0: [AnnounceTextSegment; 11] = [
    AnnounceTextSegment::Quad {
        control: [1065, 586],
        to: [1021, 635],
    },
    AnnounceTextSegment::Quad {
        control: [976, 684],
        to: [922, 696],
    },
    AnnounceTextSegment::Quad {
        control: [1024, 721],
        to: [1071, 778],
    },
    AnnounceTextSegment::Quad {
        control: [1118, 834],
        to: [1118, 933],
    },
    AnnounceTextSegment::Quad {
        control: [1118, 1045],
        to: [1035, 1113],
    },
    AnnounceTextSegment::Quad {
        control: [951, 1180],
        to: [818, 1180],
    },
    AnnounceTextSegment::Line([573, 1180]),
    AnnounceTextSegment::Line([573, 322]),
    AnnounceTextSegment::Line([799, 314]),
    AnnounceTextSegment::Quad {
        control: [926, 314],
        to: [996, 369],
    },
    AnnounceTextSegment::Quad {
        control: [1065, 424],
        to: [1065, 526],
    },
];

const BLUE_WINS_CONTOUR_1: [AnnounceTextSegment; 5] = [
    AnnounceTextSegment::Line([690, 662]),
    AnnounceTextSegment::Line([781, 666]),
    AnnounceTextSegment::Quad {
        control: [948, 666],
        to: [948, 531],
    },
    AnnounceTextSegment::Quad {
        control: [948, 411],
        to: [796, 411],
    },
    AnnounceTextSegment::Line([690, 416]),
];

const BLUE_WINS_CONTOUR_2: [AnnounceTextSegment; 7] = [
    AnnounceTextSegment::Line([690, 1080]),
    AnnounceTextSegment::Line([783, 1086]),
    AnnounceTextSegment::Quad {
        control: [893, 1086],
        to: [944, 1045],
    },
    AnnounceTextSegment::Quad {
        control: [995, 1004],
        to: [995, 914],
    },
    AnnounceTextSegment::Quad {
        control: [995, 830],
        to: [947, 790],
    },
    AnnounceTextSegment::Quad {
        control: [898, 750],
        to: [785, 750],
    },
    AnnounceTextSegment::Line([690, 753]),
];

const BLUE_WINS_CONTOUR_3: [AnnounceTextSegment; 7] = [
    AnnounceTextSegment::Line([1470, 1192]),
    AnnounceTextSegment::Quad {
        control: [1253, 1192],
        to: [1253, 1003],
    },
    AnnounceTextSegment::Line([1253, 295]),
    AnnounceTextSegment::Line([1364, 295]),
    AnnounceTextSegment::Line([1364, 984]),
    AnnounceTextSegment::Quad {
        control: [1364, 1035],
        to: [1394, 1064],
    },
    AnnounceTextSegment::Quad {
        control: [1423, 1092],
        to: [1470, 1092],
    },
];

const BLUE_WINS_CONTOUR_4: [AnnounceTextSegment; 15] = [
    AnnounceTextSegment::Line([1705, 953]),
    AnnounceTextSegment::Quad {
        control: [1705, 1098],
        to: [1831, 1098],
    },
    AnnounceTextSegment::Quad {
        control: [1886, 1098],
        to: [1931, 1066],
    },
    AnnounceTextSegment::Quad {
        control: [1977, 1035],
        to: [1992, 994],
    },
    AnnounceTextSegment::Line([1992, 553]),
    AnnounceTextSegment::Line([2104, 553]),
    AnnounceTextSegment::Line([2104, 1180]),
    AnnounceTextSegment::Line([1992, 1180]),
    AnnounceTextSegment::Line([1992, 1093]),
    AnnounceTextSegment::Quad {
        control: [1974, 1131],
        to: [1917, 1161],
    },
    AnnounceTextSegment::Quad {
        control: [1860, 1192],
        to: [1806, 1192],
    },
    AnnounceTextSegment::Quad {
        control: [1703, 1192],
        to: [1649, 1133],
    },
    AnnounceTextSegment::Quad {
        control: [1594, 1073],
        to: [1594, 964],
    },
    AnnounceTextSegment::Line([1594, 553]),
    AnnounceTextSegment::Line([1705, 553]),
];

const BLUE_WINS_CONTOUR_5: [AnnounceTextSegment; 5] = [
    AnnounceTextSegment::Quad {
        control: [2439, 635],
        to: [2388, 683],
    },
    AnnounceTextSegment::Quad {
        control: [2340, 729],
        to: [2333, 797],
    },
    AnnounceTextSegment::Line([2681, 797]),
    AnnounceTextSegment::Quad {
        control: [2681, 729],
        to: [2639, 684],
    },
    AnnounceTextSegment::Quad {
        control: [2592, 635],
        to: [2512, 635],
    },
];

const BLUE_WINS_CONTOUR_6: [AnnounceTextSegment; 14] = [
    AnnounceTextSegment::Quad {
        control: [2445, 1098],
        to: [2528, 1098],
    },
    AnnounceTextSegment::Quad {
        control: [2624, 1098],
        to: [2687, 1043],
    },
    AnnounceTextSegment::Line([2734, 1123]),
    AnnounceTextSegment::Quad {
        control: [2708, 1148],
        to: [2655, 1167],
    },
    AnnounceTextSegment::Quad {
        control: [2589, 1192],
        to: [2507, 1192],
    },
    AnnounceTextSegment::Quad {
        control: [2388, 1192],
        to: [2305, 1112],
    },
    AnnounceTextSegment::Quad {
        control: [2214, 1023],
        to: [2214, 874],
    },
    AnnounceTextSegment::Quad {
        control: [2214, 718],
        to: [2307, 625],
    },
    AnnounceTextSegment::Quad {
        control: [2392, 541],
        to: [2508, 541],
    },
    AnnounceTextSegment::Quad {
        control: [2641, 541],
        to: [2718, 616],
    },
    AnnounceTextSegment::Quad {
        control: [2791, 689],
        to: [2791, 810],
    },
    AnnounceTextSegment::Quad {
        control: [2791, 846],
        to: [2783, 878],
    },
    AnnounceTextSegment::Line([2331, 878]),
    AnnounceTextSegment::Quad {
        control: [2331, 988],
        to: [2391, 1046],
    },
];

const BLUE_WINS_CONTOUR_7: [AnnounceTextSegment; 13] = [
    AnnounceTextSegment::Line([4205, 322]),
    AnnounceTextSegment::Line([3938, 1192]),
    AnnounceTextSegment::Line([3901, 1192]),
    AnnounceTextSegment::Line([3696, 600]),
    AnnounceTextSegment::Line([3505, 1192]),
    AnnounceTextSegment::Line([3469, 1192]),
    AnnounceTextSegment::Line([3198, 322]),
    AnnounceTextSegment::Line([3320, 322]),
    AnnounceTextSegment::Line([3492, 921]),
    AnnounceTextSegment::Line([3681, 322]),
    AnnounceTextSegment::Line([3722, 322]),
    AnnounceTextSegment::Line([3908, 920]),
    AnnounceTextSegment::Line([4083, 322]),
];

const BLUE_WINS_CONTOUR_8: [AnnounceTextSegment; 8] = [
    AnnounceTextSegment::Quad {
        control: [4433, 315],
        to: [4454, 336],
    },
    AnnounceTextSegment::Quad {
        control: [4474, 356],
        to: [4474, 384],
    },
    AnnounceTextSegment::Quad {
        control: [4474, 412],
        to: [4454, 434],
    },
    AnnounceTextSegment::Quad {
        control: [4433, 453],
        to: [4405, 453],
    },
    AnnounceTextSegment::Quad {
        control: [4376, 453],
        to: [4356, 434],
    },
    AnnounceTextSegment::Quad {
        control: [4335, 412],
        to: [4335, 384],
    },
    AnnounceTextSegment::Quad {
        control: [4335, 355],
        to: [4355, 335],
    },
    AnnounceTextSegment::Quad {
        control: [4375, 315],
        to: [4405, 315],
    },
];

const BLUE_WINS_CONTOUR_9: [AnnounceTextSegment; 6] = [
    AnnounceTextSegment::Line([4455, 1180]),
    AnnounceTextSegment::Line([4344, 1180]),
    AnnounceTextSegment::Line([4344, 647]),
    AnnounceTextSegment::Line([4257, 647]),
    AnnounceTextSegment::Line([4257, 553]),
    AnnounceTextSegment::Line([4455, 553]),
];

const BLUE_WINS_CONTOUR_10: [AnnounceTextSegment; 14] = [
    AnnounceTextSegment::Line([5127, 1180]),
    AnnounceTextSegment::Line([5015, 1180]),
    AnnounceTextSegment::Line([5015, 816]),
    AnnounceTextSegment::Quad {
        control: [5015, 715],
        to: [4986, 675],
    },
    AnnounceTextSegment::Quad {
        control: [4955, 635],
        to: [4884, 635],
    },
    AnnounceTextSegment::Quad {
        control: [4846, 635],
        to: [4804, 657],
    },
    AnnounceTextSegment::Quad {
        control: [4763, 681],
        to: [4741, 714],
    },
    AnnounceTextSegment::Line([4741, 1180]),
    AnnounceTextSegment::Line([4630, 1180]),
    AnnounceTextSegment::Line([4630, 553]),
    AnnounceTextSegment::Line([4706, 553]),
    AnnounceTextSegment::Line([4741, 634]),
    AnnounceTextSegment::Quad {
        control: [4796, 541],
        to: [4920, 541],
    },
    AnnounceTextSegment::Quad {
        control: [5127, 541],
        to: [5127, 792],
    },
];

const BLUE_WINS_CONTOUR_11: [AnnounceTextSegment; 22] = [
    AnnounceTextSegment::Line([5592, 688]),
    AnnounceTextSegment::Quad {
        control: [5526, 635],
        to: [5459, 635],
    },
    AnnounceTextSegment::Quad {
        control: [5419, 635],
        to: [5392, 654],
    },
    AnnounceTextSegment::Quad {
        control: [5364, 673],
        to: [5364, 701],
    },
    AnnounceTextSegment::Quad {
        control: [5364, 762],
        to: [5434, 792],
    },
    AnnounceTextSegment::Line([5513, 828]),
    AnnounceTextSegment::Quad {
        control: [5586, 862],
        to: [5620, 905],
    },
    AnnounceTextSegment::Quad {
        control: [5653, 948],
        to: [5653, 1012],
    },
    AnnounceTextSegment::Quad {
        control: [5653, 1097],
        to: [5594, 1145],
    },
    AnnounceTextSegment::Quad {
        control: [5534, 1192],
        to: [5430, 1192],
    },
    AnnounceTextSegment::Quad {
        control: [5330, 1192],
        to: [5244, 1142],
    },
    AnnounceTextSegment::Line([5282, 1037]),
    AnnounceTextSegment::Quad {
        control: [5376, 1098],
        to: [5432, 1098],
    },
    AnnounceTextSegment::Quad {
        control: [5535, 1098],
        to: [5535, 1011],
    },
    AnnounceTextSegment::Quad {
        control: [5535, 949],
        to: [5436, 905],
    },
    AnnounceTextSegment::Quad {
        control: [5360, 869],
        to: [5333, 852],
    },
    AnnounceTextSegment::Quad {
        control: [5306, 833],
        to: [5287, 811],
    },
    AnnounceTextSegment::Quad {
        control: [5267, 787],
        to: [5258, 762],
    },
    AnnounceTextSegment::Quad {
        control: [5247, 735],
        to: [5247, 705],
    },
    AnnounceTextSegment::Quad {
        control: [5247, 628],
        to: [5303, 585],
    },
    AnnounceTextSegment::Quad {
        control: [5360, 541],
        to: [5451, 541],
    },
    AnnounceTextSegment::Quad {
        control: [5519, 541],
        to: [5623, 585],
    },
];

const BLUE_WINS_CONTOURS: [AnnounceTextContour; 12] = [
    AnnounceTextContour {
        start: [1065, 526],
        segments: &BLUE_WINS_CONTOUR_0,
    },
    AnnounceTextContour {
        start: [690, 416],
        segments: &BLUE_WINS_CONTOUR_1,
    },
    AnnounceTextContour {
        start: [690, 753],
        segments: &BLUE_WINS_CONTOUR_2,
    },
    AnnounceTextContour {
        start: [1470, 1092],
        segments: &BLUE_WINS_CONTOUR_3,
    },
    AnnounceTextContour {
        start: [1705, 553],
        segments: &BLUE_WINS_CONTOUR_4,
    },
    AnnounceTextContour {
        start: [2512, 635],
        segments: &BLUE_WINS_CONTOUR_5,
    },
    AnnounceTextContour {
        start: [2391, 1046],
        segments: &BLUE_WINS_CONTOUR_6,
    },
    AnnounceTextContour {
        start: [4083, 322],
        segments: &BLUE_WINS_CONTOUR_7,
    },
    AnnounceTextContour {
        start: [4405, 315],
        segments: &BLUE_WINS_CONTOUR_8,
    },
    AnnounceTextContour {
        start: [4455, 553],
        segments: &BLUE_WINS_CONTOUR_9,
    },
    AnnounceTextContour {
        start: [5127, 792],
        segments: &BLUE_WINS_CONTOUR_10,
    },
    AnnounceTextContour {
        start: [5623, 585],
        segments: &BLUE_WINS_CONTOUR_11,
    },
];

pub const BLUE_WINS: AnnounceTextDefinition = AnnounceTextDefinition {
    text: "Blue Wins",
    define_text_id: 146,
    font_ids: &BLUE_WINS_FONT_IDS,
    color_rgb: [136, 247, 255],
    bounds_centipx: [575, 6215, 295, 1190],
    contours: &BLUE_WINS_CONTOURS,
};

const RED_WINS_FONT_IDS: [u16; 1] = [26];

const RED_WINS_CONTOUR_0: [AnnounceTextSegment; 7] = [
    AnnounceTextSegment::Line([856, 707]),
    AnnounceTextSegment::Line([938, 712]),
    AnnounceTextSegment::Quad {
        control: [1043, 712],
        to: [1091, 678],
    },
    AnnounceTextSegment::Quad {
        control: [1140, 644],
        to: [1140, 557],
    },
    AnnounceTextSegment::Quad {
        control: [1140, 485],
        to: [1088, 453],
    },
    AnnounceTextSegment::Quad {
        control: [1036, 422],
        to: [925, 422],
    },
    AnnounceTextSegment::Line([856, 428]),
];

const RED_WINS_CONTOUR_1: [AnnounceTextSegment; 12] = [
    AnnounceTextSegment::Line([953, 313]),
    AnnounceTextSegment::Quad {
        control: [1263, 313],
        to: [1263, 559],
    },
    AnnounceTextSegment::Quad {
        control: [1263, 640],
        to: [1213, 705],
    },
    AnnounceTextSegment::Quad {
        control: [1163, 770],
        to: [1095, 785],
    },
    AnnounceTextSegment::Line([1343, 1180]),
    AnnounceTextSegment::Line([1210, 1180]),
    AnnounceTextSegment::Line([980, 812]),
    AnnounceTextSegment::Line([856, 806]),
    AnnounceTextSegment::Line([856, 1180]),
    AnnounceTextSegment::Line([739, 1180]),
    AnnounceTextSegment::Line([739, 322]),
    AnnounceTextSegment::Line([830, 318]),
];

const RED_WINS_CONTOUR_2: [AnnounceTextSegment; 5] = [
    AnnounceTextSegment::Quad {
        control: [1609, 635],
        to: [1558, 683],
    },
    AnnounceTextSegment::Quad {
        control: [1510, 729],
        to: [1503, 797],
    },
    AnnounceTextSegment::Line([1851, 797]),
    AnnounceTextSegment::Quad {
        control: [1851, 729],
        to: [1809, 684],
    },
    AnnounceTextSegment::Quad {
        control: [1762, 635],
        to: [1682, 635],
    },
];

const RED_WINS_CONTOUR_3: [AnnounceTextSegment; 14] = [
    AnnounceTextSegment::Quad {
        control: [1615, 1098],
        to: [1698, 1098],
    },
    AnnounceTextSegment::Quad {
        control: [1794, 1098],
        to: [1857, 1043],
    },
    AnnounceTextSegment::Line([1904, 1123]),
    AnnounceTextSegment::Quad {
        control: [1878, 1148],
        to: [1825, 1167],
    },
    AnnounceTextSegment::Quad {
        control: [1759, 1192],
        to: [1677, 1192],
    },
    AnnounceTextSegment::Quad {
        control: [1558, 1192],
        to: [1475, 1112],
    },
    AnnounceTextSegment::Quad {
        control: [1384, 1023],
        to: [1384, 874],
    },
    AnnounceTextSegment::Quad {
        control: [1384, 718],
        to: [1477, 625],
    },
    AnnounceTextSegment::Quad {
        control: [1562, 541],
        to: [1678, 541],
    },
    AnnounceTextSegment::Quad {
        control: [1811, 541],
        to: [1888, 616],
    },
    AnnounceTextSegment::Quad {
        control: [1961, 689],
        to: [1961, 810],
    },
    AnnounceTextSegment::Quad {
        control: [1961, 846],
        to: [1953, 878],
    },
    AnnounceTextSegment::Line([1501, 878]),
    AnnounceTextSegment::Quad {
        control: [1501, 988],
        to: [1561, 1046],
    },
];

const RED_WINS_CONTOUR_4: [AnnounceTextSegment; 11] = [
    AnnounceTextSegment::Line([2588, 295]),
    AnnounceTextSegment::Line([2588, 1180]),
    AnnounceTextSegment::Line([2477, 1180]),
    AnnounceTextSegment::Line([2477, 1133]),
    AnnounceTextSegment::Quad {
        control: [2420, 1192],
        to: [2308, 1192],
    },
    AnnounceTextSegment::Quad {
        control: [2191, 1192],
        to: [2117, 1107],
    },
    AnnounceTextSegment::Quad {
        control: [2045, 1023],
        to: [2045, 882],
    },
    AnnounceTextSegment::Quad {
        control: [2045, 741],
        to: [2129, 641],
    },
    AnnounceTextSegment::Quad {
        control: [2213, 541],
        to: [2329, 541],
    },
    AnnounceTextSegment::Quad {
        control: [2427, 541],
        to: [2477, 587],
    },
    AnnounceTextSegment::Line([2477, 295]),
];

const RED_WINS_CONTOUR_5: [AnnounceTextSegment; 7] = [
    AnnounceTextSegment::Quad {
        control: [2468, 1065],
        to: [2477, 1046],
    },
    AnnounceTextSegment::Line([2477, 698]),
    AnnounceTextSegment::Quad {
        control: [2435, 635],
        to: [2362, 635],
    },
    AnnounceTextSegment::Quad {
        control: [2272, 635],
        to: [2217, 702],
    },
    AnnounceTextSegment::Quad {
        control: [2162, 769],
        to: [2162, 872],
    },
    AnnounceTextSegment::Quad {
        control: [2162, 1098],
        to: [2368, 1098],
    },
    AnnounceTextSegment::Quad {
        control: [2394, 1098],
        to: [2431, 1082],
    },
];

const RED_WINS_CONTOUR_6: [AnnounceTextSegment; 13] = [
    AnnounceTextSegment::Line([4045, 322]),
    AnnounceTextSegment::Line([3778, 1192]),
    AnnounceTextSegment::Line([3741, 1192]),
    AnnounceTextSegment::Line([3536, 600]),
    AnnounceTextSegment::Line([3345, 1192]),
    AnnounceTextSegment::Line([3309, 1192]),
    AnnounceTextSegment::Line([3038, 322]),
    AnnounceTextSegment::Line([3160, 322]),
    AnnounceTextSegment::Line([3332, 921]),
    AnnounceTextSegment::Line([3521, 322]),
    AnnounceTextSegment::Line([3562, 322]),
    AnnounceTextSegment::Line([3748, 920]),
    AnnounceTextSegment::Line([3923, 322]),
];

const RED_WINS_CONTOUR_7: [AnnounceTextSegment; 8] = [
    AnnounceTextSegment::Quad {
        control: [4273, 315],
        to: [4294, 336],
    },
    AnnounceTextSegment::Quad {
        control: [4314, 356],
        to: [4314, 384],
    },
    AnnounceTextSegment::Quad {
        control: [4314, 412],
        to: [4294, 434],
    },
    AnnounceTextSegment::Quad {
        control: [4273, 453],
        to: [4245, 453],
    },
    AnnounceTextSegment::Quad {
        control: [4216, 453],
        to: [4196, 434],
    },
    AnnounceTextSegment::Quad {
        control: [4175, 412],
        to: [4175, 384],
    },
    AnnounceTextSegment::Quad {
        control: [4175, 355],
        to: [4195, 335],
    },
    AnnounceTextSegment::Quad {
        control: [4215, 315],
        to: [4245, 315],
    },
];

const RED_WINS_CONTOUR_8: [AnnounceTextSegment; 6] = [
    AnnounceTextSegment::Line([4295, 1180]),
    AnnounceTextSegment::Line([4184, 1180]),
    AnnounceTextSegment::Line([4184, 647]),
    AnnounceTextSegment::Line([4097, 647]),
    AnnounceTextSegment::Line([4097, 553]),
    AnnounceTextSegment::Line([4295, 553]),
];

const RED_WINS_CONTOUR_9: [AnnounceTextSegment; 14] = [
    AnnounceTextSegment::Line([4967, 1180]),
    AnnounceTextSegment::Line([4855, 1180]),
    AnnounceTextSegment::Line([4855, 816]),
    AnnounceTextSegment::Quad {
        control: [4855, 715],
        to: [4826, 675],
    },
    AnnounceTextSegment::Quad {
        control: [4795, 635],
        to: [4724, 635],
    },
    AnnounceTextSegment::Quad {
        control: [4686, 635],
        to: [4644, 657],
    },
    AnnounceTextSegment::Quad {
        control: [4603, 681],
        to: [4581, 714],
    },
    AnnounceTextSegment::Line([4581, 1180]),
    AnnounceTextSegment::Line([4470, 1180]),
    AnnounceTextSegment::Line([4470, 553]),
    AnnounceTextSegment::Line([4546, 553]),
    AnnounceTextSegment::Line([4581, 634]),
    AnnounceTextSegment::Quad {
        control: [4636, 541],
        to: [4760, 541],
    },
    AnnounceTextSegment::Quad {
        control: [4967, 541],
        to: [4967, 792],
    },
];

const RED_WINS_CONTOUR_10: [AnnounceTextSegment; 22] = [
    AnnounceTextSegment::Line([5432, 688]),
    AnnounceTextSegment::Quad {
        control: [5366, 635],
        to: [5299, 635],
    },
    AnnounceTextSegment::Quad {
        control: [5259, 635],
        to: [5232, 654],
    },
    AnnounceTextSegment::Quad {
        control: [5204, 673],
        to: [5204, 701],
    },
    AnnounceTextSegment::Quad {
        control: [5204, 762],
        to: [5274, 792],
    },
    AnnounceTextSegment::Line([5353, 828]),
    AnnounceTextSegment::Quad {
        control: [5426, 862],
        to: [5460, 905],
    },
    AnnounceTextSegment::Quad {
        control: [5493, 948],
        to: [5493, 1012],
    },
    AnnounceTextSegment::Quad {
        control: [5493, 1097],
        to: [5434, 1145],
    },
    AnnounceTextSegment::Quad {
        control: [5374, 1192],
        to: [5270, 1192],
    },
    AnnounceTextSegment::Quad {
        control: [5170, 1192],
        to: [5084, 1142],
    },
    AnnounceTextSegment::Line([5122, 1037]),
    AnnounceTextSegment::Quad {
        control: [5216, 1098],
        to: [5272, 1098],
    },
    AnnounceTextSegment::Quad {
        control: [5375, 1098],
        to: [5375, 1011],
    },
    AnnounceTextSegment::Quad {
        control: [5375, 949],
        to: [5276, 905],
    },
    AnnounceTextSegment::Quad {
        control: [5200, 869],
        to: [5173, 852],
    },
    AnnounceTextSegment::Quad {
        control: [5146, 833],
        to: [5127, 811],
    },
    AnnounceTextSegment::Quad {
        control: [5107, 787],
        to: [5098, 762],
    },
    AnnounceTextSegment::Quad {
        control: [5087, 735],
        to: [5087, 705],
    },
    AnnounceTextSegment::Quad {
        control: [5087, 628],
        to: [5143, 585],
    },
    AnnounceTextSegment::Quad {
        control: [5200, 541],
        to: [5291, 541],
    },
    AnnounceTextSegment::Quad {
        control: [5359, 541],
        to: [5463, 585],
    },
];

const RED_WINS_CONTOURS: [AnnounceTextContour; 11] = [
    AnnounceTextContour {
        start: [856, 428],
        segments: &RED_WINS_CONTOUR_0,
    },
    AnnounceTextContour {
        start: [830, 318],
        segments: &RED_WINS_CONTOUR_1,
    },
    AnnounceTextContour {
        start: [1682, 635],
        segments: &RED_WINS_CONTOUR_2,
    },
    AnnounceTextContour {
        start: [1561, 1046],
        segments: &RED_WINS_CONTOUR_3,
    },
    AnnounceTextContour {
        start: [2477, 295],
        segments: &RED_WINS_CONTOUR_4,
    },
    AnnounceTextContour {
        start: [2431, 1082],
        segments: &RED_WINS_CONTOUR_5,
    },
    AnnounceTextContour {
        start: [3923, 322],
        segments: &RED_WINS_CONTOUR_6,
    },
    AnnounceTextContour {
        start: [4245, 315],
        segments: &RED_WINS_CONTOUR_7,
    },
    AnnounceTextContour {
        start: [4295, 553],
        segments: &RED_WINS_CONTOUR_8,
    },
    AnnounceTextContour {
        start: [4967, 792],
        segments: &RED_WINS_CONTOUR_9,
    },
    AnnounceTextContour {
        start: [5463, 585],
        segments: &RED_WINS_CONTOUR_10,
    },
];

pub const RED_WINS: AnnounceTextDefinition = AnnounceTextDefinition {
    text: "Red Wins",
    define_text_id: 150,
    font_ids: &RED_WINS_FONT_IDS,
    color_rgb: [255, 178, 51],
    bounds_centipx: [740, 6055, 295, 1190],
    contours: &RED_WINS_CONTOURS,
};

const ROUND_FONT_IDS: [u16; 1] = [26];

const ROUND_CONTOUR_0: [AnnounceTextSegment; 7] = [
    AnnounceTextSegment::Line([5771, 707]),
    AnnounceTextSegment::Line([5853, 712]),
    AnnounceTextSegment::Quad {
        control: [5958, 712],
        to: [6006, 678],
    },
    AnnounceTextSegment::Quad {
        control: [6055, 644],
        to: [6055, 557],
    },
    AnnounceTextSegment::Quad {
        control: [6055, 485],
        to: [6003, 453],
    },
    AnnounceTextSegment::Quad {
        control: [5951, 422],
        to: [5840, 422],
    },
    AnnounceTextSegment::Line([5771, 428]),
];

const ROUND_CONTOUR_1: [AnnounceTextSegment; 12] = [
    AnnounceTextSegment::Line([5868, 313]),
    AnnounceTextSegment::Quad {
        control: [6178, 313],
        to: [6178, 559],
    },
    AnnounceTextSegment::Quad {
        control: [6178, 640],
        to: [6128, 705],
    },
    AnnounceTextSegment::Quad {
        control: [6078, 770],
        to: [6010, 785],
    },
    AnnounceTextSegment::Line([6258, 1180]),
    AnnounceTextSegment::Line([6125, 1180]),
    AnnounceTextSegment::Line([5895, 812]),
    AnnounceTextSegment::Line([5771, 806]),
    AnnounceTextSegment::Line([5771, 1180]),
    AnnounceTextSegment::Line([5654, 1180]),
    AnnounceTextSegment::Line([5654, 322]),
    AnnounceTextSegment::Line([5745, 318]),
];

const ROUND_CONTOUR_2: [AnnounceTextSegment; 6] = [
    AnnounceTextSegment::Quad {
        control: [6366, 1101],
        to: [6532, 1101],
    },
    AnnounceTextSegment::Quad {
        control: [6611, 1101],
        to: [6655, 1038],
    },
    AnnounceTextSegment::Quad {
        control: [6699, 975],
        to: [6699, 865],
    },
    AnnounceTextSegment::Quad {
        control: [6699, 632],
        to: [6532, 632],
    },
    AnnounceTextSegment::Quad {
        control: [6456, 632],
        to: [6412, 694],
    },
    AnnounceTextSegment::Quad {
        control: [6366, 756],
        to: [6366, 865],
    },
];

const ROUND_CONTOUR_3: [AnnounceTextSegment; 8] = [
    AnnounceTextSegment::Quad {
        control: [6405, 541],
        to: [6532, 541],
    },
    AnnounceTextSegment::Quad {
        control: [6667, 541],
        to: [6742, 627],
    },
    AnnounceTextSegment::Quad {
        control: [6816, 712],
        to: [6816, 865],
    },
    AnnounceTextSegment::Quad {
        control: [6816, 1017],
        to: [6740, 1105],
    },
    AnnounceTextSegment::Quad {
        control: [6664, 1192],
        to: [6532, 1192],
    },
    AnnounceTextSegment::Quad {
        control: [6399, 1192],
        to: [6324, 1104],
    },
    AnnounceTextSegment::Quad {
        control: [6249, 1015],
        to: [6249, 865],
    },
    AnnounceTextSegment::Quad {
        control: [6249, 719],
        to: [6327, 630],
    },
];

const ROUND_CONTOUR_4: [AnnounceTextSegment; 15] = [
    AnnounceTextSegment::Line([7040, 953]),
    AnnounceTextSegment::Quad {
        control: [7040, 1098],
        to: [7166, 1098],
    },
    AnnounceTextSegment::Quad {
        control: [7221, 1098],
        to: [7266, 1066],
    },
    AnnounceTextSegment::Quad {
        control: [7312, 1035],
        to: [7327, 994],
    },
    AnnounceTextSegment::Line([7327, 553]),
    AnnounceTextSegment::Line([7439, 553]),
    AnnounceTextSegment::Line([7439, 1180]),
    AnnounceTextSegment::Line([7327, 1180]),
    AnnounceTextSegment::Line([7327, 1093]),
    AnnounceTextSegment::Quad {
        control: [7309, 1131],
        to: [7252, 1161],
    },
    AnnounceTextSegment::Quad {
        control: [7195, 1192],
        to: [7141, 1192],
    },
    AnnounceTextSegment::Quad {
        control: [7038, 1192],
        to: [6984, 1133],
    },
    AnnounceTextSegment::Quad {
        control: [6929, 1073],
        to: [6929, 964],
    },
    AnnounceTextSegment::Line([6929, 553]),
    AnnounceTextSegment::Line([7040, 553]),
];

const ROUND_CONTOUR_5: [AnnounceTextSegment; 14] = [
    AnnounceTextSegment::Line([8087, 1180]),
    AnnounceTextSegment::Line([7975, 1180]),
    AnnounceTextSegment::Line([7975, 816]),
    AnnounceTextSegment::Quad {
        control: [7975, 715],
        to: [7946, 675],
    },
    AnnounceTextSegment::Quad {
        control: [7915, 635],
        to: [7844, 635],
    },
    AnnounceTextSegment::Quad {
        control: [7806, 635],
        to: [7764, 657],
    },
    AnnounceTextSegment::Quad {
        control: [7723, 681],
        to: [7701, 714],
    },
    AnnounceTextSegment::Line([7701, 1180]),
    AnnounceTextSegment::Line([7590, 1180]),
    AnnounceTextSegment::Line([7590, 553]),
    AnnounceTextSegment::Line([7666, 553]),
    AnnounceTextSegment::Line([7701, 634]),
    AnnounceTextSegment::Quad {
        control: [7756, 541],
        to: [7880, 541],
    },
    AnnounceTextSegment::Quad {
        control: [8087, 541],
        to: [8087, 792],
    },
];

const ROUND_CONTOUR_6: [AnnounceTextSegment; 11] = [
    AnnounceTextSegment::Line([8753, 295]),
    AnnounceTextSegment::Line([8753, 1180]),
    AnnounceTextSegment::Line([8642, 1180]),
    AnnounceTextSegment::Line([8642, 1133]),
    AnnounceTextSegment::Quad {
        control: [8585, 1192],
        to: [8473, 1192],
    },
    AnnounceTextSegment::Quad {
        control: [8356, 1192],
        to: [8282, 1107],
    },
    AnnounceTextSegment::Quad {
        control: [8210, 1023],
        to: [8210, 882],
    },
    AnnounceTextSegment::Quad {
        control: [8210, 741],
        to: [8294, 641],
    },
    AnnounceTextSegment::Quad {
        control: [8378, 541],
        to: [8494, 541],
    },
    AnnounceTextSegment::Quad {
        control: [8592, 541],
        to: [8642, 587],
    },
    AnnounceTextSegment::Line([8642, 295]),
];

const ROUND_CONTOUR_7: [AnnounceTextSegment; 7] = [
    AnnounceTextSegment::Quad {
        control: [8633, 1065],
        to: [8642, 1046],
    },
    AnnounceTextSegment::Line([8642, 698]),
    AnnounceTextSegment::Quad {
        control: [8600, 635],
        to: [8527, 635],
    },
    AnnounceTextSegment::Quad {
        control: [8437, 635],
        to: [8382, 702],
    },
    AnnounceTextSegment::Quad {
        control: [8327, 769],
        to: [8327, 872],
    },
    AnnounceTextSegment::Quad {
        control: [8327, 1098],
        to: [8533, 1098],
    },
    AnnounceTextSegment::Quad {
        control: [8559, 1098],
        to: [8596, 1082],
    },
];

const ROUND_CONTOURS: [AnnounceTextContour; 8] = [
    AnnounceTextContour {
        start: [5771, 428],
        segments: &ROUND_CONTOUR_0,
    },
    AnnounceTextContour {
        start: [5745, 318],
        segments: &ROUND_CONTOUR_1,
    },
    AnnounceTextContour {
        start: [6366, 865],
        segments: &ROUND_CONTOUR_2,
    },
    AnnounceTextContour {
        start: [6327, 630],
        segments: &ROUND_CONTOUR_3,
    },
    AnnounceTextContour {
        start: [7040, 553],
        segments: &ROUND_CONTOUR_4,
    },
    AnnounceTextContour {
        start: [8087, 792],
        segments: &ROUND_CONTOUR_5,
    },
    AnnounceTextContour {
        start: [8642, 295],
        segments: &ROUND_CONTOUR_6,
    },
    AnnounceTextContour {
        start: [8596, 1082],
        segments: &ROUND_CONTOUR_7,
    },
];

pub const ROUND: AnnounceTextDefinition = AnnounceTextDefinition {
    text: "Round",
    define_text_id: 152,
    font_ids: &ROUND_FONT_IDS,
    color_rgb: [255, 255, 255],
    bounds_centipx: [5655, 9360, 295, 1190],
    contours: &ROUND_CONTOURS,
};

const BLUE_MATCH_FONT_IDS: [u16; 1] = [26];

const BLUE_MATCH_CONTOUR_0: [AnnounceTextSegment; 13] = [
    AnnounceTextSegment::Line([2331, 1180]),
    AnnounceTextSegment::Line([2220, 1180]),
    AnnounceTextSegment::Line([2118, 629]),
    AnnounceTextSegment::Line([1930, 1192]),
    AnnounceTextSegment::Line([1901, 1192]),
    AnnounceTextSegment::Line([1708, 629]),
    AnnounceTextSegment::Line([1608, 1180]),
    AnnounceTextSegment::Line([1497, 1180]),
    AnnounceTextSegment::Line([1657, 322]),
    AnnounceTextSegment::Line([1711, 322]),
    AnnounceTextSegment::Line([1915, 948]),
    AnnounceTextSegment::Line([2104, 322]),
    AnnounceTextSegment::Line([2156, 322]),
];

const BLUE_MATCH_CONTOUR_1: [AnnounceTextSegment; 18] = [
    AnnounceTextSegment::Quad {
        control: [2746, 541],
        to: [2808, 603],
    },
    AnnounceTextSegment::Quad {
        control: [2869, 666],
        to: [2869, 800],
    },
    AnnounceTextSegment::Line([2869, 1025]),
    AnnounceTextSegment::Quad {
        control: [2869, 1109],
        to: [2919, 1135],
    },
    AnnounceTextSegment::Line([2919, 1192]),
    AnnounceTextSegment::Quad {
        control: [2851, 1192],
        to: [2818, 1172],
    },
    AnnounceTextSegment::Quad {
        control: [2784, 1153],
        to: [2769, 1109],
    },
    AnnounceTextSegment::Quad {
        control: [2702, 1192],
        to: [2565, 1192],
    },
    AnnounceTextSegment::Quad {
        control: [2491, 1192],
        to: [2437, 1139],
    },
    AnnounceTextSegment::Quad {
        control: [2382, 1085],
        to: [2382, 1005],
    },
    AnnounceTextSegment::Quad {
        control: [2382, 909],
        to: [2466, 844],
    },
    AnnounceTextSegment::Quad {
        control: [2549, 778],
        to: [2678, 778],
    },
    AnnounceTextSegment::Line([2758, 793]),
    AnnounceTextSegment::Quad {
        control: [2758, 641],
        to: [2622, 641],
    },
    AnnounceTextSegment::Quad {
        control: [2518, 641],
        to: [2462, 697],
    },
    AnnounceTextSegment::Line([2415, 603]),
    AnnounceTextSegment::Quad {
        control: [2446, 578],
        to: [2503, 560],
    },
    AnnounceTextSegment::Quad {
        control: [2559, 541],
        to: [2609, 541],
    },
];

const BLUE_MATCH_CONTOUR_2: [AnnounceTextSegment; 6] = [
    AnnounceTextSegment::Quad {
        control: [2600, 860],
        to: [2547, 903],
    },
    AnnounceTextSegment::Quad {
        control: [2493, 947],
        to: [2493, 1007],
    },
    AnnounceTextSegment::Quad {
        control: [2493, 1104],
        to: [2609, 1104],
    },
    AnnounceTextSegment::Quad {
        control: [2694, 1104],
        to: [2758, 1024],
    },
    AnnounceTextSegment::Line([2758, 872]),
    AnnounceTextSegment::Line([2684, 860]),
];

const BLUE_MATCH_CONTOUR_3: [AnnounceTextSegment; 18] = [
    AnnounceTextSegment::Line([3085, 422]),
    AnnounceTextSegment::Line([3196, 378]),
    AnnounceTextSegment::Line([3196, 553]),
    AnnounceTextSegment::Line([3368, 553]),
    AnnounceTextSegment::Line([3368, 641]),
    AnnounceTextSegment::Line([3196, 641]),
    AnnounceTextSegment::Line([3196, 953]),
    AnnounceTextSegment::Quad {
        control: [3196, 1031],
        to: [3223, 1065],
    },
    AnnounceTextSegment::Quad {
        control: [3249, 1098],
        to: [3308, 1098],
    },
    AnnounceTextSegment::Quad {
        control: [3351, 1098],
        to: [3396, 1077],
    },
    AnnounceTextSegment::Line([3413, 1174]),
    AnnounceTextSegment::Line([3261, 1192]),
    AnnounceTextSegment::Quad {
        control: [3186, 1192],
        to: [3136, 1137],
    },
    AnnounceTextSegment::Quad {
        control: [3085, 1082],
        to: [3085, 997],
    },
    AnnounceTextSegment::Line([3085, 641]),
    AnnounceTextSegment::Line([3012, 641]),
    AnnounceTextSegment::Line([3012, 553]),
    AnnounceTextSegment::Line([3085, 553]),
];

const BLUE_MATCH_CONTOUR_4: [AnnounceTextSegment; 16] = [
    AnnounceTextSegment::Quad {
        control: [3966, 582],
        to: [3993, 603],
    },
    AnnounceTextSegment::Line([3938, 682]),
    AnnounceTextSegment::Quad {
        control: [3920, 666],
        to: [3878, 650],
    },
    AnnounceTextSegment::Line([3793, 635]),
    AnnounceTextSegment::Quad {
        control: [3702, 635],
        to: [3649, 698],
    },
    AnnounceTextSegment::Quad {
        control: [3596, 762],
        to: [3596, 873],
    },
    AnnounceTextSegment::Quad {
        control: [3596, 983],
        to: [3650, 1041],
    },
    AnnounceTextSegment::Quad {
        control: [3705, 1098],
        to: [3801, 1098],
    },
    AnnounceTextSegment::Quad {
        control: [3876, 1098],
        to: [3952, 1041],
    },
    AnnounceTextSegment::Line([3997, 1134]),
    AnnounceTextSegment::Quad {
        control: [3906, 1192],
        to: [3774, 1192],
    },
    AnnounceTextSegment::Quad {
        control: [3646, 1192],
        to: [3562, 1106],
    },
    AnnounceTextSegment::Quad {
        control: [3479, 1019],
        to: [3479, 873],
    },
    AnnounceTextSegment::Quad {
        control: [3479, 723],
        to: [3565, 632],
    },
    AnnounceTextSegment::Quad {
        control: [3652, 541],
        to: [3803, 541],
    },
    AnnounceTextSegment::Line([3909, 561]),
];

const BLUE_MATCH_CONTOUR_5: [AnnounceTextSegment; 16] = [
    AnnounceTextSegment::Line([4226, 622]),
    AnnounceTextSegment::Quad {
        control: [4248, 587],
        to: [4298, 565],
    },
    AnnounceTextSegment::Quad {
        control: [4347, 541],
        to: [4399, 541],
    },
    AnnounceTextSegment::Quad {
        control: [4499, 541],
        to: [4556, 607],
    },
    AnnounceTextSegment::Quad {
        control: [4613, 673],
        to: [4613, 786],
    },
    AnnounceTextSegment::Line([4613, 1180]),
    AnnounceTextSegment::Line([4501, 1180]),
    AnnounceTextSegment::Line([4501, 786]),
    AnnounceTextSegment::Quad {
        control: [4501, 716],
        to: [4466, 675],
    },
    AnnounceTextSegment::Quad {
        control: [4432, 635],
        to: [4369, 635],
    },
    AnnounceTextSegment::Quad {
        control: [4329, 635],
        to: [4288, 659],
    },
    AnnounceTextSegment::Quad {
        control: [4247, 682],
        to: [4226, 714],
    },
    AnnounceTextSegment::Line([4226, 1180]),
    AnnounceTextSegment::Line([4115, 1180]),
    AnnounceTextSegment::Line([4115, 295]),
    AnnounceTextSegment::Line([4226, 295]),
];

const BLUE_MATCH_CONTOURS: [AnnounceTextContour; 6] = [
    AnnounceTextContour {
        start: [2156, 322],
        segments: &BLUE_MATCH_CONTOUR_0,
    },
    AnnounceTextContour {
        start: [2609, 541],
        segments: &BLUE_MATCH_CONTOUR_1,
    },
    AnnounceTextContour {
        start: [2684, 860],
        segments: &BLUE_MATCH_CONTOUR_2,
    },
    AnnounceTextContour {
        start: [3085, 553],
        segments: &BLUE_MATCH_CONTOUR_3,
    },
    AnnounceTextContour {
        start: [3909, 561],
        segments: &BLUE_MATCH_CONTOUR_4,
    },
    AnnounceTextContour {
        start: [4226, 295],
        segments: &BLUE_MATCH_CONTOUR_5,
    },
];

pub const BLUE_MATCH: AnnounceTextDefinition = AnnounceTextDefinition {
    text: "Match",
    define_text_id: 157,
    font_ids: &BLUE_MATCH_FONT_IDS,
    color_rgb: [136, 247, 255],
    bounds_centipx: [1495, 5215, 295, 1190],
    contours: &BLUE_MATCH_CONTOURS,
};

const RED_MATCH_FONT_IDS: [u16; 1] = [26];

const RED_MATCH_CONTOUR_0: [AnnounceTextSegment; 13] = [
    AnnounceTextSegment::Line([2331, 1180]),
    AnnounceTextSegment::Line([2220, 1180]),
    AnnounceTextSegment::Line([2118, 629]),
    AnnounceTextSegment::Line([1930, 1192]),
    AnnounceTextSegment::Line([1901, 1192]),
    AnnounceTextSegment::Line([1708, 629]),
    AnnounceTextSegment::Line([1608, 1180]),
    AnnounceTextSegment::Line([1497, 1180]),
    AnnounceTextSegment::Line([1657, 322]),
    AnnounceTextSegment::Line([1711, 322]),
    AnnounceTextSegment::Line([1915, 948]),
    AnnounceTextSegment::Line([2104, 322]),
    AnnounceTextSegment::Line([2156, 322]),
];

const RED_MATCH_CONTOUR_1: [AnnounceTextSegment; 18] = [
    AnnounceTextSegment::Quad {
        control: [2746, 541],
        to: [2808, 603],
    },
    AnnounceTextSegment::Quad {
        control: [2869, 666],
        to: [2869, 800],
    },
    AnnounceTextSegment::Line([2869, 1025]),
    AnnounceTextSegment::Quad {
        control: [2869, 1109],
        to: [2919, 1135],
    },
    AnnounceTextSegment::Line([2919, 1192]),
    AnnounceTextSegment::Quad {
        control: [2851, 1192],
        to: [2818, 1172],
    },
    AnnounceTextSegment::Quad {
        control: [2784, 1153],
        to: [2769, 1109],
    },
    AnnounceTextSegment::Quad {
        control: [2702, 1192],
        to: [2565, 1192],
    },
    AnnounceTextSegment::Quad {
        control: [2491, 1192],
        to: [2437, 1139],
    },
    AnnounceTextSegment::Quad {
        control: [2382, 1085],
        to: [2382, 1005],
    },
    AnnounceTextSegment::Quad {
        control: [2382, 909],
        to: [2466, 844],
    },
    AnnounceTextSegment::Quad {
        control: [2549, 778],
        to: [2678, 778],
    },
    AnnounceTextSegment::Line([2758, 793]),
    AnnounceTextSegment::Quad {
        control: [2758, 641],
        to: [2622, 641],
    },
    AnnounceTextSegment::Quad {
        control: [2518, 641],
        to: [2462, 697],
    },
    AnnounceTextSegment::Line([2415, 603]),
    AnnounceTextSegment::Quad {
        control: [2446, 578],
        to: [2503, 560],
    },
    AnnounceTextSegment::Quad {
        control: [2559, 541],
        to: [2609, 541],
    },
];

const RED_MATCH_CONTOUR_2: [AnnounceTextSegment; 6] = [
    AnnounceTextSegment::Quad {
        control: [2600, 860],
        to: [2547, 903],
    },
    AnnounceTextSegment::Quad {
        control: [2493, 947],
        to: [2493, 1007],
    },
    AnnounceTextSegment::Quad {
        control: [2493, 1104],
        to: [2609, 1104],
    },
    AnnounceTextSegment::Quad {
        control: [2694, 1104],
        to: [2758, 1024],
    },
    AnnounceTextSegment::Line([2758, 872]),
    AnnounceTextSegment::Line([2684, 860]),
];

const RED_MATCH_CONTOUR_3: [AnnounceTextSegment; 18] = [
    AnnounceTextSegment::Line([3085, 422]),
    AnnounceTextSegment::Line([3196, 378]),
    AnnounceTextSegment::Line([3196, 553]),
    AnnounceTextSegment::Line([3368, 553]),
    AnnounceTextSegment::Line([3368, 641]),
    AnnounceTextSegment::Line([3196, 641]),
    AnnounceTextSegment::Line([3196, 953]),
    AnnounceTextSegment::Quad {
        control: [3196, 1031],
        to: [3223, 1065],
    },
    AnnounceTextSegment::Quad {
        control: [3249, 1098],
        to: [3308, 1098],
    },
    AnnounceTextSegment::Quad {
        control: [3351, 1098],
        to: [3396, 1077],
    },
    AnnounceTextSegment::Line([3413, 1174]),
    AnnounceTextSegment::Line([3261, 1192]),
    AnnounceTextSegment::Quad {
        control: [3186, 1192],
        to: [3136, 1137],
    },
    AnnounceTextSegment::Quad {
        control: [3085, 1082],
        to: [3085, 997],
    },
    AnnounceTextSegment::Line([3085, 641]),
    AnnounceTextSegment::Line([3012, 641]),
    AnnounceTextSegment::Line([3012, 553]),
    AnnounceTextSegment::Line([3085, 553]),
];

const RED_MATCH_CONTOUR_4: [AnnounceTextSegment; 16] = [
    AnnounceTextSegment::Quad {
        control: [3966, 582],
        to: [3993, 603],
    },
    AnnounceTextSegment::Line([3938, 682]),
    AnnounceTextSegment::Quad {
        control: [3920, 666],
        to: [3878, 650],
    },
    AnnounceTextSegment::Line([3793, 635]),
    AnnounceTextSegment::Quad {
        control: [3702, 635],
        to: [3649, 698],
    },
    AnnounceTextSegment::Quad {
        control: [3596, 762],
        to: [3596, 873],
    },
    AnnounceTextSegment::Quad {
        control: [3596, 983],
        to: [3650, 1041],
    },
    AnnounceTextSegment::Quad {
        control: [3705, 1098],
        to: [3801, 1098],
    },
    AnnounceTextSegment::Quad {
        control: [3876, 1098],
        to: [3952, 1041],
    },
    AnnounceTextSegment::Line([3997, 1134]),
    AnnounceTextSegment::Quad {
        control: [3906, 1192],
        to: [3774, 1192],
    },
    AnnounceTextSegment::Quad {
        control: [3646, 1192],
        to: [3562, 1106],
    },
    AnnounceTextSegment::Quad {
        control: [3479, 1019],
        to: [3479, 873],
    },
    AnnounceTextSegment::Quad {
        control: [3479, 723],
        to: [3565, 632],
    },
    AnnounceTextSegment::Quad {
        control: [3652, 541],
        to: [3803, 541],
    },
    AnnounceTextSegment::Line([3909, 561]),
];

const RED_MATCH_CONTOUR_5: [AnnounceTextSegment; 16] = [
    AnnounceTextSegment::Line([4226, 622]),
    AnnounceTextSegment::Quad {
        control: [4248, 587],
        to: [4298, 565],
    },
    AnnounceTextSegment::Quad {
        control: [4347, 541],
        to: [4399, 541],
    },
    AnnounceTextSegment::Quad {
        control: [4499, 541],
        to: [4556, 607],
    },
    AnnounceTextSegment::Quad {
        control: [4613, 673],
        to: [4613, 786],
    },
    AnnounceTextSegment::Line([4613, 1180]),
    AnnounceTextSegment::Line([4501, 1180]),
    AnnounceTextSegment::Line([4501, 786]),
    AnnounceTextSegment::Quad {
        control: [4501, 716],
        to: [4466, 675],
    },
    AnnounceTextSegment::Quad {
        control: [4432, 635],
        to: [4369, 635],
    },
    AnnounceTextSegment::Quad {
        control: [4329, 635],
        to: [4288, 659],
    },
    AnnounceTextSegment::Quad {
        control: [4247, 682],
        to: [4226, 714],
    },
    AnnounceTextSegment::Line([4226, 1180]),
    AnnounceTextSegment::Line([4115, 1180]),
    AnnounceTextSegment::Line([4115, 295]),
    AnnounceTextSegment::Line([4226, 295]),
];

const RED_MATCH_CONTOURS: [AnnounceTextContour; 6] = [
    AnnounceTextContour {
        start: [2156, 322],
        segments: &RED_MATCH_CONTOUR_0,
    },
    AnnounceTextContour {
        start: [2609, 541],
        segments: &RED_MATCH_CONTOUR_1,
    },
    AnnounceTextContour {
        start: [2684, 860],
        segments: &RED_MATCH_CONTOUR_2,
    },
    AnnounceTextContour {
        start: [3085, 553],
        segments: &RED_MATCH_CONTOUR_3,
    },
    AnnounceTextContour {
        start: [3909, 561],
        segments: &RED_MATCH_CONTOUR_4,
    },
    AnnounceTextContour {
        start: [4226, 295],
        segments: &RED_MATCH_CONTOUR_5,
    },
];

pub const RED_MATCH: AnnounceTextDefinition = AnnounceTextDefinition {
    text: "Match",
    define_text_id: 160,
    font_ids: &RED_MATCH_FONT_IDS,
    color_rgb: [255, 178, 51],
    bounds_centipx: [1495, 5215, 295, 1190],
    contours: &RED_MATCH_CONTOURS,
};

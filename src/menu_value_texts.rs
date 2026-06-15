// Generated from gravity_arcade.swf menu value DefineText tags.
// Coordinates are stored as SWF text-local centipixels; divide by 100 for pixels.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuValueSegment {
    Line([i16; 2]),
    Quad { control: [i16; 2], to: [i16; 2] },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MenuValueContour {
    pub(super) start: [i16; 2],
    pub(super) segments: &'static [MenuValueSegment],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MenuValueDefinition {
    pub(super) text: &'static str,
    pub(super) define_text_id: u16,
    pub(super) font_ids: &'static [u16],
    pub(super) color_rgb: [u8; 3],
    pub(super) bounds_centipx: [i16; 4],
    pub(super) contours: &'static [MenuValueContour],
}

const QUESTION_FONT_IDS: [u16; 1] = [54];

const QUESTION_CONTOUR_0: [MenuValueSegment; 18] = [
    MenuValueSegment::Quad {
        control: [482, 412],
        to: [482, 499],
    },
    MenuValueSegment::Quad {
        control: [482, 559],
        to: [458, 601],
    },
    MenuValueSegment::Quad {
        control: [435, 642],
        to: [377, 697],
    },
    MenuValueSegment::Quad {
        control: [319, 753],
        to: [301, 790],
    },
    MenuValueSegment::Quad {
        control: [284, 827],
        to: [284, 871],
    },
    MenuValueSegment::Quad {
        control: [284, 880],
        to: [292, 920],
    },
    MenuValueSegment::Line([179, 920]),
    MenuValueSegment::Line([169, 891]),
    MenuValueSegment::Quad {
        control: [159, 865],
        to: [159, 845],
    },
    MenuValueSegment::Quad {
        control: [159, 809],
        to: [170, 775],
    },
    MenuValueSegment::Quad {
        control: [179, 741],
        to: [199, 711],
    },
    MenuValueSegment::Quad {
        control: [219, 681],
        to: [279, 616],
    },
    MenuValueSegment::Quad {
        control: [338, 551],
        to: [338, 509],
    },
    MenuValueSegment::Quad {
        control: [338, 426],
        to: [230, 426],
    },
    MenuValueSegment::Quad {
        control: [178, 426],
        to: [121, 479],
    },
    MenuValueSegment::Line([61, 366]),
    MenuValueSegment::Quad {
        control: [136, 307],
        to: [259, 307],
    },
    MenuValueSegment::Quad {
        control: [354, 307],
        to: [418, 360],
    },
];

const QUESTION_CONTOUR_1: [MenuValueSegment; 8] = [
    MenuValueSegment::Quad {
        control: [202, 1000],
        to: [241, 1000],
    },
    MenuValueSegment::Quad {
        control: [281, 1000],
        to: [309, 1028],
    },
    MenuValueSegment::Quad {
        control: [338, 1056],
        to: [338, 1096],
    },
    MenuValueSegment::Quad {
        control: [338, 1135],
        to: [309, 1164],
    },
    MenuValueSegment::Quad {
        control: [281, 1192],
        to: [241, 1192],
    },
    MenuValueSegment::Quad {
        control: [202, 1192],
        to: [173, 1164],
    },
    MenuValueSegment::Quad {
        control: [145, 1135],
        to: [145, 1096],
    },
    MenuValueSegment::Quad {
        control: [145, 1056],
        to: [173, 1028],
    },
];

const QUESTION_CONTOURS: [MenuValueContour; 2] = [
    MenuValueContour {
        start: [418, 360],
        segments: &QUESTION_CONTOUR_0,
    },
    MenuValueContour {
        start: [173, 1028],
        segments: &QUESTION_CONTOUR_1,
    },
];

pub const QUESTION: MenuValueDefinition = MenuValueDefinition {
    text: "?",
    define_text_id: 55,
    font_ids: &QUESTION_FONT_IDS,
    color_rgb: [255, 255, 255],
    bounds_centipx: [60, 1050, 305, 1190],
    contours: &QUESTION_CONTOURS,
};

const MATCH_SINGLE_FONT_IDS: [u16; 1] = [26];

const MATCH_SINGLE_CONTOUR_0: [MenuValueSegment; 22] = [
    MenuValueSegment::Line([2371, 844]),
    MenuValueSegment::Quad {
        control: [2283, 773],
        to: [2194, 773],
    },
    MenuValueSegment::Quad {
        control: [2141, 773],
        to: [2105, 798],
    },
    MenuValueSegment::Quad {
        control: [2068, 823],
        to: [2068, 861],
    },
    MenuValueSegment::Quad {
        control: [2068, 942],
        to: [2160, 983],
    },
    MenuValueSegment::Line([2266, 1031]),
    MenuValueSegment::Quad {
        control: [2363, 1077],
        to: [2408, 1133],
    },
    MenuValueSegment::Quad {
        control: [2452, 1191],
        to: [2452, 1277],
    },
    MenuValueSegment::Quad {
        control: [2452, 1389],
        to: [2374, 1453],
    },
    MenuValueSegment::Quad {
        control: [2294, 1516],
        to: [2155, 1516],
    },
    MenuValueSegment::Quad {
        control: [2022, 1516],
        to: [1907, 1450],
    },
    MenuValueSegment::Line([1958, 1309]),
    MenuValueSegment::Quad {
        control: [2083, 1391],
        to: [2158, 1391],
    },
    MenuValueSegment::Quad {
        control: [2296, 1391],
        to: [2296, 1275],
    },
    MenuValueSegment::Quad {
        control: [2296, 1192],
        to: [2163, 1133],
    },
    MenuValueSegment::Quad {
        control: [2061, 1086],
        to: [2025, 1062],
    },
    MenuValueSegment::Quad {
        control: [1989, 1038],
        to: [1964, 1008],
    },
    MenuValueSegment::Quad {
        control: [1938, 977],
        to: [1925, 942],
    },
    MenuValueSegment::Quad {
        control: [1911, 906],
        to: [1911, 867],
    },
    MenuValueSegment::Quad {
        control: [1911, 764],
        to: [1986, 706],
    },
    MenuValueSegment::Quad {
        control: [2061, 648],
        to: [2183, 648],
    },
    MenuValueSegment::Quad {
        control: [2274, 648],
        to: [2413, 706],
    },
];

const MATCH_SINGLE_CONTOUR_1: [MenuValueSegment; 8] = [
    MenuValueSegment::Quad {
        control: [2802, 347],
        to: [2830, 375],
    },
    MenuValueSegment::Quad {
        control: [2857, 402],
        to: [2857, 439],
    },
    MenuValueSegment::Quad {
        control: [2857, 477],
        to: [2830, 505],
    },
    MenuValueSegment::Quad {
        control: [2802, 531],
        to: [2764, 531],
    },
    MenuValueSegment::Quad {
        control: [2727, 531],
        to: [2700, 505],
    },
    MenuValueSegment::Quad {
        control: [2672, 477],
        to: [2672, 439],
    },
    MenuValueSegment::Quad {
        control: [2672, 400],
        to: [2699, 373],
    },
    MenuValueSegment::Quad {
        control: [2725, 347],
        to: [2764, 347],
    },
];

const MATCH_SINGLE_CONTOUR_2: [MenuValueSegment; 6] = [
    MenuValueSegment::Line([2832, 1500]),
    MenuValueSegment::Line([2683, 1500]),
    MenuValueSegment::Line([2683, 789]),
    MenuValueSegment::Line([2568, 789]),
    MenuValueSegment::Line([2568, 664]),
    MenuValueSegment::Line([2832, 664]),
];

const MATCH_SINGLE_CONTOUR_3: [MenuValueSegment; 14] = [
    MenuValueSegment::Line([3729, 1500]),
    MenuValueSegment::Line([3580, 1500]),
    MenuValueSegment::Line([3580, 1014]),
    MenuValueSegment::Quad {
        control: [3580, 880],
        to: [3541, 827],
    },
    MenuValueSegment::Quad {
        control: [3501, 773],
        to: [3405, 773],
    },
    MenuValueSegment::Quad {
        control: [3355, 773],
        to: [3299, 803],
    },
    MenuValueSegment::Quad {
        control: [3244, 834],
        to: [3215, 878],
    },
    MenuValueSegment::Line([3215, 1500]),
    MenuValueSegment::Line([3066, 1500]),
    MenuValueSegment::Line([3066, 664]),
    MenuValueSegment::Line([3168, 664]),
    MenuValueSegment::Line([3215, 772]),
    MenuValueSegment::Quad {
        control: [3288, 648],
        to: [3454, 648],
    },
    MenuValueSegment::Quad {
        control: [3729, 648],
        to: [3729, 983],
    },
];

const MATCH_SINGLE_CONTOUR_4: [MenuValueSegment; 32] = [
    MenuValueSegment::Line([4490, 769]),
    MenuValueSegment::Quad {
        control: [4548, 842],
        to: [4548, 962],
    },
    MenuValueSegment::Quad {
        control: [4548, 1089],
        to: [4468, 1175],
    },
    MenuValueSegment::Quad {
        control: [4390, 1261],
        to: [4262, 1273],
    },
    MenuValueSegment::Line([4138, 1286]),
    MenuValueSegment::Line([4080, 1303]),
    MenuValueSegment::Quad {
        control: [4043, 1317],
        to: [4043, 1341],
    },
    MenuValueSegment::Quad {
        control: [4043, 1372],
        to: [4119, 1372],
    },
    MenuValueSegment::Line([4224, 1361]),
    MenuValueSegment::Line([4330, 1348]),
    MenuValueSegment::Quad {
        control: [4454, 1348],
        to: [4522, 1408],
    },
    MenuValueSegment::Quad {
        control: [4591, 1466],
        to: [4591, 1570],
    },
    MenuValueSegment::Quad {
        control: [4591, 1686],
        to: [4488, 1758],
    },
    MenuValueSegment::Quad {
        control: [4385, 1828],
        to: [4226, 1828],
    },
    MenuValueSegment::Quad {
        control: [4144, 1828],
        to: [4055, 1800],
    },
    MenuValueSegment::Quad {
        control: [3965, 1770],
        to: [3910, 1730],
    },
    MenuValueSegment::Line([3991, 1611]),
    MenuValueSegment::Quad {
        control: [4121, 1697],
        to: [4230, 1697],
    },
    MenuValueSegment::Quad {
        control: [4330, 1697],
        to: [4390, 1662],
    },
    MenuValueSegment::Quad {
        control: [4448, 1628],
        to: [4448, 1577],
    },
    MenuValueSegment::Quad {
        control: [4448, 1475],
        to: [4301, 1475],
    },
    MenuValueSegment::Line([4210, 1488]),
    MenuValueSegment::Line([4107, 1500]),
    MenuValueSegment::Quad {
        control: [3929, 1500],
        to: [3929, 1366],
    },
    MenuValueSegment::Quad {
        control: [3929, 1323],
        to: [3971, 1291],
    },
    MenuValueSegment::Quad {
        control: [4013, 1256],
        to: [4074, 1242],
    },
    MenuValueSegment::Quad {
        control: [3898, 1159],
        to: [3898, 955],
    },
    MenuValueSegment::Quad {
        control: [3898, 823],
        to: [3990, 736],
    },
    MenuValueSegment::Quad {
        control: [4080, 648],
        to: [4215, 648],
    },
    MenuValueSegment::Quad {
        control: [4338, 648],
        to: [4408, 698],
    },
    MenuValueSegment::Line([4482, 609]),
    MenuValueSegment::Line([4579, 702]),
];

const MATCH_SINGLE_CONTOUR_5: [MenuValueSegment; 8] = [
    MenuValueSegment::Quad {
        control: [4148, 767],
        to: [4099, 822],
    },
    MenuValueSegment::Quad {
        control: [4051, 877],
        to: [4051, 955],
    },
    MenuValueSegment::Quad {
        control: [4051, 1042],
        to: [4098, 1100],
    },
    MenuValueSegment::Quad {
        control: [4144, 1158],
        to: [4226, 1158],
    },
    MenuValueSegment::Quad {
        control: [4304, 1158],
        to: [4349, 1102],
    },
    MenuValueSegment::Quad {
        control: [4393, 1045],
        to: [4393, 955],
    },
    MenuValueSegment::Quad {
        control: [4393, 877],
        to: [4346, 822],
    },
    MenuValueSegment::Quad {
        control: [4298, 767],
        to: [4226, 767],
    },
];

const MATCH_SINGLE_CONTOUR_6: [MenuValueSegment; 7] = [
    MenuValueSegment::Line([5046, 1516]),
    MenuValueSegment::Quad {
        control: [4757, 1516],
        to: [4757, 1264],
    },
    MenuValueSegment::Line([4757, 320]),
    MenuValueSegment::Line([4906, 320]),
    MenuValueSegment::Line([4906, 1239]),
    MenuValueSegment::Quad {
        control: [4906, 1306],
        to: [4945, 1345],
    },
    MenuValueSegment::Quad {
        control: [4984, 1383],
        to: [5046, 1383],
    },
];

const MATCH_SINGLE_CONTOUR_7: [MenuValueSegment; 5] = [
    MenuValueSegment::Quad {
        control: [5462, 773],
        to: [5394, 838],
    },
    MenuValueSegment::Quad {
        control: [5330, 898],
        to: [5321, 989],
    },
    MenuValueSegment::Line([5785, 989]),
    MenuValueSegment::Quad {
        control: [5785, 898],
        to: [5729, 839],
    },
    MenuValueSegment::Quad {
        control: [5666, 773],
        to: [5560, 773],
    },
];

const MATCH_SINGLE_CONTOUR_8: [MenuValueSegment; 14] = [
    MenuValueSegment::Quad {
        control: [5469, 1391],
        to: [5580, 1391],
    },
    MenuValueSegment::Quad {
        control: [5708, 1391],
        to: [5793, 1317],
    },
    MenuValueSegment::Line([5855, 1423]),
    MenuValueSegment::Quad {
        control: [5821, 1458],
        to: [5751, 1483],
    },
    MenuValueSegment::Quad {
        control: [5662, 1516],
        to: [5552, 1516],
    },
    MenuValueSegment::Quad {
        control: [5394, 1516],
        to: [5283, 1409],
    },
    MenuValueSegment::Quad {
        control: [5162, 1291],
        to: [5162, 1092],
    },
    MenuValueSegment::Quad {
        control: [5162, 884],
        to: [5287, 759],
    },
    MenuValueSegment::Quad {
        control: [5399, 648],
        to: [5554, 648],
    },
    MenuValueSegment::Quad {
        control: [5732, 648],
        to: [5833, 748],
    },
    MenuValueSegment::Quad {
        control: [5932, 845],
        to: [5932, 1006],
    },
    MenuValueSegment::Quad {
        control: [5932, 1055],
        to: [5921, 1097],
    },
    MenuValueSegment::Line([5318, 1097]),
    MenuValueSegment::Quad {
        control: [5318, 1244],
        to: [5398, 1322],
    },
];

const MATCH_SINGLE_CONTOUR_9: [MenuValueSegment; 24] = [
    MenuValueSegment::Quad {
        control: [7688, 802],
        to: [7688, 941],
    },
    MenuValueSegment::Line([7688, 1500]),
    MenuValueSegment::Line([7540, 1500]),
    MenuValueSegment::Line([7540, 970]),
    MenuValueSegment::Quad {
        control: [7540, 773],
        to: [7368, 773],
    },
    MenuValueSegment::Quad {
        control: [7315, 773],
        to: [7268, 806],
    },
    MenuValueSegment::Quad {
        control: [7221, 839],
        to: [7204, 881],
    },
    MenuValueSegment::Line([7204, 1500]),
    MenuValueSegment::Line([7056, 1500]),
    MenuValueSegment::Line([7056, 906]),
    MenuValueSegment::Quad {
        control: [7056, 844],
        to: [7009, 809],
    },
    MenuValueSegment::Quad {
        control: [6962, 773],
        to: [6885, 773],
    },
    MenuValueSegment::Quad {
        control: [6842, 773],
        to: [6792, 808],
    },
    MenuValueSegment::Quad {
        control: [6740, 842],
        to: [6720, 883],
    },
    MenuValueSegment::Line([6720, 1500]),
    MenuValueSegment::Line([6571, 1500]),
    MenuValueSegment::Line([6571, 664]),
    MenuValueSegment::Line([6668, 664]),
    MenuValueSegment::Line([6717, 761]),
    MenuValueSegment::Quad {
        control: [6803, 648],
        to: [6932, 648],
    },
    MenuValueSegment::Quad {
        control: [7112, 648],
        to: [7184, 759],
    },
    MenuValueSegment::Quad {
        control: [7209, 712],
        to: [7276, 680],
    },
    MenuValueSegment::Quad {
        control: [7345, 648],
        to: [7417, 648],
    },
    MenuValueSegment::Quad {
        control: [7546, 648],
        to: [7617, 725],
    },
];

const MATCH_SINGLE_CONTOUR_10: [MenuValueSegment; 18] = [
    MenuValueSegment::Quad {
        control: [8343, 648],
        to: [8426, 731],
    },
    MenuValueSegment::Quad {
        control: [8508, 814],
        to: [8508, 994],
    },
    MenuValueSegment::Line([8508, 1294]),
    MenuValueSegment::Quad {
        control: [8508, 1405],
        to: [8573, 1441],
    },
    MenuValueSegment::Line([8573, 1516]),
    MenuValueSegment::Quad {
        control: [8482, 1516],
        to: [8439, 1489],
    },
    MenuValueSegment::Quad {
        control: [8393, 1464],
        to: [8373, 1405],
    },
    MenuValueSegment::Quad {
        control: [8284, 1516],
        to: [8101, 1516],
    },
    MenuValueSegment::Quad {
        control: [8003, 1516],
        to: [7931, 1445],
    },
    MenuValueSegment::Quad {
        control: [7858, 1373],
        to: [7858, 1267],
    },
    MenuValueSegment::Quad {
        control: [7858, 1139],
        to: [7970, 1052],
    },
    MenuValueSegment::Quad {
        control: [8081, 964],
        to: [8253, 964],
    },
    MenuValueSegment::Line([8359, 984]),
    MenuValueSegment::Quad {
        control: [8359, 781],
        to: [8178, 781],
    },
    MenuValueSegment::Quad {
        control: [8039, 781],
        to: [7964, 856],
    },
    MenuValueSegment::Line([7901, 731]),
    MenuValueSegment::Quad {
        control: [7943, 697],
        to: [8018, 673],
    },
    MenuValueSegment::Quad {
        control: [8093, 648],
        to: [8161, 648],
    },
];

const MATCH_SINGLE_CONTOUR_11: [MenuValueSegment; 6] = [
    MenuValueSegment::Quad {
        control: [8148, 1073],
        to: [8078, 1131],
    },
    MenuValueSegment::Quad {
        control: [8006, 1189],
        to: [8006, 1269],
    },
    MenuValueSegment::Quad {
        control: [8006, 1398],
        to: [8161, 1398],
    },
    MenuValueSegment::Quad {
        control: [8273, 1398],
        to: [8359, 1292],
    },
    MenuValueSegment::Line([8359, 1089]),
    MenuValueSegment::Line([8261, 1073]),
];

const MATCH_SINGLE_CONTOUR_12: [MenuValueSegment; 18] = [
    MenuValueSegment::Line([8794, 489]),
    MenuValueSegment::Line([8943, 431]),
    MenuValueSegment::Line([8943, 664]),
    MenuValueSegment::Line([9172, 664]),
    MenuValueSegment::Line([9172, 781]),
    MenuValueSegment::Line([8943, 781]),
    MenuValueSegment::Line([8943, 1197]),
    MenuValueSegment::Quad {
        control: [8943, 1302],
        to: [8979, 1347],
    },
    MenuValueSegment::Quad {
        control: [9013, 1391],
        to: [9093, 1391],
    },
    MenuValueSegment::Quad {
        control: [9149, 1391],
        to: [9210, 1362],
    },
    MenuValueSegment::Line([9232, 1492]),
    MenuValueSegment::Line([9030, 1516]),
    MenuValueSegment::Quad {
        control: [8930, 1516],
        to: [8863, 1442],
    },
    MenuValueSegment::Quad {
        control: [8794, 1369],
        to: [8794, 1256],
    },
    MenuValueSegment::Line([8794, 781]),
    MenuValueSegment::Line([8698, 781]),
    MenuValueSegment::Line([8698, 664]),
    MenuValueSegment::Line([8794, 664]),
];

const MATCH_SINGLE_CONTOUR_13: [MenuValueSegment; 16] = [
    MenuValueSegment::Quad {
        control: [9972, 703],
        to: [10008, 731],
    },
    MenuValueSegment::Line([9934, 836]),
    MenuValueSegment::Quad {
        control: [9911, 814],
        to: [9854, 794],
    },
    MenuValueSegment::Line([9740, 773]),
    MenuValueSegment::Quad {
        control: [9620, 773],
        to: [9548, 858],
    },
    MenuValueSegment::Quad {
        control: [9478, 942],
        to: [9478, 1091],
    },
    MenuValueSegment::Quad {
        control: [9478, 1238],
        to: [9550, 1314],
    },
    MenuValueSegment::Quad {
        control: [9623, 1391],
        to: [9751, 1391],
    },
    MenuValueSegment::Quad {
        control: [9851, 1391],
        to: [9953, 1314],
    },
    MenuValueSegment::Line([10012, 1439]),
    MenuValueSegment::Quad {
        control: [9892, 1516],
        to: [9715, 1516],
    },
    MenuValueSegment::Quad {
        control: [9545, 1516],
        to: [9432, 1402],
    },
    MenuValueSegment::Quad {
        control: [9322, 1286],
        to: [9322, 1091],
    },
    MenuValueSegment::Quad {
        control: [9322, 891],
        to: [9437, 769],
    },
    MenuValueSegment::Quad {
        control: [9553, 648],
        to: [9754, 648],
    },
    MenuValueSegment::Line([9895, 675]),
];

const MATCH_SINGLE_CONTOUR_14: [MenuValueSegment; 16] = [
    MenuValueSegment::Line([10315, 756]),
    MenuValueSegment::Quad {
        control: [10344, 709],
        to: [10410, 680],
    },
    MenuValueSegment::Quad {
        control: [10476, 648],
        to: [10546, 648],
    },
    MenuValueSegment::Quad {
        control: [10679, 648],
        to: [10755, 736],
    },
    MenuValueSegment::Quad {
        control: [10830, 823],
        to: [10830, 975],
    },
    MenuValueSegment::Line([10830, 1500]),
    MenuValueSegment::Line([10682, 1500]),
    MenuValueSegment::Line([10682, 975]),
    MenuValueSegment::Quad {
        control: [10682, 881],
        to: [10635, 827],
    },
    MenuValueSegment::Quad {
        control: [10590, 773],
        to: [10505, 773],
    },
    MenuValueSegment::Quad {
        control: [10452, 773],
        to: [10398, 805],
    },
    MenuValueSegment::Quad {
        control: [10343, 836],
        to: [10315, 878],
    },
    MenuValueSegment::Line([10315, 1500]),
    MenuValueSegment::Line([10166, 1500]),
    MenuValueSegment::Line([10166, 320]),
    MenuValueSegment::Line([10315, 320]),
];

const MATCH_SINGLE_CONTOURS: [MenuValueContour; 15] = [
    MenuValueContour {
        start: [2413, 706],
        segments: &MATCH_SINGLE_CONTOUR_0,
    },
    MenuValueContour {
        start: [2764, 347],
        segments: &MATCH_SINGLE_CONTOUR_1,
    },
    MenuValueContour {
        start: [2832, 664],
        segments: &MATCH_SINGLE_CONTOUR_2,
    },
    MenuValueContour {
        start: [3729, 983],
        segments: &MATCH_SINGLE_CONTOUR_3,
    },
    MenuValueContour {
        start: [4579, 702],
        segments: &MATCH_SINGLE_CONTOUR_4,
    },
    MenuValueContour {
        start: [4226, 767],
        segments: &MATCH_SINGLE_CONTOUR_5,
    },
    MenuValueContour {
        start: [5046, 1383],
        segments: &MATCH_SINGLE_CONTOUR_6,
    },
    MenuValueContour {
        start: [5560, 773],
        segments: &MATCH_SINGLE_CONTOUR_7,
    },
    MenuValueContour {
        start: [5398, 1322],
        segments: &MATCH_SINGLE_CONTOUR_8,
    },
    MenuValueContour {
        start: [7617, 725],
        segments: &MATCH_SINGLE_CONTOUR_9,
    },
    MenuValueContour {
        start: [8161, 648],
        segments: &MATCH_SINGLE_CONTOUR_10,
    },
    MenuValueContour {
        start: [8261, 1073],
        segments: &MATCH_SINGLE_CONTOUR_11,
    },
    MenuValueContour {
        start: [8794, 664],
        segments: &MATCH_SINGLE_CONTOUR_12,
    },
    MenuValueContour {
        start: [9895, 675],
        segments: &MATCH_SINGLE_CONTOUR_13,
    },
    MenuValueContour {
        start: [10315, 320],
        segments: &MATCH_SINGLE_CONTOUR_14,
    },
];

pub const MATCH_SINGLE: MenuValueDefinition = MenuValueDefinition {
    text: "single match",
    define_text_id: 43,
    font_ids: &MATCH_SINGLE_FONT_IDS,
    color_rgb: [255, 255, 255],
    bounds_centipx: [1905, 11635, 320, 1830],
    contours: &MATCH_SINGLE_CONTOURS,
};

const MATCH_BEST_OF_3_FONT_IDS: [u16; 1] = [26];

const MATCH_BEST_OF_3_CONTOUR_0: [MenuValueSegment; 13] = [
    MenuValueSegment::Line([3500, 725]),
    MenuValueSegment::Quad {
        control: [3520, 697],
        to: [3579, 672],
    },
    MenuValueSegment::Quad {
        control: [3637, 648],
        to: [3693, 648],
    },
    MenuValueSegment::Quad {
        control: [3865, 648],
        to: [3972, 767],
    },
    MenuValueSegment::Quad {
        control: [4078, 886],
        to: [4078, 1067],
    },
    MenuValueSegment::Quad {
        control: [4078, 1277],
        to: [3972, 1397],
    },
    MenuValueSegment::Quad {
        control: [3864, 1516],
        to: [3681, 1516],
    },
    MenuValueSegment::Quad {
        control: [3622, 1516],
        to: [3565, 1494],
    },
    MenuValueSegment::Quad {
        control: [3508, 1472],
        to: [3478, 1441],
    },
    MenuValueSegment::Line([3425, 1516]),
    MenuValueSegment::Line([3351, 1516]),
    MenuValueSegment::Line([3351, 320]),
    MenuValueSegment::Line([3500, 320]),
];

const MATCH_BEST_OF_3_CONTOUR_1: [MenuValueSegment; 9] = [
    MenuValueSegment::Line([3500, 1322]),
    MenuValueSegment::Quad {
        control: [3500, 1334],
        to: [3554, 1362],
    },
    MenuValueSegment::Quad {
        control: [3611, 1391],
        to: [3639, 1391],
    },
    MenuValueSegment::Quad {
        control: [3790, 1391],
        to: [3856, 1319],
    },
    MenuValueSegment::Quad {
        control: [3922, 1245],
        to: [3922, 1075],
    },
    MenuValueSegment::Quad {
        control: [3922, 933],
        to: [3845, 853],
    },
    MenuValueSegment::Quad {
        control: [3768, 773],
        to: [3639, 773],
    },
    MenuValueSegment::Quad {
        control: [3612, 773],
        to: [3564, 797],
    },
    MenuValueSegment::Line([3500, 839]),
];

const MATCH_BEST_OF_3_CONTOUR_2: [MenuValueSegment; 5] = [
    MenuValueSegment::Quad {
        control: [4487, 773],
        to: [4419, 838],
    },
    MenuValueSegment::Quad {
        control: [4355, 898],
        to: [4346, 989],
    },
    MenuValueSegment::Line([4810, 989]),
    MenuValueSegment::Quad {
        control: [4810, 898],
        to: [4754, 839],
    },
    MenuValueSegment::Quad {
        control: [4691, 773],
        to: [4585, 773],
    },
];

const MATCH_BEST_OF_3_CONTOUR_3: [MenuValueSegment; 14] = [
    MenuValueSegment::Quad {
        control: [4494, 1391],
        to: [4605, 1391],
    },
    MenuValueSegment::Quad {
        control: [4733, 1391],
        to: [4818, 1317],
    },
    MenuValueSegment::Line([4880, 1423]),
    MenuValueSegment::Quad {
        control: [4846, 1458],
        to: [4776, 1483],
    },
    MenuValueSegment::Quad {
        control: [4687, 1516],
        to: [4577, 1516],
    },
    MenuValueSegment::Quad {
        control: [4419, 1516],
        to: [4308, 1409],
    },
    MenuValueSegment::Quad {
        control: [4187, 1291],
        to: [4187, 1092],
    },
    MenuValueSegment::Quad {
        control: [4187, 884],
        to: [4312, 759],
    },
    MenuValueSegment::Quad {
        control: [4424, 648],
        to: [4579, 648],
    },
    MenuValueSegment::Quad {
        control: [4757, 648],
        to: [4858, 748],
    },
    MenuValueSegment::Quad {
        control: [4957, 845],
        to: [4957, 1006],
    },
    MenuValueSegment::Quad {
        control: [4957, 1055],
        to: [4946, 1097],
    },
    MenuValueSegment::Line([4343, 1097]),
    MenuValueSegment::Quad {
        control: [4343, 1244],
        to: [4422, 1322],
    },
];

const MATCH_BEST_OF_3_CONTOUR_4: [MenuValueSegment; 22] = [
    MenuValueSegment::Line([5526, 844]),
    MenuValueSegment::Quad {
        control: [5438, 773],
        to: [5349, 773],
    },
    MenuValueSegment::Quad {
        control: [5296, 773],
        to: [5260, 798],
    },
    MenuValueSegment::Quad {
        control: [5222, 823],
        to: [5222, 861],
    },
    MenuValueSegment::Quad {
        control: [5222, 942],
        to: [5315, 983],
    },
    MenuValueSegment::Line([5421, 1031]),
    MenuValueSegment::Quad {
        control: [5518, 1077],
        to: [5563, 1133],
    },
    MenuValueSegment::Quad {
        control: [5607, 1191],
        to: [5607, 1277],
    },
    MenuValueSegment::Quad {
        control: [5607, 1389],
        to: [5529, 1453],
    },
    MenuValueSegment::Quad {
        control: [5449, 1516],
        to: [5310, 1516],
    },
    MenuValueSegment::Quad {
        control: [5177, 1516],
        to: [5062, 1450],
    },
    MenuValueSegment::Line([5113, 1309]),
    MenuValueSegment::Quad {
        control: [5238, 1391],
        to: [5313, 1391],
    },
    MenuValueSegment::Quad {
        control: [5451, 1391],
        to: [5451, 1275],
    },
    MenuValueSegment::Quad {
        control: [5451, 1192],
        to: [5318, 1133],
    },
    MenuValueSegment::Quad {
        control: [5216, 1086],
        to: [5180, 1062],
    },
    MenuValueSegment::Quad {
        control: [5144, 1038],
        to: [5119, 1008],
    },
    MenuValueSegment::Quad {
        control: [5093, 977],
        to: [5080, 942],
    },
    MenuValueSegment::Quad {
        control: [5066, 906],
        to: [5066, 867],
    },
    MenuValueSegment::Quad {
        control: [5066, 764],
        to: [5141, 706],
    },
    MenuValueSegment::Quad {
        control: [5216, 648],
        to: [5338, 648],
    },
    MenuValueSegment::Quad {
        control: [5429, 648],
        to: [5568, 706],
    },
];

const MATCH_BEST_OF_3_CONTOUR_5: [MenuValueSegment; 18] = [
    MenuValueSegment::Line([5819, 489]),
    MenuValueSegment::Line([5968, 431]),
    MenuValueSegment::Line([5968, 664]),
    MenuValueSegment::Line([6198, 664]),
    MenuValueSegment::Line([6198, 781]),
    MenuValueSegment::Line([5968, 781]),
    MenuValueSegment::Line([5968, 1197]),
    MenuValueSegment::Quad {
        control: [5968, 1302],
        to: [6004, 1347],
    },
    MenuValueSegment::Quad {
        control: [6038, 1391],
        to: [6118, 1391],
    },
    MenuValueSegment::Quad {
        control: [6174, 1391],
        to: [6235, 1362],
    },
    MenuValueSegment::Line([6257, 1492]),
    MenuValueSegment::Line([6055, 1516]),
    MenuValueSegment::Quad {
        control: [5955, 1516],
        to: [5888, 1442],
    },
    MenuValueSegment::Quad {
        control: [5819, 1369],
        to: [5819, 1256],
    },
    MenuValueSegment::Line([5819, 781]),
    MenuValueSegment::Line([5722, 781]),
    MenuValueSegment::Line([5722, 664]),
    MenuValueSegment::Line([5819, 664]),
];

const MATCH_BEST_OF_3_CONTOUR_6: [MenuValueSegment; 6] = [
    MenuValueSegment::Quad {
        control: [6983, 1395],
        to: [7205, 1395],
    },
    MenuValueSegment::Quad {
        control: [7309, 1395],
        to: [7369, 1311],
    },
    MenuValueSegment::Quad {
        control: [7427, 1227],
        to: [7427, 1080],
    },
    MenuValueSegment::Quad {
        control: [7427, 769],
        to: [7205, 769],
    },
    MenuValueSegment::Quad {
        control: [7103, 769],
        to: [7044, 852],
    },
    MenuValueSegment::Quad {
        control: [6983, 934],
        to: [6983, 1080],
    },
];

const MATCH_BEST_OF_3_CONTOUR_7: [MenuValueSegment; 8] = [
    MenuValueSegment::Quad {
        control: [7034, 648],
        to: [7205, 648],
    },
    MenuValueSegment::Quad {
        control: [7384, 648],
        to: [7484, 762],
    },
    MenuValueSegment::Quad {
        control: [7583, 877],
        to: [7583, 1080],
    },
    MenuValueSegment::Quad {
        control: [7583, 1283],
        to: [7481, 1400],
    },
    MenuValueSegment::Quad {
        control: [7380, 1516],
        to: [7205, 1516],
    },
    MenuValueSegment::Quad {
        control: [7027, 1516],
        to: [6927, 1398],
    },
    MenuValueSegment::Quad {
        control: [6827, 1280],
        to: [6827, 1080],
    },
    MenuValueSegment::Quad {
        control: [6827, 886],
        to: [6931, 767],
    },
];

const MATCH_BEST_OF_3_CONTOUR_8: [MenuValueSegment; 17] = [
    MenuValueSegment::Quad {
        control: [7808, 503],
        to: [7887, 412],
    },
    MenuValueSegment::Quad {
        control: [7966, 320],
        to: [8096, 320],
    },
    MenuValueSegment::Line([8235, 344]),
    MenuValueSegment::Line([8193, 453]),
    MenuValueSegment::Line([8108, 438]),
    MenuValueSegment::Quad {
        control: [8041, 438],
        to: [7997, 489],
    },
    MenuValueSegment::Quad {
        control: [7952, 541],
        to: [7952, 620],
    },
    MenuValueSegment::Quad {
        control: [7952, 641],
        to: [7957, 664],
    },
    MenuValueSegment::Line([8127, 664]),
    MenuValueSegment::Line([8127, 789]),
    MenuValueSegment::Line([7957, 789]),
    MenuValueSegment::Line([7957, 1500]),
    MenuValueSegment::Line([7808, 1500]),
    MenuValueSegment::Line([7808, 789]),
    MenuValueSegment::Line([7687, 789]),
    MenuValueSegment::Line([7687, 664]),
    MenuValueSegment::Line([7808, 664]),
];

const MATCH_BEST_OF_3_CONTOUR_9: [MenuValueSegment; 23] = [
    MenuValueSegment::Quad {
        control: [9402, 495],
        to: [9402, 620],
    },
    MenuValueSegment::Quad {
        control: [9402, 716],
        to: [9350, 789],
    },
    MenuValueSegment::Quad {
        control: [9297, 862],
        to: [9225, 888],
    },
    MenuValueSegment::Quad {
        control: [9324, 920],
        to: [9383, 997],
    },
    MenuValueSegment::Quad {
        control: [9441, 1073],
        to: [9441, 1183],
    },
    MenuValueSegment::Quad {
        control: [9441, 1342],
        to: [9339, 1430],
    },
    MenuValueSegment::Quad {
        control: [9239, 1516],
        to: [9057, 1516],
    },
    MenuValueSegment::Quad {
        control: [8980, 1516],
        to: [8907, 1488],
    },
    MenuValueSegment::Quad {
        control: [8833, 1459],
        to: [8791, 1419],
    },
    MenuValueSegment::Line([8866, 1298]),
    MenuValueSegment::Quad {
        control: [8942, 1383],
        to: [9060, 1383],
    },
    MenuValueSegment::Quad {
        control: [9277, 1383],
        to: [9277, 1170],
    },
    MenuValueSegment::Quad {
        control: [9277, 1073],
        to: [9213, 1014],
    },
    MenuValueSegment::Quad {
        control: [9150, 953],
        to: [9046, 953],
    },
    MenuValueSegment::Line([9033, 953]),
    MenuValueSegment::Line([9033, 827]),
    MenuValueSegment::Line([9039, 827]),
    MenuValueSegment::Quad {
        control: [9238, 827],
        to: [9238, 652],
    },
    MenuValueSegment::Quad {
        control: [9238, 469],
        to: [9052, 469],
    },
    MenuValueSegment::Quad {
        control: [8950, 469],
        to: [8889, 536],
    },
    MenuValueSegment::Line([8821, 430]),
    MenuValueSegment::Quad {
        control: [8892, 336],
        to: [9063, 336],
    },
    MenuValueSegment::Quad {
        control: [9213, 336],
        to: [9308, 416],
    },
];

const MATCH_BEST_OF_3_CONTOURS: [MenuValueContour; 10] = [
    MenuValueContour {
        start: [3500, 320],
        segments: &MATCH_BEST_OF_3_CONTOUR_0,
    },
    MenuValueContour {
        start: [3500, 839],
        segments: &MATCH_BEST_OF_3_CONTOUR_1,
    },
    MenuValueContour {
        start: [4585, 773],
        segments: &MATCH_BEST_OF_3_CONTOUR_2,
    },
    MenuValueContour {
        start: [4422, 1322],
        segments: &MATCH_BEST_OF_3_CONTOUR_3,
    },
    MenuValueContour {
        start: [5568, 706],
        segments: &MATCH_BEST_OF_3_CONTOUR_4,
    },
    MenuValueContour {
        start: [5819, 664],
        segments: &MATCH_BEST_OF_3_CONTOUR_5,
    },
    MenuValueContour {
        start: [6983, 1080],
        segments: &MATCH_BEST_OF_3_CONTOUR_6,
    },
    MenuValueContour {
        start: [6931, 767],
        segments: &MATCH_BEST_OF_3_CONTOUR_7,
    },
    MenuValueContour {
        start: [7808, 664],
        segments: &MATCH_BEST_OF_3_CONTOUR_8,
    },
    MenuValueContour {
        start: [9308, 416],
        segments: &MATCH_BEST_OF_3_CONTOUR_9,
    },
];

pub const MATCH_BEST_OF_3: MenuValueDefinition = MenuValueDefinition {
    text: "best of 3",
    define_text_id: 44,
    font_ids: &MATCH_BEST_OF_3_FONT_IDS,
    color_rgb: [255, 255, 255],
    bounds_centipx: [3350, 10245, 320, 1515],
    contours: &MATCH_BEST_OF_3_CONTOURS,
};

const MATCH_BEST_OF_5_FONT_IDS: [u16; 1] = [26];

const MATCH_BEST_OF_5_CONTOUR_0: [MenuValueSegment; 13] = [
    MenuValueSegment::Line([3500, 725]),
    MenuValueSegment::Quad {
        control: [3520, 697],
        to: [3579, 672],
    },
    MenuValueSegment::Quad {
        control: [3637, 648],
        to: [3693, 648],
    },
    MenuValueSegment::Quad {
        control: [3865, 648],
        to: [3972, 767],
    },
    MenuValueSegment::Quad {
        control: [4078, 886],
        to: [4078, 1067],
    },
    MenuValueSegment::Quad {
        control: [4078, 1277],
        to: [3972, 1397],
    },
    MenuValueSegment::Quad {
        control: [3864, 1516],
        to: [3681, 1516],
    },
    MenuValueSegment::Quad {
        control: [3622, 1516],
        to: [3565, 1494],
    },
    MenuValueSegment::Quad {
        control: [3508, 1472],
        to: [3478, 1441],
    },
    MenuValueSegment::Line([3425, 1516]),
    MenuValueSegment::Line([3351, 1516]),
    MenuValueSegment::Line([3351, 320]),
    MenuValueSegment::Line([3500, 320]),
];

const MATCH_BEST_OF_5_CONTOUR_1: [MenuValueSegment; 9] = [
    MenuValueSegment::Line([3500, 1322]),
    MenuValueSegment::Quad {
        control: [3500, 1334],
        to: [3554, 1362],
    },
    MenuValueSegment::Quad {
        control: [3611, 1391],
        to: [3639, 1391],
    },
    MenuValueSegment::Quad {
        control: [3790, 1391],
        to: [3856, 1319],
    },
    MenuValueSegment::Quad {
        control: [3922, 1245],
        to: [3922, 1075],
    },
    MenuValueSegment::Quad {
        control: [3922, 933],
        to: [3845, 853],
    },
    MenuValueSegment::Quad {
        control: [3768, 773],
        to: [3639, 773],
    },
    MenuValueSegment::Quad {
        control: [3612, 773],
        to: [3564, 797],
    },
    MenuValueSegment::Line([3500, 839]),
];

const MATCH_BEST_OF_5_CONTOUR_2: [MenuValueSegment; 5] = [
    MenuValueSegment::Quad {
        control: [4487, 773],
        to: [4419, 838],
    },
    MenuValueSegment::Quad {
        control: [4355, 898],
        to: [4346, 989],
    },
    MenuValueSegment::Line([4810, 989]),
    MenuValueSegment::Quad {
        control: [4810, 898],
        to: [4754, 839],
    },
    MenuValueSegment::Quad {
        control: [4691, 773],
        to: [4585, 773],
    },
];

const MATCH_BEST_OF_5_CONTOUR_3: [MenuValueSegment; 14] = [
    MenuValueSegment::Quad {
        control: [4494, 1391],
        to: [4605, 1391],
    },
    MenuValueSegment::Quad {
        control: [4733, 1391],
        to: [4818, 1317],
    },
    MenuValueSegment::Line([4880, 1423]),
    MenuValueSegment::Quad {
        control: [4846, 1458],
        to: [4776, 1483],
    },
    MenuValueSegment::Quad {
        control: [4687, 1516],
        to: [4577, 1516],
    },
    MenuValueSegment::Quad {
        control: [4419, 1516],
        to: [4308, 1409],
    },
    MenuValueSegment::Quad {
        control: [4187, 1291],
        to: [4187, 1092],
    },
    MenuValueSegment::Quad {
        control: [4187, 884],
        to: [4312, 759],
    },
    MenuValueSegment::Quad {
        control: [4424, 648],
        to: [4579, 648],
    },
    MenuValueSegment::Quad {
        control: [4757, 648],
        to: [4858, 748],
    },
    MenuValueSegment::Quad {
        control: [4957, 845],
        to: [4957, 1006],
    },
    MenuValueSegment::Quad {
        control: [4957, 1055],
        to: [4946, 1097],
    },
    MenuValueSegment::Line([4343, 1097]),
    MenuValueSegment::Quad {
        control: [4343, 1244],
        to: [4422, 1322],
    },
];

const MATCH_BEST_OF_5_CONTOUR_4: [MenuValueSegment; 22] = [
    MenuValueSegment::Line([5526, 844]),
    MenuValueSegment::Quad {
        control: [5438, 773],
        to: [5349, 773],
    },
    MenuValueSegment::Quad {
        control: [5296, 773],
        to: [5260, 798],
    },
    MenuValueSegment::Quad {
        control: [5222, 823],
        to: [5222, 861],
    },
    MenuValueSegment::Quad {
        control: [5222, 942],
        to: [5315, 983],
    },
    MenuValueSegment::Line([5421, 1031]),
    MenuValueSegment::Quad {
        control: [5518, 1077],
        to: [5563, 1133],
    },
    MenuValueSegment::Quad {
        control: [5607, 1191],
        to: [5607, 1277],
    },
    MenuValueSegment::Quad {
        control: [5607, 1389],
        to: [5529, 1453],
    },
    MenuValueSegment::Quad {
        control: [5449, 1516],
        to: [5310, 1516],
    },
    MenuValueSegment::Quad {
        control: [5177, 1516],
        to: [5062, 1450],
    },
    MenuValueSegment::Line([5113, 1309]),
    MenuValueSegment::Quad {
        control: [5238, 1391],
        to: [5313, 1391],
    },
    MenuValueSegment::Quad {
        control: [5451, 1391],
        to: [5451, 1275],
    },
    MenuValueSegment::Quad {
        control: [5451, 1192],
        to: [5318, 1133],
    },
    MenuValueSegment::Quad {
        control: [5216, 1086],
        to: [5180, 1062],
    },
    MenuValueSegment::Quad {
        control: [5144, 1038],
        to: [5119, 1008],
    },
    MenuValueSegment::Quad {
        control: [5093, 977],
        to: [5080, 942],
    },
    MenuValueSegment::Quad {
        control: [5066, 906],
        to: [5066, 867],
    },
    MenuValueSegment::Quad {
        control: [5066, 764],
        to: [5141, 706],
    },
    MenuValueSegment::Quad {
        control: [5216, 648],
        to: [5338, 648],
    },
    MenuValueSegment::Quad {
        control: [5429, 648],
        to: [5568, 706],
    },
];

const MATCH_BEST_OF_5_CONTOUR_5: [MenuValueSegment; 18] = [
    MenuValueSegment::Line([5819, 489]),
    MenuValueSegment::Line([5968, 431]),
    MenuValueSegment::Line([5968, 664]),
    MenuValueSegment::Line([6198, 664]),
    MenuValueSegment::Line([6198, 781]),
    MenuValueSegment::Line([5968, 781]),
    MenuValueSegment::Line([5968, 1197]),
    MenuValueSegment::Quad {
        control: [5968, 1302],
        to: [6004, 1347],
    },
    MenuValueSegment::Quad {
        control: [6038, 1391],
        to: [6118, 1391],
    },
    MenuValueSegment::Quad {
        control: [6174, 1391],
        to: [6235, 1362],
    },
    MenuValueSegment::Line([6257, 1492]),
    MenuValueSegment::Line([6055, 1516]),
    MenuValueSegment::Quad {
        control: [5955, 1516],
        to: [5888, 1442],
    },
    MenuValueSegment::Quad {
        control: [5819, 1369],
        to: [5819, 1256],
    },
    MenuValueSegment::Line([5819, 781]),
    MenuValueSegment::Line([5722, 781]),
    MenuValueSegment::Line([5722, 664]),
    MenuValueSegment::Line([5819, 664]),
];

const MATCH_BEST_OF_5_CONTOUR_6: [MenuValueSegment; 6] = [
    MenuValueSegment::Quad {
        control: [6983, 1395],
        to: [7205, 1395],
    },
    MenuValueSegment::Quad {
        control: [7309, 1395],
        to: [7369, 1311],
    },
    MenuValueSegment::Quad {
        control: [7427, 1227],
        to: [7427, 1080],
    },
    MenuValueSegment::Quad {
        control: [7427, 769],
        to: [7205, 769],
    },
    MenuValueSegment::Quad {
        control: [7103, 769],
        to: [7044, 852],
    },
    MenuValueSegment::Quad {
        control: [6983, 934],
        to: [6983, 1080],
    },
];

const MATCH_BEST_OF_5_CONTOUR_7: [MenuValueSegment; 8] = [
    MenuValueSegment::Quad {
        control: [7034, 648],
        to: [7205, 648],
    },
    MenuValueSegment::Quad {
        control: [7384, 648],
        to: [7484, 762],
    },
    MenuValueSegment::Quad {
        control: [7583, 877],
        to: [7583, 1080],
    },
    MenuValueSegment::Quad {
        control: [7583, 1283],
        to: [7481, 1400],
    },
    MenuValueSegment::Quad {
        control: [7380, 1516],
        to: [7205, 1516],
    },
    MenuValueSegment::Quad {
        control: [7027, 1516],
        to: [6927, 1398],
    },
    MenuValueSegment::Quad {
        control: [6827, 1280],
        to: [6827, 1080],
    },
    MenuValueSegment::Quad {
        control: [6827, 886],
        to: [6931, 767],
    },
];

const MATCH_BEST_OF_5_CONTOUR_8: [MenuValueSegment; 17] = [
    MenuValueSegment::Quad {
        control: [7808, 503],
        to: [7887, 412],
    },
    MenuValueSegment::Quad {
        control: [7966, 320],
        to: [8096, 320],
    },
    MenuValueSegment::Line([8235, 344]),
    MenuValueSegment::Line([8193, 453]),
    MenuValueSegment::Line([8108, 438]),
    MenuValueSegment::Quad {
        control: [8041, 438],
        to: [7997, 489],
    },
    MenuValueSegment::Quad {
        control: [7952, 541],
        to: [7952, 620],
    },
    MenuValueSegment::Quad {
        control: [7952, 641],
        to: [7957, 664],
    },
    MenuValueSegment::Line([8127, 664]),
    MenuValueSegment::Line([8127, 789]),
    MenuValueSegment::Line([7957, 789]),
    MenuValueSegment::Line([7957, 1500]),
    MenuValueSegment::Line([7808, 1500]),
    MenuValueSegment::Line([7808, 789]),
    MenuValueSegment::Line([7687, 789]),
    MenuValueSegment::Line([7687, 664]),
    MenuValueSegment::Line([7808, 664]),
];

const MATCH_BEST_OF_5_CONTOUR_9: [MenuValueSegment; 16] = [
    MenuValueSegment::Line([8982, 484]),
    MenuValueSegment::Line([8982, 762]),
    MenuValueSegment::Quad {
        control: [9039, 719],
        to: [9128, 719],
    },
    MenuValueSegment::Quad {
        control: [9288, 719],
        to: [9372, 814],
    },
    MenuValueSegment::Quad {
        control: [9457, 909],
        to: [9457, 1086],
    },
    MenuValueSegment::Quad {
        control: [9457, 1516],
        to: [9072, 1516],
    },
    MenuValueSegment::Quad {
        control: [8913, 1516],
        to: [8807, 1427],
    },
    MenuValueSegment::Line([8868, 1297]),
    MenuValueSegment::Quad {
        control: [8974, 1383],
        to: [9072, 1383],
    },
    MenuValueSegment::Quad {
        control: [9292, 1383],
        to: [9292, 1108],
    },
    MenuValueSegment::Quad {
        control: [9292, 852],
        to: [9075, 852],
    },
    MenuValueSegment::Quad {
        control: [8971, 852],
        to: [8886, 942],
    },
    MenuValueSegment::Line([8833, 905]),
    MenuValueSegment::Line([8833, 352]),
    MenuValueSegment::Line([9403, 352]),
    MenuValueSegment::Line([9403, 484]),
];

const MATCH_BEST_OF_5_CONTOURS: [MenuValueContour; 10] = [
    MenuValueContour {
        start: [3500, 320],
        segments: &MATCH_BEST_OF_5_CONTOUR_0,
    },
    MenuValueContour {
        start: [3500, 839],
        segments: &MATCH_BEST_OF_5_CONTOUR_1,
    },
    MenuValueContour {
        start: [4585, 773],
        segments: &MATCH_BEST_OF_5_CONTOUR_2,
    },
    MenuValueContour {
        start: [4422, 1322],
        segments: &MATCH_BEST_OF_5_CONTOUR_3,
    },
    MenuValueContour {
        start: [5568, 706],
        segments: &MATCH_BEST_OF_5_CONTOUR_4,
    },
    MenuValueContour {
        start: [5819, 664],
        segments: &MATCH_BEST_OF_5_CONTOUR_5,
    },
    MenuValueContour {
        start: [6983, 1080],
        segments: &MATCH_BEST_OF_5_CONTOUR_6,
    },
    MenuValueContour {
        start: [6931, 767],
        segments: &MATCH_BEST_OF_5_CONTOUR_7,
    },
    MenuValueContour {
        start: [7808, 664],
        segments: &MATCH_BEST_OF_5_CONTOUR_8,
    },
    MenuValueContour {
        start: [9403, 484],
        segments: &MATCH_BEST_OF_5_CONTOUR_9,
    },
];

pub const MATCH_BEST_OF_5: MenuValueDefinition = MenuValueDefinition {
    text: "best of 5",
    define_text_id: 45,
    font_ids: &MATCH_BEST_OF_5_FONT_IDS,
    color_rgb: [255, 255, 255],
    bounds_centipx: [3350, 10245, 320, 1515],
    contours: &MATCH_BEST_OF_5_CONTOURS,
};

const MATCH_BEST_OF_7_FONT_IDS: [u16; 1] = [26];

const MATCH_BEST_OF_7_CONTOUR_0: [MenuValueSegment; 13] = [
    MenuValueSegment::Line([3500, 725]),
    MenuValueSegment::Quad {
        control: [3520, 697],
        to: [3579, 672],
    },
    MenuValueSegment::Quad {
        control: [3637, 648],
        to: [3693, 648],
    },
    MenuValueSegment::Quad {
        control: [3865, 648],
        to: [3972, 767],
    },
    MenuValueSegment::Quad {
        control: [4078, 886],
        to: [4078, 1067],
    },
    MenuValueSegment::Quad {
        control: [4078, 1277],
        to: [3972, 1397],
    },
    MenuValueSegment::Quad {
        control: [3864, 1516],
        to: [3681, 1516],
    },
    MenuValueSegment::Quad {
        control: [3622, 1516],
        to: [3565, 1494],
    },
    MenuValueSegment::Quad {
        control: [3508, 1472],
        to: [3478, 1441],
    },
    MenuValueSegment::Line([3425, 1516]),
    MenuValueSegment::Line([3351, 1516]),
    MenuValueSegment::Line([3351, 320]),
    MenuValueSegment::Line([3500, 320]),
];

const MATCH_BEST_OF_7_CONTOUR_1: [MenuValueSegment; 9] = [
    MenuValueSegment::Line([3500, 1322]),
    MenuValueSegment::Quad {
        control: [3500, 1334],
        to: [3554, 1362],
    },
    MenuValueSegment::Quad {
        control: [3611, 1391],
        to: [3639, 1391],
    },
    MenuValueSegment::Quad {
        control: [3790, 1391],
        to: [3856, 1319],
    },
    MenuValueSegment::Quad {
        control: [3922, 1245],
        to: [3922, 1075],
    },
    MenuValueSegment::Quad {
        control: [3922, 933],
        to: [3845, 853],
    },
    MenuValueSegment::Quad {
        control: [3768, 773],
        to: [3639, 773],
    },
    MenuValueSegment::Quad {
        control: [3612, 773],
        to: [3564, 797],
    },
    MenuValueSegment::Line([3500, 839]),
];

const MATCH_BEST_OF_7_CONTOUR_2: [MenuValueSegment; 5] = [
    MenuValueSegment::Quad {
        control: [4487, 773],
        to: [4419, 838],
    },
    MenuValueSegment::Quad {
        control: [4355, 898],
        to: [4346, 989],
    },
    MenuValueSegment::Line([4810, 989]),
    MenuValueSegment::Quad {
        control: [4810, 898],
        to: [4754, 839],
    },
    MenuValueSegment::Quad {
        control: [4691, 773],
        to: [4585, 773],
    },
];

const MATCH_BEST_OF_7_CONTOUR_3: [MenuValueSegment; 14] = [
    MenuValueSegment::Quad {
        control: [4494, 1391],
        to: [4605, 1391],
    },
    MenuValueSegment::Quad {
        control: [4733, 1391],
        to: [4818, 1317],
    },
    MenuValueSegment::Line([4880, 1423]),
    MenuValueSegment::Quad {
        control: [4846, 1458],
        to: [4776, 1483],
    },
    MenuValueSegment::Quad {
        control: [4687, 1516],
        to: [4577, 1516],
    },
    MenuValueSegment::Quad {
        control: [4419, 1516],
        to: [4308, 1409],
    },
    MenuValueSegment::Quad {
        control: [4187, 1291],
        to: [4187, 1092],
    },
    MenuValueSegment::Quad {
        control: [4187, 884],
        to: [4312, 759],
    },
    MenuValueSegment::Quad {
        control: [4424, 648],
        to: [4579, 648],
    },
    MenuValueSegment::Quad {
        control: [4757, 648],
        to: [4858, 748],
    },
    MenuValueSegment::Quad {
        control: [4957, 845],
        to: [4957, 1006],
    },
    MenuValueSegment::Quad {
        control: [4957, 1055],
        to: [4946, 1097],
    },
    MenuValueSegment::Line([4343, 1097]),
    MenuValueSegment::Quad {
        control: [4343, 1244],
        to: [4422, 1322],
    },
];

const MATCH_BEST_OF_7_CONTOUR_4: [MenuValueSegment; 22] = [
    MenuValueSegment::Line([5526, 844]),
    MenuValueSegment::Quad {
        control: [5438, 773],
        to: [5349, 773],
    },
    MenuValueSegment::Quad {
        control: [5296, 773],
        to: [5260, 798],
    },
    MenuValueSegment::Quad {
        control: [5222, 823],
        to: [5222, 861],
    },
    MenuValueSegment::Quad {
        control: [5222, 942],
        to: [5315, 983],
    },
    MenuValueSegment::Line([5421, 1031]),
    MenuValueSegment::Quad {
        control: [5518, 1077],
        to: [5563, 1133],
    },
    MenuValueSegment::Quad {
        control: [5607, 1191],
        to: [5607, 1277],
    },
    MenuValueSegment::Quad {
        control: [5607, 1389],
        to: [5529, 1453],
    },
    MenuValueSegment::Quad {
        control: [5449, 1516],
        to: [5310, 1516],
    },
    MenuValueSegment::Quad {
        control: [5177, 1516],
        to: [5062, 1450],
    },
    MenuValueSegment::Line([5113, 1309]),
    MenuValueSegment::Quad {
        control: [5238, 1391],
        to: [5313, 1391],
    },
    MenuValueSegment::Quad {
        control: [5451, 1391],
        to: [5451, 1275],
    },
    MenuValueSegment::Quad {
        control: [5451, 1192],
        to: [5318, 1133],
    },
    MenuValueSegment::Quad {
        control: [5216, 1086],
        to: [5180, 1062],
    },
    MenuValueSegment::Quad {
        control: [5144, 1038],
        to: [5119, 1008],
    },
    MenuValueSegment::Quad {
        control: [5093, 977],
        to: [5080, 942],
    },
    MenuValueSegment::Quad {
        control: [5066, 906],
        to: [5066, 867],
    },
    MenuValueSegment::Quad {
        control: [5066, 764],
        to: [5141, 706],
    },
    MenuValueSegment::Quad {
        control: [5216, 648],
        to: [5338, 648],
    },
    MenuValueSegment::Quad {
        control: [5429, 648],
        to: [5568, 706],
    },
];

const MATCH_BEST_OF_7_CONTOUR_5: [MenuValueSegment; 18] = [
    MenuValueSegment::Line([5819, 489]),
    MenuValueSegment::Line([5968, 431]),
    MenuValueSegment::Line([5968, 664]),
    MenuValueSegment::Line([6198, 664]),
    MenuValueSegment::Line([6198, 781]),
    MenuValueSegment::Line([5968, 781]),
    MenuValueSegment::Line([5968, 1197]),
    MenuValueSegment::Quad {
        control: [5968, 1302],
        to: [6004, 1347],
    },
    MenuValueSegment::Quad {
        control: [6038, 1391],
        to: [6118, 1391],
    },
    MenuValueSegment::Quad {
        control: [6174, 1391],
        to: [6235, 1362],
    },
    MenuValueSegment::Line([6257, 1492]),
    MenuValueSegment::Line([6055, 1516]),
    MenuValueSegment::Quad {
        control: [5955, 1516],
        to: [5888, 1442],
    },
    MenuValueSegment::Quad {
        control: [5819, 1369],
        to: [5819, 1256],
    },
    MenuValueSegment::Line([5819, 781]),
    MenuValueSegment::Line([5722, 781]),
    MenuValueSegment::Line([5722, 664]),
    MenuValueSegment::Line([5819, 664]),
];

const MATCH_BEST_OF_7_CONTOUR_6: [MenuValueSegment; 6] = [
    MenuValueSegment::Quad {
        control: [6983, 1395],
        to: [7205, 1395],
    },
    MenuValueSegment::Quad {
        control: [7309, 1395],
        to: [7369, 1311],
    },
    MenuValueSegment::Quad {
        control: [7427, 1227],
        to: [7427, 1080],
    },
    MenuValueSegment::Quad {
        control: [7427, 769],
        to: [7205, 769],
    },
    MenuValueSegment::Quad {
        control: [7103, 769],
        to: [7044, 852],
    },
    MenuValueSegment::Quad {
        control: [6983, 934],
        to: [6983, 1080],
    },
];

const MATCH_BEST_OF_7_CONTOUR_7: [MenuValueSegment; 8] = [
    MenuValueSegment::Quad {
        control: [7034, 648],
        to: [7205, 648],
    },
    MenuValueSegment::Quad {
        control: [7384, 648],
        to: [7484, 762],
    },
    MenuValueSegment::Quad {
        control: [7583, 877],
        to: [7583, 1080],
    },
    MenuValueSegment::Quad {
        control: [7583, 1283],
        to: [7481, 1400],
    },
    MenuValueSegment::Quad {
        control: [7380, 1516],
        to: [7205, 1516],
    },
    MenuValueSegment::Quad {
        control: [7027, 1516],
        to: [6927, 1398],
    },
    MenuValueSegment::Quad {
        control: [6827, 1280],
        to: [6827, 1080],
    },
    MenuValueSegment::Quad {
        control: [6827, 886],
        to: [6931, 767],
    },
];

const MATCH_BEST_OF_7_CONTOUR_8: [MenuValueSegment; 17] = [
    MenuValueSegment::Quad {
        control: [7808, 503],
        to: [7887, 412],
    },
    MenuValueSegment::Quad {
        control: [7966, 320],
        to: [8096, 320],
    },
    MenuValueSegment::Line([8235, 344]),
    MenuValueSegment::Line([8193, 453]),
    MenuValueSegment::Line([8108, 438]),
    MenuValueSegment::Quad {
        control: [8041, 438],
        to: [7997, 489],
    },
    MenuValueSegment::Quad {
        control: [7952, 541],
        to: [7952, 620],
    },
    MenuValueSegment::Quad {
        control: [7952, 641],
        to: [7957, 664],
    },
    MenuValueSegment::Line([8127, 664]),
    MenuValueSegment::Line([8127, 789]),
    MenuValueSegment::Line([7957, 789]),
    MenuValueSegment::Line([7957, 1500]),
    MenuValueSegment::Line([7808, 1500]),
    MenuValueSegment::Line([7808, 789]),
    MenuValueSegment::Line([7687, 789]),
    MenuValueSegment::Line([7687, 664]),
    MenuValueSegment::Line([7808, 664]),
];

const MATCH_BEST_OF_7_CONTOUR_9: [MenuValueSegment; 12] = [
    MenuValueSegment::Line([9413, 611]),
    MenuValueSegment::Line([9307, 831]),
    MenuValueSegment::Line([9197, 1077]),
    MenuValueSegment::Line([9105, 1309]),
    MenuValueSegment::Quad {
        control: [9064, 1416],
        to: [9039, 1500],
    },
    MenuValueSegment::Line([8866, 1500]),
    MenuValueSegment::Quad {
        control: [8924, 1317],
        to: [9061, 1016],
    },
    MenuValueSegment::Quad {
        control: [9197, 712],
        to: [9311, 500],
    },
    MenuValueSegment::Line([8771, 500]),
    MenuValueSegment::Line([8771, 352]),
    MenuValueSegment::Line([9513, 352]),
    MenuValueSegment::Line([9513, 412]),
];

const MATCH_BEST_OF_7_CONTOURS: [MenuValueContour; 10] = [
    MenuValueContour {
        start: [3500, 320],
        segments: &MATCH_BEST_OF_7_CONTOUR_0,
    },
    MenuValueContour {
        start: [3500, 839],
        segments: &MATCH_BEST_OF_7_CONTOUR_1,
    },
    MenuValueContour {
        start: [4585, 773],
        segments: &MATCH_BEST_OF_7_CONTOUR_2,
    },
    MenuValueContour {
        start: [4422, 1322],
        segments: &MATCH_BEST_OF_7_CONTOUR_3,
    },
    MenuValueContour {
        start: [5568, 706],
        segments: &MATCH_BEST_OF_7_CONTOUR_4,
    },
    MenuValueContour {
        start: [5819, 664],
        segments: &MATCH_BEST_OF_7_CONTOUR_5,
    },
    MenuValueContour {
        start: [6983, 1080],
        segments: &MATCH_BEST_OF_7_CONTOUR_6,
    },
    MenuValueContour {
        start: [6931, 767],
        segments: &MATCH_BEST_OF_7_CONTOUR_7,
    },
    MenuValueContour {
        start: [7808, 664],
        segments: &MATCH_BEST_OF_7_CONTOUR_8,
    },
    MenuValueContour {
        start: [9513, 412],
        segments: &MATCH_BEST_OF_7_CONTOUR_9,
    },
];

pub const MATCH_BEST_OF_7: MenuValueDefinition = MenuValueDefinition {
    text: "best of 7",
    define_text_id: 46,
    font_ids: &MATCH_BEST_OF_7_FONT_IDS,
    color_rgb: [255, 255, 255],
    bounds_centipx: [3350, 10245, 320, 1515],
    contours: &MATCH_BEST_OF_7_CONTOURS,
};

const GRAVITY_LOW_FONT_IDS: [u16; 1] = [26];

const GRAVITY_LOW_CONTOUR_0: [MenuValueSegment; 7] = [
    MenuValueSegment::Line([5541, 1516]),
    MenuValueSegment::Quad {
        control: [5252, 1516],
        to: [5252, 1264],
    },
    MenuValueSegment::Line([5252, 320]),
    MenuValueSegment::Line([5401, 320]),
    MenuValueSegment::Line([5401, 1239]),
    MenuValueSegment::Quad {
        control: [5401, 1306],
        to: [5440, 1345],
    },
    MenuValueSegment::Quad {
        control: [5479, 1383],
        to: [5541, 1383],
    },
];

const GRAVITY_LOW_CONTOUR_1: [MenuValueSegment; 6] = [
    MenuValueSegment::Quad {
        control: [5813, 1395],
        to: [6035, 1395],
    },
    MenuValueSegment::Quad {
        control: [6139, 1395],
        to: [6199, 1311],
    },
    MenuValueSegment::Quad {
        control: [6257, 1227],
        to: [6257, 1080],
    },
    MenuValueSegment::Quad {
        control: [6257, 769],
        to: [6035, 769],
    },
    MenuValueSegment::Quad {
        control: [5933, 769],
        to: [5874, 852],
    },
    MenuValueSegment::Quad {
        control: [5813, 934],
        to: [5813, 1080],
    },
];

const GRAVITY_LOW_CONTOUR_2: [MenuValueSegment; 8] = [
    MenuValueSegment::Quad {
        control: [5864, 648],
        to: [6035, 648],
    },
    MenuValueSegment::Quad {
        control: [6214, 648],
        to: [6314, 762],
    },
    MenuValueSegment::Quad {
        control: [6413, 877],
        to: [6413, 1080],
    },
    MenuValueSegment::Quad {
        control: [6413, 1283],
        to: [6311, 1400],
    },
    MenuValueSegment::Quad {
        control: [6210, 1516],
        to: [6035, 1516],
    },
    MenuValueSegment::Quad {
        control: [5857, 1516],
        to: [5757, 1398],
    },
    MenuValueSegment::Quad {
        control: [5657, 1280],
        to: [5657, 1080],
    },
    MenuValueSegment::Quad {
        control: [5657, 886],
        to: [5761, 767],
    },
];

const GRAVITY_LOW_CONTOUR_3: [MenuValueSegment; 13] = [
    MenuValueSegment::Line([7345, 1516]),
    MenuValueSegment::Line([7306, 1516]),
    MenuValueSegment::Line([7060, 947]),
    MenuValueSegment::Line([6815, 1516]),
    MenuValueSegment::Line([6776, 1516]),
    MenuValueSegment::Line([6478, 661]),
    MenuValueSegment::Line([6635, 661]),
    MenuValueSegment::Line([6815, 1211]),
    MenuValueSegment::Line([7037, 661]),
    MenuValueSegment::Line([7076, 661]),
    MenuValueSegment::Line([7306, 1211]),
    MenuValueSegment::Line([7498, 661]),
    MenuValueSegment::Line([7645, 661]),
];

const GRAVITY_LOW_CONTOURS: [MenuValueContour; 4] = [
    MenuValueContour {
        start: [5541, 1383],
        segments: &GRAVITY_LOW_CONTOUR_0,
    },
    MenuValueContour {
        start: [5813, 1080],
        segments: &GRAVITY_LOW_CONTOUR_1,
    },
    MenuValueContour {
        start: [5761, 767],
        segments: &GRAVITY_LOW_CONTOUR_2,
    },
    MenuValueContour {
        start: [7645, 661],
        segments: &GRAVITY_LOW_CONTOUR_3,
    },
];

pub const GRAVITY_LOW: MenuValueDefinition = MenuValueDefinition {
    text: "low",
    define_text_id: 56,
    font_ids: &GRAVITY_LOW_FONT_IDS,
    color_rgb: [255, 255, 255],
    bounds_centipx: [5250, 8355, 320, 1515],
    contours: &GRAVITY_LOW_CONTOURS,
};

const GRAVITY_MEDIUM_FONT_IDS: [u16; 1] = [26];

const GRAVITY_MEDIUM_CONTOUR_0: [MenuValueSegment; 24] = [
    MenuValueSegment::Quad {
        control: [4743, 802],
        to: [4743, 941],
    },
    MenuValueSegment::Line([4743, 1500]),
    MenuValueSegment::Line([4595, 1500]),
    MenuValueSegment::Line([4595, 970]),
    MenuValueSegment::Quad {
        control: [4595, 773],
        to: [4423, 773],
    },
    MenuValueSegment::Quad {
        control: [4370, 773],
        to: [4323, 806],
    },
    MenuValueSegment::Quad {
        control: [4276, 839],
        to: [4259, 881],
    },
    MenuValueSegment::Line([4259, 1500]),
    MenuValueSegment::Line([4111, 1500]),
    MenuValueSegment::Line([4111, 906]),
    MenuValueSegment::Quad {
        control: [4111, 844],
        to: [4064, 809],
    },
    MenuValueSegment::Quad {
        control: [4017, 773],
        to: [3940, 773],
    },
    MenuValueSegment::Quad {
        control: [3897, 773],
        to: [3847, 808],
    },
    MenuValueSegment::Quad {
        control: [3795, 842],
        to: [3775, 883],
    },
    MenuValueSegment::Line([3775, 1500]),
    MenuValueSegment::Line([3626, 1500]),
    MenuValueSegment::Line([3626, 664]),
    MenuValueSegment::Line([3723, 664]),
    MenuValueSegment::Line([3772, 761]),
    MenuValueSegment::Quad {
        control: [3858, 648],
        to: [3987, 648],
    },
    MenuValueSegment::Quad {
        control: [4167, 648],
        to: [4239, 759],
    },
    MenuValueSegment::Quad {
        control: [4264, 712],
        to: [4331, 680],
    },
    MenuValueSegment::Quad {
        control: [4400, 648],
        to: [4472, 648],
    },
    MenuValueSegment::Quad {
        control: [4601, 648],
        to: [4672, 725],
    },
];

const GRAVITY_MEDIUM_CONTOUR_1: [MenuValueSegment; 5] = [
    MenuValueSegment::Quad {
        control: [5202, 773],
        to: [5134, 838],
    },
    MenuValueSegment::Quad {
        control: [5070, 898],
        to: [5061, 989],
    },
    MenuValueSegment::Line([5525, 989]),
    MenuValueSegment::Quad {
        control: [5525, 898],
        to: [5469, 839],
    },
    MenuValueSegment::Quad {
        control: [5406, 773],
        to: [5300, 773],
    },
];

const GRAVITY_MEDIUM_CONTOUR_2: [MenuValueSegment; 14] = [
    MenuValueSegment::Quad {
        control: [5209, 1391],
        to: [5320, 1391],
    },
    MenuValueSegment::Quad {
        control: [5448, 1391],
        to: [5533, 1317],
    },
    MenuValueSegment::Line([5595, 1423]),
    MenuValueSegment::Quad {
        control: [5561, 1458],
        to: [5491, 1483],
    },
    MenuValueSegment::Quad {
        control: [5402, 1516],
        to: [5292, 1516],
    },
    MenuValueSegment::Quad {
        control: [5134, 1516],
        to: [5023, 1409],
    },
    MenuValueSegment::Quad {
        control: [4902, 1291],
        to: [4902, 1092],
    },
    MenuValueSegment::Quad {
        control: [4902, 884],
        to: [5027, 759],
    },
    MenuValueSegment::Quad {
        control: [5139, 648],
        to: [5294, 648],
    },
    MenuValueSegment::Quad {
        control: [5472, 648],
        to: [5573, 748],
    },
    MenuValueSegment::Quad {
        control: [5672, 845],
        to: [5672, 1006],
    },
    MenuValueSegment::Quad {
        control: [5672, 1055],
        to: [5661, 1097],
    },
    MenuValueSegment::Line([5058, 1097]),
    MenuValueSegment::Quad {
        control: [5058, 1244],
        to: [5138, 1322],
    },
];

const GRAVITY_MEDIUM_CONTOUR_3: [MenuValueSegment; 11] = [
    MenuValueSegment::Line([6509, 320]),
    MenuValueSegment::Line([6509, 1500]),
    MenuValueSegment::Line([6361, 1500]),
    MenuValueSegment::Line([6361, 1438]),
    MenuValueSegment::Quad {
        control: [6284, 1516],
        to: [6136, 1516],
    },
    MenuValueSegment::Quad {
        control: [5980, 1516],
        to: [5881, 1403],
    },
    MenuValueSegment::Quad {
        control: [5784, 1291],
        to: [5784, 1103],
    },
    MenuValueSegment::Quad {
        control: [5784, 914],
        to: [5897, 781],
    },
    MenuValueSegment::Quad {
        control: [6009, 648],
        to: [6164, 648],
    },
    MenuValueSegment::Quad {
        control: [6294, 648],
        to: [6361, 709],
    },
    MenuValueSegment::Line([6361, 320]),
];

const GRAVITY_MEDIUM_CONTOUR_4: [MenuValueSegment; 7] = [
    MenuValueSegment::Quad {
        control: [6348, 1347],
        to: [6361, 1322],
    },
    MenuValueSegment::Line([6361, 858]),
    MenuValueSegment::Quad {
        control: [6305, 773],
        to: [6208, 773],
    },
    MenuValueSegment::Quad {
        control: [6088, 773],
        to: [6014, 862],
    },
    MenuValueSegment::Quad {
        control: [5941, 952],
        to: [5941, 1089],
    },
    MenuValueSegment::Quad {
        control: [5941, 1391],
        to: [6216, 1391],
    },
    MenuValueSegment::Quad {
        control: [6250, 1391],
        to: [6300, 1369],
    },
];

const GRAVITY_MEDIUM_CONTOUR_5: [MenuValueSegment; 8] = [
    MenuValueSegment::Quad {
        control: [6912, 347],
        to: [6940, 375],
    },
    MenuValueSegment::Quad {
        control: [6967, 402],
        to: [6967, 439],
    },
    MenuValueSegment::Quad {
        control: [6967, 477],
        to: [6940, 505],
    },
    MenuValueSegment::Quad {
        control: [6912, 531],
        to: [6874, 531],
    },
    MenuValueSegment::Quad {
        control: [6837, 531],
        to: [6810, 505],
    },
    MenuValueSegment::Quad {
        control: [6782, 477],
        to: [6782, 439],
    },
    MenuValueSegment::Quad {
        control: [6782, 400],
        to: [6809, 373],
    },
    MenuValueSegment::Quad {
        control: [6835, 347],
        to: [6874, 347],
    },
];

const GRAVITY_MEDIUM_CONTOUR_6: [MenuValueSegment; 6] = [
    MenuValueSegment::Line([6942, 1500]),
    MenuValueSegment::Line([6793, 1500]),
    MenuValueSegment::Line([6793, 789]),
    MenuValueSegment::Line([6678, 789]),
    MenuValueSegment::Line([6678, 664]),
    MenuValueSegment::Line([6942, 664]),
];

const GRAVITY_MEDIUM_CONTOUR_7: [MenuValueSegment; 15] = [
    MenuValueSegment::Line([7317, 1197]),
    MenuValueSegment::Quad {
        control: [7317, 1391],
        to: [7484, 1391],
    },
    MenuValueSegment::Quad {
        control: [7558, 1391],
        to: [7618, 1348],
    },
    MenuValueSegment::Quad {
        control: [7679, 1306],
        to: [7700, 1252],
    },
    MenuValueSegment::Line([7700, 664]),
    MenuValueSegment::Line([7848, 664]),
    MenuValueSegment::Line([7848, 1500]),
    MenuValueSegment::Line([7700, 1500]),
    MenuValueSegment::Line([7700, 1384]),
    MenuValueSegment::Quad {
        control: [7675, 1434],
        to: [7600, 1475],
    },
    MenuValueSegment::Quad {
        control: [7523, 1516],
        to: [7451, 1516],
    },
    MenuValueSegment::Quad {
        control: [7314, 1516],
        to: [7242, 1438],
    },
    MenuValueSegment::Quad {
        control: [7168, 1358],
        to: [7168, 1212],
    },
    MenuValueSegment::Line([7168, 664]),
    MenuValueSegment::Line([7317, 664]),
];

const GRAVITY_MEDIUM_CONTOUR_8: [MenuValueSegment; 24] = [
    MenuValueSegment::Quad {
        control: [9168, 802],
        to: [9168, 941],
    },
    MenuValueSegment::Line([9168, 1500]),
    MenuValueSegment::Line([9020, 1500]),
    MenuValueSegment::Line([9020, 970]),
    MenuValueSegment::Quad {
        control: [9020, 773],
        to: [8848, 773],
    },
    MenuValueSegment::Quad {
        control: [8795, 773],
        to: [8748, 806],
    },
    MenuValueSegment::Quad {
        control: [8701, 839],
        to: [8684, 881],
    },
    MenuValueSegment::Line([8684, 1500]),
    MenuValueSegment::Line([8536, 1500]),
    MenuValueSegment::Line([8536, 906]),
    MenuValueSegment::Quad {
        control: [8536, 844],
        to: [8489, 809],
    },
    MenuValueSegment::Quad {
        control: [8442, 773],
        to: [8365, 773],
    },
    MenuValueSegment::Quad {
        control: [8322, 773],
        to: [8272, 808],
    },
    MenuValueSegment::Quad {
        control: [8220, 842],
        to: [8200, 883],
    },
    MenuValueSegment::Line([8200, 1500]),
    MenuValueSegment::Line([8051, 1500]),
    MenuValueSegment::Line([8051, 664]),
    MenuValueSegment::Line([8148, 664]),
    MenuValueSegment::Line([8197, 761]),
    MenuValueSegment::Quad {
        control: [8282, 648],
        to: [8412, 648],
    },
    MenuValueSegment::Quad {
        control: [8592, 648],
        to: [8664, 759],
    },
    MenuValueSegment::Quad {
        control: [8689, 712],
        to: [8756, 680],
    },
    MenuValueSegment::Quad {
        control: [8825, 648],
        to: [8897, 648],
    },
    MenuValueSegment::Quad {
        control: [9026, 648],
        to: [9097, 725],
    },
];

const GRAVITY_MEDIUM_CONTOURS: [MenuValueContour; 9] = [
    MenuValueContour {
        start: [4672, 725],
        segments: &GRAVITY_MEDIUM_CONTOUR_0,
    },
    MenuValueContour {
        start: [5300, 773],
        segments: &GRAVITY_MEDIUM_CONTOUR_1,
    },
    MenuValueContour {
        start: [5138, 1322],
        segments: &GRAVITY_MEDIUM_CONTOUR_2,
    },
    MenuValueContour {
        start: [6361, 320],
        segments: &GRAVITY_MEDIUM_CONTOUR_3,
    },
    MenuValueContour {
        start: [6300, 1369],
        segments: &GRAVITY_MEDIUM_CONTOUR_4,
    },
    MenuValueContour {
        start: [6874, 347],
        segments: &GRAVITY_MEDIUM_CONTOUR_5,
    },
    MenuValueContour {
        start: [6942, 664],
        segments: &GRAVITY_MEDIUM_CONTOUR_6,
    },
    MenuValueContour {
        start: [7317, 664],
        segments: &GRAVITY_MEDIUM_CONTOUR_7,
    },
    MenuValueContour {
        start: [9097, 725],
        segments: &GRAVITY_MEDIUM_CONTOUR_8,
    },
];

pub const GRAVITY_MEDIUM: MenuValueDefinition = MenuValueDefinition {
    text: "medium",
    define_text_id: 57,
    font_ids: &GRAVITY_MEDIUM_FONT_IDS,
    color_rgb: [255, 255, 255],
    bounds_centipx: [3625, 9975, 320, 1515],
    contours: &GRAVITY_MEDIUM_CONTOURS,
};

const GRAVITY_HIGH_FONT_IDS: [u16; 1] = [26];

const GRAVITY_HIGH_CONTOUR_0: [MenuValueSegment; 16] = [
    MenuValueSegment::Line([5145, 756]),
    MenuValueSegment::Quad {
        control: [5174, 709],
        to: [5240, 680],
    },
    MenuValueSegment::Quad {
        control: [5306, 648],
        to: [5376, 648],
    },
    MenuValueSegment::Quad {
        control: [5509, 648],
        to: [5585, 736],
    },
    MenuValueSegment::Quad {
        control: [5660, 823],
        to: [5660, 975],
    },
    MenuValueSegment::Line([5660, 1500]),
    MenuValueSegment::Line([5512, 1500]),
    MenuValueSegment::Line([5512, 975]),
    MenuValueSegment::Quad {
        control: [5512, 881],
        to: [5465, 827],
    },
    MenuValueSegment::Quad {
        control: [5420, 773],
        to: [5335, 773],
    },
    MenuValueSegment::Quad {
        control: [5282, 773],
        to: [5228, 805],
    },
    MenuValueSegment::Quad {
        control: [5173, 836],
        to: [5145, 878],
    },
    MenuValueSegment::Line([5145, 1500]),
    MenuValueSegment::Line([4996, 1500]),
    MenuValueSegment::Line([4996, 320]),
    MenuValueSegment::Line([5145, 320]),
];

const GRAVITY_HIGH_CONTOUR_1: [MenuValueSegment; 8] = [
    MenuValueSegment::Quad {
        control: [6062, 347],
        to: [6090, 375],
    },
    MenuValueSegment::Quad {
        control: [6117, 402],
        to: [6117, 439],
    },
    MenuValueSegment::Quad {
        control: [6117, 477],
        to: [6090, 505],
    },
    MenuValueSegment::Quad {
        control: [6062, 531],
        to: [6024, 531],
    },
    MenuValueSegment::Quad {
        control: [5987, 531],
        to: [5960, 505],
    },
    MenuValueSegment::Quad {
        control: [5932, 477],
        to: [5932, 439],
    },
    MenuValueSegment::Quad {
        control: [5932, 400],
        to: [5959, 373],
    },
    MenuValueSegment::Quad {
        control: [5985, 347],
        to: [6024, 347],
    },
];

const GRAVITY_HIGH_CONTOUR_2: [MenuValueSegment; 6] = [
    MenuValueSegment::Line([6092, 1500]),
    MenuValueSegment::Line([5943, 1500]),
    MenuValueSegment::Line([5943, 789]),
    MenuValueSegment::Line([5828, 789]),
    MenuValueSegment::Line([5828, 664]),
    MenuValueSegment::Line([6092, 664]),
];

const GRAVITY_HIGH_CONTOUR_3: [MenuValueSegment; 32] = [
    MenuValueSegment::Line([6875, 769]),
    MenuValueSegment::Quad {
        control: [6932, 842],
        to: [6932, 962],
    },
    MenuValueSegment::Quad {
        control: [6932, 1089],
        to: [6853, 1175],
    },
    MenuValueSegment::Quad {
        control: [6775, 1261],
        to: [6647, 1273],
    },
    MenuValueSegment::Line([6523, 1286]),
    MenuValueSegment::Line([6465, 1303]),
    MenuValueSegment::Quad {
        control: [6428, 1317],
        to: [6428, 1341],
    },
    MenuValueSegment::Quad {
        control: [6428, 1372],
        to: [6504, 1372],
    },
    MenuValueSegment::Line([6609, 1361]),
    MenuValueSegment::Line([6715, 1348]),
    MenuValueSegment::Quad {
        control: [6839, 1348],
        to: [6908, 1408],
    },
    MenuValueSegment::Quad {
        control: [6976, 1466],
        to: [6976, 1570],
    },
    MenuValueSegment::Quad {
        control: [6976, 1686],
        to: [6873, 1758],
    },
    MenuValueSegment::Quad {
        control: [6770, 1828],
        to: [6611, 1828],
    },
    MenuValueSegment::Quad {
        control: [6529, 1828],
        to: [6440, 1800],
    },
    MenuValueSegment::Quad {
        control: [6350, 1770],
        to: [6295, 1730],
    },
    MenuValueSegment::Line([6376, 1611]),
    MenuValueSegment::Quad {
        control: [6506, 1697],
        to: [6615, 1697],
    },
    MenuValueSegment::Quad {
        control: [6715, 1697],
        to: [6775, 1662],
    },
    MenuValueSegment::Quad {
        control: [6832, 1628],
        to: [6832, 1577],
    },
    MenuValueSegment::Quad {
        control: [6832, 1475],
        to: [6686, 1475],
    },
    MenuValueSegment::Line([6595, 1488]),
    MenuValueSegment::Line([6492, 1500]),
    MenuValueSegment::Quad {
        control: [6314, 1500],
        to: [6314, 1366],
    },
    MenuValueSegment::Quad {
        control: [6314, 1323],
        to: [6356, 1291],
    },
    MenuValueSegment::Quad {
        control: [6398, 1256],
        to: [6459, 1242],
    },
    MenuValueSegment::Quad {
        control: [6282, 1159],
        to: [6282, 955],
    },
    MenuValueSegment::Quad {
        control: [6282, 823],
        to: [6375, 736],
    },
    MenuValueSegment::Quad {
        control: [6465, 648],
        to: [6600, 648],
    },
    MenuValueSegment::Quad {
        control: [6723, 648],
        to: [6793, 698],
    },
    MenuValueSegment::Line([6867, 609]),
    MenuValueSegment::Line([6964, 702]),
];

const GRAVITY_HIGH_CONTOUR_4: [MenuValueSegment; 8] = [
    MenuValueSegment::Quad {
        control: [6532, 767],
        to: [6484, 822],
    },
    MenuValueSegment::Quad {
        control: [6436, 877],
        to: [6436, 955],
    },
    MenuValueSegment::Quad {
        control: [6436, 1042],
        to: [6482, 1100],
    },
    MenuValueSegment::Quad {
        control: [6529, 1158],
        to: [6611, 1158],
    },
    MenuValueSegment::Quad {
        control: [6689, 1158],
        to: [6734, 1102],
    },
    MenuValueSegment::Quad {
        control: [6778, 1045],
        to: [6778, 955],
    },
    MenuValueSegment::Quad {
        control: [6778, 877],
        to: [6731, 822],
    },
    MenuValueSegment::Quad {
        control: [6682, 767],
        to: [6611, 767],
    },
];

const GRAVITY_HIGH_CONTOUR_5: [MenuValueSegment; 16] = [
    MenuValueSegment::Line([7280, 756]),
    MenuValueSegment::Quad {
        control: [7309, 709],
        to: [7375, 680],
    },
    MenuValueSegment::Quad {
        control: [7441, 648],
        to: [7511, 648],
    },
    MenuValueSegment::Quad {
        control: [7644, 648],
        to: [7720, 736],
    },
    MenuValueSegment::Quad {
        control: [7795, 823],
        to: [7795, 975],
    },
    MenuValueSegment::Line([7795, 1500]),
    MenuValueSegment::Line([7647, 1500]),
    MenuValueSegment::Line([7647, 975]),
    MenuValueSegment::Quad {
        control: [7647, 881],
        to: [7600, 827],
    },
    MenuValueSegment::Quad {
        control: [7555, 773],
        to: [7470, 773],
    },
    MenuValueSegment::Quad {
        control: [7417, 773],
        to: [7362, 805],
    },
    MenuValueSegment::Quad {
        control: [7308, 836],
        to: [7280, 878],
    },
    MenuValueSegment::Line([7280, 1500]),
    MenuValueSegment::Line([7131, 1500]),
    MenuValueSegment::Line([7131, 320]),
    MenuValueSegment::Line([7280, 320]),
];

const GRAVITY_HIGH_CONTOURS: [MenuValueContour; 6] = [
    MenuValueContour {
        start: [5145, 320],
        segments: &GRAVITY_HIGH_CONTOUR_0,
    },
    MenuValueContour {
        start: [6024, 347],
        segments: &GRAVITY_HIGH_CONTOUR_1,
    },
    MenuValueContour {
        start: [6092, 664],
        segments: &GRAVITY_HIGH_CONTOUR_2,
    },
    MenuValueContour {
        start: [6964, 702],
        segments: &GRAVITY_HIGH_CONTOUR_3,
    },
    MenuValueContour {
        start: [6611, 767],
        segments: &GRAVITY_HIGH_CONTOUR_4,
    },
    MenuValueContour {
        start: [7280, 320],
        segments: &GRAVITY_HIGH_CONTOUR_5,
    },
];

pub const GRAVITY_HIGH: MenuValueDefinition = MenuValueDefinition {
    text: "high",
    define_text_id: 58,
    font_ids: &GRAVITY_HIGH_FONT_IDS,
    color_rgb: [255, 255, 255],
    bounds_centipx: [4995, 8600, 320, 1830],
    contours: &GRAVITY_HIGH_CONTOURS,
};

const GRAVITY_VERY_HIGH_FONT_IDS: [u16; 1] = [26];

const GRAVITY_VERY_HIGH_CONTOUR_0: [MenuValueSegment; 7] = [
    MenuValueSegment::Line([3523, 1516]),
    MenuValueSegment::Line([3484, 1516]),
    MenuValueSegment::Line([3124, 661]),
    MenuValueSegment::Line([3287, 661]),
    MenuValueSegment::Line([3509, 1247]),
    MenuValueSegment::Line([3734, 661]),
    MenuValueSegment::Line([3890, 661]),
];

const GRAVITY_VERY_HIGH_CONTOUR_1: [MenuValueSegment; 5] = [
    MenuValueSegment::Quad {
        control: [4252, 773],
        to: [4184, 838],
    },
    MenuValueSegment::Quad {
        control: [4120, 898],
        to: [4111, 989],
    },
    MenuValueSegment::Line([4575, 989]),
    MenuValueSegment::Quad {
        control: [4575, 898],
        to: [4519, 839],
    },
    MenuValueSegment::Quad {
        control: [4456, 773],
        to: [4350, 773],
    },
];

const GRAVITY_VERY_HIGH_CONTOUR_2: [MenuValueSegment; 14] = [
    MenuValueSegment::Quad {
        control: [4259, 1391],
        to: [4370, 1391],
    },
    MenuValueSegment::Quad {
        control: [4498, 1391],
        to: [4583, 1317],
    },
    MenuValueSegment::Line([4645, 1423]),
    MenuValueSegment::Quad {
        control: [4611, 1458],
        to: [4541, 1483],
    },
    MenuValueSegment::Quad {
        control: [4452, 1516],
        to: [4342, 1516],
    },
    MenuValueSegment::Quad {
        control: [4184, 1516],
        to: [4073, 1409],
    },
    MenuValueSegment::Quad {
        control: [3952, 1291],
        to: [3952, 1092],
    },
    MenuValueSegment::Quad {
        control: [3952, 884],
        to: [4077, 759],
    },
    MenuValueSegment::Quad {
        control: [4189, 648],
        to: [4344, 648],
    },
    MenuValueSegment::Quad {
        control: [4522, 648],
        to: [4623, 748],
    },
    MenuValueSegment::Quad {
        control: [4722, 845],
        to: [4722, 1006],
    },
    MenuValueSegment::Quad {
        control: [4722, 1055],
        to: [4711, 1097],
    },
    MenuValueSegment::Line([4108, 1097]),
    MenuValueSegment::Quad {
        control: [4108, 1244],
        to: [4188, 1322],
    },
];

const GRAVITY_VERY_HIGH_CONTOUR_3: [MenuValueSegment; 11] = [
    MenuValueSegment::Quad {
        control: [5288, 773],
        to: [5238, 773],
    },
    MenuValueSegment::Quad {
        control: [5159, 773],
        to: [5100, 845],
    },
    MenuValueSegment::Quad {
        control: [5041, 919],
        to: [5041, 1020],
    },
    MenuValueSegment::Line([5041, 1500]),
    MenuValueSegment::Line([4892, 1500]),
    MenuValueSegment::Line([4892, 664]),
    MenuValueSegment::Line([5041, 664]),
    MenuValueSegment::Line([5041, 797]),
    MenuValueSegment::Quad {
        control: [5122, 648],
        to: [5283, 648],
    },
    MenuValueSegment::Line([5397, 662]),
    MenuValueSegment::Line([5336, 806]),
];

const GRAVITY_VERY_HIGH_CONTOUR_4: [MenuValueSegment; 13] = [
    MenuValueSegment::Line([5820, 1642]),
    MenuValueSegment::Quad {
        control: [5792, 1722],
        to: [5700, 1775],
    },
    MenuValueSegment::Quad {
        control: [5606, 1828],
        to: [5492, 1828],
    },
    MenuValueSegment::Line([5492, 1695]),
    MenuValueSegment::Quad {
        control: [5586, 1695],
        to: [5651, 1653],
    },
    MenuValueSegment::Quad {
        control: [5718, 1609],
        to: [5718, 1547],
    },
    MenuValueSegment::Quad {
        control: [5718, 1478],
        to: [5693, 1411],
    },
    MenuValueSegment::Line([5631, 1245]),
    MenuValueSegment::Line([5404, 664]),
    MenuValueSegment::Line([5556, 664]),
    MenuValueSegment::Line([5803, 1311]),
    MenuValueSegment::Line([6023, 664]),
    MenuValueSegment::Line([6175, 664]),
];

const GRAVITY_VERY_HIGH_CONTOUR_5: [MenuValueSegment; 16] = [
    MenuValueSegment::Line([6920, 756]),
    MenuValueSegment::Quad {
        control: [6949, 709],
        to: [7015, 680],
    },
    MenuValueSegment::Quad {
        control: [7081, 648],
        to: [7151, 648],
    },
    MenuValueSegment::Quad {
        control: [7284, 648],
        to: [7360, 736],
    },
    MenuValueSegment::Quad {
        control: [7435, 823],
        to: [7435, 975],
    },
    MenuValueSegment::Line([7435, 1500]),
    MenuValueSegment::Line([7287, 1500]),
    MenuValueSegment::Line([7287, 975]),
    MenuValueSegment::Quad {
        control: [7287, 881],
        to: [7240, 827],
    },
    MenuValueSegment::Quad {
        control: [7195, 773],
        to: [7110, 773],
    },
    MenuValueSegment::Quad {
        control: [7057, 773],
        to: [7003, 805],
    },
    MenuValueSegment::Quad {
        control: [6948, 836],
        to: [6920, 878],
    },
    MenuValueSegment::Line([6920, 1500]),
    MenuValueSegment::Line([6771, 1500]),
    MenuValueSegment::Line([6771, 320]),
    MenuValueSegment::Line([6920, 320]),
];

const GRAVITY_VERY_HIGH_CONTOUR_6: [MenuValueSegment; 8] = [
    MenuValueSegment::Quad {
        control: [7837, 347],
        to: [7865, 375],
    },
    MenuValueSegment::Quad {
        control: [7892, 402],
        to: [7892, 439],
    },
    MenuValueSegment::Quad {
        control: [7892, 477],
        to: [7865, 505],
    },
    MenuValueSegment::Quad {
        control: [7837, 531],
        to: [7799, 531],
    },
    MenuValueSegment::Quad {
        control: [7762, 531],
        to: [7735, 505],
    },
    MenuValueSegment::Quad {
        control: [7707, 477],
        to: [7707, 439],
    },
    MenuValueSegment::Quad {
        control: [7707, 400],
        to: [7734, 373],
    },
    MenuValueSegment::Quad {
        control: [7760, 347],
        to: [7799, 347],
    },
];

const GRAVITY_VERY_HIGH_CONTOUR_7: [MenuValueSegment; 6] = [
    MenuValueSegment::Line([7867, 1500]),
    MenuValueSegment::Line([7718, 1500]),
    MenuValueSegment::Line([7718, 789]),
    MenuValueSegment::Line([7603, 789]),
    MenuValueSegment::Line([7603, 664]),
    MenuValueSegment::Line([7867, 664]),
];

const GRAVITY_VERY_HIGH_CONTOUR_8: [MenuValueSegment; 32] = [
    MenuValueSegment::Line([8650, 769]),
    MenuValueSegment::Quad {
        control: [8707, 842],
        to: [8707, 962],
    },
    MenuValueSegment::Quad {
        control: [8707, 1089],
        to: [8628, 1175],
    },
    MenuValueSegment::Quad {
        control: [8550, 1261],
        to: [8422, 1273],
    },
    MenuValueSegment::Line([8298, 1286]),
    MenuValueSegment::Line([8240, 1303]),
    MenuValueSegment::Quad {
        control: [8203, 1317],
        to: [8203, 1341],
    },
    MenuValueSegment::Quad {
        control: [8203, 1372],
        to: [8279, 1372],
    },
    MenuValueSegment::Line([8384, 1361]),
    MenuValueSegment::Line([8490, 1348]),
    MenuValueSegment::Quad {
        control: [8614, 1348],
        to: [8682, 1408],
    },
    MenuValueSegment::Quad {
        control: [8751, 1466],
        to: [8751, 1570],
    },
    MenuValueSegment::Quad {
        control: [8751, 1686],
        to: [8648, 1758],
    },
    MenuValueSegment::Quad {
        control: [8545, 1828],
        to: [8386, 1828],
    },
    MenuValueSegment::Quad {
        control: [8304, 1828],
        to: [8215, 1800],
    },
    MenuValueSegment::Quad {
        control: [8125, 1770],
        to: [8070, 1730],
    },
    MenuValueSegment::Line([8151, 1611]),
    MenuValueSegment::Quad {
        control: [8281, 1697],
        to: [8390, 1697],
    },
    MenuValueSegment::Quad {
        control: [8490, 1697],
        to: [8550, 1662],
    },
    MenuValueSegment::Quad {
        control: [8607, 1628],
        to: [8607, 1577],
    },
    MenuValueSegment::Quad {
        control: [8607, 1475],
        to: [8461, 1475],
    },
    MenuValueSegment::Line([8370, 1488]),
    MenuValueSegment::Line([8267, 1500]),
    MenuValueSegment::Quad {
        control: [8089, 1500],
        to: [8089, 1366],
    },
    MenuValueSegment::Quad {
        control: [8089, 1323],
        to: [8131, 1291],
    },
    MenuValueSegment::Quad {
        control: [8173, 1256],
        to: [8234, 1242],
    },
    MenuValueSegment::Quad {
        control: [8057, 1159],
        to: [8057, 955],
    },
    MenuValueSegment::Quad {
        control: [8057, 823],
        to: [8150, 736],
    },
    MenuValueSegment::Quad {
        control: [8240, 648],
        to: [8375, 648],
    },
    MenuValueSegment::Quad {
        control: [8498, 648],
        to: [8568, 698],
    },
    MenuValueSegment::Line([8642, 609]),
    MenuValueSegment::Line([8739, 702]),
];

const GRAVITY_VERY_HIGH_CONTOUR_9: [MenuValueSegment; 8] = [
    MenuValueSegment::Quad {
        control: [8307, 767],
        to: [8259, 822],
    },
    MenuValueSegment::Quad {
        control: [8211, 877],
        to: [8211, 955],
    },
    MenuValueSegment::Quad {
        control: [8211, 1042],
        to: [8257, 1100],
    },
    MenuValueSegment::Quad {
        control: [8304, 1158],
        to: [8386, 1158],
    },
    MenuValueSegment::Quad {
        control: [8464, 1158],
        to: [8509, 1102],
    },
    MenuValueSegment::Quad {
        control: [8553, 1045],
        to: [8553, 955],
    },
    MenuValueSegment::Quad {
        control: [8553, 877],
        to: [8506, 822],
    },
    MenuValueSegment::Quad {
        control: [8457, 767],
        to: [8386, 767],
    },
];

const GRAVITY_VERY_HIGH_CONTOUR_10: [MenuValueSegment; 16] = [
    MenuValueSegment::Line([9055, 756]),
    MenuValueSegment::Quad {
        control: [9084, 709],
        to: [9150, 680],
    },
    MenuValueSegment::Quad {
        control: [9216, 648],
        to: [9286, 648],
    },
    MenuValueSegment::Quad {
        control: [9419, 648],
        to: [9495, 736],
    },
    MenuValueSegment::Quad {
        control: [9570, 823],
        to: [9570, 975],
    },
    MenuValueSegment::Line([9570, 1500]),
    MenuValueSegment::Line([9422, 1500]),
    MenuValueSegment::Line([9422, 975]),
    MenuValueSegment::Quad {
        control: [9422, 881],
        to: [9375, 827],
    },
    MenuValueSegment::Quad {
        control: [9330, 773],
        to: [9245, 773],
    },
    MenuValueSegment::Quad {
        control: [9192, 773],
        to: [9138, 805],
    },
    MenuValueSegment::Quad {
        control: [9083, 836],
        to: [9055, 878],
    },
    MenuValueSegment::Line([9055, 1500]),
    MenuValueSegment::Line([8906, 1500]),
    MenuValueSegment::Line([8906, 320]),
    MenuValueSegment::Line([9055, 320]),
];

const GRAVITY_VERY_HIGH_CONTOURS: [MenuValueContour; 11] = [
    MenuValueContour {
        start: [3890, 661],
        segments: &GRAVITY_VERY_HIGH_CONTOUR_0,
    },
    MenuValueContour {
        start: [4350, 773],
        segments: &GRAVITY_VERY_HIGH_CONTOUR_1,
    },
    MenuValueContour {
        start: [4188, 1322],
        segments: &GRAVITY_VERY_HIGH_CONTOUR_2,
    },
    MenuValueContour {
        start: [5336, 806],
        segments: &GRAVITY_VERY_HIGH_CONTOUR_3,
    },
    MenuValueContour {
        start: [6175, 664],
        segments: &GRAVITY_VERY_HIGH_CONTOUR_4,
    },
    MenuValueContour {
        start: [6920, 320],
        segments: &GRAVITY_VERY_HIGH_CONTOUR_5,
    },
    MenuValueContour {
        start: [7799, 347],
        segments: &GRAVITY_VERY_HIGH_CONTOUR_6,
    },
    MenuValueContour {
        start: [7867, 664],
        segments: &GRAVITY_VERY_HIGH_CONTOUR_7,
    },
    MenuValueContour {
        start: [8739, 702],
        segments: &GRAVITY_VERY_HIGH_CONTOUR_8,
    },
    MenuValueContour {
        start: [8386, 767],
        segments: &GRAVITY_VERY_HIGH_CONTOUR_9,
    },
    MenuValueContour {
        start: [9055, 320],
        segments: &GRAVITY_VERY_HIGH_CONTOUR_10,
    },
];

pub const GRAVITY_VERY_HIGH: MenuValueDefinition = MenuValueDefinition {
    text: "very high",
    define_text_id: 59,
    font_ids: &GRAVITY_VERY_HIGH_FONT_IDS,
    color_rgb: [255, 255, 255],
    bounds_centipx: [3125, 10375, 320, 1830],
    contours: &GRAVITY_VERY_HIGH_CONTOURS,
};

const GRAVITY_BLACK_HOLE_FONT_IDS: [u16; 2] = [54, 26];

const GRAVITY_BLACK_HOLE_CONTOUR_0: [MenuValueSegment; 11] = [
    MenuValueSegment::Line([2931, 695]),
    MenuValueSegment::Quad {
        control: [3009, 648],
        to: [3101, 648],
    },
    MenuValueSegment::Quad {
        control: [3284, 648],
        to: [3397, 764],
    },
    MenuValueSegment::Quad {
        control: [3509, 880],
        to: [3509, 1061],
    },
    MenuValueSegment::Quad {
        control: [3509, 1272],
        to: [3397, 1394],
    },
    MenuValueSegment::Quad {
        control: [3283, 1516],
        to: [3089, 1516],
    },
    MenuValueSegment::Quad {
        control: [2973, 1516],
        to: [2889, 1455],
    },
    MenuValueSegment::Line([2845, 1516]),
    MenuValueSegment::Line([2736, 1516]),
    MenuValueSegment::Line([2736, 352]),
    MenuValueSegment::Line([2931, 305]),
];

const GRAVITY_BLACK_HOLE_CONTOUR_1: [MenuValueSegment; 6] = [
    MenuValueSegment::Line([2931, 1300]),
    MenuValueSegment::Quad {
        control: [2973, 1350],
        to: [3047, 1350],
    },
    MenuValueSegment::Quad {
        control: [3189, 1350],
        to: [3248, 1283],
    },
    MenuValueSegment::Quad {
        control: [3306, 1216],
        to: [3306, 1069],
    },
    MenuValueSegment::Quad {
        control: [3306, 814],
        to: [3062, 814],
    },
    MenuValueSegment::Quad {
        control: [2976, 814],
        to: [2931, 862],
    },
];

const GRAVITY_BLACK_HOLE_CONTOUR_2: [MenuValueSegment; 6] = [
    MenuValueSegment::Line([3909, 305]),
    MenuValueSegment::Line([3909, 1245]),
    MenuValueSegment::Quad {
        control: [3909, 1400],
        to: [4002, 1430],
    },
    MenuValueSegment::Quad {
        control: [3956, 1516],
        to: [3847, 1516],
    },
    MenuValueSegment::Quad {
        control: [3714, 1516],
        to: [3714, 1331],
    },
    MenuValueSegment::Line([3714, 352]),
];

const GRAVITY_BLACK_HOLE_CONTOUR_3: [MenuValueSegment; 19] = [
    MenuValueSegment::Quad {
        control: [4614, 648],
        to: [4703, 736],
    },
    MenuValueSegment::Quad {
        control: [4792, 822],
        to: [4792, 1066],
    },
    MenuValueSegment::Line([4792, 1244]),
    MenuValueSegment::Quad {
        control: [4792, 1411],
        to: [4859, 1455],
    },
    MenuValueSegment::Quad {
        control: [4836, 1497],
        to: [4806, 1506],
    },
    MenuValueSegment::Line([4737, 1516]),
    MenuValueSegment::Quad {
        control: [4695, 1516],
        to: [4662, 1484],
    },
    MenuValueSegment::Quad {
        control: [4628, 1453],
        to: [4617, 1417],
    },
    MenuValueSegment::Quad {
        control: [4590, 1461],
        to: [4525, 1489],
    },
    MenuValueSegment::Quad {
        control: [4458, 1516],
        to: [4386, 1516],
    },
    MenuValueSegment::Quad {
        control: [4251, 1516],
        to: [4173, 1448],
    },
    MenuValueSegment::Quad {
        control: [4097, 1381],
        to: [4097, 1256],
    },
    MenuValueSegment::Quad {
        control: [4097, 1111],
        to: [4206, 1030],
    },
    MenuValueSegment::Quad {
        control: [4314, 947],
        to: [4515, 947],
    },
    MenuValueSegment::Line([4597, 959]),
    MenuValueSegment::Quad {
        control: [4597, 811],
        to: [4409, 811],
    },
    MenuValueSegment::Quad {
        control: [4300, 811],
        to: [4225, 848],
    },
    MenuValueSegment::Line([4182, 697]),
    MenuValueSegment::Quad {
        control: [4284, 648],
        to: [4423, 648],
    },
];

const GRAVITY_BLACK_HOLE_CONTOUR_4: [MenuValueSegment; 5] = [
    MenuValueSegment::Quad {
        control: [4292, 1097],
        to: [4292, 1248],
    },
    MenuValueSegment::Quad {
        control: [4292, 1361],
        to: [4422, 1361],
    },
    MenuValueSegment::Quad {
        control: [4598, 1361],
        to: [4598, 1184],
    },
    MenuValueSegment::Line([4598, 1106]),
    MenuValueSegment::Line([4523, 1097]),
];

const GRAVITY_BLACK_HOLE_CONTOUR_5: [MenuValueSegment; 14] = [
    MenuValueSegment::Quad {
        control: [5150, 959],
        to: [5150, 1089],
    },
    MenuValueSegment::Quad {
        control: [5150, 1353],
        to: [5400, 1353],
    },
    MenuValueSegment::Quad {
        control: [5509, 1353],
        to: [5592, 1281],
    },
    MenuValueSegment::Line([5664, 1434]),
    MenuValueSegment::Quad {
        control: [5578, 1488],
        to: [5515, 1502],
    },
    MenuValueSegment::Line([5364, 1516]),
    MenuValueSegment::Quad {
        control: [5170, 1516],
        to: [5059, 1403],
    },
    MenuValueSegment::Quad {
        control: [4947, 1291],
        to: [4947, 1089],
    },
    MenuValueSegment::Quad {
        control: [4947, 892],
        to: [5068, 770],
    },
    MenuValueSegment::Quad {
        control: [5192, 648],
        to: [5403, 648],
    },
    MenuValueSegment::Quad {
        control: [5550, 648],
        to: [5658, 730],
    },
    MenuValueSegment::Line([5573, 877]),
    MenuValueSegment::Quad {
        control: [5504, 811],
        to: [5389, 811],
    },
    MenuValueSegment::Quad {
        control: [5278, 811],
        to: [5214, 886],
    },
];

const GRAVITY_BLACK_HOLE_CONTOUR_6: [MenuValueSegment; 12] = [
    MenuValueSegment::Line([6588, 1500]),
    MenuValueSegment::Line([6356, 1500]),
    MenuValueSegment::Line([6110, 1122]),
    MenuValueSegment::Line([6017, 1220]),
    MenuValueSegment::Line([6017, 1500]),
    MenuValueSegment::Line([5821, 1500]),
    MenuValueSegment::Line([5821, 352]),
    MenuValueSegment::Line([6017, 305]),
    MenuValueSegment::Line([6017, 984]),
    MenuValueSegment::Line([6295, 664]),
    MenuValueSegment::Line([6531, 664]),
    MenuValueSegment::Line([6248, 981]),
];

const GRAVITY_BLACK_HOLE_CONTOUR_7: [MenuValueSegment; 16] = [
    MenuValueSegment::Line([3380, 756]),
    MenuValueSegment::Quad {
        control: [3409, 709],
        to: [3475, 680],
    },
    MenuValueSegment::Quad {
        control: [3541, 648],
        to: [3611, 648],
    },
    MenuValueSegment::Quad {
        control: [3744, 648],
        to: [3820, 736],
    },
    MenuValueSegment::Quad {
        control: [3895, 823],
        to: [3895, 975],
    },
    MenuValueSegment::Line([3895, 1500]),
    MenuValueSegment::Line([3747, 1500]),
    MenuValueSegment::Line([3747, 975]),
    MenuValueSegment::Quad {
        control: [3747, 881],
        to: [3700, 827],
    },
    MenuValueSegment::Quad {
        control: [3655, 773],
        to: [3570, 773],
    },
    MenuValueSegment::Quad {
        control: [3517, 773],
        to: [3462, 805],
    },
    MenuValueSegment::Quad {
        control: [3408, 836],
        to: [3380, 878],
    },
    MenuValueSegment::Line([3380, 1500]),
    MenuValueSegment::Line([3231, 1500]),
    MenuValueSegment::Line([3231, 320]),
    MenuValueSegment::Line([3380, 320]),
];

const GRAVITY_BLACK_HOLE_CONTOUR_8: [MenuValueSegment; 6] = [
    MenuValueSegment::Quad {
        control: [4208, 1395],
        to: [4430, 1395],
    },
    MenuValueSegment::Quad {
        control: [4534, 1395],
        to: [4594, 1311],
    },
    MenuValueSegment::Quad {
        control: [4652, 1227],
        to: [4652, 1080],
    },
    MenuValueSegment::Quad {
        control: [4652, 769],
        to: [4430, 769],
    },
    MenuValueSegment::Quad {
        control: [4328, 769],
        to: [4269, 852],
    },
    MenuValueSegment::Quad {
        control: [4208, 934],
        to: [4208, 1080],
    },
];

const GRAVITY_BLACK_HOLE_CONTOUR_9: [MenuValueSegment; 8] = [
    MenuValueSegment::Quad {
        control: [4259, 648],
        to: [4430, 648],
    },
    MenuValueSegment::Quad {
        control: [4609, 648],
        to: [4709, 762],
    },
    MenuValueSegment::Quad {
        control: [4808, 877],
        to: [4808, 1080],
    },
    MenuValueSegment::Quad {
        control: [4808, 1283],
        to: [4706, 1400],
    },
    MenuValueSegment::Quad {
        control: [4605, 1516],
        to: [4430, 1516],
    },
    MenuValueSegment::Quad {
        control: [4252, 1516],
        to: [4152, 1398],
    },
    MenuValueSegment::Quad {
        control: [4052, 1280],
        to: [4052, 1080],
    },
    MenuValueSegment::Quad {
        control: [4052, 886],
        to: [4156, 767],
    },
];

const GRAVITY_BLACK_HOLE_CONTOUR_10: [MenuValueSegment; 7] = [
    MenuValueSegment::Line([5266, 1516]),
    MenuValueSegment::Quad {
        control: [4977, 1516],
        to: [4977, 1264],
    },
    MenuValueSegment::Line([4977, 320]),
    MenuValueSegment::Line([5126, 320]),
    MenuValueSegment::Line([5126, 1239]),
    MenuValueSegment::Quad {
        control: [5126, 1306],
        to: [5165, 1345],
    },
    MenuValueSegment::Quad {
        control: [5204, 1383],
        to: [5266, 1383],
    },
];

const GRAVITY_BLACK_HOLE_CONTOUR_11: [MenuValueSegment; 5] = [
    MenuValueSegment::Quad {
        control: [5682, 773],
        to: [5614, 838],
    },
    MenuValueSegment::Quad {
        control: [5550, 898],
        to: [5541, 989],
    },
    MenuValueSegment::Line([6005, 989]),
    MenuValueSegment::Quad {
        control: [6005, 898],
        to: [5949, 839],
    },
    MenuValueSegment::Quad {
        control: [5886, 773],
        to: [5780, 773],
    },
];

const GRAVITY_BLACK_HOLE_CONTOUR_12: [MenuValueSegment; 14] = [
    MenuValueSegment::Quad {
        control: [5689, 1391],
        to: [5800, 1391],
    },
    MenuValueSegment::Quad {
        control: [5928, 1391],
        to: [6013, 1317],
    },
    MenuValueSegment::Line([6075, 1423]),
    MenuValueSegment::Quad {
        control: [6041, 1458],
        to: [5971, 1483],
    },
    MenuValueSegment::Quad {
        control: [5882, 1516],
        to: [5772, 1516],
    },
    MenuValueSegment::Quad {
        control: [5614, 1516],
        to: [5503, 1409],
    },
    MenuValueSegment::Quad {
        control: [5382, 1291],
        to: [5382, 1092],
    },
    MenuValueSegment::Quad {
        control: [5382, 884],
        to: [5507, 759],
    },
    MenuValueSegment::Quad {
        control: [5619, 648],
        to: [5774, 648],
    },
    MenuValueSegment::Quad {
        control: [5952, 648],
        to: [6053, 748],
    },
    MenuValueSegment::Quad {
        control: [6152, 845],
        to: [6152, 1006],
    },
    MenuValueSegment::Quad {
        control: [6152, 1055],
        to: [6141, 1097],
    },
    MenuValueSegment::Line([5538, 1097]),
    MenuValueSegment::Quad {
        control: [5538, 1244],
        to: [5618, 1322],
    },
];

const GRAVITY_BLACK_HOLE_CONTOURS: [MenuValueContour; 13] = [
    MenuValueContour {
        start: [2931, 305],
        segments: &GRAVITY_BLACK_HOLE_CONTOUR_0,
    },
    MenuValueContour {
        start: [2931, 862],
        segments: &GRAVITY_BLACK_HOLE_CONTOUR_1,
    },
    MenuValueContour {
        start: [3714, 352],
        segments: &GRAVITY_BLACK_HOLE_CONTOUR_2,
    },
    MenuValueContour {
        start: [4423, 648],
        segments: &GRAVITY_BLACK_HOLE_CONTOUR_3,
    },
    MenuValueContour {
        start: [4523, 1097],
        segments: &GRAVITY_BLACK_HOLE_CONTOUR_4,
    },
    MenuValueContour {
        start: [5214, 886],
        segments: &GRAVITY_BLACK_HOLE_CONTOUR_5,
    },
    MenuValueContour {
        start: [6248, 981],
        segments: &GRAVITY_BLACK_HOLE_CONTOUR_6,
    },
    MenuValueContour {
        start: [3380, 320],
        segments: &GRAVITY_BLACK_HOLE_CONTOUR_7,
    },
    MenuValueContour {
        start: [4208, 1080],
        segments: &GRAVITY_BLACK_HOLE_CONTOUR_8,
    },
    MenuValueContour {
        start: [4156, 767],
        segments: &GRAVITY_BLACK_HOLE_CONTOUR_9,
    },
    MenuValueContour {
        start: [5266, 1383],
        segments: &GRAVITY_BLACK_HOLE_CONTOUR_10,
    },
    MenuValueContour {
        start: [5780, 773],
        segments: &GRAVITY_BLACK_HOLE_CONTOUR_11,
    },
    MenuValueContour {
        start: [5618, 1322],
        segments: &GRAVITY_BLACK_HOLE_CONTOUR_12,
    },
];

pub const GRAVITY_BLACK_HOLE: MenuValueDefinition = MenuValueDefinition {
    text: "black hole",
    define_text_id: 62,
    font_ids: &GRAVITY_BLACK_HOLE_FONT_IDS,
    color_rgb: [255, 255, 255],
    bounds_centipx: [2735, 10850, 305, 1515],
    contours: &GRAVITY_BLACK_HOLE_CONTOURS,
};

const SPEED_DISABLED_FONT_IDS: [u16; 1] = [26];

const SPEED_DISABLED_CONTOUR_0: [MenuValueSegment; 11] = [
    MenuValueSegment::Line([4199, 320]),
    MenuValueSegment::Line([4199, 1500]),
    MenuValueSegment::Line([4051, 1500]),
    MenuValueSegment::Line([4051, 1438]),
    MenuValueSegment::Quad {
        control: [3974, 1516],
        to: [3826, 1516],
    },
    MenuValueSegment::Quad {
        control: [3670, 1516],
        to: [3571, 1403],
    },
    MenuValueSegment::Quad {
        control: [3474, 1291],
        to: [3474, 1103],
    },
    MenuValueSegment::Quad {
        control: [3474, 914],
        to: [3587, 781],
    },
    MenuValueSegment::Quad {
        control: [3699, 648],
        to: [3854, 648],
    },
    MenuValueSegment::Quad {
        control: [3984, 648],
        to: [4051, 709],
    },
    MenuValueSegment::Line([4051, 320]),
];

const SPEED_DISABLED_CONTOUR_1: [MenuValueSegment; 7] = [
    MenuValueSegment::Quad {
        control: [4038, 1347],
        to: [4051, 1322],
    },
    MenuValueSegment::Line([4051, 858]),
    MenuValueSegment::Quad {
        control: [3995, 773],
        to: [3898, 773],
    },
    MenuValueSegment::Quad {
        control: [3778, 773],
        to: [3704, 862],
    },
    MenuValueSegment::Quad {
        control: [3631, 952],
        to: [3631, 1089],
    },
    MenuValueSegment::Quad {
        control: [3631, 1391],
        to: [3906, 1391],
    },
    MenuValueSegment::Quad {
        control: [3940, 1391],
        to: [3990, 1369],
    },
];

const SPEED_DISABLED_CONTOUR_2: [MenuValueSegment; 8] = [
    MenuValueSegment::Quad {
        control: [4602, 347],
        to: [4630, 375],
    },
    MenuValueSegment::Quad {
        control: [4657, 402],
        to: [4657, 439],
    },
    MenuValueSegment::Quad {
        control: [4657, 477],
        to: [4630, 505],
    },
    MenuValueSegment::Quad {
        control: [4602, 531],
        to: [4564, 531],
    },
    MenuValueSegment::Quad {
        control: [4527, 531],
        to: [4500, 505],
    },
    MenuValueSegment::Quad {
        control: [4472, 477],
        to: [4472, 439],
    },
    MenuValueSegment::Quad {
        control: [4472, 400],
        to: [4499, 373],
    },
    MenuValueSegment::Quad {
        control: [4525, 347],
        to: [4564, 347],
    },
];

const SPEED_DISABLED_CONTOUR_3: [MenuValueSegment; 6] = [
    MenuValueSegment::Line([4632, 1500]),
    MenuValueSegment::Line([4483, 1500]),
    MenuValueSegment::Line([4483, 789]),
    MenuValueSegment::Line([4368, 789]),
    MenuValueSegment::Line([4368, 664]),
    MenuValueSegment::Line([4632, 664]),
];

const SPEED_DISABLED_CONTOUR_4: [MenuValueSegment; 22] = [
    MenuValueSegment::Line([5276, 844]),
    MenuValueSegment::Quad {
        control: [5188, 773],
        to: [5099, 773],
    },
    MenuValueSegment::Quad {
        control: [5046, 773],
        to: [5010, 798],
    },
    MenuValueSegment::Quad {
        control: [4972, 823],
        to: [4972, 861],
    },
    MenuValueSegment::Quad {
        control: [4972, 942],
        to: [5065, 983],
    },
    MenuValueSegment::Line([5171, 1031]),
    MenuValueSegment::Quad {
        control: [5268, 1077],
        to: [5313, 1133],
    },
    MenuValueSegment::Quad {
        control: [5357, 1191],
        to: [5357, 1277],
    },
    MenuValueSegment::Quad {
        control: [5357, 1389],
        to: [5279, 1453],
    },
    MenuValueSegment::Quad {
        control: [5199, 1516],
        to: [5060, 1516],
    },
    MenuValueSegment::Quad {
        control: [4927, 1516],
        to: [4812, 1450],
    },
    MenuValueSegment::Line([4863, 1309]),
    MenuValueSegment::Quad {
        control: [4988, 1391],
        to: [5063, 1391],
    },
    MenuValueSegment::Quad {
        control: [5201, 1391],
        to: [5201, 1275],
    },
    MenuValueSegment::Quad {
        control: [5201, 1192],
        to: [5068, 1133],
    },
    MenuValueSegment::Quad {
        control: [4966, 1086],
        to: [4930, 1062],
    },
    MenuValueSegment::Quad {
        control: [4894, 1038],
        to: [4869, 1008],
    },
    MenuValueSegment::Quad {
        control: [4843, 977],
        to: [4830, 942],
    },
    MenuValueSegment::Quad {
        control: [4816, 906],
        to: [4816, 867],
    },
    MenuValueSegment::Quad {
        control: [4816, 764],
        to: [4891, 706],
    },
    MenuValueSegment::Quad {
        control: [4966, 648],
        to: [5088, 648],
    },
    MenuValueSegment::Quad {
        control: [5179, 648],
        to: [5318, 706],
    },
];

const SPEED_DISABLED_CONTOUR_5: [MenuValueSegment; 18] = [
    MenuValueSegment::Quad {
        control: [5958, 648],
        to: [6041, 731],
    },
    MenuValueSegment::Quad {
        control: [6122, 814],
        to: [6122, 994],
    },
    MenuValueSegment::Line([6122, 1294]),
    MenuValueSegment::Quad {
        control: [6122, 1405],
        to: [6188, 1441],
    },
    MenuValueSegment::Line([6188, 1516]),
    MenuValueSegment::Quad {
        control: [6097, 1516],
        to: [6054, 1489],
    },
    MenuValueSegment::Quad {
        control: [6008, 1464],
        to: [5988, 1405],
    },
    MenuValueSegment::Quad {
        control: [5899, 1516],
        to: [5716, 1516],
    },
    MenuValueSegment::Quad {
        control: [5618, 1516],
        to: [5546, 1445],
    },
    MenuValueSegment::Quad {
        control: [5472, 1373],
        to: [5472, 1267],
    },
    MenuValueSegment::Quad {
        control: [5472, 1139],
        to: [5585, 1052],
    },
    MenuValueSegment::Quad {
        control: [5696, 964],
        to: [5868, 964],
    },
    MenuValueSegment::Line([5974, 984]),
    MenuValueSegment::Quad {
        control: [5974, 781],
        to: [5793, 781],
    },
    MenuValueSegment::Quad {
        control: [5654, 781],
        to: [5579, 856],
    },
    MenuValueSegment::Line([5516, 731]),
    MenuValueSegment::Quad {
        control: [5558, 697],
        to: [5633, 673],
    },
    MenuValueSegment::Quad {
        control: [5708, 648],
        to: [5776, 648],
    },
];

const SPEED_DISABLED_CONTOUR_6: [MenuValueSegment; 6] = [
    MenuValueSegment::Quad {
        control: [5763, 1073],
        to: [5693, 1131],
    },
    MenuValueSegment::Quad {
        control: [5621, 1189],
        to: [5621, 1269],
    },
    MenuValueSegment::Quad {
        control: [5621, 1398],
        to: [5776, 1398],
    },
    MenuValueSegment::Quad {
        control: [5888, 1398],
        to: [5974, 1292],
    },
    MenuValueSegment::Line([5974, 1089]),
    MenuValueSegment::Line([5876, 1073]),
];

const SPEED_DISABLED_CONTOUR_7: [MenuValueSegment; 13] = [
    MenuValueSegment::Line([6505, 725]),
    MenuValueSegment::Quad {
        control: [6525, 697],
        to: [6584, 672],
    },
    MenuValueSegment::Quad {
        control: [6642, 648],
        to: [6698, 648],
    },
    MenuValueSegment::Quad {
        control: [6870, 648],
        to: [6977, 767],
    },
    MenuValueSegment::Quad {
        control: [7083, 886],
        to: [7083, 1067],
    },
    MenuValueSegment::Quad {
        control: [7083, 1277],
        to: [6977, 1397],
    },
    MenuValueSegment::Quad {
        control: [6869, 1516],
        to: [6686, 1516],
    },
    MenuValueSegment::Quad {
        control: [6627, 1516],
        to: [6570, 1494],
    },
    MenuValueSegment::Quad {
        control: [6512, 1472],
        to: [6483, 1441],
    },
    MenuValueSegment::Line([6430, 1516]),
    MenuValueSegment::Line([6356, 1516]),
    MenuValueSegment::Line([6356, 320]),
    MenuValueSegment::Line([6505, 320]),
];

const SPEED_DISABLED_CONTOUR_8: [MenuValueSegment; 9] = [
    MenuValueSegment::Line([6505, 1322]),
    MenuValueSegment::Quad {
        control: [6505, 1334],
        to: [6559, 1362],
    },
    MenuValueSegment::Quad {
        control: [6616, 1391],
        to: [6644, 1391],
    },
    MenuValueSegment::Quad {
        control: [6795, 1391],
        to: [6861, 1319],
    },
    MenuValueSegment::Quad {
        control: [6927, 1245],
        to: [6927, 1075],
    },
    MenuValueSegment::Quad {
        control: [6927, 933],
        to: [6850, 853],
    },
    MenuValueSegment::Quad {
        control: [6773, 773],
        to: [6644, 773],
    },
    MenuValueSegment::Quad {
        control: [6617, 773],
        to: [6569, 797],
    },
    MenuValueSegment::Line([6505, 839]),
];

const SPEED_DISABLED_CONTOUR_9: [MenuValueSegment; 7] = [
    MenuValueSegment::Line([7546, 1516]),
    MenuValueSegment::Quad {
        control: [7257, 1516],
        to: [7257, 1264],
    },
    MenuValueSegment::Line([7257, 320]),
    MenuValueSegment::Line([7406, 320]),
    MenuValueSegment::Line([7406, 1239]),
    MenuValueSegment::Quad {
        control: [7406, 1306],
        to: [7445, 1345],
    },
    MenuValueSegment::Quad {
        control: [7484, 1383],
        to: [7546, 1383],
    },
];

const SPEED_DISABLED_CONTOUR_10: [MenuValueSegment; 5] = [
    MenuValueSegment::Quad {
        control: [7962, 773],
        to: [7894, 838],
    },
    MenuValueSegment::Quad {
        control: [7830, 898],
        to: [7821, 989],
    },
    MenuValueSegment::Line([8285, 989]),
    MenuValueSegment::Quad {
        control: [8285, 898],
        to: [8229, 839],
    },
    MenuValueSegment::Quad {
        control: [8166, 773],
        to: [8060, 773],
    },
];

const SPEED_DISABLED_CONTOUR_11: [MenuValueSegment; 14] = [
    MenuValueSegment::Quad {
        control: [7969, 1391],
        to: [8080, 1391],
    },
    MenuValueSegment::Quad {
        control: [8208, 1391],
        to: [8293, 1317],
    },
    MenuValueSegment::Line([8355, 1423]),
    MenuValueSegment::Quad {
        control: [8321, 1458],
        to: [8251, 1483],
    },
    MenuValueSegment::Quad {
        control: [8162, 1516],
        to: [8052, 1516],
    },
    MenuValueSegment::Quad {
        control: [7894, 1516],
        to: [7783, 1409],
    },
    MenuValueSegment::Quad {
        control: [7662, 1291],
        to: [7662, 1092],
    },
    MenuValueSegment::Quad {
        control: [7662, 884],
        to: [7787, 759],
    },
    MenuValueSegment::Quad {
        control: [7899, 648],
        to: [8054, 648],
    },
    MenuValueSegment::Quad {
        control: [8232, 648],
        to: [8333, 748],
    },
    MenuValueSegment::Quad {
        control: [8432, 845],
        to: [8432, 1006],
    },
    MenuValueSegment::Quad {
        control: [8432, 1055],
        to: [8421, 1097],
    },
    MenuValueSegment::Line([7818, 1097]),
    MenuValueSegment::Quad {
        control: [7818, 1244],
        to: [7897, 1322],
    },
];

const SPEED_DISABLED_CONTOUR_12: [MenuValueSegment; 11] = [
    MenuValueSegment::Line([9269, 320]),
    MenuValueSegment::Line([9269, 1500]),
    MenuValueSegment::Line([9121, 1500]),
    MenuValueSegment::Line([9121, 1438]),
    MenuValueSegment::Quad {
        control: [9044, 1516],
        to: [8896, 1516],
    },
    MenuValueSegment::Quad {
        control: [8740, 1516],
        to: [8641, 1403],
    },
    MenuValueSegment::Quad {
        control: [8544, 1291],
        to: [8544, 1103],
    },
    MenuValueSegment::Quad {
        control: [8544, 914],
        to: [8657, 781],
    },
    MenuValueSegment::Quad {
        control: [8769, 648],
        to: [8924, 648],
    },
    MenuValueSegment::Quad {
        control: [9054, 648],
        to: [9121, 709],
    },
    MenuValueSegment::Line([9121, 320]),
];

const SPEED_DISABLED_CONTOUR_13: [MenuValueSegment; 7] = [
    MenuValueSegment::Quad {
        control: [9108, 1347],
        to: [9121, 1322],
    },
    MenuValueSegment::Line([9121, 858]),
    MenuValueSegment::Quad {
        control: [9065, 773],
        to: [8968, 773],
    },
    MenuValueSegment::Quad {
        control: [8848, 773],
        to: [8774, 862],
    },
    MenuValueSegment::Quad {
        control: [8701, 952],
        to: [8701, 1089],
    },
    MenuValueSegment::Quad {
        control: [8701, 1391],
        to: [8976, 1391],
    },
    MenuValueSegment::Quad {
        control: [9010, 1391],
        to: [9060, 1369],
    },
];

const SPEED_DISABLED_CONTOURS: [MenuValueContour; 14] = [
    MenuValueContour {
        start: [4051, 320],
        segments: &SPEED_DISABLED_CONTOUR_0,
    },
    MenuValueContour {
        start: [3990, 1369],
        segments: &SPEED_DISABLED_CONTOUR_1,
    },
    MenuValueContour {
        start: [4564, 347],
        segments: &SPEED_DISABLED_CONTOUR_2,
    },
    MenuValueContour {
        start: [4632, 664],
        segments: &SPEED_DISABLED_CONTOUR_3,
    },
    MenuValueContour {
        start: [5318, 706],
        segments: &SPEED_DISABLED_CONTOUR_4,
    },
    MenuValueContour {
        start: [5776, 648],
        segments: &SPEED_DISABLED_CONTOUR_5,
    },
    MenuValueContour {
        start: [5876, 1073],
        segments: &SPEED_DISABLED_CONTOUR_6,
    },
    MenuValueContour {
        start: [6505, 320],
        segments: &SPEED_DISABLED_CONTOUR_7,
    },
    MenuValueContour {
        start: [6505, 839],
        segments: &SPEED_DISABLED_CONTOUR_8,
    },
    MenuValueContour {
        start: [7546, 1383],
        segments: &SPEED_DISABLED_CONTOUR_9,
    },
    MenuValueContour {
        start: [8060, 773],
        segments: &SPEED_DISABLED_CONTOUR_10,
    },
    MenuValueContour {
        start: [7897, 1322],
        segments: &SPEED_DISABLED_CONTOUR_11,
    },
    MenuValueContour {
        start: [9121, 320],
        segments: &SPEED_DISABLED_CONTOUR_12,
    },
    MenuValueContour {
        start: [9060, 1369],
        segments: &SPEED_DISABLED_CONTOUR_13,
    },
];

pub const SPEED_DISABLED: MenuValueDefinition = MenuValueDefinition {
    text: "disabled",
    define_text_id: 68,
    font_ids: &SPEED_DISABLED_FONT_IDS,
    color_rgb: [255, 255, 255],
    bounds_centipx: [3475, 10075, 320, 1515],
    contours: &SPEED_DISABLED_CONTOURS,
};

const SPEED_ENABLED_FONT_IDS: [u16; 1] = [26];

const SPEED_ENABLED_CONTOUR_0: [MenuValueSegment; 5] = [
    MenuValueSegment::Quad {
        control: [3892, 773],
        to: [3824, 838],
    },
    MenuValueSegment::Quad {
        control: [3760, 898],
        to: [3751, 989],
    },
    MenuValueSegment::Line([4215, 989]),
    MenuValueSegment::Quad {
        control: [4215, 898],
        to: [4159, 839],
    },
    MenuValueSegment::Quad {
        control: [4096, 773],
        to: [3990, 773],
    },
];

const SPEED_ENABLED_CONTOUR_1: [MenuValueSegment; 14] = [
    MenuValueSegment::Quad {
        control: [3899, 1391],
        to: [4010, 1391],
    },
    MenuValueSegment::Quad {
        control: [4138, 1391],
        to: [4223, 1317],
    },
    MenuValueSegment::Line([4285, 1423]),
    MenuValueSegment::Quad {
        control: [4251, 1458],
        to: [4181, 1483],
    },
    MenuValueSegment::Quad {
        control: [4092, 1516],
        to: [3982, 1516],
    },
    MenuValueSegment::Quad {
        control: [3824, 1516],
        to: [3713, 1409],
    },
    MenuValueSegment::Quad {
        control: [3592, 1291],
        to: [3592, 1092],
    },
    MenuValueSegment::Quad {
        control: [3592, 884],
        to: [3717, 759],
    },
    MenuValueSegment::Quad {
        control: [3829, 648],
        to: [3984, 648],
    },
    MenuValueSegment::Quad {
        control: [4162, 648],
        to: [4263, 748],
    },
    MenuValueSegment::Quad {
        control: [4362, 845],
        to: [4362, 1006],
    },
    MenuValueSegment::Quad {
        control: [4362, 1055],
        to: [4351, 1097],
    },
    MenuValueSegment::Line([3748, 1097]),
    MenuValueSegment::Quad {
        control: [3748, 1244],
        to: [3828, 1322],
    },
];

const SPEED_ENABLED_CONTOUR_2: [MenuValueSegment; 14] = [
    MenuValueSegment::Line([5184, 1500]),
    MenuValueSegment::Line([5035, 1500]),
    MenuValueSegment::Line([5035, 1014]),
    MenuValueSegment::Quad {
        control: [5035, 880],
        to: [4996, 827],
    },
    MenuValueSegment::Quad {
        control: [4956, 773],
        to: [4860, 773],
    },
    MenuValueSegment::Quad {
        control: [4810, 773],
        to: [4754, 803],
    },
    MenuValueSegment::Quad {
        control: [4699, 834],
        to: [4670, 878],
    },
    MenuValueSegment::Line([4670, 1500]),
    MenuValueSegment::Line([4521, 1500]),
    MenuValueSegment::Line([4521, 664]),
    MenuValueSegment::Line([4623, 664]),
    MenuValueSegment::Line([4670, 772]),
    MenuValueSegment::Quad {
        control: [4743, 648],
        to: [4909, 648],
    },
    MenuValueSegment::Quad {
        control: [5184, 648],
        to: [5184, 983],
    },
];

const SPEED_ENABLED_CONTOUR_3: [MenuValueSegment; 18] = [
    MenuValueSegment::Quad {
        control: [5838, 648],
        to: [5921, 731],
    },
    MenuValueSegment::Quad {
        control: [6002, 814],
        to: [6002, 994],
    },
    MenuValueSegment::Line([6002, 1294]),
    MenuValueSegment::Quad {
        control: [6002, 1405],
        to: [6068, 1441],
    },
    MenuValueSegment::Line([6068, 1516]),
    MenuValueSegment::Quad {
        control: [5978, 1516],
        to: [5934, 1489],
    },
    MenuValueSegment::Quad {
        control: [5888, 1464],
        to: [5868, 1405],
    },
    MenuValueSegment::Quad {
        control: [5779, 1516],
        to: [5596, 1516],
    },
    MenuValueSegment::Quad {
        control: [5498, 1516],
        to: [5426, 1445],
    },
    MenuValueSegment::Quad {
        control: [5352, 1373],
        to: [5352, 1267],
    },
    MenuValueSegment::Quad {
        control: [5352, 1139],
        to: [5465, 1052],
    },
    MenuValueSegment::Quad {
        control: [5576, 964],
        to: [5748, 964],
    },
    MenuValueSegment::Line([5854, 984]),
    MenuValueSegment::Quad {
        control: [5854, 781],
        to: [5673, 781],
    },
    MenuValueSegment::Quad {
        control: [5534, 781],
        to: [5459, 856],
    },
    MenuValueSegment::Line([5396, 731]),
    MenuValueSegment::Quad {
        control: [5438, 697],
        to: [5513, 673],
    },
    MenuValueSegment::Quad {
        control: [5588, 648],
        to: [5656, 648],
    },
];

const SPEED_ENABLED_CONTOUR_4: [MenuValueSegment; 6] = [
    MenuValueSegment::Quad {
        control: [5643, 1073],
        to: [5573, 1131],
    },
    MenuValueSegment::Quad {
        control: [5501, 1189],
        to: [5501, 1269],
    },
    MenuValueSegment::Quad {
        control: [5501, 1398],
        to: [5656, 1398],
    },
    MenuValueSegment::Quad {
        control: [5768, 1398],
        to: [5854, 1292],
    },
    MenuValueSegment::Line([5854, 1089]),
    MenuValueSegment::Line([5756, 1073]),
];

const SPEED_ENABLED_CONTOUR_5: [MenuValueSegment; 13] = [
    MenuValueSegment::Line([6385, 725]),
    MenuValueSegment::Quad {
        control: [6405, 697],
        to: [6464, 672],
    },
    MenuValueSegment::Quad {
        control: [6522, 648],
        to: [6578, 648],
    },
    MenuValueSegment::Quad {
        control: [6750, 648],
        to: [6857, 767],
    },
    MenuValueSegment::Quad {
        control: [6963, 886],
        to: [6963, 1067],
    },
    MenuValueSegment::Quad {
        control: [6963, 1277],
        to: [6857, 1397],
    },
    MenuValueSegment::Quad {
        control: [6749, 1516],
        to: [6566, 1516],
    },
    MenuValueSegment::Quad {
        control: [6507, 1516],
        to: [6450, 1494],
    },
    MenuValueSegment::Quad {
        control: [6392, 1472],
        to: [6363, 1441],
    },
    MenuValueSegment::Line([6310, 1516]),
    MenuValueSegment::Line([6236, 1516]),
    MenuValueSegment::Line([6236, 320]),
    MenuValueSegment::Line([6385, 320]),
];

const SPEED_ENABLED_CONTOUR_6: [MenuValueSegment; 9] = [
    MenuValueSegment::Line([6385, 1322]),
    MenuValueSegment::Quad {
        control: [6385, 1334],
        to: [6439, 1362],
    },
    MenuValueSegment::Quad {
        control: [6496, 1391],
        to: [6524, 1391],
    },
    MenuValueSegment::Quad {
        control: [6675, 1391],
        to: [6741, 1319],
    },
    MenuValueSegment::Quad {
        control: [6807, 1245],
        to: [6807, 1075],
    },
    MenuValueSegment::Quad {
        control: [6807, 933],
        to: [6730, 853],
    },
    MenuValueSegment::Quad {
        control: [6653, 773],
        to: [6524, 773],
    },
    MenuValueSegment::Quad {
        control: [6497, 773],
        to: [6449, 797],
    },
    MenuValueSegment::Line([6385, 839]),
];

const SPEED_ENABLED_CONTOUR_7: [MenuValueSegment; 7] = [
    MenuValueSegment::Line([7426, 1516]),
    MenuValueSegment::Quad {
        control: [7137, 1516],
        to: [7137, 1264],
    },
    MenuValueSegment::Line([7137, 320]),
    MenuValueSegment::Line([7286, 320]),
    MenuValueSegment::Line([7286, 1239]),
    MenuValueSegment::Quad {
        control: [7286, 1306],
        to: [7325, 1345],
    },
    MenuValueSegment::Quad {
        control: [7364, 1383],
        to: [7426, 1383],
    },
];

const SPEED_ENABLED_CONTOUR_8: [MenuValueSegment; 5] = [
    MenuValueSegment::Quad {
        control: [7842, 773],
        to: [7774, 838],
    },
    MenuValueSegment::Quad {
        control: [7710, 898],
        to: [7701, 989],
    },
    MenuValueSegment::Line([8165, 989]),
    MenuValueSegment::Quad {
        control: [8165, 898],
        to: [8109, 839],
    },
    MenuValueSegment::Quad {
        control: [8046, 773],
        to: [7940, 773],
    },
];

const SPEED_ENABLED_CONTOUR_9: [MenuValueSegment; 14] = [
    MenuValueSegment::Quad {
        control: [7849, 1391],
        to: [7960, 1391],
    },
    MenuValueSegment::Quad {
        control: [8088, 1391],
        to: [8173, 1317],
    },
    MenuValueSegment::Line([8235, 1423]),
    MenuValueSegment::Quad {
        control: [8201, 1458],
        to: [8131, 1483],
    },
    MenuValueSegment::Quad {
        control: [8042, 1516],
        to: [7932, 1516],
    },
    MenuValueSegment::Quad {
        control: [7774, 1516],
        to: [7663, 1409],
    },
    MenuValueSegment::Quad {
        control: [7542, 1291],
        to: [7542, 1092],
    },
    MenuValueSegment::Quad {
        control: [7542, 884],
        to: [7667, 759],
    },
    MenuValueSegment::Quad {
        control: [7779, 648],
        to: [7934, 648],
    },
    MenuValueSegment::Quad {
        control: [8112, 648],
        to: [8213, 748],
    },
    MenuValueSegment::Quad {
        control: [8312, 845],
        to: [8312, 1006],
    },
    MenuValueSegment::Quad {
        control: [8312, 1055],
        to: [8301, 1097],
    },
    MenuValueSegment::Line([7698, 1097]),
    MenuValueSegment::Quad {
        control: [7698, 1244],
        to: [7778, 1322],
    },
];

const SPEED_ENABLED_CONTOUR_10: [MenuValueSegment; 11] = [
    MenuValueSegment::Line([9149, 320]),
    MenuValueSegment::Line([9149, 1500]),
    MenuValueSegment::Line([9001, 1500]),
    MenuValueSegment::Line([9001, 1438]),
    MenuValueSegment::Quad {
        control: [8924, 1516],
        to: [8776, 1516],
    },
    MenuValueSegment::Quad {
        control: [8620, 1516],
        to: [8521, 1403],
    },
    MenuValueSegment::Quad {
        control: [8424, 1291],
        to: [8424, 1103],
    },
    MenuValueSegment::Quad {
        control: [8424, 914],
        to: [8537, 781],
    },
    MenuValueSegment::Quad {
        control: [8649, 648],
        to: [8804, 648],
    },
    MenuValueSegment::Quad {
        control: [8934, 648],
        to: [9001, 709],
    },
    MenuValueSegment::Line([9001, 320]),
];

const SPEED_ENABLED_CONTOUR_11: [MenuValueSegment; 7] = [
    MenuValueSegment::Quad {
        control: [8988, 1347],
        to: [9001, 1322],
    },
    MenuValueSegment::Line([9001, 858]),
    MenuValueSegment::Quad {
        control: [8945, 773],
        to: [8848, 773],
    },
    MenuValueSegment::Quad {
        control: [8728, 773],
        to: [8654, 862],
    },
    MenuValueSegment::Quad {
        control: [8581, 952],
        to: [8581, 1089],
    },
    MenuValueSegment::Quad {
        control: [8581, 1391],
        to: [8856, 1391],
    },
    MenuValueSegment::Quad {
        control: [8890, 1391],
        to: [8940, 1369],
    },
];

const SPEED_ENABLED_CONTOURS: [MenuValueContour; 12] = [
    MenuValueContour {
        start: [3990, 773],
        segments: &SPEED_ENABLED_CONTOUR_0,
    },
    MenuValueContour {
        start: [3828, 1322],
        segments: &SPEED_ENABLED_CONTOUR_1,
    },
    MenuValueContour {
        start: [5184, 983],
        segments: &SPEED_ENABLED_CONTOUR_2,
    },
    MenuValueContour {
        start: [5656, 648],
        segments: &SPEED_ENABLED_CONTOUR_3,
    },
    MenuValueContour {
        start: [5756, 1073],
        segments: &SPEED_ENABLED_CONTOUR_4,
    },
    MenuValueContour {
        start: [6385, 320],
        segments: &SPEED_ENABLED_CONTOUR_5,
    },
    MenuValueContour {
        start: [6385, 839],
        segments: &SPEED_ENABLED_CONTOUR_6,
    },
    MenuValueContour {
        start: [7426, 1383],
        segments: &SPEED_ENABLED_CONTOUR_7,
    },
    MenuValueContour {
        start: [7940, 773],
        segments: &SPEED_ENABLED_CONTOUR_8,
    },
    MenuValueContour {
        start: [7778, 1322],
        segments: &SPEED_ENABLED_CONTOUR_9,
    },
    MenuValueContour {
        start: [9001, 320],
        segments: &SPEED_ENABLED_CONTOUR_10,
    },
    MenuValueContour {
        start: [8940, 1369],
        segments: &SPEED_ENABLED_CONTOUR_11,
    },
];

pub const SPEED_ENABLED: MenuValueDefinition = MenuValueDefinition {
    text: "enabled",
    define_text_id: 69,
    font_ids: &SPEED_ENABLED_FONT_IDS,
    color_rgb: [255, 51, 51],
    bounds_centipx: [3590, 9955, 320, 1515],
    contours: &SPEED_ENABLED_CONTOURS,
};

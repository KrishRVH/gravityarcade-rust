// Generated from gravity_arcade.swf help-screen DefineText tags.
// Coordinates are stored as SWF text-local centipixels; divide by 100 for pixels.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HelpLabelSegment {
    Line([i16; 2]),
    Quad { control: [i16; 2], to: [i16; 2] },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HelpLabelContour {
    pub(super) start: [i16; 2],
    pub(super) segments: &'static [HelpLabelSegment],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HelpLabelDefinition {
    pub(super) text: &'static str,
    pub(super) define_text_id: u16,
    pub(super) font_ids: &'static [u16],
    pub(super) color_rgb: [u8; 3],
    pub(super) bounds_centipx: [i16; 4],
    pub(super) contours: &'static [HelpLabelContour],
}

const TITLE_FONT_IDS: [u16; 1] = [26];

const TITLE_CONTOUR_0: [HelpLabelSegment; 12] = [
    HelpLabelSegment::Line([4292, 1180]),
    HelpLabelSegment::Line([4175, 1180]),
    HelpLabelSegment::Line([4175, 765]),
    HelpLabelSegment::Line([3800, 765]),
    HelpLabelSegment::Line([3800, 1180]),
    HelpLabelSegment::Line([3683, 1180]),
    HelpLabelSegment::Line([3683, 322]),
    HelpLabelSegment::Line([3800, 322]),
    HelpLabelSegment::Line([3800, 666]),
    HelpLabelSegment::Line([4175, 666]),
    HelpLabelSegment::Line([4175, 322]),
    HelpLabelSegment::Line([4292, 322]),
];

const TITLE_CONTOUR_1: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [4536, 1101],
        to: [4702, 1101],
    },
    HelpLabelSegment::Quad {
        control: [4781, 1101],
        to: [4825, 1038],
    },
    HelpLabelSegment::Quad {
        control: [4869, 975],
        to: [4869, 865],
    },
    HelpLabelSegment::Quad {
        control: [4869, 632],
        to: [4702, 632],
    },
    HelpLabelSegment::Quad {
        control: [4626, 632],
        to: [4582, 694],
    },
    HelpLabelSegment::Quad {
        control: [4536, 756],
        to: [4536, 865],
    },
];

const TITLE_CONTOUR_2: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [4575, 541],
        to: [4702, 541],
    },
    HelpLabelSegment::Quad {
        control: [4837, 541],
        to: [4912, 627],
    },
    HelpLabelSegment::Quad {
        control: [4986, 712],
        to: [4986, 865],
    },
    HelpLabelSegment::Quad {
        control: [4986, 1017],
        to: [4910, 1105],
    },
    HelpLabelSegment::Quad {
        control: [4834, 1192],
        to: [4702, 1192],
    },
    HelpLabelSegment::Quad {
        control: [4569, 1192],
        to: [4494, 1104],
    },
    HelpLabelSegment::Quad {
        control: [4419, 1015],
        to: [4419, 865],
    },
    HelpLabelSegment::Quad {
        control: [4419, 719],
        to: [4497, 630],
    },
];

const TITLE_CONTOUR_3: [HelpLabelSegment; 13] = [
    HelpLabelSegment::Line([5685, 1192]),
    HelpLabelSegment::Line([5655, 1192]),
    HelpLabelSegment::Line([5471, 765]),
    HelpLabelSegment::Line([5288, 1192]),
    HelpLabelSegment::Line([5258, 1192]),
    HelpLabelSegment::Line([5034, 551]),
    HelpLabelSegment::Line([5153, 551]),
    HelpLabelSegment::Line([5288, 963]),
    HelpLabelSegment::Line([5454, 551]),
    HelpLabelSegment::Line([5483, 551]),
    HelpLabelSegment::Line([5655, 963]),
    HelpLabelSegment::Line([5800, 551]),
    HelpLabelSegment::Line([5910, 551]),
];

const TITLE_CONTOUR_4: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Line([6400, 422]),
    HelpLabelSegment::Line([6511, 378]),
    HelpLabelSegment::Line([6511, 553]),
    HelpLabelSegment::Line([6683, 553]),
    HelpLabelSegment::Line([6683, 641]),
    HelpLabelSegment::Line([6511, 641]),
    HelpLabelSegment::Line([6511, 953]),
    HelpLabelSegment::Quad {
        control: [6511, 1031],
        to: [6538, 1065],
    },
    HelpLabelSegment::Quad {
        control: [6564, 1098],
        to: [6623, 1098],
    },
    HelpLabelSegment::Quad {
        control: [6666, 1098],
        to: [6711, 1077],
    },
    HelpLabelSegment::Line([6728, 1174]),
    HelpLabelSegment::Line([6576, 1192]),
    HelpLabelSegment::Quad {
        control: [6501, 1192],
        to: [6451, 1137],
    },
    HelpLabelSegment::Quad {
        control: [6400, 1082],
        to: [6400, 997],
    },
    HelpLabelSegment::Line([6400, 641]),
    HelpLabelSegment::Line([6327, 641]),
    HelpLabelSegment::Line([6327, 553]),
    HelpLabelSegment::Line([6400, 553]),
];

const TITLE_CONTOUR_5: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [6911, 1101],
        to: [7077, 1101],
    },
    HelpLabelSegment::Quad {
        control: [7156, 1101],
        to: [7200, 1038],
    },
    HelpLabelSegment::Quad {
        control: [7244, 975],
        to: [7244, 865],
    },
    HelpLabelSegment::Quad {
        control: [7244, 632],
        to: [7077, 632],
    },
    HelpLabelSegment::Quad {
        control: [7001, 632],
        to: [6957, 694],
    },
    HelpLabelSegment::Quad {
        control: [6911, 756],
        to: [6911, 865],
    },
];

const TITLE_CONTOUR_6: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [6950, 541],
        to: [7077, 541],
    },
    HelpLabelSegment::Quad {
        control: [7212, 541],
        to: [7287, 627],
    },
    HelpLabelSegment::Quad {
        control: [7361, 712],
        to: [7361, 865],
    },
    HelpLabelSegment::Quad {
        control: [7361, 1017],
        to: [7285, 1105],
    },
    HelpLabelSegment::Quad {
        control: [7209, 1192],
        to: [7077, 1192],
    },
    HelpLabelSegment::Quad {
        control: [6944, 1192],
        to: [6869, 1104],
    },
    HelpLabelSegment::Quad {
        control: [6794, 1015],
        to: [6794, 865],
    },
    HelpLabelSegment::Quad {
        control: [6794, 719],
        to: [6872, 630],
    },
];

const TITLE_CONTOUR_7: [HelpLabelSegment; 12] = [
    HelpLabelSegment::Line([7951, 605]),
    HelpLabelSegment::Quad {
        control: [8014, 541],
        to: [8103, 541],
    },
    HelpLabelSegment::Quad {
        control: [8237, 541],
        to: [8312, 625],
    },
    HelpLabelSegment::Quad {
        control: [8386, 708],
        to: [8386, 868],
    },
    HelpLabelSegment::Quad {
        control: [8386, 1011],
        to: [8311, 1101],
    },
    HelpLabelSegment::Quad {
        control: [8236, 1192],
        to: [8094, 1192],
    },
    HelpLabelSegment::Quad {
        control: [8054, 1192],
        to: [8010, 1178],
    },
    HelpLabelSegment::Quad {
        control: [7964, 1164],
        to: [7951, 1146],
    },
    HelpLabelSegment::Line([7951, 1426]),
    HelpLabelSegment::Line([7840, 1426]),
    HelpLabelSegment::Line([7840, 553]),
    HelpLabelSegment::Line([7951, 553]),
];

const TITLE_CONTOUR_8: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Line([7951, 1053]),
    HelpLabelSegment::Quad {
        control: [7962, 1070],
        to: [7996, 1084],
    },
    HelpLabelSegment::Quad {
        control: [8030, 1098],
        to: [8061, 1098],
    },
    HelpLabelSegment::Quad {
        control: [8269, 1098],
        to: [8269, 864],
    },
    HelpLabelSegment::Quad {
        control: [8269, 745],
        to: [8219, 690],
    },
    HelpLabelSegment::Quad {
        control: [8170, 635],
        to: [8062, 635],
    },
    HelpLabelSegment::Quad {
        control: [8039, 635],
        to: [8005, 651],
    },
    HelpLabelSegment::Line([7951, 688]),
];

const TITLE_CONTOUR_9: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([8735, 1192]),
    HelpLabelSegment::Quad {
        control: [8518, 1192],
        to: [8518, 1003],
    },
    HelpLabelSegment::Line([8518, 295]),
    HelpLabelSegment::Line([8629, 295]),
    HelpLabelSegment::Line([8629, 984]),
    HelpLabelSegment::Quad {
        control: [8629, 1035],
        to: [8659, 1064],
    },
    HelpLabelSegment::Quad {
        control: [8688, 1092],
        to: [8735, 1092],
    },
];

const TITLE_CONTOUR_10: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Quad {
        control: [9196, 541],
        to: [9258, 603],
    },
    HelpLabelSegment::Quad {
        control: [9319, 666],
        to: [9319, 800],
    },
    HelpLabelSegment::Line([9319, 1025]),
    HelpLabelSegment::Quad {
        control: [9319, 1109],
        to: [9369, 1135],
    },
    HelpLabelSegment::Line([9369, 1192]),
    HelpLabelSegment::Quad {
        control: [9301, 1192],
        to: [9268, 1172],
    },
    HelpLabelSegment::Quad {
        control: [9234, 1153],
        to: [9219, 1109],
    },
    HelpLabelSegment::Quad {
        control: [9152, 1192],
        to: [9015, 1192],
    },
    HelpLabelSegment::Quad {
        control: [8941, 1192],
        to: [8887, 1139],
    },
    HelpLabelSegment::Quad {
        control: [8832, 1085],
        to: [8832, 1005],
    },
    HelpLabelSegment::Quad {
        control: [8832, 909],
        to: [8916, 844],
    },
    HelpLabelSegment::Quad {
        control: [8999, 778],
        to: [9128, 778],
    },
    HelpLabelSegment::Line([9208, 793]),
    HelpLabelSegment::Quad {
        control: [9208, 641],
        to: [9072, 641],
    },
    HelpLabelSegment::Quad {
        control: [8968, 641],
        to: [8912, 697],
    },
    HelpLabelSegment::Line([8865, 603]),
    HelpLabelSegment::Quad {
        control: [8896, 578],
        to: [8953, 560],
    },
    HelpLabelSegment::Quad {
        control: [9009, 541],
        to: [9059, 541],
    },
];

const TITLE_CONTOUR_11: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [9050, 860],
        to: [8997, 903],
    },
    HelpLabelSegment::Quad {
        control: [8943, 947],
        to: [8943, 1007],
    },
    HelpLabelSegment::Quad {
        control: [8943, 1104],
        to: [9059, 1104],
    },
    HelpLabelSegment::Quad {
        control: [9144, 1104],
        to: [9208, 1024],
    },
    HelpLabelSegment::Line([9208, 872]),
    HelpLabelSegment::Line([9134, 860]),
];

const TITLE_CONTOUR_12: [HelpLabelSegment; 13] = [
    HelpLabelSegment::Line([9734, 1287]),
    HelpLabelSegment::Quad {
        control: [9713, 1346],
        to: [9644, 1386],
    },
    HelpLabelSegment::Quad {
        control: [9573, 1426],
        to: [9488, 1426],
    },
    HelpLabelSegment::Line([9488, 1326]),
    HelpLabelSegment::Quad {
        control: [9558, 1326],
        to: [9607, 1295],
    },
    HelpLabelSegment::Quad {
        control: [9658, 1262],
        to: [9658, 1215],
    },
    HelpLabelSegment::Quad {
        control: [9658, 1164],
        to: [9639, 1113],
    },
    HelpLabelSegment::Line([9592, 989]),
    HelpLabelSegment::Line([9422, 553]),
    HelpLabelSegment::Line([9536, 553]),
    HelpLabelSegment::Line([9721, 1038]),
    HelpLabelSegment::Line([9886, 553]),
    HelpLabelSegment::Line([10000, 553]),
];

const TITLE_CONTOUR_13: [HelpLabelSegment; 17] = [
    HelpLabelSegment::Quad {
        control: [10740, 609],
        to: [10708, 642],
    },
    HelpLabelSegment::Line([10623, 734]),
    HelpLabelSegment::Quad {
        control: [10571, 797],
        to: [10571, 875],
    },
    HelpLabelSegment::Quad {
        control: [10571, 888],
        to: [10579, 929],
    },
    HelpLabelSegment::Line([10502, 929]),
    HelpLabelSegment::Quad {
        control: [10482, 873],
        to: [10482, 848],
    },
    HelpLabelSegment::Quad {
        control: [10482, 806],
        to: [10499, 763],
    },
    HelpLabelSegment::Quad {
        control: [10514, 718],
        to: [10569, 657],
    },
    HelpLabelSegment::Line([10642, 564]),
    HelpLabelSegment::Quad {
        control: [10660, 531],
        to: [10660, 499],
    },
    HelpLabelSegment::Quad {
        control: [10660, 396],
        to: [10536, 396],
    },
    HelpLabelSegment::Quad {
        control: [10476, 396],
        to: [10432, 439],
    },
    HelpLabelSegment::Line([10388, 355]),
    HelpLabelSegment::Quad {
        control: [10448, 307],
        to: [10565, 307],
    },
    HelpLabelSegment::Quad {
        control: [10653, 307],
        to: [10712, 356],
    },
    HelpLabelSegment::Quad {
        control: [10769, 407],
        to: [10769, 490],
    },
    HelpLabelSegment::Quad {
        control: [10769, 532],
        to: [10755, 571],
    },
];

const TITLE_CONTOUR_14: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [10591, 1016],
        to: [10617, 1042],
    },
    HelpLabelSegment::Quad {
        control: [10643, 1068],
        to: [10643, 1104],
    },
    HelpLabelSegment::Quad {
        control: [10643, 1140],
        to: [10617, 1166],
    },
    HelpLabelSegment::Quad {
        control: [10591, 1192],
        to: [10555, 1192],
    },
    HelpLabelSegment::Quad {
        control: [10519, 1192],
        to: [10493, 1166],
    },
    HelpLabelSegment::Quad {
        control: [10467, 1140],
        to: [10467, 1104],
    },
    HelpLabelSegment::Quad {
        control: [10467, 1068],
        to: [10493, 1042],
    },
    HelpLabelSegment::Quad {
        control: [10519, 1016],
        to: [10555, 1016],
    },
];

const TITLE_CONTOURS: [HelpLabelContour; 15] = [
    HelpLabelContour {
        start: [4292, 322],
        segments: &TITLE_CONTOUR_0,
    },
    HelpLabelContour {
        start: [4536, 865],
        segments: &TITLE_CONTOUR_1,
    },
    HelpLabelContour {
        start: [4497, 630],
        segments: &TITLE_CONTOUR_2,
    },
    HelpLabelContour {
        start: [5910, 551],
        segments: &TITLE_CONTOUR_3,
    },
    HelpLabelContour {
        start: [6400, 553],
        segments: &TITLE_CONTOUR_4,
    },
    HelpLabelContour {
        start: [6911, 865],
        segments: &TITLE_CONTOUR_5,
    },
    HelpLabelContour {
        start: [6872, 630],
        segments: &TITLE_CONTOUR_6,
    },
    HelpLabelContour {
        start: [7951, 553],
        segments: &TITLE_CONTOUR_7,
    },
    HelpLabelContour {
        start: [7951, 688],
        segments: &TITLE_CONTOUR_8,
    },
    HelpLabelContour {
        start: [8735, 1092],
        segments: &TITLE_CONTOUR_9,
    },
    HelpLabelContour {
        start: [9059, 541],
        segments: &TITLE_CONTOUR_10,
    },
    HelpLabelContour {
        start: [9134, 860],
        segments: &TITLE_CONTOUR_11,
    },
    HelpLabelContour {
        start: [10000, 553],
        segments: &TITLE_CONTOUR_12,
    },
    HelpLabelContour {
        start: [10755, 571],
        segments: &TITLE_CONTOUR_13,
    },
    HelpLabelContour {
        start: [10555, 1016],
        segments: &TITLE_CONTOUR_14,
    },
];

pub const TITLE: HelpLabelDefinition = HelpLabelDefinition {
    text: "How to play ?",
    define_text_id: 101,
    font_ids: &TITLE_FONT_IDS,
    color_rgb: [255, 255, 255],
    bounds_centipx: [3685, 11330, 295, 1425],
    contours: &TITLE_CONTOURS,
};

const BODY_FONT_IDS: [u16; 1] = [26];

const BODY_CONTOUR_0: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Line([-11021, 1180]),
    HelpLabelSegment::Line([-11138, 1180]),
    HelpLabelSegment::Line([-11138, 523]),
    HelpLabelSegment::Line([-11313, 632]),
    HelpLabelSegment::Line([-11313, 521]),
    HelpLabelSegment::Quad {
        control: [-11246, 489],
        to: [-11171, 430],
    },
    HelpLabelSegment::Quad {
        control: [-11097, 371],
        to: [-11056, 319],
    },
    HelpLabelSegment::Line([-11021, 319]),
];

const BODY_CONTOUR_1: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [-10556, 1016],
        to: [-10530, 1042],
    },
    HelpLabelSegment::Quad {
        control: [-10505, 1068],
        to: [-10505, 1104],
    },
    HelpLabelSegment::Quad {
        control: [-10505, 1140],
        to: [-10530, 1166],
    },
    HelpLabelSegment::Quad {
        control: [-10556, 1192],
        to: [-10593, 1192],
    },
    HelpLabelSegment::Quad {
        control: [-10629, 1192],
        to: [-10655, 1166],
    },
    HelpLabelSegment::Quad {
        control: [-10680, 1140],
        to: [-10680, 1104],
    },
    HelpLabelSegment::Quad {
        control: [-10680, 1068],
        to: [-10655, 1042],
    },
    HelpLabelSegment::Quad {
        control: [-10629, 1016],
        to: [-10593, 1016],
    },
];

const BODY_CONTOUR_2: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Line([-9619, 1180]),
    HelpLabelSegment::Line([-9736, 1180]),
    HelpLabelSegment::Line([-9736, 428]),
    HelpLabelSegment::Line([-10009, 428]),
    HelpLabelSegment::Line([-10009, 322]),
    HelpLabelSegment::Line([-9333, 322]),
    HelpLabelSegment::Line([-9333, 428]),
    HelpLabelSegment::Line([-9619, 428]),
];

const BODY_CONTOUR_3: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Quad {
        control: [-9071, 635],
        to: [-9108, 635],
    },
    HelpLabelSegment::Quad {
        control: [-9167, 635],
        to: [-9211, 689],
    },
    HelpLabelSegment::Quad {
        control: [-9256, 744],
        to: [-9256, 820],
    },
    HelpLabelSegment::Line([-9256, 1180]),
    HelpLabelSegment::Line([-9367, 1180]),
    HelpLabelSegment::Line([-9367, 553]),
    HelpLabelSegment::Line([-9256, 553]),
    HelpLabelSegment::Line([-9256, 653]),
    HelpLabelSegment::Quad {
        control: [-9195, 541],
        to: [-9074, 541],
    },
    HelpLabelSegment::Line([-8989, 552]),
    HelpLabelSegment::Line([-9034, 660]),
];

const BODY_CONTOUR_4: [HelpLabelSegment; 13] = [
    HelpLabelSegment::Line([-8671, 1287]),
    HelpLabelSegment::Quad {
        control: [-8692, 1346],
        to: [-8761, 1386],
    },
    HelpLabelSegment::Quad {
        control: [-8832, 1426],
        to: [-8917, 1426],
    },
    HelpLabelSegment::Line([-8917, 1326]),
    HelpLabelSegment::Quad {
        control: [-8847, 1326],
        to: [-8798, 1295],
    },
    HelpLabelSegment::Quad {
        control: [-8747, 1262],
        to: [-8747, 1215],
    },
    HelpLabelSegment::Quad {
        control: [-8747, 1164],
        to: [-8766, 1113],
    },
    HelpLabelSegment::Line([-8813, 989]),
    HelpLabelSegment::Line([-8983, 553]),
    HelpLabelSegment::Line([-8869, 553]),
    HelpLabelSegment::Line([-8684, 1038]),
    HelpLabelSegment::Line([-8519, 553]),
    HelpLabelSegment::Line([-8405, 553]),
];

const BODY_CONTOUR_5: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Line([-7920, 422]),
    HelpLabelSegment::Line([-7809, 378]),
    HelpLabelSegment::Line([-7809, 553]),
    HelpLabelSegment::Line([-7637, 553]),
    HelpLabelSegment::Line([-7637, 641]),
    HelpLabelSegment::Line([-7809, 641]),
    HelpLabelSegment::Line([-7809, 953]),
    HelpLabelSegment::Quad {
        control: [-7809, 1031],
        to: [-7782, 1065],
    },
    HelpLabelSegment::Quad {
        control: [-7756, 1098],
        to: [-7697, 1098],
    },
    HelpLabelSegment::Quad {
        control: [-7654, 1098],
        to: [-7609, 1077],
    },
    HelpLabelSegment::Line([-7592, 1174]),
    HelpLabelSegment::Line([-7744, 1192]),
    HelpLabelSegment::Quad {
        control: [-7819, 1192],
        to: [-7869, 1137],
    },
    HelpLabelSegment::Quad {
        control: [-7920, 1082],
        to: [-7920, 997],
    },
    HelpLabelSegment::Line([-7920, 641]),
    HelpLabelSegment::Line([-7993, 641]),
    HelpLabelSegment::Line([-7993, 553]),
    HelpLabelSegment::Line([-7920, 553]),
];

const BODY_CONTOUR_6: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [-7409, 1101],
        to: [-7243, 1101],
    },
    HelpLabelSegment::Quad {
        control: [-7164, 1101],
        to: [-7120, 1038],
    },
    HelpLabelSegment::Quad {
        control: [-7076, 975],
        to: [-7076, 865],
    },
    HelpLabelSegment::Quad {
        control: [-7076, 632],
        to: [-7243, 632],
    },
    HelpLabelSegment::Quad {
        control: [-7319, 632],
        to: [-7363, 694],
    },
    HelpLabelSegment::Quad {
        control: [-7409, 756],
        to: [-7409, 865],
    },
];

const BODY_CONTOUR_7: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [-7370, 541],
        to: [-7243, 541],
    },
    HelpLabelSegment::Quad {
        control: [-7108, 541],
        to: [-7033, 627],
    },
    HelpLabelSegment::Quad {
        control: [-6959, 712],
        to: [-6959, 865],
    },
    HelpLabelSegment::Quad {
        control: [-6959, 1017],
        to: [-7035, 1105],
    },
    HelpLabelSegment::Quad {
        control: [-7111, 1192],
        to: [-7243, 1192],
    },
    HelpLabelSegment::Quad {
        control: [-7376, 1192],
        to: [-7451, 1104],
    },
    HelpLabelSegment::Quad {
        control: [-7526, 1015],
        to: [-7526, 865],
    },
    HelpLabelSegment::Quad {
        control: [-7526, 719],
        to: [-7448, 630],
    },
];

const BODY_CONTOUR_8: [HelpLabelSegment; 22] = [
    HelpLabelSegment::Line([-6173, 688]),
    HelpLabelSegment::Quad {
        control: [-6239, 635],
        to: [-6306, 635],
    },
    HelpLabelSegment::Quad {
        control: [-6346, 635],
        to: [-6372, 654],
    },
    HelpLabelSegment::Quad {
        control: [-6401, 673],
        to: [-6401, 701],
    },
    HelpLabelSegment::Quad {
        control: [-6401, 762],
        to: [-6331, 792],
    },
    HelpLabelSegment::Line([-6252, 828]),
    HelpLabelSegment::Quad {
        control: [-6179, 862],
        to: [-6145, 905],
    },
    HelpLabelSegment::Quad {
        control: [-6112, 948],
        to: [-6112, 1012],
    },
    HelpLabelSegment::Quad {
        control: [-6112, 1097],
        to: [-6171, 1145],
    },
    HelpLabelSegment::Quad {
        control: [-6231, 1192],
        to: [-6335, 1192],
    },
    HelpLabelSegment::Quad {
        control: [-6435, 1192],
        to: [-6521, 1142],
    },
    HelpLabelSegment::Line([-6483, 1037]),
    HelpLabelSegment::Quad {
        control: [-6389, 1098],
        to: [-6333, 1098],
    },
    HelpLabelSegment::Quad {
        control: [-6230, 1098],
        to: [-6230, 1011],
    },
    HelpLabelSegment::Quad {
        control: [-6230, 949],
        to: [-6329, 905],
    },
    HelpLabelSegment::Quad {
        control: [-6405, 869],
        to: [-6432, 852],
    },
    HelpLabelSegment::Quad {
        control: [-6459, 833],
        to: [-6478, 811],
    },
    HelpLabelSegment::Quad {
        control: [-6498, 787],
        to: [-6507, 762],
    },
    HelpLabelSegment::Quad {
        control: [-6518, 735],
        to: [-6518, 705],
    },
    HelpLabelSegment::Quad {
        control: [-6518, 628],
        to: [-6462, 585],
    },
    HelpLabelSegment::Quad {
        control: [-6405, 541],
        to: [-6314, 541],
    },
    HelpLabelSegment::Quad {
        control: [-6246, 541],
        to: [-6142, 585],
    },
];

const BODY_CONTOUR_9: [HelpLabelSegment; 16] = [
    HelpLabelSegment::Line([-5884, 622]),
    HelpLabelSegment::Quad {
        control: [-5862, 587],
        to: [-5812, 565],
    },
    HelpLabelSegment::Quad {
        control: [-5763, 541],
        to: [-5711, 541],
    },
    HelpLabelSegment::Quad {
        control: [-5611, 541],
        to: [-5554, 607],
    },
    HelpLabelSegment::Quad {
        control: [-5497, 673],
        to: [-5497, 786],
    },
    HelpLabelSegment::Line([-5497, 1180]),
    HelpLabelSegment::Line([-5609, 1180]),
    HelpLabelSegment::Line([-5609, 786]),
    HelpLabelSegment::Quad {
        control: [-5609, 716],
        to: [-5644, 675],
    },
    HelpLabelSegment::Quad {
        control: [-5678, 635],
        to: [-5741, 635],
    },
    HelpLabelSegment::Quad {
        control: [-5781, 635],
        to: [-5822, 659],
    },
    HelpLabelSegment::Quad {
        control: [-5863, 682],
        to: [-5884, 714],
    },
    HelpLabelSegment::Line([-5884, 1180]),
    HelpLabelSegment::Line([-5995, 1180]),
    HelpLabelSegment::Line([-5995, 295]),
    HelpLabelSegment::Line([-5884, 295]),
];

const BODY_CONTOUR_10: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [-5264, 1101],
        to: [-5098, 1101],
    },
    HelpLabelSegment::Quad {
        control: [-5019, 1101],
        to: [-4975, 1038],
    },
    HelpLabelSegment::Quad {
        control: [-4931, 975],
        to: [-4931, 865],
    },
    HelpLabelSegment::Quad {
        control: [-4931, 632],
        to: [-5098, 632],
    },
    HelpLabelSegment::Quad {
        control: [-5174, 632],
        to: [-5218, 694],
    },
    HelpLabelSegment::Quad {
        control: [-5264, 756],
        to: [-5264, 865],
    },
];

const BODY_CONTOUR_11: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [-5225, 541],
        to: [-5098, 541],
    },
    HelpLabelSegment::Quad {
        control: [-4963, 541],
        to: [-4888, 627],
    },
    HelpLabelSegment::Quad {
        control: [-4814, 712],
        to: [-4814, 865],
    },
    HelpLabelSegment::Quad {
        control: [-4814, 1017],
        to: [-4890, 1105],
    },
    HelpLabelSegment::Quad {
        control: [-4966, 1192],
        to: [-5098, 1192],
    },
    HelpLabelSegment::Quad {
        control: [-5231, 1192],
        to: [-5306, 1104],
    },
    HelpLabelSegment::Quad {
        control: [-5381, 1015],
        to: [-5381, 865],
    },
    HelpLabelSegment::Quad {
        control: [-5381, 719],
        to: [-5303, 630],
    },
];

const BODY_CONTOUR_12: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [-4619, 1101],
        to: [-4453, 1101],
    },
    HelpLabelSegment::Quad {
        control: [-4374, 1101],
        to: [-4330, 1038],
    },
    HelpLabelSegment::Quad {
        control: [-4286, 975],
        to: [-4286, 865],
    },
    HelpLabelSegment::Quad {
        control: [-4286, 632],
        to: [-4453, 632],
    },
    HelpLabelSegment::Quad {
        control: [-4529, 632],
        to: [-4573, 694],
    },
    HelpLabelSegment::Quad {
        control: [-4619, 756],
        to: [-4619, 865],
    },
];

const BODY_CONTOUR_13: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [-4580, 541],
        to: [-4453, 541],
    },
    HelpLabelSegment::Quad {
        control: [-4318, 541],
        to: [-4243, 627],
    },
    HelpLabelSegment::Quad {
        control: [-4169, 712],
        to: [-4169, 865],
    },
    HelpLabelSegment::Quad {
        control: [-4169, 1017],
        to: [-4245, 1105],
    },
    HelpLabelSegment::Quad {
        control: [-4321, 1192],
        to: [-4453, 1192],
    },
    HelpLabelSegment::Quad {
        control: [-4586, 1192],
        to: [-4661, 1104],
    },
    HelpLabelSegment::Quad {
        control: [-4736, 1015],
        to: [-4736, 865],
    },
    HelpLabelSegment::Quad {
        control: [-4736, 719],
        to: [-4658, 630],
    },
];

const BODY_CONTOUR_14: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Line([-4010, 422]),
    HelpLabelSegment::Line([-3899, 378]),
    HelpLabelSegment::Line([-3899, 553]),
    HelpLabelSegment::Line([-3727, 553]),
    HelpLabelSegment::Line([-3727, 641]),
    HelpLabelSegment::Line([-3899, 641]),
    HelpLabelSegment::Line([-3899, 953]),
    HelpLabelSegment::Quad {
        control: [-3899, 1031],
        to: [-3872, 1065],
    },
    HelpLabelSegment::Quad {
        control: [-3846, 1098],
        to: [-3787, 1098],
    },
    HelpLabelSegment::Quad {
        control: [-3744, 1098],
        to: [-3699, 1077],
    },
    HelpLabelSegment::Line([-3682, 1174]),
    HelpLabelSegment::Line([-3834, 1192]),
    HelpLabelSegment::Quad {
        control: [-3909, 1192],
        to: [-3959, 1137],
    },
    HelpLabelSegment::Quad {
        control: [-4010, 1082],
        to: [-4010, 997],
    },
    HelpLabelSegment::Line([-4010, 641]),
    HelpLabelSegment::Line([-4083, 641]),
    HelpLabelSegment::Line([-4083, 553]),
    HelpLabelSegment::Line([-4010, 553]),
];

const BODY_CONTOUR_15: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Quad {
        control: [-2884, 541],
        to: [-2822, 603],
    },
    HelpLabelSegment::Quad {
        control: [-2761, 666],
        to: [-2761, 800],
    },
    HelpLabelSegment::Line([-2761, 1025]),
    HelpLabelSegment::Quad {
        control: [-2761, 1109],
        to: [-2711, 1135],
    },
    HelpLabelSegment::Line([-2711, 1192]),
    HelpLabelSegment::Quad {
        control: [-2779, 1192],
        to: [-2812, 1172],
    },
    HelpLabelSegment::Quad {
        control: [-2846, 1153],
        to: [-2861, 1109],
    },
    HelpLabelSegment::Quad {
        control: [-2928, 1192],
        to: [-3065, 1192],
    },
    HelpLabelSegment::Quad {
        control: [-3139, 1192],
        to: [-3193, 1139],
    },
    HelpLabelSegment::Quad {
        control: [-3248, 1085],
        to: [-3248, 1005],
    },
    HelpLabelSegment::Quad {
        control: [-3248, 909],
        to: [-3164, 844],
    },
    HelpLabelSegment::Quad {
        control: [-3081, 778],
        to: [-2952, 778],
    },
    HelpLabelSegment::Line([-2872, 793]),
    HelpLabelSegment::Quad {
        control: [-2872, 641],
        to: [-3008, 641],
    },
    HelpLabelSegment::Quad {
        control: [-3112, 641],
        to: [-3168, 697],
    },
    HelpLabelSegment::Line([-3215, 603]),
    HelpLabelSegment::Quad {
        control: [-3184, 578],
        to: [-3127, 560],
    },
    HelpLabelSegment::Quad {
        control: [-3071, 541],
        to: [-3021, 541],
    },
];

const BODY_CONTOUR_16: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [-3030, 860],
        to: [-3083, 903],
    },
    HelpLabelSegment::Quad {
        control: [-3137, 947],
        to: [-3137, 1007],
    },
    HelpLabelSegment::Quad {
        control: [-3137, 1104],
        to: [-3021, 1104],
    },
    HelpLabelSegment::Quad {
        control: [-2936, 1104],
        to: [-2872, 1024],
    },
    HelpLabelSegment::Line([-2872, 872]),
    HelpLabelSegment::Line([-2946, 860]),
];

const BODY_CONTOUR_17: [HelpLabelSegment; 22] = [
    HelpLabelSegment::Line([-2278, 688]),
    HelpLabelSegment::Quad {
        control: [-2344, 635],
        to: [-2411, 635],
    },
    HelpLabelSegment::Quad {
        control: [-2451, 635],
        to: [-2478, 654],
    },
    HelpLabelSegment::Quad {
        control: [-2506, 673],
        to: [-2506, 701],
    },
    HelpLabelSegment::Quad {
        control: [-2506, 762],
        to: [-2436, 792],
    },
    HelpLabelSegment::Line([-2357, 828]),
    HelpLabelSegment::Quad {
        control: [-2284, 862],
        to: [-2250, 905],
    },
    HelpLabelSegment::Quad {
        control: [-2217, 948],
        to: [-2217, 1012],
    },
    HelpLabelSegment::Quad {
        control: [-2217, 1097],
        to: [-2276, 1145],
    },
    HelpLabelSegment::Quad {
        control: [-2336, 1192],
        to: [-2440, 1192],
    },
    HelpLabelSegment::Quad {
        control: [-2540, 1192],
        to: [-2626, 1142],
    },
    HelpLabelSegment::Line([-2588, 1037]),
    HelpLabelSegment::Quad {
        control: [-2494, 1098],
        to: [-2438, 1098],
    },
    HelpLabelSegment::Quad {
        control: [-2335, 1098],
        to: [-2335, 1011],
    },
    HelpLabelSegment::Quad {
        control: [-2335, 949],
        to: [-2434, 905],
    },
    HelpLabelSegment::Quad {
        control: [-2510, 869],
        to: [-2537, 852],
    },
    HelpLabelSegment::Quad {
        control: [-2564, 833],
        to: [-2583, 811],
    },
    HelpLabelSegment::Quad {
        control: [-2603, 787],
        to: [-2612, 762],
    },
    HelpLabelSegment::Quad {
        control: [-2623, 735],
        to: [-2623, 705],
    },
    HelpLabelSegment::Quad {
        control: [-2623, 628],
        to: [-2567, 585],
    },
    HelpLabelSegment::Quad {
        control: [-2510, 541],
        to: [-2419, 541],
    },
    HelpLabelSegment::Quad {
        control: [-2351, 541],
        to: [-2247, 585],
    },
];

const BODY_CONTOUR_18: [HelpLabelSegment; 24] = [
    HelpLabelSegment::Quad {
        control: [-902, 656],
        to: [-902, 760],
    },
    HelpLabelSegment::Line([-902, 1180]),
    HelpLabelSegment::Line([-1014, 1180]),
    HelpLabelSegment::Line([-1014, 783]),
    HelpLabelSegment::Quad {
        control: [-1014, 635],
        to: [-1143, 635],
    },
    HelpLabelSegment::Quad {
        control: [-1183, 635],
        to: [-1218, 660],
    },
    HelpLabelSegment::Quad {
        control: [-1253, 684],
        to: [-1266, 716],
    },
    HelpLabelSegment::Line([-1266, 1180]),
    HelpLabelSegment::Line([-1377, 1180]),
    HelpLabelSegment::Line([-1377, 735]),
    HelpLabelSegment::Quad {
        control: [-1377, 688],
        to: [-1412, 662],
    },
    HelpLabelSegment::Quad {
        control: [-1447, 635],
        to: [-1505, 635],
    },
    HelpLabelSegment::Quad {
        control: [-1538, 635],
        to: [-1575, 661],
    },
    HelpLabelSegment::Quad {
        control: [-1614, 687],
        to: [-1629, 717],
    },
    HelpLabelSegment::Line([-1629, 1180]),
    HelpLabelSegment::Line([-1740, 1180]),
    HelpLabelSegment::Line([-1740, 553]),
    HelpLabelSegment::Line([-1668, 553]),
    HelpLabelSegment::Line([-1631, 626]),
    HelpLabelSegment::Quad {
        control: [-1567, 541],
        to: [-1470, 541],
    },
    HelpLabelSegment::Quad {
        control: [-1335, 541],
        to: [-1281, 625],
    },
    HelpLabelSegment::Quad {
        control: [-1262, 589],
        to: [-1212, 565],
    },
    HelpLabelSegment::Quad {
        control: [-1160, 541],
        to: [-1106, 541],
    },
    HelpLabelSegment::Quad {
        control: [-1009, 541],
        to: [-956, 599],
    },
];

const BODY_CONTOUR_19: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Quad {
        control: [-414, 541],
        to: [-352, 603],
    },
    HelpLabelSegment::Quad {
        control: [-291, 666],
        to: [-291, 800],
    },
    HelpLabelSegment::Line([-291, 1025]),
    HelpLabelSegment::Quad {
        control: [-291, 1109],
        to: [-241, 1135],
    },
    HelpLabelSegment::Line([-241, 1192]),
    HelpLabelSegment::Quad {
        control: [-309, 1192],
        to: [-342, 1172],
    },
    HelpLabelSegment::Quad {
        control: [-376, 1153],
        to: [-391, 1109],
    },
    HelpLabelSegment::Quad {
        control: [-458, 1192],
        to: [-595, 1192],
    },
    HelpLabelSegment::Quad {
        control: [-669, 1192],
        to: [-723, 1139],
    },
    HelpLabelSegment::Quad {
        control: [-778, 1085],
        to: [-778, 1005],
    },
    HelpLabelSegment::Quad {
        control: [-778, 909],
        to: [-694, 844],
    },
    HelpLabelSegment::Quad {
        control: [-611, 778],
        to: [-482, 778],
    },
    HelpLabelSegment::Line([-402, 793]),
    HelpLabelSegment::Quad {
        control: [-402, 641],
        to: [-538, 641],
    },
    HelpLabelSegment::Quad {
        control: [-642, 641],
        to: [-698, 697],
    },
    HelpLabelSegment::Line([-745, 603]),
    HelpLabelSegment::Quad {
        control: [-714, 578],
        to: [-657, 560],
    },
    HelpLabelSegment::Quad {
        control: [-601, 541],
        to: [-551, 541],
    },
];

const BODY_CONTOUR_20: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [-560, 860],
        to: [-613, 903],
    },
    HelpLabelSegment::Quad {
        control: [-667, 947],
        to: [-667, 1007],
    },
    HelpLabelSegment::Quad {
        control: [-667, 1104],
        to: [-551, 1104],
    },
    HelpLabelSegment::Quad {
        control: [-466, 1104],
        to: [-402, 1024],
    },
    HelpLabelSegment::Line([-402, 872]),
    HelpLabelSegment::Line([-476, 860]),
];

const BODY_CONTOUR_21: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Line([382, 1180]),
    HelpLabelSegment::Line([270, 1180]),
    HelpLabelSegment::Line([270, 816]),
    HelpLabelSegment::Quad {
        control: [270, 715],
        to: [241, 675],
    },
    HelpLabelSegment::Quad {
        control: [210, 635],
        to: [139, 635],
    },
    HelpLabelSegment::Quad {
        control: [101, 635],
        to: [59, 657],
    },
    HelpLabelSegment::Quad {
        control: [18, 681],
        to: [-4, 714],
    },
    HelpLabelSegment::Line([-4, 1180]),
    HelpLabelSegment::Line([-115, 1180]),
    HelpLabelSegment::Line([-115, 553]),
    HelpLabelSegment::Line([-39, 553]),
    HelpLabelSegment::Line([-4, 634]),
    HelpLabelSegment::Quad {
        control: [51, 541],
        to: [175, 541],
    },
    HelpLabelSegment::Quad {
        control: [382, 541],
        to: [382, 792],
    },
];

const BODY_CONTOUR_22: [HelpLabelSegment; 13] = [
    HelpLabelSegment::Line([779, 1287]),
    HelpLabelSegment::Quad {
        control: [758, 1346],
        to: [689, 1386],
    },
    HelpLabelSegment::Quad {
        control: [618, 1426],
        to: [533, 1426],
    },
    HelpLabelSegment::Line([533, 1326]),
    HelpLabelSegment::Quad {
        control: [603, 1326],
        to: [652, 1295],
    },
    HelpLabelSegment::Quad {
        control: [703, 1262],
        to: [703, 1215],
    },
    HelpLabelSegment::Quad {
        control: [703, 1164],
        to: [684, 1113],
    },
    HelpLabelSegment::Line([637, 989]),
    HelpLabelSegment::Line([467, 553]),
    HelpLabelSegment::Line([581, 553]),
    HelpLabelSegment::Line([766, 1038]),
    HelpLabelSegment::Line([931, 553]),
    HelpLabelSegment::Line([1045, 553]),
];

const BODY_CONTOUR_23: [HelpLabelSegment; 13] = [
    HelpLabelSegment::Line([1601, 599]),
    HelpLabelSegment::Quad {
        control: [1616, 578],
        to: [1661, 559],
    },
    HelpLabelSegment::Quad {
        control: [1704, 541],
        to: [1746, 541],
    },
    HelpLabelSegment::Quad {
        control: [1875, 541],
        to: [1955, 630],
    },
    HelpLabelSegment::Quad {
        control: [2035, 719],
        to: [2035, 855],
    },
    HelpLabelSegment::Quad {
        control: [2035, 1012],
        to: [1955, 1103],
    },
    HelpLabelSegment::Quad {
        control: [1874, 1192],
        to: [1737, 1192],
    },
    HelpLabelSegment::Quad {
        control: [1692, 1192],
        to: [1650, 1175],
    },
    HelpLabelSegment::Quad {
        control: [1607, 1159],
        to: [1585, 1135],
    },
    HelpLabelSegment::Line([1545, 1192]),
    HelpLabelSegment::Line([1490, 1192]),
    HelpLabelSegment::Line([1490, 295]),
    HelpLabelSegment::Line([1601, 295]),
];

const BODY_CONTOUR_24: [HelpLabelSegment; 9] = [
    HelpLabelSegment::Line([1601, 1046]),
    HelpLabelSegment::Quad {
        control: [1601, 1056],
        to: [1642, 1077],
    },
    HelpLabelSegment::Quad {
        control: [1684, 1098],
        to: [1705, 1098],
    },
    HelpLabelSegment::Quad {
        control: [1819, 1098],
        to: [1868, 1044],
    },
    HelpLabelSegment::Quad {
        control: [1917, 989],
        to: [1917, 861],
    },
    HelpLabelSegment::Quad {
        control: [1917, 755],
        to: [1860, 695],
    },
    HelpLabelSegment::Quad {
        control: [1803, 635],
        to: [1705, 635],
    },
    HelpLabelSegment::Quad {
        control: [1685, 635],
        to: [1649, 653],
    },
    HelpLabelSegment::Line([1601, 684]),
];

const BODY_CONTOUR_25: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Quad {
        control: [2491, 541],
        to: [2553, 603],
    },
    HelpLabelSegment::Quad {
        control: [2614, 666],
        to: [2614, 800],
    },
    HelpLabelSegment::Line([2614, 1025]),
    HelpLabelSegment::Quad {
        control: [2614, 1109],
        to: [2664, 1135],
    },
    HelpLabelSegment::Line([2664, 1192]),
    HelpLabelSegment::Quad {
        control: [2596, 1192],
        to: [2563, 1172],
    },
    HelpLabelSegment::Quad {
        control: [2529, 1153],
        to: [2514, 1109],
    },
    HelpLabelSegment::Quad {
        control: [2447, 1192],
        to: [2310, 1192],
    },
    HelpLabelSegment::Quad {
        control: [2236, 1192],
        to: [2182, 1139],
    },
    HelpLabelSegment::Quad {
        control: [2127, 1085],
        to: [2127, 1005],
    },
    HelpLabelSegment::Quad {
        control: [2127, 909],
        to: [2211, 844],
    },
    HelpLabelSegment::Quad {
        control: [2294, 778],
        to: [2423, 778],
    },
    HelpLabelSegment::Line([2503, 793]),
    HelpLabelSegment::Quad {
        control: [2503, 641],
        to: [2367, 641],
    },
    HelpLabelSegment::Quad {
        control: [2263, 641],
        to: [2207, 697],
    },
    HelpLabelSegment::Line([2160, 603]),
    HelpLabelSegment::Quad {
        control: [2191, 578],
        to: [2248, 560],
    },
    HelpLabelSegment::Quad {
        control: [2304, 541],
        to: [2354, 541],
    },
];

const BODY_CONTOUR_26: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [2345, 860],
        to: [2292, 903],
    },
    HelpLabelSegment::Quad {
        control: [2238, 947],
        to: [2238, 1007],
    },
    HelpLabelSegment::Quad {
        control: [2238, 1104],
        to: [2354, 1104],
    },
    HelpLabelSegment::Quad {
        control: [2439, 1104],
        to: [2503, 1024],
    },
    HelpLabelSegment::Line([2503, 872]),
    HelpLabelSegment::Line([2429, 860]),
];

const BODY_CONTOUR_27: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([3015, 1192]),
    HelpLabelSegment::Quad {
        control: [2798, 1192],
        to: [2798, 1003],
    },
    HelpLabelSegment::Line([2798, 295]),
    HelpLabelSegment::Line([2909, 295]),
    HelpLabelSegment::Line([2909, 984]),
    HelpLabelSegment::Quad {
        control: [2909, 1035],
        to: [2939, 1064],
    },
    HelpLabelSegment::Quad {
        control: [2968, 1092],
        to: [3015, 1092],
    },
];

const BODY_CONTOUR_28: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([3370, 1192]),
    HelpLabelSegment::Quad {
        control: [3153, 1192],
        to: [3153, 1003],
    },
    HelpLabelSegment::Line([3153, 295]),
    HelpLabelSegment::Line([3264, 295]),
    HelpLabelSegment::Line([3264, 984]),
    HelpLabelSegment::Quad {
        control: [3264, 1035],
        to: [3294, 1064],
    },
    HelpLabelSegment::Quad {
        control: [3323, 1092],
        to: [3370, 1092],
    },
];

const BODY_CONTOUR_29: [HelpLabelSegment; 22] = [
    HelpLabelSegment::Line([3807, 688]),
    HelpLabelSegment::Quad {
        control: [3741, 635],
        to: [3674, 635],
    },
    HelpLabelSegment::Quad {
        control: [3634, 635],
        to: [3608, 654],
    },
    HelpLabelSegment::Quad {
        control: [3579, 673],
        to: [3579, 701],
    },
    HelpLabelSegment::Quad {
        control: [3579, 762],
        to: [3649, 792],
    },
    HelpLabelSegment::Line([3728, 828]),
    HelpLabelSegment::Quad {
        control: [3801, 862],
        to: [3835, 905],
    },
    HelpLabelSegment::Quad {
        control: [3868, 948],
        to: [3868, 1012],
    },
    HelpLabelSegment::Quad {
        control: [3868, 1097],
        to: [3809, 1145],
    },
    HelpLabelSegment::Quad {
        control: [3749, 1192],
        to: [3645, 1192],
    },
    HelpLabelSegment::Quad {
        control: [3545, 1192],
        to: [3459, 1142],
    },
    HelpLabelSegment::Line([3497, 1037]),
    HelpLabelSegment::Quad {
        control: [3591, 1098],
        to: [3647, 1098],
    },
    HelpLabelSegment::Quad {
        control: [3750, 1098],
        to: [3750, 1011],
    },
    HelpLabelSegment::Quad {
        control: [3750, 949],
        to: [3651, 905],
    },
    HelpLabelSegment::Quad {
        control: [3575, 869],
        to: [3548, 852],
    },
    HelpLabelSegment::Quad {
        control: [3521, 833],
        to: [3502, 811],
    },
    HelpLabelSegment::Quad {
        control: [3482, 787],
        to: [3473, 762],
    },
    HelpLabelSegment::Quad {
        control: [3462, 735],
        to: [3462, 705],
    },
    HelpLabelSegment::Quad {
        control: [3462, 628],
        to: [3518, 585],
    },
    HelpLabelSegment::Quad {
        control: [3575, 541],
        to: [3666, 541],
    },
    HelpLabelSegment::Quad {
        control: [3734, 541],
        to: [3838, 585],
    },
];

const BODY_CONTOUR_30: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Quad {
        control: [4676, 541],
        to: [4738, 603],
    },
    HelpLabelSegment::Quad {
        control: [4799, 666],
        to: [4799, 800],
    },
    HelpLabelSegment::Line([4799, 1025]),
    HelpLabelSegment::Quad {
        control: [4799, 1109],
        to: [4849, 1135],
    },
    HelpLabelSegment::Line([4849, 1192]),
    HelpLabelSegment::Quad {
        control: [4781, 1192],
        to: [4748, 1172],
    },
    HelpLabelSegment::Quad {
        control: [4714, 1153],
        to: [4699, 1109],
    },
    HelpLabelSegment::Quad {
        control: [4632, 1192],
        to: [4495, 1192],
    },
    HelpLabelSegment::Quad {
        control: [4421, 1192],
        to: [4367, 1139],
    },
    HelpLabelSegment::Quad {
        control: [4312, 1085],
        to: [4312, 1005],
    },
    HelpLabelSegment::Quad {
        control: [4312, 909],
        to: [4396, 844],
    },
    HelpLabelSegment::Quad {
        control: [4479, 778],
        to: [4608, 778],
    },
    HelpLabelSegment::Line([4688, 793]),
    HelpLabelSegment::Quad {
        control: [4688, 641],
        to: [4552, 641],
    },
    HelpLabelSegment::Quad {
        control: [4448, 641],
        to: [4392, 697],
    },
    HelpLabelSegment::Line([4345, 603]),
    HelpLabelSegment::Quad {
        control: [4376, 578],
        to: [4433, 560],
    },
    HelpLabelSegment::Quad {
        control: [4489, 541],
        to: [4539, 541],
    },
];

const BODY_CONTOUR_31: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [4530, 860],
        to: [4477, 903],
    },
    HelpLabelSegment::Quad {
        control: [4423, 947],
        to: [4423, 1007],
    },
    HelpLabelSegment::Quad {
        control: [4423, 1104],
        to: [4539, 1104],
    },
    HelpLabelSegment::Quad {
        control: [4624, 1104],
        to: [4688, 1024],
    },
    HelpLabelSegment::Line([4688, 872]),
    HelpLabelSegment::Line([4614, 860]),
];

const BODY_CONTOUR_32: [HelpLabelSegment; 22] = [
    HelpLabelSegment::Line([5282, 688]),
    HelpLabelSegment::Quad {
        control: [5216, 635],
        to: [5149, 635],
    },
    HelpLabelSegment::Quad {
        control: [5109, 635],
        to: [5082, 654],
    },
    HelpLabelSegment::Quad {
        control: [5054, 673],
        to: [5054, 701],
    },
    HelpLabelSegment::Quad {
        control: [5054, 762],
        to: [5124, 792],
    },
    HelpLabelSegment::Line([5203, 828]),
    HelpLabelSegment::Quad {
        control: [5276, 862],
        to: [5310, 905],
    },
    HelpLabelSegment::Quad {
        control: [5343, 948],
        to: [5343, 1012],
    },
    HelpLabelSegment::Quad {
        control: [5343, 1097],
        to: [5284, 1145],
    },
    HelpLabelSegment::Quad {
        control: [5224, 1192],
        to: [5120, 1192],
    },
    HelpLabelSegment::Quad {
        control: [5020, 1192],
        to: [4934, 1142],
    },
    HelpLabelSegment::Line([4972, 1037]),
    HelpLabelSegment::Quad {
        control: [5066, 1098],
        to: [5122, 1098],
    },
    HelpLabelSegment::Quad {
        control: [5225, 1098],
        to: [5225, 1011],
    },
    HelpLabelSegment::Quad {
        control: [5225, 949],
        to: [5126, 905],
    },
    HelpLabelSegment::Quad {
        control: [5050, 869],
        to: [5023, 852],
    },
    HelpLabelSegment::Quad {
        control: [4996, 833],
        to: [4977, 811],
    },
    HelpLabelSegment::Quad {
        control: [4957, 787],
        to: [4948, 762],
    },
    HelpLabelSegment::Quad {
        control: [4937, 735],
        to: [4937, 705],
    },
    HelpLabelSegment::Quad {
        control: [4937, 628],
        to: [4993, 585],
    },
    HelpLabelSegment::Quad {
        control: [5050, 541],
        to: [5141, 541],
    },
    HelpLabelSegment::Quad {
        control: [5209, 541],
        to: [5313, 585],
    },
];

const BODY_CONTOUR_33: [HelpLabelSegment; 12] = [
    HelpLabelSegment::Line([5931, 605]),
    HelpLabelSegment::Quad {
        control: [5994, 541],
        to: [6083, 541],
    },
    HelpLabelSegment::Quad {
        control: [6217, 541],
        to: [6292, 625],
    },
    HelpLabelSegment::Quad {
        control: [6366, 708],
        to: [6366, 868],
    },
    HelpLabelSegment::Quad {
        control: [6366, 1011],
        to: [6291, 1101],
    },
    HelpLabelSegment::Quad {
        control: [6216, 1192],
        to: [6074, 1192],
    },
    HelpLabelSegment::Quad {
        control: [6034, 1192],
        to: [5990, 1178],
    },
    HelpLabelSegment::Quad {
        control: [5944, 1164],
        to: [5931, 1146],
    },
    HelpLabelSegment::Line([5931, 1426]),
    HelpLabelSegment::Line([5820, 1426]),
    HelpLabelSegment::Line([5820, 553]),
    HelpLabelSegment::Line([5931, 553]),
];

const BODY_CONTOUR_34: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Line([5931, 1053]),
    HelpLabelSegment::Quad {
        control: [5942, 1070],
        to: [5976, 1084],
    },
    HelpLabelSegment::Quad {
        control: [6010, 1098],
        to: [6041, 1098],
    },
    HelpLabelSegment::Quad {
        control: [6249, 1098],
        to: [6249, 864],
    },
    HelpLabelSegment::Quad {
        control: [6249, 745],
        to: [6199, 690],
    },
    HelpLabelSegment::Quad {
        control: [6150, 635],
        to: [6042, 635],
    },
    HelpLabelSegment::Quad {
        control: [6019, 635],
        to: [5985, 651],
    },
    HelpLabelSegment::Line([5931, 688]),
];

const BODY_CONTOUR_35: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [6566, 1101],
        to: [6732, 1101],
    },
    HelpLabelSegment::Quad {
        control: [6811, 1101],
        to: [6855, 1038],
    },
    HelpLabelSegment::Quad {
        control: [6899, 975],
        to: [6899, 865],
    },
    HelpLabelSegment::Quad {
        control: [6899, 632],
        to: [6732, 632],
    },
    HelpLabelSegment::Quad {
        control: [6656, 632],
        to: [6612, 694],
    },
    HelpLabelSegment::Quad {
        control: [6566, 756],
        to: [6566, 865],
    },
];

const BODY_CONTOUR_36: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [6605, 541],
        to: [6732, 541],
    },
    HelpLabelSegment::Quad {
        control: [6867, 541],
        to: [6942, 627],
    },
    HelpLabelSegment::Quad {
        control: [7016, 712],
        to: [7016, 865],
    },
    HelpLabelSegment::Quad {
        control: [7016, 1017],
        to: [6940, 1105],
    },
    HelpLabelSegment::Quad {
        control: [6864, 1192],
        to: [6732, 1192],
    },
    HelpLabelSegment::Quad {
        control: [6599, 1192],
        to: [6524, 1104],
    },
    HelpLabelSegment::Quad {
        control: [6449, 1015],
        to: [6449, 865],
    },
    HelpLabelSegment::Quad {
        control: [6449, 719],
        to: [6527, 630],
    },
];

const BODY_CONTOUR_37: [HelpLabelSegment; 22] = [
    HelpLabelSegment::Line([7442, 688]),
    HelpLabelSegment::Quad {
        control: [7376, 635],
        to: [7309, 635],
    },
    HelpLabelSegment::Quad {
        control: [7269, 635],
        to: [7242, 654],
    },
    HelpLabelSegment::Quad {
        control: [7214, 673],
        to: [7214, 701],
    },
    HelpLabelSegment::Quad {
        control: [7214, 762],
        to: [7284, 792],
    },
    HelpLabelSegment::Line([7363, 828]),
    HelpLabelSegment::Quad {
        control: [7436, 862],
        to: [7470, 905],
    },
    HelpLabelSegment::Quad {
        control: [7503, 948],
        to: [7503, 1012],
    },
    HelpLabelSegment::Quad {
        control: [7503, 1097],
        to: [7444, 1145],
    },
    HelpLabelSegment::Quad {
        control: [7384, 1192],
        to: [7280, 1192],
    },
    HelpLabelSegment::Quad {
        control: [7180, 1192],
        to: [7094, 1142],
    },
    HelpLabelSegment::Line([7132, 1037]),
    HelpLabelSegment::Quad {
        control: [7226, 1098],
        to: [7282, 1098],
    },
    HelpLabelSegment::Quad {
        control: [7385, 1098],
        to: [7385, 1011],
    },
    HelpLabelSegment::Quad {
        control: [7385, 949],
        to: [7286, 905],
    },
    HelpLabelSegment::Quad {
        control: [7210, 869],
        to: [7183, 852],
    },
    HelpLabelSegment::Quad {
        control: [7156, 833],
        to: [7137, 811],
    },
    HelpLabelSegment::Quad {
        control: [7117, 787],
        to: [7108, 762],
    },
    HelpLabelSegment::Quad {
        control: [7097, 735],
        to: [7097, 705],
    },
    HelpLabelSegment::Quad {
        control: [7097, 628],
        to: [7153, 585],
    },
    HelpLabelSegment::Quad {
        control: [7210, 541],
        to: [7301, 541],
    },
    HelpLabelSegment::Quad {
        control: [7369, 541],
        to: [7473, 585],
    },
];

const BODY_CONTOUR_38: [HelpLabelSegment; 22] = [
    HelpLabelSegment::Line([7927, 688]),
    HelpLabelSegment::Quad {
        control: [7861, 635],
        to: [7794, 635],
    },
    HelpLabelSegment::Quad {
        control: [7754, 635],
        to: [7727, 654],
    },
    HelpLabelSegment::Quad {
        control: [7699, 673],
        to: [7699, 701],
    },
    HelpLabelSegment::Quad {
        control: [7699, 762],
        to: [7769, 792],
    },
    HelpLabelSegment::Line([7848, 828]),
    HelpLabelSegment::Quad {
        control: [7921, 862],
        to: [7955, 905],
    },
    HelpLabelSegment::Quad {
        control: [7988, 948],
        to: [7988, 1012],
    },
    HelpLabelSegment::Quad {
        control: [7988, 1097],
        to: [7929, 1145],
    },
    HelpLabelSegment::Quad {
        control: [7869, 1192],
        to: [7765, 1192],
    },
    HelpLabelSegment::Quad {
        control: [7665, 1192],
        to: [7579, 1142],
    },
    HelpLabelSegment::Line([7617, 1037]),
    HelpLabelSegment::Quad {
        control: [7711, 1098],
        to: [7767, 1098],
    },
    HelpLabelSegment::Quad {
        control: [7870, 1098],
        to: [7870, 1011],
    },
    HelpLabelSegment::Quad {
        control: [7870, 949],
        to: [7771, 905],
    },
    HelpLabelSegment::Quad {
        control: [7695, 869],
        to: [7668, 852],
    },
    HelpLabelSegment::Quad {
        control: [7641, 833],
        to: [7622, 811],
    },
    HelpLabelSegment::Quad {
        control: [7602, 787],
        to: [7593, 762],
    },
    HelpLabelSegment::Quad {
        control: [7582, 735],
        to: [7582, 705],
    },
    HelpLabelSegment::Quad {
        control: [7582, 628],
        to: [7638, 585],
    },
    HelpLabelSegment::Quad {
        control: [7695, 541],
        to: [7786, 541],
    },
    HelpLabelSegment::Quad {
        control: [7854, 541],
        to: [7958, 585],
    },
];

const BODY_CONTOUR_39: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [8248, 315],
        to: [8269, 336],
    },
    HelpLabelSegment::Quad {
        control: [8289, 356],
        to: [8289, 384],
    },
    HelpLabelSegment::Quad {
        control: [8289, 412],
        to: [8269, 434],
    },
    HelpLabelSegment::Quad {
        control: [8248, 453],
        to: [8220, 453],
    },
    HelpLabelSegment::Quad {
        control: [8191, 453],
        to: [8171, 434],
    },
    HelpLabelSegment::Quad {
        control: [8150, 412],
        to: [8150, 384],
    },
    HelpLabelSegment::Quad {
        control: [8150, 355],
        to: [8170, 335],
    },
    HelpLabelSegment::Quad {
        control: [8190, 315],
        to: [8220, 315],
    },
];

const BODY_CONTOUR_40: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Line([8270, 1180]),
    HelpLabelSegment::Line([8159, 1180]),
    HelpLabelSegment::Line([8159, 647]),
    HelpLabelSegment::Line([8072, 647]),
    HelpLabelSegment::Line([8072, 553]),
    HelpLabelSegment::Line([8270, 553]),
];

const BODY_CONTOUR_41: [HelpLabelSegment; 13] = [
    HelpLabelSegment::Line([8556, 599]),
    HelpLabelSegment::Quad {
        control: [8571, 578],
        to: [8616, 559],
    },
    HelpLabelSegment::Quad {
        control: [8659, 541],
        to: [8701, 541],
    },
    HelpLabelSegment::Quad {
        control: [8830, 541],
        to: [8910, 630],
    },
    HelpLabelSegment::Quad {
        control: [8990, 719],
        to: [8990, 855],
    },
    HelpLabelSegment::Quad {
        control: [8990, 1012],
        to: [8910, 1103],
    },
    HelpLabelSegment::Quad {
        control: [8829, 1192],
        to: [8692, 1192],
    },
    HelpLabelSegment::Quad {
        control: [8647, 1192],
        to: [8605, 1175],
    },
    HelpLabelSegment::Quad {
        control: [8562, 1159],
        to: [8540, 1135],
    },
    HelpLabelSegment::Line([8500, 1192]),
    HelpLabelSegment::Line([8445, 1192]),
    HelpLabelSegment::Line([8445, 295]),
    HelpLabelSegment::Line([8556, 295]),
];

const BODY_CONTOUR_42: [HelpLabelSegment; 9] = [
    HelpLabelSegment::Line([8556, 1046]),
    HelpLabelSegment::Quad {
        control: [8556, 1056],
        to: [8597, 1077],
    },
    HelpLabelSegment::Quad {
        control: [8639, 1098],
        to: [8660, 1098],
    },
    HelpLabelSegment::Quad {
        control: [8774, 1098],
        to: [8823, 1044],
    },
    HelpLabelSegment::Quad {
        control: [8872, 989],
        to: [8872, 861],
    },
    HelpLabelSegment::Quad {
        control: [8872, 755],
        to: [8815, 695],
    },
    HelpLabelSegment::Quad {
        control: [8758, 635],
        to: [8660, 635],
    },
    HelpLabelSegment::Quad {
        control: [8640, 635],
        to: [8604, 653],
    },
    HelpLabelSegment::Line([8556, 684]),
];

const BODY_CONTOUR_43: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([9340, 1192]),
    HelpLabelSegment::Quad {
        control: [9123, 1192],
        to: [9123, 1003],
    },
    HelpLabelSegment::Line([9123, 295]),
    HelpLabelSegment::Line([9234, 295]),
    HelpLabelSegment::Line([9234, 984]),
    HelpLabelSegment::Quad {
        control: [9234, 1035],
        to: [9264, 1064],
    },
    HelpLabelSegment::Quad {
        control: [9293, 1092],
        to: [9340, 1092],
    },
];

const BODY_CONTOUR_44: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [9654, 635],
        to: [9603, 683],
    },
    HelpLabelSegment::Quad {
        control: [9555, 729],
        to: [9548, 797],
    },
    HelpLabelSegment::Line([9896, 797]),
    HelpLabelSegment::Quad {
        control: [9896, 729],
        to: [9854, 684],
    },
    HelpLabelSegment::Quad {
        control: [9807, 635],
        to: [9727, 635],
    },
];

const BODY_CONTOUR_45: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [9660, 1098],
        to: [9743, 1098],
    },
    HelpLabelSegment::Quad {
        control: [9839, 1098],
        to: [9902, 1043],
    },
    HelpLabelSegment::Line([9949, 1123]),
    HelpLabelSegment::Quad {
        control: [9923, 1148],
        to: [9870, 1167],
    },
    HelpLabelSegment::Quad {
        control: [9804, 1192],
        to: [9722, 1192],
    },
    HelpLabelSegment::Quad {
        control: [9603, 1192],
        to: [9520, 1112],
    },
    HelpLabelSegment::Quad {
        control: [9429, 1023],
        to: [9429, 874],
    },
    HelpLabelSegment::Quad {
        control: [9429, 718],
        to: [9522, 625],
    },
    HelpLabelSegment::Quad {
        control: [9607, 541],
        to: [9723, 541],
    },
    HelpLabelSegment::Quad {
        control: [9856, 541],
        to: [9933, 616],
    },
    HelpLabelSegment::Quad {
        control: [10006, 689],
        to: [10006, 810],
    },
    HelpLabelSegment::Quad {
        control: [10006, 846],
        to: [9998, 878],
    },
    HelpLabelSegment::Line([9546, 878]),
    HelpLabelSegment::Quad {
        control: [9546, 988],
        to: [9606, 1046],
    },
];

const BODY_CONTOUR_46: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [10628, 315],
        to: [10649, 336],
    },
    HelpLabelSegment::Quad {
        control: [10669, 356],
        to: [10669, 384],
    },
    HelpLabelSegment::Quad {
        control: [10669, 412],
        to: [10649, 434],
    },
    HelpLabelSegment::Quad {
        control: [10628, 453],
        to: [10600, 453],
    },
    HelpLabelSegment::Quad {
        control: [10571, 453],
        to: [10551, 434],
    },
    HelpLabelSegment::Quad {
        control: [10530, 412],
        to: [10530, 384],
    },
    HelpLabelSegment::Quad {
        control: [10530, 355],
        to: [10550, 335],
    },
    HelpLabelSegment::Quad {
        control: [10570, 315],
        to: [10600, 315],
    },
];

const BODY_CONTOUR_47: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Line([10650, 1180]),
    HelpLabelSegment::Line([10539, 1180]),
    HelpLabelSegment::Line([10539, 647]),
    HelpLabelSegment::Line([10452, 647]),
    HelpLabelSegment::Line([10452, 553]),
    HelpLabelSegment::Line([10650, 553]),
];

const BODY_CONTOUR_48: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Line([11322, 1180]),
    HelpLabelSegment::Line([11210, 1180]),
    HelpLabelSegment::Line([11210, 816]),
    HelpLabelSegment::Quad {
        control: [11210, 715],
        to: [11181, 675],
    },
    HelpLabelSegment::Quad {
        control: [11150, 635],
        to: [11079, 635],
    },
    HelpLabelSegment::Quad {
        control: [11041, 635],
        to: [10999, 657],
    },
    HelpLabelSegment::Quad {
        control: [10958, 681],
        to: [10936, 714],
    },
    HelpLabelSegment::Line([10936, 1180]),
    HelpLabelSegment::Line([10825, 1180]),
    HelpLabelSegment::Line([10825, 553]),
    HelpLabelSegment::Line([10901, 553]),
    HelpLabelSegment::Line([10936, 634]),
    HelpLabelSegment::Quad {
        control: [10991, 541],
        to: [11115, 541],
    },
    HelpLabelSegment::Quad {
        control: [11322, 541],
        to: [11322, 792],
    },
];

const BODY_CONTOUR_49: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Line([11520, 422]),
    HelpLabelSegment::Line([11631, 378]),
    HelpLabelSegment::Line([11631, 553]),
    HelpLabelSegment::Line([11803, 553]),
    HelpLabelSegment::Line([11803, 641]),
    HelpLabelSegment::Line([11631, 641]),
    HelpLabelSegment::Line([11631, 953]),
    HelpLabelSegment::Quad {
        control: [11631, 1031],
        to: [11658, 1065],
    },
    HelpLabelSegment::Quad {
        control: [11684, 1098],
        to: [11743, 1098],
    },
    HelpLabelSegment::Quad {
        control: [11786, 1098],
        to: [11831, 1077],
    },
    HelpLabelSegment::Line([11848, 1174]),
    HelpLabelSegment::Line([11696, 1192]),
    HelpLabelSegment::Quad {
        control: [11621, 1192],
        to: [11571, 1137],
    },
    HelpLabelSegment::Quad {
        control: [11520, 1082],
        to: [11520, 997],
    },
    HelpLabelSegment::Line([11520, 641]),
    HelpLabelSegment::Line([11447, 641]),
    HelpLabelSegment::Line([11447, 553]),
    HelpLabelSegment::Line([11520, 553]),
];

const BODY_CONTOUR_50: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [12031, 1101],
        to: [12197, 1101],
    },
    HelpLabelSegment::Quad {
        control: [12276, 1101],
        to: [12320, 1038],
    },
    HelpLabelSegment::Quad {
        control: [12364, 975],
        to: [12364, 865],
    },
    HelpLabelSegment::Quad {
        control: [12364, 632],
        to: [12197, 632],
    },
    HelpLabelSegment::Quad {
        control: [12121, 632],
        to: [12077, 694],
    },
    HelpLabelSegment::Quad {
        control: [12031, 756],
        to: [12031, 865],
    },
];

const BODY_CONTOUR_51: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [12070, 541],
        to: [12197, 541],
    },
    HelpLabelSegment::Quad {
        control: [12332, 541],
        to: [12407, 627],
    },
    HelpLabelSegment::Quad {
        control: [12481, 712],
        to: [12481, 865],
    },
    HelpLabelSegment::Quad {
        control: [12481, 1017],
        to: [12405, 1105],
    },
    HelpLabelSegment::Quad {
        control: [12329, 1192],
        to: [12197, 1192],
    },
    HelpLabelSegment::Quad {
        control: [12064, 1192],
        to: [11989, 1104],
    },
    HelpLabelSegment::Quad {
        control: [11914, 1015],
        to: [11914, 865],
    },
    HelpLabelSegment::Quad {
        control: [11914, 719],
        to: [11992, 630],
    },
];

const BODY_CONTOUR_52: [HelpLabelSegment; 13] = [
    HelpLabelSegment::Line([13199, 1287]),
    HelpLabelSegment::Quad {
        control: [13178, 1346],
        to: [13109, 1386],
    },
    HelpLabelSegment::Quad {
        control: [13038, 1426],
        to: [12953, 1426],
    },
    HelpLabelSegment::Line([12953, 1326]),
    HelpLabelSegment::Quad {
        control: [13023, 1326],
        to: [13072, 1295],
    },
    HelpLabelSegment::Quad {
        control: [13123, 1262],
        to: [13123, 1215],
    },
    HelpLabelSegment::Quad {
        control: [13123, 1164],
        to: [13104, 1113],
    },
    HelpLabelSegment::Line([13057, 989]),
    HelpLabelSegment::Line([12887, 553]),
    HelpLabelSegment::Line([13001, 553]),
    HelpLabelSegment::Line([13186, 1038]),
    HelpLabelSegment::Line([13351, 553]),
    HelpLabelSegment::Line([13465, 553]),
];

const BODY_CONTOUR_53: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [13626, 1101],
        to: [13792, 1101],
    },
    HelpLabelSegment::Quad {
        control: [13871, 1101],
        to: [13915, 1038],
    },
    HelpLabelSegment::Quad {
        control: [13959, 975],
        to: [13959, 865],
    },
    HelpLabelSegment::Quad {
        control: [13959, 632],
        to: [13792, 632],
    },
    HelpLabelSegment::Quad {
        control: [13716, 632],
        to: [13672, 694],
    },
    HelpLabelSegment::Quad {
        control: [13626, 756],
        to: [13626, 865],
    },
];

const BODY_CONTOUR_54: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [13665, 541],
        to: [13792, 541],
    },
    HelpLabelSegment::Quad {
        control: [13927, 541],
        to: [14002, 627],
    },
    HelpLabelSegment::Quad {
        control: [14076, 712],
        to: [14076, 865],
    },
    HelpLabelSegment::Quad {
        control: [14076, 1017],
        to: [14000, 1105],
    },
    HelpLabelSegment::Quad {
        control: [13924, 1192],
        to: [13792, 1192],
    },
    HelpLabelSegment::Quad {
        control: [13659, 1192],
        to: [13584, 1104],
    },
    HelpLabelSegment::Quad {
        control: [13509, 1015],
        to: [13509, 865],
    },
    HelpLabelSegment::Quad {
        control: [13509, 719],
        to: [13587, 630],
    },
];

const BODY_CONTOUR_55: [HelpLabelSegment; 15] = [
    HelpLabelSegment::Line([14300, 953]),
    HelpLabelSegment::Quad {
        control: [14300, 1098],
        to: [14426, 1098],
    },
    HelpLabelSegment::Quad {
        control: [14481, 1098],
        to: [14526, 1066],
    },
    HelpLabelSegment::Quad {
        control: [14572, 1035],
        to: [14587, 994],
    },
    HelpLabelSegment::Line([14587, 553]),
    HelpLabelSegment::Line([14699, 553]),
    HelpLabelSegment::Line([14699, 1180]),
    HelpLabelSegment::Line([14587, 1180]),
    HelpLabelSegment::Line([14587, 1093]),
    HelpLabelSegment::Quad {
        control: [14569, 1131],
        to: [14512, 1161],
    },
    HelpLabelSegment::Quad {
        control: [14455, 1192],
        to: [14401, 1192],
    },
    HelpLabelSegment::Quad {
        control: [14298, 1192],
        to: [14244, 1133],
    },
    HelpLabelSegment::Quad {
        control: [14189, 1073],
        to: [14189, 964],
    },
    HelpLabelSegment::Line([14189, 553]),
    HelpLabelSegment::Line([14300, 553]),
];

const BODY_CONTOUR_56: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Quad {
        control: [15154, 635],
        to: [15117, 635],
    },
    HelpLabelSegment::Quad {
        control: [15058, 635],
        to: [15014, 689],
    },
    HelpLabelSegment::Quad {
        control: [14969, 744],
        to: [14969, 820],
    },
    HelpLabelSegment::Line([14969, 1180]),
    HelpLabelSegment::Line([14858, 1180]),
    HelpLabelSegment::Line([14858, 553]),
    HelpLabelSegment::Line([14969, 553]),
    HelpLabelSegment::Line([14969, 653]),
    HelpLabelSegment::Quad {
        control: [15030, 541],
        to: [15151, 541],
    },
    HelpLabelSegment::Line([15236, 552]),
    HelpLabelSegment::Line([15191, 660]),
];

const BODY_CONTOUR_57: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [15751, 1101],
        to: [15917, 1101],
    },
    HelpLabelSegment::Quad {
        control: [15996, 1101],
        to: [16040, 1038],
    },
    HelpLabelSegment::Quad {
        control: [16084, 975],
        to: [16084, 865],
    },
    HelpLabelSegment::Quad {
        control: [16084, 632],
        to: [15917, 632],
    },
    HelpLabelSegment::Quad {
        control: [15841, 632],
        to: [15797, 694],
    },
    HelpLabelSegment::Quad {
        control: [15751, 756],
        to: [15751, 865],
    },
];

const BODY_CONTOUR_58: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [15790, 541],
        to: [15917, 541],
    },
    HelpLabelSegment::Quad {
        control: [16052, 541],
        to: [16127, 627],
    },
    HelpLabelSegment::Quad {
        control: [16201, 712],
        to: [16201, 865],
    },
    HelpLabelSegment::Quad {
        control: [16201, 1017],
        to: [16125, 1105],
    },
    HelpLabelSegment::Quad {
        control: [16049, 1192],
        to: [15917, 1192],
    },
    HelpLabelSegment::Quad {
        control: [15784, 1192],
        to: [15709, 1104],
    },
    HelpLabelSegment::Quad {
        control: [15634, 1015],
        to: [15634, 865],
    },
    HelpLabelSegment::Quad {
        control: [15634, 719],
        to: [15712, 630],
    },
];

const BODY_CONTOUR_59: [HelpLabelSegment; 12] = [
    HelpLabelSegment::Line([16431, 605]),
    HelpLabelSegment::Quad {
        control: [16494, 541],
        to: [16583, 541],
    },
    HelpLabelSegment::Quad {
        control: [16717, 541],
        to: [16792, 625],
    },
    HelpLabelSegment::Quad {
        control: [16866, 708],
        to: [16866, 868],
    },
    HelpLabelSegment::Quad {
        control: [16866, 1011],
        to: [16791, 1101],
    },
    HelpLabelSegment::Quad {
        control: [16716, 1192],
        to: [16574, 1192],
    },
    HelpLabelSegment::Quad {
        control: [16534, 1192],
        to: [16490, 1178],
    },
    HelpLabelSegment::Quad {
        control: [16444, 1164],
        to: [16431, 1146],
    },
    HelpLabelSegment::Line([16431, 1426]),
    HelpLabelSegment::Line([16320, 1426]),
    HelpLabelSegment::Line([16320, 553]),
    HelpLabelSegment::Line([16431, 553]),
];

const BODY_CONTOUR_60: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Line([16431, 1053]),
    HelpLabelSegment::Quad {
        control: [16442, 1070],
        to: [16476, 1084],
    },
    HelpLabelSegment::Quad {
        control: [16510, 1098],
        to: [16541, 1098],
    },
    HelpLabelSegment::Quad {
        control: [16749, 1098],
        to: [16749, 864],
    },
    HelpLabelSegment::Quad {
        control: [16749, 745],
        to: [16699, 690],
    },
    HelpLabelSegment::Quad {
        control: [16650, 635],
        to: [16542, 635],
    },
    HelpLabelSegment::Quad {
        control: [16519, 635],
        to: [16485, 651],
    },
    HelpLabelSegment::Line([16431, 688]),
];

const BODY_CONTOUR_61: [HelpLabelSegment; 12] = [
    HelpLabelSegment::Line([17101, 605]),
    HelpLabelSegment::Quad {
        control: [17164, 541],
        to: [17253, 541],
    },
    HelpLabelSegment::Quad {
        control: [17387, 541],
        to: [17462, 625],
    },
    HelpLabelSegment::Quad {
        control: [17536, 708],
        to: [17536, 868],
    },
    HelpLabelSegment::Quad {
        control: [17536, 1011],
        to: [17461, 1101],
    },
    HelpLabelSegment::Quad {
        control: [17386, 1192],
        to: [17244, 1192],
    },
    HelpLabelSegment::Quad {
        control: [17204, 1192],
        to: [17160, 1178],
    },
    HelpLabelSegment::Quad {
        control: [17114, 1164],
        to: [17101, 1146],
    },
    HelpLabelSegment::Line([17101, 1426]),
    HelpLabelSegment::Line([16990, 1426]),
    HelpLabelSegment::Line([16990, 553]),
    HelpLabelSegment::Line([17101, 553]),
];

const BODY_CONTOUR_62: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Line([17101, 1053]),
    HelpLabelSegment::Quad {
        control: [17112, 1070],
        to: [17146, 1084],
    },
    HelpLabelSegment::Quad {
        control: [17180, 1098],
        to: [17211, 1098],
    },
    HelpLabelSegment::Quad {
        control: [17419, 1098],
        to: [17419, 864],
    },
    HelpLabelSegment::Quad {
        control: [17419, 745],
        to: [17369, 690],
    },
    HelpLabelSegment::Quad {
        control: [17320, 635],
        to: [17212, 635],
    },
    HelpLabelSegment::Quad {
        control: [17189, 635],
        to: [17155, 651],
    },
    HelpLabelSegment::Line([17101, 688]),
];

const BODY_CONTOUR_63: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [17736, 1101],
        to: [17902, 1101],
    },
    HelpLabelSegment::Quad {
        control: [17981, 1101],
        to: [18025, 1038],
    },
    HelpLabelSegment::Quad {
        control: [18069, 975],
        to: [18069, 865],
    },
    HelpLabelSegment::Quad {
        control: [18069, 632],
        to: [17902, 632],
    },
    HelpLabelSegment::Quad {
        control: [17826, 632],
        to: [17782, 694],
    },
    HelpLabelSegment::Quad {
        control: [17736, 756],
        to: [17736, 865],
    },
];

const BODY_CONTOUR_64: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [17775, 541],
        to: [17902, 541],
    },
    HelpLabelSegment::Quad {
        control: [18037, 541],
        to: [18112, 627],
    },
    HelpLabelSegment::Quad {
        control: [18186, 712],
        to: [18186, 865],
    },
    HelpLabelSegment::Quad {
        control: [18186, 1017],
        to: [18110, 1105],
    },
    HelpLabelSegment::Quad {
        control: [18034, 1192],
        to: [17902, 1192],
    },
    HelpLabelSegment::Quad {
        control: [17769, 1192],
        to: [17694, 1104],
    },
    HelpLabelSegment::Quad {
        control: [17619, 1015],
        to: [17619, 865],
    },
    HelpLabelSegment::Quad {
        control: [17619, 719],
        to: [17697, 630],
    },
];

const BODY_CONTOUR_65: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Line([18802, 1180]),
    HelpLabelSegment::Line([18690, 1180]),
    HelpLabelSegment::Line([18690, 816]),
    HelpLabelSegment::Quad {
        control: [18690, 715],
        to: [18661, 675],
    },
    HelpLabelSegment::Quad {
        control: [18630, 635],
        to: [18559, 635],
    },
    HelpLabelSegment::Quad {
        control: [18521, 635],
        to: [18479, 657],
    },
    HelpLabelSegment::Quad {
        control: [18438, 681],
        to: [18416, 714],
    },
    HelpLabelSegment::Line([18416, 1180]),
    HelpLabelSegment::Line([18305, 1180]),
    HelpLabelSegment::Line([18305, 553]),
    HelpLabelSegment::Line([18381, 553]),
    HelpLabelSegment::Line([18416, 634]),
    HelpLabelSegment::Quad {
        control: [18471, 541],
        to: [18595, 541],
    },
    HelpLabelSegment::Quad {
        control: [18802, 541],
        to: [18802, 792],
    },
];

const BODY_CONTOUR_66: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [19144, 635],
        to: [19093, 683],
    },
    HelpLabelSegment::Quad {
        control: [19045, 729],
        to: [19038, 797],
    },
    HelpLabelSegment::Line([19386, 797]),
    HelpLabelSegment::Quad {
        control: [19386, 729],
        to: [19344, 684],
    },
    HelpLabelSegment::Quad {
        control: [19297, 635],
        to: [19217, 635],
    },
];

const BODY_CONTOUR_67: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [19150, 1098],
        to: [19233, 1098],
    },
    HelpLabelSegment::Quad {
        control: [19329, 1098],
        to: [19392, 1043],
    },
    HelpLabelSegment::Line([19439, 1123]),
    HelpLabelSegment::Quad {
        control: [19413, 1148],
        to: [19360, 1167],
    },
    HelpLabelSegment::Quad {
        control: [19294, 1192],
        to: [19212, 1192],
    },
    HelpLabelSegment::Quad {
        control: [19093, 1192],
        to: [19010, 1112],
    },
    HelpLabelSegment::Quad {
        control: [18919, 1023],
        to: [18919, 874],
    },
    HelpLabelSegment::Quad {
        control: [18919, 718],
        to: [19012, 625],
    },
    HelpLabelSegment::Quad {
        control: [19097, 541],
        to: [19213, 541],
    },
    HelpLabelSegment::Quad {
        control: [19346, 541],
        to: [19423, 616],
    },
    HelpLabelSegment::Quad {
        control: [19496, 689],
        to: [19496, 810],
    },
    HelpLabelSegment::Quad {
        control: [19496, 846],
        to: [19488, 878],
    },
    HelpLabelSegment::Line([19036, 878]),
    HelpLabelSegment::Quad {
        control: [19036, 988],
        to: [19096, 1046],
    },
];

const BODY_CONTOUR_68: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Line([20112, 1180]),
    HelpLabelSegment::Line([20000, 1180]),
    HelpLabelSegment::Line([20000, 816]),
    HelpLabelSegment::Quad {
        control: [20000, 715],
        to: [19971, 675],
    },
    HelpLabelSegment::Quad {
        control: [19940, 635],
        to: [19869, 635],
    },
    HelpLabelSegment::Quad {
        control: [19831, 635],
        to: [19789, 657],
    },
    HelpLabelSegment::Quad {
        control: [19748, 681],
        to: [19726, 714],
    },
    HelpLabelSegment::Line([19726, 1180]),
    HelpLabelSegment::Line([19615, 1180]),
    HelpLabelSegment::Line([19615, 553]),
    HelpLabelSegment::Line([19691, 553]),
    HelpLabelSegment::Line([19726, 634]),
    HelpLabelSegment::Quad {
        control: [19781, 541],
        to: [19905, 541],
    },
    HelpLabelSegment::Quad {
        control: [20112, 541],
        to: [20112, 792],
    },
];

const BODY_CONTOUR_69: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Line([20310, 422]),
    HelpLabelSegment::Line([20421, 378]),
    HelpLabelSegment::Line([20421, 553]),
    HelpLabelSegment::Line([20593, 553]),
    HelpLabelSegment::Line([20593, 641]),
    HelpLabelSegment::Line([20421, 641]),
    HelpLabelSegment::Line([20421, 953]),
    HelpLabelSegment::Quad {
        control: [20421, 1031],
        to: [20448, 1065],
    },
    HelpLabelSegment::Quad {
        control: [20474, 1098],
        to: [20533, 1098],
    },
    HelpLabelSegment::Quad {
        control: [20576, 1098],
        to: [20621, 1077],
    },
    HelpLabelSegment::Line([20638, 1174]),
    HelpLabelSegment::Line([20486, 1192]),
    HelpLabelSegment::Quad {
        control: [20411, 1192],
        to: [20361, 1137],
    },
    HelpLabelSegment::Quad {
        control: [20310, 1082],
        to: [20310, 997],
    },
    HelpLabelSegment::Line([20310, 641]),
    HelpLabelSegment::Line([20237, 641]),
    HelpLabelSegment::Line([20237, 553]),
    HelpLabelSegment::Line([20310, 553]),
];

const BODY_CONTOUR_70: [HelpLabelSegment; 22] = [
    HelpLabelSegment::Line([21052, 688]),
    HelpLabelSegment::Quad {
        control: [20986, 635],
        to: [20919, 635],
    },
    HelpLabelSegment::Quad {
        control: [20879, 635],
        to: [20852, 654],
    },
    HelpLabelSegment::Quad {
        control: [20824, 673],
        to: [20824, 701],
    },
    HelpLabelSegment::Quad {
        control: [20824, 762],
        to: [20894, 792],
    },
    HelpLabelSegment::Line([20973, 828]),
    HelpLabelSegment::Quad {
        control: [21046, 862],
        to: [21080, 905],
    },
    HelpLabelSegment::Quad {
        control: [21113, 948],
        to: [21113, 1012],
    },
    HelpLabelSegment::Quad {
        control: [21113, 1097],
        to: [21054, 1145],
    },
    HelpLabelSegment::Quad {
        control: [20994, 1192],
        to: [20890, 1192],
    },
    HelpLabelSegment::Quad {
        control: [20790, 1192],
        to: [20704, 1142],
    },
    HelpLabelSegment::Line([20742, 1037]),
    HelpLabelSegment::Quad {
        control: [20836, 1098],
        to: [20892, 1098],
    },
    HelpLabelSegment::Quad {
        control: [20995, 1098],
        to: [20995, 1011],
    },
    HelpLabelSegment::Quad {
        control: [20995, 949],
        to: [20896, 905],
    },
    HelpLabelSegment::Quad {
        control: [20820, 869],
        to: [20793, 852],
    },
    HelpLabelSegment::Quad {
        control: [20766, 833],
        to: [20747, 811],
    },
    HelpLabelSegment::Quad {
        control: [20727, 787],
        to: [20718, 762],
    },
    HelpLabelSegment::Quad {
        control: [20707, 735],
        to: [20707, 705],
    },
    HelpLabelSegment::Quad {
        control: [20707, 628],
        to: [20763, 585],
    },
    HelpLabelSegment::Quad {
        control: [20820, 541],
        to: [20911, 541],
    },
    HelpLabelSegment::Quad {
        control: [20979, 541],
        to: [21083, 585],
    },
];

const BODY_CONTOUR_71: [HelpLabelSegment; 32] = [
    HelpLabelSegment::Line([22001, 632]),
    HelpLabelSegment::Quad {
        control: [22044, 687],
        to: [22044, 777],
    },
    HelpLabelSegment::Quad {
        control: [22044, 872],
        to: [21985, 936],
    },
    HelpLabelSegment::Quad {
        control: [21926, 1001],
        to: [21830, 1010],
    },
    HelpLabelSegment::Line([21737, 1019]),
    HelpLabelSegment::Line([21694, 1032]),
    HelpLabelSegment::Quad {
        control: [21666, 1043],
        to: [21666, 1060],
    },
    HelpLabelSegment::Quad {
        control: [21666, 1084],
        to: [21723, 1084],
    },
    HelpLabelSegment::Line([21802, 1076]),
    HelpLabelSegment::Line([21881, 1066]),
    HelpLabelSegment::Quad {
        control: [21974, 1066],
        to: [22026, 1111],
    },
    HelpLabelSegment::Quad {
        control: [22077, 1154],
        to: [22077, 1233],
    },
    HelpLabelSegment::Quad {
        control: [22077, 1319],
        to: [22000, 1373],
    },
    HelpLabelSegment::Quad {
        control: [21922, 1426],
        to: [21803, 1426],
    },
    HelpLabelSegment::Quad {
        control: [21742, 1426],
        to: [21675, 1405],
    },
    HelpLabelSegment::Quad {
        control: [21607, 1383],
        to: [21566, 1352],
    },
    HelpLabelSegment::Line([21627, 1263]),
    HelpLabelSegment::Quad {
        control: [21724, 1328],
        to: [21806, 1328],
    },
    HelpLabelSegment::Quad {
        control: [21881, 1328],
        to: [21926, 1302],
    },
    HelpLabelSegment::Quad {
        control: [21969, 1276],
        to: [21969, 1237],
    },
    HelpLabelSegment::Quad {
        control: [21969, 1161],
        to: [21859, 1161],
    },
    HelpLabelSegment::Line([21791, 1171]),
    HelpLabelSegment::Line([21714, 1180]),
    HelpLabelSegment::Quad {
        control: [21580, 1180],
        to: [21580, 1079],
    },
    HelpLabelSegment::Quad {
        control: [21580, 1048],
        to: [21612, 1023],
    },
    HelpLabelSegment::Quad {
        control: [21644, 997],
        to: [21689, 987],
    },
    HelpLabelSegment::Quad {
        control: [21557, 925],
        to: [21557, 771],
    },
    HelpLabelSegment::Quad {
        control: [21557, 673],
        to: [21626, 607],
    },
    HelpLabelSegment::Quad {
        control: [21694, 541],
        to: [21795, 541],
    },
    HelpLabelSegment::Quad {
        control: [21887, 541],
        to: [21940, 579],
    },
    HelpLabelSegment::Line([21995, 512]),
    HelpLabelSegment::Line([22068, 581]),
];

const BODY_CONTOUR_72: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [21744, 630],
        to: [21708, 671],
    },
    HelpLabelSegment::Quad {
        control: [21672, 712],
        to: [21672, 771],
    },
    HelpLabelSegment::Quad {
        control: [21672, 837],
        to: [21707, 880],
    },
    HelpLabelSegment::Quad {
        control: [21742, 923],
        to: [21803, 923],
    },
    HelpLabelSegment::Quad {
        control: [21862, 923],
        to: [21896, 881],
    },
    HelpLabelSegment::Quad {
        control: [21928, 839],
        to: [21928, 771],
    },
    HelpLabelSegment::Quad {
        control: [21928, 712],
        to: [21893, 671],
    },
    HelpLabelSegment::Quad {
        control: [21857, 630],
        to: [21803, 630],
    },
];

const BODY_CONTOUR_73: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [22266, 1101],
        to: [22432, 1101],
    },
    HelpLabelSegment::Quad {
        control: [22511, 1101],
        to: [22555, 1038],
    },
    HelpLabelSegment::Quad {
        control: [22599, 975],
        to: [22599, 865],
    },
    HelpLabelSegment::Quad {
        control: [22599, 632],
        to: [22432, 632],
    },
    HelpLabelSegment::Quad {
        control: [22356, 632],
        to: [22312, 694],
    },
    HelpLabelSegment::Quad {
        control: [22266, 756],
        to: [22266, 865],
    },
];

const BODY_CONTOUR_74: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [22305, 541],
        to: [22432, 541],
    },
    HelpLabelSegment::Quad {
        control: [22567, 541],
        to: [22642, 627],
    },
    HelpLabelSegment::Quad {
        control: [22716, 712],
        to: [22716, 865],
    },
    HelpLabelSegment::Quad {
        control: [22716, 1017],
        to: [22640, 1105],
    },
    HelpLabelSegment::Quad {
        control: [22564, 1192],
        to: [22432, 1192],
    },
    HelpLabelSegment::Quad {
        control: [22299, 1192],
        to: [22224, 1104],
    },
    HelpLabelSegment::Quad {
        control: [22149, 1015],
        to: [22149, 865],
    },
    HelpLabelSegment::Quad {
        control: [22149, 719],
        to: [22227, 630],
    },
];

const BODY_CONTOUR_75: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Quad {
        control: [23166, 541],
        to: [23228, 603],
    },
    HelpLabelSegment::Quad {
        control: [23289, 666],
        to: [23289, 800],
    },
    HelpLabelSegment::Line([23289, 1025]),
    HelpLabelSegment::Quad {
        control: [23289, 1109],
        to: [23339, 1135],
    },
    HelpLabelSegment::Line([23339, 1192]),
    HelpLabelSegment::Quad {
        control: [23271, 1192],
        to: [23238, 1172],
    },
    HelpLabelSegment::Quad {
        control: [23204, 1153],
        to: [23189, 1109],
    },
    HelpLabelSegment::Quad {
        control: [23122, 1192],
        to: [22985, 1192],
    },
    HelpLabelSegment::Quad {
        control: [22911, 1192],
        to: [22857, 1139],
    },
    HelpLabelSegment::Quad {
        control: [22802, 1085],
        to: [22802, 1005],
    },
    HelpLabelSegment::Quad {
        control: [22802, 909],
        to: [22886, 844],
    },
    HelpLabelSegment::Quad {
        control: [22969, 778],
        to: [23098, 778],
    },
    HelpLabelSegment::Line([23178, 793]),
    HelpLabelSegment::Quad {
        control: [23178, 641],
        to: [23042, 641],
    },
    HelpLabelSegment::Quad {
        control: [22938, 641],
        to: [22882, 697],
    },
    HelpLabelSegment::Line([22835, 603]),
    HelpLabelSegment::Quad {
        control: [22866, 578],
        to: [22923, 560],
    },
    HelpLabelSegment::Quad {
        control: [22979, 541],
        to: [23029, 541],
    },
];

const BODY_CONTOUR_76: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [23020, 860],
        to: [22967, 903],
    },
    HelpLabelSegment::Quad {
        control: [22913, 947],
        to: [22913, 1007],
    },
    HelpLabelSegment::Quad {
        control: [22913, 1104],
        to: [23029, 1104],
    },
    HelpLabelSegment::Quad {
        control: [23114, 1104],
        to: [23178, 1024],
    },
    HelpLabelSegment::Line([23178, 872]),
    HelpLabelSegment::Line([23104, 860]),
];

const BODY_CONTOUR_77: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([23690, 1192]),
    HelpLabelSegment::Quad {
        control: [23473, 1192],
        to: [23473, 1003],
    },
    HelpLabelSegment::Line([23473, 295]),
    HelpLabelSegment::Line([23584, 295]),
    HelpLabelSegment::Line([23584, 984]),
    HelpLabelSegment::Quad {
        control: [23584, 1035],
        to: [23614, 1064],
    },
    HelpLabelSegment::Quad {
        control: [23643, 1092],
        to: [23690, 1092],
    },
];

const BODY_CONTOUR_78: [HelpLabelSegment; 4] = [
    HelpLabelSegment::Line([23834, 782]),
    HelpLabelSegment::Line([24085, 782]),
    HelpLabelSegment::Line([24085, 884]),
    HelpLabelSegment::Line([23834, 884]),
];

const BODY_CONTOUR_79: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([24485, 1192]),
    HelpLabelSegment::Quad {
        control: [24268, 1192],
        to: [24268, 1003],
    },
    HelpLabelSegment::Line([24268, 295]),
    HelpLabelSegment::Line([24379, 295]),
    HelpLabelSegment::Line([24379, 984]),
    HelpLabelSegment::Quad {
        control: [24379, 1035],
        to: [24409, 1064],
    },
    HelpLabelSegment::Quad {
        control: [24438, 1092],
        to: [24485, 1092],
    },
];

const BODY_CONTOUR_80: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [24758, 315],
        to: [24779, 336],
    },
    HelpLabelSegment::Quad {
        control: [24799, 356],
        to: [24799, 384],
    },
    HelpLabelSegment::Quad {
        control: [24799, 412],
        to: [24779, 434],
    },
    HelpLabelSegment::Quad {
        control: [24758, 453],
        to: [24730, 453],
    },
    HelpLabelSegment::Quad {
        control: [24701, 453],
        to: [24681, 434],
    },
    HelpLabelSegment::Quad {
        control: [24660, 412],
        to: [24660, 384],
    },
    HelpLabelSegment::Quad {
        control: [24660, 355],
        to: [24680, 335],
    },
    HelpLabelSegment::Quad {
        control: [24700, 315],
        to: [24730, 315],
    },
];

const BODY_CONTOUR_81: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Line([24780, 1180]),
    HelpLabelSegment::Line([24669, 1180]),
    HelpLabelSegment::Line([24669, 647]),
    HelpLabelSegment::Line([24582, 647]),
    HelpLabelSegment::Line([24582, 553]),
    HelpLabelSegment::Line([24780, 553]),
];

const BODY_CONTOUR_82: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Line([25452, 1180]),
    HelpLabelSegment::Line([25340, 1180]),
    HelpLabelSegment::Line([25340, 816]),
    HelpLabelSegment::Quad {
        control: [25340, 715],
        to: [25311, 675],
    },
    HelpLabelSegment::Quad {
        control: [25280, 635],
        to: [25209, 635],
    },
    HelpLabelSegment::Quad {
        control: [25171, 635],
        to: [25129, 657],
    },
    HelpLabelSegment::Quad {
        control: [25088, 681],
        to: [25066, 714],
    },
    HelpLabelSegment::Line([25066, 1180]),
    HelpLabelSegment::Line([24955, 1180]),
    HelpLabelSegment::Line([24955, 553]),
    HelpLabelSegment::Line([25031, 553]),
    HelpLabelSegment::Line([25066, 634]),
    HelpLabelSegment::Quad {
        control: [25121, 541],
        to: [25245, 541],
    },
    HelpLabelSegment::Quad {
        control: [25452, 541],
        to: [25452, 792],
    },
];

const BODY_CONTOUR_83: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [25794, 635],
        to: [25743, 683],
    },
    HelpLabelSegment::Quad {
        control: [25695, 729],
        to: [25688, 797],
    },
    HelpLabelSegment::Line([26036, 797]),
    HelpLabelSegment::Quad {
        control: [26036, 729],
        to: [25994, 684],
    },
    HelpLabelSegment::Quad {
        control: [25947, 635],
        to: [25867, 635],
    },
];

const BODY_CONTOUR_84: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [25800, 1098],
        to: [25883, 1098],
    },
    HelpLabelSegment::Quad {
        control: [25979, 1098],
        to: [26042, 1043],
    },
    HelpLabelSegment::Line([26089, 1123]),
    HelpLabelSegment::Quad {
        control: [26063, 1148],
        to: [26010, 1167],
    },
    HelpLabelSegment::Quad {
        control: [25944, 1192],
        to: [25862, 1192],
    },
    HelpLabelSegment::Quad {
        control: [25743, 1192],
        to: [25660, 1112],
    },
    HelpLabelSegment::Quad {
        control: [25569, 1023],
        to: [25569, 874],
    },
    HelpLabelSegment::Quad {
        control: [25569, 718],
        to: [25662, 625],
    },
    HelpLabelSegment::Quad {
        control: [25747, 541],
        to: [25863, 541],
    },
    HelpLabelSegment::Quad {
        control: [25996, 541],
        to: [26073, 616],
    },
    HelpLabelSegment::Quad {
        control: [26146, 689],
        to: [26146, 810],
    },
    HelpLabelSegment::Quad {
        control: [26146, 846],
        to: [26138, 878],
    },
    HelpLabelSegment::Line([25686, 878]),
    HelpLabelSegment::Quad {
        control: [25686, 988],
        to: [25746, 1046],
    },
];

const BODY_CONTOUR_85: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [26429, 1016],
        to: [26455, 1042],
    },
    HelpLabelSegment::Quad {
        control: [26480, 1068],
        to: [26480, 1104],
    },
    HelpLabelSegment::Quad {
        control: [26480, 1140],
        to: [26455, 1166],
    },
    HelpLabelSegment::Quad {
        control: [26429, 1192],
        to: [26392, 1192],
    },
    HelpLabelSegment::Quad {
        control: [26356, 1192],
        to: [26330, 1166],
    },
    HelpLabelSegment::Quad {
        control: [26305, 1140],
        to: [26305, 1104],
    },
    HelpLabelSegment::Quad {
        control: [26305, 1068],
        to: [26330, 1042],
    },
    HelpLabelSegment::Quad {
        control: [26356, 1016],
        to: [26392, 1016],
    },
];

const BODY_CONTOUR_86: [HelpLabelSegment; 16] = [
    HelpLabelSegment::Quad {
        control: [-11218, 2107],
        to: [-11259, 2131],
    },
    HelpLabelSegment::Quad {
        control: [-11301, 2156],
        to: [-11320, 2194],
    },
    HelpLabelSegment::Line([-11395, 2132]),
    HelpLabelSegment::Quad {
        control: [-11375, 2075],
        to: [-11316, 2041],
    },
    HelpLabelSegment::Quad {
        control: [-11259, 2007],
        to: [-11176, 2007],
    },
    HelpLabelSegment::Quad {
        control: [-11053, 2007],
        to: [-10982, 2064],
    },
    HelpLabelSegment::Quad {
        control: [-10913, 2121],
        to: [-10913, 2225],
    },
    HelpLabelSegment::Quad {
        control: [-10913, 2322],
        to: [-11006, 2469],
    },
    HelpLabelSegment::Line([-11198, 2775]),
    HelpLabelSegment::Line([-10862, 2775]),
    HelpLabelSegment::Line([-10862, 2880]),
    HelpLabelSegment::Line([-11389, 2880]),
    HelpLabelSegment::Line([-11389, 2857]),
    HelpLabelSegment::Line([-11121, 2445]),
    HelpLabelSegment::Quad {
        control: [-11036, 2315],
        to: [-11036, 2225],
    },
    HelpLabelSegment::Quad {
        control: [-11036, 2107],
        to: [-11170, 2107],
    },
];

const BODY_CONTOUR_87: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [-10556, 2716],
        to: [-10530, 2742],
    },
    HelpLabelSegment::Quad {
        control: [-10505, 2768],
        to: [-10505, 2804],
    },
    HelpLabelSegment::Quad {
        control: [-10505, 2840],
        to: [-10530, 2866],
    },
    HelpLabelSegment::Quad {
        control: [-10556, 2892],
        to: [-10593, 2892],
    },
    HelpLabelSegment::Quad {
        control: [-10629, 2892],
        to: [-10655, 2866],
    },
    HelpLabelSegment::Quad {
        control: [-10680, 2840],
        to: [-10680, 2804],
    },
    HelpLabelSegment::Quad {
        control: [-10680, 2768],
        to: [-10655, 2742],
    },
    HelpLabelSegment::Quad {
        control: [-10629, 2716],
        to: [-10593, 2716],
    },
];

const BODY_CONTOUR_88: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Quad {
        control: [-9420, 2286],
        to: [-9464, 2335],
    },
    HelpLabelSegment::Quad {
        control: [-9509, 2384],
        to: [-9563, 2396],
    },
    HelpLabelSegment::Quad {
        control: [-9461, 2421],
        to: [-9414, 2478],
    },
    HelpLabelSegment::Quad {
        control: [-9367, 2534],
        to: [-9367, 2633],
    },
    HelpLabelSegment::Quad {
        control: [-9367, 2745],
        to: [-9450, 2813],
    },
    HelpLabelSegment::Quad {
        control: [-9534, 2880],
        to: [-9667, 2880],
    },
    HelpLabelSegment::Line([-9912, 2880]),
    HelpLabelSegment::Line([-9912, 2022]),
    HelpLabelSegment::Line([-9686, 2014]),
    HelpLabelSegment::Quad {
        control: [-9559, 2014],
        to: [-9489, 2069],
    },
    HelpLabelSegment::Quad {
        control: [-9420, 2124],
        to: [-9420, 2226],
    },
];

const BODY_CONTOUR_89: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Line([-9795, 2362]),
    HelpLabelSegment::Line([-9704, 2366]),
    HelpLabelSegment::Quad {
        control: [-9537, 2366],
        to: [-9537, 2231],
    },
    HelpLabelSegment::Quad {
        control: [-9537, 2111],
        to: [-9689, 2111],
    },
    HelpLabelSegment::Line([-9795, 2116]),
];

const BODY_CONTOUR_90: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([-9795, 2780]),
    HelpLabelSegment::Line([-9702, 2786]),
    HelpLabelSegment::Quad {
        control: [-9592, 2786],
        to: [-9541, 2745],
    },
    HelpLabelSegment::Quad {
        control: [-9490, 2704],
        to: [-9490, 2614],
    },
    HelpLabelSegment::Quad {
        control: [-9490, 2530],
        to: [-9538, 2490],
    },
    HelpLabelSegment::Quad {
        control: [-9588, 2450],
        to: [-9700, 2450],
    },
    HelpLabelSegment::Line([-9795, 2453]),
];

const BODY_CONTOUR_91: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [-9097, 2015],
        to: [-9076, 2036],
    },
    HelpLabelSegment::Quad {
        control: [-9056, 2056],
        to: [-9056, 2084],
    },
    HelpLabelSegment::Quad {
        control: [-9056, 2112],
        to: [-9076, 2134],
    },
    HelpLabelSegment::Quad {
        control: [-9097, 2153],
        to: [-9125, 2153],
    },
    HelpLabelSegment::Quad {
        control: [-9154, 2153],
        to: [-9174, 2134],
    },
    HelpLabelSegment::Quad {
        control: [-9195, 2112],
        to: [-9195, 2084],
    },
    HelpLabelSegment::Quad {
        control: [-9195, 2055],
        to: [-9175, 2035],
    },
    HelpLabelSegment::Quad {
        control: [-9155, 2015],
        to: [-9125, 2015],
    },
];

const BODY_CONTOUR_92: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Line([-9075, 2880]),
    HelpLabelSegment::Line([-9186, 2880]),
    HelpLabelSegment::Line([-9186, 2347]),
    HelpLabelSegment::Line([-9273, 2347]),
    HelpLabelSegment::Line([-9273, 2253]),
    HelpLabelSegment::Line([-9075, 2253]),
];

const BODY_CONTOUR_93: [HelpLabelSegment; 32] = [
    HelpLabelSegment::Line([-8489, 2332]),
    HelpLabelSegment::Quad {
        control: [-8446, 2387],
        to: [-8446, 2477],
    },
    HelpLabelSegment::Quad {
        control: [-8446, 2572],
        to: [-8505, 2636],
    },
    HelpLabelSegment::Quad {
        control: [-8564, 2701],
        to: [-8660, 2710],
    },
    HelpLabelSegment::Line([-8753, 2719]),
    HelpLabelSegment::Line([-8796, 2732]),
    HelpLabelSegment::Quad {
        control: [-8824, 2743],
        to: [-8824, 2760],
    },
    HelpLabelSegment::Quad {
        control: [-8824, 2784],
        to: [-8767, 2784],
    },
    HelpLabelSegment::Line([-8688, 2776]),
    HelpLabelSegment::Line([-8609, 2766]),
    HelpLabelSegment::Quad {
        control: [-8516, 2766],
        to: [-8464, 2811],
    },
    HelpLabelSegment::Quad {
        control: [-8413, 2854],
        to: [-8413, 2933],
    },
    HelpLabelSegment::Quad {
        control: [-8413, 3019],
        to: [-8490, 3073],
    },
    HelpLabelSegment::Quad {
        control: [-8568, 3126],
        to: [-8687, 3126],
    },
    HelpLabelSegment::Quad {
        control: [-8748, 3126],
        to: [-8815, 3105],
    },
    HelpLabelSegment::Quad {
        control: [-8883, 3083],
        to: [-8924, 3052],
    },
    HelpLabelSegment::Line([-8863, 2963]),
    HelpLabelSegment::Quad {
        control: [-8766, 3028],
        to: [-8684, 3028],
    },
    HelpLabelSegment::Quad {
        control: [-8609, 3028],
        to: [-8564, 3002],
    },
    HelpLabelSegment::Quad {
        control: [-8521, 2976],
        to: [-8521, 2937],
    },
    HelpLabelSegment::Quad {
        control: [-8521, 2861],
        to: [-8631, 2861],
    },
    HelpLabelSegment::Line([-8699, 2871]),
    HelpLabelSegment::Line([-8776, 2880]),
    HelpLabelSegment::Quad {
        control: [-8910, 2880],
        to: [-8910, 2779],
    },
    HelpLabelSegment::Quad {
        control: [-8910, 2748],
        to: [-8878, 2723],
    },
    HelpLabelSegment::Quad {
        control: [-8846, 2697],
        to: [-8801, 2687],
    },
    HelpLabelSegment::Quad {
        control: [-8933, 2625],
        to: [-8933, 2471],
    },
    HelpLabelSegment::Quad {
        control: [-8933, 2373],
        to: [-8864, 2307],
    },
    HelpLabelSegment::Quad {
        control: [-8796, 2241],
        to: [-8695, 2241],
    },
    HelpLabelSegment::Quad {
        control: [-8603, 2241],
        to: [-8550, 2279],
    },
    HelpLabelSegment::Line([-8495, 2212]),
    HelpLabelSegment::Line([-8422, 2281]),
];

const BODY_CONTOUR_94: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [-8746, 2330],
        to: [-8782, 2371],
    },
    HelpLabelSegment::Quad {
        control: [-8818, 2412],
        to: [-8818, 2471],
    },
    HelpLabelSegment::Quad {
        control: [-8818, 2537],
        to: [-8783, 2580],
    },
    HelpLabelSegment::Quad {
        control: [-8748, 2623],
        to: [-8687, 2623],
    },
    HelpLabelSegment::Quad {
        control: [-8628, 2623],
        to: [-8594, 2581],
    },
    HelpLabelSegment::Quad {
        control: [-8562, 2539],
        to: [-8562, 2471],
    },
    HelpLabelSegment::Quad {
        control: [-8562, 2412],
        to: [-8597, 2371],
    },
    HelpLabelSegment::Quad {
        control: [-8633, 2330],
        to: [-8687, 2330],
    },
];

const BODY_CONTOUR_95: [HelpLabelSegment; 13] = [
    HelpLabelSegment::Line([-7829, 2299]),
    HelpLabelSegment::Quad {
        control: [-7814, 2278],
        to: [-7769, 2259],
    },
    HelpLabelSegment::Quad {
        control: [-7726, 2241],
        to: [-7684, 2241],
    },
    HelpLabelSegment::Quad {
        control: [-7555, 2241],
        to: [-7475, 2330],
    },
    HelpLabelSegment::Quad {
        control: [-7395, 2419],
        to: [-7395, 2555],
    },
    HelpLabelSegment::Quad {
        control: [-7395, 2712],
        to: [-7475, 2803],
    },
    HelpLabelSegment::Quad {
        control: [-7556, 2892],
        to: [-7693, 2892],
    },
    HelpLabelSegment::Quad {
        control: [-7738, 2892],
        to: [-7780, 2875],
    },
    HelpLabelSegment::Quad {
        control: [-7823, 2859],
        to: [-7845, 2835],
    },
    HelpLabelSegment::Line([-7885, 2892]),
    HelpLabelSegment::Line([-7940, 2892]),
    HelpLabelSegment::Line([-7940, 1995]),
    HelpLabelSegment::Line([-7829, 1995]),
];

const BODY_CONTOUR_96: [HelpLabelSegment; 9] = [
    HelpLabelSegment::Line([-7829, 2746]),
    HelpLabelSegment::Quad {
        control: [-7829, 2756],
        to: [-7788, 2777],
    },
    HelpLabelSegment::Quad {
        control: [-7746, 2798],
        to: [-7725, 2798],
    },
    HelpLabelSegment::Quad {
        control: [-7611, 2798],
        to: [-7562, 2744],
    },
    HelpLabelSegment::Quad {
        control: [-7513, 2689],
        to: [-7513, 2561],
    },
    HelpLabelSegment::Quad {
        control: [-7513, 2455],
        to: [-7570, 2395],
    },
    HelpLabelSegment::Quad {
        control: [-7627, 2335],
        to: [-7725, 2335],
    },
    HelpLabelSegment::Quad {
        control: [-7745, 2335],
        to: [-7781, 2353],
    },
    HelpLabelSegment::Line([-7829, 2384]),
];

const BODY_CONTOUR_97: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Quad {
        control: [-6939, 2241],
        to: [-6877, 2303],
    },
    HelpLabelSegment::Quad {
        control: [-6816, 2366],
        to: [-6816, 2500],
    },
    HelpLabelSegment::Line([-6816, 2725]),
    HelpLabelSegment::Quad {
        control: [-6816, 2809],
        to: [-6766, 2835],
    },
    HelpLabelSegment::Line([-6766, 2892]),
    HelpLabelSegment::Quad {
        control: [-6834, 2892],
        to: [-6867, 2872],
    },
    HelpLabelSegment::Quad {
        control: [-6901, 2853],
        to: [-6916, 2809],
    },
    HelpLabelSegment::Quad {
        control: [-6983, 2892],
        to: [-7120, 2892],
    },
    HelpLabelSegment::Quad {
        control: [-7194, 2892],
        to: [-7248, 2839],
    },
    HelpLabelSegment::Quad {
        control: [-7303, 2785],
        to: [-7303, 2705],
    },
    HelpLabelSegment::Quad {
        control: [-7303, 2609],
        to: [-7219, 2544],
    },
    HelpLabelSegment::Quad {
        control: [-7136, 2478],
        to: [-7007, 2478],
    },
    HelpLabelSegment::Line([-6927, 2493]),
    HelpLabelSegment::Quad {
        control: [-6927, 2341],
        to: [-7063, 2341],
    },
    HelpLabelSegment::Quad {
        control: [-7167, 2341],
        to: [-7223, 2397],
    },
    HelpLabelSegment::Line([-7270, 2303]),
    HelpLabelSegment::Quad {
        control: [-7239, 2278],
        to: [-7182, 2260],
    },
    HelpLabelSegment::Quad {
        control: [-7126, 2241],
        to: [-7076, 2241],
    },
];

const BODY_CONTOUR_98: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [-7085, 2560],
        to: [-7138, 2603],
    },
    HelpLabelSegment::Quad {
        control: [-7192, 2647],
        to: [-7192, 2707],
    },
    HelpLabelSegment::Quad {
        control: [-7192, 2804],
        to: [-7076, 2804],
    },
    HelpLabelSegment::Quad {
        control: [-6991, 2804],
        to: [-6927, 2724],
    },
    HelpLabelSegment::Line([-6927, 2572]),
    HelpLabelSegment::Line([-7001, 2560]),
];

const BODY_CONTOUR_99: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([-6415, 2892]),
    HelpLabelSegment::Quad {
        control: [-6632, 2892],
        to: [-6632, 2703],
    },
    HelpLabelSegment::Line([-6632, 1995]),
    HelpLabelSegment::Line([-6521, 1995]),
    HelpLabelSegment::Line([-6521, 2684]),
    HelpLabelSegment::Quad {
        control: [-6521, 2735],
        to: [-6491, 2764],
    },
    HelpLabelSegment::Quad {
        control: [-6462, 2792],
        to: [-6415, 2792],
    },
];

const BODY_CONTOUR_100: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([-6060, 2892]),
    HelpLabelSegment::Quad {
        control: [-6277, 2892],
        to: [-6277, 2703],
    },
    HelpLabelSegment::Line([-6277, 1995]),
    HelpLabelSegment::Line([-6166, 1995]),
    HelpLabelSegment::Line([-6166, 2684]),
    HelpLabelSegment::Quad {
        control: [-6166, 2735],
        to: [-6136, 2764],
    },
    HelpLabelSegment::Quad {
        control: [-6107, 2792],
        to: [-6060, 2792],
    },
];

const BODY_CONTOUR_101: [HelpLabelSegment; 22] = [
    HelpLabelSegment::Line([-5623, 2388]),
    HelpLabelSegment::Quad {
        control: [-5689, 2335],
        to: [-5756, 2335],
    },
    HelpLabelSegment::Quad {
        control: [-5796, 2335],
        to: [-5823, 2354],
    },
    HelpLabelSegment::Quad {
        control: [-5851, 2373],
        to: [-5851, 2401],
    },
    HelpLabelSegment::Quad {
        control: [-5851, 2462],
        to: [-5781, 2492],
    },
    HelpLabelSegment::Line([-5702, 2528]),
    HelpLabelSegment::Quad {
        control: [-5629, 2562],
        to: [-5595, 2605],
    },
    HelpLabelSegment::Quad {
        control: [-5562, 2648],
        to: [-5562, 2712],
    },
    HelpLabelSegment::Quad {
        control: [-5562, 2797],
        to: [-5621, 2845],
    },
    HelpLabelSegment::Quad {
        control: [-5681, 2892],
        to: [-5785, 2892],
    },
    HelpLabelSegment::Quad {
        control: [-5885, 2892],
        to: [-5971, 2842],
    },
    HelpLabelSegment::Line([-5933, 2737]),
    HelpLabelSegment::Quad {
        control: [-5839, 2798],
        to: [-5783, 2798],
    },
    HelpLabelSegment::Quad {
        control: [-5680, 2798],
        to: [-5680, 2711],
    },
    HelpLabelSegment::Quad {
        control: [-5680, 2649],
        to: [-5779, 2605],
    },
    HelpLabelSegment::Quad {
        control: [-5855, 2569],
        to: [-5882, 2552],
    },
    HelpLabelSegment::Quad {
        control: [-5909, 2533],
        to: [-5928, 2511],
    },
    HelpLabelSegment::Quad {
        control: [-5948, 2487],
        to: [-5957, 2462],
    },
    HelpLabelSegment::Quad {
        control: [-5968, 2435],
        to: [-5968, 2405],
    },
    HelpLabelSegment::Quad {
        control: [-5968, 2328],
        to: [-5912, 2285],
    },
    HelpLabelSegment::Quad {
        control: [-5855, 2241],
        to: [-5764, 2241],
    },
    HelpLabelSegment::Quad {
        control: [-5696, 2241],
        to: [-5592, 2285],
    },
];

const BODY_CONTOUR_102: [HelpLabelSegment; 22] = [
    HelpLabelSegment::Line([-4778, 2388]),
    HelpLabelSegment::Quad {
        control: [-4844, 2335],
        to: [-4911, 2335],
    },
    HelpLabelSegment::Quad {
        control: [-4951, 2335],
        to: [-4978, 2354],
    },
    HelpLabelSegment::Quad {
        control: [-5006, 2373],
        to: [-5006, 2401],
    },
    HelpLabelSegment::Quad {
        control: [-5006, 2462],
        to: [-4936, 2492],
    },
    HelpLabelSegment::Line([-4857, 2528]),
    HelpLabelSegment::Quad {
        control: [-4784, 2562],
        to: [-4750, 2605],
    },
    HelpLabelSegment::Quad {
        control: [-4717, 2648],
        to: [-4717, 2712],
    },
    HelpLabelSegment::Quad {
        control: [-4717, 2797],
        to: [-4776, 2845],
    },
    HelpLabelSegment::Quad {
        control: [-4836, 2892],
        to: [-4940, 2892],
    },
    HelpLabelSegment::Quad {
        control: [-5040, 2892],
        to: [-5126, 2842],
    },
    HelpLabelSegment::Line([-5088, 2737]),
    HelpLabelSegment::Quad {
        control: [-4994, 2798],
        to: [-4938, 2798],
    },
    HelpLabelSegment::Quad {
        control: [-4835, 2798],
        to: [-4835, 2711],
    },
    HelpLabelSegment::Quad {
        control: [-4835, 2649],
        to: [-4934, 2605],
    },
    HelpLabelSegment::Quad {
        control: [-5010, 2569],
        to: [-5037, 2552],
    },
    HelpLabelSegment::Quad {
        control: [-5064, 2533],
        to: [-5083, 2511],
    },
    HelpLabelSegment::Quad {
        control: [-5103, 2487],
        to: [-5112, 2462],
    },
    HelpLabelSegment::Quad {
        control: [-5123, 2435],
        to: [-5123, 2405],
    },
    HelpLabelSegment::Quad {
        control: [-5123, 2328],
        to: [-5067, 2285],
    },
    HelpLabelSegment::Quad {
        control: [-5010, 2241],
        to: [-4919, 2241],
    },
    HelpLabelSegment::Quad {
        control: [-4851, 2241],
        to: [-4747, 2285],
    },
];

const BODY_CONTOUR_103: [HelpLabelSegment; 16] = [
    HelpLabelSegment::Quad {
        control: [-4154, 2282],
        to: [-4127, 2303],
    },
    HelpLabelSegment::Line([-4182, 2382]),
    HelpLabelSegment::Quad {
        control: [-4200, 2366],
        to: [-4242, 2350],
    },
    HelpLabelSegment::Line([-4327, 2335]),
    HelpLabelSegment::Quad {
        control: [-4418, 2335],
        to: [-4471, 2398],
    },
    HelpLabelSegment::Quad {
        control: [-4524, 2462],
        to: [-4524, 2573],
    },
    HelpLabelSegment::Quad {
        control: [-4524, 2683],
        to: [-4470, 2741],
    },
    HelpLabelSegment::Quad {
        control: [-4415, 2798],
        to: [-4319, 2798],
    },
    HelpLabelSegment::Quad {
        control: [-4244, 2798],
        to: [-4168, 2741],
    },
    HelpLabelSegment::Line([-4123, 2834]),
    HelpLabelSegment::Quad {
        control: [-4214, 2892],
        to: [-4346, 2892],
    },
    HelpLabelSegment::Quad {
        control: [-4474, 2892],
        to: [-4558, 2806],
    },
    HelpLabelSegment::Quad {
        control: [-4641, 2719],
        to: [-4641, 2573],
    },
    HelpLabelSegment::Quad {
        control: [-4641, 2423],
        to: [-4555, 2332],
    },
    HelpLabelSegment::Quad {
        control: [-4468, 2241],
        to: [-4317, 2241],
    },
    HelpLabelSegment::Line([-4211, 2261]),
];

const BODY_CONTOUR_104: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [-3929, 2801],
        to: [-3763, 2801],
    },
    HelpLabelSegment::Quad {
        control: [-3684, 2801],
        to: [-3640, 2738],
    },
    HelpLabelSegment::Quad {
        control: [-3596, 2675],
        to: [-3596, 2565],
    },
    HelpLabelSegment::Quad {
        control: [-3596, 2332],
        to: [-3763, 2332],
    },
    HelpLabelSegment::Quad {
        control: [-3839, 2332],
        to: [-3883, 2394],
    },
    HelpLabelSegment::Quad {
        control: [-3929, 2456],
        to: [-3929, 2565],
    },
];

const BODY_CONTOUR_105: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [-3890, 2241],
        to: [-3763, 2241],
    },
    HelpLabelSegment::Quad {
        control: [-3628, 2241],
        to: [-3553, 2327],
    },
    HelpLabelSegment::Quad {
        control: [-3479, 2412],
        to: [-3479, 2565],
    },
    HelpLabelSegment::Quad {
        control: [-3479, 2717],
        to: [-3555, 2805],
    },
    HelpLabelSegment::Quad {
        control: [-3631, 2892],
        to: [-3763, 2892],
    },
    HelpLabelSegment::Quad {
        control: [-3896, 2892],
        to: [-3971, 2804],
    },
    HelpLabelSegment::Quad {
        control: [-4046, 2715],
        to: [-4046, 2565],
    },
    HelpLabelSegment::Quad {
        control: [-4046, 2419],
        to: [-3968, 2330],
    },
];

const BODY_CONTOUR_106: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Quad {
        control: [-3056, 2335],
        to: [-3093, 2335],
    },
    HelpLabelSegment::Quad {
        control: [-3152, 2335],
        to: [-3196, 2389],
    },
    HelpLabelSegment::Quad {
        control: [-3241, 2444],
        to: [-3241, 2520],
    },
    HelpLabelSegment::Line([-3241, 2880]),
    HelpLabelSegment::Line([-3352, 2880]),
    HelpLabelSegment::Line([-3352, 2253]),
    HelpLabelSegment::Line([-3241, 2253]),
    HelpLabelSegment::Line([-3241, 2353]),
    HelpLabelSegment::Quad {
        control: [-3180, 2241],
        to: [-3059, 2241],
    },
    HelpLabelSegment::Line([-2974, 2252]),
    HelpLabelSegment::Line([-3019, 2360]),
];

const BODY_CONTOUR_107: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [-2711, 2335],
        to: [-2762, 2383],
    },
    HelpLabelSegment::Quad {
        control: [-2810, 2429],
        to: [-2817, 2497],
    },
    HelpLabelSegment::Line([-2469, 2497]),
    HelpLabelSegment::Quad {
        control: [-2469, 2429],
        to: [-2511, 2384],
    },
    HelpLabelSegment::Quad {
        control: [-2558, 2335],
        to: [-2638, 2335],
    },
];

const BODY_CONTOUR_108: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [-2705, 2798],
        to: [-2622, 2798],
    },
    HelpLabelSegment::Quad {
        control: [-2526, 2798],
        to: [-2463, 2743],
    },
    HelpLabelSegment::Line([-2416, 2823]),
    HelpLabelSegment::Quad {
        control: [-2442, 2848],
        to: [-2495, 2867],
    },
    HelpLabelSegment::Quad {
        control: [-2561, 2892],
        to: [-2643, 2892],
    },
    HelpLabelSegment::Quad {
        control: [-2762, 2892],
        to: [-2845, 2812],
    },
    HelpLabelSegment::Quad {
        control: [-2936, 2723],
        to: [-2936, 2574],
    },
    HelpLabelSegment::Quad {
        control: [-2936, 2418],
        to: [-2843, 2325],
    },
    HelpLabelSegment::Quad {
        control: [-2758, 2241],
        to: [-2642, 2241],
    },
    HelpLabelSegment::Quad {
        control: [-2509, 2241],
        to: [-2432, 2316],
    },
    HelpLabelSegment::Quad {
        control: [-2359, 2389],
        to: [-2359, 2510],
    },
    HelpLabelSegment::Quad {
        control: [-2359, 2546],
        to: [-2367, 2578],
    },
    HelpLabelSegment::Line([-2819, 2578]),
    HelpLabelSegment::Quad {
        control: [-2819, 2688],
        to: [-2759, 2746],
    },
];

const BODY_CONTOUR_109: [HelpLabelSegment; 24] = [
    HelpLabelSegment::Quad {
        control: [-1042, 2356],
        to: [-1042, 2460],
    },
    HelpLabelSegment::Line([-1042, 2880]),
    HelpLabelSegment::Line([-1154, 2880]),
    HelpLabelSegment::Line([-1154, 2483]),
    HelpLabelSegment::Quad {
        control: [-1154, 2335],
        to: [-1283, 2335],
    },
    HelpLabelSegment::Quad {
        control: [-1323, 2335],
        to: [-1358, 2360],
    },
    HelpLabelSegment::Quad {
        control: [-1393, 2384],
        to: [-1406, 2416],
    },
    HelpLabelSegment::Line([-1406, 2880]),
    HelpLabelSegment::Line([-1517, 2880]),
    HelpLabelSegment::Line([-1517, 2435]),
    HelpLabelSegment::Quad {
        control: [-1517, 2388],
        to: [-1552, 2362],
    },
    HelpLabelSegment::Quad {
        control: [-1587, 2335],
        to: [-1645, 2335],
    },
    HelpLabelSegment::Quad {
        control: [-1678, 2335],
        to: [-1715, 2361],
    },
    HelpLabelSegment::Quad {
        control: [-1754, 2387],
        to: [-1769, 2417],
    },
    HelpLabelSegment::Line([-1769, 2880]),
    HelpLabelSegment::Line([-1880, 2880]),
    HelpLabelSegment::Line([-1880, 2253]),
    HelpLabelSegment::Line([-1808, 2253]),
    HelpLabelSegment::Line([-1771, 2326]),
    HelpLabelSegment::Quad {
        control: [-1707, 2241],
        to: [-1610, 2241],
    },
    HelpLabelSegment::Quad {
        control: [-1475, 2241],
        to: [-1421, 2325],
    },
    HelpLabelSegment::Quad {
        control: [-1402, 2289],
        to: [-1352, 2265],
    },
    HelpLabelSegment::Quad {
        control: [-1300, 2241],
        to: [-1246, 2241],
    },
    HelpLabelSegment::Quad {
        control: [-1149, 2241],
        to: [-1096, 2299],
    },
];

const BODY_CONTOUR_110: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [-809, 2801],
        to: [-643, 2801],
    },
    HelpLabelSegment::Quad {
        control: [-564, 2801],
        to: [-520, 2738],
    },
    HelpLabelSegment::Quad {
        control: [-476, 2675],
        to: [-476, 2565],
    },
    HelpLabelSegment::Quad {
        control: [-476, 2332],
        to: [-643, 2332],
    },
    HelpLabelSegment::Quad {
        control: [-719, 2332],
        to: [-763, 2394],
    },
    HelpLabelSegment::Quad {
        control: [-809, 2456],
        to: [-809, 2565],
    },
];

const BODY_CONTOUR_111: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [-770, 2241],
        to: [-643, 2241],
    },
    HelpLabelSegment::Quad {
        control: [-508, 2241],
        to: [-433, 2327],
    },
    HelpLabelSegment::Quad {
        control: [-359, 2412],
        to: [-359, 2565],
    },
    HelpLabelSegment::Quad {
        control: [-359, 2717],
        to: [-435, 2805],
    },
    HelpLabelSegment::Quad {
        control: [-511, 2892],
        to: [-643, 2892],
    },
    HelpLabelSegment::Quad {
        control: [-776, 2892],
        to: [-851, 2804],
    },
    HelpLabelSegment::Quad {
        control: [-926, 2715],
        to: [-926, 2565],
    },
    HelpLabelSegment::Quad {
        control: [-926, 2419],
        to: [-848, 2330],
    },
];

const BODY_CONTOUR_112: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Quad {
        control: [64, 2335],
        to: [27, 2335],
    },
    HelpLabelSegment::Quad {
        control: [-32, 2335],
        to: [-76, 2389],
    },
    HelpLabelSegment::Quad {
        control: [-121, 2444],
        to: [-121, 2520],
    },
    HelpLabelSegment::Line([-121, 2880]),
    HelpLabelSegment::Line([-232, 2880]),
    HelpLabelSegment::Line([-232, 2253]),
    HelpLabelSegment::Line([-121, 2253]),
    HelpLabelSegment::Line([-121, 2353]),
    HelpLabelSegment::Quad {
        control: [-60, 2241],
        to: [61, 2241],
    },
    HelpLabelSegment::Line([146, 2252]),
    HelpLabelSegment::Line([101, 2360]),
];

const BODY_CONTOUR_113: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [409, 2335],
        to: [358, 2383],
    },
    HelpLabelSegment::Quad {
        control: [310, 2429],
        to: [303, 2497],
    },
    HelpLabelSegment::Line([651, 2497]),
    HelpLabelSegment::Quad {
        control: [651, 2429],
        to: [609, 2384],
    },
    HelpLabelSegment::Quad {
        control: [562, 2335],
        to: [483, 2335],
    },
];

const BODY_CONTOUR_114: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [415, 2798],
        to: [498, 2798],
    },
    HelpLabelSegment::Quad {
        control: [594, 2798],
        to: [657, 2743],
    },
    HelpLabelSegment::Line([704, 2823]),
    HelpLabelSegment::Quad {
        control: [678, 2848],
        to: [625, 2867],
    },
    HelpLabelSegment::Quad {
        control: [559, 2892],
        to: [477, 2892],
    },
    HelpLabelSegment::Quad {
        control: [358, 2892],
        to: [275, 2812],
    },
    HelpLabelSegment::Quad {
        control: [184, 2723],
        to: [184, 2574],
    },
    HelpLabelSegment::Quad {
        control: [184, 2418],
        to: [277, 2325],
    },
    HelpLabelSegment::Quad {
        control: [362, 2241],
        to: [478, 2241],
    },
    HelpLabelSegment::Quad {
        control: [611, 2241],
        to: [688, 2316],
    },
    HelpLabelSegment::Quad {
        control: [761, 2389],
        to: [761, 2510],
    },
    HelpLabelSegment::Quad {
        control: [761, 2546],
        to: [753, 2578],
    },
    HelpLabelSegment::Line([301, 2578]),
    HelpLabelSegment::Quad {
        control: [301, 2688],
        to: [361, 2746],
    },
];

const BODY_CONTOUR_115: [HelpLabelSegment; 12] = [
    HelpLabelSegment::Line([1351, 2305]),
    HelpLabelSegment::Quad {
        control: [1414, 2241],
        to: [1503, 2241],
    },
    HelpLabelSegment::Quad {
        control: [1637, 2241],
        to: [1712, 2325],
    },
    HelpLabelSegment::Quad {
        control: [1786, 2408],
        to: [1786, 2568],
    },
    HelpLabelSegment::Quad {
        control: [1786, 2711],
        to: [1711, 2801],
    },
    HelpLabelSegment::Quad {
        control: [1636, 2892],
        to: [1494, 2892],
    },
    HelpLabelSegment::Quad {
        control: [1454, 2892],
        to: [1410, 2878],
    },
    HelpLabelSegment::Quad {
        control: [1364, 2864],
        to: [1351, 2846],
    },
    HelpLabelSegment::Line([1351, 3126]),
    HelpLabelSegment::Line([1240, 3126]),
    HelpLabelSegment::Line([1240, 2253]),
    HelpLabelSegment::Line([1351, 2253]),
];

const BODY_CONTOUR_116: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Line([1351, 2753]),
    HelpLabelSegment::Quad {
        control: [1362, 2770],
        to: [1396, 2784],
    },
    HelpLabelSegment::Quad {
        control: [1430, 2798],
        to: [1461, 2798],
    },
    HelpLabelSegment::Quad {
        control: [1669, 2798],
        to: [1669, 2564],
    },
    HelpLabelSegment::Quad {
        control: [1669, 2445],
        to: [1619, 2390],
    },
    HelpLabelSegment::Quad {
        control: [1570, 2335],
        to: [1462, 2335],
    },
    HelpLabelSegment::Quad {
        control: [1439, 2335],
        to: [1405, 2351],
    },
    HelpLabelSegment::Line([1351, 2388]),
];

const BODY_CONTOUR_117: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [1986, 2801],
        to: [2152, 2801],
    },
    HelpLabelSegment::Quad {
        control: [2231, 2801],
        to: [2275, 2738],
    },
    HelpLabelSegment::Quad {
        control: [2319, 2675],
        to: [2319, 2565],
    },
    HelpLabelSegment::Quad {
        control: [2319, 2332],
        to: [2152, 2332],
    },
    HelpLabelSegment::Quad {
        control: [2076, 2332],
        to: [2032, 2394],
    },
    HelpLabelSegment::Quad {
        control: [1986, 2456],
        to: [1986, 2565],
    },
];

const BODY_CONTOUR_118: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [2025, 2241],
        to: [2152, 2241],
    },
    HelpLabelSegment::Quad {
        control: [2287, 2241],
        to: [2362, 2327],
    },
    HelpLabelSegment::Quad {
        control: [2436, 2412],
        to: [2436, 2565],
    },
    HelpLabelSegment::Quad {
        control: [2436, 2717],
        to: [2360, 2805],
    },
    HelpLabelSegment::Quad {
        control: [2284, 2892],
        to: [2152, 2892],
    },
    HelpLabelSegment::Quad {
        control: [2019, 2892],
        to: [1944, 2804],
    },
    HelpLabelSegment::Quad {
        control: [1869, 2715],
        to: [1869, 2565],
    },
    HelpLabelSegment::Quad {
        control: [1869, 2419],
        to: [1947, 2330],
    },
];

const BODY_CONTOUR_119: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [2698, 2015],
        to: [2719, 2036],
    },
    HelpLabelSegment::Quad {
        control: [2739, 2056],
        to: [2739, 2084],
    },
    HelpLabelSegment::Quad {
        control: [2739, 2112],
        to: [2719, 2134],
    },
    HelpLabelSegment::Quad {
        control: [2698, 2153],
        to: [2670, 2153],
    },
    HelpLabelSegment::Quad {
        control: [2641, 2153],
        to: [2621, 2134],
    },
    HelpLabelSegment::Quad {
        control: [2600, 2112],
        to: [2600, 2084],
    },
    HelpLabelSegment::Quad {
        control: [2600, 2055],
        to: [2620, 2035],
    },
    HelpLabelSegment::Quad {
        control: [2640, 2015],
        to: [2670, 2015],
    },
];

const BODY_CONTOUR_120: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Line([2720, 2880]),
    HelpLabelSegment::Line([2609, 2880]),
    HelpLabelSegment::Line([2609, 2347]),
    HelpLabelSegment::Line([2522, 2347]),
    HelpLabelSegment::Line([2522, 2253]),
    HelpLabelSegment::Line([2720, 2253]),
];

const BODY_CONTOUR_121: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Line([3392, 2880]),
    HelpLabelSegment::Line([3280, 2880]),
    HelpLabelSegment::Line([3280, 2516]),
    HelpLabelSegment::Quad {
        control: [3280, 2415],
        to: [3251, 2375],
    },
    HelpLabelSegment::Quad {
        control: [3220, 2335],
        to: [3149, 2335],
    },
    HelpLabelSegment::Quad {
        control: [3111, 2335],
        to: [3069, 2357],
    },
    HelpLabelSegment::Quad {
        control: [3028, 2381],
        to: [3006, 2414],
    },
    HelpLabelSegment::Line([3006, 2880]),
    HelpLabelSegment::Line([2895, 2880]),
    HelpLabelSegment::Line([2895, 2253]),
    HelpLabelSegment::Line([2971, 2253]),
    HelpLabelSegment::Line([3006, 2334]),
    HelpLabelSegment::Quad {
        control: [3061, 2241],
        to: [3185, 2241],
    },
    HelpLabelSegment::Quad {
        control: [3392, 2241],
        to: [3392, 2492],
    },
];

const BODY_CONTOUR_122: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Line([3590, 2122]),
    HelpLabelSegment::Line([3701, 2078]),
    HelpLabelSegment::Line([3701, 2253]),
    HelpLabelSegment::Line([3873, 2253]),
    HelpLabelSegment::Line([3873, 2341]),
    HelpLabelSegment::Line([3701, 2341]),
    HelpLabelSegment::Line([3701, 2653]),
    HelpLabelSegment::Quad {
        control: [3701, 2731],
        to: [3728, 2765],
    },
    HelpLabelSegment::Quad {
        control: [3754, 2798],
        to: [3813, 2798],
    },
    HelpLabelSegment::Quad {
        control: [3856, 2798],
        to: [3901, 2777],
    },
    HelpLabelSegment::Line([3918, 2874]),
    HelpLabelSegment::Line([3766, 2892]),
    HelpLabelSegment::Quad {
        control: [3691, 2892],
        to: [3641, 2837],
    },
    HelpLabelSegment::Quad {
        control: [3590, 2782],
        to: [3590, 2697],
    },
    HelpLabelSegment::Line([3590, 2341]),
    HelpLabelSegment::Line([3517, 2341]),
    HelpLabelSegment::Line([3517, 2253]),
    HelpLabelSegment::Line([3590, 2253]),
];

const BODY_CONTOUR_123: [HelpLabelSegment; 22] = [
    HelpLabelSegment::Line([4332, 2388]),
    HelpLabelSegment::Quad {
        control: [4266, 2335],
        to: [4199, 2335],
    },
    HelpLabelSegment::Quad {
        control: [4159, 2335],
        to: [4132, 2354],
    },
    HelpLabelSegment::Quad {
        control: [4104, 2373],
        to: [4104, 2401],
    },
    HelpLabelSegment::Quad {
        control: [4104, 2462],
        to: [4174, 2492],
    },
    HelpLabelSegment::Line([4253, 2528]),
    HelpLabelSegment::Quad {
        control: [4326, 2562],
        to: [4360, 2605],
    },
    HelpLabelSegment::Quad {
        control: [4393, 2648],
        to: [4393, 2712],
    },
    HelpLabelSegment::Quad {
        control: [4393, 2797],
        to: [4334, 2845],
    },
    HelpLabelSegment::Quad {
        control: [4274, 2892],
        to: [4170, 2892],
    },
    HelpLabelSegment::Quad {
        control: [4070, 2892],
        to: [3984, 2842],
    },
    HelpLabelSegment::Line([4022, 2737]),
    HelpLabelSegment::Quad {
        control: [4116, 2798],
        to: [4172, 2798],
    },
    HelpLabelSegment::Quad {
        control: [4275, 2798],
        to: [4275, 2711],
    },
    HelpLabelSegment::Quad {
        control: [4275, 2649],
        to: [4176, 2605],
    },
    HelpLabelSegment::Quad {
        control: [4100, 2569],
        to: [4073, 2552],
    },
    HelpLabelSegment::Quad {
        control: [4046, 2533],
        to: [4027, 2511],
    },
    HelpLabelSegment::Quad {
        control: [4007, 2487],
        to: [3998, 2462],
    },
    HelpLabelSegment::Quad {
        control: [3987, 2435],
        to: [3987, 2405],
    },
    HelpLabelSegment::Quad {
        control: [3987, 2328],
        to: [4043, 2285],
    },
    HelpLabelSegment::Quad {
        control: [4100, 2241],
        to: [4191, 2241],
    },
    HelpLabelSegment::Quad {
        control: [4259, 2241],
        to: [4363, 2285],
    },
];

const BODY_CONTOUR_124: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Line([4910, 2122]),
    HelpLabelSegment::Line([5021, 2078]),
    HelpLabelSegment::Line([5021, 2253]),
    HelpLabelSegment::Line([5193, 2253]),
    HelpLabelSegment::Line([5193, 2341]),
    HelpLabelSegment::Line([5021, 2341]),
    HelpLabelSegment::Line([5021, 2653]),
    HelpLabelSegment::Quad {
        control: [5021, 2731],
        to: [5048, 2765],
    },
    HelpLabelSegment::Quad {
        control: [5074, 2798],
        to: [5133, 2798],
    },
    HelpLabelSegment::Quad {
        control: [5176, 2798],
        to: [5221, 2777],
    },
    HelpLabelSegment::Line([5238, 2874]),
    HelpLabelSegment::Line([5086, 2892]),
    HelpLabelSegment::Quad {
        control: [5011, 2892],
        to: [4961, 2837],
    },
    HelpLabelSegment::Quad {
        control: [4910, 2782],
        to: [4910, 2697],
    },
    HelpLabelSegment::Line([4910, 2341]),
    HelpLabelSegment::Line([4837, 2341]),
    HelpLabelSegment::Line([4837, 2253]),
    HelpLabelSegment::Line([4910, 2253]),
];

const BODY_CONTOUR_125: [HelpLabelSegment; 16] = [
    HelpLabelSegment::Line([5456, 2322]),
    HelpLabelSegment::Quad {
        control: [5478, 2287],
        to: [5527, 2265],
    },
    HelpLabelSegment::Quad {
        control: [5577, 2241],
        to: [5629, 2241],
    },
    HelpLabelSegment::Quad {
        control: [5729, 2241],
        to: [5786, 2307],
    },
    HelpLabelSegment::Quad {
        control: [5843, 2373],
        to: [5843, 2486],
    },
    HelpLabelSegment::Line([5843, 2880]),
    HelpLabelSegment::Line([5731, 2880]),
    HelpLabelSegment::Line([5731, 2486]),
    HelpLabelSegment::Quad {
        control: [5731, 2416],
        to: [5696, 2375],
    },
    HelpLabelSegment::Quad {
        control: [5662, 2335],
        to: [5599, 2335],
    },
    HelpLabelSegment::Quad {
        control: [5559, 2335],
        to: [5518, 2359],
    },
    HelpLabelSegment::Quad {
        control: [5477, 2382],
        to: [5456, 2414],
    },
    HelpLabelSegment::Line([5456, 2880]),
    HelpLabelSegment::Line([5345, 2880]),
    HelpLabelSegment::Line([5345, 1995]),
    HelpLabelSegment::Line([5456, 1995]),
];

const BODY_CONTOUR_126: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Quad {
        control: [6331, 2241],
        to: [6393, 2303],
    },
    HelpLabelSegment::Quad {
        control: [6454, 2366],
        to: [6454, 2500],
    },
    HelpLabelSegment::Line([6454, 2725]),
    HelpLabelSegment::Quad {
        control: [6454, 2809],
        to: [6504, 2835],
    },
    HelpLabelSegment::Line([6504, 2892]),
    HelpLabelSegment::Quad {
        control: [6436, 2892],
        to: [6403, 2872],
    },
    HelpLabelSegment::Quad {
        control: [6369, 2853],
        to: [6354, 2809],
    },
    HelpLabelSegment::Quad {
        control: [6287, 2892],
        to: [6150, 2892],
    },
    HelpLabelSegment::Quad {
        control: [6076, 2892],
        to: [6022, 2839],
    },
    HelpLabelSegment::Quad {
        control: [5967, 2785],
        to: [5967, 2705],
    },
    HelpLabelSegment::Quad {
        control: [5967, 2609],
        to: [6051, 2544],
    },
    HelpLabelSegment::Quad {
        control: [6134, 2478],
        to: [6263, 2478],
    },
    HelpLabelSegment::Line([6343, 2493]),
    HelpLabelSegment::Quad {
        control: [6343, 2341],
        to: [6207, 2341],
    },
    HelpLabelSegment::Quad {
        control: [6103, 2341],
        to: [6047, 2397],
    },
    HelpLabelSegment::Line([6000, 2303]),
    HelpLabelSegment::Quad {
        control: [6031, 2278],
        to: [6088, 2260],
    },
    HelpLabelSegment::Quad {
        control: [6144, 2241],
        to: [6194, 2241],
    },
];

const BODY_CONTOUR_127: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [6185, 2560],
        to: [6132, 2603],
    },
    HelpLabelSegment::Quad {
        control: [6078, 2647],
        to: [6078, 2707],
    },
    HelpLabelSegment::Quad {
        control: [6078, 2804],
        to: [6194, 2804],
    },
    HelpLabelSegment::Quad {
        control: [6279, 2804],
        to: [6343, 2724],
    },
    HelpLabelSegment::Line([6343, 2572]),
    HelpLabelSegment::Line([6269, 2560]),
];

const BODY_CONTOUR_128: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Line([7127, 2880]),
    HelpLabelSegment::Line([7015, 2880]),
    HelpLabelSegment::Line([7015, 2516]),
    HelpLabelSegment::Quad {
        control: [7015, 2415],
        to: [6986, 2375],
    },
    HelpLabelSegment::Quad {
        control: [6955, 2335],
        to: [6884, 2335],
    },
    HelpLabelSegment::Quad {
        control: [6846, 2335],
        to: [6804, 2357],
    },
    HelpLabelSegment::Quad {
        control: [6763, 2381],
        to: [6741, 2414],
    },
    HelpLabelSegment::Line([6741, 2880]),
    HelpLabelSegment::Line([6630, 2880]),
    HelpLabelSegment::Line([6630, 2253]),
    HelpLabelSegment::Line([6706, 2253]),
    HelpLabelSegment::Line([6741, 2334]),
    HelpLabelSegment::Quad {
        control: [6796, 2241],
        to: [6920, 2241],
    },
    HelpLabelSegment::Quad {
        control: [7127, 2241],
        to: [7127, 2492],
    },
];

const BODY_CONTOUR_129: [HelpLabelSegment; 22] = [
    HelpLabelSegment::Line([7952, 2388]),
    HelpLabelSegment::Quad {
        control: [7886, 2335],
        to: [7819, 2335],
    },
    HelpLabelSegment::Quad {
        control: [7779, 2335],
        to: [7753, 2354],
    },
    HelpLabelSegment::Quad {
        control: [7724, 2373],
        to: [7724, 2401],
    },
    HelpLabelSegment::Quad {
        control: [7724, 2462],
        to: [7794, 2492],
    },
    HelpLabelSegment::Line([7873, 2528]),
    HelpLabelSegment::Quad {
        control: [7946, 2562],
        to: [7980, 2605],
    },
    HelpLabelSegment::Quad {
        control: [8013, 2648],
        to: [8013, 2712],
    },
    HelpLabelSegment::Quad {
        control: [8013, 2797],
        to: [7954, 2845],
    },
    HelpLabelSegment::Quad {
        control: [7894, 2892],
        to: [7790, 2892],
    },
    HelpLabelSegment::Quad {
        control: [7690, 2892],
        to: [7604, 2842],
    },
    HelpLabelSegment::Line([7642, 2737]),
    HelpLabelSegment::Quad {
        control: [7736, 2798],
        to: [7792, 2798],
    },
    HelpLabelSegment::Quad {
        control: [7895, 2798],
        to: [7895, 2711],
    },
    HelpLabelSegment::Quad {
        control: [7895, 2649],
        to: [7796, 2605],
    },
    HelpLabelSegment::Quad {
        control: [7720, 2569],
        to: [7693, 2552],
    },
    HelpLabelSegment::Quad {
        control: [7666, 2533],
        to: [7647, 2511],
    },
    HelpLabelSegment::Quad {
        control: [7627, 2487],
        to: [7618, 2462],
    },
    HelpLabelSegment::Quad {
        control: [7607, 2435],
        to: [7607, 2405],
    },
    HelpLabelSegment::Quad {
        control: [7607, 2328],
        to: [7663, 2285],
    },
    HelpLabelSegment::Quad {
        control: [7720, 2241],
        to: [7811, 2241],
    },
    HelpLabelSegment::Quad {
        control: [7879, 2241],
        to: [7983, 2285],
    },
];

const BODY_CONTOUR_130: [HelpLabelSegment; 24] = [
    HelpLabelSegment::Quad {
        control: [8968, 2356],
        to: [8968, 2460],
    },
    HelpLabelSegment::Line([8968, 2880]),
    HelpLabelSegment::Line([8856, 2880]),
    HelpLabelSegment::Line([8856, 2483]),
    HelpLabelSegment::Quad {
        control: [8856, 2335],
        to: [8727, 2335],
    },
    HelpLabelSegment::Quad {
        control: [8688, 2335],
        to: [8652, 2360],
    },
    HelpLabelSegment::Quad {
        control: [8617, 2384],
        to: [8604, 2416],
    },
    HelpLabelSegment::Line([8604, 2880]),
    HelpLabelSegment::Line([8493, 2880]),
    HelpLabelSegment::Line([8493, 2435]),
    HelpLabelSegment::Quad {
        control: [8493, 2388],
        to: [8458, 2362],
    },
    HelpLabelSegment::Quad {
        control: [8423, 2335],
        to: [8365, 2335],
    },
    HelpLabelSegment::Quad {
        control: [8332, 2335],
        to: [8295, 2361],
    },
    HelpLabelSegment::Quad {
        control: [8256, 2387],
        to: [8241, 2417],
    },
    HelpLabelSegment::Line([8241, 2880]),
    HelpLabelSegment::Line([8130, 2880]),
    HelpLabelSegment::Line([8130, 2253]),
    HelpLabelSegment::Line([8202, 2253]),
    HelpLabelSegment::Line([8239, 2326]),
    HelpLabelSegment::Quad {
        control: [8303, 2241],
        to: [8400, 2241],
    },
    HelpLabelSegment::Quad {
        control: [8535, 2241],
        to: [8589, 2325],
    },
    HelpLabelSegment::Quad {
        control: [8608, 2289],
        to: [8658, 2265],
    },
    HelpLabelSegment::Quad {
        control: [8710, 2241],
        to: [8764, 2241],
    },
    HelpLabelSegment::Quad {
        control: [8861, 2241],
        to: [8914, 2299],
    },
];

const BODY_CONTOUR_131: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Quad {
        control: [9456, 2241],
        to: [9518, 2303],
    },
    HelpLabelSegment::Quad {
        control: [9579, 2366],
        to: [9579, 2500],
    },
    HelpLabelSegment::Line([9579, 2725]),
    HelpLabelSegment::Quad {
        control: [9579, 2809],
        to: [9629, 2835],
    },
    HelpLabelSegment::Line([9629, 2892]),
    HelpLabelSegment::Quad {
        control: [9561, 2892],
        to: [9528, 2872],
    },
    HelpLabelSegment::Quad {
        control: [9494, 2853],
        to: [9479, 2809],
    },
    HelpLabelSegment::Quad {
        control: [9412, 2892],
        to: [9275, 2892],
    },
    HelpLabelSegment::Quad {
        control: [9201, 2892],
        to: [9147, 2839],
    },
    HelpLabelSegment::Quad {
        control: [9092, 2785],
        to: [9092, 2705],
    },
    HelpLabelSegment::Quad {
        control: [9092, 2609],
        to: [9176, 2544],
    },
    HelpLabelSegment::Quad {
        control: [9259, 2478],
        to: [9388, 2478],
    },
    HelpLabelSegment::Line([9468, 2493]),
    HelpLabelSegment::Quad {
        control: [9468, 2341],
        to: [9332, 2341],
    },
    HelpLabelSegment::Quad {
        control: [9228, 2341],
        to: [9172, 2397],
    },
    HelpLabelSegment::Line([9125, 2303]),
    HelpLabelSegment::Quad {
        control: [9156, 2278],
        to: [9213, 2260],
    },
    HelpLabelSegment::Quad {
        control: [9269, 2241],
        to: [9319, 2241],
    },
];

const BODY_CONTOUR_132: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [9310, 2560],
        to: [9257, 2603],
    },
    HelpLabelSegment::Quad {
        control: [9203, 2647],
        to: [9203, 2707],
    },
    HelpLabelSegment::Quad {
        control: [9203, 2804],
        to: [9319, 2804],
    },
    HelpLabelSegment::Quad {
        control: [9404, 2804],
        to: [9468, 2724],
    },
    HelpLabelSegment::Line([9468, 2572]),
    HelpLabelSegment::Line([9394, 2560]),
];

const BODY_CONTOUR_133: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([9980, 2892]),
    HelpLabelSegment::Quad {
        control: [9763, 2892],
        to: [9763, 2703],
    },
    HelpLabelSegment::Line([9763, 1995]),
    HelpLabelSegment::Line([9874, 1995]),
    HelpLabelSegment::Line([9874, 2684]),
    HelpLabelSegment::Quad {
        control: [9874, 2735],
        to: [9904, 2764],
    },
    HelpLabelSegment::Quad {
        control: [9933, 2792],
        to: [9980, 2792],
    },
];

const BODY_CONTOUR_134: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([10335, 2892]),
    HelpLabelSegment::Quad {
        control: [10118, 2892],
        to: [10118, 2703],
    },
    HelpLabelSegment::Line([10118, 1995]),
    HelpLabelSegment::Line([10229, 1995]),
    HelpLabelSegment::Line([10229, 2684]),
    HelpLabelSegment::Quad {
        control: [10229, 2735],
        to: [10259, 2764],
    },
    HelpLabelSegment::Quad {
        control: [10288, 2792],
        to: [10335, 2792],
    },
];

const BODY_CONTOUR_135: [HelpLabelSegment; 13] = [
    HelpLabelSegment::Line([10936, 2299]),
    HelpLabelSegment::Quad {
        control: [10951, 2278],
        to: [10996, 2259],
    },
    HelpLabelSegment::Quad {
        control: [11039, 2241],
        to: [11081, 2241],
    },
    HelpLabelSegment::Quad {
        control: [11210, 2241],
        to: [11290, 2330],
    },
    HelpLabelSegment::Quad {
        control: [11370, 2419],
        to: [11370, 2555],
    },
    HelpLabelSegment::Quad {
        control: [11370, 2712],
        to: [11290, 2803],
    },
    HelpLabelSegment::Quad {
        control: [11209, 2892],
        to: [11072, 2892],
    },
    HelpLabelSegment::Quad {
        control: [11027, 2892],
        to: [10985, 2875],
    },
    HelpLabelSegment::Quad {
        control: [10942, 2859],
        to: [10920, 2835],
    },
    HelpLabelSegment::Line([10880, 2892]),
    HelpLabelSegment::Line([10825, 2892]),
    HelpLabelSegment::Line([10825, 1995]),
    HelpLabelSegment::Line([10936, 1995]),
];

const BODY_CONTOUR_136: [HelpLabelSegment; 9] = [
    HelpLabelSegment::Line([10936, 2746]),
    HelpLabelSegment::Quad {
        control: [10936, 2756],
        to: [10977, 2777],
    },
    HelpLabelSegment::Quad {
        control: [11019, 2798],
        to: [11040, 2798],
    },
    HelpLabelSegment::Quad {
        control: [11154, 2798],
        to: [11203, 2744],
    },
    HelpLabelSegment::Quad {
        control: [11252, 2689],
        to: [11252, 2561],
    },
    HelpLabelSegment::Quad {
        control: [11252, 2455],
        to: [11195, 2395],
    },
    HelpLabelSegment::Quad {
        control: [11138, 2335],
        to: [11040, 2335],
    },
    HelpLabelSegment::Quad {
        control: [11020, 2335],
        to: [10984, 2353],
    },
    HelpLabelSegment::Line([10936, 2384]),
];

const BODY_CONTOUR_137: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Quad {
        control: [11826, 2241],
        to: [11888, 2303],
    },
    HelpLabelSegment::Quad {
        control: [11949, 2366],
        to: [11949, 2500],
    },
    HelpLabelSegment::Line([11949, 2725]),
    HelpLabelSegment::Quad {
        control: [11949, 2809],
        to: [11999, 2835],
    },
    HelpLabelSegment::Line([11999, 2892]),
    HelpLabelSegment::Quad {
        control: [11931, 2892],
        to: [11898, 2872],
    },
    HelpLabelSegment::Quad {
        control: [11864, 2853],
        to: [11849, 2809],
    },
    HelpLabelSegment::Quad {
        control: [11782, 2892],
        to: [11645, 2892],
    },
    HelpLabelSegment::Quad {
        control: [11571, 2892],
        to: [11517, 2839],
    },
    HelpLabelSegment::Quad {
        control: [11462, 2785],
        to: [11462, 2705],
    },
    HelpLabelSegment::Quad {
        control: [11462, 2609],
        to: [11546, 2544],
    },
    HelpLabelSegment::Quad {
        control: [11629, 2478],
        to: [11758, 2478],
    },
    HelpLabelSegment::Line([11838, 2493]),
    HelpLabelSegment::Quad {
        control: [11838, 2341],
        to: [11702, 2341],
    },
    HelpLabelSegment::Quad {
        control: [11598, 2341],
        to: [11542, 2397],
    },
    HelpLabelSegment::Line([11495, 2303]),
    HelpLabelSegment::Quad {
        control: [11526, 2278],
        to: [11583, 2260],
    },
    HelpLabelSegment::Quad {
        control: [11639, 2241],
        to: [11689, 2241],
    },
];

const BODY_CONTOUR_138: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [11680, 2560],
        to: [11627, 2603],
    },
    HelpLabelSegment::Quad {
        control: [11573, 2647],
        to: [11573, 2707],
    },
    HelpLabelSegment::Quad {
        control: [11573, 2804],
        to: [11689, 2804],
    },
    HelpLabelSegment::Quad {
        control: [11774, 2804],
        to: [11838, 2724],
    },
    HelpLabelSegment::Line([11838, 2572]),
    HelpLabelSegment::Line([11764, 2560]),
];

const BODY_CONTOUR_139: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([12350, 2892]),
    HelpLabelSegment::Quad {
        control: [12133, 2892],
        to: [12133, 2703],
    },
    HelpLabelSegment::Line([12133, 1995]),
    HelpLabelSegment::Line([12244, 1995]),
    HelpLabelSegment::Line([12244, 2684]),
    HelpLabelSegment::Quad {
        control: [12244, 2735],
        to: [12274, 2764],
    },
    HelpLabelSegment::Quad {
        control: [12303, 2792],
        to: [12350, 2792],
    },
];

const BODY_CONTOUR_140: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([12705, 2892]),
    HelpLabelSegment::Quad {
        control: [12488, 2892],
        to: [12488, 2703],
    },
    HelpLabelSegment::Line([12488, 1995]),
    HelpLabelSegment::Line([12599, 1995]),
    HelpLabelSegment::Line([12599, 2684]),
    HelpLabelSegment::Quad {
        control: [12599, 2735],
        to: [12629, 2764],
    },
    HelpLabelSegment::Quad {
        control: [12658, 2792],
        to: [12705, 2792],
    },
];

const BODY_CONTOUR_141: [HelpLabelSegment; 22] = [
    HelpLabelSegment::Line([13142, 2388]),
    HelpLabelSegment::Quad {
        control: [13076, 2335],
        to: [13009, 2335],
    },
    HelpLabelSegment::Quad {
        control: [12969, 2335],
        to: [12943, 2354],
    },
    HelpLabelSegment::Quad {
        control: [12914, 2373],
        to: [12914, 2401],
    },
    HelpLabelSegment::Quad {
        control: [12914, 2462],
        to: [12984, 2492],
    },
    HelpLabelSegment::Line([13063, 2528]),
    HelpLabelSegment::Quad {
        control: [13136, 2562],
        to: [13170, 2605],
    },
    HelpLabelSegment::Quad {
        control: [13203, 2648],
        to: [13203, 2712],
    },
    HelpLabelSegment::Quad {
        control: [13203, 2797],
        to: [13144, 2845],
    },
    HelpLabelSegment::Quad {
        control: [13084, 2892],
        to: [12980, 2892],
    },
    HelpLabelSegment::Quad {
        control: [12880, 2892],
        to: [12794, 2842],
    },
    HelpLabelSegment::Line([12832, 2737]),
    HelpLabelSegment::Quad {
        control: [12926, 2798],
        to: [12982, 2798],
    },
    HelpLabelSegment::Quad {
        control: [13085, 2798],
        to: [13085, 2711],
    },
    HelpLabelSegment::Quad {
        control: [13085, 2649],
        to: [12986, 2605],
    },
    HelpLabelSegment::Quad {
        control: [12910, 2569],
        to: [12883, 2552],
    },
    HelpLabelSegment::Quad {
        control: [12856, 2533],
        to: [12837, 2511],
    },
    HelpLabelSegment::Quad {
        control: [12817, 2487],
        to: [12808, 2462],
    },
    HelpLabelSegment::Quad {
        control: [12797, 2435],
        to: [12797, 2405],
    },
    HelpLabelSegment::Quad {
        control: [12797, 2328],
        to: [12853, 2285],
    },
    HelpLabelSegment::Quad {
        control: [12910, 2241],
        to: [13001, 2241],
    },
    HelpLabelSegment::Quad {
        control: [13069, 2241],
        to: [13173, 2285],
    },
];

const BODY_CONTOUR_142: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [13484, 2716],
        to: [13510, 2742],
    },
    HelpLabelSegment::Quad {
        control: [13535, 2768],
        to: [13535, 2804],
    },
    HelpLabelSegment::Quad {
        control: [13535, 2840],
        to: [13510, 2866],
    },
    HelpLabelSegment::Quad {
        control: [13484, 2892],
        to: [13447, 2892],
    },
    HelpLabelSegment::Quad {
        control: [13411, 2892],
        to: [13385, 2866],
    },
    HelpLabelSegment::Quad {
        control: [13360, 2840],
        to: [13360, 2804],
    },
    HelpLabelSegment::Quad {
        control: [13360, 2768],
        to: [13385, 2742],
    },
    HelpLabelSegment::Quad {
        control: [13411, 2716],
        to: [13447, 2716],
    },
];

const BODY_CONTOUR_143: [HelpLabelSegment; 23] = [
    HelpLabelSegment::Quad {
        control: [-10907, 3826],
        to: [-10907, 3920],
    },
    HelpLabelSegment::Quad {
        control: [-10907, 3992],
        to: [-10946, 4047],
    },
    HelpLabelSegment::Quad {
        control: [-10986, 4102],
        to: [-11040, 4121],
    },
    HelpLabelSegment::Quad {
        control: [-10966, 4145],
        to: [-10921, 4203],
    },
    HelpLabelSegment::Quad {
        control: [-10878, 4260],
        to: [-10878, 4342],
    },
    HelpLabelSegment::Quad {
        control: [-10878, 4462],
        to: [-10954, 4527],
    },
    HelpLabelSegment::Quad {
        control: [-11029, 4592],
        to: [-11166, 4592],
    },
    HelpLabelSegment::Quad {
        control: [-11224, 4592],
        to: [-11279, 4571],
    },
    HelpLabelSegment::Quad {
        control: [-11334, 4550],
        to: [-11366, 4519],
    },
    HelpLabelSegment::Line([-11309, 4429]),
    HelpLabelSegment::Quad {
        control: [-11252, 4492],
        to: [-11164, 4492],
    },
    HelpLabelSegment::Quad {
        control: [-11001, 4492],
        to: [-11001, 4333],
    },
    HelpLabelSegment::Quad {
        control: [-11001, 4260],
        to: [-11049, 4216],
    },
    HelpLabelSegment::Quad {
        control: [-11096, 4170],
        to: [-11175, 4170],
    },
    HelpLabelSegment::Line([-11184, 4170]),
    HelpLabelSegment::Line([-11184, 4075]),
    HelpLabelSegment::Line([-11179, 4075]),
    HelpLabelSegment::Quad {
        control: [-11030, 4075],
        to: [-11030, 3944],
    },
    HelpLabelSegment::Quad {
        control: [-11030, 3807],
        to: [-11170, 3807],
    },
    HelpLabelSegment::Quad {
        control: [-11246, 3807],
        to: [-11292, 3857],
    },
    HelpLabelSegment::Line([-11343, 3777]),
    HelpLabelSegment::Quad {
        control: [-11289, 3707],
        to: [-11162, 3707],
    },
    HelpLabelSegment::Quad {
        control: [-11049, 3707],
        to: [-10978, 3767],
    },
];

const BODY_CONTOUR_144: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [-10556, 4416],
        to: [-10530, 4442],
    },
    HelpLabelSegment::Quad {
        control: [-10505, 4468],
        to: [-10505, 4504],
    },
    HelpLabelSegment::Quad {
        control: [-10505, 4540],
        to: [-10530, 4566],
    },
    HelpLabelSegment::Quad {
        control: [-10556, 4592],
        to: [-10593, 4592],
    },
    HelpLabelSegment::Quad {
        control: [-10629, 4592],
        to: [-10655, 4566],
    },
    HelpLabelSegment::Quad {
        control: [-10680, 4540],
        to: [-10680, 4504],
    },
    HelpLabelSegment::Quad {
        control: [-10680, 4468],
        to: [-10655, 4442],
    },
    HelpLabelSegment::Quad {
        control: [-10629, 4416],
        to: [-10593, 4416],
    },
];

const BODY_CONTOUR_145: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Quad {
        control: [-9420, 3986],
        to: [-9464, 4035],
    },
    HelpLabelSegment::Quad {
        control: [-9509, 4084],
        to: [-9563, 4096],
    },
    HelpLabelSegment::Quad {
        control: [-9461, 4121],
        to: [-9414, 4178],
    },
    HelpLabelSegment::Quad {
        control: [-9367, 4234],
        to: [-9367, 4333],
    },
    HelpLabelSegment::Quad {
        control: [-9367, 4445],
        to: [-9450, 4513],
    },
    HelpLabelSegment::Quad {
        control: [-9534, 4580],
        to: [-9667, 4580],
    },
    HelpLabelSegment::Line([-9912, 4580]),
    HelpLabelSegment::Line([-9912, 3722]),
    HelpLabelSegment::Line([-9686, 3714]),
    HelpLabelSegment::Quad {
        control: [-9559, 3714],
        to: [-9489, 3769],
    },
    HelpLabelSegment::Quad {
        control: [-9420, 3824],
        to: [-9420, 3926],
    },
];

const BODY_CONTOUR_146: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Line([-9795, 4062]),
    HelpLabelSegment::Line([-9704, 4066]),
    HelpLabelSegment::Quad {
        control: [-9537, 4066],
        to: [-9537, 3931],
    },
    HelpLabelSegment::Quad {
        control: [-9537, 3811],
        to: [-9689, 3811],
    },
    HelpLabelSegment::Line([-9795, 3816]),
];

const BODY_CONTOUR_147: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([-9795, 4480]),
    HelpLabelSegment::Line([-9702, 4486]),
    HelpLabelSegment::Quad {
        control: [-9592, 4486],
        to: [-9541, 4445],
    },
    HelpLabelSegment::Quad {
        control: [-9490, 4404],
        to: [-9490, 4314],
    },
    HelpLabelSegment::Quad {
        control: [-9490, 4230],
        to: [-9538, 4190],
    },
    HelpLabelSegment::Quad {
        control: [-9588, 4150],
        to: [-9700, 4150],
    },
    HelpLabelSegment::Line([-9795, 4153]),
];

const BODY_CONTOUR_148: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Quad {
        control: [-8909, 3941],
        to: [-8847, 4003],
    },
    HelpLabelSegment::Quad {
        control: [-8786, 4066],
        to: [-8786, 4200],
    },
    HelpLabelSegment::Line([-8786, 4425]),
    HelpLabelSegment::Quad {
        control: [-8786, 4509],
        to: [-8736, 4535],
    },
    HelpLabelSegment::Line([-8736, 4592]),
    HelpLabelSegment::Quad {
        control: [-8804, 4592],
        to: [-8837, 4572],
    },
    HelpLabelSegment::Quad {
        control: [-8871, 4553],
        to: [-8886, 4509],
    },
    HelpLabelSegment::Quad {
        control: [-8953, 4592],
        to: [-9090, 4592],
    },
    HelpLabelSegment::Quad {
        control: [-9164, 4592],
        to: [-9218, 4539],
    },
    HelpLabelSegment::Quad {
        control: [-9273, 4485],
        to: [-9273, 4405],
    },
    HelpLabelSegment::Quad {
        control: [-9273, 4309],
        to: [-9189, 4244],
    },
    HelpLabelSegment::Quad {
        control: [-9106, 4178],
        to: [-8977, 4178],
    },
    HelpLabelSegment::Line([-8897, 4193]),
    HelpLabelSegment::Quad {
        control: [-8897, 4041],
        to: [-9033, 4041],
    },
    HelpLabelSegment::Quad {
        control: [-9137, 4041],
        to: [-9193, 4097],
    },
    HelpLabelSegment::Line([-9240, 4003]),
    HelpLabelSegment::Quad {
        control: [-9209, 3978],
        to: [-9152, 3960],
    },
    HelpLabelSegment::Quad {
        control: [-9096, 3941],
        to: [-9046, 3941],
    },
];

const BODY_CONTOUR_149: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [-9055, 4260],
        to: [-9108, 4303],
    },
    HelpLabelSegment::Quad {
        control: [-9162, 4347],
        to: [-9162, 4407],
    },
    HelpLabelSegment::Quad {
        control: [-9162, 4504],
        to: [-9046, 4504],
    },
    HelpLabelSegment::Quad {
        control: [-8961, 4504],
        to: [-8897, 4424],
    },
    HelpLabelSegment::Line([-8897, 4272]),
    HelpLabelSegment::Line([-8971, 4260]),
];

const BODY_CONTOUR_150: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([-8385, 4592]),
    HelpLabelSegment::Quad {
        control: [-8602, 4592],
        to: [-8602, 4403],
    },
    HelpLabelSegment::Line([-8602, 3695]),
    HelpLabelSegment::Line([-8491, 3695]),
    HelpLabelSegment::Line([-8491, 4384]),
    HelpLabelSegment::Quad {
        control: [-8491, 4435],
        to: [-8461, 4464],
    },
    HelpLabelSegment::Quad {
        control: [-8432, 4492],
        to: [-8385, 4492],
    },
];

const BODY_CONTOUR_151: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([-8030, 4592]),
    HelpLabelSegment::Quad {
        control: [-8247, 4592],
        to: [-8247, 4403],
    },
    HelpLabelSegment::Line([-8247, 3695]),
    HelpLabelSegment::Line([-8136, 3695]),
    HelpLabelSegment::Line([-8136, 4384]),
    HelpLabelSegment::Quad {
        control: [-8136, 4435],
        to: [-8106, 4464],
    },
    HelpLabelSegment::Quad {
        control: [-8077, 4492],
        to: [-8030, 4492],
    },
];

const BODY_CONTOUR_152: [HelpLabelSegment; 22] = [
    HelpLabelSegment::Line([-7593, 4088]),
    HelpLabelSegment::Quad {
        control: [-7659, 4035],
        to: [-7726, 4035],
    },
    HelpLabelSegment::Quad {
        control: [-7766, 4035],
        to: [-7792, 4054],
    },
    HelpLabelSegment::Quad {
        control: [-7821, 4073],
        to: [-7821, 4101],
    },
    HelpLabelSegment::Quad {
        control: [-7821, 4162],
        to: [-7751, 4192],
    },
    HelpLabelSegment::Line([-7672, 4228]),
    HelpLabelSegment::Quad {
        control: [-7599, 4262],
        to: [-7565, 4305],
    },
    HelpLabelSegment::Quad {
        control: [-7532, 4348],
        to: [-7532, 4412],
    },
    HelpLabelSegment::Quad {
        control: [-7532, 4497],
        to: [-7591, 4545],
    },
    HelpLabelSegment::Quad {
        control: [-7651, 4592],
        to: [-7755, 4592],
    },
    HelpLabelSegment::Quad {
        control: [-7855, 4592],
        to: [-7941, 4542],
    },
    HelpLabelSegment::Line([-7903, 4437]),
    HelpLabelSegment::Quad {
        control: [-7809, 4498],
        to: [-7753, 4498],
    },
    HelpLabelSegment::Quad {
        control: [-7650, 4498],
        to: [-7650, 4411],
    },
    HelpLabelSegment::Quad {
        control: [-7650, 4349],
        to: [-7749, 4305],
    },
    HelpLabelSegment::Quad {
        control: [-7825, 4269],
        to: [-7852, 4252],
    },
    HelpLabelSegment::Quad {
        control: [-7879, 4233],
        to: [-7898, 4211],
    },
    HelpLabelSegment::Quad {
        control: [-7918, 4187],
        to: [-7927, 4162],
    },
    HelpLabelSegment::Quad {
        control: [-7938, 4135],
        to: [-7938, 4105],
    },
    HelpLabelSegment::Quad {
        control: [-7938, 4028],
        to: [-7882, 3985],
    },
    HelpLabelSegment::Quad {
        control: [-7825, 3941],
        to: [-7734, 3941],
    },
    HelpLabelSegment::Quad {
        control: [-7666, 3941],
        to: [-7562, 3985],
    },
];

const BODY_CONTOUR_153: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Quad {
        control: [-6724, 3941],
        to: [-6662, 4003],
    },
    HelpLabelSegment::Quad {
        control: [-6601, 4066],
        to: [-6601, 4200],
    },
    HelpLabelSegment::Line([-6601, 4425]),
    HelpLabelSegment::Quad {
        control: [-6601, 4509],
        to: [-6551, 4535],
    },
    HelpLabelSegment::Line([-6551, 4592]),
    HelpLabelSegment::Quad {
        control: [-6619, 4592],
        to: [-6652, 4572],
    },
    HelpLabelSegment::Quad {
        control: [-6686, 4553],
        to: [-6701, 4509],
    },
    HelpLabelSegment::Quad {
        control: [-6768, 4592],
        to: [-6905, 4592],
    },
    HelpLabelSegment::Quad {
        control: [-6979, 4592],
        to: [-7033, 4539],
    },
    HelpLabelSegment::Quad {
        control: [-7088, 4485],
        to: [-7088, 4405],
    },
    HelpLabelSegment::Quad {
        control: [-7088, 4309],
        to: [-7004, 4244],
    },
    HelpLabelSegment::Quad {
        control: [-6921, 4178],
        to: [-6792, 4178],
    },
    HelpLabelSegment::Line([-6712, 4193]),
    HelpLabelSegment::Quad {
        control: [-6712, 4041],
        to: [-6848, 4041],
    },
    HelpLabelSegment::Quad {
        control: [-6952, 4041],
        to: [-7008, 4097],
    },
    HelpLabelSegment::Line([-7055, 4003]),
    HelpLabelSegment::Quad {
        control: [-7024, 3978],
        to: [-6967, 3960],
    },
    HelpLabelSegment::Quad {
        control: [-6911, 3941],
        to: [-6861, 3941],
    },
];

const BODY_CONTOUR_154: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [-6870, 4260],
        to: [-6923, 4303],
    },
    HelpLabelSegment::Quad {
        control: [-6977, 4347],
        to: [-6977, 4407],
    },
    HelpLabelSegment::Quad {
        control: [-6977, 4504],
        to: [-6861, 4504],
    },
    HelpLabelSegment::Quad {
        control: [-6776, 4504],
        to: [-6712, 4424],
    },
    HelpLabelSegment::Line([-6712, 4272]),
    HelpLabelSegment::Line([-6786, 4260]),
];

const BODY_CONTOUR_155: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Quad {
        control: [-6121, 4035],
        to: [-6158, 4035],
    },
    HelpLabelSegment::Quad {
        control: [-6217, 4035],
        to: [-6261, 4089],
    },
    HelpLabelSegment::Quad {
        control: [-6306, 4144],
        to: [-6306, 4220],
    },
    HelpLabelSegment::Line([-6306, 4580]),
    HelpLabelSegment::Line([-6417, 4580]),
    HelpLabelSegment::Line([-6417, 3953]),
    HelpLabelSegment::Line([-6306, 3953]),
    HelpLabelSegment::Line([-6306, 4053]),
    HelpLabelSegment::Quad {
        control: [-6245, 3941],
        to: [-6124, 3941],
    },
    HelpLabelSegment::Line([-6039, 3952]),
    HelpLabelSegment::Line([-6084, 4060]),
];

const BODY_CONTOUR_156: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [-5776, 4035],
        to: [-5827, 4083],
    },
    HelpLabelSegment::Quad {
        control: [-5875, 4129],
        to: [-5882, 4197],
    },
    HelpLabelSegment::Line([-5534, 4197]),
    HelpLabelSegment::Quad {
        control: [-5534, 4129],
        to: [-5576, 4084],
    },
    HelpLabelSegment::Quad {
        control: [-5623, 4035],
        to: [-5702, 4035],
    },
];

const BODY_CONTOUR_157: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [-5770, 4498],
        to: [-5687, 4498],
    },
    HelpLabelSegment::Quad {
        control: [-5591, 4498],
        to: [-5528, 4443],
    },
    HelpLabelSegment::Line([-5481, 4523]),
    HelpLabelSegment::Quad {
        control: [-5507, 4548],
        to: [-5560, 4567],
    },
    HelpLabelSegment::Quad {
        control: [-5626, 4592],
        to: [-5708, 4592],
    },
    HelpLabelSegment::Quad {
        control: [-5827, 4592],
        to: [-5910, 4512],
    },
    HelpLabelSegment::Quad {
        control: [-6001, 4423],
        to: [-6001, 4274],
    },
    HelpLabelSegment::Quad {
        control: [-6001, 4118],
        to: [-5908, 4025],
    },
    HelpLabelSegment::Quad {
        control: [-5823, 3941],
        to: [-5707, 3941],
    },
    HelpLabelSegment::Quad {
        control: [-5574, 3941],
        to: [-5497, 4016],
    },
    HelpLabelSegment::Quad {
        control: [-5424, 4089],
        to: [-5424, 4210],
    },
    HelpLabelSegment::Quad {
        control: [-5424, 4246],
        to: [-5432, 4278],
    },
    HelpLabelSegment::Line([-5884, 4278]),
    HelpLabelSegment::Quad {
        control: [-5884, 4388],
        to: [-5824, 4446],
    },
];

const BODY_CONTOUR_158: [HelpLabelSegment; 22] = [
    HelpLabelSegment::Line([-4638, 4088]),
    HelpLabelSegment::Quad {
        control: [-4704, 4035],
        to: [-4771, 4035],
    },
    HelpLabelSegment::Quad {
        control: [-4811, 4035],
        to: [-4838, 4054],
    },
    HelpLabelSegment::Quad {
        control: [-4866, 4073],
        to: [-4866, 4101],
    },
    HelpLabelSegment::Quad {
        control: [-4866, 4162],
        to: [-4796, 4192],
    },
    HelpLabelSegment::Line([-4717, 4228]),
    HelpLabelSegment::Quad {
        control: [-4644, 4262],
        to: [-4610, 4305],
    },
    HelpLabelSegment::Quad {
        control: [-4577, 4348],
        to: [-4577, 4412],
    },
    HelpLabelSegment::Quad {
        control: [-4577, 4497],
        to: [-4636, 4545],
    },
    HelpLabelSegment::Quad {
        control: [-4696, 4592],
        to: [-4800, 4592],
    },
    HelpLabelSegment::Quad {
        control: [-4900, 4592],
        to: [-4986, 4542],
    },
    HelpLabelSegment::Line([-4948, 4437]),
    HelpLabelSegment::Quad {
        control: [-4854, 4498],
        to: [-4798, 4498],
    },
    HelpLabelSegment::Quad {
        control: [-4695, 4498],
        to: [-4695, 4411],
    },
    HelpLabelSegment::Quad {
        control: [-4695, 4349],
        to: [-4794, 4305],
    },
    HelpLabelSegment::Quad {
        control: [-4870, 4269],
        to: [-4897, 4252],
    },
    HelpLabelSegment::Quad {
        control: [-4924, 4233],
        to: [-4943, 4211],
    },
    HelpLabelSegment::Quad {
        control: [-4963, 4187],
        to: [-4972, 4162],
    },
    HelpLabelSegment::Quad {
        control: [-4983, 4135],
        to: [-4983, 4105],
    },
    HelpLabelSegment::Quad {
        control: [-4983, 4028],
        to: [-4927, 3985],
    },
    HelpLabelSegment::Quad {
        control: [-4870, 3941],
        to: [-4779, 3941],
    },
    HelpLabelSegment::Quad {
        control: [-4711, 3941],
        to: [-4607, 3985],
    },
];

const BODY_CONTOUR_159: [HelpLabelSegment; 15] = [
    HelpLabelSegment::Line([-4355, 4353]),
    HelpLabelSegment::Quad {
        control: [-4355, 4498],
        to: [-4229, 4498],
    },
    HelpLabelSegment::Quad {
        control: [-4174, 4498],
        to: [-4129, 4466],
    },
    HelpLabelSegment::Quad {
        control: [-4083, 4435],
        to: [-4068, 4394],
    },
    HelpLabelSegment::Line([-4068, 3953]),
    HelpLabelSegment::Line([-3956, 3953]),
    HelpLabelSegment::Line([-3956, 4580]),
    HelpLabelSegment::Line([-4068, 4580]),
    HelpLabelSegment::Line([-4068, 4493]),
    HelpLabelSegment::Quad {
        control: [-4086, 4531],
        to: [-4143, 4561],
    },
    HelpLabelSegment::Quad {
        control: [-4200, 4592],
        to: [-4254, 4592],
    },
    HelpLabelSegment::Quad {
        control: [-4357, 4592],
        to: [-4411, 4533],
    },
    HelpLabelSegment::Quad {
        control: [-4466, 4473],
        to: [-4466, 4364],
    },
    HelpLabelSegment::Line([-4466, 3953]),
    HelpLabelSegment::Line([-4355, 3953]),
];

const BODY_CONTOUR_160: [HelpLabelSegment; 13] = [
    HelpLabelSegment::Line([-3694, 3999]),
    HelpLabelSegment::Quad {
        control: [-3679, 3978],
        to: [-3634, 3959],
    },
    HelpLabelSegment::Quad {
        control: [-3591, 3941],
        to: [-3549, 3941],
    },
    HelpLabelSegment::Quad {
        control: [-3420, 3941],
        to: [-3340, 4030],
    },
    HelpLabelSegment::Quad {
        control: [-3260, 4119],
        to: [-3260, 4255],
    },
    HelpLabelSegment::Quad {
        control: [-3260, 4412],
        to: [-3340, 4503],
    },
    HelpLabelSegment::Quad {
        control: [-3421, 4592],
        to: [-3558, 4592],
    },
    HelpLabelSegment::Quad {
        control: [-3603, 4592],
        to: [-3645, 4575],
    },
    HelpLabelSegment::Quad {
        control: [-3688, 4559],
        to: [-3710, 4535],
    },
    HelpLabelSegment::Line([-3750, 4592]),
    HelpLabelSegment::Line([-3805, 4592]),
    HelpLabelSegment::Line([-3805, 3695]),
    HelpLabelSegment::Line([-3694, 3695]),
];

const BODY_CONTOUR_161: [HelpLabelSegment; 9] = [
    HelpLabelSegment::Line([-3694, 4446]),
    HelpLabelSegment::Quad {
        control: [-3694, 4456],
        to: [-3653, 4477],
    },
    HelpLabelSegment::Quad {
        control: [-3611, 4498],
        to: [-3590, 4498],
    },
    HelpLabelSegment::Quad {
        control: [-3476, 4498],
        to: [-3427, 4444],
    },
    HelpLabelSegment::Quad {
        control: [-3378, 4389],
        to: [-3378, 4261],
    },
    HelpLabelSegment::Quad {
        control: [-3378, 4155],
        to: [-3435, 4095],
    },
    HelpLabelSegment::Quad {
        control: [-3492, 4035],
        to: [-3590, 4035],
    },
    HelpLabelSegment::Quad {
        control: [-3610, 4035],
        to: [-3646, 4053],
    },
    HelpLabelSegment::Line([-3694, 4084]),
];

const BODY_CONTOUR_162: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [-2992, 3715],
        to: [-2964, 3715],
    },
    HelpLabelSegment::Quad {
        control: [-2935, 3715],
        to: [-2915, 3736],
    },
    HelpLabelSegment::Quad {
        control: [-2895, 3756],
        to: [-2895, 3784],
    },
    HelpLabelSegment::Quad {
        control: [-2895, 3812],
        to: [-2915, 3834],
    },
    HelpLabelSegment::Quad {
        control: [-2935, 3853],
        to: [-2964, 3853],
    },
    HelpLabelSegment::Quad {
        control: [-2992, 3853],
        to: [-3012, 3834],
    },
    HelpLabelSegment::Quad {
        control: [-3032, 3812],
        to: [-3032, 3784],
    },
    HelpLabelSegment::Quad {
        control: [-3032, 3755],
        to: [-3012, 3735],
    },
];

const BODY_CONTOUR_163: [HelpLabelSegment; 10] = [
    HelpLabelSegment::Line([-2893, 4575]),
    HelpLabelSegment::Quad {
        control: [-2893, 4704],
        to: [-2969, 4765],
    },
    HelpLabelSegment::Quad {
        control: [-3045, 4826],
        to: [-3204, 4826],
    },
    HelpLabelSegment::Line([-3204, 4726]),
    HelpLabelSegment::Quad {
        control: [-3090, 4726],
        to: [-3046, 4692],
    },
    HelpLabelSegment::Quad {
        control: [-3004, 4657],
        to: [-3004, 4578],
    },
    HelpLabelSegment::Line([-3004, 4047]),
    HelpLabelSegment::Line([-3129, 4047]),
    HelpLabelSegment::Line([-3129, 3953]),
    HelpLabelSegment::Line([-2893, 3953]),
];

const BODY_CONTOUR_164: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [-2511, 4035],
        to: [-2562, 4083],
    },
    HelpLabelSegment::Quad {
        control: [-2610, 4129],
        to: [-2617, 4197],
    },
    HelpLabelSegment::Line([-2269, 4197]),
    HelpLabelSegment::Quad {
        control: [-2269, 4129],
        to: [-2311, 4084],
    },
    HelpLabelSegment::Quad {
        control: [-2358, 4035],
        to: [-2438, 4035],
    },
];

const BODY_CONTOUR_165: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [-2505, 4498],
        to: [-2422, 4498],
    },
    HelpLabelSegment::Quad {
        control: [-2326, 4498],
        to: [-2263, 4443],
    },
    HelpLabelSegment::Line([-2216, 4523]),
    HelpLabelSegment::Quad {
        control: [-2242, 4548],
        to: [-2295, 4567],
    },
    HelpLabelSegment::Quad {
        control: [-2361, 4592],
        to: [-2443, 4592],
    },
    HelpLabelSegment::Quad {
        control: [-2562, 4592],
        to: [-2645, 4512],
    },
    HelpLabelSegment::Quad {
        control: [-2736, 4423],
        to: [-2736, 4274],
    },
    HelpLabelSegment::Quad {
        control: [-2736, 4118],
        to: [-2643, 4025],
    },
    HelpLabelSegment::Quad {
        control: [-2558, 3941],
        to: [-2442, 3941],
    },
    HelpLabelSegment::Quad {
        control: [-2309, 3941],
        to: [-2232, 4016],
    },
    HelpLabelSegment::Quad {
        control: [-2159, 4089],
        to: [-2159, 4210],
    },
    HelpLabelSegment::Quad {
        control: [-2159, 4246],
        to: [-2167, 4278],
    },
    HelpLabelSegment::Line([-2619, 4278]),
    HelpLabelSegment::Quad {
        control: [-2619, 4388],
        to: [-2559, 4446],
    },
];

const BODY_CONTOUR_166: [HelpLabelSegment; 16] = [
    HelpLabelSegment::Quad {
        control: [-1594, 3982],
        to: [-1567, 4003],
    },
    HelpLabelSegment::Line([-1622, 4082]),
    HelpLabelSegment::Quad {
        control: [-1640, 4066],
        to: [-1682, 4050],
    },
    HelpLabelSegment::Line([-1767, 4035]),
    HelpLabelSegment::Quad {
        control: [-1858, 4035],
        to: [-1911, 4098],
    },
    HelpLabelSegment::Quad {
        control: [-1964, 4162],
        to: [-1964, 4273],
    },
    HelpLabelSegment::Quad {
        control: [-1964, 4383],
        to: [-1910, 4441],
    },
    HelpLabelSegment::Quad {
        control: [-1855, 4498],
        to: [-1759, 4498],
    },
    HelpLabelSegment::Quad {
        control: [-1684, 4498],
        to: [-1608, 4441],
    },
    HelpLabelSegment::Line([-1563, 4534]),
    HelpLabelSegment::Quad {
        control: [-1654, 4592],
        to: [-1786, 4592],
    },
    HelpLabelSegment::Quad {
        control: [-1914, 4592],
        to: [-1998, 4506],
    },
    HelpLabelSegment::Quad {
        control: [-2081, 4419],
        to: [-2081, 4273],
    },
    HelpLabelSegment::Quad {
        control: [-2081, 4123],
        to: [-1995, 4032],
    },
    HelpLabelSegment::Quad {
        control: [-1908, 3941],
        to: [-1757, 3941],
    },
    HelpLabelSegment::Line([-1651, 3961]),
];

const BODY_CONTOUR_167: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Line([-1405, 3822]),
    HelpLabelSegment::Line([-1294, 3778]),
    HelpLabelSegment::Line([-1294, 3953]),
    HelpLabelSegment::Line([-1122, 3953]),
    HelpLabelSegment::Line([-1122, 4041]),
    HelpLabelSegment::Line([-1294, 4041]),
    HelpLabelSegment::Line([-1294, 4353]),
    HelpLabelSegment::Quad {
        control: [-1294, 4431],
        to: [-1267, 4465],
    },
    HelpLabelSegment::Quad {
        control: [-1241, 4498],
        to: [-1182, 4498],
    },
    HelpLabelSegment::Quad {
        control: [-1139, 4498],
        to: [-1094, 4477],
    },
    HelpLabelSegment::Line([-1077, 4574]),
    HelpLabelSegment::Line([-1229, 4592]),
    HelpLabelSegment::Quad {
        control: [-1304, 4592],
        to: [-1354, 4537],
    },
    HelpLabelSegment::Quad {
        control: [-1405, 4482],
        to: [-1405, 4397],
    },
    HelpLabelSegment::Line([-1405, 4041]),
    HelpLabelSegment::Line([-1478, 4041]),
    HelpLabelSegment::Line([-1478, 3953]),
    HelpLabelSegment::Line([-1405, 3953]),
];

const BODY_CONTOUR_168: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Line([-570, 3822]),
    HelpLabelSegment::Line([-459, 3778]),
    HelpLabelSegment::Line([-459, 3953]),
    HelpLabelSegment::Line([-287, 3953]),
    HelpLabelSegment::Line([-287, 4041]),
    HelpLabelSegment::Line([-459, 4041]),
    HelpLabelSegment::Line([-459, 4353]),
    HelpLabelSegment::Quad {
        control: [-459, 4431],
        to: [-432, 4465],
    },
    HelpLabelSegment::Quad {
        control: [-406, 4498],
        to: [-347, 4498],
    },
    HelpLabelSegment::Quad {
        control: [-304, 4498],
        to: [-259, 4477],
    },
    HelpLabelSegment::Line([-242, 4574]),
    HelpLabelSegment::Line([-394, 4592]),
    HelpLabelSegment::Quad {
        control: [-469, 4592],
        to: [-519, 4537],
    },
    HelpLabelSegment::Quad {
        control: [-570, 4482],
        to: [-570, 4397],
    },
    HelpLabelSegment::Line([-570, 4041]),
    HelpLabelSegment::Line([-643, 4041]),
    HelpLabelSegment::Line([-643, 3953]),
    HelpLabelSegment::Line([-570, 3953]),
];

const BODY_CONTOUR_169: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [-59, 4501],
        to: [107, 4501],
    },
    HelpLabelSegment::Quad {
        control: [186, 4501],
        to: [230, 4438],
    },
    HelpLabelSegment::Quad {
        control: [274, 4375],
        to: [274, 4265],
    },
    HelpLabelSegment::Quad {
        control: [274, 4032],
        to: [107, 4032],
    },
    HelpLabelSegment::Quad {
        control: [31, 4032],
        to: [-13, 4094],
    },
    HelpLabelSegment::Quad {
        control: [-59, 4156],
        to: [-59, 4265],
    },
];

const BODY_CONTOUR_170: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [-20, 3941],
        to: [107, 3941],
    },
    HelpLabelSegment::Quad {
        control: [242, 3941],
        to: [317, 4027],
    },
    HelpLabelSegment::Quad {
        control: [391, 4112],
        to: [391, 4265],
    },
    HelpLabelSegment::Quad {
        control: [391, 4417],
        to: [315, 4505],
    },
    HelpLabelSegment::Quad {
        control: [239, 4592],
        to: [107, 4592],
    },
    HelpLabelSegment::Quad {
        control: [-26, 4592],
        to: [-101, 4504],
    },
    HelpLabelSegment::Quad {
        control: [-176, 4415],
        to: [-176, 4265],
    },
    HelpLabelSegment::Quad {
        control: [-176, 4119],
        to: [-98, 4030],
    },
];

const BODY_CONTOUR_171: [HelpLabelSegment; 17] = [
    HelpLabelSegment::Quad {
        control: [920, 3832],
        to: [979, 3764],
    },
    HelpLabelSegment::Quad {
        control: [1038, 3695],
        to: [1136, 3695],
    },
    HelpLabelSegment::Line([1240, 3713]),
    HelpLabelSegment::Line([1208, 3795]),
    HelpLabelSegment::Line([1145, 3783]),
    HelpLabelSegment::Quad {
        control: [1095, 3783],
        to: [1062, 3822],
    },
    HelpLabelSegment::Quad {
        control: [1028, 3860],
        to: [1028, 3920],
    },
    HelpLabelSegment::Quad {
        control: [1028, 3935],
        to: [1031, 3953],
    },
    HelpLabelSegment::Line([1159, 3953]),
    HelpLabelSegment::Line([1159, 4047]),
    HelpLabelSegment::Line([1031, 4047]),
    HelpLabelSegment::Line([1031, 4580]),
    HelpLabelSegment::Line([920, 4580]),
    HelpLabelSegment::Line([920, 4047]),
    HelpLabelSegment::Line([829, 4047]),
    HelpLabelSegment::Line([829, 3953]),
    HelpLabelSegment::Line([920, 3953]),
];

const BODY_CONTOUR_172: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [1391, 4501],
        to: [1557, 4501],
    },
    HelpLabelSegment::Quad {
        control: [1636, 4501],
        to: [1680, 4438],
    },
    HelpLabelSegment::Quad {
        control: [1724, 4375],
        to: [1724, 4265],
    },
    HelpLabelSegment::Quad {
        control: [1724, 4032],
        to: [1557, 4032],
    },
    HelpLabelSegment::Quad {
        control: [1481, 4032],
        to: [1437, 4094],
    },
    HelpLabelSegment::Quad {
        control: [1391, 4156],
        to: [1391, 4265],
    },
];

const BODY_CONTOUR_173: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [1430, 3941],
        to: [1557, 3941],
    },
    HelpLabelSegment::Quad {
        control: [1692, 3941],
        to: [1767, 4027],
    },
    HelpLabelSegment::Quad {
        control: [1841, 4112],
        to: [1841, 4265],
    },
    HelpLabelSegment::Quad {
        control: [1841, 4417],
        to: [1765, 4505],
    },
    HelpLabelSegment::Quad {
        control: [1689, 4592],
        to: [1557, 4592],
    },
    HelpLabelSegment::Quad {
        control: [1424, 4592],
        to: [1349, 4504],
    },
    HelpLabelSegment::Quad {
        control: [1274, 4415],
        to: [1274, 4265],
    },
    HelpLabelSegment::Quad {
        control: [1274, 4119],
        to: [1352, 4030],
    },
];

const BODY_CONTOUR_174: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Quad {
        control: [2264, 4035],
        to: [2227, 4035],
    },
    HelpLabelSegment::Quad {
        control: [2168, 4035],
        to: [2124, 4089],
    },
    HelpLabelSegment::Quad {
        control: [2079, 4144],
        to: [2079, 4220],
    },
    HelpLabelSegment::Line([2079, 4580]),
    HelpLabelSegment::Line([1968, 4580]),
    HelpLabelSegment::Line([1968, 3953]),
    HelpLabelSegment::Line([2079, 3953]),
    HelpLabelSegment::Line([2079, 4053]),
    HelpLabelSegment::Quad {
        control: [2140, 3941],
        to: [2261, 3941],
    },
    HelpLabelSegment::Line([2346, 3952]),
    HelpLabelSegment::Line([2301, 4060]),
];

const BODY_CONTOUR_175: [HelpLabelSegment; 16] = [
    HelpLabelSegment::Quad {
        control: [2871, 3982],
        to: [2898, 4003],
    },
    HelpLabelSegment::Line([2843, 4082]),
    HelpLabelSegment::Quad {
        control: [2825, 4066],
        to: [2783, 4050],
    },
    HelpLabelSegment::Line([2698, 4035]),
    HelpLabelSegment::Quad {
        control: [2608, 4035],
        to: [2554, 4098],
    },
    HelpLabelSegment::Quad {
        control: [2501, 4162],
        to: [2501, 4273],
    },
    HelpLabelSegment::Quad {
        control: [2501, 4383],
        to: [2555, 4441],
    },
    HelpLabelSegment::Quad {
        control: [2610, 4498],
        to: [2706, 4498],
    },
    HelpLabelSegment::Quad {
        control: [2781, 4498],
        to: [2857, 4441],
    },
    HelpLabelSegment::Line([2902, 4534]),
    HelpLabelSegment::Quad {
        control: [2811, 4592],
        to: [2679, 4592],
    },
    HelpLabelSegment::Quad {
        control: [2551, 4592],
        to: [2467, 4506],
    },
    HelpLabelSegment::Quad {
        control: [2384, 4419],
        to: [2384, 4273],
    },
    HelpLabelSegment::Quad {
        control: [2384, 4123],
        to: [2470, 4032],
    },
    HelpLabelSegment::Quad {
        control: [2557, 3941],
        to: [2708, 3941],
    },
    HelpLabelSegment::Line([2814, 3961]),
];

const BODY_CONTOUR_176: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [3204, 4035],
        to: [3153, 4083],
    },
    HelpLabelSegment::Quad {
        control: [3105, 4129],
        to: [3098, 4197],
    },
    HelpLabelSegment::Line([3446, 4197]),
    HelpLabelSegment::Quad {
        control: [3446, 4129],
        to: [3404, 4084],
    },
    HelpLabelSegment::Quad {
        control: [3357, 4035],
        to: [3277, 4035],
    },
];

const BODY_CONTOUR_177: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [3210, 4498],
        to: [3293, 4498],
    },
    HelpLabelSegment::Quad {
        control: [3389, 4498],
        to: [3452, 4443],
    },
    HelpLabelSegment::Line([3499, 4523]),
    HelpLabelSegment::Quad {
        control: [3473, 4548],
        to: [3420, 4567],
    },
    HelpLabelSegment::Quad {
        control: [3354, 4592],
        to: [3272, 4592],
    },
    HelpLabelSegment::Quad {
        control: [3153, 4592],
        to: [3070, 4512],
    },
    HelpLabelSegment::Quad {
        control: [2979, 4423],
        to: [2979, 4274],
    },
    HelpLabelSegment::Quad {
        control: [2979, 4118],
        to: [3072, 4025],
    },
    HelpLabelSegment::Quad {
        control: [3157, 3941],
        to: [3273, 3941],
    },
    HelpLabelSegment::Quad {
        control: [3406, 3941],
        to: [3483, 4016],
    },
    HelpLabelSegment::Quad {
        control: [3556, 4089],
        to: [3556, 4210],
    },
    HelpLabelSegment::Quad {
        control: [3556, 4246],
        to: [3548, 4278],
    },
    HelpLabelSegment::Line([3096, 4278]),
    HelpLabelSegment::Quad {
        control: [3096, 4388],
        to: [3156, 4446],
    },
];

const BODY_CONTOUR_178: [HelpLabelSegment; 22] = [
    HelpLabelSegment::Line([3982, 4088]),
    HelpLabelSegment::Quad {
        control: [3916, 4035],
        to: [3849, 4035],
    },
    HelpLabelSegment::Quad {
        control: [3809, 4035],
        to: [3783, 4054],
    },
    HelpLabelSegment::Quad {
        control: [3754, 4073],
        to: [3754, 4101],
    },
    HelpLabelSegment::Quad {
        control: [3754, 4162],
        to: [3824, 4192],
    },
    HelpLabelSegment::Line([3903, 4228]),
    HelpLabelSegment::Quad {
        control: [3976, 4262],
        to: [4010, 4305],
    },
    HelpLabelSegment::Quad {
        control: [4043, 4348],
        to: [4043, 4412],
    },
    HelpLabelSegment::Quad {
        control: [4043, 4497],
        to: [3984, 4545],
    },
    HelpLabelSegment::Quad {
        control: [3924, 4592],
        to: [3820, 4592],
    },
    HelpLabelSegment::Quad {
        control: [3720, 4592],
        to: [3634, 4542],
    },
    HelpLabelSegment::Line([3672, 4437]),
    HelpLabelSegment::Quad {
        control: [3766, 4498],
        to: [3822, 4498],
    },
    HelpLabelSegment::Quad {
        control: [3925, 4498],
        to: [3925, 4411],
    },
    HelpLabelSegment::Quad {
        control: [3925, 4349],
        to: [3826, 4305],
    },
    HelpLabelSegment::Quad {
        control: [3750, 4269],
        to: [3723, 4252],
    },
    HelpLabelSegment::Quad {
        control: [3696, 4233],
        to: [3677, 4211],
    },
    HelpLabelSegment::Quad {
        control: [3657, 4187],
        to: [3648, 4162],
    },
    HelpLabelSegment::Quad {
        control: [3637, 4135],
        to: [3637, 4105],
    },
    HelpLabelSegment::Quad {
        control: [3637, 4028],
        to: [3693, 3985],
    },
    HelpLabelSegment::Quad {
        control: [3750, 3941],
        to: [3841, 3941],
    },
    HelpLabelSegment::Quad {
        control: [3909, 3941],
        to: [4013, 3985],
    },
];

const BODY_CONTOUR_179: [HelpLabelSegment; 10] = [
    HelpLabelSegment::Quad {
        control: [4382, 4589],
        to: [4350, 4653],
    },
    HelpLabelSegment::Quad {
        control: [4317, 4715],
        to: [4211, 4793],
    },
    HelpLabelSegment::Line([4182, 4751]),
    HelpLabelSegment::Quad {
        control: [4283, 4668],
        to: [4283, 4610],
    },
    HelpLabelSegment::Quad {
        control: [4283, 4585],
        to: [4265, 4559],
    },
    HelpLabelSegment::Quad {
        control: [4215, 4534],
        to: [4215, 4490],
    },
    HelpLabelSegment::Quad {
        control: [4215, 4458],
        to: [4238, 4438],
    },
    HelpLabelSegment::Quad {
        control: [4263, 4418],
        to: [4299, 4418],
    },
    HelpLabelSegment::Quad {
        control: [4331, 4418],
        to: [4357, 4446],
    },
    HelpLabelSegment::Quad {
        control: [4382, 4475],
        to: [4382, 4512],
    },
];

const BODY_CONTOUR_180: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Line([5000, 3822]),
    HelpLabelSegment::Line([5111, 3778]),
    HelpLabelSegment::Line([5111, 3953]),
    HelpLabelSegment::Line([5283, 3953]),
    HelpLabelSegment::Line([5283, 4041]),
    HelpLabelSegment::Line([5111, 4041]),
    HelpLabelSegment::Line([5111, 4353]),
    HelpLabelSegment::Quad {
        control: [5111, 4431],
        to: [5138, 4465],
    },
    HelpLabelSegment::Quad {
        control: [5164, 4498],
        to: [5223, 4498],
    },
    HelpLabelSegment::Quad {
        control: [5266, 4498],
        to: [5311, 4477],
    },
    HelpLabelSegment::Line([5328, 4574]),
    HelpLabelSegment::Line([5176, 4592]),
    HelpLabelSegment::Quad {
        control: [5101, 4592],
        to: [5051, 4537],
    },
    HelpLabelSegment::Quad {
        control: [5000, 4482],
        to: [5000, 4397],
    },
    HelpLabelSegment::Line([5000, 4041]),
    HelpLabelSegment::Line([4927, 4041]),
    HelpLabelSegment::Line([4927, 3953]),
    HelpLabelSegment::Line([5000, 3953]),
];

const BODY_CONTOUR_181: [HelpLabelSegment; 16] = [
    HelpLabelSegment::Line([5546, 4022]),
    HelpLabelSegment::Quad {
        control: [5568, 3987],
        to: [5618, 3965],
    },
    HelpLabelSegment::Quad {
        control: [5667, 3941],
        to: [5719, 3941],
    },
    HelpLabelSegment::Quad {
        control: [5819, 3941],
        to: [5876, 4007],
    },
    HelpLabelSegment::Quad {
        control: [5933, 4073],
        to: [5933, 4186],
    },
    HelpLabelSegment::Line([5933, 4580]),
    HelpLabelSegment::Line([5821, 4580]),
    HelpLabelSegment::Line([5821, 4186]),
    HelpLabelSegment::Quad {
        control: [5821, 4116],
        to: [5786, 4075],
    },
    HelpLabelSegment::Quad {
        control: [5752, 4035],
        to: [5689, 4035],
    },
    HelpLabelSegment::Quad {
        control: [5649, 4035],
        to: [5608, 4059],
    },
    HelpLabelSegment::Quad {
        control: [5567, 4082],
        to: [5546, 4114],
    },
    HelpLabelSegment::Line([5546, 4580]),
    HelpLabelSegment::Line([5435, 4580]),
    HelpLabelSegment::Line([5435, 3695]),
    HelpLabelSegment::Line([5546, 3695]),
];

const BODY_CONTOUR_182: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [6274, 4035],
        to: [6223, 4083],
    },
    HelpLabelSegment::Quad {
        control: [6175, 4129],
        to: [6168, 4197],
    },
    HelpLabelSegment::Line([6516, 4197]),
    HelpLabelSegment::Quad {
        control: [6516, 4129],
        to: [6474, 4084],
    },
    HelpLabelSegment::Quad {
        control: [6427, 4035],
        to: [6348, 4035],
    },
];

const BODY_CONTOUR_183: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [6280, 4498],
        to: [6363, 4498],
    },
    HelpLabelSegment::Quad {
        control: [6459, 4498],
        to: [6522, 4443],
    },
    HelpLabelSegment::Line([6569, 4523]),
    HelpLabelSegment::Quad {
        control: [6543, 4548],
        to: [6490, 4567],
    },
    HelpLabelSegment::Quad {
        control: [6424, 4592],
        to: [6342, 4592],
    },
    HelpLabelSegment::Quad {
        control: [6223, 4592],
        to: [6140, 4512],
    },
    HelpLabelSegment::Quad {
        control: [6049, 4423],
        to: [6049, 4274],
    },
    HelpLabelSegment::Quad {
        control: [6049, 4118],
        to: [6142, 4025],
    },
    HelpLabelSegment::Quad {
        control: [6227, 3941],
        to: [6343, 3941],
    },
    HelpLabelSegment::Quad {
        control: [6476, 3941],
        to: [6553, 4016],
    },
    HelpLabelSegment::Quad {
        control: [6626, 4089],
        to: [6626, 4210],
    },
    HelpLabelSegment::Quad {
        control: [6626, 4246],
        to: [6618, 4278],
    },
    HelpLabelSegment::Line([6166, 4278]),
    HelpLabelSegment::Quad {
        control: [6166, 4388],
        to: [6226, 4446],
    },
];

const BODY_CONTOUR_184: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Quad {
        control: [7436, 3941],
        to: [7498, 4003],
    },
    HelpLabelSegment::Quad {
        control: [7559, 4066],
        to: [7559, 4200],
    },
    HelpLabelSegment::Line([7559, 4425]),
    HelpLabelSegment::Quad {
        control: [7559, 4509],
        to: [7609, 4535],
    },
    HelpLabelSegment::Line([7609, 4592]),
    HelpLabelSegment::Quad {
        control: [7541, 4592],
        to: [7508, 4572],
    },
    HelpLabelSegment::Quad {
        control: [7474, 4553],
        to: [7459, 4509],
    },
    HelpLabelSegment::Quad {
        control: [7392, 4592],
        to: [7255, 4592],
    },
    HelpLabelSegment::Quad {
        control: [7181, 4592],
        to: [7127, 4539],
    },
    HelpLabelSegment::Quad {
        control: [7072, 4485],
        to: [7072, 4405],
    },
    HelpLabelSegment::Quad {
        control: [7072, 4309],
        to: [7156, 4244],
    },
    HelpLabelSegment::Quad {
        control: [7239, 4178],
        to: [7368, 4178],
    },
    HelpLabelSegment::Line([7448, 4193]),
    HelpLabelSegment::Quad {
        control: [7448, 4041],
        to: [7312, 4041],
    },
    HelpLabelSegment::Quad {
        control: [7208, 4041],
        to: [7152, 4097],
    },
    HelpLabelSegment::Line([7105, 4003]),
    HelpLabelSegment::Quad {
        control: [7136, 3978],
        to: [7193, 3960],
    },
    HelpLabelSegment::Quad {
        control: [7249, 3941],
        to: [7299, 3941],
    },
];

const BODY_CONTOUR_185: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [7290, 4260],
        to: [7237, 4303],
    },
    HelpLabelSegment::Quad {
        control: [7183, 4347],
        to: [7183, 4407],
    },
    HelpLabelSegment::Quad {
        control: [7183, 4504],
        to: [7299, 4504],
    },
    HelpLabelSegment::Quad {
        control: [7384, 4504],
        to: [7448, 4424],
    },
    HelpLabelSegment::Line([7448, 4272]),
    HelpLabelSegment::Line([7374, 4260]),
];

const BODY_CONTOUR_186: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Line([7775, 3822]),
    HelpLabelSegment::Line([7886, 3778]),
    HelpLabelSegment::Line([7886, 3953]),
    HelpLabelSegment::Line([8058, 3953]),
    HelpLabelSegment::Line([8058, 4041]),
    HelpLabelSegment::Line([7886, 4041]),
    HelpLabelSegment::Line([7886, 4353]),
    HelpLabelSegment::Quad {
        control: [7886, 4431],
        to: [7913, 4465],
    },
    HelpLabelSegment::Quad {
        control: [7939, 4498],
        to: [7998, 4498],
    },
    HelpLabelSegment::Quad {
        control: [8041, 4498],
        to: [8086, 4477],
    },
    HelpLabelSegment::Line([8103, 4574]),
    HelpLabelSegment::Line([7951, 4592]),
    HelpLabelSegment::Quad {
        control: [7876, 4592],
        to: [7826, 4537],
    },
    HelpLabelSegment::Quad {
        control: [7775, 4482],
        to: [7775, 4397],
    },
    HelpLabelSegment::Line([7775, 4041]),
    HelpLabelSegment::Line([7702, 4041]),
    HelpLabelSegment::Line([7702, 3953]),
    HelpLabelSegment::Line([7775, 3953]),
];

const BODY_CONTOUR_187: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Line([8250, 3822]),
    HelpLabelSegment::Line([8361, 3778]),
    HelpLabelSegment::Line([8361, 3953]),
    HelpLabelSegment::Line([8533, 3953]),
    HelpLabelSegment::Line([8533, 4041]),
    HelpLabelSegment::Line([8361, 4041]),
    HelpLabelSegment::Line([8361, 4353]),
    HelpLabelSegment::Quad {
        control: [8361, 4431],
        to: [8388, 4465],
    },
    HelpLabelSegment::Quad {
        control: [8414, 4498],
        to: [8473, 4498],
    },
    HelpLabelSegment::Quad {
        control: [8516, 4498],
        to: [8561, 4477],
    },
    HelpLabelSegment::Line([8578, 4574]),
    HelpLabelSegment::Line([8426, 4592]),
    HelpLabelSegment::Quad {
        control: [8351, 4592],
        to: [8301, 4537],
    },
    HelpLabelSegment::Quad {
        control: [8250, 4482],
        to: [8250, 4397],
    },
    HelpLabelSegment::Line([8250, 4041]),
    HelpLabelSegment::Line([8177, 4041]),
    HelpLabelSegment::Line([8177, 3953]),
    HelpLabelSegment::Line([8250, 3953]),
];

const BODY_CONTOUR_188: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Quad {
        control: [8989, 4035],
        to: [8952, 4035],
    },
    HelpLabelSegment::Quad {
        control: [8893, 4035],
        to: [8849, 4089],
    },
    HelpLabelSegment::Quad {
        control: [8804, 4144],
        to: [8804, 4220],
    },
    HelpLabelSegment::Line([8804, 4580]),
    HelpLabelSegment::Line([8693, 4580]),
    HelpLabelSegment::Line([8693, 3953]),
    HelpLabelSegment::Line([8804, 3953]),
    HelpLabelSegment::Line([8804, 4053]),
    HelpLabelSegment::Quad {
        control: [8865, 3941],
        to: [8986, 3941],
    },
    HelpLabelSegment::Line([9071, 3952]),
    HelpLabelSegment::Line([9026, 4060]),
];

const BODY_CONTOUR_189: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Quad {
        control: [9481, 3941],
        to: [9543, 4003],
    },
    HelpLabelSegment::Quad {
        control: [9604, 4066],
        to: [9604, 4200],
    },
    HelpLabelSegment::Line([9604, 4425]),
    HelpLabelSegment::Quad {
        control: [9604, 4509],
        to: [9654, 4535],
    },
    HelpLabelSegment::Line([9654, 4592]),
    HelpLabelSegment::Quad {
        control: [9586, 4592],
        to: [9553, 4572],
    },
    HelpLabelSegment::Quad {
        control: [9519, 4553],
        to: [9504, 4509],
    },
    HelpLabelSegment::Quad {
        control: [9437, 4592],
        to: [9300, 4592],
    },
    HelpLabelSegment::Quad {
        control: [9226, 4592],
        to: [9172, 4539],
    },
    HelpLabelSegment::Quad {
        control: [9117, 4485],
        to: [9117, 4405],
    },
    HelpLabelSegment::Quad {
        control: [9117, 4309],
        to: [9201, 4244],
    },
    HelpLabelSegment::Quad {
        control: [9284, 4178],
        to: [9413, 4178],
    },
    HelpLabelSegment::Line([9493, 4193]),
    HelpLabelSegment::Quad {
        control: [9493, 4041],
        to: [9357, 4041],
    },
    HelpLabelSegment::Quad {
        control: [9253, 4041],
        to: [9197, 4097],
    },
    HelpLabelSegment::Line([9150, 4003]),
    HelpLabelSegment::Quad {
        control: [9181, 3978],
        to: [9238, 3960],
    },
    HelpLabelSegment::Quad {
        control: [9294, 3941],
        to: [9344, 3941],
    },
];

const BODY_CONTOUR_190: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [9335, 4260],
        to: [9282, 4303],
    },
    HelpLabelSegment::Quad {
        control: [9228, 4347],
        to: [9228, 4407],
    },
    HelpLabelSegment::Quad {
        control: [9228, 4504],
        to: [9344, 4504],
    },
    HelpLabelSegment::Quad {
        control: [9429, 4504],
        to: [9493, 4424],
    },
    HelpLabelSegment::Line([9493, 4272]),
    HelpLabelSegment::Line([9419, 4260]),
];

const BODY_CONTOUR_191: [HelpLabelSegment; 16] = [
    HelpLabelSegment::Quad {
        control: [10226, 3982],
        to: [10253, 4003],
    },
    HelpLabelSegment::Line([10198, 4082]),
    HelpLabelSegment::Quad {
        control: [10180, 4066],
        to: [10138, 4050],
    },
    HelpLabelSegment::Line([10053, 4035]),
    HelpLabelSegment::Quad {
        control: [9963, 4035],
        to: [9909, 4098],
    },
    HelpLabelSegment::Quad {
        control: [9856, 4162],
        to: [9856, 4273],
    },
    HelpLabelSegment::Quad {
        control: [9856, 4383],
        to: [9910, 4441],
    },
    HelpLabelSegment::Quad {
        control: [9965, 4498],
        to: [10061, 4498],
    },
    HelpLabelSegment::Quad {
        control: [10136, 4498],
        to: [10212, 4441],
    },
    HelpLabelSegment::Line([10257, 4534]),
    HelpLabelSegment::Quad {
        control: [10166, 4592],
        to: [10034, 4592],
    },
    HelpLabelSegment::Quad {
        control: [9906, 4592],
        to: [9822, 4506],
    },
    HelpLabelSegment::Quad {
        control: [9739, 4419],
        to: [9739, 4273],
    },
    HelpLabelSegment::Quad {
        control: [9739, 4123],
        to: [9825, 4032],
    },
    HelpLabelSegment::Quad {
        control: [9912, 3941],
        to: [10063, 3941],
    },
    HelpLabelSegment::Line([10169, 3961]),
];

const BODY_CONTOUR_192: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Line([10415, 3822]),
    HelpLabelSegment::Line([10526, 3778]),
    HelpLabelSegment::Line([10526, 3953]),
    HelpLabelSegment::Line([10698, 3953]),
    HelpLabelSegment::Line([10698, 4041]),
    HelpLabelSegment::Line([10526, 4041]),
    HelpLabelSegment::Line([10526, 4353]),
    HelpLabelSegment::Quad {
        control: [10526, 4431],
        to: [10553, 4465],
    },
    HelpLabelSegment::Quad {
        control: [10579, 4498],
        to: [10638, 4498],
    },
    HelpLabelSegment::Quad {
        control: [10681, 4498],
        to: [10726, 4477],
    },
    HelpLabelSegment::Line([10743, 4574]),
    HelpLabelSegment::Line([10591, 4592]),
    HelpLabelSegment::Quad {
        control: [10516, 4592],
        to: [10466, 4537],
    },
    HelpLabelSegment::Quad {
        control: [10415, 4482],
        to: [10415, 4397],
    },
    HelpLabelSegment::Line([10415, 4041]),
    HelpLabelSegment::Line([10342, 4041]),
    HelpLabelSegment::Line([10342, 3953]),
    HelpLabelSegment::Line([10415, 3953]),
];

const BODY_CONTOUR_193: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [11394, 4035],
        to: [11343, 4083],
    },
    HelpLabelSegment::Quad {
        control: [11295, 4129],
        to: [11288, 4197],
    },
    HelpLabelSegment::Line([11636, 4197]),
    HelpLabelSegment::Quad {
        control: [11636, 4129],
        to: [11594, 4084],
    },
    HelpLabelSegment::Quad {
        control: [11547, 4035],
        to: [11468, 4035],
    },
];

const BODY_CONTOUR_194: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [11400, 4498],
        to: [11483, 4498],
    },
    HelpLabelSegment::Quad {
        control: [11579, 4498],
        to: [11642, 4443],
    },
    HelpLabelSegment::Line([11689, 4523]),
    HelpLabelSegment::Quad {
        control: [11663, 4548],
        to: [11610, 4567],
    },
    HelpLabelSegment::Quad {
        control: [11544, 4592],
        to: [11462, 4592],
    },
    HelpLabelSegment::Quad {
        control: [11343, 4592],
        to: [11260, 4512],
    },
    HelpLabelSegment::Quad {
        control: [11169, 4423],
        to: [11169, 4274],
    },
    HelpLabelSegment::Quad {
        control: [11169, 4118],
        to: [11262, 4025],
    },
    HelpLabelSegment::Quad {
        control: [11347, 3941],
        to: [11463, 3941],
    },
    HelpLabelSegment::Quad {
        control: [11596, 3941],
        to: [11673, 4016],
    },
    HelpLabelSegment::Quad {
        control: [11746, 4089],
        to: [11746, 4210],
    },
    HelpLabelSegment::Quad {
        control: [11746, 4246],
        to: [11738, 4278],
    },
    HelpLabelSegment::Line([11286, 4278]),
    HelpLabelSegment::Quad {
        control: [11286, 4388],
        to: [11346, 4446],
    },
];

const BODY_CONTOUR_195: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Quad {
        control: [12196, 3941],
        to: [12258, 4003],
    },
    HelpLabelSegment::Quad {
        control: [12319, 4066],
        to: [12319, 4200],
    },
    HelpLabelSegment::Line([12319, 4425]),
    HelpLabelSegment::Quad {
        control: [12319, 4509],
        to: [12369, 4535],
    },
    HelpLabelSegment::Line([12369, 4592]),
    HelpLabelSegment::Quad {
        control: [12301, 4592],
        to: [12268, 4572],
    },
    HelpLabelSegment::Quad {
        control: [12234, 4553],
        to: [12219, 4509],
    },
    HelpLabelSegment::Quad {
        control: [12152, 4592],
        to: [12015, 4592],
    },
    HelpLabelSegment::Quad {
        control: [11941, 4592],
        to: [11887, 4539],
    },
    HelpLabelSegment::Quad {
        control: [11832, 4485],
        to: [11832, 4405],
    },
    HelpLabelSegment::Quad {
        control: [11832, 4309],
        to: [11916, 4244],
    },
    HelpLabelSegment::Quad {
        control: [11999, 4178],
        to: [12128, 4178],
    },
    HelpLabelSegment::Line([12208, 4193]),
    HelpLabelSegment::Quad {
        control: [12208, 4041],
        to: [12072, 4041],
    },
    HelpLabelSegment::Quad {
        control: [11968, 4041],
        to: [11912, 4097],
    },
    HelpLabelSegment::Line([11865, 4003]),
    HelpLabelSegment::Quad {
        control: [11896, 3978],
        to: [11953, 3960],
    },
    HelpLabelSegment::Quad {
        control: [12009, 3941],
        to: [12059, 3941],
    },
];

const BODY_CONTOUR_196: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [12050, 4260],
        to: [11997, 4303],
    },
    HelpLabelSegment::Quad {
        control: [11943, 4347],
        to: [11943, 4407],
    },
    HelpLabelSegment::Quad {
        control: [11943, 4504],
        to: [12059, 4504],
    },
    HelpLabelSegment::Quad {
        control: [12144, 4504],
        to: [12208, 4424],
    },
    HelpLabelSegment::Line([12208, 4272]),
    HelpLabelSegment::Line([12134, 4260]),
];

const BODY_CONTOUR_197: [HelpLabelSegment; 16] = [
    HelpLabelSegment::Quad {
        control: [12941, 3982],
        to: [12968, 4003],
    },
    HelpLabelSegment::Line([12913, 4082]),
    HelpLabelSegment::Quad {
        control: [12895, 4066],
        to: [12853, 4050],
    },
    HelpLabelSegment::Line([12768, 4035]),
    HelpLabelSegment::Quad {
        control: [12678, 4035],
        to: [12624, 4098],
    },
    HelpLabelSegment::Quad {
        control: [12571, 4162],
        to: [12571, 4273],
    },
    HelpLabelSegment::Quad {
        control: [12571, 4383],
        to: [12625, 4441],
    },
    HelpLabelSegment::Quad {
        control: [12680, 4498],
        to: [12776, 4498],
    },
    HelpLabelSegment::Quad {
        control: [12851, 4498],
        to: [12927, 4441],
    },
    HelpLabelSegment::Line([12972, 4534]),
    HelpLabelSegment::Quad {
        control: [12881, 4592],
        to: [12749, 4592],
    },
    HelpLabelSegment::Quad {
        control: [12621, 4592],
        to: [12537, 4506],
    },
    HelpLabelSegment::Quad {
        control: [12454, 4419],
        to: [12454, 4273],
    },
    HelpLabelSegment::Quad {
        control: [12454, 4123],
        to: [12540, 4032],
    },
    HelpLabelSegment::Quad {
        control: [12627, 3941],
        to: [12778, 3941],
    },
    HelpLabelSegment::Line([12884, 3961]),
];

const BODY_CONTOUR_198: [HelpLabelSegment; 16] = [
    HelpLabelSegment::Line([13201, 4022]),
    HelpLabelSegment::Quad {
        control: [13223, 3987],
        to: [13273, 3965],
    },
    HelpLabelSegment::Quad {
        control: [13322, 3941],
        to: [13374, 3941],
    },
    HelpLabelSegment::Quad {
        control: [13474, 3941],
        to: [13531, 4007],
    },
    HelpLabelSegment::Quad {
        control: [13588, 4073],
        to: [13588, 4186],
    },
    HelpLabelSegment::Line([13588, 4580]),
    HelpLabelSegment::Line([13476, 4580]),
    HelpLabelSegment::Line([13476, 4186]),
    HelpLabelSegment::Quad {
        control: [13476, 4116],
        to: [13441, 4075],
    },
    HelpLabelSegment::Quad {
        control: [13407, 4035],
        to: [13344, 4035],
    },
    HelpLabelSegment::Quad {
        control: [13304, 4035],
        to: [13263, 4059],
    },
    HelpLabelSegment::Quad {
        control: [13222, 4082],
        to: [13201, 4114],
    },
    HelpLabelSegment::Line([13201, 4580]),
    HelpLabelSegment::Line([13090, 4580]),
    HelpLabelSegment::Line([13090, 3695]),
    HelpLabelSegment::Line([13201, 3695]),
];

const BODY_CONTOUR_199: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [14181, 4501],
        to: [14347, 4501],
    },
    HelpLabelSegment::Quad {
        control: [14426, 4501],
        to: [14470, 4438],
    },
    HelpLabelSegment::Quad {
        control: [14514, 4375],
        to: [14514, 4265],
    },
    HelpLabelSegment::Quad {
        control: [14514, 4032],
        to: [14347, 4032],
    },
    HelpLabelSegment::Quad {
        control: [14271, 4032],
        to: [14227, 4094],
    },
    HelpLabelSegment::Quad {
        control: [14181, 4156],
        to: [14181, 4265],
    },
];

const BODY_CONTOUR_200: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [14220, 3941],
        to: [14347, 3941],
    },
    HelpLabelSegment::Quad {
        control: [14482, 3941],
        to: [14557, 4027],
    },
    HelpLabelSegment::Quad {
        control: [14631, 4112],
        to: [14631, 4265],
    },
    HelpLabelSegment::Quad {
        control: [14631, 4417],
        to: [14555, 4505],
    },
    HelpLabelSegment::Quad {
        control: [14479, 4592],
        to: [14347, 4592],
    },
    HelpLabelSegment::Quad {
        control: [14214, 4592],
        to: [14139, 4504],
    },
    HelpLabelSegment::Quad {
        control: [14064, 4415],
        to: [14064, 4265],
    },
    HelpLabelSegment::Quad {
        control: [14064, 4119],
        to: [14142, 4030],
    },
];

const BODY_CONTOUR_201: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Line([14790, 3822]),
    HelpLabelSegment::Line([14901, 3778]),
    HelpLabelSegment::Line([14901, 3953]),
    HelpLabelSegment::Line([15073, 3953]),
    HelpLabelSegment::Line([15073, 4041]),
    HelpLabelSegment::Line([14901, 4041]),
    HelpLabelSegment::Line([14901, 4353]),
    HelpLabelSegment::Quad {
        control: [14901, 4431],
        to: [14928, 4465],
    },
    HelpLabelSegment::Quad {
        control: [14954, 4498],
        to: [15013, 4498],
    },
    HelpLabelSegment::Quad {
        control: [15056, 4498],
        to: [15101, 4477],
    },
    HelpLabelSegment::Line([15118, 4574]),
    HelpLabelSegment::Line([14966, 4592]),
    HelpLabelSegment::Quad {
        control: [14891, 4592],
        to: [14841, 4537],
    },
    HelpLabelSegment::Quad {
        control: [14790, 4482],
        to: [14790, 4397],
    },
    HelpLabelSegment::Line([14790, 4041]),
    HelpLabelSegment::Line([14717, 4041]),
    HelpLabelSegment::Line([14717, 3953]),
    HelpLabelSegment::Line([14790, 3953]),
];

const BODY_CONTOUR_202: [HelpLabelSegment; 16] = [
    HelpLabelSegment::Line([15336, 4022]),
    HelpLabelSegment::Quad {
        control: [15358, 3987],
        to: [15408, 3965],
    },
    HelpLabelSegment::Quad {
        control: [15457, 3941],
        to: [15509, 3941],
    },
    HelpLabelSegment::Quad {
        control: [15609, 3941],
        to: [15666, 4007],
    },
    HelpLabelSegment::Quad {
        control: [15723, 4073],
        to: [15723, 4186],
    },
    HelpLabelSegment::Line([15723, 4580]),
    HelpLabelSegment::Line([15611, 4580]),
    HelpLabelSegment::Line([15611, 4186]),
    HelpLabelSegment::Quad {
        control: [15611, 4116],
        to: [15576, 4075],
    },
    HelpLabelSegment::Quad {
        control: [15542, 4035],
        to: [15479, 4035],
    },
    HelpLabelSegment::Quad {
        control: [15439, 4035],
        to: [15398, 4059],
    },
    HelpLabelSegment::Quad {
        control: [15357, 4082],
        to: [15336, 4114],
    },
    HelpLabelSegment::Line([15336, 4580]),
    HelpLabelSegment::Line([15225, 4580]),
    HelpLabelSegment::Line([15225, 3695]),
    HelpLabelSegment::Line([15336, 3695]),
];

const BODY_CONTOUR_203: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [16064, 4035],
        to: [16013, 4083],
    },
    HelpLabelSegment::Quad {
        control: [15965, 4129],
        to: [15958, 4197],
    },
    HelpLabelSegment::Line([16306, 4197]),
    HelpLabelSegment::Quad {
        control: [16306, 4129],
        to: [16264, 4084],
    },
    HelpLabelSegment::Quad {
        control: [16217, 4035],
        to: [16138, 4035],
    },
];

const BODY_CONTOUR_204: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [16070, 4498],
        to: [16153, 4498],
    },
    HelpLabelSegment::Quad {
        control: [16249, 4498],
        to: [16312, 4443],
    },
    HelpLabelSegment::Line([16359, 4523]),
    HelpLabelSegment::Quad {
        control: [16333, 4548],
        to: [16280, 4567],
    },
    HelpLabelSegment::Quad {
        control: [16214, 4592],
        to: [16132, 4592],
    },
    HelpLabelSegment::Quad {
        control: [16013, 4592],
        to: [15930, 4512],
    },
    HelpLabelSegment::Quad {
        control: [15839, 4423],
        to: [15839, 4274],
    },
    HelpLabelSegment::Quad {
        control: [15839, 4118],
        to: [15932, 4025],
    },
    HelpLabelSegment::Quad {
        control: [16017, 3941],
        to: [16133, 3941],
    },
    HelpLabelSegment::Quad {
        control: [16266, 3941],
        to: [16343, 4016],
    },
    HelpLabelSegment::Quad {
        control: [16416, 4089],
        to: [16416, 4210],
    },
    HelpLabelSegment::Quad {
        control: [16416, 4246],
        to: [16408, 4278],
    },
    HelpLabelSegment::Line([15956, 4278]),
    HelpLabelSegment::Quad {
        control: [15956, 4388],
        to: [16016, 4446],
    },
];

const BODY_CONTOUR_205: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Quad {
        control: [16839, 4035],
        to: [16802, 4035],
    },
    HelpLabelSegment::Quad {
        control: [16743, 4035],
        to: [16699, 4089],
    },
    HelpLabelSegment::Quad {
        control: [16654, 4144],
        to: [16654, 4220],
    },
    HelpLabelSegment::Line([16654, 4580]),
    HelpLabelSegment::Line([16543, 4580]),
    HelpLabelSegment::Line([16543, 3953]),
    HelpLabelSegment::Line([16654, 3953]),
    HelpLabelSegment::Line([16654, 4053]),
    HelpLabelSegment::Quad {
        control: [16715, 3941],
        to: [16836, 3941],
    },
    HelpLabelSegment::Line([16921, 3952]),
    HelpLabelSegment::Line([16876, 4060]),
];

const BODY_CONTOUR_206: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [17004, 4416],
        to: [17030, 4442],
    },
    HelpLabelSegment::Quad {
        control: [17055, 4468],
        to: [17055, 4504],
    },
    HelpLabelSegment::Quad {
        control: [17055, 4540],
        to: [17030, 4566],
    },
    HelpLabelSegment::Quad {
        control: [17004, 4592],
        to: [16967, 4592],
    },
    HelpLabelSegment::Quad {
        control: [16931, 4592],
        to: [16905, 4566],
    },
    HelpLabelSegment::Quad {
        control: [16880, 4540],
        to: [16880, 4504],
    },
    HelpLabelSegment::Quad {
        control: [16880, 4468],
        to: [16905, 4442],
    },
    HelpLabelSegment::Quad {
        control: [16931, 4416],
        to: [16967, 4416],
    },
];

const BODY_CONTOUR_207: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [17444, 4416],
        to: [17470, 4442],
    },
    HelpLabelSegment::Quad {
        control: [17495, 4468],
        to: [17495, 4504],
    },
    HelpLabelSegment::Quad {
        control: [17495, 4540],
        to: [17470, 4566],
    },
    HelpLabelSegment::Quad {
        control: [17444, 4592],
        to: [17407, 4592],
    },
    HelpLabelSegment::Quad {
        control: [17371, 4592],
        to: [17345, 4566],
    },
    HelpLabelSegment::Quad {
        control: [17320, 4540],
        to: [17320, 4504],
    },
    HelpLabelSegment::Quad {
        control: [17320, 4468],
        to: [17345, 4442],
    },
    HelpLabelSegment::Quad {
        control: [17371, 4416],
        to: [17407, 4416],
    },
];

const BODY_CONTOUR_208: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [17884, 4416],
        to: [17910, 4442],
    },
    HelpLabelSegment::Quad {
        control: [17935, 4468],
        to: [17935, 4504],
    },
    HelpLabelSegment::Quad {
        control: [17935, 4540],
        to: [17910, 4566],
    },
    HelpLabelSegment::Quad {
        control: [17884, 4592],
        to: [17847, 4592],
    },
    HelpLabelSegment::Quad {
        control: [17811, 4592],
        to: [17785, 4566],
    },
    HelpLabelSegment::Quad {
        control: [17760, 4540],
        to: [17760, 4504],
    },
    HelpLabelSegment::Quad {
        control: [17760, 4468],
        to: [17785, 4442],
    },
    HelpLabelSegment::Quad {
        control: [17811, 4416],
        to: [17847, 4416],
    },
];

const BODY_CONTOUR_209: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [-9746, 6116],
        to: [-9720, 6142],
    },
    HelpLabelSegment::Quad {
        control: [-9695, 6168],
        to: [-9695, 6204],
    },
    HelpLabelSegment::Quad {
        control: [-9695, 6240],
        to: [-9720, 6266],
    },
    HelpLabelSegment::Quad {
        control: [-9746, 6292],
        to: [-9783, 6292],
    },
    HelpLabelSegment::Quad {
        control: [-9819, 6292],
        to: [-9845, 6266],
    },
    HelpLabelSegment::Quad {
        control: [-9870, 6240],
        to: [-9870, 6204],
    },
    HelpLabelSegment::Quad {
        control: [-9870, 6168],
        to: [-9845, 6142],
    },
    HelpLabelSegment::Quad {
        control: [-9819, 6116],
        to: [-9783, 6116],
    },
];

const BODY_CONTOUR_210: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [-9306, 6116],
        to: [-9280, 6142],
    },
    HelpLabelSegment::Quad {
        control: [-9255, 6168],
        to: [-9255, 6204],
    },
    HelpLabelSegment::Quad {
        control: [-9255, 6240],
        to: [-9280, 6266],
    },
    HelpLabelSegment::Quad {
        control: [-9306, 6292],
        to: [-9343, 6292],
    },
    HelpLabelSegment::Quad {
        control: [-9379, 6292],
        to: [-9405, 6266],
    },
    HelpLabelSegment::Quad {
        control: [-9430, 6240],
        to: [-9430, 6204],
    },
    HelpLabelSegment::Quad {
        control: [-9430, 6168],
        to: [-9405, 6142],
    },
    HelpLabelSegment::Quad {
        control: [-9379, 6116],
        to: [-9343, 6116],
    },
];

const BODY_CONTOUR_211: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [-8866, 6116],
        to: [-8840, 6142],
    },
    HelpLabelSegment::Quad {
        control: [-8815, 6168],
        to: [-8815, 6204],
    },
    HelpLabelSegment::Quad {
        control: [-8815, 6240],
        to: [-8840, 6266],
    },
    HelpLabelSegment::Quad {
        control: [-8866, 6292],
        to: [-8903, 6292],
    },
    HelpLabelSegment::Quad {
        control: [-8939, 6292],
        to: [-8965, 6266],
    },
    HelpLabelSegment::Quad {
        control: [-8990, 6240],
        to: [-8990, 6204],
    },
    HelpLabelSegment::Quad {
        control: [-8990, 6168],
        to: [-8965, 6142],
    },
    HelpLabelSegment::Quad {
        control: [-8939, 6116],
        to: [-8903, 6116],
    },
];

const BODY_CONTOUR_212: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [-8514, 6201],
        to: [-8348, 6201],
    },
    HelpLabelSegment::Quad {
        control: [-8269, 6201],
        to: [-8225, 6138],
    },
    HelpLabelSegment::Quad {
        control: [-8181, 6075],
        to: [-8181, 5965],
    },
    HelpLabelSegment::Quad {
        control: [-8181, 5732],
        to: [-8348, 5732],
    },
    HelpLabelSegment::Quad {
        control: [-8424, 5732],
        to: [-8468, 5794],
    },
    HelpLabelSegment::Quad {
        control: [-8514, 5856],
        to: [-8514, 5965],
    },
];

const BODY_CONTOUR_213: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [-8475, 5641],
        to: [-8348, 5641],
    },
    HelpLabelSegment::Quad {
        control: [-8213, 5641],
        to: [-8138, 5727],
    },
    HelpLabelSegment::Quad {
        control: [-8064, 5812],
        to: [-8064, 5965],
    },
    HelpLabelSegment::Quad {
        control: [-8064, 6117],
        to: [-8140, 6205],
    },
    HelpLabelSegment::Quad {
        control: [-8216, 6292],
        to: [-8348, 6292],
    },
    HelpLabelSegment::Quad {
        control: [-8481, 6292],
        to: [-8556, 6204],
    },
    HelpLabelSegment::Quad {
        control: [-8631, 6115],
        to: [-8631, 5965],
    },
    HelpLabelSegment::Quad {
        control: [-8631, 5819],
        to: [-8553, 5730],
    },
];

const BODY_CONTOUR_214: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Quad {
        control: [-7641, 5735],
        to: [-7678, 5735],
    },
    HelpLabelSegment::Quad {
        control: [-7737, 5735],
        to: [-7781, 5789],
    },
    HelpLabelSegment::Quad {
        control: [-7826, 5844],
        to: [-7826, 5920],
    },
    HelpLabelSegment::Line([-7826, 6280]),
    HelpLabelSegment::Line([-7937, 6280]),
    HelpLabelSegment::Line([-7937, 5653]),
    HelpLabelSegment::Line([-7826, 5653]),
    HelpLabelSegment::Line([-7826, 5753]),
    HelpLabelSegment::Quad {
        control: [-7765, 5641],
        to: [-7644, 5641],
    },
    HelpLabelSegment::Line([-7559, 5652]),
    HelpLabelSegment::Line([-7604, 5760]),
];

const BODY_CONTOUR_215: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Quad {
        control: [-6816, 5735],
        to: [-6853, 5735],
    },
    HelpLabelSegment::Quad {
        control: [-6912, 5735],
        to: [-6956, 5789],
    },
    HelpLabelSegment::Quad {
        control: [-7001, 5844],
        to: [-7001, 5920],
    },
    HelpLabelSegment::Line([-7001, 6280]),
    HelpLabelSegment::Line([-7112, 6280]),
    HelpLabelSegment::Line([-7112, 5653]),
    HelpLabelSegment::Line([-7001, 5653]),
    HelpLabelSegment::Line([-7001, 5753]),
    HelpLabelSegment::Quad {
        control: [-6940, 5641],
        to: [-6819, 5641],
    },
    HelpLabelSegment::Line([-6734, 5652]),
    HelpLabelSegment::Line([-6779, 5760]),
];

const BODY_CONTOUR_216: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [-6471, 5735],
        to: [-6522, 5783],
    },
    HelpLabelSegment::Quad {
        control: [-6570, 5829],
        to: [-6577, 5897],
    },
    HelpLabelSegment::Line([-6229, 5897]),
    HelpLabelSegment::Quad {
        control: [-6229, 5829],
        to: [-6271, 5784],
    },
    HelpLabelSegment::Quad {
        control: [-6318, 5735],
        to: [-6397, 5735],
    },
];

const BODY_CONTOUR_217: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [-6465, 6198],
        to: [-6382, 6198],
    },
    HelpLabelSegment::Quad {
        control: [-6286, 6198],
        to: [-6223, 6143],
    },
    HelpLabelSegment::Line([-6176, 6223]),
    HelpLabelSegment::Quad {
        control: [-6202, 6248],
        to: [-6255, 6267],
    },
    HelpLabelSegment::Quad {
        control: [-6321, 6292],
        to: [-6403, 6292],
    },
    HelpLabelSegment::Quad {
        control: [-6522, 6292],
        to: [-6605, 6212],
    },
    HelpLabelSegment::Quad {
        control: [-6696, 6123],
        to: [-6696, 5974],
    },
    HelpLabelSegment::Quad {
        control: [-6696, 5818],
        to: [-6603, 5725],
    },
    HelpLabelSegment::Quad {
        control: [-6518, 5641],
        to: [-6402, 5641],
    },
    HelpLabelSegment::Quad {
        control: [-6269, 5641],
        to: [-6192, 5716],
    },
    HelpLabelSegment::Quad {
        control: [-6119, 5789],
        to: [-6119, 5910],
    },
    HelpLabelSegment::Quad {
        control: [-6119, 5946],
        to: [-6127, 5978],
    },
    HelpLabelSegment::Line([-6579, 5978]),
    HelpLabelSegment::Quad {
        control: [-6579, 6088],
        to: [-6519, 6146],
    },
];

const BODY_CONTOUR_218: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [-5857, 5415],
        to: [-5829, 5415],
    },
    HelpLabelSegment::Quad {
        control: [-5800, 5415],
        to: [-5780, 5436],
    },
    HelpLabelSegment::Quad {
        control: [-5760, 5456],
        to: [-5760, 5484],
    },
    HelpLabelSegment::Quad {
        control: [-5760, 5512],
        to: [-5780, 5534],
    },
    HelpLabelSegment::Quad {
        control: [-5800, 5553],
        to: [-5829, 5553],
    },
    HelpLabelSegment::Quad {
        control: [-5857, 5553],
        to: [-5877, 5534],
    },
    HelpLabelSegment::Quad {
        control: [-5897, 5512],
        to: [-5897, 5484],
    },
    HelpLabelSegment::Quad {
        control: [-5897, 5455],
        to: [-5877, 5435],
    },
];

const BODY_CONTOUR_219: [HelpLabelSegment; 10] = [
    HelpLabelSegment::Line([-5758, 6275]),
    HelpLabelSegment::Quad {
        control: [-5758, 6404],
        to: [-5834, 6465],
    },
    HelpLabelSegment::Quad {
        control: [-5910, 6526],
        to: [-6069, 6526],
    },
    HelpLabelSegment::Line([-6069, 6426]),
    HelpLabelSegment::Quad {
        control: [-5955, 6426],
        to: [-5911, 6392],
    },
    HelpLabelSegment::Quad {
        control: [-5869, 6357],
        to: [-5869, 6278],
    },
    HelpLabelSegment::Line([-5869, 5747]),
    HelpLabelSegment::Line([-5994, 5747]),
    HelpLabelSegment::Line([-5994, 5653]),
    HelpLabelSegment::Line([-5758, 5653]),
];

const BODY_CONTOUR_220: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [-5376, 5735],
        to: [-5427, 5783],
    },
    HelpLabelSegment::Quad {
        control: [-5475, 5829],
        to: [-5482, 5897],
    },
    HelpLabelSegment::Line([-5134, 5897]),
    HelpLabelSegment::Quad {
        control: [-5134, 5829],
        to: [-5176, 5784],
    },
    HelpLabelSegment::Quad {
        control: [-5223, 5735],
        to: [-5302, 5735],
    },
];

const BODY_CONTOUR_221: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [-5370, 6198],
        to: [-5287, 6198],
    },
    HelpLabelSegment::Quad {
        control: [-5191, 6198],
        to: [-5128, 6143],
    },
    HelpLabelSegment::Line([-5081, 6223]),
    HelpLabelSegment::Quad {
        control: [-5107, 6248],
        to: [-5160, 6267],
    },
    HelpLabelSegment::Quad {
        control: [-5226, 6292],
        to: [-5308, 6292],
    },
    HelpLabelSegment::Quad {
        control: [-5427, 6292],
        to: [-5510, 6212],
    },
    HelpLabelSegment::Quad {
        control: [-5601, 6123],
        to: [-5601, 5974],
    },
    HelpLabelSegment::Quad {
        control: [-5601, 5818],
        to: [-5508, 5725],
    },
    HelpLabelSegment::Quad {
        control: [-5423, 5641],
        to: [-5307, 5641],
    },
    HelpLabelSegment::Quad {
        control: [-5174, 5641],
        to: [-5097, 5716],
    },
    HelpLabelSegment::Quad {
        control: [-5024, 5789],
        to: [-5024, 5910],
    },
    HelpLabelSegment::Quad {
        control: [-5024, 5946],
        to: [-5032, 5978],
    },
    HelpLabelSegment::Line([-5484, 5978]),
    HelpLabelSegment::Quad {
        control: [-5484, 6088],
        to: [-5424, 6146],
    },
];

const BODY_CONTOUR_222: [HelpLabelSegment; 16] = [
    HelpLabelSegment::Quad {
        control: [-4459, 5682],
        to: [-4432, 5703],
    },
    HelpLabelSegment::Line([-4487, 5782]),
    HelpLabelSegment::Quad {
        control: [-4505, 5766],
        to: [-4547, 5750],
    },
    HelpLabelSegment::Line([-4632, 5735]),
    HelpLabelSegment::Quad {
        control: [-4722, 5735],
        to: [-4776, 5798],
    },
    HelpLabelSegment::Quad {
        control: [-4829, 5862],
        to: [-4829, 5973],
    },
    HelpLabelSegment::Quad {
        control: [-4829, 6083],
        to: [-4775, 6141],
    },
    HelpLabelSegment::Quad {
        control: [-4720, 6198],
        to: [-4624, 6198],
    },
    HelpLabelSegment::Quad {
        control: [-4549, 6198],
        to: [-4473, 6141],
    },
    HelpLabelSegment::Line([-4428, 6234]),
    HelpLabelSegment::Quad {
        control: [-4519, 6292],
        to: [-4651, 6292],
    },
    HelpLabelSegment::Quad {
        control: [-4779, 6292],
        to: [-4863, 6206],
    },
    HelpLabelSegment::Quad {
        control: [-4946, 6119],
        to: [-4946, 5973],
    },
    HelpLabelSegment::Quad {
        control: [-4946, 5823],
        to: [-4860, 5732],
    },
    HelpLabelSegment::Quad {
        control: [-4773, 5641],
        to: [-4622, 5641],
    },
    HelpLabelSegment::Line([-4516, 5661]),
];

const BODY_CONTOUR_223: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Line([-4270, 5522]),
    HelpLabelSegment::Line([-4159, 5478]),
    HelpLabelSegment::Line([-4159, 5653]),
    HelpLabelSegment::Line([-3987, 5653]),
    HelpLabelSegment::Line([-3987, 5741]),
    HelpLabelSegment::Line([-4159, 5741]),
    HelpLabelSegment::Line([-4159, 6053]),
    HelpLabelSegment::Quad {
        control: [-4159, 6131],
        to: [-4132, 6165],
    },
    HelpLabelSegment::Quad {
        control: [-4106, 6198],
        to: [-4047, 6198],
    },
    HelpLabelSegment::Quad {
        control: [-4004, 6198],
        to: [-3959, 6177],
    },
    HelpLabelSegment::Line([-3942, 6274]),
    HelpLabelSegment::Line([-4094, 6292]),
    HelpLabelSegment::Quad {
        control: [-4169, 6292],
        to: [-4219, 6237],
    },
    HelpLabelSegment::Quad {
        control: [-4270, 6182],
        to: [-4270, 6097],
    },
    HelpLabelSegment::Line([-4270, 5741]),
    HelpLabelSegment::Line([-4343, 5741]),
    HelpLabelSegment::Line([-4343, 5653]),
    HelpLabelSegment::Line([-4270, 5653]),
];

const BODY_CONTOUR_224: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [-3291, 5735],
        to: [-3342, 5783],
    },
    HelpLabelSegment::Quad {
        control: [-3390, 5829],
        to: [-3397, 5897],
    },
    HelpLabelSegment::Line([-3049, 5897]),
    HelpLabelSegment::Quad {
        control: [-3049, 5829],
        to: [-3091, 5784],
    },
    HelpLabelSegment::Quad {
        control: [-3138, 5735],
        to: [-3217, 5735],
    },
];

const BODY_CONTOUR_225: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [-3285, 6198],
        to: [-3202, 6198],
    },
    HelpLabelSegment::Quad {
        control: [-3106, 6198],
        to: [-3043, 6143],
    },
    HelpLabelSegment::Line([-2996, 6223]),
    HelpLabelSegment::Quad {
        control: [-3022, 6248],
        to: [-3075, 6267],
    },
    HelpLabelSegment::Quad {
        control: [-3141, 6292],
        to: [-3223, 6292],
    },
    HelpLabelSegment::Quad {
        control: [-3342, 6292],
        to: [-3425, 6212],
    },
    HelpLabelSegment::Quad {
        control: [-3516, 6123],
        to: [-3516, 5974],
    },
    HelpLabelSegment::Quad {
        control: [-3516, 5818],
        to: [-3423, 5725],
    },
    HelpLabelSegment::Quad {
        control: [-3338, 5641],
        to: [-3222, 5641],
    },
    HelpLabelSegment::Quad {
        control: [-3089, 5641],
        to: [-3012, 5716],
    },
    HelpLabelSegment::Quad {
        control: [-2939, 5789],
        to: [-2939, 5910],
    },
    HelpLabelSegment::Quad {
        control: [-2939, 5946],
        to: [-2947, 5978],
    },
    HelpLabelSegment::Line([-3399, 5978]),
    HelpLabelSegment::Quad {
        control: [-3399, 6088],
        to: [-3339, 6146],
    },
];

const BODY_CONTOUR_226: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Quad {
        control: [-2489, 5641],
        to: [-2427, 5703],
    },
    HelpLabelSegment::Quad {
        control: [-2366, 5766],
        to: [-2366, 5900],
    },
    HelpLabelSegment::Line([-2366, 6125]),
    HelpLabelSegment::Quad {
        control: [-2366, 6209],
        to: [-2316, 6235],
    },
    HelpLabelSegment::Line([-2316, 6292]),
    HelpLabelSegment::Quad {
        control: [-2384, 6292],
        to: [-2417, 6272],
    },
    HelpLabelSegment::Quad {
        control: [-2451, 6253],
        to: [-2466, 6209],
    },
    HelpLabelSegment::Quad {
        control: [-2533, 6292],
        to: [-2670, 6292],
    },
    HelpLabelSegment::Quad {
        control: [-2744, 6292],
        to: [-2798, 6239],
    },
    HelpLabelSegment::Quad {
        control: [-2853, 6185],
        to: [-2853, 6105],
    },
    HelpLabelSegment::Quad {
        control: [-2853, 6009],
        to: [-2769, 5944],
    },
    HelpLabelSegment::Quad {
        control: [-2686, 5878],
        to: [-2557, 5878],
    },
    HelpLabelSegment::Line([-2477, 5893]),
    HelpLabelSegment::Quad {
        control: [-2477, 5741],
        to: [-2613, 5741],
    },
    HelpLabelSegment::Quad {
        control: [-2717, 5741],
        to: [-2773, 5797],
    },
    HelpLabelSegment::Line([-2820, 5703]),
    HelpLabelSegment::Quad {
        control: [-2789, 5678],
        to: [-2732, 5660],
    },
    HelpLabelSegment::Quad {
        control: [-2676, 5641],
        to: [-2626, 5641],
    },
];

const BODY_CONTOUR_227: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [-2635, 5960],
        to: [-2688, 6003],
    },
    HelpLabelSegment::Quad {
        control: [-2742, 6047],
        to: [-2742, 6107],
    },
    HelpLabelSegment::Quad {
        control: [-2742, 6204],
        to: [-2626, 6204],
    },
    HelpLabelSegment::Quad {
        control: [-2541, 6204],
        to: [-2477, 6124],
    },
    HelpLabelSegment::Line([-2477, 5972]),
    HelpLabelSegment::Line([-2551, 5960]),
];

const BODY_CONTOUR_228: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Line([-2150, 5522]),
    HelpLabelSegment::Line([-2039, 5478]),
    HelpLabelSegment::Line([-2039, 5653]),
    HelpLabelSegment::Line([-1867, 5653]),
    HelpLabelSegment::Line([-1867, 5741]),
    HelpLabelSegment::Line([-2039, 5741]),
    HelpLabelSegment::Line([-2039, 6053]),
    HelpLabelSegment::Quad {
        control: [-2039, 6131],
        to: [-2012, 6165],
    },
    HelpLabelSegment::Quad {
        control: [-1986, 6198],
        to: [-1927, 6198],
    },
    HelpLabelSegment::Quad {
        control: [-1884, 6198],
        to: [-1839, 6177],
    },
    HelpLabelSegment::Line([-1822, 6274]),
    HelpLabelSegment::Line([-1974, 6292]),
    HelpLabelSegment::Quad {
        control: [-2049, 6292],
        to: [-2099, 6237],
    },
    HelpLabelSegment::Quad {
        control: [-2150, 6182],
        to: [-2150, 6097],
    },
    HelpLabelSegment::Line([-2150, 5741]),
    HelpLabelSegment::Line([-2223, 5741]),
    HelpLabelSegment::Line([-2223, 5653]),
    HelpLabelSegment::Line([-2150, 5653]),
];

const BODY_CONTOUR_229: [HelpLabelSegment; 16] = [
    HelpLabelSegment::Quad {
        control: [-1269, 5682],
        to: [-1242, 5703],
    },
    HelpLabelSegment::Line([-1297, 5782]),
    HelpLabelSegment::Quad {
        control: [-1315, 5766],
        to: [-1357, 5750],
    },
    HelpLabelSegment::Line([-1442, 5735]),
    HelpLabelSegment::Quad {
        control: [-1533, 5735],
        to: [-1586, 5798],
    },
    HelpLabelSegment::Quad {
        control: [-1639, 5862],
        to: [-1639, 5973],
    },
    HelpLabelSegment::Quad {
        control: [-1639, 6083],
        to: [-1585, 6141],
    },
    HelpLabelSegment::Quad {
        control: [-1530, 6198],
        to: [-1434, 6198],
    },
    HelpLabelSegment::Quad {
        control: [-1359, 6198],
        to: [-1283, 6141],
    },
    HelpLabelSegment::Line([-1238, 6234]),
    HelpLabelSegment::Quad {
        control: [-1329, 6292],
        to: [-1461, 6292],
    },
    HelpLabelSegment::Quad {
        control: [-1589, 6292],
        to: [-1673, 6206],
    },
    HelpLabelSegment::Quad {
        control: [-1756, 6119],
        to: [-1756, 5973],
    },
    HelpLabelSegment::Quad {
        control: [-1756, 5823],
        to: [-1670, 5732],
    },
    HelpLabelSegment::Quad {
        control: [-1583, 5641],
        to: [-1432, 5641],
    },
    HelpLabelSegment::Line([-1326, 5661]),
];

const BODY_CONTOUR_230: [HelpLabelSegment; 16] = [
    HelpLabelSegment::Line([-1009, 5722]),
    HelpLabelSegment::Quad {
        control: [-987, 5687],
        to: [-938, 5665],
    },
    HelpLabelSegment::Quad {
        control: [-888, 5641],
        to: [-836, 5641],
    },
    HelpLabelSegment::Quad {
        control: [-736, 5641],
        to: [-679, 5707],
    },
    HelpLabelSegment::Quad {
        control: [-622, 5773],
        to: [-622, 5886],
    },
    HelpLabelSegment::Line([-622, 6280]),
    HelpLabelSegment::Line([-734, 6280]),
    HelpLabelSegment::Line([-734, 5886]),
    HelpLabelSegment::Quad {
        control: [-734, 5816],
        to: [-769, 5775],
    },
    HelpLabelSegment::Quad {
        control: [-803, 5735],
        to: [-866, 5735],
    },
    HelpLabelSegment::Quad {
        control: [-906, 5735],
        to: [-947, 5759],
    },
    HelpLabelSegment::Quad {
        control: [-988, 5782],
        to: [-1009, 5814],
    },
    HelpLabelSegment::Line([-1009, 6280]),
    HelpLabelSegment::Line([-1120, 6280]),
    HelpLabelSegment::Line([-1120, 5395]),
    HelpLabelSegment::Line([-1009, 5395]),
];

const BODY_CONTOUR_231: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [-29, 6201],
        to: [137, 6201],
    },
    HelpLabelSegment::Quad {
        control: [216, 6201],
        to: [260, 6138],
    },
    HelpLabelSegment::Quad {
        control: [304, 6075],
        to: [304, 5965],
    },
    HelpLabelSegment::Quad {
        control: [304, 5732],
        to: [137, 5732],
    },
    HelpLabelSegment::Quad {
        control: [61, 5732],
        to: [17, 5794],
    },
    HelpLabelSegment::Quad {
        control: [-29, 5856],
        to: [-29, 5965],
    },
];

const BODY_CONTOUR_232: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [10, 5641],
        to: [137, 5641],
    },
    HelpLabelSegment::Quad {
        control: [272, 5641],
        to: [347, 5727],
    },
    HelpLabelSegment::Quad {
        control: [421, 5812],
        to: [421, 5965],
    },
    HelpLabelSegment::Quad {
        control: [421, 6117],
        to: [345, 6205],
    },
    HelpLabelSegment::Quad {
        control: [269, 6292],
        to: [137, 6292],
    },
    HelpLabelSegment::Quad {
        control: [4, 6292],
        to: [-71, 6204],
    },
    HelpLabelSegment::Quad {
        control: [-146, 6115],
        to: [-146, 5965],
    },
    HelpLabelSegment::Quad {
        control: [-146, 5819],
        to: [-68, 5730],
    },
];

const BODY_CONTOUR_233: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Line([580, 5522]),
    HelpLabelSegment::Line([691, 5478]),
    HelpLabelSegment::Line([691, 5653]),
    HelpLabelSegment::Line([863, 5653]),
    HelpLabelSegment::Line([863, 5741]),
    HelpLabelSegment::Line([691, 5741]),
    HelpLabelSegment::Line([691, 6053]),
    HelpLabelSegment::Quad {
        control: [691, 6131],
        to: [718, 6165],
    },
    HelpLabelSegment::Quad {
        control: [744, 6198],
        to: [803, 6198],
    },
    HelpLabelSegment::Quad {
        control: [846, 6198],
        to: [891, 6177],
    },
    HelpLabelSegment::Line([908, 6274]),
    HelpLabelSegment::Line([756, 6292]),
    HelpLabelSegment::Quad {
        control: [681, 6292],
        to: [631, 6237],
    },
    HelpLabelSegment::Quad {
        control: [580, 6182],
        to: [580, 6097],
    },
    HelpLabelSegment::Line([580, 5741]),
    HelpLabelSegment::Line([507, 5741]),
    HelpLabelSegment::Line([507, 5653]),
    HelpLabelSegment::Line([580, 5653]),
];

const BODY_CONTOUR_234: [HelpLabelSegment; 16] = [
    HelpLabelSegment::Line([1126, 5722]),
    HelpLabelSegment::Quad {
        control: [1148, 5687],
        to: [1197, 5665],
    },
    HelpLabelSegment::Quad {
        control: [1247, 5641],
        to: [1299, 5641],
    },
    HelpLabelSegment::Quad {
        control: [1399, 5641],
        to: [1456, 5707],
    },
    HelpLabelSegment::Quad {
        control: [1513, 5773],
        to: [1513, 5886],
    },
    HelpLabelSegment::Line([1513, 6280]),
    HelpLabelSegment::Line([1401, 6280]),
    HelpLabelSegment::Line([1401, 5886]),
    HelpLabelSegment::Quad {
        control: [1401, 5816],
        to: [1366, 5775],
    },
    HelpLabelSegment::Quad {
        control: [1332, 5735],
        to: [1269, 5735],
    },
    HelpLabelSegment::Quad {
        control: [1229, 5735],
        to: [1188, 5759],
    },
    HelpLabelSegment::Quad {
        control: [1147, 5782],
        to: [1126, 5814],
    },
    HelpLabelSegment::Line([1126, 6280]),
    HelpLabelSegment::Line([1015, 6280]),
    HelpLabelSegment::Line([1015, 5395]),
    HelpLabelSegment::Line([1126, 5395]),
];

const BODY_CONTOUR_235: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [1854, 5735],
        to: [1803, 5783],
    },
    HelpLabelSegment::Quad {
        control: [1755, 5829],
        to: [1748, 5897],
    },
    HelpLabelSegment::Line([2096, 5897]),
    HelpLabelSegment::Quad {
        control: [2096, 5829],
        to: [2054, 5784],
    },
    HelpLabelSegment::Quad {
        control: [2007, 5735],
        to: [1927, 5735],
    },
];

const BODY_CONTOUR_236: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [1860, 6198],
        to: [1943, 6198],
    },
    HelpLabelSegment::Quad {
        control: [2039, 6198],
        to: [2102, 6143],
    },
    HelpLabelSegment::Line([2149, 6223]),
    HelpLabelSegment::Quad {
        control: [2123, 6248],
        to: [2070, 6267],
    },
    HelpLabelSegment::Quad {
        control: [2004, 6292],
        to: [1922, 6292],
    },
    HelpLabelSegment::Quad {
        control: [1803, 6292],
        to: [1720, 6212],
    },
    HelpLabelSegment::Quad {
        control: [1629, 6123],
        to: [1629, 5974],
    },
    HelpLabelSegment::Quad {
        control: [1629, 5818],
        to: [1722, 5725],
    },
    HelpLabelSegment::Quad {
        control: [1807, 5641],
        to: [1923, 5641],
    },
    HelpLabelSegment::Quad {
        control: [2056, 5641],
        to: [2133, 5716],
    },
    HelpLabelSegment::Quad {
        control: [2206, 5789],
        to: [2206, 5910],
    },
    HelpLabelSegment::Quad {
        control: [2206, 5946],
        to: [2198, 5978],
    },
    HelpLabelSegment::Line([1746, 5978]),
    HelpLabelSegment::Quad {
        control: [1746, 6088],
        to: [1806, 6146],
    },
];

const BODY_CONTOUR_237: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Quad {
        control: [2629, 5735],
        to: [2592, 5735],
    },
    HelpLabelSegment::Quad {
        control: [2533, 5735],
        to: [2489, 5789],
    },
    HelpLabelSegment::Quad {
        control: [2444, 5844],
        to: [2444, 5920],
    },
    HelpLabelSegment::Line([2444, 6280]),
    HelpLabelSegment::Line([2333, 6280]),
    HelpLabelSegment::Line([2333, 5653]),
    HelpLabelSegment::Line([2444, 5653]),
    HelpLabelSegment::Line([2444, 5753]),
    HelpLabelSegment::Quad {
        control: [2505, 5641],
        to: [2626, 5641],
    },
    HelpLabelSegment::Line([2711, 5652]),
    HelpLabelSegment::Line([2666, 5760]),
];

const BODY_CONTOUR_238: [HelpLabelSegment; 4] = [
    HelpLabelSegment::Line([3164, 5882]),
    HelpLabelSegment::Line([3415, 5882]),
    HelpLabelSegment::Line([3415, 5984]),
    HelpLabelSegment::Line([3164, 5984]),
];

const BODY_CONTOUR_239: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Line([3990, 5522]),
    HelpLabelSegment::Line([4101, 5478]),
    HelpLabelSegment::Line([4101, 5653]),
    HelpLabelSegment::Line([4273, 5653]),
    HelpLabelSegment::Line([4273, 5741]),
    HelpLabelSegment::Line([4101, 5741]),
    HelpLabelSegment::Line([4101, 6053]),
    HelpLabelSegment::Quad {
        control: [4101, 6131],
        to: [4128, 6165],
    },
    HelpLabelSegment::Quad {
        control: [4154, 6198],
        to: [4213, 6198],
    },
    HelpLabelSegment::Quad {
        control: [4256, 6198],
        to: [4301, 6177],
    },
    HelpLabelSegment::Line([4318, 6274]),
    HelpLabelSegment::Line([4166, 6292]),
    HelpLabelSegment::Quad {
        control: [4091, 6292],
        to: [4041, 6237],
    },
    HelpLabelSegment::Quad {
        control: [3990, 6182],
        to: [3990, 6097],
    },
    HelpLabelSegment::Line([3990, 5741]),
    HelpLabelSegment::Line([3917, 5741]),
    HelpLabelSegment::Line([3917, 5653]),
    HelpLabelSegment::Line([3990, 5653]),
];

const BODY_CONTOUR_240: [HelpLabelSegment; 16] = [
    HelpLabelSegment::Line([4536, 5722]),
    HelpLabelSegment::Quad {
        control: [4558, 5687],
        to: [4608, 5665],
    },
    HelpLabelSegment::Quad {
        control: [4657, 5641],
        to: [4709, 5641],
    },
    HelpLabelSegment::Quad {
        control: [4809, 5641],
        to: [4866, 5707],
    },
    HelpLabelSegment::Quad {
        control: [4923, 5773],
        to: [4923, 5886],
    },
    HelpLabelSegment::Line([4923, 6280]),
    HelpLabelSegment::Line([4811, 6280]),
    HelpLabelSegment::Line([4811, 5886]),
    HelpLabelSegment::Quad {
        control: [4811, 5816],
        to: [4776, 5775],
    },
    HelpLabelSegment::Quad {
        control: [4742, 5735],
        to: [4679, 5735],
    },
    HelpLabelSegment::Quad {
        control: [4639, 5735],
        to: [4598, 5759],
    },
    HelpLabelSegment::Quad {
        control: [4557, 5782],
        to: [4536, 5814],
    },
    HelpLabelSegment::Line([4536, 6280]),
    HelpLabelSegment::Line([4425, 6280]),
    HelpLabelSegment::Line([4425, 5395]),
    HelpLabelSegment::Line([4536, 5395]),
];

const BODY_CONTOUR_241: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Quad {
        control: [5411, 5641],
        to: [5473, 5703],
    },
    HelpLabelSegment::Quad {
        control: [5534, 5766],
        to: [5534, 5900],
    },
    HelpLabelSegment::Line([5534, 6125]),
    HelpLabelSegment::Quad {
        control: [5534, 6209],
        to: [5584, 6235],
    },
    HelpLabelSegment::Line([5584, 6292]),
    HelpLabelSegment::Quad {
        control: [5516, 6292],
        to: [5483, 6272],
    },
    HelpLabelSegment::Quad {
        control: [5449, 6253],
        to: [5434, 6209],
    },
    HelpLabelSegment::Quad {
        control: [5367, 6292],
        to: [5230, 6292],
    },
    HelpLabelSegment::Quad {
        control: [5156, 6292],
        to: [5102, 6239],
    },
    HelpLabelSegment::Quad {
        control: [5047, 6185],
        to: [5047, 6105],
    },
    HelpLabelSegment::Quad {
        control: [5047, 6009],
        to: [5131, 5944],
    },
    HelpLabelSegment::Quad {
        control: [5214, 5878],
        to: [5343, 5878],
    },
    HelpLabelSegment::Line([5423, 5893]),
    HelpLabelSegment::Quad {
        control: [5423, 5741],
        to: [5287, 5741],
    },
    HelpLabelSegment::Quad {
        control: [5183, 5741],
        to: [5127, 5797],
    },
    HelpLabelSegment::Line([5080, 5703]),
    HelpLabelSegment::Quad {
        control: [5111, 5678],
        to: [5168, 5660],
    },
    HelpLabelSegment::Quad {
        control: [5224, 5641],
        to: [5274, 5641],
    },
];

const BODY_CONTOUR_242: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [5265, 5960],
        to: [5212, 6003],
    },
    HelpLabelSegment::Quad {
        control: [5158, 6047],
        to: [5158, 6107],
    },
    HelpLabelSegment::Quad {
        control: [5158, 6204],
        to: [5274, 6204],
    },
    HelpLabelSegment::Quad {
        control: [5359, 6204],
        to: [5423, 6124],
    },
    HelpLabelSegment::Line([5423, 5972]),
    HelpLabelSegment::Line([5349, 5960]),
];

const BODY_CONTOUR_243: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Line([5750, 5522]),
    HelpLabelSegment::Line([5861, 5478]),
    HelpLabelSegment::Line([5861, 5653]),
    HelpLabelSegment::Line([6033, 5653]),
    HelpLabelSegment::Line([6033, 5741]),
    HelpLabelSegment::Line([5861, 5741]),
    HelpLabelSegment::Line([5861, 6053]),
    HelpLabelSegment::Quad {
        control: [5861, 6131],
        to: [5888, 6165],
    },
    HelpLabelSegment::Quad {
        control: [5914, 6198],
        to: [5973, 6198],
    },
    HelpLabelSegment::Quad {
        control: [6016, 6198],
        to: [6061, 6177],
    },
    HelpLabelSegment::Line([6078, 6274]),
    HelpLabelSegment::Line([5926, 6292]),
    HelpLabelSegment::Quad {
        control: [5851, 6292],
        to: [5801, 6237],
    },
    HelpLabelSegment::Quad {
        control: [5750, 6182],
        to: [5750, 6097],
    },
    HelpLabelSegment::Line([5750, 5741]),
    HelpLabelSegment::Line([5677, 5741]),
    HelpLabelSegment::Line([5677, 5653]),
    HelpLabelSegment::Line([5750, 5653]),
];

const BODY_CONTOUR_244: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Line([7053, 5395]),
    HelpLabelSegment::Line([7053, 6280]),
    HelpLabelSegment::Line([6942, 6280]),
    HelpLabelSegment::Line([6942, 6233]),
    HelpLabelSegment::Quad {
        control: [6885, 6292],
        to: [6773, 6292],
    },
    HelpLabelSegment::Quad {
        control: [6656, 6292],
        to: [6582, 6207],
    },
    HelpLabelSegment::Quad {
        control: [6510, 6123],
        to: [6510, 5982],
    },
    HelpLabelSegment::Quad {
        control: [6510, 5841],
        to: [6594, 5741],
    },
    HelpLabelSegment::Quad {
        control: [6678, 5641],
        to: [6794, 5641],
    },
    HelpLabelSegment::Quad {
        control: [6892, 5641],
        to: [6942, 5687],
    },
    HelpLabelSegment::Line([6942, 5395]),
];

const BODY_CONTOUR_245: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Quad {
        control: [6933, 6165],
        to: [6942, 6146],
    },
    HelpLabelSegment::Line([6942, 5798]),
    HelpLabelSegment::Quad {
        control: [6900, 5735],
        to: [6827, 5735],
    },
    HelpLabelSegment::Quad {
        control: [6737, 5735],
        to: [6682, 5802],
    },
    HelpLabelSegment::Quad {
        control: [6627, 5869],
        to: [6627, 5972],
    },
    HelpLabelSegment::Quad {
        control: [6627, 6198],
        to: [6833, 6198],
    },
    HelpLabelSegment::Quad {
        control: [6859, 6198],
        to: [6896, 6182],
    },
];

const BODY_CONTOUR_246: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [7399, 5735],
        to: [7348, 5783],
    },
    HelpLabelSegment::Quad {
        control: [7300, 5829],
        to: [7293, 5897],
    },
    HelpLabelSegment::Line([7641, 5897]),
    HelpLabelSegment::Quad {
        control: [7641, 5829],
        to: [7599, 5784],
    },
    HelpLabelSegment::Quad {
        control: [7552, 5735],
        to: [7473, 5735],
    },
];

const BODY_CONTOUR_247: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [7405, 6198],
        to: [7488, 6198],
    },
    HelpLabelSegment::Quad {
        control: [7584, 6198],
        to: [7647, 6143],
    },
    HelpLabelSegment::Line([7694, 6223]),
    HelpLabelSegment::Quad {
        control: [7668, 6248],
        to: [7615, 6267],
    },
    HelpLabelSegment::Quad {
        control: [7549, 6292],
        to: [7467, 6292],
    },
    HelpLabelSegment::Quad {
        control: [7348, 6292],
        to: [7265, 6212],
    },
    HelpLabelSegment::Quad {
        control: [7174, 6123],
        to: [7174, 5974],
    },
    HelpLabelSegment::Quad {
        control: [7174, 5818],
        to: [7267, 5725],
    },
    HelpLabelSegment::Quad {
        control: [7352, 5641],
        to: [7468, 5641],
    },
    HelpLabelSegment::Quad {
        control: [7601, 5641],
        to: [7678, 5716],
    },
    HelpLabelSegment::Quad {
        control: [7751, 5789],
        to: [7751, 5910],
    },
    HelpLabelSegment::Quad {
        control: [7751, 5946],
        to: [7743, 5978],
    },
    HelpLabelSegment::Line([7291, 5978]),
    HelpLabelSegment::Quad {
        control: [7291, 6088],
        to: [7351, 6146],
    },
];

const BODY_CONTOUR_248: [HelpLabelSegment; 12] = [
    HelpLabelSegment::Line([7981, 5705]),
    HelpLabelSegment::Quad {
        control: [8044, 5641],
        to: [8133, 5641],
    },
    HelpLabelSegment::Quad {
        control: [8267, 5641],
        to: [8342, 5725],
    },
    HelpLabelSegment::Quad {
        control: [8416, 5808],
        to: [8416, 5968],
    },
    HelpLabelSegment::Quad {
        control: [8416, 6111],
        to: [8341, 6201],
    },
    HelpLabelSegment::Quad {
        control: [8266, 6292],
        to: [8124, 6292],
    },
    HelpLabelSegment::Quad {
        control: [8084, 6292],
        to: [8040, 6278],
    },
    HelpLabelSegment::Quad {
        control: [7994, 6264],
        to: [7981, 6246],
    },
    HelpLabelSegment::Line([7981, 6526]),
    HelpLabelSegment::Line([7870, 6526]),
    HelpLabelSegment::Line([7870, 5653]),
    HelpLabelSegment::Line([7981, 5653]),
];

const BODY_CONTOUR_249: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Line([7981, 6153]),
    HelpLabelSegment::Quad {
        control: [7992, 6170],
        to: [8026, 6184],
    },
    HelpLabelSegment::Quad {
        control: [8060, 6198],
        to: [8091, 6198],
    },
    HelpLabelSegment::Quad {
        control: [8299, 6198],
        to: [8299, 5964],
    },
    HelpLabelSegment::Quad {
        control: [8299, 5845],
        to: [8249, 5790],
    },
    HelpLabelSegment::Quad {
        control: [8200, 5735],
        to: [8092, 5735],
    },
    HelpLabelSegment::Quad {
        control: [8069, 5735],
        to: [8035, 5751],
    },
    HelpLabelSegment::Line([7981, 5788]),
];

const BODY_CONTOUR_250: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [8724, 5735],
        to: [8673, 5783],
    },
    HelpLabelSegment::Quad {
        control: [8625, 5829],
        to: [8618, 5897],
    },
    HelpLabelSegment::Line([8966, 5897]),
    HelpLabelSegment::Quad {
        control: [8966, 5829],
        to: [8924, 5784],
    },
    HelpLabelSegment::Quad {
        control: [8877, 5735],
        to: [8798, 5735],
    },
];

const BODY_CONTOUR_251: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [8730, 6198],
        to: [8813, 6198],
    },
    HelpLabelSegment::Quad {
        control: [8909, 6198],
        to: [8972, 6143],
    },
    HelpLabelSegment::Line([9019, 6223]),
    HelpLabelSegment::Quad {
        control: [8993, 6248],
        to: [8940, 6267],
    },
    HelpLabelSegment::Quad {
        control: [8874, 6292],
        to: [8792, 6292],
    },
    HelpLabelSegment::Quad {
        control: [8673, 6292],
        to: [8590, 6212],
    },
    HelpLabelSegment::Quad {
        control: [8499, 6123],
        to: [8499, 5974],
    },
    HelpLabelSegment::Quad {
        control: [8499, 5818],
        to: [8592, 5725],
    },
    HelpLabelSegment::Quad {
        control: [8677, 5641],
        to: [8793, 5641],
    },
    HelpLabelSegment::Quad {
        control: [8926, 5641],
        to: [9003, 5716],
    },
    HelpLabelSegment::Quad {
        control: [9076, 5789],
        to: [9076, 5910],
    },
    HelpLabelSegment::Quad {
        control: [9076, 5946],
        to: [9068, 5978],
    },
    HelpLabelSegment::Line([8616, 5978]),
    HelpLabelSegment::Quad {
        control: [8616, 6088],
        to: [8676, 6146],
    },
];

const BODY_CONTOUR_252: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Line([9692, 6280]),
    HelpLabelSegment::Line([9580, 6280]),
    HelpLabelSegment::Line([9580, 5916]),
    HelpLabelSegment::Quad {
        control: [9580, 5815],
        to: [9551, 5775],
    },
    HelpLabelSegment::Quad {
        control: [9520, 5735],
        to: [9449, 5735],
    },
    HelpLabelSegment::Quad {
        control: [9411, 5735],
        to: [9369, 5757],
    },
    HelpLabelSegment::Quad {
        control: [9328, 5781],
        to: [9306, 5814],
    },
    HelpLabelSegment::Line([9306, 6280]),
    HelpLabelSegment::Line([9195, 6280]),
    HelpLabelSegment::Line([9195, 5653]),
    HelpLabelSegment::Line([9271, 5653]),
    HelpLabelSegment::Line([9306, 5734]),
    HelpLabelSegment::Quad {
        control: [9361, 5641],
        to: [9485, 5641],
    },
    HelpLabelSegment::Quad {
        control: [9692, 5641],
        to: [9692, 5892],
    },
];

const BODY_CONTOUR_253: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Line([10358, 5395]),
    HelpLabelSegment::Line([10358, 6280]),
    HelpLabelSegment::Line([10247, 6280]),
    HelpLabelSegment::Line([10247, 6233]),
    HelpLabelSegment::Quad {
        control: [10190, 6292],
        to: [10078, 6292],
    },
    HelpLabelSegment::Quad {
        control: [9961, 6292],
        to: [9887, 6207],
    },
    HelpLabelSegment::Quad {
        control: [9815, 6123],
        to: [9815, 5982],
    },
    HelpLabelSegment::Quad {
        control: [9815, 5841],
        to: [9899, 5741],
    },
    HelpLabelSegment::Quad {
        control: [9983, 5641],
        to: [10099, 5641],
    },
    HelpLabelSegment::Quad {
        control: [10197, 5641],
        to: [10247, 5687],
    },
    HelpLabelSegment::Line([10247, 5395]),
];

const BODY_CONTOUR_254: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Quad {
        control: [10238, 6165],
        to: [10247, 6146],
    },
    HelpLabelSegment::Line([10247, 5798]),
    HelpLabelSegment::Quad {
        control: [10205, 5735],
        to: [10132, 5735],
    },
    HelpLabelSegment::Quad {
        control: [10042, 5735],
        to: [9987, 5802],
    },
    HelpLabelSegment::Quad {
        control: [9932, 5869],
        to: [9932, 5972],
    },
    HelpLabelSegment::Quad {
        control: [9932, 6198],
        to: [10138, 6198],
    },
    HelpLabelSegment::Quad {
        control: [10164, 6198],
        to: [10201, 6182],
    },
];

const BODY_CONTOUR_255: [HelpLabelSegment; 22] = [
    HelpLabelSegment::Line([10827, 5788]),
    HelpLabelSegment::Quad {
        control: [10761, 5735],
        to: [10694, 5735],
    },
    HelpLabelSegment::Quad {
        control: [10654, 5735],
        to: [10628, 5754],
    },
    HelpLabelSegment::Quad {
        control: [10599, 5773],
        to: [10599, 5801],
    },
    HelpLabelSegment::Quad {
        control: [10599, 5862],
        to: [10669, 5892],
    },
    HelpLabelSegment::Line([10748, 5928]),
    HelpLabelSegment::Quad {
        control: [10821, 5962],
        to: [10855, 6005],
    },
    HelpLabelSegment::Quad {
        control: [10888, 6048],
        to: [10888, 6112],
    },
    HelpLabelSegment::Quad {
        control: [10888, 6197],
        to: [10829, 6245],
    },
    HelpLabelSegment::Quad {
        control: [10769, 6292],
        to: [10665, 6292],
    },
    HelpLabelSegment::Quad {
        control: [10565, 6292],
        to: [10479, 6242],
    },
    HelpLabelSegment::Line([10517, 6137]),
    HelpLabelSegment::Quad {
        control: [10611, 6198],
        to: [10667, 6198],
    },
    HelpLabelSegment::Quad {
        control: [10770, 6198],
        to: [10770, 6111],
    },
    HelpLabelSegment::Quad {
        control: [10770, 6049],
        to: [10671, 6005],
    },
    HelpLabelSegment::Quad {
        control: [10595, 5969],
        to: [10568, 5952],
    },
    HelpLabelSegment::Quad {
        control: [10541, 5933],
        to: [10522, 5911],
    },
    HelpLabelSegment::Quad {
        control: [10502, 5887],
        to: [10493, 5862],
    },
    HelpLabelSegment::Quad {
        control: [10482, 5835],
        to: [10482, 5805],
    },
    HelpLabelSegment::Quad {
        control: [10482, 5728],
        to: [10538, 5685],
    },
    HelpLabelSegment::Quad {
        control: [10595, 5641],
        to: [10686, 5641],
    },
    HelpLabelSegment::Quad {
        control: [10754, 5641],
        to: [10858, 5685],
    },
];

const BODY_CONTOUR_256: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [11441, 6201],
        to: [11607, 6201],
    },
    HelpLabelSegment::Quad {
        control: [11686, 6201],
        to: [11730, 6138],
    },
    HelpLabelSegment::Quad {
        control: [11774, 6075],
        to: [11774, 5965],
    },
    HelpLabelSegment::Quad {
        control: [11774, 5732],
        to: [11607, 5732],
    },
    HelpLabelSegment::Quad {
        control: [11531, 5732],
        to: [11487, 5794],
    },
    HelpLabelSegment::Quad {
        control: [11441, 5856],
        to: [11441, 5965],
    },
];

const BODY_CONTOUR_257: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [11480, 5641],
        to: [11607, 5641],
    },
    HelpLabelSegment::Quad {
        control: [11742, 5641],
        to: [11817, 5727],
    },
    HelpLabelSegment::Quad {
        control: [11891, 5812],
        to: [11891, 5965],
    },
    HelpLabelSegment::Quad {
        control: [11891, 6117],
        to: [11815, 6205],
    },
    HelpLabelSegment::Quad {
        control: [11739, 6292],
        to: [11607, 6292],
    },
    HelpLabelSegment::Quad {
        control: [11474, 6292],
        to: [11399, 6204],
    },
    HelpLabelSegment::Quad {
        control: [11324, 6115],
        to: [11324, 5965],
    },
    HelpLabelSegment::Quad {
        control: [11324, 5819],
        to: [11402, 5730],
    },
];

const BODY_CONTOUR_258: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Line([12507, 6280]),
    HelpLabelSegment::Line([12395, 6280]),
    HelpLabelSegment::Line([12395, 5916]),
    HelpLabelSegment::Quad {
        control: [12395, 5815],
        to: [12366, 5775],
    },
    HelpLabelSegment::Quad {
        control: [12335, 5735],
        to: [12264, 5735],
    },
    HelpLabelSegment::Quad {
        control: [12226, 5735],
        to: [12184, 5757],
    },
    HelpLabelSegment::Quad {
        control: [12143, 5781],
        to: [12121, 5814],
    },
    HelpLabelSegment::Line([12121, 6280]),
    HelpLabelSegment::Line([12010, 6280]),
    HelpLabelSegment::Line([12010, 5653]),
    HelpLabelSegment::Line([12086, 5653]),
    HelpLabelSegment::Line([12121, 5734]),
    HelpLabelSegment::Quad {
        control: [12176, 5641],
        to: [12300, 5641],
    },
    HelpLabelSegment::Quad {
        control: [12507, 5641],
        to: [12507, 5892],
    },
];

const BODY_CONTOUR_259: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Line([13065, 5522]),
    HelpLabelSegment::Line([13176, 5478]),
    HelpLabelSegment::Line([13176, 5653]),
    HelpLabelSegment::Line([13348, 5653]),
    HelpLabelSegment::Line([13348, 5741]),
    HelpLabelSegment::Line([13176, 5741]),
    HelpLabelSegment::Line([13176, 6053]),
    HelpLabelSegment::Quad {
        control: [13176, 6131],
        to: [13203, 6165],
    },
    HelpLabelSegment::Quad {
        control: [13229, 6198],
        to: [13288, 6198],
    },
    HelpLabelSegment::Quad {
        control: [13331, 6198],
        to: [13376, 6177],
    },
    HelpLabelSegment::Line([13393, 6274]),
    HelpLabelSegment::Line([13241, 6292]),
    HelpLabelSegment::Quad {
        control: [13166, 6292],
        to: [13116, 6237],
    },
    HelpLabelSegment::Quad {
        control: [13065, 6182],
        to: [13065, 6097],
    },
    HelpLabelSegment::Line([13065, 5741]),
    HelpLabelSegment::Line([12992, 5741]),
    HelpLabelSegment::Line([12992, 5653]),
    HelpLabelSegment::Line([13065, 5653]),
];

const BODY_CONTOUR_260: [HelpLabelSegment; 16] = [
    HelpLabelSegment::Line([13611, 5722]),
    HelpLabelSegment::Quad {
        control: [13633, 5687],
        to: [13682, 5665],
    },
    HelpLabelSegment::Quad {
        control: [13732, 5641],
        to: [13784, 5641],
    },
    HelpLabelSegment::Quad {
        control: [13884, 5641],
        to: [13941, 5707],
    },
    HelpLabelSegment::Quad {
        control: [13998, 5773],
        to: [13998, 5886],
    },
    HelpLabelSegment::Line([13998, 6280]),
    HelpLabelSegment::Line([13886, 6280]),
    HelpLabelSegment::Line([13886, 5886]),
    HelpLabelSegment::Quad {
        control: [13886, 5816],
        to: [13851, 5775],
    },
    HelpLabelSegment::Quad {
        control: [13817, 5735],
        to: [13754, 5735],
    },
    HelpLabelSegment::Quad {
        control: [13714, 5735],
        to: [13673, 5759],
    },
    HelpLabelSegment::Quad {
        control: [13632, 5782],
        to: [13611, 5814],
    },
    HelpLabelSegment::Line([13611, 6280]),
    HelpLabelSegment::Line([13500, 6280]),
    HelpLabelSegment::Line([13500, 5395]),
    HelpLabelSegment::Line([13611, 5395]),
];

const BODY_CONTOUR_261: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [14339, 5735],
        to: [14288, 5783],
    },
    HelpLabelSegment::Quad {
        control: [14240, 5829],
        to: [14233, 5897],
    },
    HelpLabelSegment::Line([14581, 5897]),
    HelpLabelSegment::Quad {
        control: [14581, 5829],
        to: [14539, 5784],
    },
    HelpLabelSegment::Quad {
        control: [14492, 5735],
        to: [14412, 5735],
    },
];

const BODY_CONTOUR_262: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [14345, 6198],
        to: [14428, 6198],
    },
    HelpLabelSegment::Quad {
        control: [14524, 6198],
        to: [14587, 6143],
    },
    HelpLabelSegment::Line([14634, 6223]),
    HelpLabelSegment::Quad {
        control: [14608, 6248],
        to: [14555, 6267],
    },
    HelpLabelSegment::Quad {
        control: [14489, 6292],
        to: [14407, 6292],
    },
    HelpLabelSegment::Quad {
        control: [14288, 6292],
        to: [14205, 6212],
    },
    HelpLabelSegment::Quad {
        control: [14114, 6123],
        to: [14114, 5974],
    },
    HelpLabelSegment::Quad {
        control: [14114, 5818],
        to: [14207, 5725],
    },
    HelpLabelSegment::Quad {
        control: [14292, 5641],
        to: [14408, 5641],
    },
    HelpLabelSegment::Quad {
        control: [14541, 5641],
        to: [14618, 5716],
    },
    HelpLabelSegment::Quad {
        control: [14691, 5789],
        to: [14691, 5910],
    },
    HelpLabelSegment::Quad {
        control: [14691, 5946],
        to: [14683, 5978],
    },
    HelpLabelSegment::Line([14231, 5978]),
    HelpLabelSegment::Quad {
        control: [14231, 6088],
        to: [14291, 6146],
    },
];

const BODY_CONTOUR_263: [HelpLabelSegment; 32] = [
    HelpLabelSegment::Line([15581, 5732]),
    HelpLabelSegment::Quad {
        control: [15624, 5787],
        to: [15624, 5877],
    },
    HelpLabelSegment::Quad {
        control: [15624, 5972],
        to: [15565, 6036],
    },
    HelpLabelSegment::Quad {
        control: [15506, 6101],
        to: [15410, 6110],
    },
    HelpLabelSegment::Line([15317, 6119]),
    HelpLabelSegment::Line([15274, 6132]),
    HelpLabelSegment::Quad {
        control: [15246, 6143],
        to: [15246, 6160],
    },
    HelpLabelSegment::Quad {
        control: [15246, 6184],
        to: [15303, 6184],
    },
    HelpLabelSegment::Line([15382, 6176]),
    HelpLabelSegment::Line([15461, 6166]),
    HelpLabelSegment::Quad {
        control: [15554, 6166],
        to: [15606, 6211],
    },
    HelpLabelSegment::Quad {
        control: [15657, 6254],
        to: [15657, 6333],
    },
    HelpLabelSegment::Quad {
        control: [15657, 6419],
        to: [15580, 6473],
    },
    HelpLabelSegment::Quad {
        control: [15503, 6526],
        to: [15383, 6526],
    },
    HelpLabelSegment::Quad {
        control: [15322, 6526],
        to: [15255, 6505],
    },
    HelpLabelSegment::Quad {
        control: [15187, 6483],
        to: [15146, 6452],
    },
    HelpLabelSegment::Line([15207, 6363]),
    HelpLabelSegment::Quad {
        control: [15304, 6428],
        to: [15386, 6428],
    },
    HelpLabelSegment::Quad {
        control: [15461, 6428],
        to: [15506, 6402],
    },
    HelpLabelSegment::Quad {
        control: [15549, 6376],
        to: [15549, 6337],
    },
    HelpLabelSegment::Quad {
        control: [15549, 6261],
        to: [15439, 6261],
    },
    HelpLabelSegment::Line([15371, 6271]),
    HelpLabelSegment::Line([15294, 6280]),
    HelpLabelSegment::Quad {
        control: [15160, 6280],
        to: [15160, 6179],
    },
    HelpLabelSegment::Quad {
        control: [15160, 6148],
        to: [15192, 6123],
    },
    HelpLabelSegment::Quad {
        control: [15224, 6097],
        to: [15269, 6087],
    },
    HelpLabelSegment::Quad {
        control: [15137, 6025],
        to: [15137, 5871],
    },
    HelpLabelSegment::Quad {
        control: [15137, 5773],
        to: [15206, 5707],
    },
    HelpLabelSegment::Quad {
        control: [15274, 5641],
        to: [15375, 5641],
    },
    HelpLabelSegment::Quad {
        control: [15467, 5641],
        to: [15520, 5679],
    },
    HelpLabelSegment::Line([15575, 5612]),
    HelpLabelSegment::Line([15648, 5681]),
];

const BODY_CONTOUR_264: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [15324, 5730],
        to: [15288, 5771],
    },
    HelpLabelSegment::Quad {
        control: [15252, 5812],
        to: [15252, 5871],
    },
    HelpLabelSegment::Quad {
        control: [15252, 5937],
        to: [15287, 5980],
    },
    HelpLabelSegment::Quad {
        control: [15322, 6023],
        to: [15383, 6023],
    },
    HelpLabelSegment::Quad {
        control: [15442, 6023],
        to: [15476, 5981],
    },
    HelpLabelSegment::Quad {
        control: [15508, 5939],
        to: [15508, 5871],
    },
    HelpLabelSegment::Quad {
        control: [15508, 5812],
        to: [15473, 5771],
    },
    HelpLabelSegment::Quad {
        control: [15437, 5730],
        to: [15383, 5730],
    },
];

const BODY_CONTOUR_265: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Quad {
        control: [16074, 5735],
        to: [16037, 5735],
    },
    HelpLabelSegment::Quad {
        control: [15978, 5735],
        to: [15934, 5789],
    },
    HelpLabelSegment::Quad {
        control: [15889, 5844],
        to: [15889, 5920],
    },
    HelpLabelSegment::Line([15889, 6280]),
    HelpLabelSegment::Line([15778, 6280]),
    HelpLabelSegment::Line([15778, 5653]),
    HelpLabelSegment::Line([15889, 5653]),
    HelpLabelSegment::Line([15889, 5753]),
    HelpLabelSegment::Quad {
        control: [15950, 5641],
        to: [16071, 5641],
    },
    HelpLabelSegment::Line([16156, 5652]),
    HelpLabelSegment::Line([16111, 5760]),
];

const BODY_CONTOUR_266: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Quad {
        control: [16566, 5641],
        to: [16628, 5703],
    },
    HelpLabelSegment::Quad {
        control: [16689, 5766],
        to: [16689, 5900],
    },
    HelpLabelSegment::Line([16689, 6125]),
    HelpLabelSegment::Quad {
        control: [16689, 6209],
        to: [16739, 6235],
    },
    HelpLabelSegment::Line([16739, 6292]),
    HelpLabelSegment::Quad {
        control: [16671, 6292],
        to: [16638, 6272],
    },
    HelpLabelSegment::Quad {
        control: [16604, 6253],
        to: [16589, 6209],
    },
    HelpLabelSegment::Quad {
        control: [16522, 6292],
        to: [16385, 6292],
    },
    HelpLabelSegment::Quad {
        control: [16311, 6292],
        to: [16257, 6239],
    },
    HelpLabelSegment::Quad {
        control: [16202, 6185],
        to: [16202, 6105],
    },
    HelpLabelSegment::Quad {
        control: [16202, 6009],
        to: [16286, 5944],
    },
    HelpLabelSegment::Quad {
        control: [16369, 5878],
        to: [16498, 5878],
    },
    HelpLabelSegment::Line([16578, 5893]),
    HelpLabelSegment::Quad {
        control: [16578, 5741],
        to: [16442, 5741],
    },
    HelpLabelSegment::Quad {
        control: [16338, 5741],
        to: [16282, 5797],
    },
    HelpLabelSegment::Line([16235, 5703]),
    HelpLabelSegment::Quad {
        control: [16266, 5678],
        to: [16323, 5660],
    },
    HelpLabelSegment::Quad {
        control: [16379, 5641],
        to: [16429, 5641],
    },
];

const BODY_CONTOUR_267: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [16420, 5960],
        to: [16367, 6003],
    },
    HelpLabelSegment::Quad {
        control: [16313, 6047],
        to: [16313, 6107],
    },
    HelpLabelSegment::Quad {
        control: [16313, 6204],
        to: [16429, 6204],
    },
    HelpLabelSegment::Quad {
        control: [16514, 6204],
        to: [16578, 6124],
    },
    HelpLabelSegment::Line([16578, 5972]),
    HelpLabelSegment::Line([16504, 5960]),
];

const BODY_CONTOUR_268: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([17091, 6292]),
    HelpLabelSegment::Line([17062, 6292]),
    HelpLabelSegment::Line([16792, 5651]),
    HelpLabelSegment::Line([16914, 5651]),
    HelpLabelSegment::Line([17080, 6090]),
    HelpLabelSegment::Line([17249, 5651]),
    HelpLabelSegment::Line([17366, 5651]),
];

const BODY_CONTOUR_269: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [17598, 5415],
        to: [17619, 5436],
    },
    HelpLabelSegment::Quad {
        control: [17639, 5456],
        to: [17639, 5484],
    },
    HelpLabelSegment::Quad {
        control: [17639, 5512],
        to: [17619, 5534],
    },
    HelpLabelSegment::Quad {
        control: [17598, 5553],
        to: [17570, 5553],
    },
    HelpLabelSegment::Quad {
        control: [17541, 5553],
        to: [17521, 5534],
    },
    HelpLabelSegment::Quad {
        control: [17500, 5512],
        to: [17500, 5484],
    },
    HelpLabelSegment::Quad {
        control: [17500, 5455],
        to: [17520, 5435],
    },
    HelpLabelSegment::Quad {
        control: [17540, 5415],
        to: [17570, 5415],
    },
];

const BODY_CONTOUR_270: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Line([17620, 6280]),
    HelpLabelSegment::Line([17509, 6280]),
    HelpLabelSegment::Line([17509, 5747]),
    HelpLabelSegment::Line([17422, 5747]),
    HelpLabelSegment::Line([17422, 5653]),
    HelpLabelSegment::Line([17620, 5653]),
];

const BODY_CONTOUR_271: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Line([17835, 5522]),
    HelpLabelSegment::Line([17946, 5478]),
    HelpLabelSegment::Line([17946, 5653]),
    HelpLabelSegment::Line([18118, 5653]),
    HelpLabelSegment::Line([18118, 5741]),
    HelpLabelSegment::Line([17946, 5741]),
    HelpLabelSegment::Line([17946, 6053]),
    HelpLabelSegment::Quad {
        control: [17946, 6131],
        to: [17973, 6165],
    },
    HelpLabelSegment::Quad {
        control: [17999, 6198],
        to: [18058, 6198],
    },
    HelpLabelSegment::Quad {
        control: [18101, 6198],
        to: [18146, 6177],
    },
    HelpLabelSegment::Line([18163, 6274]),
    HelpLabelSegment::Line([18011, 6292]),
    HelpLabelSegment::Quad {
        control: [17936, 6292],
        to: [17886, 6237],
    },
    HelpLabelSegment::Quad {
        control: [17835, 6182],
        to: [17835, 6097],
    },
    HelpLabelSegment::Line([17835, 5741]),
    HelpLabelSegment::Line([17762, 5741]),
    HelpLabelSegment::Line([17762, 5653]),
    HelpLabelSegment::Line([17835, 5653]),
];

const BODY_CONTOUR_272: [HelpLabelSegment; 13] = [
    HelpLabelSegment::Line([18509, 6387]),
    HelpLabelSegment::Quad {
        control: [18488, 6446],
        to: [18419, 6486],
    },
    HelpLabelSegment::Quad {
        control: [18348, 6526],
        to: [18263, 6526],
    },
    HelpLabelSegment::Line([18263, 6426]),
    HelpLabelSegment::Quad {
        control: [18333, 6426],
        to: [18382, 6395],
    },
    HelpLabelSegment::Quad {
        control: [18433, 6362],
        to: [18433, 6315],
    },
    HelpLabelSegment::Quad {
        control: [18433, 6264],
        to: [18414, 6213],
    },
    HelpLabelSegment::Line([18367, 6089]),
    HelpLabelSegment::Line([18197, 5653]),
    HelpLabelSegment::Line([18311, 5653]),
    HelpLabelSegment::Line([18496, 6138]),
    HelpLabelSegment::Line([18661, 5653]),
    HelpLabelSegment::Line([18775, 5653]),
];

const BODY_CONTOUR_273: [HelpLabelSegment; 24] = [
    HelpLabelSegment::Quad {
        control: [20058, 5756],
        to: [20058, 5860],
    },
    HelpLabelSegment::Line([20058, 6280]),
    HelpLabelSegment::Line([19946, 6280]),
    HelpLabelSegment::Line([19946, 5883]),
    HelpLabelSegment::Quad {
        control: [19946, 5735],
        to: [19817, 5735],
    },
    HelpLabelSegment::Quad {
        control: [19777, 5735],
        to: [19742, 5760],
    },
    HelpLabelSegment::Quad {
        control: [19707, 5784],
        to: [19694, 5816],
    },
    HelpLabelSegment::Line([19694, 6280]),
    HelpLabelSegment::Line([19583, 6280]),
    HelpLabelSegment::Line([19583, 5835]),
    HelpLabelSegment::Quad {
        control: [19583, 5788],
        to: [19548, 5762],
    },
    HelpLabelSegment::Quad {
        control: [19513, 5735],
        to: [19455, 5735],
    },
    HelpLabelSegment::Quad {
        control: [19422, 5735],
        to: [19385, 5761],
    },
    HelpLabelSegment::Quad {
        control: [19346, 5787],
        to: [19331, 5817],
    },
    HelpLabelSegment::Line([19331, 6280]),
    HelpLabelSegment::Line([19220, 6280]),
    HelpLabelSegment::Line([19220, 5653]),
    HelpLabelSegment::Line([19292, 5653]),
    HelpLabelSegment::Line([19329, 5726]),
    HelpLabelSegment::Quad {
        control: [19393, 5641],
        to: [19490, 5641],
    },
    HelpLabelSegment::Quad {
        control: [19625, 5641],
        to: [19679, 5725],
    },
    HelpLabelSegment::Quad {
        control: [19698, 5689],
        to: [19748, 5665],
    },
    HelpLabelSegment::Quad {
        control: [19800, 5641],
        to: [19854, 5641],
    },
    HelpLabelSegment::Quad {
        control: [19951, 5641],
        to: [20004, 5699],
    },
];

const BODY_CONTOUR_274: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [20291, 6201],
        to: [20457, 6201],
    },
    HelpLabelSegment::Quad {
        control: [20536, 6201],
        to: [20580, 6138],
    },
    HelpLabelSegment::Quad {
        control: [20624, 6075],
        to: [20624, 5965],
    },
    HelpLabelSegment::Quad {
        control: [20624, 5732],
        to: [20457, 5732],
    },
    HelpLabelSegment::Quad {
        control: [20381, 5732],
        to: [20337, 5794],
    },
    HelpLabelSegment::Quad {
        control: [20291, 5856],
        to: [20291, 5965],
    },
];

const BODY_CONTOUR_275: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [20330, 5641],
        to: [20457, 5641],
    },
    HelpLabelSegment::Quad {
        control: [20592, 5641],
        to: [20667, 5727],
    },
    HelpLabelSegment::Quad {
        control: [20741, 5812],
        to: [20741, 5965],
    },
    HelpLabelSegment::Quad {
        control: [20741, 6117],
        to: [20665, 6205],
    },
    HelpLabelSegment::Quad {
        control: [20589, 6292],
        to: [20457, 6292],
    },
    HelpLabelSegment::Quad {
        control: [20324, 6292],
        to: [20249, 6204],
    },
    HelpLabelSegment::Quad {
        control: [20174, 6115],
        to: [20174, 5965],
    },
    HelpLabelSegment::Quad {
        control: [20174, 5819],
        to: [20252, 5730],
    },
];

const BODY_CONTOUR_276: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Line([21368, 5395]),
    HelpLabelSegment::Line([21368, 6280]),
    HelpLabelSegment::Line([21257, 6280]),
    HelpLabelSegment::Line([21257, 6233]),
    HelpLabelSegment::Quad {
        control: [21200, 6292],
        to: [21088, 6292],
    },
    HelpLabelSegment::Quad {
        control: [20971, 6292],
        to: [20897, 6207],
    },
    HelpLabelSegment::Quad {
        control: [20825, 6123],
        to: [20825, 5982],
    },
    HelpLabelSegment::Quad {
        control: [20825, 5841],
        to: [20909, 5741],
    },
    HelpLabelSegment::Quad {
        control: [20993, 5641],
        to: [21109, 5641],
    },
    HelpLabelSegment::Quad {
        control: [21207, 5641],
        to: [21257, 5687],
    },
    HelpLabelSegment::Line([21257, 5395]),
];

const BODY_CONTOUR_277: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Quad {
        control: [21248, 6165],
        to: [21257, 6146],
    },
    HelpLabelSegment::Line([21257, 5798]),
    HelpLabelSegment::Quad {
        control: [21215, 5735],
        to: [21142, 5735],
    },
    HelpLabelSegment::Quad {
        control: [21052, 5735],
        to: [20997, 5802],
    },
    HelpLabelSegment::Quad {
        control: [20942, 5869],
        to: [20942, 5972],
    },
    HelpLabelSegment::Quad {
        control: [20942, 6198],
        to: [21148, 6198],
    },
    HelpLabelSegment::Quad {
        control: [21174, 6198],
        to: [21211, 6182],
    },
];

const BODY_CONTOUR_278: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [21714, 5735],
        to: [21663, 5783],
    },
    HelpLabelSegment::Quad {
        control: [21615, 5829],
        to: [21608, 5897],
    },
    HelpLabelSegment::Line([21956, 5897]),
    HelpLabelSegment::Quad {
        control: [21956, 5829],
        to: [21914, 5784],
    },
    HelpLabelSegment::Quad {
        control: [21867, 5735],
        to: [21787, 5735],
    },
];

const BODY_CONTOUR_279: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [21720, 6198],
        to: [21803, 6198],
    },
    HelpLabelSegment::Quad {
        control: [21899, 6198],
        to: [21962, 6143],
    },
    HelpLabelSegment::Line([22009, 6223]),
    HelpLabelSegment::Quad {
        control: [21983, 6248],
        to: [21930, 6267],
    },
    HelpLabelSegment::Quad {
        control: [21864, 6292],
        to: [21782, 6292],
    },
    HelpLabelSegment::Quad {
        control: [21663, 6292],
        to: [21580, 6212],
    },
    HelpLabelSegment::Quad {
        control: [21489, 6123],
        to: [21489, 5974],
    },
    HelpLabelSegment::Quad {
        control: [21489, 5818],
        to: [21582, 5725],
    },
    HelpLabelSegment::Quad {
        control: [21667, 5641],
        to: [21783, 5641],
    },
    HelpLabelSegment::Quad {
        control: [21916, 5641],
        to: [21993, 5716],
    },
    HelpLabelSegment::Quad {
        control: [22066, 5789],
        to: [22066, 5910],
    },
    HelpLabelSegment::Quad {
        control: [22066, 5946],
        to: [22058, 5978],
    },
    HelpLabelSegment::Line([21606, 5978]),
    HelpLabelSegment::Quad {
        control: [21606, 6088],
        to: [21666, 6146],
    },
];

const BODY_CONTOUR_280: [HelpLabelSegment; 13] = [
    HelpLabelSegment::Line([22784, 6387]),
    HelpLabelSegment::Quad {
        control: [22763, 6446],
        to: [22694, 6486],
    },
    HelpLabelSegment::Quad {
        control: [22623, 6526],
        to: [22538, 6526],
    },
    HelpLabelSegment::Line([22538, 6426]),
    HelpLabelSegment::Quad {
        control: [22608, 6426],
        to: [22657, 6395],
    },
    HelpLabelSegment::Quad {
        control: [22708, 6362],
        to: [22708, 6315],
    },
    HelpLabelSegment::Quad {
        control: [22708, 6264],
        to: [22689, 6213],
    },
    HelpLabelSegment::Line([22642, 6089]),
    HelpLabelSegment::Line([22472, 5653]),
    HelpLabelSegment::Line([22586, 5653]),
    HelpLabelSegment::Line([22771, 6138]),
    HelpLabelSegment::Line([22936, 5653]),
    HelpLabelSegment::Line([23050, 5653]),
];

const BODY_CONTOUR_281: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [23211, 6201],
        to: [23377, 6201],
    },
    HelpLabelSegment::Quad {
        control: [23456, 6201],
        to: [23500, 6138],
    },
    HelpLabelSegment::Quad {
        control: [23544, 6075],
        to: [23544, 5965],
    },
    HelpLabelSegment::Quad {
        control: [23544, 5732],
        to: [23377, 5732],
    },
    HelpLabelSegment::Quad {
        control: [23301, 5732],
        to: [23257, 5794],
    },
    HelpLabelSegment::Quad {
        control: [23211, 5856],
        to: [23211, 5965],
    },
];

const BODY_CONTOUR_282: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [23250, 5641],
        to: [23377, 5641],
    },
    HelpLabelSegment::Quad {
        control: [23512, 5641],
        to: [23587, 5727],
    },
    HelpLabelSegment::Quad {
        control: [23661, 5812],
        to: [23661, 5965],
    },
    HelpLabelSegment::Quad {
        control: [23661, 6117],
        to: [23585, 6205],
    },
    HelpLabelSegment::Quad {
        control: [23509, 6292],
        to: [23377, 6292],
    },
    HelpLabelSegment::Quad {
        control: [23244, 6292],
        to: [23169, 6204],
    },
    HelpLabelSegment::Quad {
        control: [23094, 6115],
        to: [23094, 5965],
    },
    HelpLabelSegment::Quad {
        control: [23094, 5819],
        to: [23172, 5730],
    },
];

const BODY_CONTOUR_283: [HelpLabelSegment; 15] = [
    HelpLabelSegment::Line([23885, 6053]),
    HelpLabelSegment::Quad {
        control: [23885, 6198],
        to: [24011, 6198],
    },
    HelpLabelSegment::Quad {
        control: [24066, 6198],
        to: [24111, 6166],
    },
    HelpLabelSegment::Quad {
        control: [24157, 6135],
        to: [24172, 6094],
    },
    HelpLabelSegment::Line([24172, 5653]),
    HelpLabelSegment::Line([24284, 5653]),
    HelpLabelSegment::Line([24284, 6280]),
    HelpLabelSegment::Line([24172, 6280]),
    HelpLabelSegment::Line([24172, 6193]),
    HelpLabelSegment::Quad {
        control: [24154, 6231],
        to: [24097, 6261],
    },
    HelpLabelSegment::Quad {
        control: [24040, 6292],
        to: [23986, 6292],
    },
    HelpLabelSegment::Quad {
        control: [23883, 6292],
        to: [23829, 6233],
    },
    HelpLabelSegment::Quad {
        control: [23774, 6173],
        to: [23774, 6064],
    },
    HelpLabelSegment::Line([23774, 5653]),
    HelpLabelSegment::Line([23885, 5653]),
];

const BODY_CONTOUR_284: [HelpLabelSegment; 12] = [
    HelpLabelSegment::Line([24906, 5705]),
    HelpLabelSegment::Quad {
        control: [24969, 5641],
        to: [25058, 5641],
    },
    HelpLabelSegment::Quad {
        control: [25192, 5641],
        to: [25267, 5725],
    },
    HelpLabelSegment::Quad {
        control: [25341, 5808],
        to: [25341, 5968],
    },
    HelpLabelSegment::Quad {
        control: [25341, 6111],
        to: [25266, 6201],
    },
    HelpLabelSegment::Quad {
        control: [25191, 6292],
        to: [25049, 6292],
    },
    HelpLabelSegment::Quad {
        control: [25009, 6292],
        to: [24965, 6278],
    },
    HelpLabelSegment::Quad {
        control: [24919, 6264],
        to: [24906, 6246],
    },
    HelpLabelSegment::Line([24906, 6526]),
    HelpLabelSegment::Line([24795, 6526]),
    HelpLabelSegment::Line([24795, 5653]),
    HelpLabelSegment::Line([24906, 5653]),
];

const BODY_CONTOUR_285: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Line([24906, 6153]),
    HelpLabelSegment::Quad {
        control: [24917, 6170],
        to: [24951, 6184],
    },
    HelpLabelSegment::Quad {
        control: [24985, 6198],
        to: [25016, 6198],
    },
    HelpLabelSegment::Quad {
        control: [25224, 6198],
        to: [25224, 5964],
    },
    HelpLabelSegment::Quad {
        control: [25224, 5845],
        to: [25174, 5790],
    },
    HelpLabelSegment::Quad {
        control: [25125, 5735],
        to: [25017, 5735],
    },
    HelpLabelSegment::Quad {
        control: [24994, 5735],
        to: [24960, 5751],
    },
    HelpLabelSegment::Line([24906, 5788]),
];

const BODY_CONTOUR_286: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([25690, 6292]),
    HelpLabelSegment::Quad {
        control: [25473, 6292],
        to: [25473, 6103],
    },
    HelpLabelSegment::Line([25473, 5395]),
    HelpLabelSegment::Line([25584, 5395]),
    HelpLabelSegment::Line([25584, 6084]),
    HelpLabelSegment::Quad {
        control: [25584, 6135],
        to: [25614, 6164],
    },
    HelpLabelSegment::Quad {
        control: [25643, 6192],
        to: [25690, 6192],
    },
];

const BODY_CONTOUR_287: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Quad {
        control: [26151, 5641],
        to: [26213, 5703],
    },
    HelpLabelSegment::Quad {
        control: [26274, 5766],
        to: [26274, 5900],
    },
    HelpLabelSegment::Line([26274, 6125]),
    HelpLabelSegment::Quad {
        control: [26274, 6209],
        to: [26324, 6235],
    },
    HelpLabelSegment::Line([26324, 6292]),
    HelpLabelSegment::Quad {
        control: [26256, 6292],
        to: [26223, 6272],
    },
    HelpLabelSegment::Quad {
        control: [26189, 6253],
        to: [26174, 6209],
    },
    HelpLabelSegment::Quad {
        control: [26107, 6292],
        to: [25970, 6292],
    },
    HelpLabelSegment::Quad {
        control: [25896, 6292],
        to: [25842, 6239],
    },
    HelpLabelSegment::Quad {
        control: [25787, 6185],
        to: [25787, 6105],
    },
    HelpLabelSegment::Quad {
        control: [25787, 6009],
        to: [25871, 5944],
    },
    HelpLabelSegment::Quad {
        control: [25954, 5878],
        to: [26083, 5878],
    },
    HelpLabelSegment::Line([26163, 5893]),
    HelpLabelSegment::Quad {
        control: [26163, 5741],
        to: [26027, 5741],
    },
    HelpLabelSegment::Quad {
        control: [25923, 5741],
        to: [25867, 5797],
    },
    HelpLabelSegment::Line([25820, 5703]),
    HelpLabelSegment::Quad {
        control: [25851, 5678],
        to: [25908, 5660],
    },
    HelpLabelSegment::Quad {
        control: [25964, 5641],
        to: [26014, 5641],
    },
];

const BODY_CONTOUR_288: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [26005, 5960],
        to: [25952, 6003],
    },
    HelpLabelSegment::Quad {
        control: [25898, 6047],
        to: [25898, 6107],
    },
    HelpLabelSegment::Quad {
        control: [25898, 6204],
        to: [26014, 6204],
    },
    HelpLabelSegment::Quad {
        control: [26099, 6204],
        to: [26163, 6124],
    },
    HelpLabelSegment::Line([26163, 5972]),
    HelpLabelSegment::Line([26089, 5960]),
];

const BODY_CONTOUR_289: [HelpLabelSegment; 13] = [
    HelpLabelSegment::Line([26689, 6387]),
    HelpLabelSegment::Quad {
        control: [26668, 6446],
        to: [26599, 6486],
    },
    HelpLabelSegment::Quad {
        control: [26528, 6526],
        to: [26443, 6526],
    },
    HelpLabelSegment::Line([26443, 6426]),
    HelpLabelSegment::Quad {
        control: [26513, 6426],
        to: [26562, 6395],
    },
    HelpLabelSegment::Quad {
        control: [26613, 6362],
        to: [26613, 6315],
    },
    HelpLabelSegment::Quad {
        control: [26613, 6264],
        to: [26594, 6213],
    },
    HelpLabelSegment::Line([26547, 6089]),
    HelpLabelSegment::Line([26377, 5653]),
    HelpLabelSegment::Line([26491, 5653]),
    HelpLabelSegment::Line([26676, 6138]),
    HelpLabelSegment::Line([26841, 5653]),
    HelpLabelSegment::Line([26955, 5653]),
];

const BODY_CONTOUR_290: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [27059, 6116],
        to: [27085, 6142],
    },
    HelpLabelSegment::Quad {
        control: [27110, 6168],
        to: [27110, 6204],
    },
    HelpLabelSegment::Quad {
        control: [27110, 6240],
        to: [27085, 6266],
    },
    HelpLabelSegment::Quad {
        control: [27059, 6292],
        to: [27022, 6292],
    },
    HelpLabelSegment::Quad {
        control: [26986, 6292],
        to: [26960, 6266],
    },
    HelpLabelSegment::Quad {
        control: [26935, 6240],
        to: [26935, 6204],
    },
    HelpLabelSegment::Quad {
        control: [26935, 6168],
        to: [26960, 6142],
    },
    HelpLabelSegment::Quad {
        control: [26986, 6116],
        to: [27022, 6116],
    },
];

const BODY_CONTOUR_291: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Line([-10913, 7657]),
    HelpLabelSegment::Line([-10826, 7657]),
    HelpLabelSegment::Line([-10826, 7747]),
    HelpLabelSegment::Line([-10913, 7747]),
    HelpLabelSegment::Line([-10913, 7980]),
    HelpLabelSegment::Line([-11025, 7980]),
    HelpLabelSegment::Line([-11025, 7747]),
    HelpLabelSegment::Line([-11415, 7747]),
    HelpLabelSegment::Line([-11415, 7680]),
    HelpLabelSegment::Line([-10954, 7119]),
    HelpLabelSegment::Line([-10913, 7119]),
];

const BODY_CONTOUR_292: [HelpLabelSegment; 3] = [
    HelpLabelSegment::Line([-11025, 7352]),
    HelpLabelSegment::Line([-11276, 7657]),
    HelpLabelSegment::Line([-11025, 7657]),
];

const BODY_CONTOUR_293: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [-10556, 7816],
        to: [-10530, 7842],
    },
    HelpLabelSegment::Quad {
        control: [-10505, 7868],
        to: [-10505, 7904],
    },
    HelpLabelSegment::Quad {
        control: [-10505, 7940],
        to: [-10530, 7966],
    },
    HelpLabelSegment::Quad {
        control: [-10556, 7992],
        to: [-10593, 7992],
    },
    HelpLabelSegment::Quad {
        control: [-10629, 7992],
        to: [-10655, 7966],
    },
    HelpLabelSegment::Quad {
        control: [-10680, 7940],
        to: [-10680, 7904],
    },
    HelpLabelSegment::Quad {
        control: [-10680, 7868],
        to: [-10655, 7842],
    },
    HelpLabelSegment::Quad {
        control: [-10629, 7816],
        to: [-10593, 7816],
    },
];

const BODY_CONTOUR_294: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Line([-9619, 7980]),
    HelpLabelSegment::Line([-9736, 7980]),
    HelpLabelSegment::Line([-9736, 7228]),
    HelpLabelSegment::Line([-10009, 7228]),
    HelpLabelSegment::Line([-10009, 7122]),
    HelpLabelSegment::Line([-9333, 7122]),
    HelpLabelSegment::Line([-9333, 7228]),
    HelpLabelSegment::Line([-9619, 7228]),
];

const BODY_CONTOUR_295: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [-9319, 7901],
        to: [-9153, 7901],
    },
    HelpLabelSegment::Quad {
        control: [-9074, 7901],
        to: [-9030, 7838],
    },
    HelpLabelSegment::Quad {
        control: [-8986, 7775],
        to: [-8986, 7665],
    },
    HelpLabelSegment::Quad {
        control: [-8986, 7432],
        to: [-9153, 7432],
    },
    HelpLabelSegment::Quad {
        control: [-9229, 7432],
        to: [-9273, 7494],
    },
    HelpLabelSegment::Quad {
        control: [-9319, 7556],
        to: [-9319, 7665],
    },
];

const BODY_CONTOUR_296: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [-9280, 7341],
        to: [-9153, 7341],
    },
    HelpLabelSegment::Quad {
        control: [-9018, 7341],
        to: [-8943, 7427],
    },
    HelpLabelSegment::Quad {
        control: [-8869, 7512],
        to: [-8869, 7665],
    },
    HelpLabelSegment::Quad {
        control: [-8869, 7817],
        to: [-8945, 7905],
    },
    HelpLabelSegment::Quad {
        control: [-9021, 7992],
        to: [-9153, 7992],
    },
    HelpLabelSegment::Quad {
        control: [-9286, 7992],
        to: [-9361, 7904],
    },
    HelpLabelSegment::Quad {
        control: [-9436, 7815],
        to: [-9436, 7665],
    },
    HelpLabelSegment::Quad {
        control: [-9436, 7519],
        to: [-9358, 7430],
    },
];

const BODY_CONTOUR_297: [HelpLabelSegment; 15] = [
    HelpLabelSegment::Line([-8645, 7753]),
    HelpLabelSegment::Quad {
        control: [-8645, 7898],
        to: [-8519, 7898],
    },
    HelpLabelSegment::Quad {
        control: [-8464, 7898],
        to: [-8419, 7866],
    },
    HelpLabelSegment::Quad {
        control: [-8373, 7835],
        to: [-8358, 7794],
    },
    HelpLabelSegment::Line([-8358, 7353]),
    HelpLabelSegment::Line([-8246, 7353]),
    HelpLabelSegment::Line([-8246, 7980]),
    HelpLabelSegment::Line([-8358, 7980]),
    HelpLabelSegment::Line([-8358, 7893]),
    HelpLabelSegment::Quad {
        control: [-8376, 7931],
        to: [-8433, 7961],
    },
    HelpLabelSegment::Quad {
        control: [-8490, 7992],
        to: [-8544, 7992],
    },
    HelpLabelSegment::Quad {
        control: [-8647, 7992],
        to: [-8701, 7933],
    },
    HelpLabelSegment::Quad {
        control: [-8756, 7873],
        to: [-8756, 7764],
    },
    HelpLabelSegment::Line([-8756, 7353]),
    HelpLabelSegment::Line([-8645, 7353]),
];

const BODY_CONTOUR_298: [HelpLabelSegment; 16] = [
    HelpLabelSegment::Quad {
        control: [-7649, 7382],
        to: [-7622, 7403],
    },
    HelpLabelSegment::Line([-7677, 7482]),
    HelpLabelSegment::Quad {
        control: [-7695, 7466],
        to: [-7737, 7450],
    },
    HelpLabelSegment::Line([-7822, 7435]),
    HelpLabelSegment::Quad {
        control: [-7912, 7435],
        to: [-7966, 7498],
    },
    HelpLabelSegment::Quad {
        control: [-8019, 7562],
        to: [-8019, 7673],
    },
    HelpLabelSegment::Quad {
        control: [-8019, 7783],
        to: [-7965, 7841],
    },
    HelpLabelSegment::Quad {
        control: [-7910, 7898],
        to: [-7814, 7898],
    },
    HelpLabelSegment::Quad {
        control: [-7739, 7898],
        to: [-7663, 7841],
    },
    HelpLabelSegment::Line([-7618, 7934]),
    HelpLabelSegment::Quad {
        control: [-7709, 7992],
        to: [-7841, 7992],
    },
    HelpLabelSegment::Quad {
        control: [-7969, 7992],
        to: [-8053, 7906],
    },
    HelpLabelSegment::Quad {
        control: [-8136, 7819],
        to: [-8136, 7673],
    },
    HelpLabelSegment::Quad {
        control: [-8136, 7523],
        to: [-8050, 7432],
    },
    HelpLabelSegment::Quad {
        control: [-7963, 7341],
        to: [-7812, 7341],
    },
    HelpLabelSegment::Line([-7706, 7361]),
];

const BODY_CONTOUR_299: [HelpLabelSegment; 16] = [
    HelpLabelSegment::Line([-7389, 7422]),
    HelpLabelSegment::Quad {
        control: [-7367, 7387],
        to: [-7318, 7365],
    },
    HelpLabelSegment::Quad {
        control: [-7268, 7341],
        to: [-7216, 7341],
    },
    HelpLabelSegment::Quad {
        control: [-7116, 7341],
        to: [-7059, 7407],
    },
    HelpLabelSegment::Quad {
        control: [-7002, 7473],
        to: [-7002, 7586],
    },
    HelpLabelSegment::Line([-7002, 7980]),
    HelpLabelSegment::Line([-7114, 7980]),
    HelpLabelSegment::Line([-7114, 7586]),
    HelpLabelSegment::Quad {
        control: [-7114, 7516],
        to: [-7149, 7475],
    },
    HelpLabelSegment::Quad {
        control: [-7183, 7435],
        to: [-7246, 7435],
    },
    HelpLabelSegment::Quad {
        control: [-7286, 7435],
        to: [-7327, 7459],
    },
    HelpLabelSegment::Quad {
        control: [-7368, 7482],
        to: [-7389, 7514],
    },
    HelpLabelSegment::Line([-7389, 7980]),
    HelpLabelSegment::Line([-7500, 7980]),
    HelpLabelSegment::Line([-7500, 7095]),
    HelpLabelSegment::Line([-7389, 7095]),
];

const BODY_CONTOUR_300: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [-6702, 7115],
        to: [-6681, 7136],
    },
    HelpLabelSegment::Quad {
        control: [-6661, 7156],
        to: [-6661, 7184],
    },
    HelpLabelSegment::Quad {
        control: [-6661, 7212],
        to: [-6681, 7234],
    },
    HelpLabelSegment::Quad {
        control: [-6702, 7253],
        to: [-6730, 7253],
    },
    HelpLabelSegment::Quad {
        control: [-6759, 7253],
        to: [-6779, 7234],
    },
    HelpLabelSegment::Quad {
        control: [-6800, 7212],
        to: [-6800, 7184],
    },
    HelpLabelSegment::Quad {
        control: [-6800, 7155],
        to: [-6780, 7135],
    },
    HelpLabelSegment::Quad {
        control: [-6760, 7115],
        to: [-6730, 7115],
    },
];

const BODY_CONTOUR_301: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Line([-6680, 7980]),
    HelpLabelSegment::Line([-6791, 7980]),
    HelpLabelSegment::Line([-6791, 7447]),
    HelpLabelSegment::Line([-6878, 7447]),
    HelpLabelSegment::Line([-6878, 7353]),
    HelpLabelSegment::Line([-6680, 7353]),
];

const BODY_CONTOUR_302: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Line([-6008, 7980]),
    HelpLabelSegment::Line([-6120, 7980]),
    HelpLabelSegment::Line([-6120, 7616]),
    HelpLabelSegment::Quad {
        control: [-6120, 7515],
        to: [-6149, 7475],
    },
    HelpLabelSegment::Quad {
        control: [-6180, 7435],
        to: [-6251, 7435],
    },
    HelpLabelSegment::Quad {
        control: [-6289, 7435],
        to: [-6331, 7457],
    },
    HelpLabelSegment::Quad {
        control: [-6372, 7481],
        to: [-6394, 7514],
    },
    HelpLabelSegment::Line([-6394, 7980]),
    HelpLabelSegment::Line([-6505, 7980]),
    HelpLabelSegment::Line([-6505, 7353]),
    HelpLabelSegment::Line([-6429, 7353]),
    HelpLabelSegment::Line([-6394, 7434]),
    HelpLabelSegment::Quad {
        control: [-6339, 7341],
        to: [-6215, 7341],
    },
    HelpLabelSegment::Quad {
        control: [-6008, 7341],
        to: [-6008, 7592],
    },
];

const BODY_CONTOUR_303: [HelpLabelSegment; 32] = [
    HelpLabelSegment::Line([-5439, 7432]),
    HelpLabelSegment::Quad {
        control: [-5396, 7487],
        to: [-5396, 7577],
    },
    HelpLabelSegment::Quad {
        control: [-5396, 7672],
        to: [-5455, 7736],
    },
    HelpLabelSegment::Quad {
        control: [-5514, 7801],
        to: [-5610, 7810],
    },
    HelpLabelSegment::Line([-5703, 7819]),
    HelpLabelSegment::Line([-5746, 7832]),
    HelpLabelSegment::Quad {
        control: [-5774, 7843],
        to: [-5774, 7860],
    },
    HelpLabelSegment::Quad {
        control: [-5774, 7884],
        to: [-5717, 7884],
    },
    HelpLabelSegment::Line([-5638, 7876]),
    HelpLabelSegment::Line([-5559, 7866]),
    HelpLabelSegment::Quad {
        control: [-5466, 7866],
        to: [-5414, 7911],
    },
    HelpLabelSegment::Quad {
        control: [-5363, 7954],
        to: [-5363, 8033],
    },
    HelpLabelSegment::Quad {
        control: [-5363, 8119],
        to: [-5440, 8173],
    },
    HelpLabelSegment::Quad {
        control: [-5518, 8226],
        to: [-5637, 8226],
    },
    HelpLabelSegment::Quad {
        control: [-5698, 8226],
        to: [-5765, 8205],
    },
    HelpLabelSegment::Quad {
        control: [-5833, 8183],
        to: [-5874, 8152],
    },
    HelpLabelSegment::Line([-5813, 8063]),
    HelpLabelSegment::Quad {
        control: [-5716, 8128],
        to: [-5634, 8128],
    },
    HelpLabelSegment::Quad {
        control: [-5559, 8128],
        to: [-5514, 8102],
    },
    HelpLabelSegment::Quad {
        control: [-5471, 8076],
        to: [-5471, 8037],
    },
    HelpLabelSegment::Quad {
        control: [-5471, 7961],
        to: [-5581, 7961],
    },
    HelpLabelSegment::Line([-5649, 7971]),
    HelpLabelSegment::Line([-5726, 7980]),
    HelpLabelSegment::Quad {
        control: [-5860, 7980],
        to: [-5860, 7879],
    },
    HelpLabelSegment::Quad {
        control: [-5860, 7848],
        to: [-5828, 7823],
    },
    HelpLabelSegment::Quad {
        control: [-5796, 7797],
        to: [-5751, 7787],
    },
    HelpLabelSegment::Quad {
        control: [-5883, 7725],
        to: [-5883, 7571],
    },
    HelpLabelSegment::Quad {
        control: [-5883, 7473],
        to: [-5814, 7407],
    },
    HelpLabelSegment::Quad {
        control: [-5746, 7341],
        to: [-5645, 7341],
    },
    HelpLabelSegment::Quad {
        control: [-5553, 7341],
        to: [-5500, 7379],
    },
    HelpLabelSegment::Line([-5445, 7312]),
    HelpLabelSegment::Line([-5372, 7381]),
];

const BODY_CONTOUR_304: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [-5696, 7430],
        to: [-5732, 7471],
    },
    HelpLabelSegment::Quad {
        control: [-5768, 7512],
        to: [-5768, 7571],
    },
    HelpLabelSegment::Quad {
        control: [-5768, 7637],
        to: [-5733, 7680],
    },
    HelpLabelSegment::Quad {
        control: [-5698, 7723],
        to: [-5637, 7723],
    },
    HelpLabelSegment::Quad {
        control: [-5578, 7723],
        to: [-5544, 7681],
    },
    HelpLabelSegment::Quad {
        control: [-5512, 7639],
        to: [-5512, 7571],
    },
    HelpLabelSegment::Quad {
        control: [-5512, 7512],
        to: [-5547, 7471],
    },
    HelpLabelSegment::Quad {
        control: [-5583, 7430],
        to: [-5637, 7430],
    },
];

const BODY_CONTOUR_305: [HelpLabelSegment; 13] = [
    HelpLabelSegment::Line([-4779, 7399]),
    HelpLabelSegment::Quad {
        control: [-4764, 7378],
        to: [-4719, 7359],
    },
    HelpLabelSegment::Quad {
        control: [-4676, 7341],
        to: [-4634, 7341],
    },
    HelpLabelSegment::Quad {
        control: [-4505, 7341],
        to: [-4425, 7430],
    },
    HelpLabelSegment::Quad {
        control: [-4345, 7519],
        to: [-4345, 7655],
    },
    HelpLabelSegment::Quad {
        control: [-4345, 7812],
        to: [-4425, 7903],
    },
    HelpLabelSegment::Quad {
        control: [-4506, 7992],
        to: [-4643, 7992],
    },
    HelpLabelSegment::Quad {
        control: [-4688, 7992],
        to: [-4730, 7975],
    },
    HelpLabelSegment::Quad {
        control: [-4773, 7959],
        to: [-4795, 7935],
    },
    HelpLabelSegment::Line([-4835, 7992]),
    HelpLabelSegment::Line([-4890, 7992]),
    HelpLabelSegment::Line([-4890, 7095]),
    HelpLabelSegment::Line([-4779, 7095]),
];

const BODY_CONTOUR_306: [HelpLabelSegment; 9] = [
    HelpLabelSegment::Line([-4779, 7846]),
    HelpLabelSegment::Quad {
        control: [-4779, 7856],
        to: [-4738, 7877],
    },
    HelpLabelSegment::Quad {
        control: [-4696, 7898],
        to: [-4675, 7898],
    },
    HelpLabelSegment::Quad {
        control: [-4561, 7898],
        to: [-4512, 7844],
    },
    HelpLabelSegment::Quad {
        control: [-4463, 7789],
        to: [-4463, 7661],
    },
    HelpLabelSegment::Quad {
        control: [-4463, 7555],
        to: [-4520, 7495],
    },
    HelpLabelSegment::Quad {
        control: [-4577, 7435],
        to: [-4675, 7435],
    },
    HelpLabelSegment::Quad {
        control: [-4695, 7435],
        to: [-4731, 7453],
    },
    HelpLabelSegment::Line([-4779, 7484]),
];

const BODY_CONTOUR_307: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Quad {
        control: [-3889, 7341],
        to: [-3827, 7403],
    },
    HelpLabelSegment::Quad {
        control: [-3766, 7466],
        to: [-3766, 7600],
    },
    HelpLabelSegment::Line([-3766, 7825]),
    HelpLabelSegment::Quad {
        control: [-3766, 7909],
        to: [-3716, 7935],
    },
    HelpLabelSegment::Line([-3716, 7992]),
    HelpLabelSegment::Quad {
        control: [-3784, 7992],
        to: [-3817, 7972],
    },
    HelpLabelSegment::Quad {
        control: [-3851, 7953],
        to: [-3866, 7909],
    },
    HelpLabelSegment::Quad {
        control: [-3933, 7992],
        to: [-4070, 7992],
    },
    HelpLabelSegment::Quad {
        control: [-4144, 7992],
        to: [-4198, 7939],
    },
    HelpLabelSegment::Quad {
        control: [-4253, 7885],
        to: [-4253, 7805],
    },
    HelpLabelSegment::Quad {
        control: [-4253, 7709],
        to: [-4169, 7644],
    },
    HelpLabelSegment::Quad {
        control: [-4086, 7578],
        to: [-3957, 7578],
    },
    HelpLabelSegment::Line([-3877, 7593]),
    HelpLabelSegment::Quad {
        control: [-3877, 7441],
        to: [-4013, 7441],
    },
    HelpLabelSegment::Quad {
        control: [-4117, 7441],
        to: [-4173, 7497],
    },
    HelpLabelSegment::Line([-4220, 7403]),
    HelpLabelSegment::Quad {
        control: [-4189, 7378],
        to: [-4132, 7360],
    },
    HelpLabelSegment::Quad {
        control: [-4076, 7341],
        to: [-4026, 7341],
    },
];

const BODY_CONTOUR_308: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [-4035, 7660],
        to: [-4088, 7703],
    },
    HelpLabelSegment::Quad {
        control: [-4142, 7747],
        to: [-4142, 7807],
    },
    HelpLabelSegment::Quad {
        control: [-4142, 7904],
        to: [-4026, 7904],
    },
    HelpLabelSegment::Quad {
        control: [-3941, 7904],
        to: [-3877, 7824],
    },
    HelpLabelSegment::Line([-3877, 7672]),
    HelpLabelSegment::Line([-3951, 7660]),
];

const BODY_CONTOUR_309: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([-3365, 7992]),
    HelpLabelSegment::Quad {
        control: [-3582, 7992],
        to: [-3582, 7803],
    },
    HelpLabelSegment::Line([-3582, 7095]),
    HelpLabelSegment::Line([-3471, 7095]),
    HelpLabelSegment::Line([-3471, 7784]),
    HelpLabelSegment::Quad {
        control: [-3471, 7835],
        to: [-3441, 7864],
    },
    HelpLabelSegment::Quad {
        control: [-3412, 7892],
        to: [-3365, 7892],
    },
];

const BODY_CONTOUR_310: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([-3010, 7992]),
    HelpLabelSegment::Quad {
        control: [-3227, 7992],
        to: [-3227, 7803],
    },
    HelpLabelSegment::Line([-3227, 7095]),
    HelpLabelSegment::Line([-3116, 7095]),
    HelpLabelSegment::Line([-3116, 7784]),
    HelpLabelSegment::Quad {
        control: [-3116, 7835],
        to: [-3086, 7864],
    },
    HelpLabelSegment::Quad {
        control: [-3057, 7892],
        to: [-3010, 7892],
    },
];

const BODY_CONTOUR_311: [HelpLabelSegment; 22] = [
    HelpLabelSegment::Line([-2573, 7488]),
    HelpLabelSegment::Quad {
        control: [-2639, 7435],
        to: [-2706, 7435],
    },
    HelpLabelSegment::Quad {
        control: [-2746, 7435],
        to: [-2773, 7454],
    },
    HelpLabelSegment::Quad {
        control: [-2801, 7473],
        to: [-2801, 7501],
    },
    HelpLabelSegment::Quad {
        control: [-2801, 7562],
        to: [-2731, 7592],
    },
    HelpLabelSegment::Line([-2652, 7628]),
    HelpLabelSegment::Quad {
        control: [-2579, 7662],
        to: [-2545, 7705],
    },
    HelpLabelSegment::Quad {
        control: [-2512, 7748],
        to: [-2512, 7812],
    },
    HelpLabelSegment::Quad {
        control: [-2512, 7897],
        to: [-2571, 7945],
    },
    HelpLabelSegment::Quad {
        control: [-2631, 7992],
        to: [-2735, 7992],
    },
    HelpLabelSegment::Quad {
        control: [-2835, 7992],
        to: [-2921, 7942],
    },
    HelpLabelSegment::Line([-2883, 7837]),
    HelpLabelSegment::Quad {
        control: [-2789, 7898],
        to: [-2733, 7898],
    },
    HelpLabelSegment::Quad {
        control: [-2630, 7898],
        to: [-2630, 7811],
    },
    HelpLabelSegment::Quad {
        control: [-2630, 7749],
        to: [-2729, 7705],
    },
    HelpLabelSegment::Quad {
        control: [-2805, 7669],
        to: [-2832, 7652],
    },
    HelpLabelSegment::Quad {
        control: [-2859, 7633],
        to: [-2878, 7611],
    },
    HelpLabelSegment::Quad {
        control: [-2898, 7587],
        to: [-2907, 7562],
    },
    HelpLabelSegment::Quad {
        control: [-2918, 7535],
        to: [-2918, 7505],
    },
    HelpLabelSegment::Quad {
        control: [-2918, 7428],
        to: [-2862, 7385],
    },
    HelpLabelSegment::Quad {
        control: [-2805, 7341],
        to: [-2714, 7341],
    },
    HelpLabelSegment::Quad {
        control: [-2646, 7341],
        to: [-2542, 7385],
    },
];

const BODY_CONTOUR_312: [HelpLabelSegment; 24] = [
    HelpLabelSegment::Quad {
        control: [-1197, 7456],
        to: [-1197, 7560],
    },
    HelpLabelSegment::Line([-1197, 7980]),
    HelpLabelSegment::Line([-1309, 7980]),
    HelpLabelSegment::Line([-1309, 7583]),
    HelpLabelSegment::Quad {
        control: [-1309, 7435],
        to: [-1438, 7435],
    },
    HelpLabelSegment::Quad {
        control: [-1478, 7435],
        to: [-1513, 7460],
    },
    HelpLabelSegment::Quad {
        control: [-1548, 7484],
        to: [-1561, 7516],
    },
    HelpLabelSegment::Line([-1561, 7980]),
    HelpLabelSegment::Line([-1672, 7980]),
    HelpLabelSegment::Line([-1672, 7535]),
    HelpLabelSegment::Quad {
        control: [-1672, 7488],
        to: [-1707, 7462],
    },
    HelpLabelSegment::Quad {
        control: [-1742, 7435],
        to: [-1800, 7435],
    },
    HelpLabelSegment::Quad {
        control: [-1833, 7435],
        to: [-1870, 7461],
    },
    HelpLabelSegment::Quad {
        control: [-1909, 7487],
        to: [-1924, 7517],
    },
    HelpLabelSegment::Line([-1924, 7980]),
    HelpLabelSegment::Line([-2035, 7980]),
    HelpLabelSegment::Line([-2035, 7353]),
    HelpLabelSegment::Line([-1963, 7353]),
    HelpLabelSegment::Line([-1926, 7426]),
    HelpLabelSegment::Quad {
        control: [-1862, 7341],
        to: [-1765, 7341],
    },
    HelpLabelSegment::Quad {
        control: [-1630, 7341],
        to: [-1576, 7425],
    },
    HelpLabelSegment::Quad {
        control: [-1557, 7389],
        to: [-1507, 7365],
    },
    HelpLabelSegment::Quad {
        control: [-1455, 7341],
        to: [-1401, 7341],
    },
    HelpLabelSegment::Quad {
        control: [-1304, 7341],
        to: [-1251, 7399],
    },
];

const BODY_CONTOUR_313: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [-856, 7435],
        to: [-907, 7483],
    },
    HelpLabelSegment::Quad {
        control: [-955, 7529],
        to: [-962, 7597],
    },
    HelpLabelSegment::Line([-614, 7597]),
    HelpLabelSegment::Quad {
        control: [-614, 7529],
        to: [-656, 7484],
    },
    HelpLabelSegment::Quad {
        control: [-703, 7435],
        to: [-783, 7435],
    },
];

const BODY_CONTOUR_314: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [-850, 7898],
        to: [-767, 7898],
    },
    HelpLabelSegment::Quad {
        control: [-671, 7898],
        to: [-608, 7843],
    },
    HelpLabelSegment::Line([-561, 7923]),
    HelpLabelSegment::Quad {
        control: [-587, 7948],
        to: [-640, 7967],
    },
    HelpLabelSegment::Quad {
        control: [-706, 7992],
        to: [-788, 7992],
    },
    HelpLabelSegment::Quad {
        control: [-907, 7992],
        to: [-990, 7912],
    },
    HelpLabelSegment::Quad {
        control: [-1081, 7823],
        to: [-1081, 7674],
    },
    HelpLabelSegment::Quad {
        control: [-1081, 7518],
        to: [-988, 7425],
    },
    HelpLabelSegment::Quad {
        control: [-903, 7341],
        to: [-787, 7341],
    },
    HelpLabelSegment::Quad {
        control: [-654, 7341],
        to: [-577, 7416],
    },
    HelpLabelSegment::Quad {
        control: [-504, 7489],
        to: [-504, 7610],
    },
    HelpLabelSegment::Quad {
        control: [-504, 7646],
        to: [-512, 7678],
    },
    HelpLabelSegment::Line([-964, 7678]),
    HelpLabelSegment::Quad {
        control: [-964, 7788],
        to: [-904, 7846],
    },
];

const BODY_CONTOUR_315: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Quad {
        control: [-81, 7435],
        to: [-118, 7435],
    },
    HelpLabelSegment::Quad {
        control: [-177, 7435],
        to: [-221, 7489],
    },
    HelpLabelSegment::Quad {
        control: [-266, 7544],
        to: [-266, 7620],
    },
    HelpLabelSegment::Line([-266, 7980]),
    HelpLabelSegment::Line([-377, 7980]),
    HelpLabelSegment::Line([-377, 7353]),
    HelpLabelSegment::Line([-266, 7353]),
    HelpLabelSegment::Line([-266, 7453]),
    HelpLabelSegment::Quad {
        control: [-205, 7341],
        to: [-84, 7341],
    },
    HelpLabelSegment::Line([1, 7352]),
    HelpLabelSegment::Line([-44, 7460]),
];

const BODY_CONTOUR_316: [HelpLabelSegment; 32] = [
    HelpLabelSegment::Line([491, 7432]),
    HelpLabelSegment::Quad {
        control: [534, 7487],
        to: [534, 7577],
    },
    HelpLabelSegment::Quad {
        control: [534, 7672],
        to: [475, 7736],
    },
    HelpLabelSegment::Quad {
        control: [416, 7801],
        to: [320, 7810],
    },
    HelpLabelSegment::Line([227, 7819]),
    HelpLabelSegment::Line([184, 7832]),
    HelpLabelSegment::Quad {
        control: [156, 7843],
        to: [156, 7860],
    },
    HelpLabelSegment::Quad {
        control: [156, 7884],
        to: [213, 7884],
    },
    HelpLabelSegment::Line([292, 7876]),
    HelpLabelSegment::Line([371, 7866]),
    HelpLabelSegment::Quad {
        control: [464, 7866],
        to: [516, 7911],
    },
    HelpLabelSegment::Quad {
        control: [567, 7954],
        to: [567, 8033],
    },
    HelpLabelSegment::Quad {
        control: [567, 8119],
        to: [490, 8173],
    },
    HelpLabelSegment::Quad {
        control: [412, 8226],
        to: [293, 8226],
    },
    HelpLabelSegment::Quad {
        control: [232, 8226],
        to: [165, 8205],
    },
    HelpLabelSegment::Quad {
        control: [97, 8183],
        to: [56, 8152],
    },
    HelpLabelSegment::Line([117, 8063]),
    HelpLabelSegment::Quad {
        control: [214, 8128],
        to: [296, 8128],
    },
    HelpLabelSegment::Quad {
        control: [371, 8128],
        to: [416, 8102],
    },
    HelpLabelSegment::Quad {
        control: [459, 8076],
        to: [459, 8037],
    },
    HelpLabelSegment::Quad {
        control: [459, 7961],
        to: [349, 7961],
    },
    HelpLabelSegment::Line([281, 7971]),
    HelpLabelSegment::Line([204, 7980]),
    HelpLabelSegment::Quad {
        control: [70, 7980],
        to: [70, 7879],
    },
    HelpLabelSegment::Quad {
        control: [70, 7848],
        to: [102, 7823],
    },
    HelpLabelSegment::Quad {
        control: [134, 7797],
        to: [179, 7787],
    },
    HelpLabelSegment::Quad {
        control: [47, 7725],
        to: [47, 7571],
    },
    HelpLabelSegment::Quad {
        control: [47, 7473],
        to: [116, 7407],
    },
    HelpLabelSegment::Quad {
        control: [184, 7341],
        to: [285, 7341],
    },
    HelpLabelSegment::Quad {
        control: [377, 7341],
        to: [430, 7379],
    },
    HelpLabelSegment::Line([485, 7312]),
    HelpLabelSegment::Line([558, 7381]),
];

const BODY_CONTOUR_317: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [234, 7430],
        to: [198, 7471],
    },
    HelpLabelSegment::Quad {
        control: [162, 7512],
        to: [162, 7571],
    },
    HelpLabelSegment::Quad {
        control: [162, 7637],
        to: [197, 7680],
    },
    HelpLabelSegment::Quad {
        control: [232, 7723],
        to: [293, 7723],
    },
    HelpLabelSegment::Quad {
        control: [352, 7723],
        to: [386, 7681],
    },
    HelpLabelSegment::Quad {
        control: [418, 7639],
        to: [418, 7571],
    },
    HelpLabelSegment::Quad {
        control: [418, 7512],
        to: [383, 7471],
    },
    HelpLabelSegment::Quad {
        control: [347, 7430],
        to: [293, 7430],
    },
];

const BODY_CONTOUR_318: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [864, 7435],
        to: [813, 7483],
    },
    HelpLabelSegment::Quad {
        control: [765, 7529],
        to: [758, 7597],
    },
    HelpLabelSegment::Line([1106, 7597]),
    HelpLabelSegment::Quad {
        control: [1106, 7529],
        to: [1064, 7484],
    },
    HelpLabelSegment::Quad {
        control: [1017, 7435],
        to: [937, 7435],
    },
];

const BODY_CONTOUR_319: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [870, 7898],
        to: [953, 7898],
    },
    HelpLabelSegment::Quad {
        control: [1049, 7898],
        to: [1112, 7843],
    },
    HelpLabelSegment::Line([1159, 7923]),
    HelpLabelSegment::Quad {
        control: [1133, 7948],
        to: [1080, 7967],
    },
    HelpLabelSegment::Quad {
        control: [1014, 7992],
        to: [932, 7992],
    },
    HelpLabelSegment::Quad {
        control: [813, 7992],
        to: [730, 7912],
    },
    HelpLabelSegment::Quad {
        control: [639, 7823],
        to: [639, 7674],
    },
    HelpLabelSegment::Quad {
        control: [639, 7518],
        to: [732, 7425],
    },
    HelpLabelSegment::Quad {
        control: [817, 7341],
        to: [933, 7341],
    },
    HelpLabelSegment::Quad {
        control: [1066, 7341],
        to: [1143, 7416],
    },
    HelpLabelSegment::Quad {
        control: [1216, 7489],
        to: [1216, 7610],
    },
    HelpLabelSegment::Quad {
        control: [1216, 7646],
        to: [1208, 7678],
    },
    HelpLabelSegment::Line([756, 7678]),
    HelpLabelSegment::Quad {
        control: [756, 7788],
        to: [816, 7846],
    },
];

const BODY_CONTOUR_320: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [1838, 7115],
        to: [1859, 7136],
    },
    HelpLabelSegment::Quad {
        control: [1879, 7156],
        to: [1879, 7184],
    },
    HelpLabelSegment::Quad {
        control: [1879, 7212],
        to: [1859, 7234],
    },
    HelpLabelSegment::Quad {
        control: [1838, 7253],
        to: [1810, 7253],
    },
    HelpLabelSegment::Quad {
        control: [1781, 7253],
        to: [1761, 7234],
    },
    HelpLabelSegment::Quad {
        control: [1740, 7212],
        to: [1740, 7184],
    },
    HelpLabelSegment::Quad {
        control: [1740, 7155],
        to: [1760, 7135],
    },
    HelpLabelSegment::Quad {
        control: [1780, 7115],
        to: [1810, 7115],
    },
];

const BODY_CONTOUR_321: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Line([1860, 7980]),
    HelpLabelSegment::Line([1749, 7980]),
    HelpLabelSegment::Line([1749, 7447]),
    HelpLabelSegment::Line([1662, 7447]),
    HelpLabelSegment::Line([1662, 7353]),
    HelpLabelSegment::Line([1860, 7353]),
];

const BODY_CONTOUR_322: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Line([2532, 7980]),
    HelpLabelSegment::Line([2420, 7980]),
    HelpLabelSegment::Line([2420, 7616]),
    HelpLabelSegment::Quad {
        control: [2420, 7515],
        to: [2391, 7475],
    },
    HelpLabelSegment::Quad {
        control: [2360, 7435],
        to: [2289, 7435],
    },
    HelpLabelSegment::Quad {
        control: [2251, 7435],
        to: [2209, 7457],
    },
    HelpLabelSegment::Quad {
        control: [2168, 7481],
        to: [2146, 7514],
    },
    HelpLabelSegment::Line([2146, 7980]),
    HelpLabelSegment::Line([2035, 7980]),
    HelpLabelSegment::Line([2035, 7353]),
    HelpLabelSegment::Line([2111, 7353]),
    HelpLabelSegment::Line([2146, 7434]),
    HelpLabelSegment::Quad {
        control: [2201, 7341],
        to: [2325, 7341],
    },
    HelpLabelSegment::Quad {
        control: [2532, 7341],
        to: [2532, 7592],
    },
];

const BODY_CONTOUR_323: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Line([2730, 7222]),
    HelpLabelSegment::Line([2841, 7178]),
    HelpLabelSegment::Line([2841, 7353]),
    HelpLabelSegment::Line([3013, 7353]),
    HelpLabelSegment::Line([3013, 7441]),
    HelpLabelSegment::Line([2841, 7441]),
    HelpLabelSegment::Line([2841, 7753]),
    HelpLabelSegment::Quad {
        control: [2841, 7831],
        to: [2868, 7865],
    },
    HelpLabelSegment::Quad {
        control: [2894, 7898],
        to: [2953, 7898],
    },
    HelpLabelSegment::Quad {
        control: [2996, 7898],
        to: [3041, 7877],
    },
    HelpLabelSegment::Line([3058, 7974]),
    HelpLabelSegment::Line([2906, 7992]),
    HelpLabelSegment::Quad {
        control: [2831, 7992],
        to: [2781, 7937],
    },
    HelpLabelSegment::Quad {
        control: [2730, 7882],
        to: [2730, 7797],
    },
    HelpLabelSegment::Line([2730, 7441]),
    HelpLabelSegment::Line([2657, 7441]),
    HelpLabelSegment::Line([2657, 7353]),
    HelpLabelSegment::Line([2730, 7353]),
];

const BODY_CONTOUR_324: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [3241, 7901],
        to: [3407, 7901],
    },
    HelpLabelSegment::Quad {
        control: [3486, 7901],
        to: [3530, 7838],
    },
    HelpLabelSegment::Quad {
        control: [3574, 7775],
        to: [3574, 7665],
    },
    HelpLabelSegment::Quad {
        control: [3574, 7432],
        to: [3407, 7432],
    },
    HelpLabelSegment::Quad {
        control: [3331, 7432],
        to: [3287, 7494],
    },
    HelpLabelSegment::Quad {
        control: [3241, 7556],
        to: [3241, 7665],
    },
];

const BODY_CONTOUR_325: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [3280, 7341],
        to: [3407, 7341],
    },
    HelpLabelSegment::Quad {
        control: [3542, 7341],
        to: [3617, 7427],
    },
    HelpLabelSegment::Quad {
        control: [3691, 7512],
        to: [3691, 7665],
    },
    HelpLabelSegment::Quad {
        control: [3691, 7817],
        to: [3615, 7905],
    },
    HelpLabelSegment::Quad {
        control: [3539, 7992],
        to: [3407, 7992],
    },
    HelpLabelSegment::Quad {
        control: [3274, 7992],
        to: [3199, 7904],
    },
    HelpLabelSegment::Quad {
        control: [3124, 7815],
        to: [3124, 7665],
    },
    HelpLabelSegment::Quad {
        control: [3124, 7519],
        to: [3202, 7430],
    },
];

const BODY_CONTOUR_326: [HelpLabelSegment; 13] = [
    HelpLabelSegment::Line([4281, 7399]),
    HelpLabelSegment::Quad {
        control: [4296, 7378],
        to: [4341, 7359],
    },
    HelpLabelSegment::Quad {
        control: [4384, 7341],
        to: [4426, 7341],
    },
    HelpLabelSegment::Quad {
        control: [4555, 7341],
        to: [4635, 7430],
    },
    HelpLabelSegment::Quad {
        control: [4715, 7519],
        to: [4715, 7655],
    },
    HelpLabelSegment::Quad {
        control: [4715, 7812],
        to: [4635, 7903],
    },
    HelpLabelSegment::Quad {
        control: [4554, 7992],
        to: [4417, 7992],
    },
    HelpLabelSegment::Quad {
        control: [4372, 7992],
        to: [4330, 7975],
    },
    HelpLabelSegment::Quad {
        control: [4287, 7959],
        to: [4265, 7935],
    },
    HelpLabelSegment::Line([4225, 7992]),
    HelpLabelSegment::Line([4170, 7992]),
    HelpLabelSegment::Line([4170, 7095]),
    HelpLabelSegment::Line([4281, 7095]),
];

const BODY_CONTOUR_327: [HelpLabelSegment; 9] = [
    HelpLabelSegment::Line([4281, 7846]),
    HelpLabelSegment::Quad {
        control: [4281, 7856],
        to: [4322, 7877],
    },
    HelpLabelSegment::Quad {
        control: [4364, 7898],
        to: [4385, 7898],
    },
    HelpLabelSegment::Quad {
        control: [4499, 7898],
        to: [4548, 7844],
    },
    HelpLabelSegment::Quad {
        control: [4597, 7789],
        to: [4597, 7661],
    },
    HelpLabelSegment::Quad {
        control: [4597, 7555],
        to: [4540, 7495],
    },
    HelpLabelSegment::Quad {
        control: [4483, 7435],
        to: [4385, 7435],
    },
    HelpLabelSegment::Quad {
        control: [4365, 7435],
        to: [4329, 7453],
    },
    HelpLabelSegment::Line([4281, 7484]),
];

const BODY_CONTOUR_328: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [4983, 7115],
        to: [5004, 7136],
    },
    HelpLabelSegment::Quad {
        control: [5024, 7156],
        to: [5024, 7184],
    },
    HelpLabelSegment::Quad {
        control: [5024, 7212],
        to: [5004, 7234],
    },
    HelpLabelSegment::Quad {
        control: [4983, 7253],
        to: [4955, 7253],
    },
    HelpLabelSegment::Quad {
        control: [4926, 7253],
        to: [4906, 7234],
    },
    HelpLabelSegment::Quad {
        control: [4885, 7212],
        to: [4885, 7184],
    },
    HelpLabelSegment::Quad {
        control: [4885, 7155],
        to: [4905, 7135],
    },
    HelpLabelSegment::Quad {
        control: [4925, 7115],
        to: [4955, 7115],
    },
];

const BODY_CONTOUR_329: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Line([5005, 7980]),
    HelpLabelSegment::Line([4894, 7980]),
    HelpLabelSegment::Line([4894, 7447]),
    HelpLabelSegment::Line([4807, 7447]),
    HelpLabelSegment::Line([4807, 7353]),
    HelpLabelSegment::Line([5005, 7353]),
];

const BODY_CONTOUR_330: [HelpLabelSegment; 32] = [
    HelpLabelSegment::Line([5591, 7432]),
    HelpLabelSegment::Quad {
        control: [5634, 7487],
        to: [5634, 7577],
    },
    HelpLabelSegment::Quad {
        control: [5634, 7672],
        to: [5575, 7736],
    },
    HelpLabelSegment::Quad {
        control: [5516, 7801],
        to: [5420, 7810],
    },
    HelpLabelSegment::Line([5327, 7819]),
    HelpLabelSegment::Line([5284, 7832]),
    HelpLabelSegment::Quad {
        control: [5256, 7843],
        to: [5256, 7860],
    },
    HelpLabelSegment::Quad {
        control: [5256, 7884],
        to: [5313, 7884],
    },
    HelpLabelSegment::Line([5392, 7876]),
    HelpLabelSegment::Line([5471, 7866]),
    HelpLabelSegment::Quad {
        control: [5564, 7866],
        to: [5616, 7911],
    },
    HelpLabelSegment::Quad {
        control: [5667, 7954],
        to: [5667, 8033],
    },
    HelpLabelSegment::Quad {
        control: [5667, 8119],
        to: [5590, 8173],
    },
    HelpLabelSegment::Quad {
        control: [5512, 8226],
        to: [5393, 8226],
    },
    HelpLabelSegment::Quad {
        control: [5332, 8226],
        to: [5265, 8205],
    },
    HelpLabelSegment::Quad {
        control: [5197, 8183],
        to: [5156, 8152],
    },
    HelpLabelSegment::Line([5217, 8063]),
    HelpLabelSegment::Quad {
        control: [5314, 8128],
        to: [5396, 8128],
    },
    HelpLabelSegment::Quad {
        control: [5471, 8128],
        to: [5516, 8102],
    },
    HelpLabelSegment::Quad {
        control: [5559, 8076],
        to: [5559, 8037],
    },
    HelpLabelSegment::Quad {
        control: [5559, 7961],
        to: [5449, 7961],
    },
    HelpLabelSegment::Line([5381, 7971]),
    HelpLabelSegment::Line([5304, 7980]),
    HelpLabelSegment::Quad {
        control: [5170, 7980],
        to: [5170, 7879],
    },
    HelpLabelSegment::Quad {
        control: [5170, 7848],
        to: [5202, 7823],
    },
    HelpLabelSegment::Quad {
        control: [5234, 7797],
        to: [5279, 7787],
    },
    HelpLabelSegment::Quad {
        control: [5147, 7725],
        to: [5147, 7571],
    },
    HelpLabelSegment::Quad {
        control: [5147, 7473],
        to: [5216, 7407],
    },
    HelpLabelSegment::Quad {
        control: [5284, 7341],
        to: [5385, 7341],
    },
    HelpLabelSegment::Quad {
        control: [5477, 7341],
        to: [5530, 7379],
    },
    HelpLabelSegment::Line([5585, 7312]),
    HelpLabelSegment::Line([5658, 7381]),
];

const BODY_CONTOUR_331: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [5334, 7430],
        to: [5298, 7471],
    },
    HelpLabelSegment::Quad {
        control: [5262, 7512],
        to: [5262, 7571],
    },
    HelpLabelSegment::Quad {
        control: [5262, 7637],
        to: [5297, 7680],
    },
    HelpLabelSegment::Quad {
        control: [5332, 7723],
        to: [5393, 7723],
    },
    HelpLabelSegment::Quad {
        control: [5452, 7723],
        to: [5486, 7681],
    },
    HelpLabelSegment::Quad {
        control: [5518, 7639],
        to: [5518, 7571],
    },
    HelpLabelSegment::Quad {
        control: [5518, 7512],
        to: [5483, 7471],
    },
    HelpLabelSegment::Quad {
        control: [5447, 7430],
        to: [5393, 7430],
    },
];

const BODY_CONTOUR_332: [HelpLabelSegment; 32] = [
    HelpLabelSegment::Line([6191, 7432]),
    HelpLabelSegment::Quad {
        control: [6234, 7487],
        to: [6234, 7577],
    },
    HelpLabelSegment::Quad {
        control: [6234, 7672],
        to: [6175, 7736],
    },
    HelpLabelSegment::Quad {
        control: [6116, 7801],
        to: [6020, 7810],
    },
    HelpLabelSegment::Line([5927, 7819]),
    HelpLabelSegment::Line([5884, 7832]),
    HelpLabelSegment::Quad {
        control: [5856, 7843],
        to: [5856, 7860],
    },
    HelpLabelSegment::Quad {
        control: [5856, 7884],
        to: [5913, 7884],
    },
    HelpLabelSegment::Line([5992, 7876]),
    HelpLabelSegment::Line([6071, 7866]),
    HelpLabelSegment::Quad {
        control: [6164, 7866],
        to: [6216, 7911],
    },
    HelpLabelSegment::Quad {
        control: [6267, 7954],
        to: [6267, 8033],
    },
    HelpLabelSegment::Quad {
        control: [6267, 8119],
        to: [6190, 8173],
    },
    HelpLabelSegment::Quad {
        control: [6112, 8226],
        to: [5993, 8226],
    },
    HelpLabelSegment::Quad {
        control: [5932, 8226],
        to: [5865, 8205],
    },
    HelpLabelSegment::Quad {
        control: [5797, 8183],
        to: [5756, 8152],
    },
    HelpLabelSegment::Line([5817, 8063]),
    HelpLabelSegment::Quad {
        control: [5914, 8128],
        to: [5996, 8128],
    },
    HelpLabelSegment::Quad {
        control: [6071, 8128],
        to: [6116, 8102],
    },
    HelpLabelSegment::Quad {
        control: [6159, 8076],
        to: [6159, 8037],
    },
    HelpLabelSegment::Quad {
        control: [6159, 7961],
        to: [6049, 7961],
    },
    HelpLabelSegment::Line([5981, 7971]),
    HelpLabelSegment::Line([5904, 7980]),
    HelpLabelSegment::Quad {
        control: [5770, 7980],
        to: [5770, 7879],
    },
    HelpLabelSegment::Quad {
        control: [5770, 7848],
        to: [5802, 7823],
    },
    HelpLabelSegment::Quad {
        control: [5834, 7797],
        to: [5879, 7787],
    },
    HelpLabelSegment::Quad {
        control: [5747, 7725],
        to: [5747, 7571],
    },
    HelpLabelSegment::Quad {
        control: [5747, 7473],
        to: [5816, 7407],
    },
    HelpLabelSegment::Quad {
        control: [5884, 7341],
        to: [5985, 7341],
    },
    HelpLabelSegment::Quad {
        control: [6077, 7341],
        to: [6130, 7379],
    },
    HelpLabelSegment::Line([6185, 7312]),
    HelpLabelSegment::Line([6258, 7381]),
];

const BODY_CONTOUR_333: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [5934, 7430],
        to: [5898, 7471],
    },
    HelpLabelSegment::Quad {
        control: [5862, 7512],
        to: [5862, 7571],
    },
    HelpLabelSegment::Quad {
        control: [5862, 7637],
        to: [5897, 7680],
    },
    HelpLabelSegment::Quad {
        control: [5932, 7723],
        to: [5993, 7723],
    },
    HelpLabelSegment::Quad {
        control: [6052, 7723],
        to: [6086, 7681],
    },
    HelpLabelSegment::Quad {
        control: [6118, 7639],
        to: [6118, 7571],
    },
    HelpLabelSegment::Quad {
        control: [6118, 7512],
        to: [6083, 7471],
    },
    HelpLabelSegment::Quad {
        control: [6047, 7430],
        to: [5993, 7430],
    },
];

const BODY_CONTOUR_334: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [6564, 7435],
        to: [6513, 7483],
    },
    HelpLabelSegment::Quad {
        control: [6465, 7529],
        to: [6458, 7597],
    },
    HelpLabelSegment::Line([6806, 7597]),
    HelpLabelSegment::Quad {
        control: [6806, 7529],
        to: [6764, 7484],
    },
    HelpLabelSegment::Quad {
        control: [6717, 7435],
        to: [6637, 7435],
    },
];

const BODY_CONTOUR_335: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [6570, 7898],
        to: [6653, 7898],
    },
    HelpLabelSegment::Quad {
        control: [6749, 7898],
        to: [6812, 7843],
    },
    HelpLabelSegment::Line([6859, 7923]),
    HelpLabelSegment::Quad {
        control: [6833, 7948],
        to: [6780, 7967],
    },
    HelpLabelSegment::Quad {
        control: [6714, 7992],
        to: [6632, 7992],
    },
    HelpLabelSegment::Quad {
        control: [6513, 7992],
        to: [6430, 7912],
    },
    HelpLabelSegment::Quad {
        control: [6339, 7823],
        to: [6339, 7674],
    },
    HelpLabelSegment::Quad {
        control: [6339, 7518],
        to: [6432, 7425],
    },
    HelpLabelSegment::Quad {
        control: [6517, 7341],
        to: [6633, 7341],
    },
    HelpLabelSegment::Quad {
        control: [6766, 7341],
        to: [6843, 7416],
    },
    HelpLabelSegment::Quad {
        control: [6916, 7489],
        to: [6916, 7610],
    },
    HelpLabelSegment::Quad {
        control: [6916, 7646],
        to: [6908, 7678],
    },
    HelpLabelSegment::Line([6456, 7678]),
    HelpLabelSegment::Quad {
        control: [6456, 7788],
        to: [6516, 7846],
    },
];

const BODY_CONTOUR_336: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Quad {
        control: [7339, 7435],
        to: [7302, 7435],
    },
    HelpLabelSegment::Quad {
        control: [7243, 7435],
        to: [7199, 7489],
    },
    HelpLabelSegment::Quad {
        control: [7154, 7544],
        to: [7154, 7620],
    },
    HelpLabelSegment::Line([7154, 7980]),
    HelpLabelSegment::Line([7043, 7980]),
    HelpLabelSegment::Line([7043, 7353]),
    HelpLabelSegment::Line([7154, 7353]),
    HelpLabelSegment::Line([7154, 7453]),
    HelpLabelSegment::Quad {
        control: [7215, 7341],
        to: [7336, 7341],
    },
    HelpLabelSegment::Line([7421, 7352]),
    HelpLabelSegment::Line([7376, 7460]),
];

const BODY_CONTOUR_337: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [7936, 7901],
        to: [8102, 7901],
    },
    HelpLabelSegment::Quad {
        control: [8181, 7901],
        to: [8225, 7838],
    },
    HelpLabelSegment::Quad {
        control: [8269, 7775],
        to: [8269, 7665],
    },
    HelpLabelSegment::Quad {
        control: [8269, 7432],
        to: [8102, 7432],
    },
    HelpLabelSegment::Quad {
        control: [8026, 7432],
        to: [7982, 7494],
    },
    HelpLabelSegment::Quad {
        control: [7936, 7556],
        to: [7936, 7665],
    },
];

const BODY_CONTOUR_338: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [7975, 7341],
        to: [8102, 7341],
    },
    HelpLabelSegment::Quad {
        control: [8237, 7341],
        to: [8312, 7427],
    },
    HelpLabelSegment::Quad {
        control: [8386, 7512],
        to: [8386, 7665],
    },
    HelpLabelSegment::Quad {
        control: [8386, 7817],
        to: [8310, 7905],
    },
    HelpLabelSegment::Quad {
        control: [8234, 7992],
        to: [8102, 7992],
    },
    HelpLabelSegment::Quad {
        control: [7969, 7992],
        to: [7894, 7904],
    },
    HelpLabelSegment::Quad {
        control: [7819, 7815],
        to: [7819, 7665],
    },
    HelpLabelSegment::Quad {
        control: [7819, 7519],
        to: [7897, 7430],
    },
];

const BODY_CONTOUR_339: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Line([9002, 7980]),
    HelpLabelSegment::Line([8890, 7980]),
    HelpLabelSegment::Line([8890, 7616]),
    HelpLabelSegment::Quad {
        control: [8890, 7515],
        to: [8861, 7475],
    },
    HelpLabelSegment::Quad {
        control: [8830, 7435],
        to: [8759, 7435],
    },
    HelpLabelSegment::Quad {
        control: [8721, 7435],
        to: [8679, 7457],
    },
    HelpLabelSegment::Quad {
        control: [8638, 7481],
        to: [8616, 7514],
    },
    HelpLabelSegment::Line([8616, 7980]),
    HelpLabelSegment::Line([8505, 7980]),
    HelpLabelSegment::Line([8505, 7353]),
    HelpLabelSegment::Line([8581, 7353]),
    HelpLabelSegment::Line([8616, 7434]),
    HelpLabelSegment::Quad {
        control: [8671, 7341],
        to: [8795, 7341],
    },
    HelpLabelSegment::Quad {
        control: [9002, 7341],
        to: [9002, 7592],
    },
];

const BODY_CONTOUR_340: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [9344, 7435],
        to: [9293, 7483],
    },
    HelpLabelSegment::Quad {
        control: [9245, 7529],
        to: [9238, 7597],
    },
    HelpLabelSegment::Line([9586, 7597]),
    HelpLabelSegment::Quad {
        control: [9586, 7529],
        to: [9544, 7484],
    },
    HelpLabelSegment::Quad {
        control: [9497, 7435],
        to: [9418, 7435],
    },
];

const BODY_CONTOUR_341: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [9350, 7898],
        to: [9433, 7898],
    },
    HelpLabelSegment::Quad {
        control: [9529, 7898],
        to: [9592, 7843],
    },
    HelpLabelSegment::Line([9639, 7923]),
    HelpLabelSegment::Quad {
        control: [9613, 7948],
        to: [9560, 7967],
    },
    HelpLabelSegment::Quad {
        control: [9494, 7992],
        to: [9412, 7992],
    },
    HelpLabelSegment::Quad {
        control: [9293, 7992],
        to: [9210, 7912],
    },
    HelpLabelSegment::Quad {
        control: [9119, 7823],
        to: [9119, 7674],
    },
    HelpLabelSegment::Quad {
        control: [9119, 7518],
        to: [9212, 7425],
    },
    HelpLabelSegment::Quad {
        control: [9297, 7341],
        to: [9413, 7341],
    },
    HelpLabelSegment::Quad {
        control: [9546, 7341],
        to: [9623, 7416],
    },
    HelpLabelSegment::Quad {
        control: [9696, 7489],
        to: [9696, 7610],
    },
    HelpLabelSegment::Quad {
        control: [9696, 7646],
        to: [9688, 7678],
    },
    HelpLabelSegment::Line([9236, 7678]),
    HelpLabelSegment::Quad {
        control: [9236, 7788],
        to: [9296, 7846],
    },
];

const BODY_CONTOUR_342: [HelpLabelSegment; 22] = [
    HelpLabelSegment::Line([10122, 7488]),
    HelpLabelSegment::Quad {
        control: [10056, 7435],
        to: [9989, 7435],
    },
    HelpLabelSegment::Quad {
        control: [9949, 7435],
        to: [9922, 7454],
    },
    HelpLabelSegment::Quad {
        control: [9894, 7473],
        to: [9894, 7501],
    },
    HelpLabelSegment::Quad {
        control: [9894, 7562],
        to: [9964, 7592],
    },
    HelpLabelSegment::Line([10043, 7628]),
    HelpLabelSegment::Quad {
        control: [10116, 7662],
        to: [10150, 7705],
    },
    HelpLabelSegment::Quad {
        control: [10183, 7748],
        to: [10183, 7812],
    },
    HelpLabelSegment::Quad {
        control: [10183, 7897],
        to: [10124, 7945],
    },
    HelpLabelSegment::Quad {
        control: [10064, 7992],
        to: [9960, 7992],
    },
    HelpLabelSegment::Quad {
        control: [9860, 7992],
        to: [9774, 7942],
    },
    HelpLabelSegment::Line([9812, 7837]),
    HelpLabelSegment::Quad {
        control: [9906, 7898],
        to: [9962, 7898],
    },
    HelpLabelSegment::Quad {
        control: [10065, 7898],
        to: [10065, 7811],
    },
    HelpLabelSegment::Quad {
        control: [10065, 7749],
        to: [9966, 7705],
    },
    HelpLabelSegment::Quad {
        control: [9890, 7669],
        to: [9863, 7652],
    },
    HelpLabelSegment::Quad {
        control: [9836, 7633],
        to: [9817, 7611],
    },
    HelpLabelSegment::Quad {
        control: [9797, 7587],
        to: [9788, 7562],
    },
    HelpLabelSegment::Quad {
        control: [9777, 7535],
        to: [9777, 7505],
    },
    HelpLabelSegment::Quad {
        control: [9777, 7428],
        to: [9833, 7385],
    },
    HelpLabelSegment::Quad {
        control: [9890, 7341],
        to: [9981, 7341],
    },
    HelpLabelSegment::Quad {
        control: [10049, 7341],
        to: [10153, 7385],
    },
];

const BODY_CONTOUR_343: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [10464, 7816],
        to: [10490, 7842],
    },
    HelpLabelSegment::Quad {
        control: [10515, 7868],
        to: [10515, 7904],
    },
    HelpLabelSegment::Quad {
        control: [10515, 7940],
        to: [10490, 7966],
    },
    HelpLabelSegment::Quad {
        control: [10464, 7992],
        to: [10427, 7992],
    },
    HelpLabelSegment::Quad {
        control: [10391, 7992],
        to: [10365, 7966],
    },
    HelpLabelSegment::Quad {
        control: [10340, 7940],
        to: [10340, 7904],
    },
    HelpLabelSegment::Quad {
        control: [10340, 7868],
        to: [10365, 7842],
    },
    HelpLabelSegment::Quad {
        control: [10391, 7816],
        to: [10427, 7816],
    },
];

const BODY_CONTOUR_344: [HelpLabelSegment; 16] = [
    HelpLabelSegment::Line([-11223, 8918]),
    HelpLabelSegment::Line([-11223, 9127]),
    HelpLabelSegment::Quad {
        control: [-11179, 9094],
        to: [-11112, 9094],
    },
    HelpLabelSegment::Quad {
        control: [-10993, 9094],
        to: [-10930, 9166],
    },
    HelpLabelSegment::Quad {
        control: [-10866, 9237],
        to: [-10866, 9369],
    },
    HelpLabelSegment::Quad {
        control: [-10866, 9692],
        to: [-11155, 9692],
    },
    HelpLabelSegment::Quad {
        control: [-11274, 9692],
        to: [-11354, 9625],
    },
    HelpLabelSegment::Line([-11308, 9528]),
    HelpLabelSegment::Quad {
        control: [-11228, 9592],
        to: [-11155, 9592],
    },
    HelpLabelSegment::Quad {
        control: [-10989, 9592],
        to: [-10989, 9386],
    },
    HelpLabelSegment::Quad {
        control: [-10989, 9194],
        to: [-11152, 9194],
    },
    HelpLabelSegment::Quad {
        control: [-11231, 9194],
        to: [-11294, 9262],
    },
    HelpLabelSegment::Line([-11334, 9234]),
    HelpLabelSegment::Line([-11334, 8819]),
    HelpLabelSegment::Line([-10906, 8819]),
    HelpLabelSegment::Line([-10906, 8918]),
];

const BODY_CONTOUR_345: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [-10556, 9516],
        to: [-10530, 9542],
    },
    HelpLabelSegment::Quad {
        control: [-10505, 9568],
        to: [-10505, 9604],
    },
    HelpLabelSegment::Quad {
        control: [-10505, 9640],
        to: [-10530, 9666],
    },
    HelpLabelSegment::Quad {
        control: [-10556, 9692],
        to: [-10593, 9692],
    },
    HelpLabelSegment::Quad {
        control: [-10629, 9692],
        to: [-10655, 9666],
    },
    HelpLabelSegment::Quad {
        control: [-10680, 9640],
        to: [-10680, 9604],
    },
    HelpLabelSegment::Quad {
        control: [-10680, 9568],
        to: [-10655, 9542],
    },
    HelpLabelSegment::Quad {
        control: [-10629, 9516],
        to: [-10593, 9516],
    },
];

const BODY_CONTOUR_346: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([-9613, 9692]),
    HelpLabelSegment::Line([-9672, 9692]),
    HelpLabelSegment::Line([-9992, 8822]),
    HelpLabelSegment::Line([-9862, 8822]),
    HelpLabelSegment::Line([-9641, 9454]),
    HelpLabelSegment::Line([-9428, 8822]),
    HelpLabelSegment::Line([-9303, 8822]),
];

const BODY_CONTOUR_347: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [-9106, 9135],
        to: [-9157, 9183],
    },
    HelpLabelSegment::Quad {
        control: [-9205, 9229],
        to: [-9212, 9297],
    },
    HelpLabelSegment::Line([-8864, 9297]),
    HelpLabelSegment::Quad {
        control: [-8864, 9229],
        to: [-8906, 9184],
    },
    HelpLabelSegment::Quad {
        control: [-8953, 9135],
        to: [-9032, 9135],
    },
];

const BODY_CONTOUR_348: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [-9100, 9598],
        to: [-9017, 9598],
    },
    HelpLabelSegment::Quad {
        control: [-8921, 9598],
        to: [-8858, 9543],
    },
    HelpLabelSegment::Line([-8811, 9623]),
    HelpLabelSegment::Quad {
        control: [-8837, 9648],
        to: [-8890, 9667],
    },
    HelpLabelSegment::Quad {
        control: [-8956, 9692],
        to: [-9038, 9692],
    },
    HelpLabelSegment::Quad {
        control: [-9157, 9692],
        to: [-9240, 9612],
    },
    HelpLabelSegment::Quad {
        control: [-9331, 9523],
        to: [-9331, 9374],
    },
    HelpLabelSegment::Quad {
        control: [-9331, 9218],
        to: [-9238, 9125],
    },
    HelpLabelSegment::Quad {
        control: [-9153, 9041],
        to: [-9037, 9041],
    },
    HelpLabelSegment::Quad {
        control: [-8904, 9041],
        to: [-8827, 9116],
    },
    HelpLabelSegment::Quad {
        control: [-8754, 9189],
        to: [-8754, 9310],
    },
    HelpLabelSegment::Quad {
        control: [-8754, 9346],
        to: [-8762, 9378],
    },
    HelpLabelSegment::Line([-9214, 9378]),
    HelpLabelSegment::Quad {
        control: [-9214, 9488],
        to: [-9154, 9546],
    },
];

const BODY_CONTOUR_349: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Quad {
        control: [-8331, 9135],
        to: [-8368, 9135],
    },
    HelpLabelSegment::Quad {
        control: [-8427, 9135],
        to: [-8471, 9189],
    },
    HelpLabelSegment::Quad {
        control: [-8516, 9244],
        to: [-8516, 9320],
    },
    HelpLabelSegment::Line([-8516, 9680]),
    HelpLabelSegment::Line([-8627, 9680]),
    HelpLabelSegment::Line([-8627, 9053]),
    HelpLabelSegment::Line([-8516, 9053]),
    HelpLabelSegment::Line([-8516, 9153]),
    HelpLabelSegment::Quad {
        control: [-8455, 9041],
        to: [-8334, 9041],
    },
    HelpLabelSegment::Line([-8249, 9052]),
    HelpLabelSegment::Line([-8294, 9160]),
];

const BODY_CONTOUR_350: [HelpLabelSegment; 13] = [
    HelpLabelSegment::Line([-7931, 9787]),
    HelpLabelSegment::Quad {
        control: [-7952, 9846],
        to: [-8021, 9886],
    },
    HelpLabelSegment::Quad {
        control: [-8092, 9926],
        to: [-8177, 9926],
    },
    HelpLabelSegment::Line([-8177, 9826]),
    HelpLabelSegment::Quad {
        control: [-8107, 9826],
        to: [-8058, 9795],
    },
    HelpLabelSegment::Quad {
        control: [-8007, 9762],
        to: [-8007, 9715],
    },
    HelpLabelSegment::Quad {
        control: [-8007, 9664],
        to: [-8026, 9613],
    },
    HelpLabelSegment::Line([-8073, 9489]),
    HelpLabelSegment::Line([-8243, 9053]),
    HelpLabelSegment::Line([-8129, 9053]),
    HelpLabelSegment::Line([-7944, 9538]),
    HelpLabelSegment::Line([-7779, 9053]),
    HelpLabelSegment::Line([-7665, 9053]),
];

const BODY_CONTOUR_351: [HelpLabelSegment; 13] = [
    HelpLabelSegment::Line([-7109, 9099]),
    HelpLabelSegment::Quad {
        control: [-7094, 9078],
        to: [-7049, 9059],
    },
    HelpLabelSegment::Quad {
        control: [-7006, 9041],
        to: [-6964, 9041],
    },
    HelpLabelSegment::Quad {
        control: [-6835, 9041],
        to: [-6755, 9130],
    },
    HelpLabelSegment::Quad {
        control: [-6675, 9219],
        to: [-6675, 9355],
    },
    HelpLabelSegment::Quad {
        control: [-6675, 9512],
        to: [-6755, 9603],
    },
    HelpLabelSegment::Quad {
        control: [-6836, 9692],
        to: [-6973, 9692],
    },
    HelpLabelSegment::Quad {
        control: [-7018, 9692],
        to: [-7060, 9675],
    },
    HelpLabelSegment::Quad {
        control: [-7103, 9659],
        to: [-7125, 9635],
    },
    HelpLabelSegment::Line([-7165, 9692]),
    HelpLabelSegment::Line([-7220, 9692]),
    HelpLabelSegment::Line([-7220, 8795]),
    HelpLabelSegment::Line([-7109, 8795]),
];

const BODY_CONTOUR_352: [HelpLabelSegment; 9] = [
    HelpLabelSegment::Line([-7109, 9546]),
    HelpLabelSegment::Quad {
        control: [-7109, 9556],
        to: [-7068, 9577],
    },
    HelpLabelSegment::Quad {
        control: [-7026, 9598],
        to: [-7005, 9598],
    },
    HelpLabelSegment::Quad {
        control: [-6891, 9598],
        to: [-6842, 9544],
    },
    HelpLabelSegment::Quad {
        control: [-6793, 9489],
        to: [-6793, 9361],
    },
    HelpLabelSegment::Quad {
        control: [-6793, 9255],
        to: [-6850, 9195],
    },
    HelpLabelSegment::Quad {
        control: [-6907, 9135],
        to: [-7005, 9135],
    },
    HelpLabelSegment::Quad {
        control: [-7025, 9135],
        to: [-7061, 9153],
    },
    HelpLabelSegment::Line([-7109, 9184]),
];

const BODY_CONTOUR_353: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [-6407, 8815],
        to: [-6386, 8836],
    },
    HelpLabelSegment::Quad {
        control: [-6366, 8856],
        to: [-6366, 8884],
    },
    HelpLabelSegment::Quad {
        control: [-6366, 8912],
        to: [-6386, 8934],
    },
    HelpLabelSegment::Quad {
        control: [-6407, 8953],
        to: [-6435, 8953],
    },
    HelpLabelSegment::Quad {
        control: [-6464, 8953],
        to: [-6484, 8934],
    },
    HelpLabelSegment::Quad {
        control: [-6505, 8912],
        to: [-6505, 8884],
    },
    HelpLabelSegment::Quad {
        control: [-6505, 8855],
        to: [-6485, 8835],
    },
    HelpLabelSegment::Quad {
        control: [-6465, 8815],
        to: [-6435, 8815],
    },
];

const BODY_CONTOUR_354: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Line([-6385, 9680]),
    HelpLabelSegment::Line([-6496, 9680]),
    HelpLabelSegment::Line([-6496, 9147]),
    HelpLabelSegment::Line([-6583, 9147]),
    HelpLabelSegment::Line([-6583, 9053]),
    HelpLabelSegment::Line([-6385, 9053]),
];

const BODY_CONTOUR_355: [HelpLabelSegment; 32] = [
    HelpLabelSegment::Line([-5799, 9132]),
    HelpLabelSegment::Quad {
        control: [-5756, 9187],
        to: [-5756, 9277],
    },
    HelpLabelSegment::Quad {
        control: [-5756, 9372],
        to: [-5815, 9436],
    },
    HelpLabelSegment::Quad {
        control: [-5874, 9501],
        to: [-5970, 9510],
    },
    HelpLabelSegment::Line([-6063, 9519]),
    HelpLabelSegment::Line([-6106, 9532]),
    HelpLabelSegment::Quad {
        control: [-6134, 9543],
        to: [-6134, 9560],
    },
    HelpLabelSegment::Quad {
        control: [-6134, 9584],
        to: [-6077, 9584],
    },
    HelpLabelSegment::Line([-5998, 9576]),
    HelpLabelSegment::Line([-5919, 9566]),
    HelpLabelSegment::Quad {
        control: [-5826, 9566],
        to: [-5774, 9611],
    },
    HelpLabelSegment::Quad {
        control: [-5723, 9654],
        to: [-5723, 9733],
    },
    HelpLabelSegment::Quad {
        control: [-5723, 9819],
        to: [-5800, 9873],
    },
    HelpLabelSegment::Quad {
        control: [-5878, 9926],
        to: [-5997, 9926],
    },
    HelpLabelSegment::Quad {
        control: [-6058, 9926],
        to: [-6125, 9905],
    },
    HelpLabelSegment::Quad {
        control: [-6193, 9883],
        to: [-6234, 9852],
    },
    HelpLabelSegment::Line([-6173, 9763]),
    HelpLabelSegment::Quad {
        control: [-6076, 9828],
        to: [-5994, 9828],
    },
    HelpLabelSegment::Quad {
        control: [-5919, 9828],
        to: [-5874, 9802],
    },
    HelpLabelSegment::Quad {
        control: [-5831, 9776],
        to: [-5831, 9737],
    },
    HelpLabelSegment::Quad {
        control: [-5831, 9661],
        to: [-5941, 9661],
    },
    HelpLabelSegment::Line([-6009, 9671]),
    HelpLabelSegment::Line([-6086, 9680]),
    HelpLabelSegment::Quad {
        control: [-6220, 9680],
        to: [-6220, 9579],
    },
    HelpLabelSegment::Quad {
        control: [-6220, 9548],
        to: [-6188, 9523],
    },
    HelpLabelSegment::Quad {
        control: [-6156, 9497],
        to: [-6111, 9487],
    },
    HelpLabelSegment::Quad {
        control: [-6243, 9425],
        to: [-6243, 9271],
    },
    HelpLabelSegment::Quad {
        control: [-6243, 9173],
        to: [-6174, 9107],
    },
    HelpLabelSegment::Quad {
        control: [-6106, 9041],
        to: [-6005, 9041],
    },
    HelpLabelSegment::Quad {
        control: [-5913, 9041],
        to: [-5860, 9079],
    },
    HelpLabelSegment::Line([-5805, 9012]),
    HelpLabelSegment::Line([-5732, 9081]),
];

const BODY_CONTOUR_356: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [-6056, 9130],
        to: [-6092, 9171],
    },
    HelpLabelSegment::Quad {
        control: [-6128, 9212],
        to: [-6128, 9271],
    },
    HelpLabelSegment::Quad {
        control: [-6128, 9337],
        to: [-6093, 9380],
    },
    HelpLabelSegment::Quad {
        control: [-6058, 9423],
        to: [-5997, 9423],
    },
    HelpLabelSegment::Quad {
        control: [-5938, 9423],
        to: [-5904, 9381],
    },
    HelpLabelSegment::Quad {
        control: [-5872, 9339],
        to: [-5872, 9271],
    },
    HelpLabelSegment::Quad {
        control: [-5872, 9212],
        to: [-5907, 9171],
    },
    HelpLabelSegment::Quad {
        control: [-5943, 9130],
        to: [-5997, 9130],
    },
];

const BODY_CONTOUR_357: [HelpLabelSegment; 13] = [
    HelpLabelSegment::Line([-5139, 9099]),
    HelpLabelSegment::Quad {
        control: [-5124, 9078],
        to: [-5079, 9059],
    },
    HelpLabelSegment::Quad {
        control: [-5036, 9041],
        to: [-4994, 9041],
    },
    HelpLabelSegment::Quad {
        control: [-4865, 9041],
        to: [-4785, 9130],
    },
    HelpLabelSegment::Quad {
        control: [-4705, 9219],
        to: [-4705, 9355],
    },
    HelpLabelSegment::Quad {
        control: [-4705, 9512],
        to: [-4785, 9603],
    },
    HelpLabelSegment::Quad {
        control: [-4866, 9692],
        to: [-5003, 9692],
    },
    HelpLabelSegment::Quad {
        control: [-5048, 9692],
        to: [-5090, 9675],
    },
    HelpLabelSegment::Quad {
        control: [-5133, 9659],
        to: [-5155, 9635],
    },
    HelpLabelSegment::Line([-5195, 9692]),
    HelpLabelSegment::Line([-5250, 9692]),
    HelpLabelSegment::Line([-5250, 8795]),
    HelpLabelSegment::Line([-5139, 8795]),
];

const BODY_CONTOUR_358: [HelpLabelSegment; 9] = [
    HelpLabelSegment::Line([-5139, 9546]),
    HelpLabelSegment::Quad {
        control: [-5139, 9556],
        to: [-5098, 9577],
    },
    HelpLabelSegment::Quad {
        control: [-5056, 9598],
        to: [-5035, 9598],
    },
    HelpLabelSegment::Quad {
        control: [-4921, 9598],
        to: [-4872, 9544],
    },
    HelpLabelSegment::Quad {
        control: [-4823, 9489],
        to: [-4823, 9361],
    },
    HelpLabelSegment::Quad {
        control: [-4823, 9255],
        to: [-4880, 9195],
    },
    HelpLabelSegment::Quad {
        control: [-4937, 9135],
        to: [-5035, 9135],
    },
    HelpLabelSegment::Quad {
        control: [-5055, 9135],
        to: [-5091, 9153],
    },
    HelpLabelSegment::Line([-5139, 9184]),
];

const BODY_CONTOUR_359: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Quad {
        control: [-4249, 9041],
        to: [-4187, 9103],
    },
    HelpLabelSegment::Quad {
        control: [-4126, 9166],
        to: [-4126, 9300],
    },
    HelpLabelSegment::Line([-4126, 9525]),
    HelpLabelSegment::Quad {
        control: [-4126, 9609],
        to: [-4076, 9635],
    },
    HelpLabelSegment::Line([-4076, 9692]),
    HelpLabelSegment::Quad {
        control: [-4144, 9692],
        to: [-4177, 9672],
    },
    HelpLabelSegment::Quad {
        control: [-4211, 9653],
        to: [-4226, 9609],
    },
    HelpLabelSegment::Quad {
        control: [-4293, 9692],
        to: [-4430, 9692],
    },
    HelpLabelSegment::Quad {
        control: [-4504, 9692],
        to: [-4558, 9639],
    },
    HelpLabelSegment::Quad {
        control: [-4613, 9585],
        to: [-4613, 9505],
    },
    HelpLabelSegment::Quad {
        control: [-4613, 9409],
        to: [-4529, 9344],
    },
    HelpLabelSegment::Quad {
        control: [-4446, 9278],
        to: [-4317, 9278],
    },
    HelpLabelSegment::Line([-4237, 9293]),
    HelpLabelSegment::Quad {
        control: [-4237, 9141],
        to: [-4373, 9141],
    },
    HelpLabelSegment::Quad {
        control: [-4477, 9141],
        to: [-4533, 9197],
    },
    HelpLabelSegment::Line([-4580, 9103]),
    HelpLabelSegment::Quad {
        control: [-4549, 9078],
        to: [-4492, 9060],
    },
    HelpLabelSegment::Quad {
        control: [-4436, 9041],
        to: [-4386, 9041],
    },
];

const BODY_CONTOUR_360: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [-4395, 9360],
        to: [-4448, 9403],
    },
    HelpLabelSegment::Quad {
        control: [-4502, 9447],
        to: [-4502, 9507],
    },
    HelpLabelSegment::Quad {
        control: [-4502, 9604],
        to: [-4386, 9604],
    },
    HelpLabelSegment::Quad {
        control: [-4301, 9604],
        to: [-4237, 9524],
    },
    HelpLabelSegment::Line([-4237, 9372]),
    HelpLabelSegment::Line([-4311, 9360]),
];

const BODY_CONTOUR_361: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([-3725, 9692]),
    HelpLabelSegment::Quad {
        control: [-3942, 9692],
        to: [-3942, 9503],
    },
    HelpLabelSegment::Line([-3942, 8795]),
    HelpLabelSegment::Line([-3831, 8795]),
    HelpLabelSegment::Line([-3831, 9484]),
    HelpLabelSegment::Quad {
        control: [-3831, 9535],
        to: [-3801, 9564],
    },
    HelpLabelSegment::Quad {
        control: [-3772, 9592],
        to: [-3725, 9592],
    },
];

const BODY_CONTOUR_362: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([-3370, 9692]),
    HelpLabelSegment::Quad {
        control: [-3587, 9692],
        to: [-3587, 9503],
    },
    HelpLabelSegment::Line([-3587, 8795]),
    HelpLabelSegment::Line([-3476, 8795]),
    HelpLabelSegment::Line([-3476, 9484]),
    HelpLabelSegment::Quad {
        control: [-3476, 9535],
        to: [-3446, 9564],
    },
    HelpLabelSegment::Quad {
        control: [-3417, 9592],
        to: [-3370, 9592],
    },
];

const BODY_CONTOUR_363: [HelpLabelSegment; 22] = [
    HelpLabelSegment::Line([-2933, 9188]),
    HelpLabelSegment::Quad {
        control: [-2999, 9135],
        to: [-3066, 9135],
    },
    HelpLabelSegment::Quad {
        control: [-3106, 9135],
        to: [-3133, 9154],
    },
    HelpLabelSegment::Quad {
        control: [-3161, 9173],
        to: [-3161, 9201],
    },
    HelpLabelSegment::Quad {
        control: [-3161, 9262],
        to: [-3091, 9292],
    },
    HelpLabelSegment::Line([-3012, 9328]),
    HelpLabelSegment::Quad {
        control: [-2939, 9362],
        to: [-2905, 9405],
    },
    HelpLabelSegment::Quad {
        control: [-2872, 9448],
        to: [-2872, 9512],
    },
    HelpLabelSegment::Quad {
        control: [-2872, 9597],
        to: [-2931, 9645],
    },
    HelpLabelSegment::Quad {
        control: [-2991, 9692],
        to: [-3095, 9692],
    },
    HelpLabelSegment::Quad {
        control: [-3195, 9692],
        to: [-3281, 9642],
    },
    HelpLabelSegment::Line([-3243, 9537]),
    HelpLabelSegment::Quad {
        control: [-3149, 9598],
        to: [-3093, 9598],
    },
    HelpLabelSegment::Quad {
        control: [-2990, 9598],
        to: [-2990, 9511],
    },
    HelpLabelSegment::Quad {
        control: [-2990, 9449],
        to: [-3089, 9405],
    },
    HelpLabelSegment::Quad {
        control: [-3165, 9369],
        to: [-3192, 9352],
    },
    HelpLabelSegment::Quad {
        control: [-3219, 9333],
        to: [-3238, 9311],
    },
    HelpLabelSegment::Quad {
        control: [-3258, 9287],
        to: [-3267, 9262],
    },
    HelpLabelSegment::Quad {
        control: [-3278, 9235],
        to: [-3278, 9205],
    },
    HelpLabelSegment::Quad {
        control: [-3278, 9128],
        to: [-3222, 9085],
    },
    HelpLabelSegment::Quad {
        control: [-3165, 9041],
        to: [-3074, 9041],
    },
    HelpLabelSegment::Quad {
        control: [-3006, 9041],
        to: [-2902, 9085],
    },
];

const BODY_CONTOUR_364: [HelpLabelSegment; 22] = [
    HelpLabelSegment::Line([-2088, 9188]),
    HelpLabelSegment::Quad {
        control: [-2154, 9135],
        to: [-2221, 9135],
    },
    HelpLabelSegment::Quad {
        control: [-2261, 9135],
        to: [-2288, 9154],
    },
    HelpLabelSegment::Quad {
        control: [-2316, 9173],
        to: [-2316, 9201],
    },
    HelpLabelSegment::Quad {
        control: [-2316, 9262],
        to: [-2246, 9292],
    },
    HelpLabelSegment::Line([-2167, 9328]),
    HelpLabelSegment::Quad {
        control: [-2094, 9362],
        to: [-2060, 9405],
    },
    HelpLabelSegment::Quad {
        control: [-2027, 9448],
        to: [-2027, 9512],
    },
    HelpLabelSegment::Quad {
        control: [-2027, 9597],
        to: [-2086, 9645],
    },
    HelpLabelSegment::Quad {
        control: [-2146, 9692],
        to: [-2250, 9692],
    },
    HelpLabelSegment::Quad {
        control: [-2350, 9692],
        to: [-2436, 9642],
    },
    HelpLabelSegment::Line([-2398, 9537]),
    HelpLabelSegment::Quad {
        control: [-2304, 9598],
        to: [-2248, 9598],
    },
    HelpLabelSegment::Quad {
        control: [-2145, 9598],
        to: [-2145, 9511],
    },
    HelpLabelSegment::Quad {
        control: [-2145, 9449],
        to: [-2244, 9405],
    },
    HelpLabelSegment::Quad {
        control: [-2320, 9369],
        to: [-2347, 9352],
    },
    HelpLabelSegment::Quad {
        control: [-2374, 9333],
        to: [-2393, 9311],
    },
    HelpLabelSegment::Quad {
        control: [-2413, 9287],
        to: [-2422, 9262],
    },
    HelpLabelSegment::Quad {
        control: [-2433, 9235],
        to: [-2433, 9205],
    },
    HelpLabelSegment::Quad {
        control: [-2433, 9128],
        to: [-2377, 9085],
    },
    HelpLabelSegment::Quad {
        control: [-2320, 9041],
        to: [-2229, 9041],
    },
    HelpLabelSegment::Quad {
        control: [-2161, 9041],
        to: [-2057, 9085],
    },
];

const BODY_CONTOUR_365: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Line([-1870, 8922]),
    HelpLabelSegment::Line([-1759, 8878]),
    HelpLabelSegment::Line([-1759, 9053]),
    HelpLabelSegment::Line([-1587, 9053]),
    HelpLabelSegment::Line([-1587, 9141]),
    HelpLabelSegment::Line([-1759, 9141]),
    HelpLabelSegment::Line([-1759, 9453]),
    HelpLabelSegment::Quad {
        control: [-1759, 9531],
        to: [-1732, 9565],
    },
    HelpLabelSegment::Quad {
        control: [-1706, 9598],
        to: [-1647, 9598],
    },
    HelpLabelSegment::Quad {
        control: [-1604, 9598],
        to: [-1559, 9577],
    },
    HelpLabelSegment::Line([-1542, 9674]),
    HelpLabelSegment::Line([-1694, 9692]),
    HelpLabelSegment::Quad {
        control: [-1769, 9692],
        to: [-1819, 9637],
    },
    HelpLabelSegment::Quad {
        control: [-1870, 9582],
        to: [-1870, 9497],
    },
    HelpLabelSegment::Line([-1870, 9141]),
    HelpLabelSegment::Line([-1943, 9141]),
    HelpLabelSegment::Line([-1943, 9053]),
    HelpLabelSegment::Line([-1870, 9053]),
];

const BODY_CONTOUR_366: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Quad {
        control: [-1104, 9041],
        to: [-1042, 9103],
    },
    HelpLabelSegment::Quad {
        control: [-981, 9166],
        to: [-981, 9300],
    },
    HelpLabelSegment::Line([-981, 9525]),
    HelpLabelSegment::Quad {
        control: [-981, 9609],
        to: [-931, 9635],
    },
    HelpLabelSegment::Line([-931, 9692]),
    HelpLabelSegment::Quad {
        control: [-999, 9692],
        to: [-1032, 9672],
    },
    HelpLabelSegment::Quad {
        control: [-1066, 9653],
        to: [-1081, 9609],
    },
    HelpLabelSegment::Quad {
        control: [-1148, 9692],
        to: [-1285, 9692],
    },
    HelpLabelSegment::Quad {
        control: [-1359, 9692],
        to: [-1413, 9639],
    },
    HelpLabelSegment::Quad {
        control: [-1468, 9585],
        to: [-1468, 9505],
    },
    HelpLabelSegment::Quad {
        control: [-1468, 9409],
        to: [-1384, 9344],
    },
    HelpLabelSegment::Quad {
        control: [-1301, 9278],
        to: [-1172, 9278],
    },
    HelpLabelSegment::Line([-1092, 9293]),
    HelpLabelSegment::Quad {
        control: [-1092, 9141],
        to: [-1228, 9141],
    },
    HelpLabelSegment::Quad {
        control: [-1332, 9141],
        to: [-1388, 9197],
    },
    HelpLabelSegment::Line([-1435, 9103]),
    HelpLabelSegment::Quad {
        control: [-1404, 9078],
        to: [-1347, 9060],
    },
    HelpLabelSegment::Quad {
        control: [-1291, 9041],
        to: [-1241, 9041],
    },
];

const BODY_CONTOUR_367: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [-1250, 9360],
        to: [-1303, 9403],
    },
    HelpLabelSegment::Quad {
        control: [-1357, 9447],
        to: [-1357, 9507],
    },
    HelpLabelSegment::Quad {
        control: [-1357, 9604],
        to: [-1241, 9604],
    },
    HelpLabelSegment::Quad {
        control: [-1156, 9604],
        to: [-1092, 9524],
    },
    HelpLabelSegment::Line([-1092, 9372]),
    HelpLabelSegment::Line([-1166, 9360]),
];

const BODY_CONTOUR_368: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Quad {
        control: [-501, 9135],
        to: [-538, 9135],
    },
    HelpLabelSegment::Quad {
        control: [-597, 9135],
        to: [-641, 9189],
    },
    HelpLabelSegment::Quad {
        control: [-686, 9244],
        to: [-686, 9320],
    },
    HelpLabelSegment::Line([-686, 9680]),
    HelpLabelSegment::Line([-797, 9680]),
    HelpLabelSegment::Line([-797, 9053]),
    HelpLabelSegment::Line([-686, 9053]),
    HelpLabelSegment::Line([-686, 9153]),
    HelpLabelSegment::Quad {
        control: [-625, 9041],
        to: [-504, 9041],
    },
    HelpLabelSegment::Line([-419, 9052]),
    HelpLabelSegment::Line([-464, 9160]),
];

const BODY_CONTOUR_369: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Line([-300, 8922]),
    HelpLabelSegment::Line([-189, 8878]),
    HelpLabelSegment::Line([-189, 9053]),
    HelpLabelSegment::Line([-17, 9053]),
    HelpLabelSegment::Line([-17, 9141]),
    HelpLabelSegment::Line([-189, 9141]),
    HelpLabelSegment::Line([-189, 9453]),
    HelpLabelSegment::Quad {
        control: [-189, 9531],
        to: [-162, 9565],
    },
    HelpLabelSegment::Quad {
        control: [-136, 9598],
        to: [-77, 9598],
    },
    HelpLabelSegment::Quad {
        control: [-34, 9598],
        to: [11, 9577],
    },
    HelpLabelSegment::Line([28, 9674]),
    HelpLabelSegment::Line([-124, 9692]),
    HelpLabelSegment::Quad {
        control: [-199, 9692],
        to: [-249, 9637],
    },
    HelpLabelSegment::Quad {
        control: [-300, 9582],
        to: [-300, 9497],
    },
    HelpLabelSegment::Line([-300, 9141]),
    HelpLabelSegment::Line([-373, 9141]),
    HelpLabelSegment::Line([-373, 9053]),
    HelpLabelSegment::Line([-300, 9053]),
];

const BODY_CONTOUR_370: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Line([535, 8922]),
    HelpLabelSegment::Line([646, 8878]),
    HelpLabelSegment::Line([646, 9053]),
    HelpLabelSegment::Line([818, 9053]),
    HelpLabelSegment::Line([818, 9141]),
    HelpLabelSegment::Line([646, 9141]),
    HelpLabelSegment::Line([646, 9453]),
    HelpLabelSegment::Quad {
        control: [646, 9531],
        to: [673, 9565],
    },
    HelpLabelSegment::Quad {
        control: [699, 9598],
        to: [758, 9598],
    },
    HelpLabelSegment::Quad {
        control: [801, 9598],
        to: [846, 9577],
    },
    HelpLabelSegment::Line([863, 9674]),
    HelpLabelSegment::Line([711, 9692]),
    HelpLabelSegment::Quad {
        control: [636, 9692],
        to: [586, 9637],
    },
    HelpLabelSegment::Quad {
        control: [535, 9582],
        to: [535, 9497],
    },
    HelpLabelSegment::Line([535, 9141]),
    HelpLabelSegment::Line([462, 9141]),
    HelpLabelSegment::Line([462, 9053]),
    HelpLabelSegment::Line([535, 9053]),
];

const BODY_CONTOUR_371: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [1046, 9601],
        to: [1212, 9601],
    },
    HelpLabelSegment::Quad {
        control: [1291, 9601],
        to: [1335, 9538],
    },
    HelpLabelSegment::Quad {
        control: [1379, 9475],
        to: [1379, 9365],
    },
    HelpLabelSegment::Quad {
        control: [1379, 9132],
        to: [1212, 9132],
    },
    HelpLabelSegment::Quad {
        control: [1136, 9132],
        to: [1092, 9194],
    },
    HelpLabelSegment::Quad {
        control: [1046, 9256],
        to: [1046, 9365],
    },
];

const BODY_CONTOUR_372: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [1085, 9041],
        to: [1212, 9041],
    },
    HelpLabelSegment::Quad {
        control: [1347, 9041],
        to: [1422, 9127],
    },
    HelpLabelSegment::Quad {
        control: [1496, 9212],
        to: [1496, 9365],
    },
    HelpLabelSegment::Quad {
        control: [1496, 9517],
        to: [1420, 9605],
    },
    HelpLabelSegment::Quad {
        control: [1344, 9692],
        to: [1212, 9692],
    },
    HelpLabelSegment::Quad {
        control: [1079, 9692],
        to: [1004, 9604],
    },
    HelpLabelSegment::Quad {
        control: [929, 9515],
        to: [929, 9365],
    },
    HelpLabelSegment::Quad {
        control: [929, 9219],
        to: [1007, 9130],
    },
];

const BODY_CONTOUR_373: [HelpLabelSegment; 13] = [
    HelpLabelSegment::Line([2086, 9099]),
    HelpLabelSegment::Quad {
        control: [2101, 9078],
        to: [2146, 9059],
    },
    HelpLabelSegment::Quad {
        control: [2189, 9041],
        to: [2231, 9041],
    },
    HelpLabelSegment::Quad {
        control: [2360, 9041],
        to: [2440, 9130],
    },
    HelpLabelSegment::Quad {
        control: [2520, 9219],
        to: [2520, 9355],
    },
    HelpLabelSegment::Quad {
        control: [2520, 9512],
        to: [2440, 9603],
    },
    HelpLabelSegment::Quad {
        control: [2359, 9692],
        to: [2222, 9692],
    },
    HelpLabelSegment::Quad {
        control: [2177, 9692],
        to: [2135, 9675],
    },
    HelpLabelSegment::Quad {
        control: [2092, 9659],
        to: [2070, 9635],
    },
    HelpLabelSegment::Line([2030, 9692]),
    HelpLabelSegment::Line([1975, 9692]),
    HelpLabelSegment::Line([1975, 8795]),
    HelpLabelSegment::Line([2086, 8795]),
];

const BODY_CONTOUR_374: [HelpLabelSegment; 9] = [
    HelpLabelSegment::Line([2086, 9546]),
    HelpLabelSegment::Quad {
        control: [2086, 9556],
        to: [2127, 9577],
    },
    HelpLabelSegment::Quad {
        control: [2169, 9598],
        to: [2190, 9598],
    },
    HelpLabelSegment::Quad {
        control: [2304, 9598],
        to: [2353, 9544],
    },
    HelpLabelSegment::Quad {
        control: [2402, 9489],
        to: [2402, 9361],
    },
    HelpLabelSegment::Quad {
        control: [2402, 9255],
        to: [2345, 9195],
    },
    HelpLabelSegment::Quad {
        control: [2288, 9135],
        to: [2190, 9135],
    },
    HelpLabelSegment::Quad {
        control: [2170, 9135],
        to: [2134, 9153],
    },
    HelpLabelSegment::Line([2086, 9184]),
];

const BODY_CONTOUR_375: [HelpLabelSegment; 15] = [
    HelpLabelSegment::Line([2750, 9453]),
    HelpLabelSegment::Quad {
        control: [2750, 9598],
        to: [2876, 9598],
    },
    HelpLabelSegment::Quad {
        control: [2931, 9598],
        to: [2976, 9566],
    },
    HelpLabelSegment::Quad {
        control: [3022, 9535],
        to: [3037, 9494],
    },
    HelpLabelSegment::Line([3037, 9053]),
    HelpLabelSegment::Line([3149, 9053]),
    HelpLabelSegment::Line([3149, 9680]),
    HelpLabelSegment::Line([3037, 9680]),
    HelpLabelSegment::Line([3037, 9593]),
    HelpLabelSegment::Quad {
        control: [3019, 9631],
        to: [2962, 9661],
    },
    HelpLabelSegment::Quad {
        control: [2905, 9692],
        to: [2851, 9692],
    },
    HelpLabelSegment::Quad {
        control: [2748, 9692],
        to: [2694, 9633],
    },
    HelpLabelSegment::Quad {
        control: [2639, 9573],
        to: [2639, 9464],
    },
    HelpLabelSegment::Line([2639, 9053]),
    HelpLabelSegment::Line([2750, 9053]),
];

const BODY_CONTOUR_376: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Quad {
        control: [3604, 9135],
        to: [3567, 9135],
    },
    HelpLabelSegment::Quad {
        control: [3508, 9135],
        to: [3464, 9189],
    },
    HelpLabelSegment::Quad {
        control: [3419, 9244],
        to: [3419, 9320],
    },
    HelpLabelSegment::Line([3419, 9680]),
    HelpLabelSegment::Line([3308, 9680]),
    HelpLabelSegment::Line([3308, 9053]),
    HelpLabelSegment::Line([3419, 9053]),
    HelpLabelSegment::Line([3419, 9153]),
    HelpLabelSegment::Quad {
        control: [3480, 9041],
        to: [3601, 9041],
    },
    HelpLabelSegment::Line([3686, 9052]),
    HelpLabelSegment::Line([3641, 9160]),
];

const BODY_CONTOUR_377: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Line([4262, 9680]),
    HelpLabelSegment::Line([4150, 9680]),
    HelpLabelSegment::Line([4150, 9316]),
    HelpLabelSegment::Quad {
        control: [4150, 9215],
        to: [4121, 9175],
    },
    HelpLabelSegment::Quad {
        control: [4090, 9135],
        to: [4019, 9135],
    },
    HelpLabelSegment::Quad {
        control: [3981, 9135],
        to: [3939, 9157],
    },
    HelpLabelSegment::Quad {
        control: [3898, 9181],
        to: [3876, 9214],
    },
    HelpLabelSegment::Line([3876, 9680]),
    HelpLabelSegment::Line([3765, 9680]),
    HelpLabelSegment::Line([3765, 9053]),
    HelpLabelSegment::Line([3841, 9053]),
    HelpLabelSegment::Line([3876, 9134]),
    HelpLabelSegment::Quad {
        control: [3931, 9041],
        to: [4055, 9041],
    },
    HelpLabelSegment::Quad {
        control: [4262, 9041],
        to: [4262, 9292],
    },
];

const BODY_CONTOUR_378: [HelpLabelSegment; 4] = [
    HelpLabelSegment::Line([4794, 9282]),
    HelpLabelSegment::Line([5045, 9282]),
    HelpLabelSegment::Line([5045, 9384]),
    HelpLabelSegment::Line([4794, 9384]),
];

const BODY_CONTOUR_379: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Line([6088, 8795]),
    HelpLabelSegment::Line([6088, 9680]),
    HelpLabelSegment::Line([5977, 9680]),
    HelpLabelSegment::Line([5977, 9633]),
    HelpLabelSegment::Quad {
        control: [5920, 9692],
        to: [5808, 9692],
    },
    HelpLabelSegment::Quad {
        control: [5691, 9692],
        to: [5617, 9607],
    },
    HelpLabelSegment::Quad {
        control: [5545, 9523],
        to: [5545, 9382],
    },
    HelpLabelSegment::Quad {
        control: [5545, 9241],
        to: [5629, 9141],
    },
    HelpLabelSegment::Quad {
        control: [5713, 9041],
        to: [5829, 9041],
    },
    HelpLabelSegment::Quad {
        control: [5927, 9041],
        to: [5977, 9087],
    },
    HelpLabelSegment::Line([5977, 8795]),
];

const BODY_CONTOUR_380: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Quad {
        control: [5968, 9565],
        to: [5977, 9546],
    },
    HelpLabelSegment::Line([5977, 9198]),
    HelpLabelSegment::Quad {
        control: [5935, 9135],
        to: [5862, 9135],
    },
    HelpLabelSegment::Quad {
        control: [5772, 9135],
        to: [5717, 9202],
    },
    HelpLabelSegment::Quad {
        control: [5662, 9269],
        to: [5662, 9372],
    },
    HelpLabelSegment::Quad {
        control: [5662, 9598],
        to: [5868, 9598],
    },
    HelpLabelSegment::Quad {
        control: [5894, 9598],
        to: [5931, 9582],
    },
];

const BODY_CONTOUR_381: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [6326, 9601],
        to: [6492, 9601],
    },
    HelpLabelSegment::Quad {
        control: [6571, 9601],
        to: [6615, 9538],
    },
    HelpLabelSegment::Quad {
        control: [6659, 9475],
        to: [6659, 9365],
    },
    HelpLabelSegment::Quad {
        control: [6659, 9132],
        to: [6492, 9132],
    },
    HelpLabelSegment::Quad {
        control: [6416, 9132],
        to: [6372, 9194],
    },
    HelpLabelSegment::Quad {
        control: [6326, 9256],
        to: [6326, 9365],
    },
];

const BODY_CONTOUR_382: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [6365, 9041],
        to: [6492, 9041],
    },
    HelpLabelSegment::Quad {
        control: [6627, 9041],
        to: [6702, 9127],
    },
    HelpLabelSegment::Quad {
        control: [6776, 9212],
        to: [6776, 9365],
    },
    HelpLabelSegment::Quad {
        control: [6776, 9517],
        to: [6700, 9605],
    },
    HelpLabelSegment::Quad {
        control: [6624, 9692],
        to: [6492, 9692],
    },
    HelpLabelSegment::Quad {
        control: [6359, 9692],
        to: [6284, 9604],
    },
    HelpLabelSegment::Quad {
        control: [6209, 9515],
        to: [6209, 9365],
    },
    HelpLabelSegment::Quad {
        control: [6209, 9219],
        to: [6287, 9130],
    },
];

const BODY_CONTOUR_383: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Line([7392, 9680]),
    HelpLabelSegment::Line([7280, 9680]),
    HelpLabelSegment::Line([7280, 9316]),
    HelpLabelSegment::Quad {
        control: [7280, 9215],
        to: [7251, 9175],
    },
    HelpLabelSegment::Quad {
        control: [7220, 9135],
        to: [7149, 9135],
    },
    HelpLabelSegment::Quad {
        control: [7111, 9135],
        to: [7069, 9157],
    },
    HelpLabelSegment::Quad {
        control: [7028, 9181],
        to: [7006, 9214],
    },
    HelpLabelSegment::Line([7006, 9680]),
    HelpLabelSegment::Line([6895, 9680]),
    HelpLabelSegment::Line([6895, 9053]),
    HelpLabelSegment::Line([6971, 9053]),
    HelpLabelSegment::Line([7006, 9134]),
    HelpLabelSegment::Quad {
        control: [7061, 9041],
        to: [7185, 9041],
    },
    HelpLabelSegment::Quad {
        control: [7392, 9041],
        to: [7392, 9292],
    },
];

const BODY_CONTOUR_384: [HelpLabelSegment; 4] = [
    HelpLabelSegment::Line([7526, 9044]),
    HelpLabelSegment::Line([7512, 8821]),
    HelpLabelSegment::Line([7620, 8821]),
    HelpLabelSegment::Line([7605, 9044]),
];

const BODY_CONTOUR_385: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Line([7780, 8922]),
    HelpLabelSegment::Line([7891, 8878]),
    HelpLabelSegment::Line([7891, 9053]),
    HelpLabelSegment::Line([8063, 9053]),
    HelpLabelSegment::Line([8063, 9141]),
    HelpLabelSegment::Line([7891, 9141]),
    HelpLabelSegment::Line([7891, 9453]),
    HelpLabelSegment::Quad {
        control: [7891, 9531],
        to: [7918, 9565],
    },
    HelpLabelSegment::Quad {
        control: [7944, 9598],
        to: [8003, 9598],
    },
    HelpLabelSegment::Quad {
        control: [8046, 9598],
        to: [8091, 9577],
    },
    HelpLabelSegment::Line([8108, 9674]),
    HelpLabelSegment::Line([7956, 9692]),
    HelpLabelSegment::Quad {
        control: [7881, 9692],
        to: [7831, 9637],
    },
    HelpLabelSegment::Quad {
        control: [7780, 9582],
        to: [7780, 9497],
    },
    HelpLabelSegment::Line([7780, 9141]),
    HelpLabelSegment::Line([7707, 9141]),
    HelpLabelSegment::Line([7707, 9053]),
    HelpLabelSegment::Line([7780, 9053]),
];

const BODY_CONTOUR_386: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Line([8615, 8922]),
    HelpLabelSegment::Line([8726, 8878]),
    HelpLabelSegment::Line([8726, 9053]),
    HelpLabelSegment::Line([8898, 9053]),
    HelpLabelSegment::Line([8898, 9141]),
    HelpLabelSegment::Line([8726, 9141]),
    HelpLabelSegment::Line([8726, 9453]),
    HelpLabelSegment::Quad {
        control: [8726, 9531],
        to: [8753, 9565],
    },
    HelpLabelSegment::Quad {
        control: [8779, 9598],
        to: [8838, 9598],
    },
    HelpLabelSegment::Quad {
        control: [8881, 9598],
        to: [8926, 9577],
    },
    HelpLabelSegment::Line([8943, 9674]),
    HelpLabelSegment::Line([8791, 9692]),
    HelpLabelSegment::Quad {
        control: [8716, 9692],
        to: [8666, 9637],
    },
    HelpLabelSegment::Quad {
        control: [8615, 9582],
        to: [8615, 9497],
    },
    HelpLabelSegment::Line([8615, 9141]),
    HelpLabelSegment::Line([8542, 9141]),
    HelpLabelSegment::Line([8542, 9053]),
    HelpLabelSegment::Line([8615, 9053]),
];

const BODY_CONTOUR_387: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [9126, 9601],
        to: [9292, 9601],
    },
    HelpLabelSegment::Quad {
        control: [9371, 9601],
        to: [9415, 9538],
    },
    HelpLabelSegment::Quad {
        control: [9459, 9475],
        to: [9459, 9365],
    },
    HelpLabelSegment::Quad {
        control: [9459, 9132],
        to: [9292, 9132],
    },
    HelpLabelSegment::Quad {
        control: [9216, 9132],
        to: [9172, 9194],
    },
    HelpLabelSegment::Quad {
        control: [9126, 9256],
        to: [9126, 9365],
    },
];

const BODY_CONTOUR_388: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [9165, 9041],
        to: [9292, 9041],
    },
    HelpLabelSegment::Quad {
        control: [9427, 9041],
        to: [9502, 9127],
    },
    HelpLabelSegment::Quad {
        control: [9576, 9212],
        to: [9576, 9365],
    },
    HelpLabelSegment::Quad {
        control: [9576, 9517],
        to: [9500, 9605],
    },
    HelpLabelSegment::Quad {
        control: [9424, 9692],
        to: [9292, 9692],
    },
    HelpLabelSegment::Quad {
        control: [9159, 9692],
        to: [9084, 9604],
    },
    HelpLabelSegment::Quad {
        control: [9009, 9515],
        to: [9009, 9365],
    },
    HelpLabelSegment::Quad {
        control: [9009, 9219],
        to: [9087, 9130],
    },
];

const BODY_CONTOUR_389: [HelpLabelSegment; 15] = [
    HelpLabelSegment::Line([9800, 9453]),
    HelpLabelSegment::Quad {
        control: [9800, 9598],
        to: [9926, 9598],
    },
    HelpLabelSegment::Quad {
        control: [9981, 9598],
        to: [10026, 9566],
    },
    HelpLabelSegment::Quad {
        control: [10072, 9535],
        to: [10087, 9494],
    },
    HelpLabelSegment::Line([10087, 9053]),
    HelpLabelSegment::Line([10199, 9053]),
    HelpLabelSegment::Line([10199, 9680]),
    HelpLabelSegment::Line([10087, 9680]),
    HelpLabelSegment::Line([10087, 9593]),
    HelpLabelSegment::Quad {
        control: [10069, 9631],
        to: [10012, 9661],
    },
    HelpLabelSegment::Quad {
        control: [9955, 9692],
        to: [9901, 9692],
    },
    HelpLabelSegment::Quad {
        control: [9798, 9692],
        to: [9744, 9633],
    },
    HelpLabelSegment::Quad {
        control: [9689, 9573],
        to: [9689, 9464],
    },
    HelpLabelSegment::Line([9689, 9053]),
    HelpLabelSegment::Line([9800, 9053]),
];

const BODY_CONTOUR_390: [HelpLabelSegment; 16] = [
    HelpLabelSegment::Quad {
        control: [10796, 9082],
        to: [10823, 9103],
    },
    HelpLabelSegment::Line([10768, 9182]),
    HelpLabelSegment::Quad {
        control: [10750, 9166],
        to: [10708, 9150],
    },
    HelpLabelSegment::Line([10623, 9135]),
    HelpLabelSegment::Quad {
        control: [10532, 9135],
        to: [10479, 9198],
    },
    HelpLabelSegment::Quad {
        control: [10426, 9262],
        to: [10426, 9373],
    },
    HelpLabelSegment::Quad {
        control: [10426, 9483],
        to: [10480, 9541],
    },
    HelpLabelSegment::Quad {
        control: [10535, 9598],
        to: [10631, 9598],
    },
    HelpLabelSegment::Quad {
        control: [10706, 9598],
        to: [10782, 9541],
    },
    HelpLabelSegment::Line([10827, 9634]),
    HelpLabelSegment::Quad {
        control: [10736, 9692],
        to: [10604, 9692],
    },
    HelpLabelSegment::Quad {
        control: [10476, 9692],
        to: [10392, 9606],
    },
    HelpLabelSegment::Quad {
        control: [10309, 9519],
        to: [10309, 9373],
    },
    HelpLabelSegment::Quad {
        control: [10309, 9223],
        to: [10395, 9132],
    },
    HelpLabelSegment::Quad {
        control: [10482, 9041],
        to: [10633, 9041],
    },
    HelpLabelSegment::Line([10739, 9061]),
];

const BODY_CONTOUR_391: [HelpLabelSegment; 16] = [
    HelpLabelSegment::Line([11056, 9122]),
    HelpLabelSegment::Quad {
        control: [11078, 9087],
        to: [11127, 9065],
    },
    HelpLabelSegment::Quad {
        control: [11177, 9041],
        to: [11229, 9041],
    },
    HelpLabelSegment::Quad {
        control: [11329, 9041],
        to: [11386, 9107],
    },
    HelpLabelSegment::Quad {
        control: [11443, 9173],
        to: [11443, 9286],
    },
    HelpLabelSegment::Line([11443, 9680]),
    HelpLabelSegment::Line([11331, 9680]),
    HelpLabelSegment::Line([11331, 9286]),
    HelpLabelSegment::Quad {
        control: [11331, 9216],
        to: [11296, 9175],
    },
    HelpLabelSegment::Quad {
        control: [11262, 9135],
        to: [11199, 9135],
    },
    HelpLabelSegment::Quad {
        control: [11159, 9135],
        to: [11118, 9159],
    },
    HelpLabelSegment::Quad {
        control: [11077, 9182],
        to: [11056, 9214],
    },
    HelpLabelSegment::Line([11056, 9680]),
    HelpLabelSegment::Line([10945, 9680]),
    HelpLabelSegment::Line([10945, 8795]),
    HelpLabelSegment::Line([11056, 8795]),
];

const BODY_CONTOUR_392: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Line([12000, 8922]),
    HelpLabelSegment::Line([12111, 8878]),
    HelpLabelSegment::Line([12111, 9053]),
    HelpLabelSegment::Line([12283, 9053]),
    HelpLabelSegment::Line([12283, 9141]),
    HelpLabelSegment::Line([12111, 9141]),
    HelpLabelSegment::Line([12111, 9453]),
    HelpLabelSegment::Quad {
        control: [12111, 9531],
        to: [12138, 9565],
    },
    HelpLabelSegment::Quad {
        control: [12164, 9598],
        to: [12223, 9598],
    },
    HelpLabelSegment::Quad {
        control: [12266, 9598],
        to: [12311, 9577],
    },
    HelpLabelSegment::Line([12328, 9674]),
    HelpLabelSegment::Line([12176, 9692]),
    HelpLabelSegment::Quad {
        control: [12101, 9692],
        to: [12051, 9637],
    },
    HelpLabelSegment::Quad {
        control: [12000, 9582],
        to: [12000, 9497],
    },
    HelpLabelSegment::Line([12000, 9141]),
    HelpLabelSegment::Line([11927, 9141]),
    HelpLabelSegment::Line([11927, 9053]),
    HelpLabelSegment::Line([12000, 9053]),
];

const BODY_CONTOUR_393: [HelpLabelSegment; 16] = [
    HelpLabelSegment::Line([12546, 9122]),
    HelpLabelSegment::Quad {
        control: [12568, 9087],
        to: [12617, 9065],
    },
    HelpLabelSegment::Quad {
        control: [12667, 9041],
        to: [12719, 9041],
    },
    HelpLabelSegment::Quad {
        control: [12819, 9041],
        to: [12876, 9107],
    },
    HelpLabelSegment::Quad {
        control: [12933, 9173],
        to: [12933, 9286],
    },
    HelpLabelSegment::Line([12933, 9680]),
    HelpLabelSegment::Line([12821, 9680]),
    HelpLabelSegment::Line([12821, 9286]),
    HelpLabelSegment::Quad {
        control: [12821, 9216],
        to: [12786, 9175],
    },
    HelpLabelSegment::Quad {
        control: [12752, 9135],
        to: [12689, 9135],
    },
    HelpLabelSegment::Quad {
        control: [12649, 9135],
        to: [12608, 9159],
    },
    HelpLabelSegment::Quad {
        control: [12567, 9182],
        to: [12546, 9214],
    },
    HelpLabelSegment::Line([12546, 9680]),
    HelpLabelSegment::Line([12435, 9680]),
    HelpLabelSegment::Line([12435, 8795]),
    HelpLabelSegment::Line([12546, 8795]),
];

const BODY_CONTOUR_394: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [13274, 9135],
        to: [13223, 9183],
    },
    HelpLabelSegment::Quad {
        control: [13175, 9229],
        to: [13168, 9297],
    },
    HelpLabelSegment::Line([13516, 9297]),
    HelpLabelSegment::Quad {
        control: [13516, 9229],
        to: [13474, 9184],
    },
    HelpLabelSegment::Quad {
        control: [13427, 9135],
        to: [13347, 9135],
    },
];

const BODY_CONTOUR_395: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [13280, 9598],
        to: [13363, 9598],
    },
    HelpLabelSegment::Quad {
        control: [13459, 9598],
        to: [13522, 9543],
    },
    HelpLabelSegment::Line([13569, 9623]),
    HelpLabelSegment::Quad {
        control: [13543, 9648],
        to: [13490, 9667],
    },
    HelpLabelSegment::Quad {
        control: [13424, 9692],
        to: [13342, 9692],
    },
    HelpLabelSegment::Quad {
        control: [13223, 9692],
        to: [13140, 9612],
    },
    HelpLabelSegment::Quad {
        control: [13049, 9523],
        to: [13049, 9374],
    },
    HelpLabelSegment::Quad {
        control: [13049, 9218],
        to: [13142, 9125],
    },
    HelpLabelSegment::Quad {
        control: [13227, 9041],
        to: [13343, 9041],
    },
    HelpLabelSegment::Quad {
        control: [13476, 9041],
        to: [13553, 9116],
    },
    HelpLabelSegment::Quad {
        control: [13626, 9189],
        to: [13626, 9310],
    },
    HelpLabelSegment::Quad {
        control: [13626, 9346],
        to: [13618, 9378],
    },
    HelpLabelSegment::Line([13166, 9378]),
    HelpLabelSegment::Quad {
        control: [13166, 9488],
        to: [13226, 9546],
    },
];

const BODY_CONTOUR_396: [HelpLabelSegment; 24] = [
    HelpLabelSegment::Quad {
        control: [14583, 9156],
        to: [14583, 9260],
    },
    HelpLabelSegment::Line([14583, 9680]),
    HelpLabelSegment::Line([14471, 9680]),
    HelpLabelSegment::Line([14471, 9283]),
    HelpLabelSegment::Quad {
        control: [14471, 9135],
        to: [14342, 9135],
    },
    HelpLabelSegment::Quad {
        control: [14302, 9135],
        to: [14267, 9160],
    },
    HelpLabelSegment::Quad {
        control: [14232, 9184],
        to: [14219, 9216],
    },
    HelpLabelSegment::Line([14219, 9680]),
    HelpLabelSegment::Line([14108, 9680]),
    HelpLabelSegment::Line([14108, 9235]),
    HelpLabelSegment::Quad {
        control: [14108, 9188],
        to: [14073, 9162],
    },
    HelpLabelSegment::Quad {
        control: [14038, 9135],
        to: [13980, 9135],
    },
    HelpLabelSegment::Quad {
        control: [13947, 9135],
        to: [13910, 9161],
    },
    HelpLabelSegment::Quad {
        control: [13871, 9187],
        to: [13856, 9217],
    },
    HelpLabelSegment::Line([13856, 9680]),
    HelpLabelSegment::Line([13745, 9680]),
    HelpLabelSegment::Line([13745, 9053]),
    HelpLabelSegment::Line([13817, 9053]),
    HelpLabelSegment::Line([13854, 9126]),
    HelpLabelSegment::Quad {
        control: [13918, 9041],
        to: [14015, 9041],
    },
    HelpLabelSegment::Quad {
        control: [14150, 9041],
        to: [14204, 9125],
    },
    HelpLabelSegment::Quad {
        control: [14223, 9089],
        to: [14273, 9065],
    },
    HelpLabelSegment::Quad {
        control: [14325, 9041],
        to: [14379, 9041],
    },
    HelpLabelSegment::Quad {
        control: [14476, 9041],
        to: [14529, 9099],
    },
];

const BODY_CONTOUR_397: [HelpLabelSegment; 10] = [
    HelpLabelSegment::Quad {
        control: [14962, 9689],
        to: [14930, 9753],
    },
    HelpLabelSegment::Quad {
        control: [14897, 9815],
        to: [14791, 9893],
    },
    HelpLabelSegment::Line([14762, 9851]),
    HelpLabelSegment::Quad {
        control: [14863, 9768],
        to: [14863, 9710],
    },
    HelpLabelSegment::Quad {
        control: [14863, 9685],
        to: [14845, 9659],
    },
    HelpLabelSegment::Quad {
        control: [14795, 9634],
        to: [14795, 9590],
    },
    HelpLabelSegment::Quad {
        control: [14795, 9558],
        to: [14818, 9538],
    },
    HelpLabelSegment::Quad {
        control: [14843, 9518],
        to: [14879, 9518],
    },
    HelpLabelSegment::Quad {
        control: [14911, 9518],
        to: [14937, 9546],
    },
    HelpLabelSegment::Quad {
        control: [14962, 9575],
        to: [14962, 9612],
    },
];

const BODY_CONTOUR_398: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Line([15580, 8922]),
    HelpLabelSegment::Line([15691, 8878]),
    HelpLabelSegment::Line([15691, 9053]),
    HelpLabelSegment::Line([15863, 9053]),
    HelpLabelSegment::Line([15863, 9141]),
    HelpLabelSegment::Line([15691, 9141]),
    HelpLabelSegment::Line([15691, 9453]),
    HelpLabelSegment::Quad {
        control: [15691, 9531],
        to: [15718, 9565],
    },
    HelpLabelSegment::Quad {
        control: [15744, 9598],
        to: [15803, 9598],
    },
    HelpLabelSegment::Quad {
        control: [15846, 9598],
        to: [15891, 9577],
    },
    HelpLabelSegment::Line([15908, 9674]),
    HelpLabelSegment::Line([15756, 9692]),
    HelpLabelSegment::Quad {
        control: [15681, 9692],
        to: [15631, 9637],
    },
    HelpLabelSegment::Quad {
        control: [15580, 9582],
        to: [15580, 9497],
    },
    HelpLabelSegment::Line([15580, 9141]),
    HelpLabelSegment::Line([15507, 9141]),
    HelpLabelSegment::Line([15507, 9053]),
    HelpLabelSegment::Line([15580, 9053]),
];

const BODY_CONTOUR_399: [HelpLabelSegment; 16] = [
    HelpLabelSegment::Line([16126, 9122]),
    HelpLabelSegment::Quad {
        control: [16148, 9087],
        to: [16197, 9065],
    },
    HelpLabelSegment::Quad {
        control: [16247, 9041],
        to: [16299, 9041],
    },
    HelpLabelSegment::Quad {
        control: [16399, 9041],
        to: [16456, 9107],
    },
    HelpLabelSegment::Quad {
        control: [16513, 9173],
        to: [16513, 9286],
    },
    HelpLabelSegment::Line([16513, 9680]),
    HelpLabelSegment::Line([16401, 9680]),
    HelpLabelSegment::Line([16401, 9286]),
    HelpLabelSegment::Quad {
        control: [16401, 9216],
        to: [16366, 9175],
    },
    HelpLabelSegment::Quad {
        control: [16332, 9135],
        to: [16269, 9135],
    },
    HelpLabelSegment::Quad {
        control: [16229, 9135],
        to: [16188, 9159],
    },
    HelpLabelSegment::Quad {
        control: [16147, 9182],
        to: [16126, 9214],
    },
    HelpLabelSegment::Line([16126, 9680]),
    HelpLabelSegment::Line([16015, 9680]),
    HelpLabelSegment::Line([16015, 8795]),
    HelpLabelSegment::Line([16126, 8795]),
];

const BODY_CONTOUR_400: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [16854, 9135],
        to: [16803, 9183],
    },
    HelpLabelSegment::Quad {
        control: [16755, 9229],
        to: [16748, 9297],
    },
    HelpLabelSegment::Line([17096, 9297]),
    HelpLabelSegment::Quad {
        control: [17096, 9229],
        to: [17054, 9184],
    },
    HelpLabelSegment::Quad {
        control: [17007, 9135],
        to: [16927, 9135],
    },
];

const BODY_CONTOUR_401: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [16860, 9598],
        to: [16943, 9598],
    },
    HelpLabelSegment::Quad {
        control: [17039, 9598],
        to: [17102, 9543],
    },
    HelpLabelSegment::Line([17149, 9623]),
    HelpLabelSegment::Quad {
        control: [17123, 9648],
        to: [17070, 9667],
    },
    HelpLabelSegment::Quad {
        control: [17004, 9692],
        to: [16922, 9692],
    },
    HelpLabelSegment::Quad {
        control: [16803, 9692],
        to: [16720, 9612],
    },
    HelpLabelSegment::Quad {
        control: [16629, 9523],
        to: [16629, 9374],
    },
    HelpLabelSegment::Quad {
        control: [16629, 9218],
        to: [16722, 9125],
    },
    HelpLabelSegment::Quad {
        control: [16807, 9041],
        to: [16923, 9041],
    },
    HelpLabelSegment::Quad {
        control: [17056, 9041],
        to: [17133, 9116],
    },
    HelpLabelSegment::Quad {
        control: [17206, 9189],
        to: [17206, 9310],
    },
    HelpLabelSegment::Quad {
        control: [17206, 9346],
        to: [17198, 9378],
    },
    HelpLabelSegment::Line([16746, 9378]),
    HelpLabelSegment::Quad {
        control: [16746, 9488],
        to: [16806, 9546],
    },
];

const BODY_CONTOUR_402: [HelpLabelSegment; 13] = [
    HelpLabelSegment::Line([17564, 9787]),
    HelpLabelSegment::Quad {
        control: [17543, 9846],
        to: [17474, 9886],
    },
    HelpLabelSegment::Quad {
        control: [17403, 9926],
        to: [17318, 9926],
    },
    HelpLabelSegment::Line([17318, 9826]),
    HelpLabelSegment::Quad {
        control: [17388, 9826],
        to: [17437, 9795],
    },
    HelpLabelSegment::Quad {
        control: [17488, 9762],
        to: [17488, 9715],
    },
    HelpLabelSegment::Quad {
        control: [17488, 9664],
        to: [17469, 9613],
    },
    HelpLabelSegment::Line([17422, 9489]),
    HelpLabelSegment::Line([17252, 9053]),
    HelpLabelSegment::Line([17366, 9053]),
    HelpLabelSegment::Line([17551, 9538]),
    HelpLabelSegment::Line([17716, 9053]),
    HelpLabelSegment::Line([17830, 9053]),
];

const BODY_CONTOUR_403: [HelpLabelSegment; 12] = [
    HelpLabelSegment::Line([18386, 9105]),
    HelpLabelSegment::Quad {
        control: [18449, 9041],
        to: [18538, 9041],
    },
    HelpLabelSegment::Quad {
        control: [18672, 9041],
        to: [18747, 9125],
    },
    HelpLabelSegment::Quad {
        control: [18821, 9208],
        to: [18821, 9368],
    },
    HelpLabelSegment::Quad {
        control: [18821, 9511],
        to: [18746, 9601],
    },
    HelpLabelSegment::Quad {
        control: [18671, 9692],
        to: [18529, 9692],
    },
    HelpLabelSegment::Quad {
        control: [18489, 9692],
        to: [18445, 9678],
    },
    HelpLabelSegment::Quad {
        control: [18399, 9664],
        to: [18386, 9646],
    },
    HelpLabelSegment::Line([18386, 9926]),
    HelpLabelSegment::Line([18275, 9926]),
    HelpLabelSegment::Line([18275, 9053]),
    HelpLabelSegment::Line([18386, 9053]),
];

const BODY_CONTOUR_404: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Line([18386, 9553]),
    HelpLabelSegment::Quad {
        control: [18397, 9570],
        to: [18431, 9584],
    },
    HelpLabelSegment::Quad {
        control: [18465, 9598],
        to: [18496, 9598],
    },
    HelpLabelSegment::Quad {
        control: [18704, 9598],
        to: [18704, 9364],
    },
    HelpLabelSegment::Quad {
        control: [18704, 9245],
        to: [18654, 9190],
    },
    HelpLabelSegment::Quad {
        control: [18605, 9135],
        to: [18497, 9135],
    },
    HelpLabelSegment::Quad {
        control: [18474, 9135],
        to: [18440, 9151],
    },
    HelpLabelSegment::Line([18386, 9188]),
];

const BODY_CONTOUR_405: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Quad {
        control: [19276, 9041],
        to: [19338, 9103],
    },
    HelpLabelSegment::Quad {
        control: [19399, 9166],
        to: [19399, 9300],
    },
    HelpLabelSegment::Line([19399, 9525]),
    HelpLabelSegment::Quad {
        control: [19399, 9609],
        to: [19449, 9635],
    },
    HelpLabelSegment::Line([19449, 9692]),
    HelpLabelSegment::Quad {
        control: [19381, 9692],
        to: [19348, 9672],
    },
    HelpLabelSegment::Quad {
        control: [19314, 9653],
        to: [19299, 9609],
    },
    HelpLabelSegment::Quad {
        control: [19232, 9692],
        to: [19095, 9692],
    },
    HelpLabelSegment::Quad {
        control: [19021, 9692],
        to: [18967, 9639],
    },
    HelpLabelSegment::Quad {
        control: [18912, 9585],
        to: [18912, 9505],
    },
    HelpLabelSegment::Quad {
        control: [18912, 9409],
        to: [18996, 9344],
    },
    HelpLabelSegment::Quad {
        control: [19079, 9278],
        to: [19208, 9278],
    },
    HelpLabelSegment::Line([19288, 9293]),
    HelpLabelSegment::Quad {
        control: [19288, 9141],
        to: [19152, 9141],
    },
    HelpLabelSegment::Quad {
        control: [19048, 9141],
        to: [18992, 9197],
    },
    HelpLabelSegment::Line([18945, 9103]),
    HelpLabelSegment::Quad {
        control: [18976, 9078],
        to: [19033, 9060],
    },
    HelpLabelSegment::Quad {
        control: [19089, 9041],
        to: [19139, 9041],
    },
];

const BODY_CONTOUR_406: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [19130, 9360],
        to: [19077, 9403],
    },
    HelpLabelSegment::Quad {
        control: [19023, 9447],
        to: [19023, 9507],
    },
    HelpLabelSegment::Quad {
        control: [19023, 9604],
        to: [19139, 9604],
    },
    HelpLabelSegment::Quad {
        control: [19224, 9604],
        to: [19288, 9524],
    },
    HelpLabelSegment::Line([19288, 9372]),
    HelpLabelSegment::Line([19214, 9360]),
];

const BODY_CONTOUR_407: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Quad {
        control: [19879, 9135],
        to: [19842, 9135],
    },
    HelpLabelSegment::Quad {
        control: [19783, 9135],
        to: [19739, 9189],
    },
    HelpLabelSegment::Quad {
        control: [19694, 9244],
        to: [19694, 9320],
    },
    HelpLabelSegment::Line([19694, 9680]),
    HelpLabelSegment::Line([19583, 9680]),
    HelpLabelSegment::Line([19583, 9053]),
    HelpLabelSegment::Line([19694, 9053]),
    HelpLabelSegment::Line([19694, 9153]),
    HelpLabelSegment::Quad {
        control: [19755, 9041],
        to: [19876, 9041],
    },
    HelpLabelSegment::Line([19961, 9052]),
    HelpLabelSegment::Line([19916, 9160]),
];

const BODY_CONTOUR_408: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Quad {
        control: [20371, 9041],
        to: [20433, 9103],
    },
    HelpLabelSegment::Quad {
        control: [20494, 9166],
        to: [20494, 9300],
    },
    HelpLabelSegment::Line([20494, 9525]),
    HelpLabelSegment::Quad {
        control: [20494, 9609],
        to: [20544, 9635],
    },
    HelpLabelSegment::Line([20544, 9692]),
    HelpLabelSegment::Quad {
        control: [20476, 9692],
        to: [20443, 9672],
    },
    HelpLabelSegment::Quad {
        control: [20409, 9653],
        to: [20394, 9609],
    },
    HelpLabelSegment::Quad {
        control: [20327, 9692],
        to: [20190, 9692],
    },
    HelpLabelSegment::Quad {
        control: [20116, 9692],
        to: [20062, 9639],
    },
    HelpLabelSegment::Quad {
        control: [20007, 9585],
        to: [20007, 9505],
    },
    HelpLabelSegment::Quad {
        control: [20007, 9409],
        to: [20091, 9344],
    },
    HelpLabelSegment::Quad {
        control: [20174, 9278],
        to: [20303, 9278],
    },
    HelpLabelSegment::Line([20383, 9293]),
    HelpLabelSegment::Quad {
        control: [20383, 9141],
        to: [20247, 9141],
    },
    HelpLabelSegment::Quad {
        control: [20143, 9141],
        to: [20087, 9197],
    },
    HelpLabelSegment::Line([20040, 9103]),
    HelpLabelSegment::Quad {
        control: [20071, 9078],
        to: [20128, 9060],
    },
    HelpLabelSegment::Quad {
        control: [20184, 9041],
        to: [20234, 9041],
    },
];

const BODY_CONTOUR_409: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [20225, 9360],
        to: [20172, 9403],
    },
    HelpLabelSegment::Quad {
        control: [20118, 9447],
        to: [20118, 9507],
    },
    HelpLabelSegment::Quad {
        control: [20118, 9604],
        to: [20234, 9604],
    },
    HelpLabelSegment::Quad {
        control: [20319, 9604],
        to: [20383, 9524],
    },
    HelpLabelSegment::Line([20383, 9372]),
    HelpLabelSegment::Line([20309, 9360]),
];

const BODY_CONTOUR_410: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([20895, 9692]),
    HelpLabelSegment::Quad {
        control: [20678, 9692],
        to: [20678, 9503],
    },
    HelpLabelSegment::Line([20678, 8795]),
    HelpLabelSegment::Line([20789, 8795]),
    HelpLabelSegment::Line([20789, 9484]),
    HelpLabelSegment::Quad {
        control: [20789, 9535],
        to: [20819, 9564],
    },
    HelpLabelSegment::Quad {
        control: [20848, 9592],
        to: [20895, 9592],
    },
];

const BODY_CONTOUR_411: [HelpLabelSegment; 13] = [
    HelpLabelSegment::Line([21264, 9787]),
    HelpLabelSegment::Quad {
        control: [21243, 9846],
        to: [21174, 9886],
    },
    HelpLabelSegment::Quad {
        control: [21103, 9926],
        to: [21018, 9926],
    },
    HelpLabelSegment::Line([21018, 9826]),
    HelpLabelSegment::Quad {
        control: [21088, 9826],
        to: [21137, 9795],
    },
    HelpLabelSegment::Quad {
        control: [21188, 9762],
        to: [21188, 9715],
    },
    HelpLabelSegment::Quad {
        control: [21188, 9664],
        to: [21169, 9613],
    },
    HelpLabelSegment::Line([21122, 9489]),
    HelpLabelSegment::Line([20952, 9053]),
    HelpLabelSegment::Line([21066, 9053]),
    HelpLabelSegment::Line([21251, 9538]),
    HelpLabelSegment::Line([21416, 9053]),
    HelpLabelSegment::Line([21530, 9053]),
];

const BODY_CONTOUR_412: [HelpLabelSegment; 10] = [
    HelpLabelSegment::Line([22081, 9580]),
    HelpLabelSegment::Line([22081, 9680]),
    HelpLabelSegment::Line([21558, 9680]),
    HelpLabelSegment::Line([21558, 9651]),
    HelpLabelSegment::Line([21916, 9153]),
    HelpLabelSegment::Line([21564, 9153]),
    HelpLabelSegment::Line([21564, 9053]),
    HelpLabelSegment::Line([22078, 9053]),
    HelpLabelSegment::Line([22078, 9085]),
    HelpLabelSegment::Line([21733, 9580]),
];

const BODY_CONTOUR_413: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [22369, 9135],
        to: [22318, 9183],
    },
    HelpLabelSegment::Quad {
        control: [22270, 9229],
        to: [22263, 9297],
    },
    HelpLabelSegment::Line([22611, 9297]),
    HelpLabelSegment::Quad {
        control: [22611, 9229],
        to: [22569, 9184],
    },
    HelpLabelSegment::Quad {
        control: [22522, 9135],
        to: [22442, 9135],
    },
];

const BODY_CONTOUR_414: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [22375, 9598],
        to: [22458, 9598],
    },
    HelpLabelSegment::Quad {
        control: [22554, 9598],
        to: [22617, 9543],
    },
    HelpLabelSegment::Line([22664, 9623]),
    HelpLabelSegment::Quad {
        control: [22638, 9648],
        to: [22585, 9667],
    },
    HelpLabelSegment::Quad {
        control: [22519, 9692],
        to: [22437, 9692],
    },
    HelpLabelSegment::Quad {
        control: [22318, 9692],
        to: [22235, 9612],
    },
    HelpLabelSegment::Quad {
        control: [22144, 9523],
        to: [22144, 9374],
    },
    HelpLabelSegment::Quad {
        control: [22144, 9218],
        to: [22237, 9125],
    },
    HelpLabelSegment::Quad {
        control: [22322, 9041],
        to: [22438, 9041],
    },
    HelpLabelSegment::Quad {
        control: [22571, 9041],
        to: [22648, 9116],
    },
    HelpLabelSegment::Quad {
        control: [22721, 9189],
        to: [22721, 9310],
    },
    HelpLabelSegment::Quad {
        control: [22721, 9346],
        to: [22713, 9378],
    },
    HelpLabelSegment::Line([22261, 9378]),
    HelpLabelSegment::Quad {
        control: [22261, 9488],
        to: [22321, 9546],
    },
];

const BODY_CONTOUR_415: [HelpLabelSegment; 13] = [
    HelpLabelSegment::Line([23439, 9787]),
    HelpLabelSegment::Quad {
        control: [23418, 9846],
        to: [23349, 9886],
    },
    HelpLabelSegment::Quad {
        control: [23278, 9926],
        to: [23193, 9926],
    },
    HelpLabelSegment::Line([23193, 9826]),
    HelpLabelSegment::Quad {
        control: [23263, 9826],
        to: [23312, 9795],
    },
    HelpLabelSegment::Quad {
        control: [23363, 9762],
        to: [23363, 9715],
    },
    HelpLabelSegment::Quad {
        control: [23363, 9664],
        to: [23344, 9613],
    },
    HelpLabelSegment::Line([23297, 9489]),
    HelpLabelSegment::Line([23127, 9053]),
    HelpLabelSegment::Line([23241, 9053]),
    HelpLabelSegment::Line([23426, 9538]),
    HelpLabelSegment::Line([23591, 9053]),
    HelpLabelSegment::Line([23705, 9053]),
];

const BODY_CONTOUR_416: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [23866, 9601],
        to: [24032, 9601],
    },
    HelpLabelSegment::Quad {
        control: [24111, 9601],
        to: [24155, 9538],
    },
    HelpLabelSegment::Quad {
        control: [24199, 9475],
        to: [24199, 9365],
    },
    HelpLabelSegment::Quad {
        control: [24199, 9132],
        to: [24032, 9132],
    },
    HelpLabelSegment::Quad {
        control: [23956, 9132],
        to: [23912, 9194],
    },
    HelpLabelSegment::Quad {
        control: [23866, 9256],
        to: [23866, 9365],
    },
];

const BODY_CONTOUR_417: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [23905, 9041],
        to: [24032, 9041],
    },
    HelpLabelSegment::Quad {
        control: [24167, 9041],
        to: [24242, 9127],
    },
    HelpLabelSegment::Quad {
        control: [24316, 9212],
        to: [24316, 9365],
    },
    HelpLabelSegment::Quad {
        control: [24316, 9517],
        to: [24240, 9605],
    },
    HelpLabelSegment::Quad {
        control: [24164, 9692],
        to: [24032, 9692],
    },
    HelpLabelSegment::Quad {
        control: [23899, 9692],
        to: [23824, 9604],
    },
    HelpLabelSegment::Quad {
        control: [23749, 9515],
        to: [23749, 9365],
    },
    HelpLabelSegment::Quad {
        control: [23749, 9219],
        to: [23827, 9130],
    },
];

const BODY_CONTOUR_418: [HelpLabelSegment; 15] = [
    HelpLabelSegment::Line([24540, 9453]),
    HelpLabelSegment::Quad {
        control: [24540, 9598],
        to: [24666, 9598],
    },
    HelpLabelSegment::Quad {
        control: [24721, 9598],
        to: [24766, 9566],
    },
    HelpLabelSegment::Quad {
        control: [24812, 9535],
        to: [24827, 9494],
    },
    HelpLabelSegment::Line([24827, 9053]),
    HelpLabelSegment::Line([24939, 9053]),
    HelpLabelSegment::Line([24939, 9680]),
    HelpLabelSegment::Line([24827, 9680]),
    HelpLabelSegment::Line([24827, 9593]),
    HelpLabelSegment::Quad {
        control: [24809, 9631],
        to: [24752, 9661],
    },
    HelpLabelSegment::Quad {
        control: [24695, 9692],
        to: [24641, 9692],
    },
    HelpLabelSegment::Quad {
        control: [24538, 9692],
        to: [24484, 9633],
    },
    HelpLabelSegment::Quad {
        control: [24429, 9573],
        to: [24429, 9464],
    },
    HelpLabelSegment::Line([24429, 9053]),
    HelpLabelSegment::Line([24540, 9053]),
];

const BODY_CONTOUR_419: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [25254, 9516],
        to: [25280, 9542],
    },
    HelpLabelSegment::Quad {
        control: [25305, 9568],
        to: [25305, 9604],
    },
    HelpLabelSegment::Quad {
        control: [25305, 9640],
        to: [25280, 9666],
    },
    HelpLabelSegment::Quad {
        control: [25254, 9692],
        to: [25217, 9692],
    },
    HelpLabelSegment::Quad {
        control: [25181, 9692],
        to: [25155, 9666],
    },
    HelpLabelSegment::Quad {
        control: [25130, 9640],
        to: [25130, 9604],
    },
    HelpLabelSegment::Quad {
        control: [25130, 9568],
        to: [25155, 9542],
    },
    HelpLabelSegment::Quad {
        control: [25181, 9516],
        to: [25217, 9516],
    },
];

const BODY_CONTOUR_420: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Quad {
        control: [-11030, 10590],
        to: [-11128, 10709],
    },
    HelpLabelSegment::Quad {
        control: [-11226, 10826],
        to: [-11237, 10890],
    },
    HelpLabelSegment::Quad {
        control: [-11187, 10843],
        to: [-11101, 10843],
    },
    HelpLabelSegment::Quad {
        control: [-10987, 10843],
        to: [-10919, 10916],
    },
    HelpLabelSegment::Quad {
        control: [-10850, 10989],
        to: [-10850, 11115],
    },
    HelpLabelSegment::Quad {
        control: [-10850, 11242],
        to: [-10920, 11318],
    },
    HelpLabelSegment::Quad {
        control: [-10992, 11395],
        to: [-11098, 11395],
    },
    HelpLabelSegment::Quad {
        control: [-11381, 11395],
        to: [-11381, 11020],
    },
    HelpLabelSegment::Quad {
        control: [-11381, 10864],
        to: [-11269, 10701],
    },
    HelpLabelSegment::Quad {
        control: [-11159, 10539],
        to: [-11036, 10507],
    },
    HelpLabelSegment::Line([-10979, 10569]),
];

const BODY_CONTOUR_421: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [-11258, 11296],
        to: [-11107, 11296],
    },
    HelpLabelSegment::Quad {
        control: [-11043, 11296],
        to: [-11007, 11248],
    },
    HelpLabelSegment::Quad {
        control: [-10971, 11200],
        to: [-10971, 11121],
    },
    HelpLabelSegment::Quad {
        control: [-10971, 11040],
        to: [-11009, 10991],
    },
    HelpLabelSegment::Quad {
        control: [-11048, 10943],
        to: [-11109, 10943],
    },
    HelpLabelSegment::Quad {
        control: [-11258, 10943],
        to: [-11258, 11112],
    },
];

const BODY_CONTOUR_422: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [-10556, 11216],
        to: [-10530, 11242],
    },
    HelpLabelSegment::Quad {
        control: [-10505, 11268],
        to: [-10505, 11304],
    },
    HelpLabelSegment::Quad {
        control: [-10505, 11340],
        to: [-10530, 11366],
    },
    HelpLabelSegment::Quad {
        control: [-10556, 11392],
        to: [-10593, 11392],
    },
    HelpLabelSegment::Quad {
        control: [-10629, 11392],
        to: [-10655, 11366],
    },
    HelpLabelSegment::Quad {
        control: [-10680, 11340],
        to: [-10680, 11304],
    },
    HelpLabelSegment::Quad {
        control: [-10680, 11268],
        to: [-10655, 11242],
    },
    HelpLabelSegment::Quad {
        control: [-10629, 11216],
        to: [-10593, 11216],
    },
];

const BODY_CONTOUR_423: [HelpLabelSegment; 4] = [
    HelpLabelSegment::Line([-9774, 11380]),
    HelpLabelSegment::Line([-9891, 11380]),
    HelpLabelSegment::Line([-9891, 10522]),
    HelpLabelSegment::Line([-9774, 10522]),
];

const BODY_CONTOUR_424: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Line([-9088, 11380]),
    HelpLabelSegment::Line([-9200, 11380]),
    HelpLabelSegment::Line([-9200, 11016]),
    HelpLabelSegment::Quad {
        control: [-9200, 10915],
        to: [-9229, 10875],
    },
    HelpLabelSegment::Quad {
        control: [-9260, 10835],
        to: [-9331, 10835],
    },
    HelpLabelSegment::Quad {
        control: [-9369, 10835],
        to: [-9411, 10857],
    },
    HelpLabelSegment::Quad {
        control: [-9452, 10881],
        to: [-9474, 10914],
    },
    HelpLabelSegment::Line([-9474, 11380]),
    HelpLabelSegment::Line([-9585, 11380]),
    HelpLabelSegment::Line([-9585, 10753]),
    HelpLabelSegment::Line([-9509, 10753]),
    HelpLabelSegment::Line([-9474, 10834]),
    HelpLabelSegment::Quad {
        control: [-9419, 10741],
        to: [-9295, 10741],
    },
    HelpLabelSegment::Quad {
        control: [-9088, 10741],
        to: [-9088, 10992],
    },
];

const BODY_CONTOUR_425: [HelpLabelSegment; 27] = [
    HelpLabelSegment::Quad {
        control: [-8461, 10815],
        to: [-8439, 10833],
    },
    HelpLabelSegment::Quad {
        control: [-8417, 10851],
        to: [-8349, 10884],
    },
    HelpLabelSegment::Line([-8279, 10917]),
    HelpLabelSegment::Quad {
        control: [-8191, 10959],
        to: [-8155, 11017],
    },
    HelpLabelSegment::Quad {
        control: [-8120, 11074],
        to: [-8120, 11163],
    },
    HelpLabelSegment::Quad {
        control: [-8120, 11260],
        to: [-8198, 11327],
    },
    HelpLabelSegment::Quad {
        control: [-8275, 11395],
        to: [-8405, 11395],
    },
    HelpLabelSegment::Quad {
        control: [-8520, 11395],
        to: [-8601, 11341],
    },
    HelpLabelSegment::Line([-8557, 11234]),
    HelpLabelSegment::Quad {
        control: [-8525, 11257],
        to: [-8475, 11273],
    },
    HelpLabelSegment::Quad {
        control: [-8427, 11290],
        to: [-8389, 11290],
    },
    HelpLabelSegment::Quad {
        control: [-8320, 11290],
        to: [-8279, 11252],
    },
    HelpLabelSegment::Quad {
        control: [-8236, 11215],
        to: [-8236, 11156],
    },
    HelpLabelSegment::Quad {
        control: [-8236, 11113],
        to: [-8260, 11076],
    },
    HelpLabelSegment::Quad {
        control: [-8283, 11039],
        to: [-8376, 10994],
    },
    HelpLabelSegment::Line([-8445, 10963]),
    HelpLabelSegment::Quad {
        control: [-8533, 10922],
        to: [-8568, 10866],
    },
    HelpLabelSegment::Quad {
        control: [-8603, 10809],
        to: [-8603, 10730],
    },
    HelpLabelSegment::Quad {
        control: [-8603, 10634],
        to: [-8535, 10570],
    },
    HelpLabelSegment::Quad {
        control: [-8467, 10507],
        to: [-8361, 10507],
    },
    HelpLabelSegment::Quad {
        control: [-8218, 10507],
        to: [-8162, 10554],
    },
    HelpLabelSegment::Line([-8196, 10656]),
    HelpLabelSegment::Quad {
        control: [-8220, 10638],
        to: [-8268, 10623],
    },
    HelpLabelSegment::Line([-8357, 10607]),
    HelpLabelSegment::Quad {
        control: [-8417, 10607],
        to: [-8451, 10641],
    },
    HelpLabelSegment::Quad {
        control: [-8486, 10675],
        to: [-8486, 10728],
    },
    HelpLabelSegment::Quad {
        control: [-8486, 10761],
        to: [-8473, 10788],
    },
];

const BODY_CONTOUR_426: [HelpLabelSegment; 12] = [
    HelpLabelSegment::Line([-7884, 10805]),
    HelpLabelSegment::Quad {
        control: [-7821, 10741],
        to: [-7732, 10741],
    },
    HelpLabelSegment::Quad {
        control: [-7598, 10741],
        to: [-7523, 10825],
    },
    HelpLabelSegment::Quad {
        control: [-7449, 10908],
        to: [-7449, 11068],
    },
    HelpLabelSegment::Quad {
        control: [-7449, 11211],
        to: [-7524, 11301],
    },
    HelpLabelSegment::Quad {
        control: [-7599, 11392],
        to: [-7741, 11392],
    },
    HelpLabelSegment::Quad {
        control: [-7781, 11392],
        to: [-7825, 11378],
    },
    HelpLabelSegment::Quad {
        control: [-7871, 11364],
        to: [-7884, 11346],
    },
    HelpLabelSegment::Line([-7884, 11626]),
    HelpLabelSegment::Line([-7995, 11626]),
    HelpLabelSegment::Line([-7995, 10753]),
    HelpLabelSegment::Line([-7884, 10753]),
];

const BODY_CONTOUR_427: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Line([-7884, 11253]),
    HelpLabelSegment::Quad {
        control: [-7873, 11270],
        to: [-7839, 11284],
    },
    HelpLabelSegment::Quad {
        control: [-7805, 11298],
        to: [-7774, 11298],
    },
    HelpLabelSegment::Quad {
        control: [-7566, 11298],
        to: [-7566, 11064],
    },
    HelpLabelSegment::Quad {
        control: [-7566, 10945],
        to: [-7616, 10890],
    },
    HelpLabelSegment::Quad {
        control: [-7665, 10835],
        to: [-7773, 10835],
    },
    HelpLabelSegment::Quad {
        control: [-7796, 10835],
        to: [-7830, 10851],
    },
    HelpLabelSegment::Line([-7884, 10888]),
];

const BODY_CONTOUR_428: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [-7141, 10835],
        to: [-7192, 10883],
    },
    HelpLabelSegment::Quad {
        control: [-7240, 10929],
        to: [-7247, 10997],
    },
    HelpLabelSegment::Line([-6899, 10997]),
    HelpLabelSegment::Quad {
        control: [-6899, 10929],
        to: [-6941, 10884],
    },
    HelpLabelSegment::Quad {
        control: [-6988, 10835],
        to: [-7068, 10835],
    },
];

const BODY_CONTOUR_429: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [-7135, 11298],
        to: [-7052, 11298],
    },
    HelpLabelSegment::Quad {
        control: [-6956, 11298],
        to: [-6893, 11243],
    },
    HelpLabelSegment::Line([-6846, 11323]),
    HelpLabelSegment::Quad {
        control: [-6872, 11348],
        to: [-6925, 11367],
    },
    HelpLabelSegment::Quad {
        control: [-6991, 11392],
        to: [-7073, 11392],
    },
    HelpLabelSegment::Quad {
        control: [-7192, 11392],
        to: [-7275, 11312],
    },
    HelpLabelSegment::Quad {
        control: [-7366, 11223],
        to: [-7366, 11074],
    },
    HelpLabelSegment::Quad {
        control: [-7366, 10918],
        to: [-7273, 10825],
    },
    HelpLabelSegment::Quad {
        control: [-7188, 10741],
        to: [-7072, 10741],
    },
    HelpLabelSegment::Quad {
        control: [-6939, 10741],
        to: [-6862, 10816],
    },
    HelpLabelSegment::Quad {
        control: [-6789, 10889],
        to: [-6789, 11010],
    },
    HelpLabelSegment::Quad {
        control: [-6789, 11046],
        to: [-6797, 11078],
    },
    HelpLabelSegment::Line([-7249, 11078]),
    HelpLabelSegment::Quad {
        control: [-7249, 11188],
        to: [-7189, 11246],
    },
];

const BODY_CONTOUR_430: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [-6486, 10835],
        to: [-6537, 10883],
    },
    HelpLabelSegment::Quad {
        control: [-6585, 10929],
        to: [-6592, 10997],
    },
    HelpLabelSegment::Line([-6244, 10997]),
    HelpLabelSegment::Quad {
        control: [-6244, 10929],
        to: [-6286, 10884],
    },
    HelpLabelSegment::Quad {
        control: [-6333, 10835],
        to: [-6412, 10835],
    },
];

const BODY_CONTOUR_431: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [-6480, 11298],
        to: [-6397, 11298],
    },
    HelpLabelSegment::Quad {
        control: [-6301, 11298],
        to: [-6238, 11243],
    },
    HelpLabelSegment::Line([-6191, 11323]),
    HelpLabelSegment::Quad {
        control: [-6217, 11348],
        to: [-6270, 11367],
    },
    HelpLabelSegment::Quad {
        control: [-6336, 11392],
        to: [-6418, 11392],
    },
    HelpLabelSegment::Quad {
        control: [-6537, 11392],
        to: [-6620, 11312],
    },
    HelpLabelSegment::Quad {
        control: [-6711, 11223],
        to: [-6711, 11074],
    },
    HelpLabelSegment::Quad {
        control: [-6711, 10918],
        to: [-6618, 10825],
    },
    HelpLabelSegment::Quad {
        control: [-6533, 10741],
        to: [-6417, 10741],
    },
    HelpLabelSegment::Quad {
        control: [-6284, 10741],
        to: [-6207, 10816],
    },
    HelpLabelSegment::Quad {
        control: [-6134, 10889],
        to: [-6134, 11010],
    },
    HelpLabelSegment::Quad {
        control: [-6134, 11046],
        to: [-6142, 11078],
    },
    HelpLabelSegment::Line([-6594, 11078]),
    HelpLabelSegment::Quad {
        control: [-6594, 11188],
        to: [-6534, 11246],
    },
];

const BODY_CONTOUR_432: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Line([-5507, 10495]),
    HelpLabelSegment::Line([-5507, 11380]),
    HelpLabelSegment::Line([-5618, 11380]),
    HelpLabelSegment::Line([-5618, 11333]),
    HelpLabelSegment::Quad {
        control: [-5675, 11392],
        to: [-5787, 11392],
    },
    HelpLabelSegment::Quad {
        control: [-5904, 11392],
        to: [-5978, 11307],
    },
    HelpLabelSegment::Quad {
        control: [-6050, 11223],
        to: [-6050, 11082],
    },
    HelpLabelSegment::Quad {
        control: [-6050, 10941],
        to: [-5966, 10841],
    },
    HelpLabelSegment::Quad {
        control: [-5882, 10741],
        to: [-5766, 10741],
    },
    HelpLabelSegment::Quad {
        control: [-5668, 10741],
        to: [-5618, 10787],
    },
    HelpLabelSegment::Line([-5618, 10495]),
];

const BODY_CONTOUR_433: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Quad {
        control: [-5627, 11265],
        to: [-5618, 11246],
    },
    HelpLabelSegment::Line([-5618, 10898]),
    HelpLabelSegment::Quad {
        control: [-5660, 10835],
        to: [-5733, 10835],
    },
    HelpLabelSegment::Quad {
        control: [-5823, 10835],
        to: [-5878, 10902],
    },
    HelpLabelSegment::Quad {
        control: [-5933, 10969],
        to: [-5933, 11072],
    },
    HelpLabelSegment::Quad {
        control: [-5933, 11298],
        to: [-5727, 11298],
    },
    HelpLabelSegment::Quad {
        control: [-5701, 11298],
        to: [-5664, 11282],
    },
];

const BODY_CONTOUR_434: [HelpLabelSegment; 19] = [
    HelpLabelSegment::Quad {
        control: [-4879, 10612],
        to: [-4964, 10612],
    },
    HelpLabelSegment::Quad {
        control: [-5099, 10612],
        to: [-5177, 10707],
    },
    HelpLabelSegment::Quad {
        control: [-5255, 10802],
        to: [-5255, 10959],
    },
    HelpLabelSegment::Quad {
        control: [-5255, 11108],
        to: [-5178, 11198],
    },
    HelpLabelSegment::Quad {
        control: [-5102, 11290],
        to: [-4970, 11290],
    },
    HelpLabelSegment::Quad {
        control: [-4877, 11290],
        to: [-4818, 11234],
    },
    HelpLabelSegment::Line([-4818, 11032]),
    HelpLabelSegment::Line([-4938, 11032]),
    HelpLabelSegment::Line([-4938, 10932]),
    HelpLabelSegment::Line([-4701, 10932]),
    HelpLabelSegment::Line([-4701, 11306]),
    HelpLabelSegment::Quad {
        control: [-4748, 11346],
        to: [-4832, 11371],
    },
    HelpLabelSegment::Quad {
        control: [-4915, 11395],
        to: [-4995, 11395],
    },
    HelpLabelSegment::Quad {
        control: [-5172, 11395],
        to: [-5275, 11275],
    },
    HelpLabelSegment::Quad {
        control: [-5378, 11154],
        to: [-5378, 10955],
    },
    HelpLabelSegment::Quad {
        control: [-5378, 10755],
        to: [-5264, 10631],
    },
    HelpLabelSegment::Quad {
        control: [-5152, 10507],
        to: [-4961, 10507],
    },
    HelpLabelSegment::Quad {
        control: [-4826, 10507],
        to: [-4742, 10582],
    },
    HelpLabelSegment::Line([-4791, 10678]),
];

const BODY_CONTOUR_435: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Quad {
        control: [-4231, 10835],
        to: [-4268, 10835],
    },
    HelpLabelSegment::Quad {
        control: [-4327, 10835],
        to: [-4371, 10889],
    },
    HelpLabelSegment::Quad {
        control: [-4416, 10944],
        to: [-4416, 11020],
    },
    HelpLabelSegment::Line([-4416, 11380]),
    HelpLabelSegment::Line([-4527, 11380]),
    HelpLabelSegment::Line([-4527, 10753]),
    HelpLabelSegment::Line([-4416, 10753]),
    HelpLabelSegment::Line([-4416, 10853]),
    HelpLabelSegment::Quad {
        control: [-4355, 10741],
        to: [-4234, 10741],
    },
    HelpLabelSegment::Line([-4149, 10752]),
    HelpLabelSegment::Line([-4194, 10860]),
];

const BODY_CONTOUR_436: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Quad {
        control: [-3739, 10741],
        to: [-3677, 10803],
    },
    HelpLabelSegment::Quad {
        control: [-3616, 10866],
        to: [-3616, 11000],
    },
    HelpLabelSegment::Line([-3616, 11225]),
    HelpLabelSegment::Quad {
        control: [-3616, 11309],
        to: [-3566, 11335],
    },
    HelpLabelSegment::Line([-3566, 11392]),
    HelpLabelSegment::Quad {
        control: [-3634, 11392],
        to: [-3667, 11372],
    },
    HelpLabelSegment::Quad {
        control: [-3701, 11353],
        to: [-3716, 11309],
    },
    HelpLabelSegment::Quad {
        control: [-3783, 11392],
        to: [-3920, 11392],
    },
    HelpLabelSegment::Quad {
        control: [-3994, 11392],
        to: [-4048, 11339],
    },
    HelpLabelSegment::Quad {
        control: [-4103, 11285],
        to: [-4103, 11205],
    },
    HelpLabelSegment::Quad {
        control: [-4103, 11109],
        to: [-4019, 11044],
    },
    HelpLabelSegment::Quad {
        control: [-3936, 10978],
        to: [-3807, 10978],
    },
    HelpLabelSegment::Line([-3727, 10993]),
    HelpLabelSegment::Quad {
        control: [-3727, 10841],
        to: [-3863, 10841],
    },
    HelpLabelSegment::Quad {
        control: [-3967, 10841],
        to: [-4023, 10897],
    },
    HelpLabelSegment::Line([-4070, 10803]),
    HelpLabelSegment::Quad {
        control: [-4039, 10778],
        to: [-3982, 10760],
    },
    HelpLabelSegment::Quad {
        control: [-3926, 10741],
        to: [-3876, 10741],
    },
];

const BODY_CONTOUR_437: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [-3885, 11060],
        to: [-3938, 11103],
    },
    HelpLabelSegment::Quad {
        control: [-3992, 11147],
        to: [-3992, 11207],
    },
    HelpLabelSegment::Quad {
        control: [-3992, 11304],
        to: [-3876, 11304],
    },
    HelpLabelSegment::Quad {
        control: [-3791, 11304],
        to: [-3727, 11224],
    },
    HelpLabelSegment::Line([-3727, 11072]),
    HelpLabelSegment::Line([-3801, 11060]),
];

const BODY_CONTOUR_438: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([-3214, 11392]),
    HelpLabelSegment::Line([-3243, 11392]),
    HelpLabelSegment::Line([-3513, 10751]),
    HelpLabelSegment::Line([-3391, 10751]),
    HelpLabelSegment::Line([-3225, 11190]),
    HelpLabelSegment::Line([-3056, 10751]),
    HelpLabelSegment::Line([-2939, 10751]),
];

const BODY_CONTOUR_439: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [-2707, 10515],
        to: [-2686, 10536],
    },
    HelpLabelSegment::Quad {
        control: [-2666, 10556],
        to: [-2666, 10584],
    },
    HelpLabelSegment::Quad {
        control: [-2666, 10612],
        to: [-2686, 10634],
    },
    HelpLabelSegment::Quad {
        control: [-2707, 10653],
        to: [-2735, 10653],
    },
    HelpLabelSegment::Quad {
        control: [-2764, 10653],
        to: [-2784, 10634],
    },
    HelpLabelSegment::Quad {
        control: [-2805, 10612],
        to: [-2805, 10584],
    },
    HelpLabelSegment::Quad {
        control: [-2805, 10555],
        to: [-2785, 10535],
    },
    HelpLabelSegment::Quad {
        control: [-2765, 10515],
        to: [-2735, 10515],
    },
];

const BODY_CONTOUR_440: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Line([-2685, 11380]),
    HelpLabelSegment::Line([-2796, 11380]),
    HelpLabelSegment::Line([-2796, 10847]),
    HelpLabelSegment::Line([-2883, 10847]),
    HelpLabelSegment::Line([-2883, 10753]),
    HelpLabelSegment::Line([-2685, 10753]),
];

const BODY_CONTOUR_441: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Line([-2470, 10622]),
    HelpLabelSegment::Line([-2359, 10578]),
    HelpLabelSegment::Line([-2359, 10753]),
    HelpLabelSegment::Line([-2187, 10753]),
    HelpLabelSegment::Line([-2187, 10841]),
    HelpLabelSegment::Line([-2359, 10841]),
    HelpLabelSegment::Line([-2359, 11153]),
    HelpLabelSegment::Quad {
        control: [-2359, 11231],
        to: [-2332, 11265],
    },
    HelpLabelSegment::Quad {
        control: [-2306, 11298],
        to: [-2247, 11298],
    },
    HelpLabelSegment::Quad {
        control: [-2204, 11298],
        to: [-2159, 11277],
    },
    HelpLabelSegment::Line([-2142, 11374]),
    HelpLabelSegment::Line([-2294, 11392]),
    HelpLabelSegment::Quad {
        control: [-2369, 11392],
        to: [-2419, 11337],
    },
    HelpLabelSegment::Quad {
        control: [-2470, 11282],
        to: [-2470, 11197],
    },
    HelpLabelSegment::Line([-2470, 10841]),
    HelpLabelSegment::Line([-2543, 10841]),
    HelpLabelSegment::Line([-2543, 10753]),
    HelpLabelSegment::Line([-2470, 10753]),
];

const BODY_CONTOUR_442: [HelpLabelSegment; 13] = [
    HelpLabelSegment::Line([-1796, 11487]),
    HelpLabelSegment::Quad {
        control: [-1817, 11546],
        to: [-1886, 11586],
    },
    HelpLabelSegment::Quad {
        control: [-1957, 11626],
        to: [-2042, 11626],
    },
    HelpLabelSegment::Line([-2042, 11526]),
    HelpLabelSegment::Quad {
        control: [-1972, 11526],
        to: [-1923, 11495],
    },
    HelpLabelSegment::Quad {
        control: [-1872, 11462],
        to: [-1872, 11415],
    },
    HelpLabelSegment::Quad {
        control: [-1872, 11364],
        to: [-1891, 11313],
    },
    HelpLabelSegment::Line([-1938, 11189]),
    HelpLabelSegment::Line([-2108, 10753]),
    HelpLabelSegment::Line([-1994, 10753]),
    HelpLabelSegment::Line([-1809, 11238]),
    HelpLabelSegment::Line([-1644, 10753]),
    HelpLabelSegment::Line([-1530, 10753]),
];

const BODY_CONTOUR_443: [HelpLabelSegment; 4] = [
    HelpLabelSegment::Line([-1431, 10982]),
    HelpLabelSegment::Line([-1180, 10982]),
    HelpLabelSegment::Line([-1180, 11084]),
    HelpLabelSegment::Line([-1431, 11084]),
];

const BODY_CONTOUR_444: [HelpLabelSegment; 13] = [
    HelpLabelSegment::Line([-239, 11380]),
    HelpLabelSegment::Line([-350, 11380]),
    HelpLabelSegment::Line([-452, 10829]),
    HelpLabelSegment::Line([-640, 11392]),
    HelpLabelSegment::Line([-669, 11392]),
    HelpLabelSegment::Line([-862, 10829]),
    HelpLabelSegment::Line([-962, 11380]),
    HelpLabelSegment::Line([-1073, 11380]),
    HelpLabelSegment::Line([-913, 10522]),
    HelpLabelSegment::Line([-859, 10522]),
    HelpLabelSegment::Line([-655, 11148]),
    HelpLabelSegment::Line([-466, 10522]),
    HelpLabelSegment::Line([-414, 10522]),
];

const BODY_CONTOUR_445: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [-79, 11301],
        to: [87, 11301],
    },
    HelpLabelSegment::Quad {
        control: [166, 11301],
        to: [210, 11238],
    },
    HelpLabelSegment::Quad {
        control: [254, 11175],
        to: [254, 11065],
    },
    HelpLabelSegment::Quad {
        control: [254, 10832],
        to: [87, 10832],
    },
    HelpLabelSegment::Quad {
        control: [11, 10832],
        to: [-33, 10894],
    },
    HelpLabelSegment::Quad {
        control: [-79, 10956],
        to: [-79, 11065],
    },
];

const BODY_CONTOUR_446: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [-40, 10741],
        to: [87, 10741],
    },
    HelpLabelSegment::Quad {
        control: [222, 10741],
        to: [297, 10827],
    },
    HelpLabelSegment::Quad {
        control: [371, 10912],
        to: [371, 11065],
    },
    HelpLabelSegment::Quad {
        control: [371, 11217],
        to: [295, 11305],
    },
    HelpLabelSegment::Quad {
        control: [219, 11392],
        to: [87, 11392],
    },
    HelpLabelSegment::Quad {
        control: [-46, 11392],
        to: [-121, 11304],
    },
    HelpLabelSegment::Quad {
        control: [-196, 11215],
        to: [-196, 11065],
    },
    HelpLabelSegment::Quad {
        control: [-196, 10919],
        to: [-118, 10830],
    },
];

const BODY_CONTOUR_447: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Line([998, 10495]),
    HelpLabelSegment::Line([998, 11380]),
    HelpLabelSegment::Line([887, 11380]),
    HelpLabelSegment::Line([887, 11333]),
    HelpLabelSegment::Quad {
        control: [830, 11392],
        to: [718, 11392],
    },
    HelpLabelSegment::Quad {
        control: [601, 11392],
        to: [527, 11307],
    },
    HelpLabelSegment::Quad {
        control: [455, 11223],
        to: [455, 11082],
    },
    HelpLabelSegment::Quad {
        control: [455, 10941],
        to: [539, 10841],
    },
    HelpLabelSegment::Quad {
        control: [623, 10741],
        to: [739, 10741],
    },
    HelpLabelSegment::Quad {
        control: [837, 10741],
        to: [887, 10787],
    },
    HelpLabelSegment::Line([887, 10495]),
];

const BODY_CONTOUR_448: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Quad {
        control: [878, 11265],
        to: [887, 11246],
    },
    HelpLabelSegment::Line([887, 10898]),
    HelpLabelSegment::Quad {
        control: [845, 10835],
        to: [772, 10835],
    },
    HelpLabelSegment::Quad {
        control: [682, 10835],
        to: [627, 10902],
    },
    HelpLabelSegment::Quad {
        control: [572, 10969],
        to: [572, 11072],
    },
    HelpLabelSegment::Quad {
        control: [572, 11298],
        to: [778, 11298],
    },
    HelpLabelSegment::Quad {
        control: [804, 11298],
        to: [841, 11282],
    },
];

const BODY_CONTOUR_449: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [1344, 10835],
        to: [1293, 10883],
    },
    HelpLabelSegment::Quad {
        control: [1245, 10929],
        to: [1238, 10997],
    },
    HelpLabelSegment::Line([1586, 10997]),
    HelpLabelSegment::Quad {
        control: [1586, 10929],
        to: [1544, 10884],
    },
    HelpLabelSegment::Quad {
        control: [1497, 10835],
        to: [1418, 10835],
    },
];

const BODY_CONTOUR_450: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [1350, 11298],
        to: [1433, 11298],
    },
    HelpLabelSegment::Quad {
        control: [1529, 11298],
        to: [1592, 11243],
    },
    HelpLabelSegment::Line([1639, 11323]),
    HelpLabelSegment::Quad {
        control: [1613, 11348],
        to: [1560, 11367],
    },
    HelpLabelSegment::Quad {
        control: [1494, 11392],
        to: [1412, 11392],
    },
    HelpLabelSegment::Quad {
        control: [1293, 11392],
        to: [1210, 11312],
    },
    HelpLabelSegment::Quad {
        control: [1119, 11223],
        to: [1119, 11074],
    },
    HelpLabelSegment::Quad {
        control: [1119, 10918],
        to: [1212, 10825],
    },
    HelpLabelSegment::Quad {
        control: [1297, 10741],
        to: [1413, 10741],
    },
    HelpLabelSegment::Quad {
        control: [1546, 10741],
        to: [1623, 10816],
    },
    HelpLabelSegment::Quad {
        control: [1696, 10889],
        to: [1696, 11010],
    },
    HelpLabelSegment::Quad {
        control: [1696, 11046],
        to: [1688, 11078],
    },
    HelpLabelSegment::Line([1236, 11078]),
    HelpLabelSegment::Quad {
        control: [1236, 11188],
        to: [1296, 11246],
    },
];

const BODY_CONTOUR_451: [HelpLabelSegment; 17] = [
    HelpLabelSegment::Quad {
        control: [2225, 10632],
        to: [2284, 10564],
    },
    HelpLabelSegment::Quad {
        control: [2343, 10495],
        to: [2441, 10495],
    },
    HelpLabelSegment::Line([2545, 10513]),
    HelpLabelSegment::Line([2513, 10595]),
    HelpLabelSegment::Line([2450, 10583]),
    HelpLabelSegment::Quad {
        control: [2400, 10583],
        to: [2367, 10622],
    },
    HelpLabelSegment::Quad {
        control: [2333, 10660],
        to: [2333, 10720],
    },
    HelpLabelSegment::Quad {
        control: [2333, 10735],
        to: [2336, 10753],
    },
    HelpLabelSegment::Line([2464, 10753]),
    HelpLabelSegment::Line([2464, 10847]),
    HelpLabelSegment::Line([2336, 10847]),
    HelpLabelSegment::Line([2336, 11380]),
    HelpLabelSegment::Line([2225, 11380]),
    HelpLabelSegment::Line([2225, 10847]),
    HelpLabelSegment::Line([2134, 10847]),
    HelpLabelSegment::Line([2134, 10753]),
    HelpLabelSegment::Line([2225, 10753]),
];

const BODY_CONTOUR_452: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Quad {
        control: [2951, 10741],
        to: [3013, 10803],
    },
    HelpLabelSegment::Quad {
        control: [3074, 10866],
        to: [3074, 11000],
    },
    HelpLabelSegment::Line([3074, 11225]),
    HelpLabelSegment::Quad {
        control: [3074, 11309],
        to: [3124, 11335],
    },
    HelpLabelSegment::Line([3124, 11392]),
    HelpLabelSegment::Quad {
        control: [3056, 11392],
        to: [3023, 11372],
    },
    HelpLabelSegment::Quad {
        control: [2989, 11353],
        to: [2974, 11309],
    },
    HelpLabelSegment::Quad {
        control: [2907, 11392],
        to: [2770, 11392],
    },
    HelpLabelSegment::Quad {
        control: [2696, 11392],
        to: [2642, 11339],
    },
    HelpLabelSegment::Quad {
        control: [2587, 11285],
        to: [2587, 11205],
    },
    HelpLabelSegment::Quad {
        control: [2587, 11109],
        to: [2671, 11044],
    },
    HelpLabelSegment::Quad {
        control: [2754, 10978],
        to: [2883, 10978],
    },
    HelpLabelSegment::Line([2963, 10993]),
    HelpLabelSegment::Quad {
        control: [2963, 10841],
        to: [2827, 10841],
    },
    HelpLabelSegment::Quad {
        control: [2723, 10841],
        to: [2667, 10897],
    },
    HelpLabelSegment::Line([2620, 10803]),
    HelpLabelSegment::Quad {
        control: [2651, 10778],
        to: [2708, 10760],
    },
    HelpLabelSegment::Quad {
        control: [2764, 10741],
        to: [2814, 10741],
    },
];

const BODY_CONTOUR_453: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [2805, 11060],
        to: [2752, 11103],
    },
    HelpLabelSegment::Quad {
        control: [2698, 11147],
        to: [2698, 11207],
    },
    HelpLabelSegment::Quad {
        control: [2698, 11304],
        to: [2814, 11304],
    },
    HelpLabelSegment::Quad {
        control: [2899, 11304],
        to: [2963, 11224],
    },
    HelpLabelSegment::Line([2963, 11072]),
    HelpLabelSegment::Line([2889, 11060]),
];

const BODY_CONTOUR_454: [HelpLabelSegment; 22] = [
    HelpLabelSegment::Line([3557, 10888]),
    HelpLabelSegment::Quad {
        control: [3491, 10835],
        to: [3424, 10835],
    },
    HelpLabelSegment::Quad {
        control: [3384, 10835],
        to: [3358, 10854],
    },
    HelpLabelSegment::Quad {
        control: [3329, 10873],
        to: [3329, 10901],
    },
    HelpLabelSegment::Quad {
        control: [3329, 10962],
        to: [3399, 10992],
    },
    HelpLabelSegment::Line([3478, 11028]),
    HelpLabelSegment::Quad {
        control: [3551, 11062],
        to: [3585, 11105],
    },
    HelpLabelSegment::Quad {
        control: [3618, 11148],
        to: [3618, 11212],
    },
    HelpLabelSegment::Quad {
        control: [3618, 11297],
        to: [3559, 11345],
    },
    HelpLabelSegment::Quad {
        control: [3499, 11392],
        to: [3395, 11392],
    },
    HelpLabelSegment::Quad {
        control: [3295, 11392],
        to: [3209, 11342],
    },
    HelpLabelSegment::Line([3247, 11237]),
    HelpLabelSegment::Quad {
        control: [3341, 11298],
        to: [3397, 11298],
    },
    HelpLabelSegment::Quad {
        control: [3500, 11298],
        to: [3500, 11211],
    },
    HelpLabelSegment::Quad {
        control: [3500, 11149],
        to: [3401, 11105],
    },
    HelpLabelSegment::Quad {
        control: [3325, 11069],
        to: [3298, 11052],
    },
    HelpLabelSegment::Quad {
        control: [3271, 11033],
        to: [3252, 11011],
    },
    HelpLabelSegment::Quad {
        control: [3232, 10987],
        to: [3223, 10962],
    },
    HelpLabelSegment::Quad {
        control: [3212, 10935],
        to: [3212, 10905],
    },
    HelpLabelSegment::Quad {
        control: [3212, 10828],
        to: [3268, 10785],
    },
    HelpLabelSegment::Quad {
        control: [3325, 10741],
        to: [3416, 10741],
    },
    HelpLabelSegment::Quad {
        control: [3484, 10741],
        to: [3588, 10785],
    },
];

const BODY_CONTOUR_455: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Line([3775, 10622]),
    HelpLabelSegment::Line([3886, 10578]),
    HelpLabelSegment::Line([3886, 10753]),
    HelpLabelSegment::Line([4058, 10753]),
    HelpLabelSegment::Line([4058, 10841]),
    HelpLabelSegment::Line([3886, 10841]),
    HelpLabelSegment::Line([3886, 11153]),
    HelpLabelSegment::Quad {
        control: [3886, 11231],
        to: [3913, 11265],
    },
    HelpLabelSegment::Quad {
        control: [3939, 11298],
        to: [3998, 11298],
    },
    HelpLabelSegment::Quad {
        control: [4041, 11298],
        to: [4086, 11277],
    },
    HelpLabelSegment::Line([4103, 11374]),
    HelpLabelSegment::Line([3951, 11392]),
    HelpLabelSegment::Quad {
        control: [3876, 11392],
        to: [3826, 11337],
    },
    HelpLabelSegment::Quad {
        control: [3775, 11282],
        to: [3775, 11197],
    },
    HelpLabelSegment::Line([3775, 10841]),
    HelpLabelSegment::Line([3702, 10841]),
    HelpLabelSegment::Line([3702, 10753]),
    HelpLabelSegment::Line([3775, 10753]),
];

const BODY_CONTOUR_456: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [4394, 10835],
        to: [4343, 10883],
    },
    HelpLabelSegment::Quad {
        control: [4295, 10929],
        to: [4288, 10997],
    },
    HelpLabelSegment::Line([4636, 10997]),
    HelpLabelSegment::Quad {
        control: [4636, 10929],
        to: [4594, 10884],
    },
    HelpLabelSegment::Quad {
        control: [4547, 10835],
        to: [4468, 10835],
    },
];

const BODY_CONTOUR_457: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [4400, 11298],
        to: [4483, 11298],
    },
    HelpLabelSegment::Quad {
        control: [4579, 11298],
        to: [4642, 11243],
    },
    HelpLabelSegment::Line([4689, 11323]),
    HelpLabelSegment::Quad {
        control: [4663, 11348],
        to: [4610, 11367],
    },
    HelpLabelSegment::Quad {
        control: [4544, 11392],
        to: [4462, 11392],
    },
    HelpLabelSegment::Quad {
        control: [4343, 11392],
        to: [4260, 11312],
    },
    HelpLabelSegment::Quad {
        control: [4169, 11223],
        to: [4169, 11074],
    },
    HelpLabelSegment::Quad {
        control: [4169, 10918],
        to: [4262, 10825],
    },
    HelpLabelSegment::Quad {
        control: [4347, 10741],
        to: [4463, 10741],
    },
    HelpLabelSegment::Quad {
        control: [4596, 10741],
        to: [4673, 10816],
    },
    HelpLabelSegment::Quad {
        control: [4746, 10889],
        to: [4746, 11010],
    },
    HelpLabelSegment::Quad {
        control: [4746, 11046],
        to: [4738, 11078],
    },
    HelpLabelSegment::Line([4286, 11078]),
    HelpLabelSegment::Quad {
        control: [4286, 11188],
        to: [4346, 11246],
    },
];

const BODY_CONTOUR_458: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Quad {
        control: [5169, 10835],
        to: [5132, 10835],
    },
    HelpLabelSegment::Quad {
        control: [5073, 10835],
        to: [5029, 10889],
    },
    HelpLabelSegment::Quad {
        control: [4984, 10944],
        to: [4984, 11020],
    },
    HelpLabelSegment::Line([4984, 11380]),
    HelpLabelSegment::Line([4873, 11380]),
    HelpLabelSegment::Line([4873, 10753]),
    HelpLabelSegment::Line([4984, 10753]),
    HelpLabelSegment::Line([4984, 10853]),
    HelpLabelSegment::Quad {
        control: [5045, 10741],
        to: [5166, 10741],
    },
    HelpLabelSegment::Line([5251, 10752]),
    HelpLabelSegment::Line([5206, 10860]),
];

const BODY_CONTOUR_459: [HelpLabelSegment; 13] = [
    HelpLabelSegment::Line([5801, 10799]),
    HelpLabelSegment::Quad {
        control: [5816, 10778],
        to: [5861, 10759],
    },
    HelpLabelSegment::Quad {
        control: [5904, 10741],
        to: [5946, 10741],
    },
    HelpLabelSegment::Quad {
        control: [6075, 10741],
        to: [6155, 10830],
    },
    HelpLabelSegment::Quad {
        control: [6235, 10919],
        to: [6235, 11055],
    },
    HelpLabelSegment::Quad {
        control: [6235, 11212],
        to: [6155, 11303],
    },
    HelpLabelSegment::Quad {
        control: [6074, 11392],
        to: [5937, 11392],
    },
    HelpLabelSegment::Quad {
        control: [5892, 11392],
        to: [5850, 11375],
    },
    HelpLabelSegment::Quad {
        control: [5807, 11359],
        to: [5785, 11335],
    },
    HelpLabelSegment::Line([5745, 11392]),
    HelpLabelSegment::Line([5690, 11392]),
    HelpLabelSegment::Line([5690, 10495]),
    HelpLabelSegment::Line([5801, 10495]),
];

const BODY_CONTOUR_460: [HelpLabelSegment; 9] = [
    HelpLabelSegment::Line([5801, 11246]),
    HelpLabelSegment::Quad {
        control: [5801, 11256],
        to: [5842, 11277],
    },
    HelpLabelSegment::Quad {
        control: [5884, 11298],
        to: [5905, 11298],
    },
    HelpLabelSegment::Quad {
        control: [6019, 11298],
        to: [6068, 11244],
    },
    HelpLabelSegment::Quad {
        control: [6117, 11189],
        to: [6117, 11061],
    },
    HelpLabelSegment::Quad {
        control: [6117, 10955],
        to: [6060, 10895],
    },
    HelpLabelSegment::Quad {
        control: [6003, 10835],
        to: [5905, 10835],
    },
    HelpLabelSegment::Quad {
        control: [5885, 10835],
        to: [5849, 10853],
    },
    HelpLabelSegment::Line([5801, 10884]),
];

const BODY_CONTOUR_461: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Quad {
        control: [6691, 10741],
        to: [6753, 10803],
    },
    HelpLabelSegment::Quad {
        control: [6814, 10866],
        to: [6814, 11000],
    },
    HelpLabelSegment::Line([6814, 11225]),
    HelpLabelSegment::Quad {
        control: [6814, 11309],
        to: [6864, 11335],
    },
    HelpLabelSegment::Line([6864, 11392]),
    HelpLabelSegment::Quad {
        control: [6796, 11392],
        to: [6763, 11372],
    },
    HelpLabelSegment::Quad {
        control: [6729, 11353],
        to: [6714, 11309],
    },
    HelpLabelSegment::Quad {
        control: [6647, 11392],
        to: [6510, 11392],
    },
    HelpLabelSegment::Quad {
        control: [6436, 11392],
        to: [6382, 11339],
    },
    HelpLabelSegment::Quad {
        control: [6327, 11285],
        to: [6327, 11205],
    },
    HelpLabelSegment::Quad {
        control: [6327, 11109],
        to: [6411, 11044],
    },
    HelpLabelSegment::Quad {
        control: [6494, 10978],
        to: [6623, 10978],
    },
    HelpLabelSegment::Line([6703, 10993]),
    HelpLabelSegment::Quad {
        control: [6703, 10841],
        to: [6567, 10841],
    },
    HelpLabelSegment::Quad {
        control: [6463, 10841],
        to: [6407, 10897],
    },
    HelpLabelSegment::Line([6360, 10803]),
    HelpLabelSegment::Quad {
        control: [6391, 10778],
        to: [6448, 10760],
    },
    HelpLabelSegment::Quad {
        control: [6504, 10741],
        to: [6554, 10741],
    },
];

const BODY_CONTOUR_462: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [6545, 11060],
        to: [6492, 11103],
    },
    HelpLabelSegment::Quad {
        control: [6438, 11147],
        to: [6438, 11207],
    },
    HelpLabelSegment::Quad {
        control: [6438, 11304],
        to: [6554, 11304],
    },
    HelpLabelSegment::Quad {
        control: [6639, 11304],
        to: [6703, 11224],
    },
    HelpLabelSegment::Line([6703, 11072]),
    HelpLabelSegment::Line([6629, 11060]),
];

const BODY_CONTOUR_463: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([7215, 11392]),
    HelpLabelSegment::Quad {
        control: [6998, 11392],
        to: [6998, 11203],
    },
    HelpLabelSegment::Line([6998, 10495]),
    HelpLabelSegment::Line([7109, 10495]),
    HelpLabelSegment::Line([7109, 11184]),
    HelpLabelSegment::Quad {
        control: [7109, 11235],
        to: [7139, 11264],
    },
    HelpLabelSegment::Quad {
        control: [7168, 11292],
        to: [7215, 11292],
    },
];

const BODY_CONTOUR_464: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([7570, 11392]),
    HelpLabelSegment::Quad {
        control: [7353, 11392],
        to: [7353, 11203],
    },
    HelpLabelSegment::Line([7353, 10495]),
    HelpLabelSegment::Line([7464, 10495]),
    HelpLabelSegment::Line([7464, 11184]),
    HelpLabelSegment::Quad {
        control: [7464, 11235],
        to: [7494, 11264],
    },
    HelpLabelSegment::Quad {
        control: [7523, 11292],
        to: [7570, 11292],
    },
];

const BODY_CONTOUR_465: [HelpLabelSegment; 22] = [
    HelpLabelSegment::Line([8007, 10888]),
    HelpLabelSegment::Quad {
        control: [7941, 10835],
        to: [7874, 10835],
    },
    HelpLabelSegment::Quad {
        control: [7834, 10835],
        to: [7808, 10854],
    },
    HelpLabelSegment::Quad {
        control: [7779, 10873],
        to: [7779, 10901],
    },
    HelpLabelSegment::Quad {
        control: [7779, 10962],
        to: [7849, 10992],
    },
    HelpLabelSegment::Line([7928, 11028]),
    HelpLabelSegment::Quad {
        control: [8001, 11062],
        to: [8035, 11105],
    },
    HelpLabelSegment::Quad {
        control: [8068, 11148],
        to: [8068, 11212],
    },
    HelpLabelSegment::Quad {
        control: [8068, 11297],
        to: [8009, 11345],
    },
    HelpLabelSegment::Quad {
        control: [7949, 11392],
        to: [7845, 11392],
    },
    HelpLabelSegment::Quad {
        control: [7745, 11392],
        to: [7659, 11342],
    },
    HelpLabelSegment::Line([7697, 11237]),
    HelpLabelSegment::Quad {
        control: [7791, 11298],
        to: [7847, 11298],
    },
    HelpLabelSegment::Quad {
        control: [7950, 11298],
        to: [7950, 11211],
    },
    HelpLabelSegment::Quad {
        control: [7950, 11149],
        to: [7851, 11105],
    },
    HelpLabelSegment::Quad {
        control: [7775, 11069],
        to: [7748, 11052],
    },
    HelpLabelSegment::Quad {
        control: [7721, 11033],
        to: [7702, 11011],
    },
    HelpLabelSegment::Quad {
        control: [7682, 10987],
        to: [7673, 10962],
    },
    HelpLabelSegment::Quad {
        control: [7662, 10935],
        to: [7662, 10905],
    },
    HelpLabelSegment::Quad {
        control: [7662, 10828],
        to: [7718, 10785],
    },
    HelpLabelSegment::Quad {
        control: [7775, 10741],
        to: [7866, 10741],
    },
    HelpLabelSegment::Quad {
        control: [7934, 10741],
        to: [8038, 10785],
    },
];

const BODY_CONTOUR_466: [HelpLabelSegment; 22] = [
    HelpLabelSegment::Line([8852, 10888]),
    HelpLabelSegment::Quad {
        control: [8786, 10835],
        to: [8719, 10835],
    },
    HelpLabelSegment::Quad {
        control: [8679, 10835],
        to: [8653, 10854],
    },
    HelpLabelSegment::Quad {
        control: [8624, 10873],
        to: [8624, 10901],
    },
    HelpLabelSegment::Quad {
        control: [8624, 10962],
        to: [8694, 10992],
    },
    HelpLabelSegment::Line([8773, 11028]),
    HelpLabelSegment::Quad {
        control: [8846, 11062],
        to: [8880, 11105],
    },
    HelpLabelSegment::Quad {
        control: [8913, 11148],
        to: [8913, 11212],
    },
    HelpLabelSegment::Quad {
        control: [8913, 11297],
        to: [8854, 11345],
    },
    HelpLabelSegment::Quad {
        control: [8794, 11392],
        to: [8690, 11392],
    },
    HelpLabelSegment::Quad {
        control: [8590, 11392],
        to: [8504, 11342],
    },
    HelpLabelSegment::Line([8542, 11237]),
    HelpLabelSegment::Quad {
        control: [8636, 11298],
        to: [8692, 11298],
    },
    HelpLabelSegment::Quad {
        control: [8795, 11298],
        to: [8795, 11211],
    },
    HelpLabelSegment::Quad {
        control: [8795, 11149],
        to: [8696, 11105],
    },
    HelpLabelSegment::Quad {
        control: [8620, 11069],
        to: [8593, 11052],
    },
    HelpLabelSegment::Quad {
        control: [8566, 11033],
        to: [8547, 11011],
    },
    HelpLabelSegment::Quad {
        control: [8527, 10987],
        to: [8518, 10962],
    },
    HelpLabelSegment::Quad {
        control: [8507, 10935],
        to: [8507, 10905],
    },
    HelpLabelSegment::Quad {
        control: [8507, 10828],
        to: [8563, 10785],
    },
    HelpLabelSegment::Quad {
        control: [8620, 10741],
        to: [8711, 10741],
    },
    HelpLabelSegment::Quad {
        control: [8779, 10741],
        to: [8883, 10785],
    },
];

const BODY_CONTOUR_467: [HelpLabelSegment; 16] = [
    HelpLabelSegment::Quad {
        control: [9476, 10782],
        to: [9503, 10803],
    },
    HelpLabelSegment::Line([9448, 10882]),
    HelpLabelSegment::Quad {
        control: [9430, 10866],
        to: [9388, 10850],
    },
    HelpLabelSegment::Line([9303, 10835]),
    HelpLabelSegment::Quad {
        control: [9213, 10835],
        to: [9159, 10898],
    },
    HelpLabelSegment::Quad {
        control: [9106, 10962],
        to: [9106, 11073],
    },
    HelpLabelSegment::Quad {
        control: [9106, 11183],
        to: [9160, 11241],
    },
    HelpLabelSegment::Quad {
        control: [9215, 11298],
        to: [9311, 11298],
    },
    HelpLabelSegment::Quad {
        control: [9386, 11298],
        to: [9462, 11241],
    },
    HelpLabelSegment::Line([9507, 11334]),
    HelpLabelSegment::Quad {
        control: [9416, 11392],
        to: [9284, 11392],
    },
    HelpLabelSegment::Quad {
        control: [9156, 11392],
        to: [9072, 11306],
    },
    HelpLabelSegment::Quad {
        control: [8989, 11219],
        to: [8989, 11073],
    },
    HelpLabelSegment::Quad {
        control: [8989, 10923],
        to: [9075, 10832],
    },
    HelpLabelSegment::Quad {
        control: [9162, 10741],
        to: [9313, 10741],
    },
    HelpLabelSegment::Line([9419, 10761]),
];

const BODY_CONTOUR_468: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [9701, 11301],
        to: [9867, 11301],
    },
    HelpLabelSegment::Quad {
        control: [9946, 11301],
        to: [9990, 11238],
    },
    HelpLabelSegment::Quad {
        control: [10034, 11175],
        to: [10034, 11065],
    },
    HelpLabelSegment::Quad {
        control: [10034, 10832],
        to: [9867, 10832],
    },
    HelpLabelSegment::Quad {
        control: [9791, 10832],
        to: [9747, 10894],
    },
    HelpLabelSegment::Quad {
        control: [9701, 10956],
        to: [9701, 11065],
    },
];

const BODY_CONTOUR_469: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [9740, 10741],
        to: [9867, 10741],
    },
    HelpLabelSegment::Quad {
        control: [10002, 10741],
        to: [10077, 10827],
    },
    HelpLabelSegment::Quad {
        control: [10151, 10912],
        to: [10151, 11065],
    },
    HelpLabelSegment::Quad {
        control: [10151, 11217],
        to: [10075, 11305],
    },
    HelpLabelSegment::Quad {
        control: [9999, 11392],
        to: [9867, 11392],
    },
    HelpLabelSegment::Quad {
        control: [9734, 11392],
        to: [9659, 11304],
    },
    HelpLabelSegment::Quad {
        control: [9584, 11215],
        to: [9584, 11065],
    },
    HelpLabelSegment::Quad {
        control: [9584, 10919],
        to: [9662, 10830],
    },
];

const BODY_CONTOUR_470: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Quad {
        control: [10574, 10835],
        to: [10537, 10835],
    },
    HelpLabelSegment::Quad {
        control: [10478, 10835],
        to: [10434, 10889],
    },
    HelpLabelSegment::Quad {
        control: [10389, 10944],
        to: [10389, 11020],
    },
    HelpLabelSegment::Line([10389, 11380]),
    HelpLabelSegment::Line([10278, 11380]),
    HelpLabelSegment::Line([10278, 10753]),
    HelpLabelSegment::Line([10389, 10753]),
    HelpLabelSegment::Line([10389, 10853]),
    HelpLabelSegment::Quad {
        control: [10450, 10741],
        to: [10571, 10741],
    },
    HelpLabelSegment::Line([10656, 10752]),
    HelpLabelSegment::Line([10611, 10860]),
];

const BODY_CONTOUR_471: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [10919, 10835],
        to: [10868, 10883],
    },
    HelpLabelSegment::Quad {
        control: [10820, 10929],
        to: [10813, 10997],
    },
    HelpLabelSegment::Line([11161, 10997]),
    HelpLabelSegment::Quad {
        control: [11161, 10929],
        to: [11119, 10884],
    },
    HelpLabelSegment::Quad {
        control: [11072, 10835],
        to: [10993, 10835],
    },
];

const BODY_CONTOUR_472: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [10925, 11298],
        to: [11008, 11298],
    },
    HelpLabelSegment::Quad {
        control: [11104, 11298],
        to: [11167, 11243],
    },
    HelpLabelSegment::Line([11214, 11323]),
    HelpLabelSegment::Quad {
        control: [11188, 11348],
        to: [11135, 11367],
    },
    HelpLabelSegment::Quad {
        control: [11069, 11392],
        to: [10987, 11392],
    },
    HelpLabelSegment::Quad {
        control: [10868, 11392],
        to: [10785, 11312],
    },
    HelpLabelSegment::Quad {
        control: [10694, 11223],
        to: [10694, 11074],
    },
    HelpLabelSegment::Quad {
        control: [10694, 10918],
        to: [10787, 10825],
    },
    HelpLabelSegment::Quad {
        control: [10872, 10741],
        to: [10988, 10741],
    },
    HelpLabelSegment::Quad {
        control: [11121, 10741],
        to: [11198, 10816],
    },
    HelpLabelSegment::Quad {
        control: [11271, 10889],
        to: [11271, 11010],
    },
    HelpLabelSegment::Quad {
        control: [11271, 11046],
        to: [11263, 11078],
    },
    HelpLabelSegment::Line([10811, 11078]),
    HelpLabelSegment::Quad {
        control: [10811, 11188],
        to: [10871, 11246],
    },
];

const BODY_CONTOUR_473: [HelpLabelSegment; 24] = [
    HelpLabelSegment::Quad {
        control: [12588, 10856],
        to: [12588, 10960],
    },
    HelpLabelSegment::Line([12588, 11380]),
    HelpLabelSegment::Line([12476, 11380]),
    HelpLabelSegment::Line([12476, 10983]),
    HelpLabelSegment::Quad {
        control: [12476, 10835],
        to: [12347, 10835],
    },
    HelpLabelSegment::Quad {
        control: [12308, 10835],
        to: [12272, 10860],
    },
    HelpLabelSegment::Quad {
        control: [12237, 10884],
        to: [12224, 10916],
    },
    HelpLabelSegment::Line([12224, 11380]),
    HelpLabelSegment::Line([12113, 11380]),
    HelpLabelSegment::Line([12113, 10935]),
    HelpLabelSegment::Quad {
        control: [12113, 10888],
        to: [12078, 10862],
    },
    HelpLabelSegment::Quad {
        control: [12043, 10835],
        to: [11985, 10835],
    },
    HelpLabelSegment::Quad {
        control: [11952, 10835],
        to: [11915, 10861],
    },
    HelpLabelSegment::Quad {
        control: [11876, 10887],
        to: [11861, 10917],
    },
    HelpLabelSegment::Line([11861, 11380]),
    HelpLabelSegment::Line([11750, 11380]),
    HelpLabelSegment::Line([11750, 10753]),
    HelpLabelSegment::Line([11822, 10753]),
    HelpLabelSegment::Line([11859, 10826]),
    HelpLabelSegment::Quad {
        control: [11923, 10741],
        to: [12020, 10741],
    },
    HelpLabelSegment::Quad {
        control: [12155, 10741],
        to: [12209, 10825],
    },
    HelpLabelSegment::Quad {
        control: [12228, 10789],
        to: [12278, 10765],
    },
    HelpLabelSegment::Quad {
        control: [12330, 10741],
        to: [12384, 10741],
    },
    HelpLabelSegment::Quad {
        control: [12481, 10741],
        to: [12534, 10799],
    },
];

const BODY_CONTOUR_474: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [12821, 11301],
        to: [12987, 11301],
    },
    HelpLabelSegment::Quad {
        control: [13066, 11301],
        to: [13110, 11238],
    },
    HelpLabelSegment::Quad {
        control: [13154, 11175],
        to: [13154, 11065],
    },
    HelpLabelSegment::Quad {
        control: [13154, 10832],
        to: [12987, 10832],
    },
    HelpLabelSegment::Quad {
        control: [12911, 10832],
        to: [12867, 10894],
    },
    HelpLabelSegment::Quad {
        control: [12821, 10956],
        to: [12821, 11065],
    },
];

const BODY_CONTOUR_475: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [12860, 10741],
        to: [12987, 10741],
    },
    HelpLabelSegment::Quad {
        control: [13122, 10741],
        to: [13197, 10827],
    },
    HelpLabelSegment::Quad {
        control: [13271, 10912],
        to: [13271, 11065],
    },
    HelpLabelSegment::Quad {
        control: [13271, 11217],
        to: [13195, 11305],
    },
    HelpLabelSegment::Quad {
        control: [13119, 11392],
        to: [12987, 11392],
    },
    HelpLabelSegment::Quad {
        control: [12854, 11392],
        to: [12779, 11304],
    },
    HelpLabelSegment::Quad {
        control: [12704, 11215],
        to: [12704, 11065],
    },
    HelpLabelSegment::Quad {
        control: [12704, 10919],
        to: [12782, 10830],
    },
];

const BODY_CONTOUR_476: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Quad {
        control: [13694, 10835],
        to: [13657, 10835],
    },
    HelpLabelSegment::Quad {
        control: [13598, 10835],
        to: [13554, 10889],
    },
    HelpLabelSegment::Quad {
        control: [13509, 10944],
        to: [13509, 11020],
    },
    HelpLabelSegment::Line([13509, 11380]),
    HelpLabelSegment::Line([13398, 11380]),
    HelpLabelSegment::Line([13398, 10753]),
    HelpLabelSegment::Line([13509, 10753]),
    HelpLabelSegment::Line([13509, 10853]),
    HelpLabelSegment::Quad {
        control: [13570, 10741],
        to: [13691, 10741],
    },
    HelpLabelSegment::Line([13776, 10752]),
    HelpLabelSegment::Line([13731, 10860]),
];

const BODY_CONTOUR_477: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [14039, 10835],
        to: [13988, 10883],
    },
    HelpLabelSegment::Quad {
        control: [13940, 10929],
        to: [13933, 10997],
    },
    HelpLabelSegment::Line([14281, 10997]),
    HelpLabelSegment::Quad {
        control: [14281, 10929],
        to: [14239, 10884],
    },
    HelpLabelSegment::Quad {
        control: [14192, 10835],
        to: [14112, 10835],
    },
];

const BODY_CONTOUR_478: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [14045, 11298],
        to: [14128, 11298],
    },
    HelpLabelSegment::Quad {
        control: [14224, 11298],
        to: [14287, 11243],
    },
    HelpLabelSegment::Line([14334, 11323]),
    HelpLabelSegment::Quad {
        control: [14308, 11348],
        to: [14255, 11367],
    },
    HelpLabelSegment::Quad {
        control: [14189, 11392],
        to: [14107, 11392],
    },
    HelpLabelSegment::Quad {
        control: [13988, 11392],
        to: [13905, 11312],
    },
    HelpLabelSegment::Quad {
        control: [13814, 11223],
        to: [13814, 11074],
    },
    HelpLabelSegment::Quad {
        control: [13814, 10918],
        to: [13907, 10825],
    },
    HelpLabelSegment::Quad {
        control: [13992, 10741],
        to: [14108, 10741],
    },
    HelpLabelSegment::Quad {
        control: [14241, 10741],
        to: [14318, 10816],
    },
    HelpLabelSegment::Quad {
        control: [14391, 10889],
        to: [14391, 11010],
    },
    HelpLabelSegment::Quad {
        control: [14391, 11046],
        to: [14383, 11078],
    },
    HelpLabelSegment::Line([13931, 11078]),
    HelpLabelSegment::Quad {
        control: [13931, 11188],
        to: [13991, 11246],
    },
];

const BODY_CONTOUR_479: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Quad {
        control: [-9579, 12441],
        to: [-9517, 12503],
    },
    HelpLabelSegment::Quad {
        control: [-9456, 12566],
        to: [-9456, 12700],
    },
    HelpLabelSegment::Line([-9456, 12925]),
    HelpLabelSegment::Quad {
        control: [-9456, 13009],
        to: [-9406, 13035],
    },
    HelpLabelSegment::Line([-9406, 13092]),
    HelpLabelSegment::Quad {
        control: [-9474, 13092],
        to: [-9507, 13072],
    },
    HelpLabelSegment::Quad {
        control: [-9541, 13053],
        to: [-9556, 13009],
    },
    HelpLabelSegment::Quad {
        control: [-9623, 13092],
        to: [-9760, 13092],
    },
    HelpLabelSegment::Quad {
        control: [-9834, 13092],
        to: [-9888, 13039],
    },
    HelpLabelSegment::Quad {
        control: [-9943, 12985],
        to: [-9943, 12905],
    },
    HelpLabelSegment::Quad {
        control: [-9943, 12809],
        to: [-9859, 12744],
    },
    HelpLabelSegment::Quad {
        control: [-9776, 12678],
        to: [-9647, 12678],
    },
    HelpLabelSegment::Line([-9567, 12693]),
    HelpLabelSegment::Quad {
        control: [-9567, 12541],
        to: [-9703, 12541],
    },
    HelpLabelSegment::Quad {
        control: [-9807, 12541],
        to: [-9863, 12597],
    },
    HelpLabelSegment::Line([-9910, 12503]),
    HelpLabelSegment::Quad {
        control: [-9879, 12478],
        to: [-9822, 12460],
    },
    HelpLabelSegment::Quad {
        control: [-9766, 12441],
        to: [-9716, 12441],
    },
];

const BODY_CONTOUR_480: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [-9725, 12760],
        to: [-9778, 12803],
    },
    HelpLabelSegment::Quad {
        control: [-9832, 12847],
        to: [-9832, 12907],
    },
    HelpLabelSegment::Quad {
        control: [-9832, 13004],
        to: [-9716, 13004],
    },
    HelpLabelSegment::Quad {
        control: [-9631, 13004],
        to: [-9567, 12924],
    },
    HelpLabelSegment::Line([-9567, 12772]),
    HelpLabelSegment::Line([-9641, 12760]),
];

const BODY_CONTOUR_481: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Line([-8783, 13080]),
    HelpLabelSegment::Line([-8895, 13080]),
    HelpLabelSegment::Line([-8895, 12716]),
    HelpLabelSegment::Quad {
        control: [-8895, 12615],
        to: [-8924, 12575],
    },
    HelpLabelSegment::Quad {
        control: [-8955, 12535],
        to: [-9026, 12535],
    },
    HelpLabelSegment::Quad {
        control: [-9064, 12535],
        to: [-9106, 12557],
    },
    HelpLabelSegment::Quad {
        control: [-9147, 12581],
        to: [-9169, 12614],
    },
    HelpLabelSegment::Line([-9169, 13080]),
    HelpLabelSegment::Line([-9280, 13080]),
    HelpLabelSegment::Line([-9280, 12453]),
    HelpLabelSegment::Line([-9204, 12453]),
    HelpLabelSegment::Line([-9169, 12534]),
    HelpLabelSegment::Quad {
        control: [-9114, 12441],
        to: [-8990, 12441],
    },
    HelpLabelSegment::Quad {
        control: [-8783, 12441],
        to: [-8783, 12692],
    },
];

const BODY_CONTOUR_482: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Line([-8117, 12195]),
    HelpLabelSegment::Line([-8117, 13080]),
    HelpLabelSegment::Line([-8228, 13080]),
    HelpLabelSegment::Line([-8228, 13033]),
    HelpLabelSegment::Quad {
        control: [-8285, 13092],
        to: [-8397, 13092],
    },
    HelpLabelSegment::Quad {
        control: [-8514, 13092],
        to: [-8588, 13007],
    },
    HelpLabelSegment::Quad {
        control: [-8660, 12923],
        to: [-8660, 12782],
    },
    HelpLabelSegment::Quad {
        control: [-8660, 12641],
        to: [-8576, 12541],
    },
    HelpLabelSegment::Quad {
        control: [-8492, 12441],
        to: [-8376, 12441],
    },
    HelpLabelSegment::Quad {
        control: [-8278, 12441],
        to: [-8228, 12487],
    },
    HelpLabelSegment::Line([-8228, 12195]),
];

const BODY_CONTOUR_483: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Quad {
        control: [-8237, 12965],
        to: [-8228, 12946],
    },
    HelpLabelSegment::Line([-8228, 12598]),
    HelpLabelSegment::Quad {
        control: [-8270, 12535],
        to: [-8343, 12535],
    },
    HelpLabelSegment::Quad {
        control: [-8433, 12535],
        to: [-8488, 12602],
    },
    HelpLabelSegment::Quad {
        control: [-8543, 12669],
        to: [-8543, 12772],
    },
    HelpLabelSegment::Quad {
        control: [-8543, 12998],
        to: [-8337, 12998],
    },
    HelpLabelSegment::Quad {
        control: [-8311, 12998],
        to: [-8274, 12982],
    },
];

const BODY_CONTOUR_484: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Line([-7555, 12322]),
    HelpLabelSegment::Line([-7444, 12278]),
    HelpLabelSegment::Line([-7444, 12453]),
    HelpLabelSegment::Line([-7272, 12453]),
    HelpLabelSegment::Line([-7272, 12541]),
    HelpLabelSegment::Line([-7444, 12541]),
    HelpLabelSegment::Line([-7444, 12853]),
    HelpLabelSegment::Quad {
        control: [-7444, 12931],
        to: [-7417, 12965],
    },
    HelpLabelSegment::Quad {
        control: [-7391, 12998],
        to: [-7332, 12998],
    },
    HelpLabelSegment::Quad {
        control: [-7289, 12998],
        to: [-7244, 12977],
    },
    HelpLabelSegment::Line([-7227, 13074]),
    HelpLabelSegment::Line([-7379, 13092]),
    HelpLabelSegment::Quad {
        control: [-7454, 13092],
        to: [-7504, 13037],
    },
    HelpLabelSegment::Quad {
        control: [-7555, 12982],
        to: [-7555, 12897],
    },
    HelpLabelSegment::Line([-7555, 12541]),
    HelpLabelSegment::Line([-7628, 12541]),
    HelpLabelSegment::Line([-7628, 12453]),
    HelpLabelSegment::Line([-7555, 12453]),
];

const BODY_CONTOUR_485: [HelpLabelSegment; 16] = [
    HelpLabelSegment::Line([-7009, 12522]),
    HelpLabelSegment::Quad {
        control: [-6987, 12487],
        to: [-6938, 12465],
    },
    HelpLabelSegment::Quad {
        control: [-6888, 12441],
        to: [-6836, 12441],
    },
    HelpLabelSegment::Quad {
        control: [-6736, 12441],
        to: [-6679, 12507],
    },
    HelpLabelSegment::Quad {
        control: [-6622, 12573],
        to: [-6622, 12686],
    },
    HelpLabelSegment::Line([-6622, 13080]),
    HelpLabelSegment::Line([-6734, 13080]),
    HelpLabelSegment::Line([-6734, 12686]),
    HelpLabelSegment::Quad {
        control: [-6734, 12616],
        to: [-6769, 12575],
    },
    HelpLabelSegment::Quad {
        control: [-6803, 12535],
        to: [-6866, 12535],
    },
    HelpLabelSegment::Quad {
        control: [-6906, 12535],
        to: [-6947, 12559],
    },
    HelpLabelSegment::Quad {
        control: [-6988, 12582],
        to: [-7009, 12614],
    },
    HelpLabelSegment::Line([-7009, 13080]),
    HelpLabelSegment::Line([-7120, 13080]),
    HelpLabelSegment::Line([-7120, 12195]),
    HelpLabelSegment::Line([-7009, 12195]),
];

const BODY_CONTOUR_486: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [-6281, 12535],
        to: [-6332, 12583],
    },
    HelpLabelSegment::Quad {
        control: [-6380, 12629],
        to: [-6387, 12697],
    },
    HelpLabelSegment::Line([-6039, 12697]),
    HelpLabelSegment::Quad {
        control: [-6039, 12629],
        to: [-6081, 12584],
    },
    HelpLabelSegment::Quad {
        control: [-6128, 12535],
        to: [-6208, 12535],
    },
];

const BODY_CONTOUR_487: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [-6275, 12998],
        to: [-6192, 12998],
    },
    HelpLabelSegment::Quad {
        control: [-6096, 12998],
        to: [-6033, 12943],
    },
    HelpLabelSegment::Line([-5986, 13023]),
    HelpLabelSegment::Quad {
        control: [-6012, 13048],
        to: [-6065, 13067],
    },
    HelpLabelSegment::Quad {
        control: [-6131, 13092],
        to: [-6213, 13092],
    },
    HelpLabelSegment::Quad {
        control: [-6332, 13092],
        to: [-6415, 13012],
    },
    HelpLabelSegment::Quad {
        control: [-6506, 12923],
        to: [-6506, 12774],
    },
    HelpLabelSegment::Quad {
        control: [-6506, 12618],
        to: [-6413, 12525],
    },
    HelpLabelSegment::Quad {
        control: [-6328, 12441],
        to: [-6212, 12441],
    },
    HelpLabelSegment::Quad {
        control: [-6079, 12441],
        to: [-6002, 12516],
    },
    HelpLabelSegment::Quad {
        control: [-5929, 12589],
        to: [-5929, 12710],
    },
    HelpLabelSegment::Quad {
        control: [-5929, 12746],
        to: [-5937, 12778],
    },
    HelpLabelSegment::Line([-6389, 12778]),
    HelpLabelSegment::Quad {
        control: [-6389, 12888],
        to: [-6329, 12946],
    },
];

const BODY_CONTOUR_488: [HelpLabelSegment; 13] = [
    HelpLabelSegment::Line([-5339, 12499]),
    HelpLabelSegment::Quad {
        control: [-5324, 12478],
        to: [-5279, 12459],
    },
    HelpLabelSegment::Quad {
        control: [-5236, 12441],
        to: [-5194, 12441],
    },
    HelpLabelSegment::Quad {
        control: [-5065, 12441],
        to: [-4985, 12530],
    },
    HelpLabelSegment::Quad {
        control: [-4905, 12619],
        to: [-4905, 12755],
    },
    HelpLabelSegment::Quad {
        control: [-4905, 12912],
        to: [-4985, 13003],
    },
    HelpLabelSegment::Quad {
        control: [-5066, 13092],
        to: [-5203, 13092],
    },
    HelpLabelSegment::Quad {
        control: [-5248, 13092],
        to: [-5290, 13075],
    },
    HelpLabelSegment::Quad {
        control: [-5333, 13059],
        to: [-5355, 13035],
    },
    HelpLabelSegment::Line([-5395, 13092]),
    HelpLabelSegment::Line([-5450, 13092]),
    HelpLabelSegment::Line([-5450, 12195]),
    HelpLabelSegment::Line([-5339, 12195]),
];

const BODY_CONTOUR_489: [HelpLabelSegment; 9] = [
    HelpLabelSegment::Line([-5339, 12946]),
    HelpLabelSegment::Quad {
        control: [-5339, 12956],
        to: [-5298, 12977],
    },
    HelpLabelSegment::Quad {
        control: [-5256, 12998],
        to: [-5235, 12998],
    },
    HelpLabelSegment::Quad {
        control: [-5121, 12998],
        to: [-5072, 12944],
    },
    HelpLabelSegment::Quad {
        control: [-5023, 12889],
        to: [-5023, 12761],
    },
    HelpLabelSegment::Quad {
        control: [-5023, 12655],
        to: [-5080, 12595],
    },
    HelpLabelSegment::Quad {
        control: [-5137, 12535],
        to: [-5235, 12535],
    },
    HelpLabelSegment::Quad {
        control: [-5255, 12535],
        to: [-5291, 12553],
    },
    HelpLabelSegment::Line([-5339, 12584]),
];

const BODY_CONTOUR_490: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Quad {
        control: [-4449, 12441],
        to: [-4387, 12503],
    },
    HelpLabelSegment::Quad {
        control: [-4326, 12566],
        to: [-4326, 12700],
    },
    HelpLabelSegment::Line([-4326, 12925]),
    HelpLabelSegment::Quad {
        control: [-4326, 13009],
        to: [-4276, 13035],
    },
    HelpLabelSegment::Line([-4276, 13092]),
    HelpLabelSegment::Quad {
        control: [-4344, 13092],
        to: [-4377, 13072],
    },
    HelpLabelSegment::Quad {
        control: [-4411, 13053],
        to: [-4426, 13009],
    },
    HelpLabelSegment::Quad {
        control: [-4493, 13092],
        to: [-4630, 13092],
    },
    HelpLabelSegment::Quad {
        control: [-4704, 13092],
        to: [-4758, 13039],
    },
    HelpLabelSegment::Quad {
        control: [-4813, 12985],
        to: [-4813, 12905],
    },
    HelpLabelSegment::Quad {
        control: [-4813, 12809],
        to: [-4729, 12744],
    },
    HelpLabelSegment::Quad {
        control: [-4646, 12678],
        to: [-4517, 12678],
    },
    HelpLabelSegment::Line([-4437, 12693]),
    HelpLabelSegment::Quad {
        control: [-4437, 12541],
        to: [-4573, 12541],
    },
    HelpLabelSegment::Quad {
        control: [-4677, 12541],
        to: [-4733, 12597],
    },
    HelpLabelSegment::Line([-4780, 12503]),
    HelpLabelSegment::Quad {
        control: [-4749, 12478],
        to: [-4692, 12460],
    },
    HelpLabelSegment::Quad {
        control: [-4636, 12441],
        to: [-4586, 12441],
    },
];

const BODY_CONTOUR_491: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [-4595, 12760],
        to: [-4648, 12803],
    },
    HelpLabelSegment::Quad {
        control: [-4702, 12847],
        to: [-4702, 12907],
    },
    HelpLabelSegment::Quad {
        control: [-4702, 13004],
        to: [-4586, 13004],
    },
    HelpLabelSegment::Quad {
        control: [-4501, 13004],
        to: [-4437, 12924],
    },
    HelpLabelSegment::Line([-4437, 12772]),
    HelpLabelSegment::Line([-4511, 12760]),
];

const BODY_CONTOUR_492: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([-3925, 13092]),
    HelpLabelSegment::Quad {
        control: [-4142, 13092],
        to: [-4142, 12903],
    },
    HelpLabelSegment::Line([-4142, 12195]),
    HelpLabelSegment::Line([-4031, 12195]),
    HelpLabelSegment::Line([-4031, 12884]),
    HelpLabelSegment::Quad {
        control: [-4031, 12935],
        to: [-4001, 12964],
    },
    HelpLabelSegment::Quad {
        control: [-3972, 12992],
        to: [-3925, 12992],
    },
];

const BODY_CONTOUR_493: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([-3570, 13092]),
    HelpLabelSegment::Quad {
        control: [-3787, 13092],
        to: [-3787, 12903],
    },
    HelpLabelSegment::Line([-3787, 12195]),
    HelpLabelSegment::Line([-3676, 12195]),
    HelpLabelSegment::Line([-3676, 12884]),
    HelpLabelSegment::Quad {
        control: [-3676, 12935],
        to: [-3646, 12964],
    },
    HelpLabelSegment::Quad {
        control: [-3617, 12992],
        to: [-3570, 12992],
    },
];

const BODY_CONTOUR_494: [HelpLabelSegment; 22] = [
    HelpLabelSegment::Line([-3133, 12588]),
    HelpLabelSegment::Quad {
        control: [-3199, 12535],
        to: [-3266, 12535],
    },
    HelpLabelSegment::Quad {
        control: [-3306, 12535],
        to: [-3333, 12554],
    },
    HelpLabelSegment::Quad {
        control: [-3361, 12573],
        to: [-3361, 12601],
    },
    HelpLabelSegment::Quad {
        control: [-3361, 12662],
        to: [-3291, 12692],
    },
    HelpLabelSegment::Line([-3212, 12728]),
    HelpLabelSegment::Quad {
        control: [-3139, 12762],
        to: [-3105, 12805],
    },
    HelpLabelSegment::Quad {
        control: [-3072, 12848],
        to: [-3072, 12912],
    },
    HelpLabelSegment::Quad {
        control: [-3072, 12997],
        to: [-3131, 13045],
    },
    HelpLabelSegment::Quad {
        control: [-3191, 13092],
        to: [-3295, 13092],
    },
    HelpLabelSegment::Quad {
        control: [-3395, 13092],
        to: [-3481, 13043],
    },
    HelpLabelSegment::Line([-3443, 12937]),
    HelpLabelSegment::Quad {
        control: [-3349, 12998],
        to: [-3293, 12998],
    },
    HelpLabelSegment::Quad {
        control: [-3190, 12998],
        to: [-3190, 12911],
    },
    HelpLabelSegment::Quad {
        control: [-3190, 12849],
        to: [-3289, 12805],
    },
    HelpLabelSegment::Quad {
        control: [-3365, 12769],
        to: [-3392, 12752],
    },
    HelpLabelSegment::Quad {
        control: [-3419, 12733],
        to: [-3438, 12711],
    },
    HelpLabelSegment::Quad {
        control: [-3458, 12687],
        to: [-3467, 12662],
    },
    HelpLabelSegment::Quad {
        control: [-3478, 12635],
        to: [-3478, 12605],
    },
    HelpLabelSegment::Quad {
        control: [-3478, 12528],
        to: [-3422, 12485],
    },
    HelpLabelSegment::Quad {
        control: [-3365, 12441],
        to: [-3274, 12441],
    },
    HelpLabelSegment::Quad {
        control: [-3206, 12441],
        to: [-3102, 12485],
    },
];

const BODY_CONTOUR_495: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Line([-2555, 12322]),
    HelpLabelSegment::Line([-2444, 12278]),
    HelpLabelSegment::Line([-2444, 12453]),
    HelpLabelSegment::Line([-2272, 12453]),
    HelpLabelSegment::Line([-2272, 12541]),
    HelpLabelSegment::Line([-2444, 12541]),
    HelpLabelSegment::Line([-2444, 12853]),
    HelpLabelSegment::Quad {
        control: [-2444, 12931],
        to: [-2417, 12965],
    },
    HelpLabelSegment::Quad {
        control: [-2391, 12998],
        to: [-2332, 12998],
    },
    HelpLabelSegment::Quad {
        control: [-2289, 12998],
        to: [-2244, 12977],
    },
    HelpLabelSegment::Line([-2227, 13074]),
    HelpLabelSegment::Line([-2379, 13092]),
    HelpLabelSegment::Quad {
        control: [-2454, 13092],
        to: [-2504, 13037],
    },
    HelpLabelSegment::Quad {
        control: [-2555, 12982],
        to: [-2555, 12897],
    },
    HelpLabelSegment::Line([-2555, 12541]),
    HelpLabelSegment::Line([-2628, 12541]),
    HelpLabelSegment::Line([-2628, 12453]),
    HelpLabelSegment::Line([-2555, 12453]),
];

const BODY_CONTOUR_496: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Quad {
        control: [-1789, 12441],
        to: [-1727, 12503],
    },
    HelpLabelSegment::Quad {
        control: [-1666, 12566],
        to: [-1666, 12700],
    },
    HelpLabelSegment::Line([-1666, 12925]),
    HelpLabelSegment::Quad {
        control: [-1666, 13009],
        to: [-1616, 13035],
    },
    HelpLabelSegment::Line([-1616, 13092]),
    HelpLabelSegment::Quad {
        control: [-1684, 13092],
        to: [-1717, 13072],
    },
    HelpLabelSegment::Quad {
        control: [-1751, 13053],
        to: [-1766, 13009],
    },
    HelpLabelSegment::Quad {
        control: [-1833, 13092],
        to: [-1970, 13092],
    },
    HelpLabelSegment::Quad {
        control: [-2044, 13092],
        to: [-2098, 13039],
    },
    HelpLabelSegment::Quad {
        control: [-2153, 12985],
        to: [-2153, 12905],
    },
    HelpLabelSegment::Quad {
        control: [-2153, 12809],
        to: [-2069, 12744],
    },
    HelpLabelSegment::Quad {
        control: [-1986, 12678],
        to: [-1857, 12678],
    },
    HelpLabelSegment::Line([-1777, 12693]),
    HelpLabelSegment::Quad {
        control: [-1777, 12541],
        to: [-1913, 12541],
    },
    HelpLabelSegment::Quad {
        control: [-2017, 12541],
        to: [-2073, 12597],
    },
    HelpLabelSegment::Line([-2120, 12503]),
    HelpLabelSegment::Quad {
        control: [-2089, 12478],
        to: [-2032, 12460],
    },
    HelpLabelSegment::Quad {
        control: [-1976, 12441],
        to: [-1926, 12441],
    },
];

const BODY_CONTOUR_497: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [-1935, 12760],
        to: [-1988, 12803],
    },
    HelpLabelSegment::Quad {
        control: [-2042, 12847],
        to: [-2042, 12907],
    },
    HelpLabelSegment::Quad {
        control: [-2042, 13004],
        to: [-1926, 13004],
    },
    HelpLabelSegment::Quad {
        control: [-1841, 13004],
        to: [-1777, 12924],
    },
    HelpLabelSegment::Line([-1777, 12772]),
    HelpLabelSegment::Line([-1851, 12760]),
];

const BODY_CONTOUR_498: [HelpLabelSegment; 12] = [
    HelpLabelSegment::Line([-964, 13080]),
    HelpLabelSegment::Line([-1085, 13080]),
    HelpLabelSegment::Line([-1282, 12766]),
    HelpLabelSegment::Line([-1379, 12867]),
    HelpLabelSegment::Line([-1379, 13080]),
    HelpLabelSegment::Line([-1490, 13080]),
    HelpLabelSegment::Line([-1490, 12195]),
    HelpLabelSegment::Line([-1379, 12195]),
    HelpLabelSegment::Line([-1379, 12745]),
    HelpLabelSegment::Line([-1139, 12453]),
    HelpLabelSegment::Line([-1009, 12453]),
    HelpLabelSegment::Line([-1210, 12691]),
];

const BODY_CONTOUR_499: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [-701, 12535],
        to: [-752, 12583],
    },
    HelpLabelSegment::Quad {
        control: [-800, 12629],
        to: [-807, 12697],
    },
    HelpLabelSegment::Line([-459, 12697]),
    HelpLabelSegment::Quad {
        control: [-459, 12629],
        to: [-501, 12584],
    },
    HelpLabelSegment::Quad {
        control: [-548, 12535],
        to: [-628, 12535],
    },
];

const BODY_CONTOUR_500: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [-695, 12998],
        to: [-612, 12998],
    },
    HelpLabelSegment::Quad {
        control: [-516, 12998],
        to: [-453, 12943],
    },
    HelpLabelSegment::Line([-406, 13023]),
    HelpLabelSegment::Quad {
        control: [-432, 13048],
        to: [-485, 13067],
    },
    HelpLabelSegment::Quad {
        control: [-551, 13092],
        to: [-633, 13092],
    },
    HelpLabelSegment::Quad {
        control: [-752, 13092],
        to: [-835, 13012],
    },
    HelpLabelSegment::Quad {
        control: [-926, 12923],
        to: [-926, 12774],
    },
    HelpLabelSegment::Quad {
        control: [-926, 12618],
        to: [-833, 12525],
    },
    HelpLabelSegment::Quad {
        control: [-748, 12441],
        to: [-632, 12441],
    },
    HelpLabelSegment::Quad {
        control: [-499, 12441],
        to: [-422, 12516],
    },
    HelpLabelSegment::Quad {
        control: [-349, 12589],
        to: [-349, 12710],
    },
    HelpLabelSegment::Quad {
        control: [-349, 12746],
        to: [-357, 12778],
    },
    HelpLabelSegment::Line([-809, 12778]),
    HelpLabelSegment::Quad {
        control: [-809, 12888],
        to: [-749, 12946],
    },
];

const BODY_CONTOUR_501: [HelpLabelSegment; 15] = [
    HelpLabelSegment::Line([235, 12853]),
    HelpLabelSegment::Quad {
        control: [235, 12998],
        to: [361, 12998],
    },
    HelpLabelSegment::Quad {
        control: [416, 12998],
        to: [461, 12966],
    },
    HelpLabelSegment::Quad {
        control: [507, 12935],
        to: [522, 12894],
    },
    HelpLabelSegment::Line([522, 12453]),
    HelpLabelSegment::Line([634, 12453]),
    HelpLabelSegment::Line([634, 13080]),
    HelpLabelSegment::Line([522, 13080]),
    HelpLabelSegment::Line([522, 12993]),
    HelpLabelSegment::Quad {
        control: [504, 13031],
        to: [447, 13061],
    },
    HelpLabelSegment::Quad {
        control: [390, 13092],
        to: [336, 13092],
    },
    HelpLabelSegment::Quad {
        control: [233, 13092],
        to: [179, 13033],
    },
    HelpLabelSegment::Quad {
        control: [124, 12973],
        to: [124, 12864],
    },
    HelpLabelSegment::Line([124, 12453]),
    HelpLabelSegment::Line([235, 12453]),
];

const BODY_CONTOUR_502: [HelpLabelSegment; 12] = [
    HelpLabelSegment::Line([896, 12505]),
    HelpLabelSegment::Quad {
        control: [959, 12441],
        to: [1048, 12441],
    },
    HelpLabelSegment::Quad {
        control: [1182, 12441],
        to: [1257, 12525],
    },
    HelpLabelSegment::Quad {
        control: [1331, 12608],
        to: [1331, 12768],
    },
    HelpLabelSegment::Quad {
        control: [1331, 12911],
        to: [1256, 13001],
    },
    HelpLabelSegment::Quad {
        control: [1181, 13092],
        to: [1039, 13092],
    },
    HelpLabelSegment::Quad {
        control: [999, 13092],
        to: [955, 13078],
    },
    HelpLabelSegment::Quad {
        control: [909, 13064],
        to: [896, 13046],
    },
    HelpLabelSegment::Line([896, 13326]),
    HelpLabelSegment::Line([785, 13326]),
    HelpLabelSegment::Line([785, 12453]),
    HelpLabelSegment::Line([896, 12453]),
];

const BODY_CONTOUR_503: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Line([896, 12953]),
    HelpLabelSegment::Quad {
        control: [907, 12970],
        to: [941, 12984],
    },
    HelpLabelSegment::Quad {
        control: [975, 12998],
        to: [1006, 12998],
    },
    HelpLabelSegment::Quad {
        control: [1214, 12998],
        to: [1214, 12764],
    },
    HelpLabelSegment::Quad {
        control: [1214, 12645],
        to: [1164, 12590],
    },
    HelpLabelSegment::Quad {
        control: [1115, 12535],
        to: [1007, 12535],
    },
    HelpLabelSegment::Quad {
        control: [984, 12535],
        to: [950, 12551],
    },
    HelpLabelSegment::Line([896, 12588]),
];

const BODY_CONTOUR_504: [HelpLabelSegment; 22] = [
    HelpLabelSegment::Line([2122, 12588]),
    HelpLabelSegment::Quad {
        control: [2056, 12535],
        to: [1989, 12535],
    },
    HelpLabelSegment::Quad {
        control: [1949, 12535],
        to: [1922, 12554],
    },
    HelpLabelSegment::Quad {
        control: [1894, 12573],
        to: [1894, 12601],
    },
    HelpLabelSegment::Quad {
        control: [1894, 12662],
        to: [1964, 12692],
    },
    HelpLabelSegment::Line([2043, 12728]),
    HelpLabelSegment::Quad {
        control: [2116, 12762],
        to: [2150, 12805],
    },
    HelpLabelSegment::Quad {
        control: [2183, 12848],
        to: [2183, 12912],
    },
    HelpLabelSegment::Quad {
        control: [2183, 12997],
        to: [2124, 13045],
    },
    HelpLabelSegment::Quad {
        control: [2064, 13092],
        to: [1960, 13092],
    },
    HelpLabelSegment::Quad {
        control: [1860, 13092],
        to: [1774, 13043],
    },
    HelpLabelSegment::Line([1812, 12937]),
    HelpLabelSegment::Quad {
        control: [1906, 12998],
        to: [1962, 12998],
    },
    HelpLabelSegment::Quad {
        control: [2065, 12998],
        to: [2065, 12911],
    },
    HelpLabelSegment::Quad {
        control: [2065, 12849],
        to: [1966, 12805],
    },
    HelpLabelSegment::Quad {
        control: [1890, 12769],
        to: [1863, 12752],
    },
    HelpLabelSegment::Quad {
        control: [1836, 12733],
        to: [1817, 12711],
    },
    HelpLabelSegment::Quad {
        control: [1797, 12687],
        to: [1788, 12662],
    },
    HelpLabelSegment::Quad {
        control: [1777, 12635],
        to: [1777, 12605],
    },
    HelpLabelSegment::Quad {
        control: [1777, 12528],
        to: [1833, 12485],
    },
    HelpLabelSegment::Quad {
        control: [1890, 12441],
        to: [1981, 12441],
    },
    HelpLabelSegment::Quad {
        control: [2049, 12441],
        to: [2153, 12485],
    },
];

const BODY_CONTOUR_505: [HelpLabelSegment; 12] = [
    HelpLabelSegment::Line([2411, 12505]),
    HelpLabelSegment::Quad {
        control: [2474, 12441],
        to: [2563, 12441],
    },
    HelpLabelSegment::Quad {
        control: [2697, 12441],
        to: [2772, 12525],
    },
    HelpLabelSegment::Quad {
        control: [2846, 12608],
        to: [2846, 12768],
    },
    HelpLabelSegment::Quad {
        control: [2846, 12911],
        to: [2771, 13001],
    },
    HelpLabelSegment::Quad {
        control: [2696, 13092],
        to: [2554, 13092],
    },
    HelpLabelSegment::Quad {
        control: [2514, 13092],
        to: [2470, 13078],
    },
    HelpLabelSegment::Quad {
        control: [2424, 13064],
        to: [2411, 13046],
    },
    HelpLabelSegment::Line([2411, 13326]),
    HelpLabelSegment::Line([2300, 13326]),
    HelpLabelSegment::Line([2300, 12453]),
    HelpLabelSegment::Line([2411, 12453]),
];

const BODY_CONTOUR_506: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Line([2411, 12953]),
    HelpLabelSegment::Quad {
        control: [2422, 12970],
        to: [2456, 12984],
    },
    HelpLabelSegment::Quad {
        control: [2490, 12998],
        to: [2521, 12998],
    },
    HelpLabelSegment::Quad {
        control: [2729, 12998],
        to: [2729, 12764],
    },
    HelpLabelSegment::Quad {
        control: [2729, 12645],
        to: [2679, 12590],
    },
    HelpLabelSegment::Quad {
        control: [2630, 12535],
        to: [2522, 12535],
    },
    HelpLabelSegment::Quad {
        control: [2499, 12535],
        to: [2465, 12551],
    },
    HelpLabelSegment::Line([2411, 12588]),
];

const BODY_CONTOUR_507: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [3154, 12535],
        to: [3103, 12583],
    },
    HelpLabelSegment::Quad {
        control: [3055, 12629],
        to: [3048, 12697],
    },
    HelpLabelSegment::Line([3396, 12697]),
    HelpLabelSegment::Quad {
        control: [3396, 12629],
        to: [3354, 12584],
    },
    HelpLabelSegment::Quad {
        control: [3307, 12535],
        to: [3227, 12535],
    },
];

const BODY_CONTOUR_508: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [3160, 12998],
        to: [3243, 12998],
    },
    HelpLabelSegment::Quad {
        control: [3339, 12998],
        to: [3402, 12943],
    },
    HelpLabelSegment::Line([3449, 13023]),
    HelpLabelSegment::Quad {
        control: [3423, 13048],
        to: [3370, 13067],
    },
    HelpLabelSegment::Quad {
        control: [3304, 13092],
        to: [3222, 13092],
    },
    HelpLabelSegment::Quad {
        control: [3103, 13092],
        to: [3020, 13012],
    },
    HelpLabelSegment::Quad {
        control: [2929, 12923],
        to: [2929, 12774],
    },
    HelpLabelSegment::Quad {
        control: [2929, 12618],
        to: [3022, 12525],
    },
    HelpLabelSegment::Quad {
        control: [3107, 12441],
        to: [3223, 12441],
    },
    HelpLabelSegment::Quad {
        control: [3356, 12441],
        to: [3433, 12516],
    },
    HelpLabelSegment::Quad {
        control: [3506, 12589],
        to: [3506, 12710],
    },
    HelpLabelSegment::Quad {
        control: [3506, 12746],
        to: [3498, 12778],
    },
    HelpLabelSegment::Line([3046, 12778]),
    HelpLabelSegment::Quad {
        control: [3046, 12888],
        to: [3106, 12946],
    },
];

const BODY_CONTOUR_509: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [3809, 12535],
        to: [3758, 12583],
    },
    HelpLabelSegment::Quad {
        control: [3710, 12629],
        to: [3703, 12697],
    },
    HelpLabelSegment::Line([4051, 12697]),
    HelpLabelSegment::Quad {
        control: [4051, 12629],
        to: [4009, 12584],
    },
    HelpLabelSegment::Quad {
        control: [3962, 12535],
        to: [3882, 12535],
    },
];

const BODY_CONTOUR_510: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [3815, 12998],
        to: [3898, 12998],
    },
    HelpLabelSegment::Quad {
        control: [3994, 12998],
        to: [4057, 12943],
    },
    HelpLabelSegment::Line([4104, 13023]),
    HelpLabelSegment::Quad {
        control: [4078, 13048],
        to: [4025, 13067],
    },
    HelpLabelSegment::Quad {
        control: [3959, 13092],
        to: [3877, 13092],
    },
    HelpLabelSegment::Quad {
        control: [3758, 13092],
        to: [3675, 13012],
    },
    HelpLabelSegment::Quad {
        control: [3584, 12923],
        to: [3584, 12774],
    },
    HelpLabelSegment::Quad {
        control: [3584, 12618],
        to: [3677, 12525],
    },
    HelpLabelSegment::Quad {
        control: [3762, 12441],
        to: [3878, 12441],
    },
    HelpLabelSegment::Quad {
        control: [4011, 12441],
        to: [4088, 12516],
    },
    HelpLabelSegment::Quad {
        control: [4161, 12589],
        to: [4161, 12710],
    },
    HelpLabelSegment::Quad {
        control: [4161, 12746],
        to: [4153, 12778],
    },
    HelpLabelSegment::Line([3701, 12778]),
    HelpLabelSegment::Quad {
        control: [3701, 12888],
        to: [3761, 12946],
    },
];

const BODY_CONTOUR_511: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Line([4788, 12195]),
    HelpLabelSegment::Line([4788, 13080]),
    HelpLabelSegment::Line([4677, 13080]),
    HelpLabelSegment::Line([4677, 13033]),
    HelpLabelSegment::Quad {
        control: [4620, 13092],
        to: [4508, 13092],
    },
    HelpLabelSegment::Quad {
        control: [4391, 13092],
        to: [4317, 13007],
    },
    HelpLabelSegment::Quad {
        control: [4245, 12923],
        to: [4245, 12782],
    },
    HelpLabelSegment::Quad {
        control: [4245, 12641],
        to: [4329, 12541],
    },
    HelpLabelSegment::Quad {
        control: [4413, 12441],
        to: [4529, 12441],
    },
    HelpLabelSegment::Quad {
        control: [4627, 12441],
        to: [4677, 12487],
    },
    HelpLabelSegment::Line([4677, 12195]),
];

const BODY_CONTOUR_512: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Quad {
        control: [4668, 12965],
        to: [4677, 12946],
    },
    HelpLabelSegment::Line([4677, 12598]),
    HelpLabelSegment::Quad {
        control: [4635, 12535],
        to: [4562, 12535],
    },
    HelpLabelSegment::Quad {
        control: [4472, 12535],
        to: [4417, 12602],
    },
    HelpLabelSegment::Quad {
        control: [4362, 12669],
        to: [4362, 12772],
    },
    HelpLabelSegment::Quad {
        control: [4362, 12998],
        to: [4568, 12998],
    },
    HelpLabelSegment::Quad {
        control: [4594, 12998],
        to: [4631, 12982],
    },
];

const BODY_CONTOUR_513: [HelpLabelSegment; 10] = [
    HelpLabelSegment::Quad {
        control: [5172, 13089],
        to: [5140, 13153],
    },
    HelpLabelSegment::Quad {
        control: [5107, 13215],
        to: [5001, 13293],
    },
    HelpLabelSegment::Line([4972, 13251]),
    HelpLabelSegment::Quad {
        control: [5073, 13168],
        to: [5073, 13110],
    },
    HelpLabelSegment::Quad {
        control: [5073, 13085],
        to: [5055, 13059],
    },
    HelpLabelSegment::Quad {
        control: [5005, 13034],
        to: [5005, 12990],
    },
    HelpLabelSegment::Quad {
        control: [5005, 12958],
        to: [5028, 12938],
    },
    HelpLabelSegment::Quad {
        control: [5053, 12918],
        to: [5089, 12918],
    },
    HelpLabelSegment::Quad {
        control: [5121, 12918],
        to: [5147, 12946],
    },
    HelpLabelSegment::Quad {
        control: [5172, 12975],
        to: [5172, 13012],
    },
];

const BODY_CONTOUR_514: [HelpLabelSegment; 13] = [
    HelpLabelSegment::Line([6330, 13092]),
    HelpLabelSegment::Line([6300, 13092]),
    HelpLabelSegment::Line([6116, 12665]),
    HelpLabelSegment::Line([5932, 13092]),
    HelpLabelSegment::Line([5903, 13092]),
    HelpLabelSegment::Line([5679, 12451]),
    HelpLabelSegment::Line([5798, 12451]),
    HelpLabelSegment::Line([5932, 12863]),
    HelpLabelSegment::Line([6099, 12451]),
    HelpLabelSegment::Line([6128, 12451]),
    HelpLabelSegment::Line([6300, 12863]),
    HelpLabelSegment::Line([6445, 12451]),
    HelpLabelSegment::Line([6555, 12451]),
];

const BODY_CONTOUR_515: [HelpLabelSegment; 16] = [
    HelpLabelSegment::Line([6756, 12522]),
    HelpLabelSegment::Quad {
        control: [6778, 12487],
        to: [6827, 12465],
    },
    HelpLabelSegment::Quad {
        control: [6877, 12441],
        to: [6929, 12441],
    },
    HelpLabelSegment::Quad {
        control: [7029, 12441],
        to: [7086, 12507],
    },
    HelpLabelSegment::Quad {
        control: [7143, 12573],
        to: [7143, 12686],
    },
    HelpLabelSegment::Line([7143, 13080]),
    HelpLabelSegment::Line([7031, 13080]),
    HelpLabelSegment::Line([7031, 12686]),
    HelpLabelSegment::Quad {
        control: [7031, 12616],
        to: [6996, 12575],
    },
    HelpLabelSegment::Quad {
        control: [6962, 12535],
        to: [6899, 12535],
    },
    HelpLabelSegment::Quad {
        control: [6859, 12535],
        to: [6818, 12559],
    },
    HelpLabelSegment::Quad {
        control: [6777, 12582],
        to: [6756, 12614],
    },
    HelpLabelSegment::Line([6756, 13080]),
    HelpLabelSegment::Line([6645, 13080]),
    HelpLabelSegment::Line([6645, 12195]),
    HelpLabelSegment::Line([6756, 12195]),
];

const BODY_CONTOUR_516: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [7484, 12535],
        to: [7433, 12583],
    },
    HelpLabelSegment::Quad {
        control: [7385, 12629],
        to: [7378, 12697],
    },
    HelpLabelSegment::Line([7726, 12697]),
    HelpLabelSegment::Quad {
        control: [7726, 12629],
        to: [7684, 12584],
    },
    HelpLabelSegment::Quad {
        control: [7637, 12535],
        to: [7557, 12535],
    },
];

const BODY_CONTOUR_517: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [7490, 12998],
        to: [7573, 12998],
    },
    HelpLabelSegment::Quad {
        control: [7669, 12998],
        to: [7732, 12943],
    },
    HelpLabelSegment::Line([7779, 13023]),
    HelpLabelSegment::Quad {
        control: [7753, 13048],
        to: [7700, 13067],
    },
    HelpLabelSegment::Quad {
        control: [7634, 13092],
        to: [7552, 13092],
    },
    HelpLabelSegment::Quad {
        control: [7433, 13092],
        to: [7350, 13012],
    },
    HelpLabelSegment::Quad {
        control: [7259, 12923],
        to: [7259, 12774],
    },
    HelpLabelSegment::Quad {
        control: [7259, 12618],
        to: [7352, 12525],
    },
    HelpLabelSegment::Quad {
        control: [7437, 12441],
        to: [7553, 12441],
    },
    HelpLabelSegment::Quad {
        control: [7686, 12441],
        to: [7763, 12516],
    },
    HelpLabelSegment::Quad {
        control: [7836, 12589],
        to: [7836, 12710],
    },
    HelpLabelSegment::Quad {
        control: [7836, 12746],
        to: [7828, 12778],
    },
    HelpLabelSegment::Line([7376, 12778]),
    HelpLabelSegment::Quad {
        control: [7376, 12888],
        to: [7436, 12946],
    },
];

const BODY_CONTOUR_518: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Line([8452, 13080]),
    HelpLabelSegment::Line([8340, 13080]),
    HelpLabelSegment::Line([8340, 12716]),
    HelpLabelSegment::Quad {
        control: [8340, 12615],
        to: [8311, 12575],
    },
    HelpLabelSegment::Quad {
        control: [8280, 12535],
        to: [8209, 12535],
    },
    HelpLabelSegment::Quad {
        control: [8171, 12535],
        to: [8129, 12557],
    },
    HelpLabelSegment::Quad {
        control: [8088, 12581],
        to: [8066, 12614],
    },
    HelpLabelSegment::Line([8066, 13080]),
    HelpLabelSegment::Line([7955, 13080]),
    HelpLabelSegment::Line([7955, 12453]),
    HelpLabelSegment::Line([8031, 12453]),
    HelpLabelSegment::Line([8066, 12534]),
    HelpLabelSegment::Quad {
        control: [8121, 12441],
        to: [8245, 12441],
    },
    HelpLabelSegment::Quad {
        control: [8452, 12441],
        to: [8452, 12692],
    },
];

const BODY_CONTOUR_519: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Line([9010, 12322]),
    HelpLabelSegment::Line([9121, 12278]),
    HelpLabelSegment::Line([9121, 12453]),
    HelpLabelSegment::Line([9293, 12453]),
    HelpLabelSegment::Line([9293, 12541]),
    HelpLabelSegment::Line([9121, 12541]),
    HelpLabelSegment::Line([9121, 12853]),
    HelpLabelSegment::Quad {
        control: [9121, 12931],
        to: [9148, 12965],
    },
    HelpLabelSegment::Quad {
        control: [9174, 12998],
        to: [9233, 12998],
    },
    HelpLabelSegment::Quad {
        control: [9276, 12998],
        to: [9321, 12977],
    },
    HelpLabelSegment::Line([9338, 13074]),
    HelpLabelSegment::Line([9186, 13092]),
    HelpLabelSegment::Quad {
        control: [9111, 13092],
        to: [9061, 13037],
    },
    HelpLabelSegment::Quad {
        control: [9010, 12982],
        to: [9010, 12897],
    },
    HelpLabelSegment::Line([9010, 12541]),
    HelpLabelSegment::Line([8937, 12541]),
    HelpLabelSegment::Line([8937, 12453]),
    HelpLabelSegment::Line([9010, 12453]),
];

const BODY_CONTOUR_520: [HelpLabelSegment; 16] = [
    HelpLabelSegment::Line([9556, 12522]),
    HelpLabelSegment::Quad {
        control: [9578, 12487],
        to: [9627, 12465],
    },
    HelpLabelSegment::Quad {
        control: [9677, 12441],
        to: [9729, 12441],
    },
    HelpLabelSegment::Quad {
        control: [9829, 12441],
        to: [9886, 12507],
    },
    HelpLabelSegment::Quad {
        control: [9943, 12573],
        to: [9943, 12686],
    },
    HelpLabelSegment::Line([9943, 13080]),
    HelpLabelSegment::Line([9831, 13080]),
    HelpLabelSegment::Line([9831, 12686]),
    HelpLabelSegment::Quad {
        control: [9831, 12616],
        to: [9796, 12575],
    },
    HelpLabelSegment::Quad {
        control: [9762, 12535],
        to: [9699, 12535],
    },
    HelpLabelSegment::Quad {
        control: [9659, 12535],
        to: [9618, 12559],
    },
    HelpLabelSegment::Quad {
        control: [9577, 12582],
        to: [9556, 12614],
    },
    HelpLabelSegment::Line([9556, 13080]),
    HelpLabelSegment::Line([9445, 13080]),
    HelpLabelSegment::Line([9445, 12195]),
    HelpLabelSegment::Line([9556, 12195]),
];

const BODY_CONTOUR_521: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [10284, 12535],
        to: [10233, 12583],
    },
    HelpLabelSegment::Quad {
        control: [10185, 12629],
        to: [10178, 12697],
    },
    HelpLabelSegment::Line([10526, 12697]),
    HelpLabelSegment::Quad {
        control: [10526, 12629],
        to: [10484, 12584],
    },
    HelpLabelSegment::Quad {
        control: [10437, 12535],
        to: [10357, 12535],
    },
];

const BODY_CONTOUR_522: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [10290, 12998],
        to: [10373, 12998],
    },
    HelpLabelSegment::Quad {
        control: [10469, 12998],
        to: [10532, 12943],
    },
    HelpLabelSegment::Line([10579, 13023]),
    HelpLabelSegment::Quad {
        control: [10553, 13048],
        to: [10500, 13067],
    },
    HelpLabelSegment::Quad {
        control: [10434, 13092],
        to: [10352, 13092],
    },
    HelpLabelSegment::Quad {
        control: [10233, 13092],
        to: [10150, 13012],
    },
    HelpLabelSegment::Quad {
        control: [10059, 12923],
        to: [10059, 12774],
    },
    HelpLabelSegment::Quad {
        control: [10059, 12618],
        to: [10152, 12525],
    },
    HelpLabelSegment::Quad {
        control: [10237, 12441],
        to: [10353, 12441],
    },
    HelpLabelSegment::Quad {
        control: [10486, 12441],
        to: [10563, 12516],
    },
    HelpLabelSegment::Quad {
        control: [10636, 12589],
        to: [10636, 12710],
    },
    HelpLabelSegment::Quad {
        control: [10636, 12746],
        to: [10628, 12778],
    },
    HelpLabelSegment::Line([10176, 12778]),
    HelpLabelSegment::Quad {
        control: [10176, 12888],
        to: [10236, 12946],
    },
];

const BODY_CONTOUR_523: [HelpLabelSegment; 13] = [
    HelpLabelSegment::Line([10994, 13187]),
    HelpLabelSegment::Quad {
        control: [10973, 13246],
        to: [10904, 13286],
    },
    HelpLabelSegment::Quad {
        control: [10833, 13326],
        to: [10748, 13326],
    },
    HelpLabelSegment::Line([10748, 13226]),
    HelpLabelSegment::Quad {
        control: [10818, 13226],
        to: [10867, 13195],
    },
    HelpLabelSegment::Quad {
        control: [10918, 13162],
        to: [10918, 13115],
    },
    HelpLabelSegment::Quad {
        control: [10918, 13064],
        to: [10899, 13013],
    },
    HelpLabelSegment::Line([10852, 12889]),
    HelpLabelSegment::Line([10682, 12453]),
    HelpLabelSegment::Line([10796, 12453]),
    HelpLabelSegment::Line([10981, 12938]),
    HelpLabelSegment::Line([11146, 12453]),
    HelpLabelSegment::Line([11260, 12453]),
];

const BODY_CONTOUR_524: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Quad {
        control: [12036, 12441],
        to: [12098, 12503],
    },
    HelpLabelSegment::Quad {
        control: [12159, 12566],
        to: [12159, 12700],
    },
    HelpLabelSegment::Line([12159, 12925]),
    HelpLabelSegment::Quad {
        control: [12159, 13009],
        to: [12209, 13035],
    },
    HelpLabelSegment::Line([12209, 13092]),
    HelpLabelSegment::Quad {
        control: [12141, 13092],
        to: [12108, 13072],
    },
    HelpLabelSegment::Quad {
        control: [12074, 13053],
        to: [12059, 13009],
    },
    HelpLabelSegment::Quad {
        control: [11992, 13092],
        to: [11855, 13092],
    },
    HelpLabelSegment::Quad {
        control: [11781, 13092],
        to: [11727, 13039],
    },
    HelpLabelSegment::Quad {
        control: [11672, 12985],
        to: [11672, 12905],
    },
    HelpLabelSegment::Quad {
        control: [11672, 12809],
        to: [11756, 12744],
    },
    HelpLabelSegment::Quad {
        control: [11839, 12678],
        to: [11968, 12678],
    },
    HelpLabelSegment::Line([12048, 12693]),
    HelpLabelSegment::Quad {
        control: [12048, 12541],
        to: [11912, 12541],
    },
    HelpLabelSegment::Quad {
        control: [11808, 12541],
        to: [11752, 12597],
    },
    HelpLabelSegment::Line([11705, 12503]),
    HelpLabelSegment::Quad {
        control: [11736, 12478],
        to: [11793, 12460],
    },
    HelpLabelSegment::Quad {
        control: [11849, 12441],
        to: [11899, 12441],
    },
];

const BODY_CONTOUR_525: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [11890, 12760],
        to: [11837, 12803],
    },
    HelpLabelSegment::Quad {
        control: [11783, 12847],
        to: [11783, 12907],
    },
    HelpLabelSegment::Quad {
        control: [11783, 13004],
        to: [11899, 13004],
    },
    HelpLabelSegment::Quad {
        control: [11984, 13004],
        to: [12048, 12924],
    },
    HelpLabelSegment::Line([12048, 12772]),
    HelpLabelSegment::Line([11974, 12760]),
];

const BODY_CONTOUR_526: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Quad {
        control: [12639, 12535],
        to: [12602, 12535],
    },
    HelpLabelSegment::Quad {
        control: [12543, 12535],
        to: [12499, 12589],
    },
    HelpLabelSegment::Quad {
        control: [12454, 12644],
        to: [12454, 12720],
    },
    HelpLabelSegment::Line([12454, 13080]),
    HelpLabelSegment::Line([12343, 13080]),
    HelpLabelSegment::Line([12343, 12453]),
    HelpLabelSegment::Line([12454, 12453]),
    HelpLabelSegment::Line([12454, 12553]),
    HelpLabelSegment::Quad {
        control: [12515, 12441],
        to: [12636, 12441],
    },
    HelpLabelSegment::Line([12721, 12452]),
    HelpLabelSegment::Line([12676, 12560]),
];

const BODY_CONTOUR_527: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [12984, 12535],
        to: [12933, 12583],
    },
    HelpLabelSegment::Quad {
        control: [12885, 12629],
        to: [12878, 12697],
    },
    HelpLabelSegment::Line([13226, 12697]),
    HelpLabelSegment::Quad {
        control: [13226, 12629],
        to: [13184, 12584],
    },
    HelpLabelSegment::Quad {
        control: [13137, 12535],
        to: [13057, 12535],
    },
];

const BODY_CONTOUR_528: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [12990, 12998],
        to: [13073, 12998],
    },
    HelpLabelSegment::Quad {
        control: [13169, 12998],
        to: [13232, 12943],
    },
    HelpLabelSegment::Line([13279, 13023]),
    HelpLabelSegment::Quad {
        control: [13253, 13048],
        to: [13200, 13067],
    },
    HelpLabelSegment::Quad {
        control: [13134, 13092],
        to: [13052, 13092],
    },
    HelpLabelSegment::Quad {
        control: [12933, 13092],
        to: [12850, 13012],
    },
    HelpLabelSegment::Quad {
        control: [12759, 12923],
        to: [12759, 12774],
    },
    HelpLabelSegment::Quad {
        control: [12759, 12618],
        to: [12852, 12525],
    },
    HelpLabelSegment::Quad {
        control: [12937, 12441],
        to: [13053, 12441],
    },
    HelpLabelSegment::Quad {
        control: [13186, 12441],
        to: [13263, 12516],
    },
    HelpLabelSegment::Quad {
        control: [13336, 12589],
        to: [13336, 12710],
    },
    HelpLabelSegment::Quad {
        control: [13336, 12746],
        to: [13328, 12778],
    },
    HelpLabelSegment::Line([12876, 12778]),
    HelpLabelSegment::Quad {
        control: [12876, 12888],
        to: [12936, 12946],
    },
];

const BODY_CONTOUR_529: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Quad {
        control: [14119, 12535],
        to: [14082, 12535],
    },
    HelpLabelSegment::Quad {
        control: [14023, 12535],
        to: [13979, 12589],
    },
    HelpLabelSegment::Quad {
        control: [13934, 12644],
        to: [13934, 12720],
    },
    HelpLabelSegment::Line([13934, 13080]),
    HelpLabelSegment::Line([13823, 13080]),
    HelpLabelSegment::Line([13823, 12453]),
    HelpLabelSegment::Line([13934, 12453]),
    HelpLabelSegment::Line([13934, 12553]),
    HelpLabelSegment::Quad {
        control: [13995, 12441],
        to: [14116, 12441],
    },
    HelpLabelSegment::Line([14201, 12452]),
    HelpLabelSegment::Line([14156, 12560]),
];

const BODY_CONTOUR_530: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [14464, 12535],
        to: [14413, 12583],
    },
    HelpLabelSegment::Quad {
        control: [14365, 12629],
        to: [14358, 12697],
    },
    HelpLabelSegment::Line([14706, 12697]),
    HelpLabelSegment::Quad {
        control: [14706, 12629],
        to: [14664, 12584],
    },
    HelpLabelSegment::Quad {
        control: [14617, 12535],
        to: [14538, 12535],
    },
];

const BODY_CONTOUR_531: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [14470, 12998],
        to: [14553, 12998],
    },
    HelpLabelSegment::Quad {
        control: [14649, 12998],
        to: [14712, 12943],
    },
    HelpLabelSegment::Line([14759, 13023]),
    HelpLabelSegment::Quad {
        control: [14733, 13048],
        to: [14680, 13067],
    },
    HelpLabelSegment::Quad {
        control: [14614, 13092],
        to: [14532, 13092],
    },
    HelpLabelSegment::Quad {
        control: [14413, 13092],
        to: [14330, 13012],
    },
    HelpLabelSegment::Quad {
        control: [14239, 12923],
        to: [14239, 12774],
    },
    HelpLabelSegment::Quad {
        control: [14239, 12618],
        to: [14332, 12525],
    },
    HelpLabelSegment::Quad {
        control: [14417, 12441],
        to: [14533, 12441],
    },
    HelpLabelSegment::Quad {
        control: [14666, 12441],
        to: [14743, 12516],
    },
    HelpLabelSegment::Quad {
        control: [14816, 12589],
        to: [14816, 12710],
    },
    HelpLabelSegment::Quad {
        control: [14816, 12746],
        to: [14808, 12778],
    },
    HelpLabelSegment::Line([14356, 12778]),
    HelpLabelSegment::Quad {
        control: [14356, 12888],
        to: [14416, 12946],
    },
];

const BODY_CONTOUR_532: [HelpLabelSegment; 17] = [
    HelpLabelSegment::Quad {
        control: [14985, 12332],
        to: [15044, 12264],
    },
    HelpLabelSegment::Quad {
        control: [15103, 12195],
        to: [15201, 12195],
    },
    HelpLabelSegment::Line([15305, 12213]),
    HelpLabelSegment::Line([15273, 12295]),
    HelpLabelSegment::Line([15210, 12283]),
    HelpLabelSegment::Quad {
        control: [15160, 12283],
        to: [15127, 12322],
    },
    HelpLabelSegment::Quad {
        control: [15093, 12360],
        to: [15093, 12420],
    },
    HelpLabelSegment::Quad {
        control: [15093, 12435],
        to: [15096, 12453],
    },
    HelpLabelSegment::Line([15224, 12453]),
    HelpLabelSegment::Line([15224, 12547]),
    HelpLabelSegment::Line([15096, 12547]),
    HelpLabelSegment::Line([15096, 13080]),
    HelpLabelSegment::Line([14985, 13080]),
    HelpLabelSegment::Line([14985, 12547]),
    HelpLabelSegment::Line([14894, 12547]),
    HelpLabelSegment::Line([14894, 12453]),
    HelpLabelSegment::Line([14985, 12453]),
];

const BODY_CONTOUR_533: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([15605, 13092]),
    HelpLabelSegment::Quad {
        control: [15388, 13092],
        to: [15388, 12903],
    },
    HelpLabelSegment::Line([15388, 12195]),
    HelpLabelSegment::Line([15499, 12195]),
    HelpLabelSegment::Line([15499, 12884]),
    HelpLabelSegment::Quad {
        control: [15499, 12935],
        to: [15529, 12964],
    },
    HelpLabelSegment::Quad {
        control: [15558, 12992],
        to: [15605, 12992],
    },
];

const BODY_CONTOUR_534: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [15919, 12535],
        to: [15868, 12583],
    },
    HelpLabelSegment::Quad {
        control: [15820, 12629],
        to: [15813, 12697],
    },
    HelpLabelSegment::Line([16161, 12697]),
    HelpLabelSegment::Quad {
        control: [16161, 12629],
        to: [16119, 12584],
    },
    HelpLabelSegment::Quad {
        control: [16072, 12535],
        to: [15993, 12535],
    },
];

const BODY_CONTOUR_535: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [15925, 12998],
        to: [16008, 12998],
    },
    HelpLabelSegment::Quad {
        control: [16104, 12998],
        to: [16167, 12943],
    },
    HelpLabelSegment::Line([16214, 13023]),
    HelpLabelSegment::Quad {
        control: [16188, 13048],
        to: [16135, 13067],
    },
    HelpLabelSegment::Quad {
        control: [16069, 13092],
        to: [15987, 13092],
    },
    HelpLabelSegment::Quad {
        control: [15868, 13092],
        to: [15785, 13012],
    },
    HelpLabelSegment::Quad {
        control: [15694, 12923],
        to: [15694, 12774],
    },
    HelpLabelSegment::Quad {
        control: [15694, 12618],
        to: [15787, 12525],
    },
    HelpLabelSegment::Quad {
        control: [15872, 12441],
        to: [15988, 12441],
    },
    HelpLabelSegment::Quad {
        control: [16121, 12441],
        to: [16198, 12516],
    },
    HelpLabelSegment::Quad {
        control: [16271, 12589],
        to: [16271, 12710],
    },
    HelpLabelSegment::Quad {
        control: [16271, 12746],
        to: [16263, 12778],
    },
    HelpLabelSegment::Line([15811, 12778]),
    HelpLabelSegment::Quad {
        control: [15811, 12888],
        to: [15871, 12946],
    },
];

const BODY_CONTOUR_536: [HelpLabelSegment; 16] = [
    HelpLabelSegment::Quad {
        control: [16836, 12482],
        to: [16863, 12503],
    },
    HelpLabelSegment::Line([16808, 12582]),
    HelpLabelSegment::Quad {
        control: [16790, 12566],
        to: [16748, 12550],
    },
    HelpLabelSegment::Line([16663, 12535]),
    HelpLabelSegment::Quad {
        control: [16573, 12535],
        to: [16519, 12598],
    },
    HelpLabelSegment::Quad {
        control: [16466, 12662],
        to: [16466, 12773],
    },
    HelpLabelSegment::Quad {
        control: [16466, 12883],
        to: [16520, 12941],
    },
    HelpLabelSegment::Quad {
        control: [16575, 12998],
        to: [16671, 12998],
    },
    HelpLabelSegment::Quad {
        control: [16746, 12998],
        to: [16822, 12941],
    },
    HelpLabelSegment::Line([16867, 13034]),
    HelpLabelSegment::Quad {
        control: [16776, 13092],
        to: [16644, 13092],
    },
    HelpLabelSegment::Quad {
        control: [16516, 13092],
        to: [16432, 13006],
    },
    HelpLabelSegment::Quad {
        control: [16349, 12919],
        to: [16349, 12773],
    },
    HelpLabelSegment::Quad {
        control: [16349, 12623],
        to: [16435, 12532],
    },
    HelpLabelSegment::Quad {
        control: [16522, 12441],
        to: [16673, 12441],
    },
    HelpLabelSegment::Line([16779, 12461]),
];

const BODY_CONTOUR_537: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Line([17025, 12322]),
    HelpLabelSegment::Line([17136, 12278]),
    HelpLabelSegment::Line([17136, 12453]),
    HelpLabelSegment::Line([17308, 12453]),
    HelpLabelSegment::Line([17308, 12541]),
    HelpLabelSegment::Line([17136, 12541]),
    HelpLabelSegment::Line([17136, 12853]),
    HelpLabelSegment::Quad {
        control: [17136, 12931],
        to: [17163, 12965],
    },
    HelpLabelSegment::Quad {
        control: [17189, 12998],
        to: [17248, 12998],
    },
    HelpLabelSegment::Quad {
        control: [17291, 12998],
        to: [17336, 12977],
    },
    HelpLabelSegment::Line([17353, 13074]),
    HelpLabelSegment::Line([17201, 13092]),
    HelpLabelSegment::Quad {
        control: [17126, 13092],
        to: [17076, 13037],
    },
    HelpLabelSegment::Quad {
        control: [17025, 12982],
        to: [17025, 12897],
    },
    HelpLabelSegment::Line([17025, 12541]),
    HelpLabelSegment::Line([16952, 12541]),
    HelpLabelSegment::Line([16952, 12453]),
    HelpLabelSegment::Line([17025, 12453]),
];

const BODY_CONTOUR_538: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [17644, 12535],
        to: [17593, 12583],
    },
    HelpLabelSegment::Quad {
        control: [17545, 12629],
        to: [17538, 12697],
    },
    HelpLabelSegment::Line([17886, 12697]),
    HelpLabelSegment::Quad {
        control: [17886, 12629],
        to: [17844, 12584],
    },
    HelpLabelSegment::Quad {
        control: [17797, 12535],
        to: [17718, 12535],
    },
];

const BODY_CONTOUR_539: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [17650, 12998],
        to: [17733, 12998],
    },
    HelpLabelSegment::Quad {
        control: [17829, 12998],
        to: [17892, 12943],
    },
    HelpLabelSegment::Line([17939, 13023]),
    HelpLabelSegment::Quad {
        control: [17913, 13048],
        to: [17860, 13067],
    },
    HelpLabelSegment::Quad {
        control: [17794, 13092],
        to: [17712, 13092],
    },
    HelpLabelSegment::Quad {
        control: [17593, 13092],
        to: [17510, 13012],
    },
    HelpLabelSegment::Quad {
        control: [17419, 12923],
        to: [17419, 12774],
    },
    HelpLabelSegment::Quad {
        control: [17419, 12618],
        to: [17512, 12525],
    },
    HelpLabelSegment::Quad {
        control: [17597, 12441],
        to: [17713, 12441],
    },
    HelpLabelSegment::Quad {
        control: [17846, 12441],
        to: [17923, 12516],
    },
    HelpLabelSegment::Quad {
        control: [17996, 12589],
        to: [17996, 12710],
    },
    HelpLabelSegment::Quad {
        control: [17996, 12746],
        to: [17988, 12778],
    },
    HelpLabelSegment::Line([17536, 12778]),
    HelpLabelSegment::Quad {
        control: [17536, 12888],
        to: [17596, 12946],
    },
];

const BODY_CONTOUR_540: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Line([18623, 12195]),
    HelpLabelSegment::Line([18623, 13080]),
    HelpLabelSegment::Line([18512, 13080]),
    HelpLabelSegment::Line([18512, 13033]),
    HelpLabelSegment::Quad {
        control: [18455, 13092],
        to: [18343, 13092],
    },
    HelpLabelSegment::Quad {
        control: [18226, 13092],
        to: [18152, 13007],
    },
    HelpLabelSegment::Quad {
        control: [18080, 12923],
        to: [18080, 12782],
    },
    HelpLabelSegment::Quad {
        control: [18080, 12641],
        to: [18164, 12541],
    },
    HelpLabelSegment::Quad {
        control: [18248, 12441],
        to: [18364, 12441],
    },
    HelpLabelSegment::Quad {
        control: [18462, 12441],
        to: [18512, 12487],
    },
    HelpLabelSegment::Line([18512, 12195]),
];

const BODY_CONTOUR_541: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Quad {
        control: [18503, 12965],
        to: [18512, 12946],
    },
    HelpLabelSegment::Line([18512, 12598]),
    HelpLabelSegment::Quad {
        control: [18470, 12535],
        to: [18397, 12535],
    },
    HelpLabelSegment::Quad {
        control: [18307, 12535],
        to: [18252, 12602],
    },
    HelpLabelSegment::Quad {
        control: [18197, 12669],
        to: [18197, 12772],
    },
    HelpLabelSegment::Quad {
        control: [18197, 12998],
        to: [18403, 12998],
    },
    HelpLabelSegment::Quad {
        control: [18429, 12998],
        to: [18466, 12982],
    },
];

const BODY_CONTOUR_542: [HelpLabelSegment; 13] = [
    HelpLabelSegment::Line([19256, 12499]),
    HelpLabelSegment::Quad {
        control: [19271, 12478],
        to: [19316, 12459],
    },
    HelpLabelSegment::Quad {
        control: [19359, 12441],
        to: [19401, 12441],
    },
    HelpLabelSegment::Quad {
        control: [19530, 12441],
        to: [19610, 12530],
    },
    HelpLabelSegment::Quad {
        control: [19690, 12619],
        to: [19690, 12755],
    },
    HelpLabelSegment::Quad {
        control: [19690, 12912],
        to: [19610, 13003],
    },
    HelpLabelSegment::Quad {
        control: [19529, 13092],
        to: [19392, 13092],
    },
    HelpLabelSegment::Quad {
        control: [19347, 13092],
        to: [19305, 13075],
    },
    HelpLabelSegment::Quad {
        control: [19262, 13059],
        to: [19240, 13035],
    },
    HelpLabelSegment::Line([19200, 13092]),
    HelpLabelSegment::Line([19145, 13092]),
    HelpLabelSegment::Line([19145, 12195]),
    HelpLabelSegment::Line([19256, 12195]),
];

const BODY_CONTOUR_543: [HelpLabelSegment; 9] = [
    HelpLabelSegment::Line([19256, 12946]),
    HelpLabelSegment::Quad {
        control: [19256, 12956],
        to: [19297, 12977],
    },
    HelpLabelSegment::Quad {
        control: [19339, 12998],
        to: [19360, 12998],
    },
    HelpLabelSegment::Quad {
        control: [19474, 12998],
        to: [19523, 12944],
    },
    HelpLabelSegment::Quad {
        control: [19572, 12889],
        to: [19572, 12761],
    },
    HelpLabelSegment::Quad {
        control: [19572, 12655],
        to: [19515, 12595],
    },
    HelpLabelSegment::Quad {
        control: [19458, 12535],
        to: [19360, 12535],
    },
    HelpLabelSegment::Quad {
        control: [19340, 12535],
        to: [19304, 12553],
    },
    HelpLabelSegment::Line([19256, 12584]),
];

const BODY_CONTOUR_544: [HelpLabelSegment; 13] = [
    HelpLabelSegment::Line([20054, 13187]),
    HelpLabelSegment::Quad {
        control: [20033, 13246],
        to: [19964, 13286],
    },
    HelpLabelSegment::Quad {
        control: [19893, 13326],
        to: [19808, 13326],
    },
    HelpLabelSegment::Line([19808, 13226]),
    HelpLabelSegment::Quad {
        control: [19878, 13226],
        to: [19927, 13195],
    },
    HelpLabelSegment::Quad {
        control: [19978, 13162],
        to: [19978, 13115],
    },
    HelpLabelSegment::Quad {
        control: [19978, 13064],
        to: [19959, 13013],
    },
    HelpLabelSegment::Line([19912, 12889]),
    HelpLabelSegment::Line([19742, 12453]),
    HelpLabelSegment::Line([19856, 12453]),
    HelpLabelSegment::Line([20041, 12938]),
    HelpLabelSegment::Line([20206, 12453]),
    HelpLabelSegment::Line([20320, 12453]),
];

const BODY_CONTOUR_545: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Quad {
        control: [21096, 12441],
        to: [21158, 12503],
    },
    HelpLabelSegment::Quad {
        control: [21219, 12566],
        to: [21219, 12700],
    },
    HelpLabelSegment::Line([21219, 12925]),
    HelpLabelSegment::Quad {
        control: [21219, 13009],
        to: [21269, 13035],
    },
    HelpLabelSegment::Line([21269, 13092]),
    HelpLabelSegment::Quad {
        control: [21201, 13092],
        to: [21168, 13072],
    },
    HelpLabelSegment::Quad {
        control: [21134, 13053],
        to: [21119, 13009],
    },
    HelpLabelSegment::Quad {
        control: [21052, 13092],
        to: [20915, 13092],
    },
    HelpLabelSegment::Quad {
        control: [20841, 13092],
        to: [20787, 13039],
    },
    HelpLabelSegment::Quad {
        control: [20732, 12985],
        to: [20732, 12905],
    },
    HelpLabelSegment::Quad {
        control: [20732, 12809],
        to: [20816, 12744],
    },
    HelpLabelSegment::Quad {
        control: [20899, 12678],
        to: [21028, 12678],
    },
    HelpLabelSegment::Line([21108, 12693]),
    HelpLabelSegment::Quad {
        control: [21108, 12541],
        to: [20972, 12541],
    },
    HelpLabelSegment::Quad {
        control: [20868, 12541],
        to: [20812, 12597],
    },
    HelpLabelSegment::Line([20765, 12503]),
    HelpLabelSegment::Quad {
        control: [20796, 12478],
        to: [20853, 12460],
    },
    HelpLabelSegment::Quad {
        control: [20909, 12441],
        to: [20959, 12441],
    },
];

const BODY_CONTOUR_546: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [20950, 12760],
        to: [20897, 12803],
    },
    HelpLabelSegment::Quad {
        control: [20843, 12847],
        to: [20843, 12907],
    },
    HelpLabelSegment::Quad {
        control: [20843, 13004],
        to: [20959, 13004],
    },
    HelpLabelSegment::Quad {
        control: [21044, 13004],
        to: [21108, 12924],
    },
    HelpLabelSegment::Line([21108, 12772]),
    HelpLabelSegment::Line([21034, 12760]),
];

const BODY_CONTOUR_547: [HelpLabelSegment; 12] = [
    HelpLabelSegment::Line([21866, 12505]),
    HelpLabelSegment::Quad {
        control: [21929, 12441],
        to: [22018, 12441],
    },
    HelpLabelSegment::Quad {
        control: [22152, 12441],
        to: [22227, 12525],
    },
    HelpLabelSegment::Quad {
        control: [22301, 12608],
        to: [22301, 12768],
    },
    HelpLabelSegment::Quad {
        control: [22301, 12911],
        to: [22226, 13001],
    },
    HelpLabelSegment::Quad {
        control: [22151, 13092],
        to: [22009, 13092],
    },
    HelpLabelSegment::Quad {
        control: [21969, 13092],
        to: [21925, 13078],
    },
    HelpLabelSegment::Quad {
        control: [21879, 13064],
        to: [21866, 13046],
    },
    HelpLabelSegment::Line([21866, 13326]),
    HelpLabelSegment::Line([21755, 13326]),
    HelpLabelSegment::Line([21755, 12453]),
    HelpLabelSegment::Line([21866, 12453]),
];

const BODY_CONTOUR_548: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Line([21866, 12953]),
    HelpLabelSegment::Quad {
        control: [21877, 12970],
        to: [21911, 12984],
    },
    HelpLabelSegment::Quad {
        control: [21945, 12998],
        to: [21976, 12998],
    },
    HelpLabelSegment::Quad {
        control: [22184, 12998],
        to: [22184, 12764],
    },
    HelpLabelSegment::Quad {
        control: [22184, 12645],
        to: [22134, 12590],
    },
    HelpLabelSegment::Quad {
        control: [22085, 12535],
        to: [21977, 12535],
    },
    HelpLabelSegment::Quad {
        control: [21954, 12535],
        to: [21920, 12551],
    },
    HelpLabelSegment::Line([21866, 12588]),
];

const BODY_CONTOUR_549: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Quad {
        control: [22756, 12441],
        to: [22818, 12503],
    },
    HelpLabelSegment::Quad {
        control: [22879, 12566],
        to: [22879, 12700],
    },
    HelpLabelSegment::Line([22879, 12925]),
    HelpLabelSegment::Quad {
        control: [22879, 13009],
        to: [22929, 13035],
    },
    HelpLabelSegment::Line([22929, 13092]),
    HelpLabelSegment::Quad {
        control: [22861, 13092],
        to: [22828, 13072],
    },
    HelpLabelSegment::Quad {
        control: [22794, 13053],
        to: [22779, 13009],
    },
    HelpLabelSegment::Quad {
        control: [22712, 13092],
        to: [22575, 13092],
    },
    HelpLabelSegment::Quad {
        control: [22501, 13092],
        to: [22447, 13039],
    },
    HelpLabelSegment::Quad {
        control: [22392, 12985],
        to: [22392, 12905],
    },
    HelpLabelSegment::Quad {
        control: [22392, 12809],
        to: [22476, 12744],
    },
    HelpLabelSegment::Quad {
        control: [22559, 12678],
        to: [22688, 12678],
    },
    HelpLabelSegment::Line([22768, 12693]),
    HelpLabelSegment::Quad {
        control: [22768, 12541],
        to: [22632, 12541],
    },
    HelpLabelSegment::Quad {
        control: [22528, 12541],
        to: [22472, 12597],
    },
    HelpLabelSegment::Line([22425, 12503]),
    HelpLabelSegment::Quad {
        control: [22456, 12478],
        to: [22513, 12460],
    },
    HelpLabelSegment::Quad {
        control: [22569, 12441],
        to: [22619, 12441],
    },
];

const BODY_CONTOUR_550: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [22610, 12760],
        to: [22557, 12803],
    },
    HelpLabelSegment::Quad {
        control: [22503, 12847],
        to: [22503, 12907],
    },
    HelpLabelSegment::Quad {
        control: [22503, 13004],
        to: [22619, 13004],
    },
    HelpLabelSegment::Quad {
        control: [22704, 13004],
        to: [22768, 12924],
    },
    HelpLabelSegment::Line([22768, 12772]),
    HelpLabelSegment::Line([22694, 12760]),
];

const BODY_CONTOUR_551: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Line([23563, 12195]),
    HelpLabelSegment::Line([23563, 13080]),
    HelpLabelSegment::Line([23452, 13080]),
    HelpLabelSegment::Line([23452, 13033]),
    HelpLabelSegment::Quad {
        control: [23395, 13092],
        to: [23283, 13092],
    },
    HelpLabelSegment::Quad {
        control: [23166, 13092],
        to: [23092, 13007],
    },
    HelpLabelSegment::Quad {
        control: [23020, 12923],
        to: [23020, 12782],
    },
    HelpLabelSegment::Quad {
        control: [23020, 12641],
        to: [23104, 12541],
    },
    HelpLabelSegment::Quad {
        control: [23188, 12441],
        to: [23304, 12441],
    },
    HelpLabelSegment::Quad {
        control: [23402, 12441],
        to: [23452, 12487],
    },
    HelpLabelSegment::Line([23452, 12195]),
];

const BODY_CONTOUR_552: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Quad {
        control: [23443, 12965],
        to: [23452, 12946],
    },
    HelpLabelSegment::Line([23452, 12598]),
    HelpLabelSegment::Quad {
        control: [23410, 12535],
        to: [23337, 12535],
    },
    HelpLabelSegment::Quad {
        control: [23247, 12535],
        to: [23192, 12602],
    },
    HelpLabelSegment::Quad {
        control: [23137, 12669],
        to: [23137, 12772],
    },
    HelpLabelSegment::Quad {
        control: [23137, 12998],
        to: [23343, 12998],
    },
    HelpLabelSegment::Quad {
        control: [23369, 12998],
        to: [23406, 12982],
    },
];

const BODY_CONTOUR_553: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Line([24233, 12195]),
    HelpLabelSegment::Line([24233, 13080]),
    HelpLabelSegment::Line([24122, 13080]),
    HelpLabelSegment::Line([24122, 13033]),
    HelpLabelSegment::Quad {
        control: [24065, 13092],
        to: [23953, 13092],
    },
    HelpLabelSegment::Quad {
        control: [23836, 13092],
        to: [23762, 13007],
    },
    HelpLabelSegment::Quad {
        control: [23690, 12923],
        to: [23690, 12782],
    },
    HelpLabelSegment::Quad {
        control: [23690, 12641],
        to: [23774, 12541],
    },
    HelpLabelSegment::Quad {
        control: [23858, 12441],
        to: [23974, 12441],
    },
    HelpLabelSegment::Quad {
        control: [24072, 12441],
        to: [24122, 12487],
    },
    HelpLabelSegment::Line([24122, 12195]),
];

const BODY_CONTOUR_554: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Quad {
        control: [24113, 12965],
        to: [24122, 12946],
    },
    HelpLabelSegment::Line([24122, 12598]),
    HelpLabelSegment::Quad {
        control: [24080, 12535],
        to: [24007, 12535],
    },
    HelpLabelSegment::Quad {
        control: [23917, 12535],
        to: [23862, 12602],
    },
    HelpLabelSegment::Quad {
        control: [23807, 12669],
        to: [23807, 12772],
    },
    HelpLabelSegment::Quad {
        control: [23807, 12998],
        to: [24013, 12998],
    },
    HelpLabelSegment::Quad {
        control: [24039, 12998],
        to: [24076, 12982],
    },
];

const BODY_CONTOUR_555: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [24579, 12535],
        to: [24528, 12583],
    },
    HelpLabelSegment::Quad {
        control: [24480, 12629],
        to: [24473, 12697],
    },
    HelpLabelSegment::Line([24821, 12697]),
    HelpLabelSegment::Quad {
        control: [24821, 12629],
        to: [24779, 12584],
    },
    HelpLabelSegment::Quad {
        control: [24732, 12535],
        to: [24653, 12535],
    },
];

const BODY_CONTOUR_556: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [24585, 12998],
        to: [24668, 12998],
    },
    HelpLabelSegment::Quad {
        control: [24764, 12998],
        to: [24827, 12943],
    },
    HelpLabelSegment::Line([24874, 13023]),
    HelpLabelSegment::Quad {
        control: [24848, 13048],
        to: [24795, 13067],
    },
    HelpLabelSegment::Quad {
        control: [24729, 13092],
        to: [24647, 13092],
    },
    HelpLabelSegment::Quad {
        control: [24528, 13092],
        to: [24445, 13012],
    },
    HelpLabelSegment::Quad {
        control: [24354, 12923],
        to: [24354, 12774],
    },
    HelpLabelSegment::Quad {
        control: [24354, 12618],
        to: [24447, 12525],
    },
    HelpLabelSegment::Quad {
        control: [24532, 12441],
        to: [24648, 12441],
    },
    HelpLabelSegment::Quad {
        control: [24781, 12441],
        to: [24858, 12516],
    },
    HelpLabelSegment::Quad {
        control: [24931, 12589],
        to: [24931, 12710],
    },
    HelpLabelSegment::Quad {
        control: [24931, 12746],
        to: [24923, 12778],
    },
    HelpLabelSegment::Line([24471, 12778]),
    HelpLabelSegment::Quad {
        control: [24471, 12888],
        to: [24531, 12946],
    },
];

const BODY_CONTOUR_557: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([25275, 13092]),
    HelpLabelSegment::Quad {
        control: [25058, 13092],
        to: [25058, 12903],
    },
    HelpLabelSegment::Line([25058, 12195]),
    HelpLabelSegment::Line([25169, 12195]),
    HelpLabelSegment::Line([25169, 12884]),
    HelpLabelSegment::Quad {
        control: [25169, 12935],
        to: [25199, 12964],
    },
    HelpLabelSegment::Quad {
        control: [25228, 12992],
        to: [25275, 12992],
    },
];

const BODY_CONTOUR_558: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [25569, 12916],
        to: [25595, 12942],
    },
    HelpLabelSegment::Quad {
        control: [25620, 12968],
        to: [25620, 13004],
    },
    HelpLabelSegment::Quad {
        control: [25620, 13040],
        to: [25595, 13066],
    },
    HelpLabelSegment::Quad {
        control: [25569, 13092],
        to: [25532, 13092],
    },
    HelpLabelSegment::Quad {
        control: [25496, 13092],
        to: [25470, 13066],
    },
    HelpLabelSegment::Quad {
        control: [25445, 13040],
        to: [25445, 13004],
    },
    HelpLabelSegment::Quad {
        control: [25445, 12968],
        to: [25470, 12942],
    },
    HelpLabelSegment::Quad {
        control: [25496, 12916],
        to: [25532, 12916],
    },
];

const BODY_CONTOURS: [HelpLabelContour; 559] = [
    HelpLabelContour {
        start: [-11021, 319],
        segments: &BODY_CONTOUR_0,
    },
    HelpLabelContour {
        start: [-10593, 1016],
        segments: &BODY_CONTOUR_1,
    },
    HelpLabelContour {
        start: [-9619, 428],
        segments: &BODY_CONTOUR_2,
    },
    HelpLabelContour {
        start: [-9034, 660],
        segments: &BODY_CONTOUR_3,
    },
    HelpLabelContour {
        start: [-8405, 553],
        segments: &BODY_CONTOUR_4,
    },
    HelpLabelContour {
        start: [-7920, 553],
        segments: &BODY_CONTOUR_5,
    },
    HelpLabelContour {
        start: [-7409, 865],
        segments: &BODY_CONTOUR_6,
    },
    HelpLabelContour {
        start: [-7448, 630],
        segments: &BODY_CONTOUR_7,
    },
    HelpLabelContour {
        start: [-6142, 585],
        segments: &BODY_CONTOUR_8,
    },
    HelpLabelContour {
        start: [-5884, 295],
        segments: &BODY_CONTOUR_9,
    },
    HelpLabelContour {
        start: [-5264, 865],
        segments: &BODY_CONTOUR_10,
    },
    HelpLabelContour {
        start: [-5303, 630],
        segments: &BODY_CONTOUR_11,
    },
    HelpLabelContour {
        start: [-4619, 865],
        segments: &BODY_CONTOUR_12,
    },
    HelpLabelContour {
        start: [-4658, 630],
        segments: &BODY_CONTOUR_13,
    },
    HelpLabelContour {
        start: [-4010, 553],
        segments: &BODY_CONTOUR_14,
    },
    HelpLabelContour {
        start: [-3021, 541],
        segments: &BODY_CONTOUR_15,
    },
    HelpLabelContour {
        start: [-2946, 860],
        segments: &BODY_CONTOUR_16,
    },
    HelpLabelContour {
        start: [-2247, 585],
        segments: &BODY_CONTOUR_17,
    },
    HelpLabelContour {
        start: [-956, 599],
        segments: &BODY_CONTOUR_18,
    },
    HelpLabelContour {
        start: [-551, 541],
        segments: &BODY_CONTOUR_19,
    },
    HelpLabelContour {
        start: [-476, 860],
        segments: &BODY_CONTOUR_20,
    },
    HelpLabelContour {
        start: [382, 792],
        segments: &BODY_CONTOUR_21,
    },
    HelpLabelContour {
        start: [1045, 553],
        segments: &BODY_CONTOUR_22,
    },
    HelpLabelContour {
        start: [1601, 295],
        segments: &BODY_CONTOUR_23,
    },
    HelpLabelContour {
        start: [1601, 684],
        segments: &BODY_CONTOUR_24,
    },
    HelpLabelContour {
        start: [2354, 541],
        segments: &BODY_CONTOUR_25,
    },
    HelpLabelContour {
        start: [2429, 860],
        segments: &BODY_CONTOUR_26,
    },
    HelpLabelContour {
        start: [3015, 1092],
        segments: &BODY_CONTOUR_27,
    },
    HelpLabelContour {
        start: [3370, 1092],
        segments: &BODY_CONTOUR_28,
    },
    HelpLabelContour {
        start: [3838, 585],
        segments: &BODY_CONTOUR_29,
    },
    HelpLabelContour {
        start: [4539, 541],
        segments: &BODY_CONTOUR_30,
    },
    HelpLabelContour {
        start: [4614, 860],
        segments: &BODY_CONTOUR_31,
    },
    HelpLabelContour {
        start: [5313, 585],
        segments: &BODY_CONTOUR_32,
    },
    HelpLabelContour {
        start: [5931, 553],
        segments: &BODY_CONTOUR_33,
    },
    HelpLabelContour {
        start: [5931, 688],
        segments: &BODY_CONTOUR_34,
    },
    HelpLabelContour {
        start: [6566, 865],
        segments: &BODY_CONTOUR_35,
    },
    HelpLabelContour {
        start: [6527, 630],
        segments: &BODY_CONTOUR_36,
    },
    HelpLabelContour {
        start: [7473, 585],
        segments: &BODY_CONTOUR_37,
    },
    HelpLabelContour {
        start: [7958, 585],
        segments: &BODY_CONTOUR_38,
    },
    HelpLabelContour {
        start: [8220, 315],
        segments: &BODY_CONTOUR_39,
    },
    HelpLabelContour {
        start: [8270, 553],
        segments: &BODY_CONTOUR_40,
    },
    HelpLabelContour {
        start: [8556, 295],
        segments: &BODY_CONTOUR_41,
    },
    HelpLabelContour {
        start: [8556, 684],
        segments: &BODY_CONTOUR_42,
    },
    HelpLabelContour {
        start: [9340, 1092],
        segments: &BODY_CONTOUR_43,
    },
    HelpLabelContour {
        start: [9727, 635],
        segments: &BODY_CONTOUR_44,
    },
    HelpLabelContour {
        start: [9606, 1046],
        segments: &BODY_CONTOUR_45,
    },
    HelpLabelContour {
        start: [10600, 315],
        segments: &BODY_CONTOUR_46,
    },
    HelpLabelContour {
        start: [10650, 553],
        segments: &BODY_CONTOUR_47,
    },
    HelpLabelContour {
        start: [11322, 792],
        segments: &BODY_CONTOUR_48,
    },
    HelpLabelContour {
        start: [11520, 553],
        segments: &BODY_CONTOUR_49,
    },
    HelpLabelContour {
        start: [12031, 865],
        segments: &BODY_CONTOUR_50,
    },
    HelpLabelContour {
        start: [11992, 630],
        segments: &BODY_CONTOUR_51,
    },
    HelpLabelContour {
        start: [13465, 553],
        segments: &BODY_CONTOUR_52,
    },
    HelpLabelContour {
        start: [13626, 865],
        segments: &BODY_CONTOUR_53,
    },
    HelpLabelContour {
        start: [13587, 630],
        segments: &BODY_CONTOUR_54,
    },
    HelpLabelContour {
        start: [14300, 553],
        segments: &BODY_CONTOUR_55,
    },
    HelpLabelContour {
        start: [15191, 660],
        segments: &BODY_CONTOUR_56,
    },
    HelpLabelContour {
        start: [15751, 865],
        segments: &BODY_CONTOUR_57,
    },
    HelpLabelContour {
        start: [15712, 630],
        segments: &BODY_CONTOUR_58,
    },
    HelpLabelContour {
        start: [16431, 553],
        segments: &BODY_CONTOUR_59,
    },
    HelpLabelContour {
        start: [16431, 688],
        segments: &BODY_CONTOUR_60,
    },
    HelpLabelContour {
        start: [17101, 553],
        segments: &BODY_CONTOUR_61,
    },
    HelpLabelContour {
        start: [17101, 688],
        segments: &BODY_CONTOUR_62,
    },
    HelpLabelContour {
        start: [17736, 865],
        segments: &BODY_CONTOUR_63,
    },
    HelpLabelContour {
        start: [17697, 630],
        segments: &BODY_CONTOUR_64,
    },
    HelpLabelContour {
        start: [18802, 792],
        segments: &BODY_CONTOUR_65,
    },
    HelpLabelContour {
        start: [19217, 635],
        segments: &BODY_CONTOUR_66,
    },
    HelpLabelContour {
        start: [19096, 1046],
        segments: &BODY_CONTOUR_67,
    },
    HelpLabelContour {
        start: [20112, 792],
        segments: &BODY_CONTOUR_68,
    },
    HelpLabelContour {
        start: [20310, 553],
        segments: &BODY_CONTOUR_69,
    },
    HelpLabelContour {
        start: [21083, 585],
        segments: &BODY_CONTOUR_70,
    },
    HelpLabelContour {
        start: [22068, 581],
        segments: &BODY_CONTOUR_71,
    },
    HelpLabelContour {
        start: [21803, 630],
        segments: &BODY_CONTOUR_72,
    },
    HelpLabelContour {
        start: [22266, 865],
        segments: &BODY_CONTOUR_73,
    },
    HelpLabelContour {
        start: [22227, 630],
        segments: &BODY_CONTOUR_74,
    },
    HelpLabelContour {
        start: [23029, 541],
        segments: &BODY_CONTOUR_75,
    },
    HelpLabelContour {
        start: [23104, 860],
        segments: &BODY_CONTOUR_76,
    },
    HelpLabelContour {
        start: [23690, 1092],
        segments: &BODY_CONTOUR_77,
    },
    HelpLabelContour {
        start: [23834, 884],
        segments: &BODY_CONTOUR_78,
    },
    HelpLabelContour {
        start: [24485, 1092],
        segments: &BODY_CONTOUR_79,
    },
    HelpLabelContour {
        start: [24730, 315],
        segments: &BODY_CONTOUR_80,
    },
    HelpLabelContour {
        start: [24780, 553],
        segments: &BODY_CONTOUR_81,
    },
    HelpLabelContour {
        start: [25452, 792],
        segments: &BODY_CONTOUR_82,
    },
    HelpLabelContour {
        start: [25867, 635],
        segments: &BODY_CONTOUR_83,
    },
    HelpLabelContour {
        start: [25746, 1046],
        segments: &BODY_CONTOUR_84,
    },
    HelpLabelContour {
        start: [26392, 1016],
        segments: &BODY_CONTOUR_85,
    },
    HelpLabelContour {
        start: [-11170, 2107],
        segments: &BODY_CONTOUR_86,
    },
    HelpLabelContour {
        start: [-10593, 2716],
        segments: &BODY_CONTOUR_87,
    },
    HelpLabelContour {
        start: [-9420, 2226],
        segments: &BODY_CONTOUR_88,
    },
    HelpLabelContour {
        start: [-9795, 2116],
        segments: &BODY_CONTOUR_89,
    },
    HelpLabelContour {
        start: [-9795, 2453],
        segments: &BODY_CONTOUR_90,
    },
    HelpLabelContour {
        start: [-9125, 2015],
        segments: &BODY_CONTOUR_91,
    },
    HelpLabelContour {
        start: [-9075, 2253],
        segments: &BODY_CONTOUR_92,
    },
    HelpLabelContour {
        start: [-8422, 2281],
        segments: &BODY_CONTOUR_93,
    },
    HelpLabelContour {
        start: [-8687, 2330],
        segments: &BODY_CONTOUR_94,
    },
    HelpLabelContour {
        start: [-7829, 1995],
        segments: &BODY_CONTOUR_95,
    },
    HelpLabelContour {
        start: [-7829, 2384],
        segments: &BODY_CONTOUR_96,
    },
    HelpLabelContour {
        start: [-7076, 2241],
        segments: &BODY_CONTOUR_97,
    },
    HelpLabelContour {
        start: [-7001, 2560],
        segments: &BODY_CONTOUR_98,
    },
    HelpLabelContour {
        start: [-6415, 2792],
        segments: &BODY_CONTOUR_99,
    },
    HelpLabelContour {
        start: [-6060, 2792],
        segments: &BODY_CONTOUR_100,
    },
    HelpLabelContour {
        start: [-5592, 2285],
        segments: &BODY_CONTOUR_101,
    },
    HelpLabelContour {
        start: [-4747, 2285],
        segments: &BODY_CONTOUR_102,
    },
    HelpLabelContour {
        start: [-4211, 2261],
        segments: &BODY_CONTOUR_103,
    },
    HelpLabelContour {
        start: [-3929, 2565],
        segments: &BODY_CONTOUR_104,
    },
    HelpLabelContour {
        start: [-3968, 2330],
        segments: &BODY_CONTOUR_105,
    },
    HelpLabelContour {
        start: [-3019, 2360],
        segments: &BODY_CONTOUR_106,
    },
    HelpLabelContour {
        start: [-2638, 2335],
        segments: &BODY_CONTOUR_107,
    },
    HelpLabelContour {
        start: [-2759, 2746],
        segments: &BODY_CONTOUR_108,
    },
    HelpLabelContour {
        start: [-1096, 2299],
        segments: &BODY_CONTOUR_109,
    },
    HelpLabelContour {
        start: [-809, 2565],
        segments: &BODY_CONTOUR_110,
    },
    HelpLabelContour {
        start: [-848, 2330],
        segments: &BODY_CONTOUR_111,
    },
    HelpLabelContour {
        start: [101, 2360],
        segments: &BODY_CONTOUR_112,
    },
    HelpLabelContour {
        start: [483, 2335],
        segments: &BODY_CONTOUR_113,
    },
    HelpLabelContour {
        start: [361, 2746],
        segments: &BODY_CONTOUR_114,
    },
    HelpLabelContour {
        start: [1351, 2253],
        segments: &BODY_CONTOUR_115,
    },
    HelpLabelContour {
        start: [1351, 2388],
        segments: &BODY_CONTOUR_116,
    },
    HelpLabelContour {
        start: [1986, 2565],
        segments: &BODY_CONTOUR_117,
    },
    HelpLabelContour {
        start: [1947, 2330],
        segments: &BODY_CONTOUR_118,
    },
    HelpLabelContour {
        start: [2670, 2015],
        segments: &BODY_CONTOUR_119,
    },
    HelpLabelContour {
        start: [2720, 2253],
        segments: &BODY_CONTOUR_120,
    },
    HelpLabelContour {
        start: [3392, 2492],
        segments: &BODY_CONTOUR_121,
    },
    HelpLabelContour {
        start: [3590, 2253],
        segments: &BODY_CONTOUR_122,
    },
    HelpLabelContour {
        start: [4363, 2285],
        segments: &BODY_CONTOUR_123,
    },
    HelpLabelContour {
        start: [4910, 2253],
        segments: &BODY_CONTOUR_124,
    },
    HelpLabelContour {
        start: [5456, 1995],
        segments: &BODY_CONTOUR_125,
    },
    HelpLabelContour {
        start: [6194, 2241],
        segments: &BODY_CONTOUR_126,
    },
    HelpLabelContour {
        start: [6269, 2560],
        segments: &BODY_CONTOUR_127,
    },
    HelpLabelContour {
        start: [7127, 2492],
        segments: &BODY_CONTOUR_128,
    },
    HelpLabelContour {
        start: [7983, 2285],
        segments: &BODY_CONTOUR_129,
    },
    HelpLabelContour {
        start: [8914, 2299],
        segments: &BODY_CONTOUR_130,
    },
    HelpLabelContour {
        start: [9319, 2241],
        segments: &BODY_CONTOUR_131,
    },
    HelpLabelContour {
        start: [9394, 2560],
        segments: &BODY_CONTOUR_132,
    },
    HelpLabelContour {
        start: [9980, 2792],
        segments: &BODY_CONTOUR_133,
    },
    HelpLabelContour {
        start: [10335, 2792],
        segments: &BODY_CONTOUR_134,
    },
    HelpLabelContour {
        start: [10936, 1995],
        segments: &BODY_CONTOUR_135,
    },
    HelpLabelContour {
        start: [10936, 2384],
        segments: &BODY_CONTOUR_136,
    },
    HelpLabelContour {
        start: [11689, 2241],
        segments: &BODY_CONTOUR_137,
    },
    HelpLabelContour {
        start: [11764, 2560],
        segments: &BODY_CONTOUR_138,
    },
    HelpLabelContour {
        start: [12350, 2792],
        segments: &BODY_CONTOUR_139,
    },
    HelpLabelContour {
        start: [12705, 2792],
        segments: &BODY_CONTOUR_140,
    },
    HelpLabelContour {
        start: [13173, 2285],
        segments: &BODY_CONTOUR_141,
    },
    HelpLabelContour {
        start: [13447, 2716],
        segments: &BODY_CONTOUR_142,
    },
    HelpLabelContour {
        start: [-10978, 3767],
        segments: &BODY_CONTOUR_143,
    },
    HelpLabelContour {
        start: [-10593, 4416],
        segments: &BODY_CONTOUR_144,
    },
    HelpLabelContour {
        start: [-9420, 3926],
        segments: &BODY_CONTOUR_145,
    },
    HelpLabelContour {
        start: [-9795, 3816],
        segments: &BODY_CONTOUR_146,
    },
    HelpLabelContour {
        start: [-9795, 4153],
        segments: &BODY_CONTOUR_147,
    },
    HelpLabelContour {
        start: [-9046, 3941],
        segments: &BODY_CONTOUR_148,
    },
    HelpLabelContour {
        start: [-8971, 4260],
        segments: &BODY_CONTOUR_149,
    },
    HelpLabelContour {
        start: [-8385, 4492],
        segments: &BODY_CONTOUR_150,
    },
    HelpLabelContour {
        start: [-8030, 4492],
        segments: &BODY_CONTOUR_151,
    },
    HelpLabelContour {
        start: [-7562, 3985],
        segments: &BODY_CONTOUR_152,
    },
    HelpLabelContour {
        start: [-6861, 3941],
        segments: &BODY_CONTOUR_153,
    },
    HelpLabelContour {
        start: [-6786, 4260],
        segments: &BODY_CONTOUR_154,
    },
    HelpLabelContour {
        start: [-6084, 4060],
        segments: &BODY_CONTOUR_155,
    },
    HelpLabelContour {
        start: [-5702, 4035],
        segments: &BODY_CONTOUR_156,
    },
    HelpLabelContour {
        start: [-5824, 4446],
        segments: &BODY_CONTOUR_157,
    },
    HelpLabelContour {
        start: [-4607, 3985],
        segments: &BODY_CONTOUR_158,
    },
    HelpLabelContour {
        start: [-4355, 3953],
        segments: &BODY_CONTOUR_159,
    },
    HelpLabelContour {
        start: [-3694, 3695],
        segments: &BODY_CONTOUR_160,
    },
    HelpLabelContour {
        start: [-3694, 4084],
        segments: &BODY_CONTOUR_161,
    },
    HelpLabelContour {
        start: [-3012, 3735],
        segments: &BODY_CONTOUR_162,
    },
    HelpLabelContour {
        start: [-2893, 3953],
        segments: &BODY_CONTOUR_163,
    },
    HelpLabelContour {
        start: [-2438, 4035],
        segments: &BODY_CONTOUR_164,
    },
    HelpLabelContour {
        start: [-2559, 4446],
        segments: &BODY_CONTOUR_165,
    },
    HelpLabelContour {
        start: [-1651, 3961],
        segments: &BODY_CONTOUR_166,
    },
    HelpLabelContour {
        start: [-1405, 3953],
        segments: &BODY_CONTOUR_167,
    },
    HelpLabelContour {
        start: [-570, 3953],
        segments: &BODY_CONTOUR_168,
    },
    HelpLabelContour {
        start: [-59, 4265],
        segments: &BODY_CONTOUR_169,
    },
    HelpLabelContour {
        start: [-98, 4030],
        segments: &BODY_CONTOUR_170,
    },
    HelpLabelContour {
        start: [920, 3953],
        segments: &BODY_CONTOUR_171,
    },
    HelpLabelContour {
        start: [1391, 4265],
        segments: &BODY_CONTOUR_172,
    },
    HelpLabelContour {
        start: [1352, 4030],
        segments: &BODY_CONTOUR_173,
    },
    HelpLabelContour {
        start: [2301, 4060],
        segments: &BODY_CONTOUR_174,
    },
    HelpLabelContour {
        start: [2814, 3961],
        segments: &BODY_CONTOUR_175,
    },
    HelpLabelContour {
        start: [3277, 4035],
        segments: &BODY_CONTOUR_176,
    },
    HelpLabelContour {
        start: [3156, 4446],
        segments: &BODY_CONTOUR_177,
    },
    HelpLabelContour {
        start: [4013, 3985],
        segments: &BODY_CONTOUR_178,
    },
    HelpLabelContour {
        start: [4382, 4512],
        segments: &BODY_CONTOUR_179,
    },
    HelpLabelContour {
        start: [5000, 3953],
        segments: &BODY_CONTOUR_180,
    },
    HelpLabelContour {
        start: [5546, 3695],
        segments: &BODY_CONTOUR_181,
    },
    HelpLabelContour {
        start: [6348, 4035],
        segments: &BODY_CONTOUR_182,
    },
    HelpLabelContour {
        start: [6226, 4446],
        segments: &BODY_CONTOUR_183,
    },
    HelpLabelContour {
        start: [7299, 3941],
        segments: &BODY_CONTOUR_184,
    },
    HelpLabelContour {
        start: [7374, 4260],
        segments: &BODY_CONTOUR_185,
    },
    HelpLabelContour {
        start: [7775, 3953],
        segments: &BODY_CONTOUR_186,
    },
    HelpLabelContour {
        start: [8250, 3953],
        segments: &BODY_CONTOUR_187,
    },
    HelpLabelContour {
        start: [9026, 4060],
        segments: &BODY_CONTOUR_188,
    },
    HelpLabelContour {
        start: [9344, 3941],
        segments: &BODY_CONTOUR_189,
    },
    HelpLabelContour {
        start: [9419, 4260],
        segments: &BODY_CONTOUR_190,
    },
    HelpLabelContour {
        start: [10169, 3961],
        segments: &BODY_CONTOUR_191,
    },
    HelpLabelContour {
        start: [10415, 3953],
        segments: &BODY_CONTOUR_192,
    },
    HelpLabelContour {
        start: [11468, 4035],
        segments: &BODY_CONTOUR_193,
    },
    HelpLabelContour {
        start: [11346, 4446],
        segments: &BODY_CONTOUR_194,
    },
    HelpLabelContour {
        start: [12059, 3941],
        segments: &BODY_CONTOUR_195,
    },
    HelpLabelContour {
        start: [12134, 4260],
        segments: &BODY_CONTOUR_196,
    },
    HelpLabelContour {
        start: [12884, 3961],
        segments: &BODY_CONTOUR_197,
    },
    HelpLabelContour {
        start: [13201, 3695],
        segments: &BODY_CONTOUR_198,
    },
    HelpLabelContour {
        start: [14181, 4265],
        segments: &BODY_CONTOUR_199,
    },
    HelpLabelContour {
        start: [14142, 4030],
        segments: &BODY_CONTOUR_200,
    },
    HelpLabelContour {
        start: [14790, 3953],
        segments: &BODY_CONTOUR_201,
    },
    HelpLabelContour {
        start: [15336, 3695],
        segments: &BODY_CONTOUR_202,
    },
    HelpLabelContour {
        start: [16138, 4035],
        segments: &BODY_CONTOUR_203,
    },
    HelpLabelContour {
        start: [16016, 4446],
        segments: &BODY_CONTOUR_204,
    },
    HelpLabelContour {
        start: [16876, 4060],
        segments: &BODY_CONTOUR_205,
    },
    HelpLabelContour {
        start: [16967, 4416],
        segments: &BODY_CONTOUR_206,
    },
    HelpLabelContour {
        start: [17407, 4416],
        segments: &BODY_CONTOUR_207,
    },
    HelpLabelContour {
        start: [17847, 4416],
        segments: &BODY_CONTOUR_208,
    },
    HelpLabelContour {
        start: [-9783, 6116],
        segments: &BODY_CONTOUR_209,
    },
    HelpLabelContour {
        start: [-9343, 6116],
        segments: &BODY_CONTOUR_210,
    },
    HelpLabelContour {
        start: [-8903, 6116],
        segments: &BODY_CONTOUR_211,
    },
    HelpLabelContour {
        start: [-8514, 5965],
        segments: &BODY_CONTOUR_212,
    },
    HelpLabelContour {
        start: [-8553, 5730],
        segments: &BODY_CONTOUR_213,
    },
    HelpLabelContour {
        start: [-7604, 5760],
        segments: &BODY_CONTOUR_214,
    },
    HelpLabelContour {
        start: [-6779, 5760],
        segments: &BODY_CONTOUR_215,
    },
    HelpLabelContour {
        start: [-6397, 5735],
        segments: &BODY_CONTOUR_216,
    },
    HelpLabelContour {
        start: [-6519, 6146],
        segments: &BODY_CONTOUR_217,
    },
    HelpLabelContour {
        start: [-5877, 5435],
        segments: &BODY_CONTOUR_218,
    },
    HelpLabelContour {
        start: [-5758, 5653],
        segments: &BODY_CONTOUR_219,
    },
    HelpLabelContour {
        start: [-5302, 5735],
        segments: &BODY_CONTOUR_220,
    },
    HelpLabelContour {
        start: [-5424, 6146],
        segments: &BODY_CONTOUR_221,
    },
    HelpLabelContour {
        start: [-4516, 5661],
        segments: &BODY_CONTOUR_222,
    },
    HelpLabelContour {
        start: [-4270, 5653],
        segments: &BODY_CONTOUR_223,
    },
    HelpLabelContour {
        start: [-3217, 5735],
        segments: &BODY_CONTOUR_224,
    },
    HelpLabelContour {
        start: [-3339, 6146],
        segments: &BODY_CONTOUR_225,
    },
    HelpLabelContour {
        start: [-2626, 5641],
        segments: &BODY_CONTOUR_226,
    },
    HelpLabelContour {
        start: [-2551, 5960],
        segments: &BODY_CONTOUR_227,
    },
    HelpLabelContour {
        start: [-2150, 5653],
        segments: &BODY_CONTOUR_228,
    },
    HelpLabelContour {
        start: [-1326, 5661],
        segments: &BODY_CONTOUR_229,
    },
    HelpLabelContour {
        start: [-1009, 5395],
        segments: &BODY_CONTOUR_230,
    },
    HelpLabelContour {
        start: [-29, 5965],
        segments: &BODY_CONTOUR_231,
    },
    HelpLabelContour {
        start: [-68, 5730],
        segments: &BODY_CONTOUR_232,
    },
    HelpLabelContour {
        start: [580, 5653],
        segments: &BODY_CONTOUR_233,
    },
    HelpLabelContour {
        start: [1126, 5395],
        segments: &BODY_CONTOUR_234,
    },
    HelpLabelContour {
        start: [1927, 5735],
        segments: &BODY_CONTOUR_235,
    },
    HelpLabelContour {
        start: [1806, 6146],
        segments: &BODY_CONTOUR_236,
    },
    HelpLabelContour {
        start: [2666, 5760],
        segments: &BODY_CONTOUR_237,
    },
    HelpLabelContour {
        start: [3164, 5984],
        segments: &BODY_CONTOUR_238,
    },
    HelpLabelContour {
        start: [3990, 5653],
        segments: &BODY_CONTOUR_239,
    },
    HelpLabelContour {
        start: [4536, 5395],
        segments: &BODY_CONTOUR_240,
    },
    HelpLabelContour {
        start: [5274, 5641],
        segments: &BODY_CONTOUR_241,
    },
    HelpLabelContour {
        start: [5349, 5960],
        segments: &BODY_CONTOUR_242,
    },
    HelpLabelContour {
        start: [5750, 5653],
        segments: &BODY_CONTOUR_243,
    },
    HelpLabelContour {
        start: [6942, 5395],
        segments: &BODY_CONTOUR_244,
    },
    HelpLabelContour {
        start: [6896, 6182],
        segments: &BODY_CONTOUR_245,
    },
    HelpLabelContour {
        start: [7473, 5735],
        segments: &BODY_CONTOUR_246,
    },
    HelpLabelContour {
        start: [7351, 6146],
        segments: &BODY_CONTOUR_247,
    },
    HelpLabelContour {
        start: [7981, 5653],
        segments: &BODY_CONTOUR_248,
    },
    HelpLabelContour {
        start: [7981, 5788],
        segments: &BODY_CONTOUR_249,
    },
    HelpLabelContour {
        start: [8798, 5735],
        segments: &BODY_CONTOUR_250,
    },
    HelpLabelContour {
        start: [8676, 6146],
        segments: &BODY_CONTOUR_251,
    },
    HelpLabelContour {
        start: [9692, 5892],
        segments: &BODY_CONTOUR_252,
    },
    HelpLabelContour {
        start: [10247, 5395],
        segments: &BODY_CONTOUR_253,
    },
    HelpLabelContour {
        start: [10201, 6182],
        segments: &BODY_CONTOUR_254,
    },
    HelpLabelContour {
        start: [10858, 5685],
        segments: &BODY_CONTOUR_255,
    },
    HelpLabelContour {
        start: [11441, 5965],
        segments: &BODY_CONTOUR_256,
    },
    HelpLabelContour {
        start: [11402, 5730],
        segments: &BODY_CONTOUR_257,
    },
    HelpLabelContour {
        start: [12507, 5892],
        segments: &BODY_CONTOUR_258,
    },
    HelpLabelContour {
        start: [13065, 5653],
        segments: &BODY_CONTOUR_259,
    },
    HelpLabelContour {
        start: [13611, 5395],
        segments: &BODY_CONTOUR_260,
    },
    HelpLabelContour {
        start: [14412, 5735],
        segments: &BODY_CONTOUR_261,
    },
    HelpLabelContour {
        start: [14291, 6146],
        segments: &BODY_CONTOUR_262,
    },
    HelpLabelContour {
        start: [15648, 5681],
        segments: &BODY_CONTOUR_263,
    },
    HelpLabelContour {
        start: [15383, 5730],
        segments: &BODY_CONTOUR_264,
    },
    HelpLabelContour {
        start: [16111, 5760],
        segments: &BODY_CONTOUR_265,
    },
    HelpLabelContour {
        start: [16429, 5641],
        segments: &BODY_CONTOUR_266,
    },
    HelpLabelContour {
        start: [16504, 5960],
        segments: &BODY_CONTOUR_267,
    },
    HelpLabelContour {
        start: [17366, 5651],
        segments: &BODY_CONTOUR_268,
    },
    HelpLabelContour {
        start: [17570, 5415],
        segments: &BODY_CONTOUR_269,
    },
    HelpLabelContour {
        start: [17620, 5653],
        segments: &BODY_CONTOUR_270,
    },
    HelpLabelContour {
        start: [17835, 5653],
        segments: &BODY_CONTOUR_271,
    },
    HelpLabelContour {
        start: [18775, 5653],
        segments: &BODY_CONTOUR_272,
    },
    HelpLabelContour {
        start: [20004, 5699],
        segments: &BODY_CONTOUR_273,
    },
    HelpLabelContour {
        start: [20291, 5965],
        segments: &BODY_CONTOUR_274,
    },
    HelpLabelContour {
        start: [20252, 5730],
        segments: &BODY_CONTOUR_275,
    },
    HelpLabelContour {
        start: [21257, 5395],
        segments: &BODY_CONTOUR_276,
    },
    HelpLabelContour {
        start: [21211, 6182],
        segments: &BODY_CONTOUR_277,
    },
    HelpLabelContour {
        start: [21787, 5735],
        segments: &BODY_CONTOUR_278,
    },
    HelpLabelContour {
        start: [21666, 6146],
        segments: &BODY_CONTOUR_279,
    },
    HelpLabelContour {
        start: [23050, 5653],
        segments: &BODY_CONTOUR_280,
    },
    HelpLabelContour {
        start: [23211, 5965],
        segments: &BODY_CONTOUR_281,
    },
    HelpLabelContour {
        start: [23172, 5730],
        segments: &BODY_CONTOUR_282,
    },
    HelpLabelContour {
        start: [23885, 5653],
        segments: &BODY_CONTOUR_283,
    },
    HelpLabelContour {
        start: [24906, 5653],
        segments: &BODY_CONTOUR_284,
    },
    HelpLabelContour {
        start: [24906, 5788],
        segments: &BODY_CONTOUR_285,
    },
    HelpLabelContour {
        start: [25690, 6192],
        segments: &BODY_CONTOUR_286,
    },
    HelpLabelContour {
        start: [26014, 5641],
        segments: &BODY_CONTOUR_287,
    },
    HelpLabelContour {
        start: [26089, 5960],
        segments: &BODY_CONTOUR_288,
    },
    HelpLabelContour {
        start: [26955, 5653],
        segments: &BODY_CONTOUR_289,
    },
    HelpLabelContour {
        start: [27022, 6116],
        segments: &BODY_CONTOUR_290,
    },
    HelpLabelContour {
        start: [-10913, 7119],
        segments: &BODY_CONTOUR_291,
    },
    HelpLabelContour {
        start: [-11025, 7657],
        segments: &BODY_CONTOUR_292,
    },
    HelpLabelContour {
        start: [-10593, 7816],
        segments: &BODY_CONTOUR_293,
    },
    HelpLabelContour {
        start: [-9619, 7228],
        segments: &BODY_CONTOUR_294,
    },
    HelpLabelContour {
        start: [-9319, 7665],
        segments: &BODY_CONTOUR_295,
    },
    HelpLabelContour {
        start: [-9358, 7430],
        segments: &BODY_CONTOUR_296,
    },
    HelpLabelContour {
        start: [-8645, 7353],
        segments: &BODY_CONTOUR_297,
    },
    HelpLabelContour {
        start: [-7706, 7361],
        segments: &BODY_CONTOUR_298,
    },
    HelpLabelContour {
        start: [-7389, 7095],
        segments: &BODY_CONTOUR_299,
    },
    HelpLabelContour {
        start: [-6730, 7115],
        segments: &BODY_CONTOUR_300,
    },
    HelpLabelContour {
        start: [-6680, 7353],
        segments: &BODY_CONTOUR_301,
    },
    HelpLabelContour {
        start: [-6008, 7592],
        segments: &BODY_CONTOUR_302,
    },
    HelpLabelContour {
        start: [-5372, 7381],
        segments: &BODY_CONTOUR_303,
    },
    HelpLabelContour {
        start: [-5637, 7430],
        segments: &BODY_CONTOUR_304,
    },
    HelpLabelContour {
        start: [-4779, 7095],
        segments: &BODY_CONTOUR_305,
    },
    HelpLabelContour {
        start: [-4779, 7484],
        segments: &BODY_CONTOUR_306,
    },
    HelpLabelContour {
        start: [-4026, 7341],
        segments: &BODY_CONTOUR_307,
    },
    HelpLabelContour {
        start: [-3951, 7660],
        segments: &BODY_CONTOUR_308,
    },
    HelpLabelContour {
        start: [-3365, 7892],
        segments: &BODY_CONTOUR_309,
    },
    HelpLabelContour {
        start: [-3010, 7892],
        segments: &BODY_CONTOUR_310,
    },
    HelpLabelContour {
        start: [-2542, 7385],
        segments: &BODY_CONTOUR_311,
    },
    HelpLabelContour {
        start: [-1251, 7399],
        segments: &BODY_CONTOUR_312,
    },
    HelpLabelContour {
        start: [-783, 7435],
        segments: &BODY_CONTOUR_313,
    },
    HelpLabelContour {
        start: [-904, 7846],
        segments: &BODY_CONTOUR_314,
    },
    HelpLabelContour {
        start: [-44, 7460],
        segments: &BODY_CONTOUR_315,
    },
    HelpLabelContour {
        start: [558, 7381],
        segments: &BODY_CONTOUR_316,
    },
    HelpLabelContour {
        start: [293, 7430],
        segments: &BODY_CONTOUR_317,
    },
    HelpLabelContour {
        start: [937, 7435],
        segments: &BODY_CONTOUR_318,
    },
    HelpLabelContour {
        start: [816, 7846],
        segments: &BODY_CONTOUR_319,
    },
    HelpLabelContour {
        start: [1810, 7115],
        segments: &BODY_CONTOUR_320,
    },
    HelpLabelContour {
        start: [1860, 7353],
        segments: &BODY_CONTOUR_321,
    },
    HelpLabelContour {
        start: [2532, 7592],
        segments: &BODY_CONTOUR_322,
    },
    HelpLabelContour {
        start: [2730, 7353],
        segments: &BODY_CONTOUR_323,
    },
    HelpLabelContour {
        start: [3241, 7665],
        segments: &BODY_CONTOUR_324,
    },
    HelpLabelContour {
        start: [3202, 7430],
        segments: &BODY_CONTOUR_325,
    },
    HelpLabelContour {
        start: [4281, 7095],
        segments: &BODY_CONTOUR_326,
    },
    HelpLabelContour {
        start: [4281, 7484],
        segments: &BODY_CONTOUR_327,
    },
    HelpLabelContour {
        start: [4955, 7115],
        segments: &BODY_CONTOUR_328,
    },
    HelpLabelContour {
        start: [5005, 7353],
        segments: &BODY_CONTOUR_329,
    },
    HelpLabelContour {
        start: [5658, 7381],
        segments: &BODY_CONTOUR_330,
    },
    HelpLabelContour {
        start: [5393, 7430],
        segments: &BODY_CONTOUR_331,
    },
    HelpLabelContour {
        start: [6258, 7381],
        segments: &BODY_CONTOUR_332,
    },
    HelpLabelContour {
        start: [5993, 7430],
        segments: &BODY_CONTOUR_333,
    },
    HelpLabelContour {
        start: [6637, 7435],
        segments: &BODY_CONTOUR_334,
    },
    HelpLabelContour {
        start: [6516, 7846],
        segments: &BODY_CONTOUR_335,
    },
    HelpLabelContour {
        start: [7376, 7460],
        segments: &BODY_CONTOUR_336,
    },
    HelpLabelContour {
        start: [7936, 7665],
        segments: &BODY_CONTOUR_337,
    },
    HelpLabelContour {
        start: [7897, 7430],
        segments: &BODY_CONTOUR_338,
    },
    HelpLabelContour {
        start: [9002, 7592],
        segments: &BODY_CONTOUR_339,
    },
    HelpLabelContour {
        start: [9418, 7435],
        segments: &BODY_CONTOUR_340,
    },
    HelpLabelContour {
        start: [9296, 7846],
        segments: &BODY_CONTOUR_341,
    },
    HelpLabelContour {
        start: [10153, 7385],
        segments: &BODY_CONTOUR_342,
    },
    HelpLabelContour {
        start: [10427, 7816],
        segments: &BODY_CONTOUR_343,
    },
    HelpLabelContour {
        start: [-10906, 8918],
        segments: &BODY_CONTOUR_344,
    },
    HelpLabelContour {
        start: [-10593, 9516],
        segments: &BODY_CONTOUR_345,
    },
    HelpLabelContour {
        start: [-9303, 8822],
        segments: &BODY_CONTOUR_346,
    },
    HelpLabelContour {
        start: [-9032, 9135],
        segments: &BODY_CONTOUR_347,
    },
    HelpLabelContour {
        start: [-9154, 9546],
        segments: &BODY_CONTOUR_348,
    },
    HelpLabelContour {
        start: [-8294, 9160],
        segments: &BODY_CONTOUR_349,
    },
    HelpLabelContour {
        start: [-7665, 9053],
        segments: &BODY_CONTOUR_350,
    },
    HelpLabelContour {
        start: [-7109, 8795],
        segments: &BODY_CONTOUR_351,
    },
    HelpLabelContour {
        start: [-7109, 9184],
        segments: &BODY_CONTOUR_352,
    },
    HelpLabelContour {
        start: [-6435, 8815],
        segments: &BODY_CONTOUR_353,
    },
    HelpLabelContour {
        start: [-6385, 9053],
        segments: &BODY_CONTOUR_354,
    },
    HelpLabelContour {
        start: [-5732, 9081],
        segments: &BODY_CONTOUR_355,
    },
    HelpLabelContour {
        start: [-5997, 9130],
        segments: &BODY_CONTOUR_356,
    },
    HelpLabelContour {
        start: [-5139, 8795],
        segments: &BODY_CONTOUR_357,
    },
    HelpLabelContour {
        start: [-5139, 9184],
        segments: &BODY_CONTOUR_358,
    },
    HelpLabelContour {
        start: [-4386, 9041],
        segments: &BODY_CONTOUR_359,
    },
    HelpLabelContour {
        start: [-4311, 9360],
        segments: &BODY_CONTOUR_360,
    },
    HelpLabelContour {
        start: [-3725, 9592],
        segments: &BODY_CONTOUR_361,
    },
    HelpLabelContour {
        start: [-3370, 9592],
        segments: &BODY_CONTOUR_362,
    },
    HelpLabelContour {
        start: [-2902, 9085],
        segments: &BODY_CONTOUR_363,
    },
    HelpLabelContour {
        start: [-2057, 9085],
        segments: &BODY_CONTOUR_364,
    },
    HelpLabelContour {
        start: [-1870, 9053],
        segments: &BODY_CONTOUR_365,
    },
    HelpLabelContour {
        start: [-1241, 9041],
        segments: &BODY_CONTOUR_366,
    },
    HelpLabelContour {
        start: [-1166, 9360],
        segments: &BODY_CONTOUR_367,
    },
    HelpLabelContour {
        start: [-464, 9160],
        segments: &BODY_CONTOUR_368,
    },
    HelpLabelContour {
        start: [-300, 9053],
        segments: &BODY_CONTOUR_369,
    },
    HelpLabelContour {
        start: [535, 9053],
        segments: &BODY_CONTOUR_370,
    },
    HelpLabelContour {
        start: [1046, 9365],
        segments: &BODY_CONTOUR_371,
    },
    HelpLabelContour {
        start: [1007, 9130],
        segments: &BODY_CONTOUR_372,
    },
    HelpLabelContour {
        start: [2086, 8795],
        segments: &BODY_CONTOUR_373,
    },
    HelpLabelContour {
        start: [2086, 9184],
        segments: &BODY_CONTOUR_374,
    },
    HelpLabelContour {
        start: [2750, 9053],
        segments: &BODY_CONTOUR_375,
    },
    HelpLabelContour {
        start: [3641, 9160],
        segments: &BODY_CONTOUR_376,
    },
    HelpLabelContour {
        start: [4262, 9292],
        segments: &BODY_CONTOUR_377,
    },
    HelpLabelContour {
        start: [4794, 9384],
        segments: &BODY_CONTOUR_378,
    },
    HelpLabelContour {
        start: [5977, 8795],
        segments: &BODY_CONTOUR_379,
    },
    HelpLabelContour {
        start: [5931, 9582],
        segments: &BODY_CONTOUR_380,
    },
    HelpLabelContour {
        start: [6326, 9365],
        segments: &BODY_CONTOUR_381,
    },
    HelpLabelContour {
        start: [6287, 9130],
        segments: &BODY_CONTOUR_382,
    },
    HelpLabelContour {
        start: [7392, 9292],
        segments: &BODY_CONTOUR_383,
    },
    HelpLabelContour {
        start: [7605, 9044],
        segments: &BODY_CONTOUR_384,
    },
    HelpLabelContour {
        start: [7780, 9053],
        segments: &BODY_CONTOUR_385,
    },
    HelpLabelContour {
        start: [8615, 9053],
        segments: &BODY_CONTOUR_386,
    },
    HelpLabelContour {
        start: [9126, 9365],
        segments: &BODY_CONTOUR_387,
    },
    HelpLabelContour {
        start: [9087, 9130],
        segments: &BODY_CONTOUR_388,
    },
    HelpLabelContour {
        start: [9800, 9053],
        segments: &BODY_CONTOUR_389,
    },
    HelpLabelContour {
        start: [10739, 9061],
        segments: &BODY_CONTOUR_390,
    },
    HelpLabelContour {
        start: [11056, 8795],
        segments: &BODY_CONTOUR_391,
    },
    HelpLabelContour {
        start: [12000, 9053],
        segments: &BODY_CONTOUR_392,
    },
    HelpLabelContour {
        start: [12546, 8795],
        segments: &BODY_CONTOUR_393,
    },
    HelpLabelContour {
        start: [13347, 9135],
        segments: &BODY_CONTOUR_394,
    },
    HelpLabelContour {
        start: [13226, 9546],
        segments: &BODY_CONTOUR_395,
    },
    HelpLabelContour {
        start: [14529, 9099],
        segments: &BODY_CONTOUR_396,
    },
    HelpLabelContour {
        start: [14962, 9612],
        segments: &BODY_CONTOUR_397,
    },
    HelpLabelContour {
        start: [15580, 9053],
        segments: &BODY_CONTOUR_398,
    },
    HelpLabelContour {
        start: [16126, 8795],
        segments: &BODY_CONTOUR_399,
    },
    HelpLabelContour {
        start: [16927, 9135],
        segments: &BODY_CONTOUR_400,
    },
    HelpLabelContour {
        start: [16806, 9546],
        segments: &BODY_CONTOUR_401,
    },
    HelpLabelContour {
        start: [17830, 9053],
        segments: &BODY_CONTOUR_402,
    },
    HelpLabelContour {
        start: [18386, 9053],
        segments: &BODY_CONTOUR_403,
    },
    HelpLabelContour {
        start: [18386, 9188],
        segments: &BODY_CONTOUR_404,
    },
    HelpLabelContour {
        start: [19139, 9041],
        segments: &BODY_CONTOUR_405,
    },
    HelpLabelContour {
        start: [19214, 9360],
        segments: &BODY_CONTOUR_406,
    },
    HelpLabelContour {
        start: [19916, 9160],
        segments: &BODY_CONTOUR_407,
    },
    HelpLabelContour {
        start: [20234, 9041],
        segments: &BODY_CONTOUR_408,
    },
    HelpLabelContour {
        start: [20309, 9360],
        segments: &BODY_CONTOUR_409,
    },
    HelpLabelContour {
        start: [20895, 9592],
        segments: &BODY_CONTOUR_410,
    },
    HelpLabelContour {
        start: [21530, 9053],
        segments: &BODY_CONTOUR_411,
    },
    HelpLabelContour {
        start: [21733, 9580],
        segments: &BODY_CONTOUR_412,
    },
    HelpLabelContour {
        start: [22442, 9135],
        segments: &BODY_CONTOUR_413,
    },
    HelpLabelContour {
        start: [22321, 9546],
        segments: &BODY_CONTOUR_414,
    },
    HelpLabelContour {
        start: [23705, 9053],
        segments: &BODY_CONTOUR_415,
    },
    HelpLabelContour {
        start: [23866, 9365],
        segments: &BODY_CONTOUR_416,
    },
    HelpLabelContour {
        start: [23827, 9130],
        segments: &BODY_CONTOUR_417,
    },
    HelpLabelContour {
        start: [24540, 9053],
        segments: &BODY_CONTOUR_418,
    },
    HelpLabelContour {
        start: [25217, 9516],
        segments: &BODY_CONTOUR_419,
    },
    HelpLabelContour {
        start: [-10979, 10569],
        segments: &BODY_CONTOUR_420,
    },
    HelpLabelContour {
        start: [-11258, 11112],
        segments: &BODY_CONTOUR_421,
    },
    HelpLabelContour {
        start: [-10593, 11216],
        segments: &BODY_CONTOUR_422,
    },
    HelpLabelContour {
        start: [-9774, 10522],
        segments: &BODY_CONTOUR_423,
    },
    HelpLabelContour {
        start: [-9088, 10992],
        segments: &BODY_CONTOUR_424,
    },
    HelpLabelContour {
        start: [-8473, 10788],
        segments: &BODY_CONTOUR_425,
    },
    HelpLabelContour {
        start: [-7884, 10753],
        segments: &BODY_CONTOUR_426,
    },
    HelpLabelContour {
        start: [-7884, 10888],
        segments: &BODY_CONTOUR_427,
    },
    HelpLabelContour {
        start: [-7068, 10835],
        segments: &BODY_CONTOUR_428,
    },
    HelpLabelContour {
        start: [-7189, 11246],
        segments: &BODY_CONTOUR_429,
    },
    HelpLabelContour {
        start: [-6412, 10835],
        segments: &BODY_CONTOUR_430,
    },
    HelpLabelContour {
        start: [-6534, 11246],
        segments: &BODY_CONTOUR_431,
    },
    HelpLabelContour {
        start: [-5618, 10495],
        segments: &BODY_CONTOUR_432,
    },
    HelpLabelContour {
        start: [-5664, 11282],
        segments: &BODY_CONTOUR_433,
    },
    HelpLabelContour {
        start: [-4791, 10678],
        segments: &BODY_CONTOUR_434,
    },
    HelpLabelContour {
        start: [-4194, 10860],
        segments: &BODY_CONTOUR_435,
    },
    HelpLabelContour {
        start: [-3876, 10741],
        segments: &BODY_CONTOUR_436,
    },
    HelpLabelContour {
        start: [-3801, 11060],
        segments: &BODY_CONTOUR_437,
    },
    HelpLabelContour {
        start: [-2939, 10751],
        segments: &BODY_CONTOUR_438,
    },
    HelpLabelContour {
        start: [-2735, 10515],
        segments: &BODY_CONTOUR_439,
    },
    HelpLabelContour {
        start: [-2685, 10753],
        segments: &BODY_CONTOUR_440,
    },
    HelpLabelContour {
        start: [-2470, 10753],
        segments: &BODY_CONTOUR_441,
    },
    HelpLabelContour {
        start: [-1530, 10753],
        segments: &BODY_CONTOUR_442,
    },
    HelpLabelContour {
        start: [-1431, 11084],
        segments: &BODY_CONTOUR_443,
    },
    HelpLabelContour {
        start: [-414, 10522],
        segments: &BODY_CONTOUR_444,
    },
    HelpLabelContour {
        start: [-79, 11065],
        segments: &BODY_CONTOUR_445,
    },
    HelpLabelContour {
        start: [-118, 10830],
        segments: &BODY_CONTOUR_446,
    },
    HelpLabelContour {
        start: [887, 10495],
        segments: &BODY_CONTOUR_447,
    },
    HelpLabelContour {
        start: [841, 11282],
        segments: &BODY_CONTOUR_448,
    },
    HelpLabelContour {
        start: [1418, 10835],
        segments: &BODY_CONTOUR_449,
    },
    HelpLabelContour {
        start: [1296, 11246],
        segments: &BODY_CONTOUR_450,
    },
    HelpLabelContour {
        start: [2225, 10753],
        segments: &BODY_CONTOUR_451,
    },
    HelpLabelContour {
        start: [2814, 10741],
        segments: &BODY_CONTOUR_452,
    },
    HelpLabelContour {
        start: [2889, 11060],
        segments: &BODY_CONTOUR_453,
    },
    HelpLabelContour {
        start: [3588, 10785],
        segments: &BODY_CONTOUR_454,
    },
    HelpLabelContour {
        start: [3775, 10753],
        segments: &BODY_CONTOUR_455,
    },
    HelpLabelContour {
        start: [4468, 10835],
        segments: &BODY_CONTOUR_456,
    },
    HelpLabelContour {
        start: [4346, 11246],
        segments: &BODY_CONTOUR_457,
    },
    HelpLabelContour {
        start: [5206, 10860],
        segments: &BODY_CONTOUR_458,
    },
    HelpLabelContour {
        start: [5801, 10495],
        segments: &BODY_CONTOUR_459,
    },
    HelpLabelContour {
        start: [5801, 10884],
        segments: &BODY_CONTOUR_460,
    },
    HelpLabelContour {
        start: [6554, 10741],
        segments: &BODY_CONTOUR_461,
    },
    HelpLabelContour {
        start: [6629, 11060],
        segments: &BODY_CONTOUR_462,
    },
    HelpLabelContour {
        start: [7215, 11292],
        segments: &BODY_CONTOUR_463,
    },
    HelpLabelContour {
        start: [7570, 11292],
        segments: &BODY_CONTOUR_464,
    },
    HelpLabelContour {
        start: [8038, 10785],
        segments: &BODY_CONTOUR_465,
    },
    HelpLabelContour {
        start: [8883, 10785],
        segments: &BODY_CONTOUR_466,
    },
    HelpLabelContour {
        start: [9419, 10761],
        segments: &BODY_CONTOUR_467,
    },
    HelpLabelContour {
        start: [9701, 11065],
        segments: &BODY_CONTOUR_468,
    },
    HelpLabelContour {
        start: [9662, 10830],
        segments: &BODY_CONTOUR_469,
    },
    HelpLabelContour {
        start: [10611, 10860],
        segments: &BODY_CONTOUR_470,
    },
    HelpLabelContour {
        start: [10993, 10835],
        segments: &BODY_CONTOUR_471,
    },
    HelpLabelContour {
        start: [10871, 11246],
        segments: &BODY_CONTOUR_472,
    },
    HelpLabelContour {
        start: [12534, 10799],
        segments: &BODY_CONTOUR_473,
    },
    HelpLabelContour {
        start: [12821, 11065],
        segments: &BODY_CONTOUR_474,
    },
    HelpLabelContour {
        start: [12782, 10830],
        segments: &BODY_CONTOUR_475,
    },
    HelpLabelContour {
        start: [13731, 10860],
        segments: &BODY_CONTOUR_476,
    },
    HelpLabelContour {
        start: [14112, 10835],
        segments: &BODY_CONTOUR_477,
    },
    HelpLabelContour {
        start: [13991, 11246],
        segments: &BODY_CONTOUR_478,
    },
    HelpLabelContour {
        start: [-9716, 12441],
        segments: &BODY_CONTOUR_479,
    },
    HelpLabelContour {
        start: [-9641, 12760],
        segments: &BODY_CONTOUR_480,
    },
    HelpLabelContour {
        start: [-8783, 12692],
        segments: &BODY_CONTOUR_481,
    },
    HelpLabelContour {
        start: [-8228, 12195],
        segments: &BODY_CONTOUR_482,
    },
    HelpLabelContour {
        start: [-8274, 12982],
        segments: &BODY_CONTOUR_483,
    },
    HelpLabelContour {
        start: [-7555, 12453],
        segments: &BODY_CONTOUR_484,
    },
    HelpLabelContour {
        start: [-7009, 12195],
        segments: &BODY_CONTOUR_485,
    },
    HelpLabelContour {
        start: [-6208, 12535],
        segments: &BODY_CONTOUR_486,
    },
    HelpLabelContour {
        start: [-6329, 12946],
        segments: &BODY_CONTOUR_487,
    },
    HelpLabelContour {
        start: [-5339, 12195],
        segments: &BODY_CONTOUR_488,
    },
    HelpLabelContour {
        start: [-5339, 12584],
        segments: &BODY_CONTOUR_489,
    },
    HelpLabelContour {
        start: [-4586, 12441],
        segments: &BODY_CONTOUR_490,
    },
    HelpLabelContour {
        start: [-4511, 12760],
        segments: &BODY_CONTOUR_491,
    },
    HelpLabelContour {
        start: [-3925, 12992],
        segments: &BODY_CONTOUR_492,
    },
    HelpLabelContour {
        start: [-3570, 12992],
        segments: &BODY_CONTOUR_493,
    },
    HelpLabelContour {
        start: [-3102, 12485],
        segments: &BODY_CONTOUR_494,
    },
    HelpLabelContour {
        start: [-2555, 12453],
        segments: &BODY_CONTOUR_495,
    },
    HelpLabelContour {
        start: [-1926, 12441],
        segments: &BODY_CONTOUR_496,
    },
    HelpLabelContour {
        start: [-1851, 12760],
        segments: &BODY_CONTOUR_497,
    },
    HelpLabelContour {
        start: [-1210, 12691],
        segments: &BODY_CONTOUR_498,
    },
    HelpLabelContour {
        start: [-628, 12535],
        segments: &BODY_CONTOUR_499,
    },
    HelpLabelContour {
        start: [-749, 12946],
        segments: &BODY_CONTOUR_500,
    },
    HelpLabelContour {
        start: [235, 12453],
        segments: &BODY_CONTOUR_501,
    },
    HelpLabelContour {
        start: [896, 12453],
        segments: &BODY_CONTOUR_502,
    },
    HelpLabelContour {
        start: [896, 12588],
        segments: &BODY_CONTOUR_503,
    },
    HelpLabelContour {
        start: [2153, 12485],
        segments: &BODY_CONTOUR_504,
    },
    HelpLabelContour {
        start: [2411, 12453],
        segments: &BODY_CONTOUR_505,
    },
    HelpLabelContour {
        start: [2411, 12588],
        segments: &BODY_CONTOUR_506,
    },
    HelpLabelContour {
        start: [3227, 12535],
        segments: &BODY_CONTOUR_507,
    },
    HelpLabelContour {
        start: [3106, 12946],
        segments: &BODY_CONTOUR_508,
    },
    HelpLabelContour {
        start: [3882, 12535],
        segments: &BODY_CONTOUR_509,
    },
    HelpLabelContour {
        start: [3761, 12946],
        segments: &BODY_CONTOUR_510,
    },
    HelpLabelContour {
        start: [4677, 12195],
        segments: &BODY_CONTOUR_511,
    },
    HelpLabelContour {
        start: [4631, 12982],
        segments: &BODY_CONTOUR_512,
    },
    HelpLabelContour {
        start: [5172, 13012],
        segments: &BODY_CONTOUR_513,
    },
    HelpLabelContour {
        start: [6555, 12451],
        segments: &BODY_CONTOUR_514,
    },
    HelpLabelContour {
        start: [6756, 12195],
        segments: &BODY_CONTOUR_515,
    },
    HelpLabelContour {
        start: [7557, 12535],
        segments: &BODY_CONTOUR_516,
    },
    HelpLabelContour {
        start: [7436, 12946],
        segments: &BODY_CONTOUR_517,
    },
    HelpLabelContour {
        start: [8452, 12692],
        segments: &BODY_CONTOUR_518,
    },
    HelpLabelContour {
        start: [9010, 12453],
        segments: &BODY_CONTOUR_519,
    },
    HelpLabelContour {
        start: [9556, 12195],
        segments: &BODY_CONTOUR_520,
    },
    HelpLabelContour {
        start: [10357, 12535],
        segments: &BODY_CONTOUR_521,
    },
    HelpLabelContour {
        start: [10236, 12946],
        segments: &BODY_CONTOUR_522,
    },
    HelpLabelContour {
        start: [11260, 12453],
        segments: &BODY_CONTOUR_523,
    },
    HelpLabelContour {
        start: [11899, 12441],
        segments: &BODY_CONTOUR_524,
    },
    HelpLabelContour {
        start: [11974, 12760],
        segments: &BODY_CONTOUR_525,
    },
    HelpLabelContour {
        start: [12676, 12560],
        segments: &BODY_CONTOUR_526,
    },
    HelpLabelContour {
        start: [13057, 12535],
        segments: &BODY_CONTOUR_527,
    },
    HelpLabelContour {
        start: [12936, 12946],
        segments: &BODY_CONTOUR_528,
    },
    HelpLabelContour {
        start: [14156, 12560],
        segments: &BODY_CONTOUR_529,
    },
    HelpLabelContour {
        start: [14538, 12535],
        segments: &BODY_CONTOUR_530,
    },
    HelpLabelContour {
        start: [14416, 12946],
        segments: &BODY_CONTOUR_531,
    },
    HelpLabelContour {
        start: [14985, 12453],
        segments: &BODY_CONTOUR_532,
    },
    HelpLabelContour {
        start: [15605, 12992],
        segments: &BODY_CONTOUR_533,
    },
    HelpLabelContour {
        start: [15993, 12535],
        segments: &BODY_CONTOUR_534,
    },
    HelpLabelContour {
        start: [15871, 12946],
        segments: &BODY_CONTOUR_535,
    },
    HelpLabelContour {
        start: [16779, 12461],
        segments: &BODY_CONTOUR_536,
    },
    HelpLabelContour {
        start: [17025, 12453],
        segments: &BODY_CONTOUR_537,
    },
    HelpLabelContour {
        start: [17718, 12535],
        segments: &BODY_CONTOUR_538,
    },
    HelpLabelContour {
        start: [17596, 12946],
        segments: &BODY_CONTOUR_539,
    },
    HelpLabelContour {
        start: [18512, 12195],
        segments: &BODY_CONTOUR_540,
    },
    HelpLabelContour {
        start: [18466, 12982],
        segments: &BODY_CONTOUR_541,
    },
    HelpLabelContour {
        start: [19256, 12195],
        segments: &BODY_CONTOUR_542,
    },
    HelpLabelContour {
        start: [19256, 12584],
        segments: &BODY_CONTOUR_543,
    },
    HelpLabelContour {
        start: [20320, 12453],
        segments: &BODY_CONTOUR_544,
    },
    HelpLabelContour {
        start: [20959, 12441],
        segments: &BODY_CONTOUR_545,
    },
    HelpLabelContour {
        start: [21034, 12760],
        segments: &BODY_CONTOUR_546,
    },
    HelpLabelContour {
        start: [21866, 12453],
        segments: &BODY_CONTOUR_547,
    },
    HelpLabelContour {
        start: [21866, 12588],
        segments: &BODY_CONTOUR_548,
    },
    HelpLabelContour {
        start: [22619, 12441],
        segments: &BODY_CONTOUR_549,
    },
    HelpLabelContour {
        start: [22694, 12760],
        segments: &BODY_CONTOUR_550,
    },
    HelpLabelContour {
        start: [23452, 12195],
        segments: &BODY_CONTOUR_551,
    },
    HelpLabelContour {
        start: [23406, 12982],
        segments: &BODY_CONTOUR_552,
    },
    HelpLabelContour {
        start: [24122, 12195],
        segments: &BODY_CONTOUR_553,
    },
    HelpLabelContour {
        start: [24076, 12982],
        segments: &BODY_CONTOUR_554,
    },
    HelpLabelContour {
        start: [24653, 12535],
        segments: &BODY_CONTOUR_555,
    },
    HelpLabelContour {
        start: [24531, 12946],
        segments: &BODY_CONTOUR_556,
    },
    HelpLabelContour {
        start: [25275, 12992],
        segments: &BODY_CONTOUR_557,
    },
    HelpLabelContour {
        start: [25532, 12916],
        segments: &BODY_CONTOUR_558,
    },
];

pub const BODY: HelpLabelDefinition = HelpLabelDefinition {
    text: "1. Try to shoot as many balls as possible into your opponents goal-line.2. Big balls score more points than small balls.3. Balls are subject to forces, the attract each other...    ...or reject eatch other - that depends on the gravity mode you play.4. Touching balls merge into bigger ones.5. Very big balls start to burn - don't touch them, they paralyze you.6. In SpeedGravity-Mode faster balls score more     and the balls take up speed, when they are reflected by a paddel.",
    define_text_id: 102,
    font_ids: &BODY_FONT_IDS,
    color_rgb: [255, 255, 255],
    bounds_centipx: [-11415, 27780, 295, 13325],
    contours: &BODY_CONTOURS,
};

const BACK_FONT_IDS: [u16; 1] = [26];

const BACK_CONTOUR_0: [HelpLabelSegment; 13] = [
    HelpLabelSegment::Line([5790, 725]),
    HelpLabelSegment::Quad {
        control: [5810, 697],
        to: [5869, 672],
    },
    HelpLabelSegment::Quad {
        control: [5927, 648],
        to: [5983, 648],
    },
    HelpLabelSegment::Quad {
        control: [6155, 648],
        to: [6262, 767],
    },
    HelpLabelSegment::Quad {
        control: [6368, 886],
        to: [6368, 1067],
    },
    HelpLabelSegment::Quad {
        control: [6368, 1277],
        to: [6262, 1397],
    },
    HelpLabelSegment::Quad {
        control: [6154, 1516],
        to: [5971, 1516],
    },
    HelpLabelSegment::Quad {
        control: [5912, 1516],
        to: [5855, 1494],
    },
    HelpLabelSegment::Quad {
        control: [5797, 1472],
        to: [5768, 1441],
    },
    HelpLabelSegment::Line([5715, 1516]),
    HelpLabelSegment::Line([5641, 1516]),
    HelpLabelSegment::Line([5641, 320]),
    HelpLabelSegment::Line([5790, 320]),
];

const BACK_CONTOUR_1: [HelpLabelSegment; 9] = [
    HelpLabelSegment::Line([5790, 1322]),
    HelpLabelSegment::Quad {
        control: [5790, 1334],
        to: [5844, 1362],
    },
    HelpLabelSegment::Quad {
        control: [5901, 1391],
        to: [5929, 1391],
    },
    HelpLabelSegment::Quad {
        control: [6080, 1391],
        to: [6146, 1319],
    },
    HelpLabelSegment::Quad {
        control: [6212, 1245],
        to: [6212, 1075],
    },
    HelpLabelSegment::Quad {
        control: [6212, 933],
        to: [6135, 853],
    },
    HelpLabelSegment::Quad {
        control: [6058, 773],
        to: [5929, 773],
    },
    HelpLabelSegment::Quad {
        control: [5902, 773],
        to: [5854, 797],
    },
    HelpLabelSegment::Line([5790, 839]),
];

const BACK_CONTOUR_2: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Quad {
        control: [6973, 648],
        to: [7056, 731],
    },
    HelpLabelSegment::Quad {
        control: [7138, 814],
        to: [7138, 994],
    },
    HelpLabelSegment::Line([7138, 1294]),
    HelpLabelSegment::Quad {
        control: [7138, 1405],
        to: [7203, 1441],
    },
    HelpLabelSegment::Line([7203, 1516]),
    HelpLabelSegment::Quad {
        control: [7112, 1516],
        to: [7069, 1489],
    },
    HelpLabelSegment::Quad {
        control: [7023, 1464],
        to: [7003, 1405],
    },
    HelpLabelSegment::Quad {
        control: [6914, 1516],
        to: [6731, 1516],
    },
    HelpLabelSegment::Quad {
        control: [6633, 1516],
        to: [6561, 1445],
    },
    HelpLabelSegment::Quad {
        control: [6488, 1373],
        to: [6488, 1267],
    },
    HelpLabelSegment::Quad {
        control: [6488, 1139],
        to: [6600, 1052],
    },
    HelpLabelSegment::Quad {
        control: [6711, 964],
        to: [6883, 964],
    },
    HelpLabelSegment::Line([6989, 984]),
    HelpLabelSegment::Quad {
        control: [6989, 781],
        to: [6808, 781],
    },
    HelpLabelSegment::Quad {
        control: [6669, 781],
        to: [6594, 856],
    },
    HelpLabelSegment::Line([6531, 731]),
    HelpLabelSegment::Quad {
        control: [6573, 697],
        to: [6648, 673],
    },
    HelpLabelSegment::Quad {
        control: [6723, 648],
        to: [6791, 648],
    },
];

const BACK_CONTOUR_3: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [6778, 1073],
        to: [6708, 1131],
    },
    HelpLabelSegment::Quad {
        control: [6636, 1189],
        to: [6636, 1269],
    },
    HelpLabelSegment::Quad {
        control: [6636, 1398],
        to: [6791, 1398],
    },
    HelpLabelSegment::Quad {
        control: [6903, 1398],
        to: [6989, 1292],
    },
    HelpLabelSegment::Line([6989, 1089]),
    HelpLabelSegment::Line([6891, 1073]),
];

const BACK_CONTOUR_4: [HelpLabelSegment; 16] = [
    HelpLabelSegment::Quad {
        control: [7967, 703],
        to: [8003, 731],
    },
    HelpLabelSegment::Line([7929, 836]),
    HelpLabelSegment::Quad {
        control: [7906, 814],
        to: [7849, 794],
    },
    HelpLabelSegment::Line([7735, 773]),
    HelpLabelSegment::Quad {
        control: [7615, 773],
        to: [7543, 858],
    },
    HelpLabelSegment::Quad {
        control: [7473, 942],
        to: [7473, 1091],
    },
    HelpLabelSegment::Quad {
        control: [7473, 1238],
        to: [7545, 1314],
    },
    HelpLabelSegment::Quad {
        control: [7618, 1391],
        to: [7746, 1391],
    },
    HelpLabelSegment::Quad {
        control: [7846, 1391],
        to: [7948, 1314],
    },
    HelpLabelSegment::Line([8007, 1439]),
    HelpLabelSegment::Quad {
        control: [7887, 1516],
        to: [7710, 1516],
    },
    HelpLabelSegment::Quad {
        control: [7540, 1516],
        to: [7428, 1402],
    },
    HelpLabelSegment::Quad {
        control: [7317, 1286],
        to: [7317, 1091],
    },
    HelpLabelSegment::Quad {
        control: [7317, 891],
        to: [7432, 769],
    },
    HelpLabelSegment::Quad {
        control: [7548, 648],
        to: [7749, 648],
    },
    HelpLabelSegment::Line([7890, 675]),
];

const BACK_CONTOUR_5: [HelpLabelSegment; 12] = [
    HelpLabelSegment::Line([8863, 1500]),
    HelpLabelSegment::Line([8702, 1500]),
    HelpLabelSegment::Line([8439, 1081]),
    HelpLabelSegment::Line([8310, 1216]),
    HelpLabelSegment::Line([8310, 1500]),
    HelpLabelSegment::Line([8161, 1500]),
    HelpLabelSegment::Line([8161, 320]),
    HelpLabelSegment::Line([8310, 320]),
    HelpLabelSegment::Line([8310, 1053]),
    HelpLabelSegment::Line([8630, 664]),
    HelpLabelSegment::Line([8803, 664]),
    HelpLabelSegment::Line([8535, 981]),
];

const BACK_CONTOURS: [HelpLabelContour; 6] = [
    HelpLabelContour {
        start: [5790, 320],
        segments: &BACK_CONTOUR_0,
    },
    HelpLabelContour {
        start: [5790, 839],
        segments: &BACK_CONTOUR_1,
    },
    HelpLabelContour {
        start: [6791, 648],
        segments: &BACK_CONTOUR_2,
    },
    HelpLabelContour {
        start: [6891, 1073],
        segments: &BACK_CONTOUR_3,
    },
    HelpLabelContour {
        start: [7890, 675],
        segments: &BACK_CONTOUR_4,
    },
    HelpLabelContour {
        start: [8535, 981],
        segments: &BACK_CONTOUR_5,
    },
];

pub const BACK: HelpLabelDefinition = HelpLabelDefinition {
    text: "back",
    define_text_id: 104,
    font_ids: &BACK_FONT_IDS,
    color_rgb: [255, 255, 255],
    bounds_centipx: [5640, 9560, 320, 1515],
    contours: &BACK_CONTOURS,
};

const KEY_W_FONT_IDS: [u16; 1] = [106];

const KEY_W_CONTOUR_0: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Line([7545, 1630]),
    HelpLabelSegment::Line([7380, 1630]),
    HelpLabelSegment::Line([7195, 913]),
    HelpLabelSegment::Line([7007, 1630]),
    HelpLabelSegment::Line([6842, 1630]),
    HelpLabelSegment::Line([6555, 697]),
    HelpLabelSegment::Line([6719, 697]),
    HelpLabelSegment::Line([6923, 1437]),
    HelpLabelSegment::Line([6972, 1243]),
    HelpLabelSegment::Line([7120, 697]),
    HelpLabelSegment::Line([7283, 697]),
    HelpLabelSegment::Line([7469, 1417]),
    HelpLabelSegment::Line([7682, 697]),
    HelpLabelSegment::Line([7837, 697]),
];

const KEY_W_CONTOURS: [HelpLabelContour; 1] = [HelpLabelContour {
    start: [7837, 697],
    segments: &KEY_W_CONTOUR_0,
}];

pub const KEY_W: HelpLabelDefinition = HelpLabelDefinition {
    text: "w",
    define_text_id: 107,
    font_ids: &KEY_W_FONT_IDS,
    color_rgb: [0, 36, 85],
    bounds_centipx: [6555, 8975, 505, 1630],
    contours: &KEY_W_CONTOURS,
};

const KEY_D_FONT_IDS: [u16; 1] = [106];

const KEY_D_CONTOUR_0: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Line([7572, 1630]),
    HelpLabelSegment::Line([7424, 1630]),
    HelpLabelSegment::Line([7424, 1512]),
    HelpLabelSegment::Quad {
        control: [7336, 1651],
        to: [7164, 1651],
    },
    HelpLabelSegment::Quad {
        control: [7052, 1651],
        to: [6958, 1590],
    },
    HelpLabelSegment::Quad {
        control: [6865, 1528],
        to: [6814, 1419],
    },
    HelpLabelSegment::Quad {
        control: [6762, 1308],
        to: [6762, 1164],
    },
    HelpLabelSegment::Quad {
        control: [6762, 1025],
        to: [6809, 911],
    },
    HelpLabelSegment::Quad {
        control: [6855, 797],
        to: [6948, 737],
    },
    HelpLabelSegment::Quad {
        control: [7041, 676],
        to: [7157, 676],
    },
    HelpLabelSegment::Quad {
        control: [7241, 676],
        to: [7306, 712],
    },
    HelpLabelSegment::Quad {
        control: [7373, 748],
        to: [7414, 804],
    },
    HelpLabelSegment::Line([7414, 342]),
    HelpLabelSegment::Line([7572, 342]),
];

const KEY_D_CONTOUR_1: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [7066, 807],
        to: [6995, 893],
    },
    HelpLabelSegment::Quad {
        control: [6925, 980],
        to: [6925, 1164],
    },
    HelpLabelSegment::Quad {
        control: [6925, 1343],
        to: [7001, 1433],
    },
    HelpLabelSegment::Quad {
        control: [7076, 1521],
        to: [7178, 1521],
    },
    HelpLabelSegment::Quad {
        control: [7282, 1521],
        to: [7356, 1437],
    },
    HelpLabelSegment::Quad {
        control: [7428, 1352],
        to: [7428, 1178],
    },
    HelpLabelSegment::Quad {
        control: [7428, 987],
        to: [7354, 897],
    },
    HelpLabelSegment::Quad {
        control: [7280, 807],
        to: [7171, 807],
    },
];

const KEY_D_CONTOURS: [HelpLabelContour; 2] = [
    HelpLabelContour {
        start: [7572, 342],
        segments: &KEY_D_CONTOUR_0,
    },
    HelpLabelContour {
        start: [7171, 807],
        segments: &KEY_D_CONTOUR_1,
    },
];

pub const KEY_D: HelpLabelDefinition = HelpLabelDefinition {
    text: "d",
    define_text_id: 108,
    font_ids: &KEY_D_FONT_IDS,
    color_rgb: [0, 36, 85],
    bounds_centipx: [6760, 8825, 340, 1650],
    contours: &KEY_D_CONTOURS,
};

const KEY_S_FONT_IDS: [u16; 1] = [106];

const KEY_S_CONTOUR_0: [HelpLabelSegment; 33] = [
    HelpLabelSegment::Quad {
        control: [7451, 739],
        to: [7492, 793],
    },
    HelpLabelSegment::Quad {
        control: [7530, 848],
        to: [7546, 937],
    },
    HelpLabelSegment::Line([7392, 959]),
    HelpLabelSegment::Quad {
        control: [7381, 886],
        to: [7330, 846],
    },
    HelpLabelSegment::Quad {
        control: [7279, 806],
        to: [7188, 806],
    },
    HelpLabelSegment::Quad {
        control: [7079, 806],
        to: [7031, 843],
    },
    HelpLabelSegment::Quad {
        control: [6986, 878],
        to: [6986, 927],
    },
    HelpLabelSegment::Quad {
        control: [6986, 957],
        to: [7005, 981],
    },
    HelpLabelSegment::Quad {
        control: [7024, 1008],
        to: [7065, 1024],
    },
    HelpLabelSegment::Line([7205, 1064]),
    HelpLabelSegment::Line([7439, 1138]),
    HelpLabelSegment::Quad {
        control: [7506, 1166],
        to: [7543, 1220],
    },
    HelpLabelSegment::Quad {
        control: [7581, 1275],
        to: [7581, 1356],
    },
    HelpLabelSegment::Quad {
        control: [7581, 1435],
        to: [7536, 1505],
    },
    HelpLabelSegment::Quad {
        control: [7488, 1576],
        to: [7402, 1614],
    },
    HelpLabelSegment::Quad {
        control: [7314, 1651],
        to: [7205, 1651],
    },
    HelpLabelSegment::Quad {
        control: [7022, 1651],
        to: [6928, 1576],
    },
    HelpLabelSegment::Quad {
        control: [6833, 1500],
        to: [6806, 1352],
    },
    HelpLabelSegment::Line([6963, 1328]),
    HelpLabelSegment::Quad {
        control: [6975, 1421],
        to: [7037, 1472],
    },
    HelpLabelSegment::Quad {
        control: [7096, 1521],
        to: [7204, 1521],
    },
    HelpLabelSegment::Quad {
        control: [7312, 1521],
        to: [7365, 1477],
    },
    HelpLabelSegment::Quad {
        control: [7418, 1433],
        to: [7418, 1373],
    },
    HelpLabelSegment::Quad {
        control: [7418, 1319],
        to: [7372, 1289],
    },
    HelpLabelSegment::Quad {
        control: [7339, 1268],
        to: [7211, 1234],
    },
    HelpLabelSegment::Quad {
        control: [7037, 1191],
        to: [6970, 1159],
    },
    HelpLabelSegment::Quad {
        control: [6901, 1127],
        to: [6868, 1071],
    },
    HelpLabelSegment::Quad {
        control: [6833, 1013],
        to: [6833, 944],
    },
    HelpLabelSegment::Quad {
        control: [6833, 883],
        to: [6861, 830],
    },
    HelpLabelSegment::Quad {
        control: [6889, 776],
        to: [6938, 741],
    },
    HelpLabelSegment::Quad {
        control: [6975, 714],
        to: [7040, 695],
    },
    HelpLabelSegment::Line([7175, 676]),
    HelpLabelSegment::Quad {
        control: [7286, 676],
        to: [7369, 707],
    },
];

const KEY_S_CONTOURS: [HelpLabelContour; 1] = [HelpLabelContour {
    start: [7369, 707],
    segments: &KEY_S_CONTOUR_0,
}];

pub const KEY_S: HelpLabelDefinition = HelpLabelDefinition {
    text: "s",
    define_text_id: 109,
    font_ids: &KEY_S_FONT_IDS,
    color_rgb: [0, 36, 85],
    bounds_centipx: [6805, 8775, 505, 1650],
    contours: &KEY_S_CONTOURS,
};

const RED_MOVE_UP_FONT_IDS: [u16; 1] = [26];

const RED_MOVE_UP_CONTOUR_0: [HelpLabelSegment; 24] = [
    HelpLabelSegment::Quad {
        control: [5833, 656],
        to: [5833, 760],
    },
    HelpLabelSegment::Line([5833, 1180]),
    HelpLabelSegment::Line([5721, 1180]),
    HelpLabelSegment::Line([5721, 783]),
    HelpLabelSegment::Quad {
        control: [5721, 635],
        to: [5592, 635],
    },
    HelpLabelSegment::Quad {
        control: [5552, 635],
        to: [5517, 660],
    },
    HelpLabelSegment::Quad {
        control: [5482, 684],
        to: [5469, 716],
    },
    HelpLabelSegment::Line([5469, 1180]),
    HelpLabelSegment::Line([5358, 1180]),
    HelpLabelSegment::Line([5358, 735]),
    HelpLabelSegment::Quad {
        control: [5358, 688],
        to: [5323, 662],
    },
    HelpLabelSegment::Quad {
        control: [5288, 635],
        to: [5230, 635],
    },
    HelpLabelSegment::Quad {
        control: [5197, 635],
        to: [5160, 661],
    },
    HelpLabelSegment::Quad {
        control: [5121, 687],
        to: [5106, 717],
    },
    HelpLabelSegment::Line([5106, 1180]),
    HelpLabelSegment::Line([4995, 1180]),
    HelpLabelSegment::Line([4995, 553]),
    HelpLabelSegment::Line([5067, 553]),
    HelpLabelSegment::Line([5104, 626]),
    HelpLabelSegment::Quad {
        control: [5168, 541],
        to: [5265, 541],
    },
    HelpLabelSegment::Quad {
        control: [5400, 541],
        to: [5454, 625],
    },
    HelpLabelSegment::Quad {
        control: [5473, 589],
        to: [5523, 565],
    },
    HelpLabelSegment::Quad {
        control: [5575, 541],
        to: [5629, 541],
    },
    HelpLabelSegment::Quad {
        control: [5726, 541],
        to: [5779, 599],
    },
];

const RED_MOVE_UP_CONTOUR_1: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [6066, 1101],
        to: [6232, 1101],
    },
    HelpLabelSegment::Quad {
        control: [6311, 1101],
        to: [6355, 1038],
    },
    HelpLabelSegment::Quad {
        control: [6399, 975],
        to: [6399, 865],
    },
    HelpLabelSegment::Quad {
        control: [6399, 632],
        to: [6232, 632],
    },
    HelpLabelSegment::Quad {
        control: [6156, 632],
        to: [6112, 694],
    },
    HelpLabelSegment::Quad {
        control: [6066, 756],
        to: [6066, 865],
    },
];

const RED_MOVE_UP_CONTOUR_2: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [6105, 541],
        to: [6232, 541],
    },
    HelpLabelSegment::Quad {
        control: [6367, 541],
        to: [6442, 627],
    },
    HelpLabelSegment::Quad {
        control: [6516, 712],
        to: [6516, 865],
    },
    HelpLabelSegment::Quad {
        control: [6516, 1017],
        to: [6440, 1105],
    },
    HelpLabelSegment::Quad {
        control: [6364, 1192],
        to: [6232, 1192],
    },
    HelpLabelSegment::Quad {
        control: [6099, 1192],
        to: [6024, 1104],
    },
    HelpLabelSegment::Quad {
        control: [5949, 1015],
        to: [5949, 865],
    },
    HelpLabelSegment::Quad {
        control: [5949, 719],
        to: [6027, 630],
    },
];

const RED_MOVE_UP_CONTOUR_3: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([6861, 1192]),
    HelpLabelSegment::Line([6832, 1192]),
    HelpLabelSegment::Line([6562, 551]),
    HelpLabelSegment::Line([6684, 551]),
    HelpLabelSegment::Line([6850, 990]),
    HelpLabelSegment::Line([7019, 551]),
    HelpLabelSegment::Line([7136, 551]),
];

const RED_MOVE_UP_CONTOUR_4: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [7409, 635],
        to: [7358, 683],
    },
    HelpLabelSegment::Quad {
        control: [7310, 729],
        to: [7303, 797],
    },
    HelpLabelSegment::Line([7651, 797]),
    HelpLabelSegment::Quad {
        control: [7651, 729],
        to: [7609, 684],
    },
    HelpLabelSegment::Quad {
        control: [7562, 635],
        to: [7482, 635],
    },
];

const RED_MOVE_UP_CONTOUR_5: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [7415, 1098],
        to: [7498, 1098],
    },
    HelpLabelSegment::Quad {
        control: [7594, 1098],
        to: [7657, 1043],
    },
    HelpLabelSegment::Line([7704, 1123]),
    HelpLabelSegment::Quad {
        control: [7678, 1148],
        to: [7625, 1167],
    },
    HelpLabelSegment::Quad {
        control: [7559, 1192],
        to: [7477, 1192],
    },
    HelpLabelSegment::Quad {
        control: [7358, 1192],
        to: [7275, 1112],
    },
    HelpLabelSegment::Quad {
        control: [7184, 1023],
        to: [7184, 874],
    },
    HelpLabelSegment::Quad {
        control: [7184, 718],
        to: [7277, 625],
    },
    HelpLabelSegment::Quad {
        control: [7362, 541],
        to: [7478, 541],
    },
    HelpLabelSegment::Quad {
        control: [7611, 541],
        to: [7688, 616],
    },
    HelpLabelSegment::Quad {
        control: [7761, 689],
        to: [7761, 810],
    },
    HelpLabelSegment::Quad {
        control: [7761, 846],
        to: [7753, 878],
    },
    HelpLabelSegment::Line([7301, 878]),
    HelpLabelSegment::Quad {
        control: [7301, 988],
        to: [7361, 1046],
    },
];

const RED_MOVE_UP_CONTOUR_6: [HelpLabelSegment; 15] = [
    HelpLabelSegment::Line([8345, 953]),
    HelpLabelSegment::Quad {
        control: [8345, 1098],
        to: [8471, 1098],
    },
    HelpLabelSegment::Quad {
        control: [8526, 1098],
        to: [8571, 1066],
    },
    HelpLabelSegment::Quad {
        control: [8617, 1035],
        to: [8632, 994],
    },
    HelpLabelSegment::Line([8632, 553]),
    HelpLabelSegment::Line([8744, 553]),
    HelpLabelSegment::Line([8744, 1180]),
    HelpLabelSegment::Line([8632, 1180]),
    HelpLabelSegment::Line([8632, 1093]),
    HelpLabelSegment::Quad {
        control: [8614, 1131],
        to: [8557, 1161],
    },
    HelpLabelSegment::Quad {
        control: [8500, 1192],
        to: [8446, 1192],
    },
    HelpLabelSegment::Quad {
        control: [8343, 1192],
        to: [8289, 1133],
    },
    HelpLabelSegment::Quad {
        control: [8234, 1073],
        to: [8234, 964],
    },
    HelpLabelSegment::Line([8234, 553]),
    HelpLabelSegment::Line([8345, 553]),
];

const RED_MOVE_UP_CONTOUR_7: [HelpLabelSegment; 12] = [
    HelpLabelSegment::Line([9006, 605]),
    HelpLabelSegment::Quad {
        control: [9069, 541],
        to: [9158, 541],
    },
    HelpLabelSegment::Quad {
        control: [9292, 541],
        to: [9367, 625],
    },
    HelpLabelSegment::Quad {
        control: [9441, 708],
        to: [9441, 868],
    },
    HelpLabelSegment::Quad {
        control: [9441, 1011],
        to: [9366, 1101],
    },
    HelpLabelSegment::Quad {
        control: [9291, 1192],
        to: [9149, 1192],
    },
    HelpLabelSegment::Quad {
        control: [9109, 1192],
        to: [9065, 1178],
    },
    HelpLabelSegment::Quad {
        control: [9019, 1164],
        to: [9006, 1146],
    },
    HelpLabelSegment::Line([9006, 1426]),
    HelpLabelSegment::Line([8895, 1426]),
    HelpLabelSegment::Line([8895, 553]),
    HelpLabelSegment::Line([9006, 553]),
];

const RED_MOVE_UP_CONTOUR_8: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Line([9006, 1053]),
    HelpLabelSegment::Quad {
        control: [9017, 1070],
        to: [9051, 1084],
    },
    HelpLabelSegment::Quad {
        control: [9085, 1098],
        to: [9116, 1098],
    },
    HelpLabelSegment::Quad {
        control: [9324, 1098],
        to: [9324, 864],
    },
    HelpLabelSegment::Quad {
        control: [9324, 745],
        to: [9274, 690],
    },
    HelpLabelSegment::Quad {
        control: [9225, 635],
        to: [9117, 635],
    },
    HelpLabelSegment::Quad {
        control: [9094, 635],
        to: [9060, 651],
    },
    HelpLabelSegment::Line([9006, 688]),
];

const RED_MOVE_UP_CONTOURS: [HelpLabelContour; 9] = [
    HelpLabelContour {
        start: [5779, 599],
        segments: &RED_MOVE_UP_CONTOUR_0,
    },
    HelpLabelContour {
        start: [6066, 865],
        segments: &RED_MOVE_UP_CONTOUR_1,
    },
    HelpLabelContour {
        start: [6027, 630],
        segments: &RED_MOVE_UP_CONTOUR_2,
    },
    HelpLabelContour {
        start: [7136, 551],
        segments: &RED_MOVE_UP_CONTOUR_3,
    },
    HelpLabelContour {
        start: [7482, 635],
        segments: &RED_MOVE_UP_CONTOUR_4,
    },
    HelpLabelContour {
        start: [7361, 1046],
        segments: &RED_MOVE_UP_CONTOUR_5,
    },
    HelpLabelContour {
        start: [8345, 553],
        segments: &RED_MOVE_UP_CONTOUR_6,
    },
    HelpLabelContour {
        start: [9006, 553],
        segments: &RED_MOVE_UP_CONTOUR_7,
    },
    HelpLabelContour {
        start: [9006, 688],
        segments: &RED_MOVE_UP_CONTOUR_8,
    },
];

pub const RED_MOVE_UP: HelpLabelDefinition = HelpLabelDefinition {
    text: "move up",
    define_text_id: 111,
    font_ids: &RED_MOVE_UP_FONT_IDS,
    color_rgb: [255, 129, 34],
    bounds_centipx: [4995, 10010, 525, 1425],
    contours: &RED_MOVE_UP_CONTOURS,
};

const RED_MOVE_DOWN_FONT_IDS: [u16; 1] = [26];

const RED_MOVE_DOWN_CONTOUR_0: [HelpLabelSegment; 24] = [
    HelpLabelSegment::Quad {
        control: [5063, 656],
        to: [5063, 760],
    },
    HelpLabelSegment::Line([5063, 1180]),
    HelpLabelSegment::Line([4951, 1180]),
    HelpLabelSegment::Line([4951, 783]),
    HelpLabelSegment::Quad {
        control: [4951, 635],
        to: [4822, 635],
    },
    HelpLabelSegment::Quad {
        control: [4782, 635],
        to: [4747, 660],
    },
    HelpLabelSegment::Quad {
        control: [4712, 684],
        to: [4699, 716],
    },
    HelpLabelSegment::Line([4699, 1180]),
    HelpLabelSegment::Line([4588, 1180]),
    HelpLabelSegment::Line([4588, 735]),
    HelpLabelSegment::Quad {
        control: [4588, 688],
        to: [4553, 662],
    },
    HelpLabelSegment::Quad {
        control: [4518, 635],
        to: [4460, 635],
    },
    HelpLabelSegment::Quad {
        control: [4427, 635],
        to: [4390, 661],
    },
    HelpLabelSegment::Quad {
        control: [4351, 687],
        to: [4336, 717],
    },
    HelpLabelSegment::Line([4336, 1180]),
    HelpLabelSegment::Line([4225, 1180]),
    HelpLabelSegment::Line([4225, 553]),
    HelpLabelSegment::Line([4297, 553]),
    HelpLabelSegment::Line([4334, 626]),
    HelpLabelSegment::Quad {
        control: [4398, 541],
        to: [4495, 541],
    },
    HelpLabelSegment::Quad {
        control: [4630, 541],
        to: [4684, 625],
    },
    HelpLabelSegment::Quad {
        control: [4703, 589],
        to: [4753, 565],
    },
    HelpLabelSegment::Quad {
        control: [4805, 541],
        to: [4859, 541],
    },
    HelpLabelSegment::Quad {
        control: [4956, 541],
        to: [5009, 599],
    },
];

const RED_MOVE_DOWN_CONTOUR_1: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [5296, 1101],
        to: [5462, 1101],
    },
    HelpLabelSegment::Quad {
        control: [5541, 1101],
        to: [5585, 1038],
    },
    HelpLabelSegment::Quad {
        control: [5629, 975],
        to: [5629, 865],
    },
    HelpLabelSegment::Quad {
        control: [5629, 632],
        to: [5462, 632],
    },
    HelpLabelSegment::Quad {
        control: [5386, 632],
        to: [5342, 694],
    },
    HelpLabelSegment::Quad {
        control: [5296, 756],
        to: [5296, 865],
    },
];

const RED_MOVE_DOWN_CONTOUR_2: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [5335, 541],
        to: [5462, 541],
    },
    HelpLabelSegment::Quad {
        control: [5597, 541],
        to: [5672, 627],
    },
    HelpLabelSegment::Quad {
        control: [5746, 712],
        to: [5746, 865],
    },
    HelpLabelSegment::Quad {
        control: [5746, 1017],
        to: [5670, 1105],
    },
    HelpLabelSegment::Quad {
        control: [5594, 1192],
        to: [5462, 1192],
    },
    HelpLabelSegment::Quad {
        control: [5329, 1192],
        to: [5254, 1104],
    },
    HelpLabelSegment::Quad {
        control: [5179, 1015],
        to: [5179, 865],
    },
    HelpLabelSegment::Quad {
        control: [5179, 719],
        to: [5257, 630],
    },
];

const RED_MOVE_DOWN_CONTOUR_3: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([6091, 1192]),
    HelpLabelSegment::Line([6062, 1192]),
    HelpLabelSegment::Line([5792, 551]),
    HelpLabelSegment::Line([5914, 551]),
    HelpLabelSegment::Line([6080, 990]),
    HelpLabelSegment::Line([6249, 551]),
    HelpLabelSegment::Line([6366, 551]),
];

const RED_MOVE_DOWN_CONTOUR_4: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [6639, 635],
        to: [6588, 683],
    },
    HelpLabelSegment::Quad {
        control: [6540, 729],
        to: [6533, 797],
    },
    HelpLabelSegment::Line([6881, 797]),
    HelpLabelSegment::Quad {
        control: [6881, 729],
        to: [6839, 684],
    },
    HelpLabelSegment::Quad {
        control: [6792, 635],
        to: [6712, 635],
    },
];

const RED_MOVE_DOWN_CONTOUR_5: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [6645, 1098],
        to: [6728, 1098],
    },
    HelpLabelSegment::Quad {
        control: [6824, 1098],
        to: [6887, 1043],
    },
    HelpLabelSegment::Line([6934, 1123]),
    HelpLabelSegment::Quad {
        control: [6908, 1148],
        to: [6855, 1167],
    },
    HelpLabelSegment::Quad {
        control: [6789, 1192],
        to: [6707, 1192],
    },
    HelpLabelSegment::Quad {
        control: [6588, 1192],
        to: [6505, 1112],
    },
    HelpLabelSegment::Quad {
        control: [6414, 1023],
        to: [6414, 874],
    },
    HelpLabelSegment::Quad {
        control: [6414, 718],
        to: [6507, 625],
    },
    HelpLabelSegment::Quad {
        control: [6592, 541],
        to: [6708, 541],
    },
    HelpLabelSegment::Quad {
        control: [6841, 541],
        to: [6918, 616],
    },
    HelpLabelSegment::Quad {
        control: [6991, 689],
        to: [6991, 810],
    },
    HelpLabelSegment::Quad {
        control: [6991, 846],
        to: [6983, 878],
    },
    HelpLabelSegment::Line([6531, 878]),
    HelpLabelSegment::Quad {
        control: [6531, 988],
        to: [6591, 1046],
    },
];

const RED_MOVE_DOWN_CONTOUR_6: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Line([7978, 295]),
    HelpLabelSegment::Line([7978, 1180]),
    HelpLabelSegment::Line([7867, 1180]),
    HelpLabelSegment::Line([7867, 1133]),
    HelpLabelSegment::Quad {
        control: [7810, 1192],
        to: [7698, 1192],
    },
    HelpLabelSegment::Quad {
        control: [7581, 1192],
        to: [7507, 1107],
    },
    HelpLabelSegment::Quad {
        control: [7435, 1023],
        to: [7435, 882],
    },
    HelpLabelSegment::Quad {
        control: [7435, 741],
        to: [7519, 641],
    },
    HelpLabelSegment::Quad {
        control: [7603, 541],
        to: [7719, 541],
    },
    HelpLabelSegment::Quad {
        control: [7817, 541],
        to: [7867, 587],
    },
    HelpLabelSegment::Line([7867, 295]),
];

const RED_MOVE_DOWN_CONTOUR_7: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Quad {
        control: [7858, 1065],
        to: [7867, 1046],
    },
    HelpLabelSegment::Line([7867, 698]),
    HelpLabelSegment::Quad {
        control: [7825, 635],
        to: [7752, 635],
    },
    HelpLabelSegment::Quad {
        control: [7662, 635],
        to: [7607, 702],
    },
    HelpLabelSegment::Quad {
        control: [7552, 769],
        to: [7552, 872],
    },
    HelpLabelSegment::Quad {
        control: [7552, 1098],
        to: [7758, 1098],
    },
    HelpLabelSegment::Quad {
        control: [7784, 1098],
        to: [7821, 1082],
    },
];

const RED_MOVE_DOWN_CONTOUR_8: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [8216, 1101],
        to: [8382, 1101],
    },
    HelpLabelSegment::Quad {
        control: [8461, 1101],
        to: [8505, 1038],
    },
    HelpLabelSegment::Quad {
        control: [8549, 975],
        to: [8549, 865],
    },
    HelpLabelSegment::Quad {
        control: [8549, 632],
        to: [8382, 632],
    },
    HelpLabelSegment::Quad {
        control: [8306, 632],
        to: [8262, 694],
    },
    HelpLabelSegment::Quad {
        control: [8216, 756],
        to: [8216, 865],
    },
];

const RED_MOVE_DOWN_CONTOUR_9: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [8255, 541],
        to: [8382, 541],
    },
    HelpLabelSegment::Quad {
        control: [8517, 541],
        to: [8592, 627],
    },
    HelpLabelSegment::Quad {
        control: [8666, 712],
        to: [8666, 865],
    },
    HelpLabelSegment::Quad {
        control: [8666, 1017],
        to: [8590, 1105],
    },
    HelpLabelSegment::Quad {
        control: [8514, 1192],
        to: [8382, 1192],
    },
    HelpLabelSegment::Quad {
        control: [8249, 1192],
        to: [8174, 1104],
    },
    HelpLabelSegment::Quad {
        control: [8099, 1015],
        to: [8099, 865],
    },
    HelpLabelSegment::Quad {
        control: [8099, 719],
        to: [8177, 630],
    },
];

const RED_MOVE_DOWN_CONTOUR_10: [HelpLabelSegment; 13] = [
    HelpLabelSegment::Line([9365, 1192]),
    HelpLabelSegment::Line([9335, 1192]),
    HelpLabelSegment::Line([9151, 765]),
    HelpLabelSegment::Line([8968, 1192]),
    HelpLabelSegment::Line([8938, 1192]),
    HelpLabelSegment::Line([8714, 551]),
    HelpLabelSegment::Line([8833, 551]),
    HelpLabelSegment::Line([8968, 963]),
    HelpLabelSegment::Line([9134, 551]),
    HelpLabelSegment::Line([9163, 551]),
    HelpLabelSegment::Line([9335, 963]),
    HelpLabelSegment::Line([9480, 551]),
    HelpLabelSegment::Line([9590, 551]),
];

const RED_MOVE_DOWN_CONTOUR_11: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Line([10177, 1180]),
    HelpLabelSegment::Line([10065, 1180]),
    HelpLabelSegment::Line([10065, 816]),
    HelpLabelSegment::Quad {
        control: [10065, 715],
        to: [10036, 675],
    },
    HelpLabelSegment::Quad {
        control: [10005, 635],
        to: [9934, 635],
    },
    HelpLabelSegment::Quad {
        control: [9896, 635],
        to: [9854, 657],
    },
    HelpLabelSegment::Quad {
        control: [9813, 681],
        to: [9791, 714],
    },
    HelpLabelSegment::Line([9791, 1180]),
    HelpLabelSegment::Line([9680, 1180]),
    HelpLabelSegment::Line([9680, 553]),
    HelpLabelSegment::Line([9756, 553]),
    HelpLabelSegment::Line([9791, 634]),
    HelpLabelSegment::Quad {
        control: [9846, 541],
        to: [9970, 541],
    },
    HelpLabelSegment::Quad {
        control: [10177, 541],
        to: [10177, 792],
    },
];

const RED_MOVE_DOWN_CONTOURS: [HelpLabelContour; 12] = [
    HelpLabelContour {
        start: [5009, 599],
        segments: &RED_MOVE_DOWN_CONTOUR_0,
    },
    HelpLabelContour {
        start: [5296, 865],
        segments: &RED_MOVE_DOWN_CONTOUR_1,
    },
    HelpLabelContour {
        start: [5257, 630],
        segments: &RED_MOVE_DOWN_CONTOUR_2,
    },
    HelpLabelContour {
        start: [6366, 551],
        segments: &RED_MOVE_DOWN_CONTOUR_3,
    },
    HelpLabelContour {
        start: [6712, 635],
        segments: &RED_MOVE_DOWN_CONTOUR_4,
    },
    HelpLabelContour {
        start: [6591, 1046],
        segments: &RED_MOVE_DOWN_CONTOUR_5,
    },
    HelpLabelContour {
        start: [7867, 295],
        segments: &RED_MOVE_DOWN_CONTOUR_6,
    },
    HelpLabelContour {
        start: [7821, 1082],
        segments: &RED_MOVE_DOWN_CONTOUR_7,
    },
    HelpLabelContour {
        start: [8216, 865],
        segments: &RED_MOVE_DOWN_CONTOUR_8,
    },
    HelpLabelContour {
        start: [8177, 630],
        segments: &RED_MOVE_DOWN_CONTOUR_9,
    },
    HelpLabelContour {
        start: [9590, 551],
        segments: &RED_MOVE_DOWN_CONTOUR_10,
    },
    HelpLabelContour {
        start: [10177, 792],
        segments: &RED_MOVE_DOWN_CONTOUR_11,
    },
];

pub const RED_MOVE_DOWN: HelpLabelDefinition = HelpLabelDefinition {
    text: "move down",
    define_text_id: 112,
    font_ids: &RED_MOVE_DOWN_FONT_IDS,
    color_rgb: [255, 129, 34],
    bounds_centipx: [4225, 10780, 295, 1190],
    contours: &RED_MOVE_DOWN_CONTOURS,
};

const RED_SHOOT_FONT_IDS: [u16; 1] = [26];

const RED_SHOOT_CONTOUR_0: [HelpLabelSegment; 22] = [
    HelpLabelSegment::Line([6132, 688]),
    HelpLabelSegment::Quad {
        control: [6066, 635],
        to: [5999, 635],
    },
    HelpLabelSegment::Quad {
        control: [5959, 635],
        to: [5932, 654],
    },
    HelpLabelSegment::Quad {
        control: [5904, 673],
        to: [5904, 701],
    },
    HelpLabelSegment::Quad {
        control: [5904, 762],
        to: [5974, 792],
    },
    HelpLabelSegment::Line([6053, 828]),
    HelpLabelSegment::Quad {
        control: [6126, 862],
        to: [6160, 905],
    },
    HelpLabelSegment::Quad {
        control: [6193, 948],
        to: [6193, 1012],
    },
    HelpLabelSegment::Quad {
        control: [6193, 1097],
        to: [6134, 1145],
    },
    HelpLabelSegment::Quad {
        control: [6074, 1192],
        to: [5970, 1192],
    },
    HelpLabelSegment::Quad {
        control: [5870, 1192],
        to: [5784, 1142],
    },
    HelpLabelSegment::Line([5822, 1037]),
    HelpLabelSegment::Quad {
        control: [5916, 1098],
        to: [5972, 1098],
    },
    HelpLabelSegment::Quad {
        control: [6075, 1098],
        to: [6075, 1011],
    },
    HelpLabelSegment::Quad {
        control: [6075, 949],
        to: [5976, 905],
    },
    HelpLabelSegment::Quad {
        control: [5900, 869],
        to: [5873, 852],
    },
    HelpLabelSegment::Quad {
        control: [5846, 833],
        to: [5827, 811],
    },
    HelpLabelSegment::Quad {
        control: [5807, 787],
        to: [5798, 762],
    },
    HelpLabelSegment::Quad {
        control: [5787, 735],
        to: [5787, 705],
    },
    HelpLabelSegment::Quad {
        control: [5787, 628],
        to: [5843, 585],
    },
    HelpLabelSegment::Quad {
        control: [5900, 541],
        to: [5991, 541],
    },
    HelpLabelSegment::Quad {
        control: [6059, 541],
        to: [6163, 585],
    },
];

const RED_SHOOT_CONTOUR_1: [HelpLabelSegment; 16] = [
    HelpLabelSegment::Line([6421, 622]),
    HelpLabelSegment::Quad {
        control: [6443, 587],
        to: [6492, 565],
    },
    HelpLabelSegment::Quad {
        control: [6542, 541],
        to: [6594, 541],
    },
    HelpLabelSegment::Quad {
        control: [6694, 541],
        to: [6751, 607],
    },
    HelpLabelSegment::Quad {
        control: [6808, 673],
        to: [6808, 786],
    },
    HelpLabelSegment::Line([6808, 1180]),
    HelpLabelSegment::Line([6696, 1180]),
    HelpLabelSegment::Line([6696, 786]),
    HelpLabelSegment::Quad {
        control: [6696, 716],
        to: [6661, 675],
    },
    HelpLabelSegment::Quad {
        control: [6627, 635],
        to: [6564, 635],
    },
    HelpLabelSegment::Quad {
        control: [6524, 635],
        to: [6483, 659],
    },
    HelpLabelSegment::Quad {
        control: [6442, 682],
        to: [6421, 714],
    },
    HelpLabelSegment::Line([6421, 1180]),
    HelpLabelSegment::Line([6310, 1180]),
    HelpLabelSegment::Line([6310, 295]),
    HelpLabelSegment::Line([6421, 295]),
];

const RED_SHOOT_CONTOUR_2: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [7041, 1101],
        to: [7207, 1101],
    },
    HelpLabelSegment::Quad {
        control: [7286, 1101],
        to: [7330, 1038],
    },
    HelpLabelSegment::Quad {
        control: [7374, 975],
        to: [7374, 865],
    },
    HelpLabelSegment::Quad {
        control: [7374, 632],
        to: [7207, 632],
    },
    HelpLabelSegment::Quad {
        control: [7131, 632],
        to: [7087, 694],
    },
    HelpLabelSegment::Quad {
        control: [7041, 756],
        to: [7041, 865],
    },
];

const RED_SHOOT_CONTOUR_3: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [7080, 541],
        to: [7207, 541],
    },
    HelpLabelSegment::Quad {
        control: [7342, 541],
        to: [7417, 627],
    },
    HelpLabelSegment::Quad {
        control: [7491, 712],
        to: [7491, 865],
    },
    HelpLabelSegment::Quad {
        control: [7491, 1017],
        to: [7415, 1105],
    },
    HelpLabelSegment::Quad {
        control: [7339, 1192],
        to: [7207, 1192],
    },
    HelpLabelSegment::Quad {
        control: [7074, 1192],
        to: [6999, 1104],
    },
    HelpLabelSegment::Quad {
        control: [6924, 1015],
        to: [6924, 865],
    },
    HelpLabelSegment::Quad {
        control: [6924, 719],
        to: [7002, 630],
    },
];

const RED_SHOOT_CONTOUR_4: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [7686, 1101],
        to: [7852, 1101],
    },
    HelpLabelSegment::Quad {
        control: [7931, 1101],
        to: [7975, 1038],
    },
    HelpLabelSegment::Quad {
        control: [8019, 975],
        to: [8019, 865],
    },
    HelpLabelSegment::Quad {
        control: [8019, 632],
        to: [7852, 632],
    },
    HelpLabelSegment::Quad {
        control: [7776, 632],
        to: [7732, 694],
    },
    HelpLabelSegment::Quad {
        control: [7686, 756],
        to: [7686, 865],
    },
];

const RED_SHOOT_CONTOUR_5: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [7725, 541],
        to: [7852, 541],
    },
    HelpLabelSegment::Quad {
        control: [7987, 541],
        to: [8062, 627],
    },
    HelpLabelSegment::Quad {
        control: [8136, 712],
        to: [8136, 865],
    },
    HelpLabelSegment::Quad {
        control: [8136, 1017],
        to: [8060, 1105],
    },
    HelpLabelSegment::Quad {
        control: [7984, 1192],
        to: [7852, 1192],
    },
    HelpLabelSegment::Quad {
        control: [7719, 1192],
        to: [7644, 1104],
    },
    HelpLabelSegment::Quad {
        control: [7569, 1015],
        to: [7569, 865],
    },
    HelpLabelSegment::Quad {
        control: [7569, 719],
        to: [7647, 630],
    },
];

const RED_SHOOT_CONTOUR_6: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Line([8295, 422]),
    HelpLabelSegment::Line([8406, 378]),
    HelpLabelSegment::Line([8406, 553]),
    HelpLabelSegment::Line([8578, 553]),
    HelpLabelSegment::Line([8578, 641]),
    HelpLabelSegment::Line([8406, 641]),
    HelpLabelSegment::Line([8406, 953]),
    HelpLabelSegment::Quad {
        control: [8406, 1031],
        to: [8433, 1065],
    },
    HelpLabelSegment::Quad {
        control: [8459, 1098],
        to: [8518, 1098],
    },
    HelpLabelSegment::Quad {
        control: [8561, 1098],
        to: [8606, 1077],
    },
    HelpLabelSegment::Line([8623, 1174]),
    HelpLabelSegment::Line([8471, 1192]),
    HelpLabelSegment::Quad {
        control: [8396, 1192],
        to: [8346, 1137],
    },
    HelpLabelSegment::Quad {
        control: [8295, 1082],
        to: [8295, 997],
    },
    HelpLabelSegment::Line([8295, 641]),
    HelpLabelSegment::Line([8222, 641]),
    HelpLabelSegment::Line([8222, 553]),
    HelpLabelSegment::Line([8295, 553]),
];

const RED_SHOOT_CONTOURS: [HelpLabelContour; 7] = [
    HelpLabelContour {
        start: [6163, 585],
        segments: &RED_SHOOT_CONTOUR_0,
    },
    HelpLabelContour {
        start: [6421, 295],
        segments: &RED_SHOOT_CONTOUR_1,
    },
    HelpLabelContour {
        start: [7041, 865],
        segments: &RED_SHOOT_CONTOUR_2,
    },
    HelpLabelContour {
        start: [7002, 630],
        segments: &RED_SHOOT_CONTOUR_3,
    },
    HelpLabelContour {
        start: [7686, 865],
        segments: &RED_SHOOT_CONTOUR_4,
    },
    HelpLabelContour {
        start: [7647, 630],
        segments: &RED_SHOOT_CONTOUR_5,
    },
    HelpLabelContour {
        start: [8295, 553],
        segments: &RED_SHOOT_CONTOUR_6,
    },
];

pub const RED_SHOOT: HelpLabelDefinition = HelpLabelDefinition {
    text: "shoot",
    define_text_id: 113,
    font_ids: &RED_SHOOT_FONT_IDS,
    color_rgb: [255, 129, 34],
    bounds_centipx: [5785, 9175, 295, 1190],
    contours: &RED_SHOOT_CONTOURS,
};

const BLUE_SHOOT_FONT_IDS: [u16; 1] = [26];

const BLUE_SHOOT_CONTOUR_0: [HelpLabelSegment; 22] = [
    HelpLabelSegment::Line([6132, 688]),
    HelpLabelSegment::Quad {
        control: [6066, 635],
        to: [5999, 635],
    },
    HelpLabelSegment::Quad {
        control: [5959, 635],
        to: [5932, 654],
    },
    HelpLabelSegment::Quad {
        control: [5904, 673],
        to: [5904, 701],
    },
    HelpLabelSegment::Quad {
        control: [5904, 762],
        to: [5974, 792],
    },
    HelpLabelSegment::Line([6053, 828]),
    HelpLabelSegment::Quad {
        control: [6126, 862],
        to: [6160, 905],
    },
    HelpLabelSegment::Quad {
        control: [6193, 948],
        to: [6193, 1012],
    },
    HelpLabelSegment::Quad {
        control: [6193, 1097],
        to: [6134, 1145],
    },
    HelpLabelSegment::Quad {
        control: [6074, 1192],
        to: [5970, 1192],
    },
    HelpLabelSegment::Quad {
        control: [5870, 1192],
        to: [5784, 1142],
    },
    HelpLabelSegment::Line([5822, 1037]),
    HelpLabelSegment::Quad {
        control: [5916, 1098],
        to: [5972, 1098],
    },
    HelpLabelSegment::Quad {
        control: [6075, 1098],
        to: [6075, 1011],
    },
    HelpLabelSegment::Quad {
        control: [6075, 949],
        to: [5976, 905],
    },
    HelpLabelSegment::Quad {
        control: [5900, 869],
        to: [5873, 852],
    },
    HelpLabelSegment::Quad {
        control: [5846, 833],
        to: [5827, 811],
    },
    HelpLabelSegment::Quad {
        control: [5807, 787],
        to: [5798, 762],
    },
    HelpLabelSegment::Quad {
        control: [5787, 735],
        to: [5787, 705],
    },
    HelpLabelSegment::Quad {
        control: [5787, 628],
        to: [5843, 585],
    },
    HelpLabelSegment::Quad {
        control: [5900, 541],
        to: [5991, 541],
    },
    HelpLabelSegment::Quad {
        control: [6059, 541],
        to: [6163, 585],
    },
];

const BLUE_SHOOT_CONTOUR_1: [HelpLabelSegment; 16] = [
    HelpLabelSegment::Line([6421, 622]),
    HelpLabelSegment::Quad {
        control: [6443, 587],
        to: [6492, 565],
    },
    HelpLabelSegment::Quad {
        control: [6542, 541],
        to: [6594, 541],
    },
    HelpLabelSegment::Quad {
        control: [6694, 541],
        to: [6751, 607],
    },
    HelpLabelSegment::Quad {
        control: [6808, 673],
        to: [6808, 786],
    },
    HelpLabelSegment::Line([6808, 1180]),
    HelpLabelSegment::Line([6696, 1180]),
    HelpLabelSegment::Line([6696, 786]),
    HelpLabelSegment::Quad {
        control: [6696, 716],
        to: [6661, 675],
    },
    HelpLabelSegment::Quad {
        control: [6627, 635],
        to: [6564, 635],
    },
    HelpLabelSegment::Quad {
        control: [6524, 635],
        to: [6483, 659],
    },
    HelpLabelSegment::Quad {
        control: [6442, 682],
        to: [6421, 714],
    },
    HelpLabelSegment::Line([6421, 1180]),
    HelpLabelSegment::Line([6310, 1180]),
    HelpLabelSegment::Line([6310, 295]),
    HelpLabelSegment::Line([6421, 295]),
];

const BLUE_SHOOT_CONTOUR_2: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [7041, 1101],
        to: [7207, 1101],
    },
    HelpLabelSegment::Quad {
        control: [7286, 1101],
        to: [7330, 1038],
    },
    HelpLabelSegment::Quad {
        control: [7374, 975],
        to: [7374, 865],
    },
    HelpLabelSegment::Quad {
        control: [7374, 632],
        to: [7207, 632],
    },
    HelpLabelSegment::Quad {
        control: [7131, 632],
        to: [7087, 694],
    },
    HelpLabelSegment::Quad {
        control: [7041, 756],
        to: [7041, 865],
    },
];

const BLUE_SHOOT_CONTOUR_3: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [7080, 541],
        to: [7207, 541],
    },
    HelpLabelSegment::Quad {
        control: [7342, 541],
        to: [7417, 627],
    },
    HelpLabelSegment::Quad {
        control: [7491, 712],
        to: [7491, 865],
    },
    HelpLabelSegment::Quad {
        control: [7491, 1017],
        to: [7415, 1105],
    },
    HelpLabelSegment::Quad {
        control: [7339, 1192],
        to: [7207, 1192],
    },
    HelpLabelSegment::Quad {
        control: [7074, 1192],
        to: [6999, 1104],
    },
    HelpLabelSegment::Quad {
        control: [6924, 1015],
        to: [6924, 865],
    },
    HelpLabelSegment::Quad {
        control: [6924, 719],
        to: [7002, 630],
    },
];

const BLUE_SHOOT_CONTOUR_4: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [7686, 1101],
        to: [7852, 1101],
    },
    HelpLabelSegment::Quad {
        control: [7931, 1101],
        to: [7975, 1038],
    },
    HelpLabelSegment::Quad {
        control: [8019, 975],
        to: [8019, 865],
    },
    HelpLabelSegment::Quad {
        control: [8019, 632],
        to: [7852, 632],
    },
    HelpLabelSegment::Quad {
        control: [7776, 632],
        to: [7732, 694],
    },
    HelpLabelSegment::Quad {
        control: [7686, 756],
        to: [7686, 865],
    },
];

const BLUE_SHOOT_CONTOUR_5: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [7725, 541],
        to: [7852, 541],
    },
    HelpLabelSegment::Quad {
        control: [7987, 541],
        to: [8062, 627],
    },
    HelpLabelSegment::Quad {
        control: [8136, 712],
        to: [8136, 865],
    },
    HelpLabelSegment::Quad {
        control: [8136, 1017],
        to: [8060, 1105],
    },
    HelpLabelSegment::Quad {
        control: [7984, 1192],
        to: [7852, 1192],
    },
    HelpLabelSegment::Quad {
        control: [7719, 1192],
        to: [7644, 1104],
    },
    HelpLabelSegment::Quad {
        control: [7569, 1015],
        to: [7569, 865],
    },
    HelpLabelSegment::Quad {
        control: [7569, 719],
        to: [7647, 630],
    },
];

const BLUE_SHOOT_CONTOUR_6: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Line([8295, 422]),
    HelpLabelSegment::Line([8406, 378]),
    HelpLabelSegment::Line([8406, 553]),
    HelpLabelSegment::Line([8578, 553]),
    HelpLabelSegment::Line([8578, 641]),
    HelpLabelSegment::Line([8406, 641]),
    HelpLabelSegment::Line([8406, 953]),
    HelpLabelSegment::Quad {
        control: [8406, 1031],
        to: [8433, 1065],
    },
    HelpLabelSegment::Quad {
        control: [8459, 1098],
        to: [8518, 1098],
    },
    HelpLabelSegment::Quad {
        control: [8561, 1098],
        to: [8606, 1077],
    },
    HelpLabelSegment::Line([8623, 1174]),
    HelpLabelSegment::Line([8471, 1192]),
    HelpLabelSegment::Quad {
        control: [8396, 1192],
        to: [8346, 1137],
    },
    HelpLabelSegment::Quad {
        control: [8295, 1082],
        to: [8295, 997],
    },
    HelpLabelSegment::Line([8295, 641]),
    HelpLabelSegment::Line([8222, 641]),
    HelpLabelSegment::Line([8222, 553]),
    HelpLabelSegment::Line([8295, 553]),
];

const BLUE_SHOOT_CONTOURS: [HelpLabelContour; 7] = [
    HelpLabelContour {
        start: [6163, 585],
        segments: &BLUE_SHOOT_CONTOUR_0,
    },
    HelpLabelContour {
        start: [6421, 295],
        segments: &BLUE_SHOOT_CONTOUR_1,
    },
    HelpLabelContour {
        start: [7041, 865],
        segments: &BLUE_SHOOT_CONTOUR_2,
    },
    HelpLabelContour {
        start: [7002, 630],
        segments: &BLUE_SHOOT_CONTOUR_3,
    },
    HelpLabelContour {
        start: [7686, 865],
        segments: &BLUE_SHOOT_CONTOUR_4,
    },
    HelpLabelContour {
        start: [7647, 630],
        segments: &BLUE_SHOOT_CONTOUR_5,
    },
    HelpLabelContour {
        start: [8295, 553],
        segments: &BLUE_SHOOT_CONTOUR_6,
    },
];

pub const BLUE_SHOOT: HelpLabelDefinition = HelpLabelDefinition {
    text: "shoot",
    define_text_id: 114,
    font_ids: &BLUE_SHOOT_FONT_IDS,
    color_rgb: [102, 197, 255],
    bounds_centipx: [5785, 9175, 295, 1190],
    contours: &BLUE_SHOOT_CONTOURS,
};

const BLUE_MOVE_UP_FONT_IDS: [u16; 1] = [26];

const BLUE_MOVE_UP_CONTOUR_0: [HelpLabelSegment; 24] = [
    HelpLabelSegment::Quad {
        control: [5833, 656],
        to: [5833, 760],
    },
    HelpLabelSegment::Line([5833, 1180]),
    HelpLabelSegment::Line([5721, 1180]),
    HelpLabelSegment::Line([5721, 783]),
    HelpLabelSegment::Quad {
        control: [5721, 635],
        to: [5592, 635],
    },
    HelpLabelSegment::Quad {
        control: [5552, 635],
        to: [5517, 660],
    },
    HelpLabelSegment::Quad {
        control: [5482, 684],
        to: [5469, 716],
    },
    HelpLabelSegment::Line([5469, 1180]),
    HelpLabelSegment::Line([5358, 1180]),
    HelpLabelSegment::Line([5358, 735]),
    HelpLabelSegment::Quad {
        control: [5358, 688],
        to: [5323, 662],
    },
    HelpLabelSegment::Quad {
        control: [5288, 635],
        to: [5230, 635],
    },
    HelpLabelSegment::Quad {
        control: [5197, 635],
        to: [5160, 661],
    },
    HelpLabelSegment::Quad {
        control: [5121, 687],
        to: [5106, 717],
    },
    HelpLabelSegment::Line([5106, 1180]),
    HelpLabelSegment::Line([4995, 1180]),
    HelpLabelSegment::Line([4995, 553]),
    HelpLabelSegment::Line([5067, 553]),
    HelpLabelSegment::Line([5104, 626]),
    HelpLabelSegment::Quad {
        control: [5168, 541],
        to: [5265, 541],
    },
    HelpLabelSegment::Quad {
        control: [5400, 541],
        to: [5454, 625],
    },
    HelpLabelSegment::Quad {
        control: [5473, 589],
        to: [5523, 565],
    },
    HelpLabelSegment::Quad {
        control: [5575, 541],
        to: [5629, 541],
    },
    HelpLabelSegment::Quad {
        control: [5726, 541],
        to: [5779, 599],
    },
];

const BLUE_MOVE_UP_CONTOUR_1: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [6066, 1101],
        to: [6232, 1101],
    },
    HelpLabelSegment::Quad {
        control: [6311, 1101],
        to: [6355, 1038],
    },
    HelpLabelSegment::Quad {
        control: [6399, 975],
        to: [6399, 865],
    },
    HelpLabelSegment::Quad {
        control: [6399, 632],
        to: [6232, 632],
    },
    HelpLabelSegment::Quad {
        control: [6156, 632],
        to: [6112, 694],
    },
    HelpLabelSegment::Quad {
        control: [6066, 756],
        to: [6066, 865],
    },
];

const BLUE_MOVE_UP_CONTOUR_2: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [6105, 541],
        to: [6232, 541],
    },
    HelpLabelSegment::Quad {
        control: [6367, 541],
        to: [6442, 627],
    },
    HelpLabelSegment::Quad {
        control: [6516, 712],
        to: [6516, 865],
    },
    HelpLabelSegment::Quad {
        control: [6516, 1017],
        to: [6440, 1105],
    },
    HelpLabelSegment::Quad {
        control: [6364, 1192],
        to: [6232, 1192],
    },
    HelpLabelSegment::Quad {
        control: [6099, 1192],
        to: [6024, 1104],
    },
    HelpLabelSegment::Quad {
        control: [5949, 1015],
        to: [5949, 865],
    },
    HelpLabelSegment::Quad {
        control: [5949, 719],
        to: [6027, 630],
    },
];

const BLUE_MOVE_UP_CONTOUR_3: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([6861, 1192]),
    HelpLabelSegment::Line([6832, 1192]),
    HelpLabelSegment::Line([6562, 551]),
    HelpLabelSegment::Line([6684, 551]),
    HelpLabelSegment::Line([6850, 990]),
    HelpLabelSegment::Line([7019, 551]),
    HelpLabelSegment::Line([7136, 551]),
];

const BLUE_MOVE_UP_CONTOUR_4: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [7409, 635],
        to: [7358, 683],
    },
    HelpLabelSegment::Quad {
        control: [7310, 729],
        to: [7303, 797],
    },
    HelpLabelSegment::Line([7651, 797]),
    HelpLabelSegment::Quad {
        control: [7651, 729],
        to: [7609, 684],
    },
    HelpLabelSegment::Quad {
        control: [7562, 635],
        to: [7482, 635],
    },
];

const BLUE_MOVE_UP_CONTOUR_5: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [7415, 1098],
        to: [7498, 1098],
    },
    HelpLabelSegment::Quad {
        control: [7594, 1098],
        to: [7657, 1043],
    },
    HelpLabelSegment::Line([7704, 1123]),
    HelpLabelSegment::Quad {
        control: [7678, 1148],
        to: [7625, 1167],
    },
    HelpLabelSegment::Quad {
        control: [7559, 1192],
        to: [7477, 1192],
    },
    HelpLabelSegment::Quad {
        control: [7358, 1192],
        to: [7275, 1112],
    },
    HelpLabelSegment::Quad {
        control: [7184, 1023],
        to: [7184, 874],
    },
    HelpLabelSegment::Quad {
        control: [7184, 718],
        to: [7277, 625],
    },
    HelpLabelSegment::Quad {
        control: [7362, 541],
        to: [7478, 541],
    },
    HelpLabelSegment::Quad {
        control: [7611, 541],
        to: [7688, 616],
    },
    HelpLabelSegment::Quad {
        control: [7761, 689],
        to: [7761, 810],
    },
    HelpLabelSegment::Quad {
        control: [7761, 846],
        to: [7753, 878],
    },
    HelpLabelSegment::Line([7301, 878]),
    HelpLabelSegment::Quad {
        control: [7301, 988],
        to: [7361, 1046],
    },
];

const BLUE_MOVE_UP_CONTOUR_6: [HelpLabelSegment; 15] = [
    HelpLabelSegment::Line([8345, 953]),
    HelpLabelSegment::Quad {
        control: [8345, 1098],
        to: [8471, 1098],
    },
    HelpLabelSegment::Quad {
        control: [8526, 1098],
        to: [8571, 1066],
    },
    HelpLabelSegment::Quad {
        control: [8617, 1035],
        to: [8632, 994],
    },
    HelpLabelSegment::Line([8632, 553]),
    HelpLabelSegment::Line([8744, 553]),
    HelpLabelSegment::Line([8744, 1180]),
    HelpLabelSegment::Line([8632, 1180]),
    HelpLabelSegment::Line([8632, 1093]),
    HelpLabelSegment::Quad {
        control: [8614, 1131],
        to: [8557, 1161],
    },
    HelpLabelSegment::Quad {
        control: [8500, 1192],
        to: [8446, 1192],
    },
    HelpLabelSegment::Quad {
        control: [8343, 1192],
        to: [8289, 1133],
    },
    HelpLabelSegment::Quad {
        control: [8234, 1073],
        to: [8234, 964],
    },
    HelpLabelSegment::Line([8234, 553]),
    HelpLabelSegment::Line([8345, 553]),
];

const BLUE_MOVE_UP_CONTOUR_7: [HelpLabelSegment; 12] = [
    HelpLabelSegment::Line([9006, 605]),
    HelpLabelSegment::Quad {
        control: [9069, 541],
        to: [9158, 541],
    },
    HelpLabelSegment::Quad {
        control: [9292, 541],
        to: [9367, 625],
    },
    HelpLabelSegment::Quad {
        control: [9441, 708],
        to: [9441, 868],
    },
    HelpLabelSegment::Quad {
        control: [9441, 1011],
        to: [9366, 1101],
    },
    HelpLabelSegment::Quad {
        control: [9291, 1192],
        to: [9149, 1192],
    },
    HelpLabelSegment::Quad {
        control: [9109, 1192],
        to: [9065, 1178],
    },
    HelpLabelSegment::Quad {
        control: [9019, 1164],
        to: [9006, 1146],
    },
    HelpLabelSegment::Line([9006, 1426]),
    HelpLabelSegment::Line([8895, 1426]),
    HelpLabelSegment::Line([8895, 553]),
    HelpLabelSegment::Line([9006, 553]),
];

const BLUE_MOVE_UP_CONTOUR_8: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Line([9006, 1053]),
    HelpLabelSegment::Quad {
        control: [9017, 1070],
        to: [9051, 1084],
    },
    HelpLabelSegment::Quad {
        control: [9085, 1098],
        to: [9116, 1098],
    },
    HelpLabelSegment::Quad {
        control: [9324, 1098],
        to: [9324, 864],
    },
    HelpLabelSegment::Quad {
        control: [9324, 745],
        to: [9274, 690],
    },
    HelpLabelSegment::Quad {
        control: [9225, 635],
        to: [9117, 635],
    },
    HelpLabelSegment::Quad {
        control: [9094, 635],
        to: [9060, 651],
    },
    HelpLabelSegment::Line([9006, 688]),
];

const BLUE_MOVE_UP_CONTOURS: [HelpLabelContour; 9] = [
    HelpLabelContour {
        start: [5779, 599],
        segments: &BLUE_MOVE_UP_CONTOUR_0,
    },
    HelpLabelContour {
        start: [6066, 865],
        segments: &BLUE_MOVE_UP_CONTOUR_1,
    },
    HelpLabelContour {
        start: [6027, 630],
        segments: &BLUE_MOVE_UP_CONTOUR_2,
    },
    HelpLabelContour {
        start: [7136, 551],
        segments: &BLUE_MOVE_UP_CONTOUR_3,
    },
    HelpLabelContour {
        start: [7482, 635],
        segments: &BLUE_MOVE_UP_CONTOUR_4,
    },
    HelpLabelContour {
        start: [7361, 1046],
        segments: &BLUE_MOVE_UP_CONTOUR_5,
    },
    HelpLabelContour {
        start: [8345, 553],
        segments: &BLUE_MOVE_UP_CONTOUR_6,
    },
    HelpLabelContour {
        start: [9006, 553],
        segments: &BLUE_MOVE_UP_CONTOUR_7,
    },
    HelpLabelContour {
        start: [9006, 688],
        segments: &BLUE_MOVE_UP_CONTOUR_8,
    },
];

pub const BLUE_MOVE_UP: HelpLabelDefinition = HelpLabelDefinition {
    text: "move up",
    define_text_id: 115,
    font_ids: &BLUE_MOVE_UP_FONT_IDS,
    color_rgb: [102, 197, 255],
    bounds_centipx: [4995, 10010, 525, 1425],
    contours: &BLUE_MOVE_UP_CONTOURS,
};

const BLUE_MOVE_DOWN_FONT_IDS: [u16; 1] = [26];

const BLUE_MOVE_DOWN_CONTOUR_0: [HelpLabelSegment; 24] = [
    HelpLabelSegment::Quad {
        control: [5063, 656],
        to: [5063, 760],
    },
    HelpLabelSegment::Line([5063, 1180]),
    HelpLabelSegment::Line([4951, 1180]),
    HelpLabelSegment::Line([4951, 783]),
    HelpLabelSegment::Quad {
        control: [4951, 635],
        to: [4822, 635],
    },
    HelpLabelSegment::Quad {
        control: [4782, 635],
        to: [4747, 660],
    },
    HelpLabelSegment::Quad {
        control: [4712, 684],
        to: [4699, 716],
    },
    HelpLabelSegment::Line([4699, 1180]),
    HelpLabelSegment::Line([4588, 1180]),
    HelpLabelSegment::Line([4588, 735]),
    HelpLabelSegment::Quad {
        control: [4588, 688],
        to: [4553, 662],
    },
    HelpLabelSegment::Quad {
        control: [4518, 635],
        to: [4460, 635],
    },
    HelpLabelSegment::Quad {
        control: [4427, 635],
        to: [4390, 661],
    },
    HelpLabelSegment::Quad {
        control: [4351, 687],
        to: [4336, 717],
    },
    HelpLabelSegment::Line([4336, 1180]),
    HelpLabelSegment::Line([4225, 1180]),
    HelpLabelSegment::Line([4225, 553]),
    HelpLabelSegment::Line([4297, 553]),
    HelpLabelSegment::Line([4334, 626]),
    HelpLabelSegment::Quad {
        control: [4398, 541],
        to: [4495, 541],
    },
    HelpLabelSegment::Quad {
        control: [4630, 541],
        to: [4684, 625],
    },
    HelpLabelSegment::Quad {
        control: [4703, 589],
        to: [4753, 565],
    },
    HelpLabelSegment::Quad {
        control: [4805, 541],
        to: [4859, 541],
    },
    HelpLabelSegment::Quad {
        control: [4956, 541],
        to: [5009, 599],
    },
];

const BLUE_MOVE_DOWN_CONTOUR_1: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [5296, 1101],
        to: [5462, 1101],
    },
    HelpLabelSegment::Quad {
        control: [5541, 1101],
        to: [5585, 1038],
    },
    HelpLabelSegment::Quad {
        control: [5629, 975],
        to: [5629, 865],
    },
    HelpLabelSegment::Quad {
        control: [5629, 632],
        to: [5462, 632],
    },
    HelpLabelSegment::Quad {
        control: [5386, 632],
        to: [5342, 694],
    },
    HelpLabelSegment::Quad {
        control: [5296, 756],
        to: [5296, 865],
    },
];

const BLUE_MOVE_DOWN_CONTOUR_2: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [5335, 541],
        to: [5462, 541],
    },
    HelpLabelSegment::Quad {
        control: [5597, 541],
        to: [5672, 627],
    },
    HelpLabelSegment::Quad {
        control: [5746, 712],
        to: [5746, 865],
    },
    HelpLabelSegment::Quad {
        control: [5746, 1017],
        to: [5670, 1105],
    },
    HelpLabelSegment::Quad {
        control: [5594, 1192],
        to: [5462, 1192],
    },
    HelpLabelSegment::Quad {
        control: [5329, 1192],
        to: [5254, 1104],
    },
    HelpLabelSegment::Quad {
        control: [5179, 1015],
        to: [5179, 865],
    },
    HelpLabelSegment::Quad {
        control: [5179, 719],
        to: [5257, 630],
    },
];

const BLUE_MOVE_DOWN_CONTOUR_3: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([6091, 1192]),
    HelpLabelSegment::Line([6062, 1192]),
    HelpLabelSegment::Line([5792, 551]),
    HelpLabelSegment::Line([5914, 551]),
    HelpLabelSegment::Line([6080, 990]),
    HelpLabelSegment::Line([6249, 551]),
    HelpLabelSegment::Line([6366, 551]),
];

const BLUE_MOVE_DOWN_CONTOUR_4: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [6639, 635],
        to: [6588, 683],
    },
    HelpLabelSegment::Quad {
        control: [6540, 729],
        to: [6533, 797],
    },
    HelpLabelSegment::Line([6881, 797]),
    HelpLabelSegment::Quad {
        control: [6881, 729],
        to: [6839, 684],
    },
    HelpLabelSegment::Quad {
        control: [6792, 635],
        to: [6712, 635],
    },
];

const BLUE_MOVE_DOWN_CONTOUR_5: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [6645, 1098],
        to: [6728, 1098],
    },
    HelpLabelSegment::Quad {
        control: [6824, 1098],
        to: [6887, 1043],
    },
    HelpLabelSegment::Line([6934, 1123]),
    HelpLabelSegment::Quad {
        control: [6908, 1148],
        to: [6855, 1167],
    },
    HelpLabelSegment::Quad {
        control: [6789, 1192],
        to: [6707, 1192],
    },
    HelpLabelSegment::Quad {
        control: [6588, 1192],
        to: [6505, 1112],
    },
    HelpLabelSegment::Quad {
        control: [6414, 1023],
        to: [6414, 874],
    },
    HelpLabelSegment::Quad {
        control: [6414, 718],
        to: [6507, 625],
    },
    HelpLabelSegment::Quad {
        control: [6592, 541],
        to: [6708, 541],
    },
    HelpLabelSegment::Quad {
        control: [6841, 541],
        to: [6918, 616],
    },
    HelpLabelSegment::Quad {
        control: [6991, 689],
        to: [6991, 810],
    },
    HelpLabelSegment::Quad {
        control: [6991, 846],
        to: [6983, 878],
    },
    HelpLabelSegment::Line([6531, 878]),
    HelpLabelSegment::Quad {
        control: [6531, 988],
        to: [6591, 1046],
    },
];

const BLUE_MOVE_DOWN_CONTOUR_6: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Line([7978, 295]),
    HelpLabelSegment::Line([7978, 1180]),
    HelpLabelSegment::Line([7867, 1180]),
    HelpLabelSegment::Line([7867, 1133]),
    HelpLabelSegment::Quad {
        control: [7810, 1192],
        to: [7698, 1192],
    },
    HelpLabelSegment::Quad {
        control: [7581, 1192],
        to: [7507, 1107],
    },
    HelpLabelSegment::Quad {
        control: [7435, 1023],
        to: [7435, 882],
    },
    HelpLabelSegment::Quad {
        control: [7435, 741],
        to: [7519, 641],
    },
    HelpLabelSegment::Quad {
        control: [7603, 541],
        to: [7719, 541],
    },
    HelpLabelSegment::Quad {
        control: [7817, 541],
        to: [7867, 587],
    },
    HelpLabelSegment::Line([7867, 295]),
];

const BLUE_MOVE_DOWN_CONTOUR_7: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Quad {
        control: [7858, 1065],
        to: [7867, 1046],
    },
    HelpLabelSegment::Line([7867, 698]),
    HelpLabelSegment::Quad {
        control: [7825, 635],
        to: [7752, 635],
    },
    HelpLabelSegment::Quad {
        control: [7662, 635],
        to: [7607, 702],
    },
    HelpLabelSegment::Quad {
        control: [7552, 769],
        to: [7552, 872],
    },
    HelpLabelSegment::Quad {
        control: [7552, 1098],
        to: [7758, 1098],
    },
    HelpLabelSegment::Quad {
        control: [7784, 1098],
        to: [7821, 1082],
    },
];

const BLUE_MOVE_DOWN_CONTOUR_8: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [8216, 1101],
        to: [8382, 1101],
    },
    HelpLabelSegment::Quad {
        control: [8461, 1101],
        to: [8505, 1038],
    },
    HelpLabelSegment::Quad {
        control: [8549, 975],
        to: [8549, 865],
    },
    HelpLabelSegment::Quad {
        control: [8549, 632],
        to: [8382, 632],
    },
    HelpLabelSegment::Quad {
        control: [8306, 632],
        to: [8262, 694],
    },
    HelpLabelSegment::Quad {
        control: [8216, 756],
        to: [8216, 865],
    },
];

const BLUE_MOVE_DOWN_CONTOUR_9: [HelpLabelSegment; 8] = [
    HelpLabelSegment::Quad {
        control: [8255, 541],
        to: [8382, 541],
    },
    HelpLabelSegment::Quad {
        control: [8517, 541],
        to: [8592, 627],
    },
    HelpLabelSegment::Quad {
        control: [8666, 712],
        to: [8666, 865],
    },
    HelpLabelSegment::Quad {
        control: [8666, 1017],
        to: [8590, 1105],
    },
    HelpLabelSegment::Quad {
        control: [8514, 1192],
        to: [8382, 1192],
    },
    HelpLabelSegment::Quad {
        control: [8249, 1192],
        to: [8174, 1104],
    },
    HelpLabelSegment::Quad {
        control: [8099, 1015],
        to: [8099, 865],
    },
    HelpLabelSegment::Quad {
        control: [8099, 719],
        to: [8177, 630],
    },
];

const BLUE_MOVE_DOWN_CONTOUR_10: [HelpLabelSegment; 13] = [
    HelpLabelSegment::Line([9365, 1192]),
    HelpLabelSegment::Line([9335, 1192]),
    HelpLabelSegment::Line([9151, 765]),
    HelpLabelSegment::Line([8968, 1192]),
    HelpLabelSegment::Line([8938, 1192]),
    HelpLabelSegment::Line([8714, 551]),
    HelpLabelSegment::Line([8833, 551]),
    HelpLabelSegment::Line([8968, 963]),
    HelpLabelSegment::Line([9134, 551]),
    HelpLabelSegment::Line([9163, 551]),
    HelpLabelSegment::Line([9335, 963]),
    HelpLabelSegment::Line([9480, 551]),
    HelpLabelSegment::Line([9590, 551]),
];

const BLUE_MOVE_DOWN_CONTOUR_11: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Line([10177, 1180]),
    HelpLabelSegment::Line([10065, 1180]),
    HelpLabelSegment::Line([10065, 816]),
    HelpLabelSegment::Quad {
        control: [10065, 715],
        to: [10036, 675],
    },
    HelpLabelSegment::Quad {
        control: [10005, 635],
        to: [9934, 635],
    },
    HelpLabelSegment::Quad {
        control: [9896, 635],
        to: [9854, 657],
    },
    HelpLabelSegment::Quad {
        control: [9813, 681],
        to: [9791, 714],
    },
    HelpLabelSegment::Line([9791, 1180]),
    HelpLabelSegment::Line([9680, 1180]),
    HelpLabelSegment::Line([9680, 553]),
    HelpLabelSegment::Line([9756, 553]),
    HelpLabelSegment::Line([9791, 634]),
    HelpLabelSegment::Quad {
        control: [9846, 541],
        to: [9970, 541],
    },
    HelpLabelSegment::Quad {
        control: [10177, 541],
        to: [10177, 792],
    },
];

const BLUE_MOVE_DOWN_CONTOURS: [HelpLabelContour; 12] = [
    HelpLabelContour {
        start: [5009, 599],
        segments: &BLUE_MOVE_DOWN_CONTOUR_0,
    },
    HelpLabelContour {
        start: [5296, 865],
        segments: &BLUE_MOVE_DOWN_CONTOUR_1,
    },
    HelpLabelContour {
        start: [5257, 630],
        segments: &BLUE_MOVE_DOWN_CONTOUR_2,
    },
    HelpLabelContour {
        start: [6366, 551],
        segments: &BLUE_MOVE_DOWN_CONTOUR_3,
    },
    HelpLabelContour {
        start: [6712, 635],
        segments: &BLUE_MOVE_DOWN_CONTOUR_4,
    },
    HelpLabelContour {
        start: [6591, 1046],
        segments: &BLUE_MOVE_DOWN_CONTOUR_5,
    },
    HelpLabelContour {
        start: [7867, 295],
        segments: &BLUE_MOVE_DOWN_CONTOUR_6,
    },
    HelpLabelContour {
        start: [7821, 1082],
        segments: &BLUE_MOVE_DOWN_CONTOUR_7,
    },
    HelpLabelContour {
        start: [8216, 865],
        segments: &BLUE_MOVE_DOWN_CONTOUR_8,
    },
    HelpLabelContour {
        start: [8177, 630],
        segments: &BLUE_MOVE_DOWN_CONTOUR_9,
    },
    HelpLabelContour {
        start: [9590, 551],
        segments: &BLUE_MOVE_DOWN_CONTOUR_10,
    },
    HelpLabelContour {
        start: [10177, 792],
        segments: &BLUE_MOVE_DOWN_CONTOUR_11,
    },
];

pub const BLUE_MOVE_DOWN: HelpLabelDefinition = HelpLabelDefinition {
    text: "move down",
    define_text_id: 116,
    font_ids: &BLUE_MOVE_DOWN_FONT_IDS,
    color_rgb: [102, 197, 255],
    bounds_centipx: [4225, 10780, 295, 1190],
    contours: &BLUE_MOVE_DOWN_CONTOURS,
};

const PLAYER_RED_FONT_IDS: [u16; 1] = [26];

const PLAYER_RED_CONTOUR_0: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Quad {
        control: [4972, 316],
        to: [4972, 566],
    },
    HelpLabelSegment::Quad {
        control: [4972, 855],
        to: [4642, 855],
    },
    HelpLabelSegment::Line([4555, 850]),
    HelpLabelSegment::Line([4555, 1180]),
    HelpLabelSegment::Line([4438, 1180]),
    HelpLabelSegment::Line([4438, 322]),
    HelpLabelSegment::Line([4598, 316]),
];

const PLAYER_RED_CONTOUR_1: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Line([4555, 744]),
    HelpLabelSegment::Line([4634, 750]),
    HelpLabelSegment::Quad {
        control: [4852, 750],
        to: [4852, 579],
    },
    HelpLabelSegment::Quad {
        control: [4852, 422],
        to: [4620, 422],
    },
    HelpLabelSegment::Line([4555, 428]),
];

const PLAYER_RED_CONTOUR_2: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([5325, 1192]),
    HelpLabelSegment::Quad {
        control: [5108, 1192],
        to: [5108, 1003],
    },
    HelpLabelSegment::Line([5108, 295]),
    HelpLabelSegment::Line([5219, 295]),
    HelpLabelSegment::Line([5219, 984]),
    HelpLabelSegment::Quad {
        control: [5219, 1035],
        to: [5249, 1064],
    },
    HelpLabelSegment::Quad {
        control: [5278, 1092],
        to: [5325, 1092],
    },
];

const PLAYER_RED_CONTOUR_3: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Quad {
        control: [5786, 541],
        to: [5848, 603],
    },
    HelpLabelSegment::Quad {
        control: [5909, 666],
        to: [5909, 800],
    },
    HelpLabelSegment::Line([5909, 1025]),
    HelpLabelSegment::Quad {
        control: [5909, 1109],
        to: [5959, 1135],
    },
    HelpLabelSegment::Line([5959, 1192]),
    HelpLabelSegment::Quad {
        control: [5891, 1192],
        to: [5858, 1172],
    },
    HelpLabelSegment::Quad {
        control: [5824, 1153],
        to: [5809, 1109],
    },
    HelpLabelSegment::Quad {
        control: [5742, 1192],
        to: [5605, 1192],
    },
    HelpLabelSegment::Quad {
        control: [5531, 1192],
        to: [5477, 1139],
    },
    HelpLabelSegment::Quad {
        control: [5422, 1085],
        to: [5422, 1005],
    },
    HelpLabelSegment::Quad {
        control: [5422, 909],
        to: [5506, 844],
    },
    HelpLabelSegment::Quad {
        control: [5589, 778],
        to: [5718, 778],
    },
    HelpLabelSegment::Line([5798, 793]),
    HelpLabelSegment::Quad {
        control: [5798, 641],
        to: [5662, 641],
    },
    HelpLabelSegment::Quad {
        control: [5558, 641],
        to: [5502, 697],
    },
    HelpLabelSegment::Line([5455, 603]),
    HelpLabelSegment::Quad {
        control: [5486, 578],
        to: [5543, 560],
    },
    HelpLabelSegment::Quad {
        control: [5599, 541],
        to: [5649, 541],
    },
];

const PLAYER_RED_CONTOUR_4: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [5640, 860],
        to: [5587, 903],
    },
    HelpLabelSegment::Quad {
        control: [5533, 947],
        to: [5533, 1007],
    },
    HelpLabelSegment::Quad {
        control: [5533, 1104],
        to: [5649, 1104],
    },
    HelpLabelSegment::Quad {
        control: [5734, 1104],
        to: [5798, 1024],
    },
    HelpLabelSegment::Line([5798, 872]),
    HelpLabelSegment::Line([5724, 860]),
];

const PLAYER_RED_CONTOUR_5: [HelpLabelSegment; 13] = [
    HelpLabelSegment::Line([6324, 1287]),
    HelpLabelSegment::Quad {
        control: [6303, 1346],
        to: [6234, 1386],
    },
    HelpLabelSegment::Quad {
        control: [6163, 1426],
        to: [6078, 1426],
    },
    HelpLabelSegment::Line([6078, 1326]),
    HelpLabelSegment::Quad {
        control: [6148, 1326],
        to: [6197, 1295],
    },
    HelpLabelSegment::Quad {
        control: [6248, 1262],
        to: [6248, 1215],
    },
    HelpLabelSegment::Quad {
        control: [6248, 1164],
        to: [6229, 1113],
    },
    HelpLabelSegment::Line([6182, 989]),
    HelpLabelSegment::Line([6012, 553]),
    HelpLabelSegment::Line([6126, 553]),
    HelpLabelSegment::Line([6311, 1038]),
    HelpLabelSegment::Line([6476, 553]),
    HelpLabelSegment::Line([6590, 553]),
];

const PLAYER_RED_CONTOUR_6: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [6859, 635],
        to: [6808, 683],
    },
    HelpLabelSegment::Quad {
        control: [6760, 729],
        to: [6753, 797],
    },
    HelpLabelSegment::Line([7101, 797]),
    HelpLabelSegment::Quad {
        control: [7101, 729],
        to: [7059, 684],
    },
    HelpLabelSegment::Quad {
        control: [7012, 635],
        to: [6932, 635],
    },
];

const PLAYER_RED_CONTOUR_7: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [6865, 1098],
        to: [6948, 1098],
    },
    HelpLabelSegment::Quad {
        control: [7044, 1098],
        to: [7107, 1043],
    },
    HelpLabelSegment::Line([7154, 1123]),
    HelpLabelSegment::Quad {
        control: [7128, 1148],
        to: [7075, 1167],
    },
    HelpLabelSegment::Quad {
        control: [7009, 1192],
        to: [6927, 1192],
    },
    HelpLabelSegment::Quad {
        control: [6808, 1192],
        to: [6725, 1112],
    },
    HelpLabelSegment::Quad {
        control: [6634, 1023],
        to: [6634, 874],
    },
    HelpLabelSegment::Quad {
        control: [6634, 718],
        to: [6727, 625],
    },
    HelpLabelSegment::Quad {
        control: [6812, 541],
        to: [6928, 541],
    },
    HelpLabelSegment::Quad {
        control: [7061, 541],
        to: [7138, 616],
    },
    HelpLabelSegment::Quad {
        control: [7211, 689],
        to: [7211, 810],
    },
    HelpLabelSegment::Quad {
        control: [7211, 846],
        to: [7203, 878],
    },
    HelpLabelSegment::Line([6751, 878]),
    HelpLabelSegment::Quad {
        control: [6751, 988],
        to: [6811, 1046],
    },
];

const PLAYER_RED_CONTOUR_8: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Quad {
        control: [7634, 635],
        to: [7597, 635],
    },
    HelpLabelSegment::Quad {
        control: [7538, 635],
        to: [7494, 689],
    },
    HelpLabelSegment::Quad {
        control: [7449, 744],
        to: [7449, 820],
    },
    HelpLabelSegment::Line([7449, 1180]),
    HelpLabelSegment::Line([7338, 1180]),
    HelpLabelSegment::Line([7338, 553]),
    HelpLabelSegment::Line([7449, 553]),
    HelpLabelSegment::Line([7449, 653]),
    HelpLabelSegment::Quad {
        control: [7510, 541],
        to: [7631, 541],
    },
    HelpLabelSegment::Line([7716, 552]),
    HelpLabelSegment::Line([7671, 660]),
];

const PLAYER_RED_CONTOUR_9: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([8286, 707]),
    HelpLabelSegment::Line([8368, 712]),
    HelpLabelSegment::Quad {
        control: [8473, 712],
        to: [8521, 678],
    },
    HelpLabelSegment::Quad {
        control: [8570, 644],
        to: [8570, 557],
    },
    HelpLabelSegment::Quad {
        control: [8570, 485],
        to: [8518, 453],
    },
    HelpLabelSegment::Quad {
        control: [8466, 422],
        to: [8355, 422],
    },
    HelpLabelSegment::Line([8286, 428]),
];

const PLAYER_RED_CONTOUR_10: [HelpLabelSegment; 12] = [
    HelpLabelSegment::Line([8383, 313]),
    HelpLabelSegment::Quad {
        control: [8693, 313],
        to: [8693, 559],
    },
    HelpLabelSegment::Quad {
        control: [8693, 640],
        to: [8643, 705],
    },
    HelpLabelSegment::Quad {
        control: [8593, 770],
        to: [8525, 785],
    },
    HelpLabelSegment::Line([8773, 1180]),
    HelpLabelSegment::Line([8640, 1180]),
    HelpLabelSegment::Line([8410, 812]),
    HelpLabelSegment::Line([8286, 806]),
    HelpLabelSegment::Line([8286, 1180]),
    HelpLabelSegment::Line([8169, 1180]),
    HelpLabelSegment::Line([8169, 322]),
    HelpLabelSegment::Line([8260, 318]),
];

const PLAYER_RED_CONTOUR_11: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [8989, 635],
        to: [8938, 683],
    },
    HelpLabelSegment::Quad {
        control: [8890, 729],
        to: [8883, 797],
    },
    HelpLabelSegment::Line([9231, 797]),
    HelpLabelSegment::Quad {
        control: [9231, 729],
        to: [9189, 684],
    },
    HelpLabelSegment::Quad {
        control: [9142, 635],
        to: [9062, 635],
    },
];

const PLAYER_RED_CONTOUR_12: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [8995, 1098],
        to: [9078, 1098],
    },
    HelpLabelSegment::Quad {
        control: [9174, 1098],
        to: [9237, 1043],
    },
    HelpLabelSegment::Line([9284, 1123]),
    HelpLabelSegment::Quad {
        control: [9258, 1148],
        to: [9205, 1167],
    },
    HelpLabelSegment::Quad {
        control: [9139, 1192],
        to: [9057, 1192],
    },
    HelpLabelSegment::Quad {
        control: [8938, 1192],
        to: [8855, 1112],
    },
    HelpLabelSegment::Quad {
        control: [8764, 1023],
        to: [8764, 874],
    },
    HelpLabelSegment::Quad {
        control: [8764, 718],
        to: [8857, 625],
    },
    HelpLabelSegment::Quad {
        control: [8942, 541],
        to: [9058, 541],
    },
    HelpLabelSegment::Quad {
        control: [9191, 541],
        to: [9268, 616],
    },
    HelpLabelSegment::Quad {
        control: [9341, 689],
        to: [9341, 810],
    },
    HelpLabelSegment::Quad {
        control: [9341, 846],
        to: [9333, 878],
    },
    HelpLabelSegment::Line([8881, 878]),
    HelpLabelSegment::Quad {
        control: [8881, 988],
        to: [8941, 1046],
    },
];

const PLAYER_RED_CONTOUR_13: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Line([9968, 295]),
    HelpLabelSegment::Line([9968, 1180]),
    HelpLabelSegment::Line([9857, 1180]),
    HelpLabelSegment::Line([9857, 1133]),
    HelpLabelSegment::Quad {
        control: [9800, 1192],
        to: [9688, 1192],
    },
    HelpLabelSegment::Quad {
        control: [9571, 1192],
        to: [9497, 1107],
    },
    HelpLabelSegment::Quad {
        control: [9425, 1023],
        to: [9425, 882],
    },
    HelpLabelSegment::Quad {
        control: [9425, 741],
        to: [9509, 641],
    },
    HelpLabelSegment::Quad {
        control: [9593, 541],
        to: [9709, 541],
    },
    HelpLabelSegment::Quad {
        control: [9807, 541],
        to: [9857, 587],
    },
    HelpLabelSegment::Line([9857, 295]),
];

const PLAYER_RED_CONTOUR_14: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Quad {
        control: [9848, 1065],
        to: [9857, 1046],
    },
    HelpLabelSegment::Line([9857, 698]),
    HelpLabelSegment::Quad {
        control: [9815, 635],
        to: [9742, 635],
    },
    HelpLabelSegment::Quad {
        control: [9652, 635],
        to: [9597, 702],
    },
    HelpLabelSegment::Quad {
        control: [9542, 769],
        to: [9542, 872],
    },
    HelpLabelSegment::Quad {
        control: [9542, 1098],
        to: [9748, 1098],
    },
    HelpLabelSegment::Quad {
        control: [9774, 1098],
        to: [9811, 1082],
    },
];

const PLAYER_RED_CONTOURS: [HelpLabelContour; 15] = [
    HelpLabelContour {
        start: [4598, 316],
        segments: &PLAYER_RED_CONTOUR_0,
    },
    HelpLabelContour {
        start: [4555, 428],
        segments: &PLAYER_RED_CONTOUR_1,
    },
    HelpLabelContour {
        start: [5325, 1092],
        segments: &PLAYER_RED_CONTOUR_2,
    },
    HelpLabelContour {
        start: [5649, 541],
        segments: &PLAYER_RED_CONTOUR_3,
    },
    HelpLabelContour {
        start: [5724, 860],
        segments: &PLAYER_RED_CONTOUR_4,
    },
    HelpLabelContour {
        start: [6590, 553],
        segments: &PLAYER_RED_CONTOUR_5,
    },
    HelpLabelContour {
        start: [6932, 635],
        segments: &PLAYER_RED_CONTOUR_6,
    },
    HelpLabelContour {
        start: [6811, 1046],
        segments: &PLAYER_RED_CONTOUR_7,
    },
    HelpLabelContour {
        start: [7671, 660],
        segments: &PLAYER_RED_CONTOUR_8,
    },
    HelpLabelContour {
        start: [8286, 428],
        segments: &PLAYER_RED_CONTOUR_9,
    },
    HelpLabelContour {
        start: [8260, 318],
        segments: &PLAYER_RED_CONTOUR_10,
    },
    HelpLabelContour {
        start: [9062, 635],
        segments: &PLAYER_RED_CONTOUR_11,
    },
    HelpLabelContour {
        start: [8941, 1046],
        segments: &PLAYER_RED_CONTOUR_12,
    },
    HelpLabelContour {
        start: [9857, 295],
        segments: &PLAYER_RED_CONTOUR_13,
    },
    HelpLabelContour {
        start: [9811, 1082],
        segments: &PLAYER_RED_CONTOUR_14,
    },
];

pub const PLAYER_RED: HelpLabelDefinition = HelpLabelDefinition {
    text: "Player Red",
    define_text_id: 117,
    font_ids: &PLAYER_RED_FONT_IDS,
    color_rgb: [255, 129, 34],
    bounds_centipx: [4440, 10575, 295, 1425],
    contours: &PLAYER_RED_CONTOURS,
};

const PLAYER_BLUE_FONT_IDS: [u16; 1] = [26];

const PLAYER_BLUE_CONTOUR_0: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Quad {
        control: [4787, 316],
        to: [4787, 566],
    },
    HelpLabelSegment::Quad {
        control: [4787, 855],
        to: [4457, 855],
    },
    HelpLabelSegment::Line([4370, 850]),
    HelpLabelSegment::Line([4370, 1180]),
    HelpLabelSegment::Line([4253, 1180]),
    HelpLabelSegment::Line([4253, 322]),
    HelpLabelSegment::Line([4413, 316]),
];

const PLAYER_BLUE_CONTOUR_1: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Line([4370, 744]),
    HelpLabelSegment::Line([4449, 750]),
    HelpLabelSegment::Quad {
        control: [4667, 750],
        to: [4667, 579],
    },
    HelpLabelSegment::Quad {
        control: [4667, 422],
        to: [4435, 422],
    },
    HelpLabelSegment::Line([4370, 428]),
];

const PLAYER_BLUE_CONTOUR_2: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([5140, 1192]),
    HelpLabelSegment::Quad {
        control: [4923, 1192],
        to: [4923, 1003],
    },
    HelpLabelSegment::Line([4923, 295]),
    HelpLabelSegment::Line([5034, 295]),
    HelpLabelSegment::Line([5034, 984]),
    HelpLabelSegment::Quad {
        control: [5034, 1035],
        to: [5064, 1064],
    },
    HelpLabelSegment::Quad {
        control: [5093, 1092],
        to: [5140, 1092],
    },
];

const PLAYER_BLUE_CONTOUR_3: [HelpLabelSegment; 18] = [
    HelpLabelSegment::Quad {
        control: [5601, 541],
        to: [5663, 603],
    },
    HelpLabelSegment::Quad {
        control: [5724, 666],
        to: [5724, 800],
    },
    HelpLabelSegment::Line([5724, 1025]),
    HelpLabelSegment::Quad {
        control: [5724, 1109],
        to: [5774, 1135],
    },
    HelpLabelSegment::Line([5774, 1192]),
    HelpLabelSegment::Quad {
        control: [5706, 1192],
        to: [5673, 1172],
    },
    HelpLabelSegment::Quad {
        control: [5639, 1153],
        to: [5624, 1109],
    },
    HelpLabelSegment::Quad {
        control: [5557, 1192],
        to: [5420, 1192],
    },
    HelpLabelSegment::Quad {
        control: [5346, 1192],
        to: [5292, 1139],
    },
    HelpLabelSegment::Quad {
        control: [5237, 1085],
        to: [5237, 1005],
    },
    HelpLabelSegment::Quad {
        control: [5237, 909],
        to: [5321, 844],
    },
    HelpLabelSegment::Quad {
        control: [5404, 778],
        to: [5533, 778],
    },
    HelpLabelSegment::Line([5613, 793]),
    HelpLabelSegment::Quad {
        control: [5613, 641],
        to: [5477, 641],
    },
    HelpLabelSegment::Quad {
        control: [5373, 641],
        to: [5317, 697],
    },
    HelpLabelSegment::Line([5270, 603]),
    HelpLabelSegment::Quad {
        control: [5301, 578],
        to: [5358, 560],
    },
    HelpLabelSegment::Quad {
        control: [5414, 541],
        to: [5464, 541],
    },
];

const PLAYER_BLUE_CONTOUR_4: [HelpLabelSegment; 6] = [
    HelpLabelSegment::Quad {
        control: [5455, 860],
        to: [5402, 903],
    },
    HelpLabelSegment::Quad {
        control: [5348, 947],
        to: [5348, 1007],
    },
    HelpLabelSegment::Quad {
        control: [5348, 1104],
        to: [5464, 1104],
    },
    HelpLabelSegment::Quad {
        control: [5549, 1104],
        to: [5613, 1024],
    },
    HelpLabelSegment::Line([5613, 872]),
    HelpLabelSegment::Line([5539, 860]),
];

const PLAYER_BLUE_CONTOUR_5: [HelpLabelSegment; 13] = [
    HelpLabelSegment::Line([6139, 1287]),
    HelpLabelSegment::Quad {
        control: [6118, 1346],
        to: [6049, 1386],
    },
    HelpLabelSegment::Quad {
        control: [5978, 1426],
        to: [5893, 1426],
    },
    HelpLabelSegment::Line([5893, 1326]),
    HelpLabelSegment::Quad {
        control: [5963, 1326],
        to: [6012, 1295],
    },
    HelpLabelSegment::Quad {
        control: [6063, 1262],
        to: [6063, 1215],
    },
    HelpLabelSegment::Quad {
        control: [6063, 1164],
        to: [6044, 1113],
    },
    HelpLabelSegment::Line([5997, 989]),
    HelpLabelSegment::Line([5827, 553]),
    HelpLabelSegment::Line([5941, 553]),
    HelpLabelSegment::Line([6126, 1038]),
    HelpLabelSegment::Line([6291, 553]),
    HelpLabelSegment::Line([6405, 553]),
];

const PLAYER_BLUE_CONTOUR_6: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [6674, 635],
        to: [6623, 683],
    },
    HelpLabelSegment::Quad {
        control: [6575, 729],
        to: [6568, 797],
    },
    HelpLabelSegment::Line([6916, 797]),
    HelpLabelSegment::Quad {
        control: [6916, 729],
        to: [6874, 684],
    },
    HelpLabelSegment::Quad {
        control: [6827, 635],
        to: [6747, 635],
    },
];

const PLAYER_BLUE_CONTOUR_7: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [6680, 1098],
        to: [6763, 1098],
    },
    HelpLabelSegment::Quad {
        control: [6859, 1098],
        to: [6922, 1043],
    },
    HelpLabelSegment::Line([6969, 1123]),
    HelpLabelSegment::Quad {
        control: [6943, 1148],
        to: [6890, 1167],
    },
    HelpLabelSegment::Quad {
        control: [6824, 1192],
        to: [6742, 1192],
    },
    HelpLabelSegment::Quad {
        control: [6623, 1192],
        to: [6540, 1112],
    },
    HelpLabelSegment::Quad {
        control: [6449, 1023],
        to: [6449, 874],
    },
    HelpLabelSegment::Quad {
        control: [6449, 718],
        to: [6542, 625],
    },
    HelpLabelSegment::Quad {
        control: [6627, 541],
        to: [6743, 541],
    },
    HelpLabelSegment::Quad {
        control: [6876, 541],
        to: [6953, 616],
    },
    HelpLabelSegment::Quad {
        control: [7026, 689],
        to: [7026, 810],
    },
    HelpLabelSegment::Quad {
        control: [7026, 846],
        to: [7018, 878],
    },
    HelpLabelSegment::Line([6566, 878]),
    HelpLabelSegment::Quad {
        control: [6566, 988],
        to: [6626, 1046],
    },
];

const PLAYER_BLUE_CONTOUR_8: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Quad {
        control: [7449, 635],
        to: [7412, 635],
    },
    HelpLabelSegment::Quad {
        control: [7353, 635],
        to: [7309, 689],
    },
    HelpLabelSegment::Quad {
        control: [7264, 744],
        to: [7264, 820],
    },
    HelpLabelSegment::Line([7264, 1180]),
    HelpLabelSegment::Line([7153, 1180]),
    HelpLabelSegment::Line([7153, 553]),
    HelpLabelSegment::Line([7264, 553]),
    HelpLabelSegment::Line([7264, 653]),
    HelpLabelSegment::Quad {
        control: [7325, 541],
        to: [7446, 541],
    },
    HelpLabelSegment::Line([7531, 552]),
    HelpLabelSegment::Line([7486, 660]),
];

const PLAYER_BLUE_CONTOUR_9: [HelpLabelSegment; 11] = [
    HelpLabelSegment::Quad {
        control: [8470, 586],
        to: [8426, 635],
    },
    HelpLabelSegment::Quad {
        control: [8381, 684],
        to: [8327, 696],
    },
    HelpLabelSegment::Quad {
        control: [8429, 721],
        to: [8476, 778],
    },
    HelpLabelSegment::Quad {
        control: [8523, 834],
        to: [8523, 933],
    },
    HelpLabelSegment::Quad {
        control: [8523, 1045],
        to: [8440, 1113],
    },
    HelpLabelSegment::Quad {
        control: [8356, 1180],
        to: [8223, 1180],
    },
    HelpLabelSegment::Line([7978, 1180]),
    HelpLabelSegment::Line([7978, 322]),
    HelpLabelSegment::Line([8204, 314]),
    HelpLabelSegment::Quad {
        control: [8331, 314],
        to: [8401, 369],
    },
    HelpLabelSegment::Quad {
        control: [8470, 424],
        to: [8470, 526],
    },
];

const PLAYER_BLUE_CONTOUR_10: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Line([8095, 662]),
    HelpLabelSegment::Line([8186, 666]),
    HelpLabelSegment::Quad {
        control: [8353, 666],
        to: [8353, 531],
    },
    HelpLabelSegment::Quad {
        control: [8353, 411],
        to: [8201, 411],
    },
    HelpLabelSegment::Line([8095, 416]),
];

const PLAYER_BLUE_CONTOUR_11: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([8095, 1080]),
    HelpLabelSegment::Line([8188, 1086]),
    HelpLabelSegment::Quad {
        control: [8298, 1086],
        to: [8349, 1045],
    },
    HelpLabelSegment::Quad {
        control: [8400, 1004],
        to: [8400, 914],
    },
    HelpLabelSegment::Quad {
        control: [8400, 830],
        to: [8352, 790],
    },
    HelpLabelSegment::Quad {
        control: [8302, 750],
        to: [8190, 750],
    },
    HelpLabelSegment::Line([8095, 753]),
];

const PLAYER_BLUE_CONTOUR_12: [HelpLabelSegment; 7] = [
    HelpLabelSegment::Line([8875, 1192]),
    HelpLabelSegment::Quad {
        control: [8658, 1192],
        to: [8658, 1003],
    },
    HelpLabelSegment::Line([8658, 295]),
    HelpLabelSegment::Line([8769, 295]),
    HelpLabelSegment::Line([8769, 984]),
    HelpLabelSegment::Quad {
        control: [8769, 1035],
        to: [8799, 1064],
    },
    HelpLabelSegment::Quad {
        control: [8828, 1092],
        to: [8875, 1092],
    },
];

const PLAYER_BLUE_CONTOUR_13: [HelpLabelSegment; 15] = [
    HelpLabelSegment::Line([9110, 953]),
    HelpLabelSegment::Quad {
        control: [9110, 1098],
        to: [9236, 1098],
    },
    HelpLabelSegment::Quad {
        control: [9291, 1098],
        to: [9336, 1066],
    },
    HelpLabelSegment::Quad {
        control: [9382, 1035],
        to: [9397, 994],
    },
    HelpLabelSegment::Line([9397, 553]),
    HelpLabelSegment::Line([9509, 553]),
    HelpLabelSegment::Line([9509, 1180]),
    HelpLabelSegment::Line([9397, 1180]),
    HelpLabelSegment::Line([9397, 1093]),
    HelpLabelSegment::Quad {
        control: [9379, 1131],
        to: [9322, 1161],
    },
    HelpLabelSegment::Quad {
        control: [9265, 1192],
        to: [9211, 1192],
    },
    HelpLabelSegment::Quad {
        control: [9108, 1192],
        to: [9054, 1133],
    },
    HelpLabelSegment::Quad {
        control: [8999, 1073],
        to: [8999, 964],
    },
    HelpLabelSegment::Line([8999, 553]),
    HelpLabelSegment::Line([9110, 553]),
];

const PLAYER_BLUE_CONTOUR_14: [HelpLabelSegment; 5] = [
    HelpLabelSegment::Quad {
        control: [9844, 635],
        to: [9793, 683],
    },
    HelpLabelSegment::Quad {
        control: [9745, 729],
        to: [9738, 797],
    },
    HelpLabelSegment::Line([10086, 797]),
    HelpLabelSegment::Quad {
        control: [10086, 729],
        to: [10044, 684],
    },
    HelpLabelSegment::Quad {
        control: [9997, 635],
        to: [9918, 635],
    },
];

const PLAYER_BLUE_CONTOUR_15: [HelpLabelSegment; 14] = [
    HelpLabelSegment::Quad {
        control: [9850, 1098],
        to: [9933, 1098],
    },
    HelpLabelSegment::Quad {
        control: [10029, 1098],
        to: [10092, 1043],
    },
    HelpLabelSegment::Line([10139, 1123]),
    HelpLabelSegment::Quad {
        control: [10113, 1148],
        to: [10060, 1167],
    },
    HelpLabelSegment::Quad {
        control: [9994, 1192],
        to: [9912, 1192],
    },
    HelpLabelSegment::Quad {
        control: [9793, 1192],
        to: [9710, 1112],
    },
    HelpLabelSegment::Quad {
        control: [9619, 1023],
        to: [9619, 874],
    },
    HelpLabelSegment::Quad {
        control: [9619, 718],
        to: [9712, 625],
    },
    HelpLabelSegment::Quad {
        control: [9797, 541],
        to: [9913, 541],
    },
    HelpLabelSegment::Quad {
        control: [10046, 541],
        to: [10123, 616],
    },
    HelpLabelSegment::Quad {
        control: [10196, 689],
        to: [10196, 810],
    },
    HelpLabelSegment::Quad {
        control: [10196, 846],
        to: [10188, 878],
    },
    HelpLabelSegment::Line([9736, 878]),
    HelpLabelSegment::Quad {
        control: [9736, 988],
        to: [9796, 1046],
    },
];

const PLAYER_BLUE_CONTOURS: [HelpLabelContour; 16] = [
    HelpLabelContour {
        start: [4413, 316],
        segments: &PLAYER_BLUE_CONTOUR_0,
    },
    HelpLabelContour {
        start: [4370, 428],
        segments: &PLAYER_BLUE_CONTOUR_1,
    },
    HelpLabelContour {
        start: [5140, 1092],
        segments: &PLAYER_BLUE_CONTOUR_2,
    },
    HelpLabelContour {
        start: [5464, 541],
        segments: &PLAYER_BLUE_CONTOUR_3,
    },
    HelpLabelContour {
        start: [5539, 860],
        segments: &PLAYER_BLUE_CONTOUR_4,
    },
    HelpLabelContour {
        start: [6405, 553],
        segments: &PLAYER_BLUE_CONTOUR_5,
    },
    HelpLabelContour {
        start: [6747, 635],
        segments: &PLAYER_BLUE_CONTOUR_6,
    },
    HelpLabelContour {
        start: [6626, 1046],
        segments: &PLAYER_BLUE_CONTOUR_7,
    },
    HelpLabelContour {
        start: [7486, 660],
        segments: &PLAYER_BLUE_CONTOUR_8,
    },
    HelpLabelContour {
        start: [8470, 526],
        segments: &PLAYER_BLUE_CONTOUR_9,
    },
    HelpLabelContour {
        start: [8095, 416],
        segments: &PLAYER_BLUE_CONTOUR_10,
    },
    HelpLabelContour {
        start: [8095, 753],
        segments: &PLAYER_BLUE_CONTOUR_11,
    },
    HelpLabelContour {
        start: [8875, 1092],
        segments: &PLAYER_BLUE_CONTOUR_12,
    },
    HelpLabelContour {
        start: [9110, 553],
        segments: &PLAYER_BLUE_CONTOUR_13,
    },
    HelpLabelContour {
        start: [9918, 635],
        segments: &PLAYER_BLUE_CONTOUR_14,
    },
    HelpLabelContour {
        start: [9796, 1046],
        segments: &PLAYER_BLUE_CONTOUR_15,
    },
];

pub const PLAYER_BLUE: HelpLabelDefinition = HelpLabelDefinition {
    text: "Player Blue",
    define_text_id: 118,
    font_ids: &PLAYER_BLUE_FONT_IDS,
    color_rgb: [102, 197, 255],
    bounds_centipx: [4255, 10760, 295, 1425],
    contours: &PLAYER_BLUE_CONTOURS,
};

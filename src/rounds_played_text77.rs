// Generated from gravity_arcade.swf DefineText 77 and DefineFont 26.
// Coordinates are stored as SWF text-local centipixels; divide by 100 for pixels.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoundsPlayedSegment {
    Line([i16; 2]),
    Quad { control: [i16; 2], to: [i16; 2] },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoundsPlayedContour {
    pub(super) start: [i16; 2],
    pub(super) segments: &'static [RoundsPlayedSegment],
}

pub const TEXT: &str = "rounds played:";
pub const FONT_ID: u16 = 26;
pub const DEFINE_TEXT_ID: u16 = 77;
pub const BOUNDS_CENTIPX: [i16; 4] = [3325, 11705, 295, 1425];

const CONTOUR_0: [RoundsPlayedSegment; 11] = [
    RoundsPlayedSegment::Quad {
        control: [3619, 635],
        to: [3582, 635],
    },
    RoundsPlayedSegment::Quad {
        control: [3523, 635],
        to: [3479, 689],
    },
    RoundsPlayedSegment::Quad {
        control: [3434, 744],
        to: [3434, 820],
    },
    RoundsPlayedSegment::Line([3434, 1180]),
    RoundsPlayedSegment::Line([3323, 1180]),
    RoundsPlayedSegment::Line([3323, 553]),
    RoundsPlayedSegment::Line([3434, 553]),
    RoundsPlayedSegment::Line([3434, 653]),
    RoundsPlayedSegment::Quad {
        control: [3495, 541],
        to: [3616, 541],
    },
    RoundsPlayedSegment::Line([3701, 552]),
    RoundsPlayedSegment::Line([3656, 660]),
];

const CONTOUR_1: [RoundsPlayedSegment; 6] = [
    RoundsPlayedSegment::Quad {
        control: [3856, 1101],
        to: [4022, 1101],
    },
    RoundsPlayedSegment::Quad {
        control: [4101, 1101],
        to: [4145, 1038],
    },
    RoundsPlayedSegment::Quad {
        control: [4189, 975],
        to: [4189, 865],
    },
    RoundsPlayedSegment::Quad {
        control: [4189, 632],
        to: [4022, 632],
    },
    RoundsPlayedSegment::Quad {
        control: [3946, 632],
        to: [3902, 694],
    },
    RoundsPlayedSegment::Quad {
        control: [3856, 756],
        to: [3856, 865],
    },
];

const CONTOUR_2: [RoundsPlayedSegment; 8] = [
    RoundsPlayedSegment::Quad {
        control: [3895, 541],
        to: [4022, 541],
    },
    RoundsPlayedSegment::Quad {
        control: [4157, 541],
        to: [4232, 627],
    },
    RoundsPlayedSegment::Quad {
        control: [4306, 712],
        to: [4306, 865],
    },
    RoundsPlayedSegment::Quad {
        control: [4306, 1017],
        to: [4230, 1105],
    },
    RoundsPlayedSegment::Quad {
        control: [4154, 1192],
        to: [4022, 1192],
    },
    RoundsPlayedSegment::Quad {
        control: [3889, 1192],
        to: [3814, 1104],
    },
    RoundsPlayedSegment::Quad {
        control: [3739, 1015],
        to: [3739, 865],
    },
    RoundsPlayedSegment::Quad {
        control: [3739, 719],
        to: [3817, 630],
    },
];

const CONTOUR_3: [RoundsPlayedSegment; 15] = [
    RoundsPlayedSegment::Line([4530, 953]),
    RoundsPlayedSegment::Quad {
        control: [4530, 1098],
        to: [4656, 1098],
    },
    RoundsPlayedSegment::Quad {
        control: [4711, 1098],
        to: [4756, 1066],
    },
    RoundsPlayedSegment::Quad {
        control: [4802, 1035],
        to: [4817, 994],
    },
    RoundsPlayedSegment::Line([4817, 553]),
    RoundsPlayedSegment::Line([4929, 553]),
    RoundsPlayedSegment::Line([4929, 1180]),
    RoundsPlayedSegment::Line([4817, 1180]),
    RoundsPlayedSegment::Line([4817, 1093]),
    RoundsPlayedSegment::Quad {
        control: [4799, 1131],
        to: [4742, 1161],
    },
    RoundsPlayedSegment::Quad {
        control: [4685, 1192],
        to: [4631, 1192],
    },
    RoundsPlayedSegment::Quad {
        control: [4528, 1192],
        to: [4474, 1133],
    },
    RoundsPlayedSegment::Quad {
        control: [4419, 1073],
        to: [4419, 964],
    },
    RoundsPlayedSegment::Line([4419, 553]),
    RoundsPlayedSegment::Line([4530, 553]),
];

const CONTOUR_4: [RoundsPlayedSegment; 14] = [
    RoundsPlayedSegment::Line([5577, 1180]),
    RoundsPlayedSegment::Line([5465, 1180]),
    RoundsPlayedSegment::Line([5465, 816]),
    RoundsPlayedSegment::Quad {
        control: [5465, 715],
        to: [5436, 675],
    },
    RoundsPlayedSegment::Quad {
        control: [5405, 635],
        to: [5334, 635],
    },
    RoundsPlayedSegment::Quad {
        control: [5296, 635],
        to: [5254, 657],
    },
    RoundsPlayedSegment::Quad {
        control: [5213, 681],
        to: [5191, 714],
    },
    RoundsPlayedSegment::Line([5191, 1180]),
    RoundsPlayedSegment::Line([5080, 1180]),
    RoundsPlayedSegment::Line([5080, 553]),
    RoundsPlayedSegment::Line([5156, 553]),
    RoundsPlayedSegment::Line([5191, 634]),
    RoundsPlayedSegment::Quad {
        control: [5246, 541],
        to: [5370, 541],
    },
    RoundsPlayedSegment::Quad {
        control: [5577, 541],
        to: [5577, 792],
    },
];

const CONTOUR_5: [RoundsPlayedSegment; 11] = [
    RoundsPlayedSegment::Line([6243, 295]),
    RoundsPlayedSegment::Line([6243, 1180]),
    RoundsPlayedSegment::Line([6132, 1180]),
    RoundsPlayedSegment::Line([6132, 1133]),
    RoundsPlayedSegment::Quad {
        control: [6075, 1192],
        to: [5963, 1192],
    },
    RoundsPlayedSegment::Quad {
        control: [5846, 1192],
        to: [5772, 1107],
    },
    RoundsPlayedSegment::Quad {
        control: [5700, 1023],
        to: [5700, 882],
    },
    RoundsPlayedSegment::Quad {
        control: [5700, 741],
        to: [5784, 641],
    },
    RoundsPlayedSegment::Quad {
        control: [5868, 541],
        to: [5984, 541],
    },
    RoundsPlayedSegment::Quad {
        control: [6082, 541],
        to: [6132, 587],
    },
    RoundsPlayedSegment::Line([6132, 295]),
];

const CONTOUR_6: [RoundsPlayedSegment; 7] = [
    RoundsPlayedSegment::Quad {
        control: [6123, 1065],
        to: [6132, 1046],
    },
    RoundsPlayedSegment::Line([6132, 698]),
    RoundsPlayedSegment::Quad {
        control: [6090, 635],
        to: [6017, 635],
    },
    RoundsPlayedSegment::Quad {
        control: [5927, 635],
        to: [5872, 702],
    },
    RoundsPlayedSegment::Quad {
        control: [5817, 769],
        to: [5817, 872],
    },
    RoundsPlayedSegment::Quad {
        control: [5817, 1098],
        to: [6023, 1098],
    },
    RoundsPlayedSegment::Quad {
        control: [6049, 1098],
        to: [6086, 1082],
    },
];

const CONTOUR_7: [RoundsPlayedSegment; 22] = [
    RoundsPlayedSegment::Line([6712, 688]),
    RoundsPlayedSegment::Quad {
        control: [6646, 635],
        to: [6579, 635],
    },
    RoundsPlayedSegment::Quad {
        control: [6539, 635],
        to: [6512, 654],
    },
    RoundsPlayedSegment::Quad {
        control: [6484, 673],
        to: [6484, 701],
    },
    RoundsPlayedSegment::Quad {
        control: [6484, 762],
        to: [6554, 792],
    },
    RoundsPlayedSegment::Line([6633, 828]),
    RoundsPlayedSegment::Quad {
        control: [6706, 862],
        to: [6740, 905],
    },
    RoundsPlayedSegment::Quad {
        control: [6773, 948],
        to: [6773, 1012],
    },
    RoundsPlayedSegment::Quad {
        control: [6773, 1097],
        to: [6714, 1145],
    },
    RoundsPlayedSegment::Quad {
        control: [6654, 1192],
        to: [6550, 1192],
    },
    RoundsPlayedSegment::Quad {
        control: [6450, 1192],
        to: [6364, 1142],
    },
    RoundsPlayedSegment::Line([6402, 1037]),
    RoundsPlayedSegment::Quad {
        control: [6496, 1098],
        to: [6552, 1098],
    },
    RoundsPlayedSegment::Quad {
        control: [6655, 1098],
        to: [6655, 1011],
    },
    RoundsPlayedSegment::Quad {
        control: [6655, 949],
        to: [6556, 905],
    },
    RoundsPlayedSegment::Quad {
        control: [6480, 869],
        to: [6453, 852],
    },
    RoundsPlayedSegment::Quad {
        control: [6426, 833],
        to: [6407, 811],
    },
    RoundsPlayedSegment::Quad {
        control: [6387, 787],
        to: [6378, 762],
    },
    RoundsPlayedSegment::Quad {
        control: [6367, 735],
        to: [6367, 705],
    },
    RoundsPlayedSegment::Quad {
        control: [6367, 628],
        to: [6423, 585],
    },
    RoundsPlayedSegment::Quad {
        control: [6480, 541],
        to: [6571, 541],
    },
    RoundsPlayedSegment::Quad {
        control: [6639, 541],
        to: [6743, 585],
    },
];

const CONTOUR_8: [RoundsPlayedSegment; 12] = [
    RoundsPlayedSegment::Line([7361, 605]),
    RoundsPlayedSegment::Quad {
        control: [7424, 541],
        to: [7513, 541],
    },
    RoundsPlayedSegment::Quad {
        control: [7647, 541],
        to: [7722, 625],
    },
    RoundsPlayedSegment::Quad {
        control: [7796, 708],
        to: [7796, 868],
    },
    RoundsPlayedSegment::Quad {
        control: [7796, 1011],
        to: [7721, 1101],
    },
    RoundsPlayedSegment::Quad {
        control: [7646, 1192],
        to: [7504, 1192],
    },
    RoundsPlayedSegment::Quad {
        control: [7464, 1192],
        to: [7420, 1178],
    },
    RoundsPlayedSegment::Quad {
        control: [7374, 1164],
        to: [7361, 1146],
    },
    RoundsPlayedSegment::Line([7361, 1426]),
    RoundsPlayedSegment::Line([7250, 1426]),
    RoundsPlayedSegment::Line([7250, 553]),
    RoundsPlayedSegment::Line([7361, 553]),
];

const CONTOUR_9: [RoundsPlayedSegment; 8] = [
    RoundsPlayedSegment::Line([7361, 1053]),
    RoundsPlayedSegment::Quad {
        control: [7372, 1070],
        to: [7406, 1084],
    },
    RoundsPlayedSegment::Quad {
        control: [7440, 1098],
        to: [7471, 1098],
    },
    RoundsPlayedSegment::Quad {
        control: [7679, 1098],
        to: [7679, 864],
    },
    RoundsPlayedSegment::Quad {
        control: [7679, 745],
        to: [7629, 690],
    },
    RoundsPlayedSegment::Quad {
        control: [7580, 635],
        to: [7472, 635],
    },
    RoundsPlayedSegment::Quad {
        control: [7449, 635],
        to: [7415, 651],
    },
    RoundsPlayedSegment::Line([7361, 688]),
];

const CONTOUR_10: [RoundsPlayedSegment; 7] = [
    RoundsPlayedSegment::Line([8145, 1192]),
    RoundsPlayedSegment::Quad {
        control: [7928, 1192],
        to: [7928, 1003],
    },
    RoundsPlayedSegment::Line([7928, 295]),
    RoundsPlayedSegment::Line([8039, 295]),
    RoundsPlayedSegment::Line([8039, 984]),
    RoundsPlayedSegment::Quad {
        control: [8039, 1035],
        to: [8069, 1064],
    },
    RoundsPlayedSegment::Quad {
        control: [8098, 1092],
        to: [8145, 1092],
    },
];

const CONTOUR_11: [RoundsPlayedSegment; 18] = [
    RoundsPlayedSegment::Quad {
        control: [8606, 541],
        to: [8668, 603],
    },
    RoundsPlayedSegment::Quad {
        control: [8729, 666],
        to: [8729, 800],
    },
    RoundsPlayedSegment::Line([8729, 1025]),
    RoundsPlayedSegment::Quad {
        control: [8729, 1109],
        to: [8779, 1135],
    },
    RoundsPlayedSegment::Line([8779, 1192]),
    RoundsPlayedSegment::Quad {
        control: [8711, 1192],
        to: [8678, 1172],
    },
    RoundsPlayedSegment::Quad {
        control: [8644, 1153],
        to: [8629, 1109],
    },
    RoundsPlayedSegment::Quad {
        control: [8562, 1192],
        to: [8425, 1192],
    },
    RoundsPlayedSegment::Quad {
        control: [8351, 1192],
        to: [8297, 1139],
    },
    RoundsPlayedSegment::Quad {
        control: [8242, 1085],
        to: [8242, 1005],
    },
    RoundsPlayedSegment::Quad {
        control: [8242, 909],
        to: [8326, 844],
    },
    RoundsPlayedSegment::Quad {
        control: [8409, 778],
        to: [8538, 778],
    },
    RoundsPlayedSegment::Line([8618, 793]),
    RoundsPlayedSegment::Quad {
        control: [8618, 641],
        to: [8482, 641],
    },
    RoundsPlayedSegment::Quad {
        control: [8378, 641],
        to: [8322, 697],
    },
    RoundsPlayedSegment::Line([8275, 603]),
    RoundsPlayedSegment::Quad {
        control: [8306, 578],
        to: [8363, 560],
    },
    RoundsPlayedSegment::Quad {
        control: [8419, 541],
        to: [8469, 541],
    },
];

const CONTOUR_12: [RoundsPlayedSegment; 6] = [
    RoundsPlayedSegment::Quad {
        control: [8460, 860],
        to: [8407, 903],
    },
    RoundsPlayedSegment::Quad {
        control: [8353, 947],
        to: [8353, 1007],
    },
    RoundsPlayedSegment::Quad {
        control: [8353, 1104],
        to: [8469, 1104],
    },
    RoundsPlayedSegment::Quad {
        control: [8554, 1104],
        to: [8618, 1024],
    },
    RoundsPlayedSegment::Line([8618, 872]),
    RoundsPlayedSegment::Line([8544, 860]),
];

const CONTOUR_13: [RoundsPlayedSegment; 13] = [
    RoundsPlayedSegment::Line([9144, 1287]),
    RoundsPlayedSegment::Quad {
        control: [9123, 1346],
        to: [9054, 1386],
    },
    RoundsPlayedSegment::Quad {
        control: [8983, 1426],
        to: [8898, 1426],
    },
    RoundsPlayedSegment::Line([8898, 1326]),
    RoundsPlayedSegment::Quad {
        control: [8968, 1326],
        to: [9017, 1295],
    },
    RoundsPlayedSegment::Quad {
        control: [9068, 1262],
        to: [9068, 1215],
    },
    RoundsPlayedSegment::Quad {
        control: [9068, 1164],
        to: [9049, 1113],
    },
    RoundsPlayedSegment::Line([9002, 989]),
    RoundsPlayedSegment::Line([8832, 553]),
    RoundsPlayedSegment::Line([8946, 553]),
    RoundsPlayedSegment::Line([9131, 1038]),
    RoundsPlayedSegment::Line([9296, 553]),
    RoundsPlayedSegment::Line([9410, 553]),
];

const CONTOUR_14: [RoundsPlayedSegment; 5] = [
    RoundsPlayedSegment::Quad {
        control: [9679, 635],
        to: [9628, 683],
    },
    RoundsPlayedSegment::Quad {
        control: [9580, 729],
        to: [9573, 797],
    },
    RoundsPlayedSegment::Line([9921, 797]),
    RoundsPlayedSegment::Quad {
        control: [9921, 729],
        to: [9879, 684],
    },
    RoundsPlayedSegment::Quad {
        control: [9832, 635],
        to: [9752, 635],
    },
];

const CONTOUR_15: [RoundsPlayedSegment; 14] = [
    RoundsPlayedSegment::Quad {
        control: [9685, 1098],
        to: [9768, 1098],
    },
    RoundsPlayedSegment::Quad {
        control: [9864, 1098],
        to: [9927, 1043],
    },
    RoundsPlayedSegment::Line([9974, 1123]),
    RoundsPlayedSegment::Quad {
        control: [9948, 1148],
        to: [9895, 1167],
    },
    RoundsPlayedSegment::Quad {
        control: [9829, 1192],
        to: [9747, 1192],
    },
    RoundsPlayedSegment::Quad {
        control: [9628, 1192],
        to: [9545, 1112],
    },
    RoundsPlayedSegment::Quad {
        control: [9454, 1023],
        to: [9454, 874],
    },
    RoundsPlayedSegment::Quad {
        control: [9454, 718],
        to: [9547, 625],
    },
    RoundsPlayedSegment::Quad {
        control: [9632, 541],
        to: [9748, 541],
    },
    RoundsPlayedSegment::Quad {
        control: [9881, 541],
        to: [9958, 616],
    },
    RoundsPlayedSegment::Quad {
        control: [10031, 689],
        to: [10031, 810],
    },
    RoundsPlayedSegment::Quad {
        control: [10031, 846],
        to: [10023, 878],
    },
    RoundsPlayedSegment::Line([9571, 878]),
    RoundsPlayedSegment::Quad {
        control: [9571, 988],
        to: [9631, 1046],
    },
];

const CONTOUR_16: [RoundsPlayedSegment; 11] = [
    RoundsPlayedSegment::Line([10658, 295]),
    RoundsPlayedSegment::Line([10658, 1180]),
    RoundsPlayedSegment::Line([10547, 1180]),
    RoundsPlayedSegment::Line([10547, 1133]),
    RoundsPlayedSegment::Quad {
        control: [10490, 1192],
        to: [10378, 1192],
    },
    RoundsPlayedSegment::Quad {
        control: [10261, 1192],
        to: [10187, 1107],
    },
    RoundsPlayedSegment::Quad {
        control: [10115, 1023],
        to: [10115, 882],
    },
    RoundsPlayedSegment::Quad {
        control: [10115, 741],
        to: [10199, 641],
    },
    RoundsPlayedSegment::Quad {
        control: [10283, 541],
        to: [10399, 541],
    },
    RoundsPlayedSegment::Quad {
        control: [10497, 541],
        to: [10547, 587],
    },
    RoundsPlayedSegment::Line([10547, 295]),
];

const CONTOUR_17: [RoundsPlayedSegment; 7] = [
    RoundsPlayedSegment::Quad {
        control: [10538, 1065],
        to: [10547, 1046],
    },
    RoundsPlayedSegment::Line([10547, 698]),
    RoundsPlayedSegment::Quad {
        control: [10505, 635],
        to: [10432, 635],
    },
    RoundsPlayedSegment::Quad {
        control: [10342, 635],
        to: [10287, 702],
    },
    RoundsPlayedSegment::Quad {
        control: [10232, 769],
        to: [10232, 872],
    },
    RoundsPlayedSegment::Quad {
        control: [10232, 1098],
        to: [10438, 1098],
    },
    RoundsPlayedSegment::Quad {
        control: [10464, 1098],
        to: [10501, 1082],
    },
];

const CONTOUR_18: [RoundsPlayedSegment; 8] = [
    RoundsPlayedSegment::Quad {
        control: [11035, 593],
        to: [11035, 629],
    },
    RoundsPlayedSegment::Quad {
        control: [11035, 666],
        to: [11010, 691],
    },
    RoundsPlayedSegment::Quad {
        control: [10984, 717],
        to: [10947, 717],
    },
    RoundsPlayedSegment::Quad {
        control: [10911, 717],
        to: [10885, 691],
    },
    RoundsPlayedSegment::Quad {
        control: [10860, 666],
        to: [10860, 629],
    },
    RoundsPlayedSegment::Quad {
        control: [10860, 593],
        to: [10885, 567],
    },
    RoundsPlayedSegment::Quad {
        control: [10911, 541],
        to: [10947, 541],
    },
    RoundsPlayedSegment::Quad {
        control: [10984, 541],
        to: [11010, 567],
    },
];

const CONTOUR_19: [RoundsPlayedSegment; 8] = [
    RoundsPlayedSegment::Quad {
        control: [10984, 1016],
        to: [11010, 1042],
    },
    RoundsPlayedSegment::Quad {
        control: [11035, 1068],
        to: [11035, 1104],
    },
    RoundsPlayedSegment::Quad {
        control: [11035, 1140],
        to: [11010, 1166],
    },
    RoundsPlayedSegment::Quad {
        control: [10984, 1192],
        to: [10947, 1192],
    },
    RoundsPlayedSegment::Quad {
        control: [10911, 1192],
        to: [10885, 1166],
    },
    RoundsPlayedSegment::Quad {
        control: [10860, 1140],
        to: [10860, 1104],
    },
    RoundsPlayedSegment::Quad {
        control: [10860, 1068],
        to: [10885, 1042],
    },
    RoundsPlayedSegment::Quad {
        control: [10911, 1016],
        to: [10947, 1016],
    },
];

pub const CONTOURS: [RoundsPlayedContour; 20] = [
    RoundsPlayedContour {
        start: [3656, 660],
        segments: &CONTOUR_0,
    },
    RoundsPlayedContour {
        start: [3856, 865],
        segments: &CONTOUR_1,
    },
    RoundsPlayedContour {
        start: [3817, 630],
        segments: &CONTOUR_2,
    },
    RoundsPlayedContour {
        start: [4530, 553],
        segments: &CONTOUR_3,
    },
    RoundsPlayedContour {
        start: [5577, 792],
        segments: &CONTOUR_4,
    },
    RoundsPlayedContour {
        start: [6132, 295],
        segments: &CONTOUR_5,
    },
    RoundsPlayedContour {
        start: [6086, 1082],
        segments: &CONTOUR_6,
    },
    RoundsPlayedContour {
        start: [6743, 585],
        segments: &CONTOUR_7,
    },
    RoundsPlayedContour {
        start: [7361, 553],
        segments: &CONTOUR_8,
    },
    RoundsPlayedContour {
        start: [7361, 688],
        segments: &CONTOUR_9,
    },
    RoundsPlayedContour {
        start: [8145, 1092],
        segments: &CONTOUR_10,
    },
    RoundsPlayedContour {
        start: [8469, 541],
        segments: &CONTOUR_11,
    },
    RoundsPlayedContour {
        start: [8544, 860],
        segments: &CONTOUR_12,
    },
    RoundsPlayedContour {
        start: [9410, 553],
        segments: &CONTOUR_13,
    },
    RoundsPlayedContour {
        start: [9752, 635],
        segments: &CONTOUR_14,
    },
    RoundsPlayedContour {
        start: [9631, 1046],
        segments: &CONTOUR_15,
    },
    RoundsPlayedContour {
        start: [10547, 295],
        segments: &CONTOUR_16,
    },
    RoundsPlayedContour {
        start: [10501, 1082],
        segments: &CONTOUR_17,
    },
    RoundsPlayedContour {
        start: [11010, 567],
        segments: &CONTOUR_18,
    },
    RoundsPlayedContour {
        start: [10947, 1016],
        segments: &CONTOUR_19,
    },
];

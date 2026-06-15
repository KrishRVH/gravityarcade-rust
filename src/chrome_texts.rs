// Generated from gravity_arcade.swf header/version DefineText tags.
// Coordinates are stored as SWF text-local centipixels; divide by 100 for pixels.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChromeTextSegment {
    Line([i16; 2]),
    Quad { control: [i16; 2], to: [i16; 2] },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChromeTextContour {
    pub(super) start: [i16; 2],
    pub(super) segments: &'static [ChromeTextSegment],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChromeTextDefinition {
    pub(super) text: &'static str,
    pub(super) define_text_id: u16,
    pub(super) font_ids: &'static [u16],
    pub(super) color_rgb: [u8; 3],
    pub(super) bounds_centipx: [i16; 4],
    pub(super) contours: &'static [ChromeTextContour],
}

const VERSION_FONT_IDS: [u16; 1] = [26];

const VERSION_CONTOUR_0: [ChromeTextSegment; 7] = [
    ChromeTextSegment::Line([-9804, 1192]),
    ChromeTextSegment::Line([-9833, 1192]),
    ChromeTextSegment::Line([-10103, 551]),
    ChromeTextSegment::Line([-9981, 551]),
    ChromeTextSegment::Line([-9815, 990]),
    ChromeTextSegment::Line([-9646, 551]),
    ChromeTextSegment::Line([-9529, 551]),
];

const VERSION_CONTOUR_1: [ChromeTextSegment; 5] = [
    ChromeTextSegment::Quad {
        control: [-9256, 635],
        to: [-9307, 683],
    },
    ChromeTextSegment::Quad {
        control: [-9355, 729],
        to: [-9362, 797],
    },
    ChromeTextSegment::Line([-9014, 797]),
    ChromeTextSegment::Quad {
        control: [-9014, 729],
        to: [-9056, 684],
    },
    ChromeTextSegment::Quad {
        control: [-9103, 635],
        to: [-9182, 635],
    },
];

const VERSION_CONTOUR_2: [ChromeTextSegment; 14] = [
    ChromeTextSegment::Quad {
        control: [-9250, 1098],
        to: [-9167, 1098],
    },
    ChromeTextSegment::Quad {
        control: [-9071, 1098],
        to: [-9008, 1043],
    },
    ChromeTextSegment::Line([-8961, 1123]),
    ChromeTextSegment::Quad {
        control: [-8987, 1148],
        to: [-9040, 1167],
    },
    ChromeTextSegment::Quad {
        control: [-9106, 1192],
        to: [-9188, 1192],
    },
    ChromeTextSegment::Quad {
        control: [-9307, 1192],
        to: [-9390, 1112],
    },
    ChromeTextSegment::Quad {
        control: [-9481, 1023],
        to: [-9481, 874],
    },
    ChromeTextSegment::Quad {
        control: [-9481, 718],
        to: [-9388, 625],
    },
    ChromeTextSegment::Quad {
        control: [-9303, 541],
        to: [-9187, 541],
    },
    ChromeTextSegment::Quad {
        control: [-9054, 541],
        to: [-8977, 616],
    },
    ChromeTextSegment::Quad {
        control: [-8904, 689],
        to: [-8904, 810],
    },
    ChromeTextSegment::Quad {
        control: [-8904, 846],
        to: [-8912, 878],
    },
    ChromeTextSegment::Line([-9364, 878]),
    ChromeTextSegment::Quad {
        control: [-9364, 988],
        to: [-9304, 1046],
    },
];

const VERSION_CONTOUR_3: [ChromeTextSegment; 11] = [
    ChromeTextSegment::Quad {
        control: [-8481, 635],
        to: [-8518, 635],
    },
    ChromeTextSegment::Quad {
        control: [-8577, 635],
        to: [-8621, 689],
    },
    ChromeTextSegment::Quad {
        control: [-8666, 744],
        to: [-8666, 820],
    },
    ChromeTextSegment::Line([-8666, 1180]),
    ChromeTextSegment::Line([-8777, 1180]),
    ChromeTextSegment::Line([-8777, 553]),
    ChromeTextSegment::Line([-8666, 553]),
    ChromeTextSegment::Line([-8666, 653]),
    ChromeTextSegment::Quad {
        control: [-8605, 541],
        to: [-8484, 541],
    },
    ChromeTextSegment::Line([-8399, 552]),
    ChromeTextSegment::Line([-8444, 660]),
];

const VERSION_CONTOUR_4: [ChromeTextSegment; 22] = [
    ChromeTextSegment::Line([-8013, 688]),
    ChromeTextSegment::Quad {
        control: [-8079, 635],
        to: [-8146, 635],
    },
    ChromeTextSegment::Quad {
        control: [-8186, 635],
        to: [-8212, 654],
    },
    ChromeTextSegment::Quad {
        control: [-8241, 673],
        to: [-8241, 701],
    },
    ChromeTextSegment::Quad {
        control: [-8241, 762],
        to: [-8171, 792],
    },
    ChromeTextSegment::Line([-8092, 828]),
    ChromeTextSegment::Quad {
        control: [-8019, 862],
        to: [-7985, 905],
    },
    ChromeTextSegment::Quad {
        control: [-7952, 948],
        to: [-7952, 1012],
    },
    ChromeTextSegment::Quad {
        control: [-7952, 1097],
        to: [-8011, 1145],
    },
    ChromeTextSegment::Quad {
        control: [-8071, 1192],
        to: [-8175, 1192],
    },
    ChromeTextSegment::Quad {
        control: [-8275, 1192],
        to: [-8361, 1142],
    },
    ChromeTextSegment::Line([-8323, 1037]),
    ChromeTextSegment::Quad {
        control: [-8229, 1098],
        to: [-8173, 1098],
    },
    ChromeTextSegment::Quad {
        control: [-8070, 1098],
        to: [-8070, 1011],
    },
    ChromeTextSegment::Quad {
        control: [-8070, 949],
        to: [-8169, 905],
    },
    ChromeTextSegment::Quad {
        control: [-8245, 869],
        to: [-8272, 852],
    },
    ChromeTextSegment::Quad {
        control: [-8299, 833],
        to: [-8318, 811],
    },
    ChromeTextSegment::Quad {
        control: [-8338, 787],
        to: [-8347, 762],
    },
    ChromeTextSegment::Quad {
        control: [-8358, 735],
        to: [-8358, 705],
    },
    ChromeTextSegment::Quad {
        control: [-8358, 628],
        to: [-8302, 585],
    },
    ChromeTextSegment::Quad {
        control: [-8245, 541],
        to: [-8154, 541],
    },
    ChromeTextSegment::Quad {
        control: [-8086, 541],
        to: [-7982, 585],
    },
];

const VERSION_CONTOUR_5: [ChromeTextSegment; 8] = [
    ChromeTextSegment::Quad {
        control: [-7692, 315],
        to: [-7671, 336],
    },
    ChromeTextSegment::Quad {
        control: [-7651, 356],
        to: [-7651, 384],
    },
    ChromeTextSegment::Quad {
        control: [-7651, 412],
        to: [-7671, 434],
    },
    ChromeTextSegment::Quad {
        control: [-7692, 453],
        to: [-7720, 453],
    },
    ChromeTextSegment::Quad {
        control: [-7749, 453],
        to: [-7769, 434],
    },
    ChromeTextSegment::Quad {
        control: [-7790, 412],
        to: [-7790, 384],
    },
    ChromeTextSegment::Quad {
        control: [-7790, 355],
        to: [-7770, 335],
    },
    ChromeTextSegment::Quad {
        control: [-7750, 315],
        to: [-7720, 315],
    },
];

const VERSION_CONTOUR_6: [ChromeTextSegment; 6] = [
    ChromeTextSegment::Line([-7670, 1180]),
    ChromeTextSegment::Line([-7781, 1180]),
    ChromeTextSegment::Line([-7781, 647]),
    ChromeTextSegment::Line([-7868, 647]),
    ChromeTextSegment::Line([-7868, 553]),
    ChromeTextSegment::Line([-7670, 553]),
];

const VERSION_CONTOUR_7: [ChromeTextSegment; 6] = [
    ChromeTextSegment::Quad {
        control: [-7419, 1101],
        to: [-7253, 1101],
    },
    ChromeTextSegment::Quad {
        control: [-7174, 1101],
        to: [-7130, 1038],
    },
    ChromeTextSegment::Quad {
        control: [-7086, 975],
        to: [-7086, 865],
    },
    ChromeTextSegment::Quad {
        control: [-7086, 632],
        to: [-7253, 632],
    },
    ChromeTextSegment::Quad {
        control: [-7329, 632],
        to: [-7373, 694],
    },
    ChromeTextSegment::Quad {
        control: [-7419, 756],
        to: [-7419, 865],
    },
];

const VERSION_CONTOUR_8: [ChromeTextSegment; 8] = [
    ChromeTextSegment::Quad {
        control: [-7380, 541],
        to: [-7253, 541],
    },
    ChromeTextSegment::Quad {
        control: [-7118, 541],
        to: [-7043, 627],
    },
    ChromeTextSegment::Quad {
        control: [-6969, 712],
        to: [-6969, 865],
    },
    ChromeTextSegment::Quad {
        control: [-6969, 1017],
        to: [-7045, 1105],
    },
    ChromeTextSegment::Quad {
        control: [-7121, 1192],
        to: [-7253, 1192],
    },
    ChromeTextSegment::Quad {
        control: [-7386, 1192],
        to: [-7461, 1104],
    },
    ChromeTextSegment::Quad {
        control: [-7536, 1015],
        to: [-7536, 865],
    },
    ChromeTextSegment::Quad {
        control: [-7536, 719],
        to: [-7458, 630],
    },
];

const VERSION_CONTOUR_9: [ChromeTextSegment; 14] = [
    ChromeTextSegment::Line([-6353, 1180]),
    ChromeTextSegment::Line([-6465, 1180]),
    ChromeTextSegment::Line([-6465, 816]),
    ChromeTextSegment::Quad {
        control: [-6465, 715],
        to: [-6494, 675],
    },
    ChromeTextSegment::Quad {
        control: [-6525, 635],
        to: [-6596, 635],
    },
    ChromeTextSegment::Quad {
        control: [-6634, 635],
        to: [-6676, 657],
    },
    ChromeTextSegment::Quad {
        control: [-6717, 681],
        to: [-6739, 714],
    },
    ChromeTextSegment::Line([-6739, 1180]),
    ChromeTextSegment::Line([-6850, 1180]),
    ChromeTextSegment::Line([-6850, 553]),
    ChromeTextSegment::Line([-6774, 553]),
    ChromeTextSegment::Line([-6739, 634]),
    ChromeTextSegment::Quad {
        control: [-6684, 541],
        to: [-6560, 541],
    },
    ChromeTextSegment::Quad {
        control: [-6353, 541],
        to: [-6353, 792],
    },
];

const VERSION_CONTOUR_10: [ChromeTextSegment; 8] = [
    ChromeTextSegment::Line([-5506, 1180]),
    ChromeTextSegment::Line([-5623, 1180]),
    ChromeTextSegment::Line([-5623, 523]),
    ChromeTextSegment::Line([-5798, 632]),
    ChromeTextSegment::Line([-5798, 521]),
    ChromeTextSegment::Quad {
        control: [-5731, 489],
        to: [-5656, 430],
    },
    ChromeTextSegment::Quad {
        control: [-5582, 371],
        to: [-5541, 319],
    },
    ChromeTextSegment::Line([-5506, 319]),
];

const VERSION_CONTOUR_11: [ChromeTextSegment; 8] = [
    ChromeTextSegment::Quad {
        control: [-5041, 1016],
        to: [-5015, 1042],
    },
    ChromeTextSegment::Quad {
        control: [-4990, 1068],
        to: [-4990, 1104],
    },
    ChromeTextSegment::Quad {
        control: [-4990, 1140],
        to: [-5015, 1166],
    },
    ChromeTextSegment::Quad {
        control: [-5041, 1192],
        to: [-5078, 1192],
    },
    ChromeTextSegment::Quad {
        control: [-5114, 1192],
        to: [-5140, 1166],
    },
    ChromeTextSegment::Quad {
        control: [-5165, 1140],
        to: [-5165, 1104],
    },
    ChromeTextSegment::Quad {
        control: [-5165, 1068],
        to: [-5140, 1042],
    },
    ChromeTextSegment::Quad {
        control: [-5114, 1016],
        to: [-5078, 1016],
    },
];

const VERSION_CONTOUR_12: [ChromeTextSegment; 23] = [
    ChromeTextSegment::Quad {
        control: [-4322, 426],
        to: [-4322, 520],
    },
    ChromeTextSegment::Quad {
        control: [-4322, 592],
        to: [-4361, 647],
    },
    ChromeTextSegment::Quad {
        control: [-4401, 702],
        to: [-4455, 721],
    },
    ChromeTextSegment::Quad {
        control: [-4381, 745],
        to: [-4336, 803],
    },
    ChromeTextSegment::Quad {
        control: [-4293, 860],
        to: [-4293, 942],
    },
    ChromeTextSegment::Quad {
        control: [-4293, 1062],
        to: [-4369, 1127],
    },
    ChromeTextSegment::Quad {
        control: [-4444, 1192],
        to: [-4581, 1192],
    },
    ChromeTextSegment::Quad {
        control: [-4639, 1192],
        to: [-4694, 1171],
    },
    ChromeTextSegment::Quad {
        control: [-4749, 1150],
        to: [-4781, 1119],
    },
    ChromeTextSegment::Line([-4724, 1029]),
    ChromeTextSegment::Quad {
        control: [-4667, 1092],
        to: [-4579, 1092],
    },
    ChromeTextSegment::Quad {
        control: [-4416, 1092],
        to: [-4416, 933],
    },
    ChromeTextSegment::Quad {
        control: [-4416, 860],
        to: [-4464, 816],
    },
    ChromeTextSegment::Quad {
        control: [-4511, 770],
        to: [-4590, 770],
    },
    ChromeTextSegment::Line([-4599, 770]),
    ChromeTextSegment::Line([-4599, 675]),
    ChromeTextSegment::Line([-4594, 675]),
    ChromeTextSegment::Quad {
        control: [-4445, 675],
        to: [-4445, 544],
    },
    ChromeTextSegment::Quad {
        control: [-4445, 407],
        to: [-4585, 407],
    },
    ChromeTextSegment::Quad {
        control: [-4661, 407],
        to: [-4707, 457],
    },
    ChromeTextSegment::Line([-4758, 377]),
    ChromeTextSegment::Quad {
        control: [-4704, 307],
        to: [-4577, 307],
    },
    ChromeTextSegment::Quad {
        control: [-4464, 307],
        to: [-4393, 367],
    },
];

const VERSION_CONTOUR_13: [ChromeTextSegment; 23] = [
    ChromeTextSegment::Quad {
        control: [-3692, 426],
        to: [-3692, 520],
    },
    ChromeTextSegment::Quad {
        control: [-3692, 592],
        to: [-3731, 647],
    },
    ChromeTextSegment::Quad {
        control: [-3771, 702],
        to: [-3825, 721],
    },
    ChromeTextSegment::Quad {
        control: [-3751, 745],
        to: [-3706, 803],
    },
    ChromeTextSegment::Quad {
        control: [-3663, 860],
        to: [-3663, 942],
    },
    ChromeTextSegment::Quad {
        control: [-3663, 1062],
        to: [-3739, 1127],
    },
    ChromeTextSegment::Quad {
        control: [-3814, 1192],
        to: [-3951, 1192],
    },
    ChromeTextSegment::Quad {
        control: [-4009, 1192],
        to: [-4064, 1171],
    },
    ChromeTextSegment::Quad {
        control: [-4119, 1150],
        to: [-4151, 1119],
    },
    ChromeTextSegment::Line([-4094, 1029]),
    ChromeTextSegment::Quad {
        control: [-4037, 1092],
        to: [-3949, 1092],
    },
    ChromeTextSegment::Quad {
        control: [-3786, 1092],
        to: [-3786, 933],
    },
    ChromeTextSegment::Quad {
        control: [-3786, 860],
        to: [-3834, 816],
    },
    ChromeTextSegment::Quad {
        control: [-3881, 770],
        to: [-3960, 770],
    },
    ChromeTextSegment::Line([-3969, 770]),
    ChromeTextSegment::Line([-3969, 675]),
    ChromeTextSegment::Line([-3964, 675]),
    ChromeTextSegment::Quad {
        control: [-3815, 675],
        to: [-3815, 544],
    },
    ChromeTextSegment::Quad {
        control: [-3815, 407],
        to: [-3955, 407],
    },
    ChromeTextSegment::Quad {
        control: [-4031, 407],
        to: [-4077, 457],
    },
    ChromeTextSegment::Line([-4128, 377]),
    ChromeTextSegment::Quad {
        control: [-4074, 307],
        to: [-3947, 307],
    },
    ChromeTextSegment::Quad {
        control: [-3834, 307],
        to: [-3763, 367],
    },
];

const VERSION_CONTOUR_14: [ChromeTextSegment; 4] = [
    ChromeTextSegment::Line([-3131, 782]),
    ChromeTextSegment::Line([-2880, 782]),
    ChromeTextSegment::Line([-2880, 884]),
    ChromeTextSegment::Line([-3131, 884]),
];

const VERSION_CONTOUR_15: [ChromeTextSegment; 8] = [
    ChromeTextSegment::Line([-2063, 364]),
    ChromeTextSegment::Quad {
        control: [-2179, 532],
        to: [-2179, 854],
    },
    ChromeTextSegment::Quad {
        control: [-2179, 1236],
        to: [-2063, 1370],
    },
    ChromeTextSegment::Line([-2063, 1431]),
    ChromeTextSegment::Quad {
        control: [-2181, 1349],
        to: [-2246, 1194],
    },
    ChromeTextSegment::Quad {
        control: [-2311, 1039],
        to: [-2311, 853],
    },
    ChromeTextSegment::Quad {
        control: [-2311, 694],
        to: [-2238, 531],
    },
    ChromeTextSegment::Quad {
        control: [-2163, 368],
        to: [-2063, 316],
    },
];

const VERSION_CONTOUR_16: [ChromeTextSegment; 16] = [
    ChromeTextSegment::Quad {
        control: [-1459, 582],
        to: [-1432, 603],
    },
    ChromeTextSegment::Line([-1487, 682]),
    ChromeTextSegment::Quad {
        control: [-1505, 666],
        to: [-1547, 650],
    },
    ChromeTextSegment::Line([-1632, 635]),
    ChromeTextSegment::Quad {
        control: [-1723, 635],
        to: [-1776, 698],
    },
    ChromeTextSegment::Quad {
        control: [-1829, 762],
        to: [-1829, 873],
    },
    ChromeTextSegment::Quad {
        control: [-1829, 983],
        to: [-1775, 1041],
    },
    ChromeTextSegment::Quad {
        control: [-1720, 1098],
        to: [-1624, 1098],
    },
    ChromeTextSegment::Quad {
        control: [-1549, 1098],
        to: [-1473, 1041],
    },
    ChromeTextSegment::Line([-1428, 1134]),
    ChromeTextSegment::Quad {
        control: [-1519, 1192],
        to: [-1651, 1192],
    },
    ChromeTextSegment::Quad {
        control: [-1779, 1192],
        to: [-1863, 1106],
    },
    ChromeTextSegment::Quad {
        control: [-1946, 1019],
        to: [-1946, 873],
    },
    ChromeTextSegment::Quad {
        control: [-1946, 723],
        to: [-1860, 632],
    },
    ChromeTextSegment::Quad {
        control: [-1773, 541],
        to: [-1622, 541],
    },
    ChromeTextSegment::Line([-1516, 561]),
];

const VERSION_CONTOUR_17: [ChromeTextSegment; 8] = [
    ChromeTextSegment::Quad {
        control: [-1028, 1039],
        to: [-1092, 1194],
    },
    ChromeTextSegment::Quad {
        control: [-1158, 1349],
        to: [-1276, 1431],
    },
    ChromeTextSegment::Line([-1276, 1370]),
    ChromeTextSegment::Quad {
        control: [-1160, 1236],
        to: [-1160, 854],
    },
    ChromeTextSegment::Quad {
        control: [-1160, 532],
        to: [-1276, 364],
    },
    ChromeTextSegment::Line([-1276, 316]),
    ChromeTextSegment::Quad {
        control: [-1177, 368],
        to: [-1102, 531],
    },
    ChromeTextSegment::Quad {
        control: [-1028, 694],
        to: [-1028, 853],
    },
];

const VERSION_CONTOUR_18: [ChromeTextSegment; 13] = [
    ChromeTextSegment::Line([-399, 599]),
    ChromeTextSegment::Quad {
        control: [-384, 578],
        to: [-339, 559],
    },
    ChromeTextSegment::Quad {
        control: [-296, 541],
        to: [-254, 541],
    },
    ChromeTextSegment::Quad {
        control: [-125, 541],
        to: [-45, 630],
    },
    ChromeTextSegment::Quad {
        control: [35, 719],
        to: [35, 855],
    },
    ChromeTextSegment::Quad {
        control: [35, 1012],
        to: [-45, 1103],
    },
    ChromeTextSegment::Quad {
        control: [-126, 1192],
        to: [-263, 1192],
    },
    ChromeTextSegment::Quad {
        control: [-308, 1192],
        to: [-350, 1175],
    },
    ChromeTextSegment::Quad {
        control: [-393, 1159],
        to: [-415, 1135],
    },
    ChromeTextSegment::Line([-455, 1192]),
    ChromeTextSegment::Line([-510, 1192]),
    ChromeTextSegment::Line([-510, 295]),
    ChromeTextSegment::Line([-399, 295]),
];

const VERSION_CONTOUR_19: [ChromeTextSegment; 9] = [
    ChromeTextSegment::Line([-399, 1046]),
    ChromeTextSegment::Quad {
        control: [-399, 1056],
        to: [-358, 1077],
    },
    ChromeTextSegment::Quad {
        control: [-316, 1098],
        to: [-295, 1098],
    },
    ChromeTextSegment::Quad {
        control: [-181, 1098],
        to: [-132, 1044],
    },
    ChromeTextSegment::Quad {
        control: [-83, 989],
        to: [-83, 861],
    },
    ChromeTextSegment::Quad {
        control: [-83, 755],
        to: [-140, 695],
    },
    ChromeTextSegment::Quad {
        control: [-197, 635],
        to: [-295, 635],
    },
    ChromeTextSegment::Quad {
        control: [-315, 635],
        to: [-351, 653],
    },
    ChromeTextSegment::Line([-399, 684]),
];

const VERSION_CONTOUR_20: [ChromeTextSegment; 13] = [
    ChromeTextSegment::Line([399, 1287]),
    ChromeTextSegment::Quad {
        control: [378, 1346],
        to: [309, 1386],
    },
    ChromeTextSegment::Quad {
        control: [238, 1426],
        to: [153, 1426],
    },
    ChromeTextSegment::Line([153, 1326]),
    ChromeTextSegment::Quad {
        control: [223, 1326],
        to: [272, 1295],
    },
    ChromeTextSegment::Quad {
        control: [323, 1262],
        to: [323, 1215],
    },
    ChromeTextSegment::Quad {
        control: [323, 1164],
        to: [304, 1113],
    },
    ChromeTextSegment::Line([257, 989]),
    ChromeTextSegment::Line([87, 553]),
    ChromeTextSegment::Line([201, 553]),
    ChromeTextSegment::Line([386, 1038]),
    ChromeTextSegment::Line([551, 553]),
    ChromeTextSegment::Line([665, 553]),
];

const VERSION_CONTOUR_21: [ChromeTextSegment; 8] = [
    ChromeTextSegment::Quad {
        control: [1325, 593],
        to: [1325, 629],
    },
    ChromeTextSegment::Quad {
        control: [1325, 666],
        to: [1300, 691],
    },
    ChromeTextSegment::Quad {
        control: [1274, 717],
        to: [1237, 717],
    },
    ChromeTextSegment::Quad {
        control: [1201, 717],
        to: [1175, 691],
    },
    ChromeTextSegment::Quad {
        control: [1150, 666],
        to: [1150, 629],
    },
    ChromeTextSegment::Quad {
        control: [1150, 593],
        to: [1175, 567],
    },
    ChromeTextSegment::Quad {
        control: [1201, 541],
        to: [1237, 541],
    },
    ChromeTextSegment::Quad {
        control: [1274, 541],
        to: [1300, 567],
    },
];

const VERSION_CONTOUR_22: [ChromeTextSegment; 8] = [
    ChromeTextSegment::Quad {
        control: [1274, 1016],
        to: [1300, 1042],
    },
    ChromeTextSegment::Quad {
        control: [1325, 1068],
        to: [1325, 1104],
    },
    ChromeTextSegment::Quad {
        control: [1325, 1140],
        to: [1300, 1166],
    },
    ChromeTextSegment::Quad {
        control: [1274, 1192],
        to: [1237, 1192],
    },
    ChromeTextSegment::Quad {
        control: [1201, 1192],
        to: [1175, 1166],
    },
    ChromeTextSegment::Quad {
        control: [1150, 1140],
        to: [1150, 1104],
    },
    ChromeTextSegment::Quad {
        control: [1150, 1068],
        to: [1175, 1042],
    },
    ChromeTextSegment::Quad {
        control: [1201, 1016],
        to: [1237, 1016],
    },
];

const VERSION_CONTOUR_23: [ChromeTextSegment; 14] = [
    ChromeTextSegment::Line([2047, 1180]),
    ChromeTextSegment::Line([1935, 1180]),
    ChromeTextSegment::Line([1935, 816]),
    ChromeTextSegment::Quad {
        control: [1935, 715],
        to: [1906, 675],
    },
    ChromeTextSegment::Quad {
        control: [1875, 635],
        to: [1804, 635],
    },
    ChromeTextSegment::Quad {
        control: [1766, 635],
        to: [1724, 657],
    },
    ChromeTextSegment::Quad {
        control: [1683, 681],
        to: [1661, 714],
    },
    ChromeTextSegment::Line([1661, 1180]),
    ChromeTextSegment::Line([1550, 1180]),
    ChromeTextSegment::Line([1550, 553]),
    ChromeTextSegment::Line([1626, 553]),
    ChromeTextSegment::Line([1661, 634]),
    ChromeTextSegment::Quad {
        control: [1716, 541],
        to: [1840, 541],
    },
    ChromeTextSegment::Quad {
        control: [2047, 541],
        to: [2047, 792],
    },
];

const VERSION_CONTOUR_24: [ChromeTextSegment; 5] = [
    ChromeTextSegment::Quad {
        control: [2389, 635],
        to: [2338, 683],
    },
    ChromeTextSegment::Quad {
        control: [2290, 729],
        to: [2283, 797],
    },
    ChromeTextSegment::Line([2631, 797]),
    ChromeTextSegment::Quad {
        control: [2631, 729],
        to: [2589, 684],
    },
    ChromeTextSegment::Quad {
        control: [2542, 635],
        to: [2462, 635],
    },
];

const VERSION_CONTOUR_25: [ChromeTextSegment; 14] = [
    ChromeTextSegment::Quad {
        control: [2395, 1098],
        to: [2478, 1098],
    },
    ChromeTextSegment::Quad {
        control: [2574, 1098],
        to: [2637, 1043],
    },
    ChromeTextSegment::Line([2684, 1123]),
    ChromeTextSegment::Quad {
        control: [2658, 1148],
        to: [2605, 1167],
    },
    ChromeTextSegment::Quad {
        control: [2539, 1192],
        to: [2457, 1192],
    },
    ChromeTextSegment::Quad {
        control: [2338, 1192],
        to: [2255, 1112],
    },
    ChromeTextSegment::Quad {
        control: [2164, 1023],
        to: [2164, 874],
    },
    ChromeTextSegment::Quad {
        control: [2164, 718],
        to: [2257, 625],
    },
    ChromeTextSegment::Quad {
        control: [2342, 541],
        to: [2458, 541],
    },
    ChromeTextSegment::Quad {
        control: [2591, 541],
        to: [2668, 616],
    },
    ChromeTextSegment::Quad {
        control: [2741, 689],
        to: [2741, 810],
    },
    ChromeTextSegment::Quad {
        control: [2741, 846],
        to: [2733, 878],
    },
    ChromeTextSegment::Line([2281, 878]),
    ChromeTextSegment::Quad {
        control: [2281, 988],
        to: [2341, 1046],
    },
];

const VERSION_CONTOUR_26: [ChromeTextSegment; 6] = [
    ChromeTextSegment::Quad {
        control: [2936, 1101],
        to: [3102, 1101],
    },
    ChromeTextSegment::Quad {
        control: [3181, 1101],
        to: [3225, 1038],
    },
    ChromeTextSegment::Quad {
        control: [3269, 975],
        to: [3269, 865],
    },
    ChromeTextSegment::Quad {
        control: [3269, 632],
        to: [3102, 632],
    },
    ChromeTextSegment::Quad {
        control: [3026, 632],
        to: [2982, 694],
    },
    ChromeTextSegment::Quad {
        control: [2936, 756],
        to: [2936, 865],
    },
];

const VERSION_CONTOUR_27: [ChromeTextSegment; 8] = [
    ChromeTextSegment::Quad {
        control: [2975, 541],
        to: [3102, 541],
    },
    ChromeTextSegment::Quad {
        control: [3237, 541],
        to: [3312, 627],
    },
    ChromeTextSegment::Quad {
        control: [3386, 712],
        to: [3386, 865],
    },
    ChromeTextSegment::Quad {
        control: [3386, 1017],
        to: [3310, 1105],
    },
    ChromeTextSegment::Quad {
        control: [3234, 1192],
        to: [3102, 1192],
    },
    ChromeTextSegment::Quad {
        control: [2969, 1192],
        to: [2894, 1104],
    },
    ChromeTextSegment::Quad {
        control: [2819, 1015],
        to: [2819, 865],
    },
    ChromeTextSegment::Quad {
        control: [2819, 719],
        to: [2897, 630],
    },
];

const VERSION_CONTOUR_28: [ChromeTextSegment; 12] = [
    ChromeTextSegment::Line([4031, 1180]),
    ChromeTextSegment::Line([3910, 1180]),
    ChromeTextSegment::Line([3713, 866]),
    ChromeTextSegment::Line([3616, 967]),
    ChromeTextSegment::Line([3616, 1180]),
    ChromeTextSegment::Line([3505, 1180]),
    ChromeTextSegment::Line([3505, 295]),
    ChromeTextSegment::Line([3616, 295]),
    ChromeTextSegment::Line([3616, 845]),
    ChromeTextSegment::Line([3856, 553]),
    ChromeTextSegment::Line([3986, 553]),
    ChromeTextSegment::Line([3785, 791]),
];

const VERSION_CONTOUR_29: [ChromeTextSegment; 6] = [
    ChromeTextSegment::Quad {
        control: [4186, 1101],
        to: [4352, 1101],
    },
    ChromeTextSegment::Quad {
        control: [4431, 1101],
        to: [4475, 1038],
    },
    ChromeTextSegment::Quad {
        control: [4519, 975],
        to: [4519, 865],
    },
    ChromeTextSegment::Quad {
        control: [4519, 632],
        to: [4352, 632],
    },
    ChromeTextSegment::Quad {
        control: [4276, 632],
        to: [4232, 694],
    },
    ChromeTextSegment::Quad {
        control: [4186, 756],
        to: [4186, 865],
    },
];

const VERSION_CONTOUR_30: [ChromeTextSegment; 8] = [
    ChromeTextSegment::Quad {
        control: [4225, 541],
        to: [4352, 541],
    },
    ChromeTextSegment::Quad {
        control: [4487, 541],
        to: [4562, 627],
    },
    ChromeTextSegment::Quad {
        control: [4636, 712],
        to: [4636, 865],
    },
    ChromeTextSegment::Quad {
        control: [4636, 1017],
        to: [4560, 1105],
    },
    ChromeTextSegment::Quad {
        control: [4484, 1192],
        to: [4352, 1192],
    },
    ChromeTextSegment::Quad {
        control: [4219, 1192],
        to: [4144, 1104],
    },
    ChromeTextSegment::Quad {
        control: [4069, 1015],
        to: [4069, 865],
    },
    ChromeTextSegment::Quad {
        control: [4069, 719],
        to: [4147, 630],
    },
];

const VERSION_CONTOUR_31: [ChromeTextSegment; 7] = [
    ChromeTextSegment::Line([4980, 1192]),
    ChromeTextSegment::Quad {
        control: [4763, 1192],
        to: [4763, 1003],
    },
    ChromeTextSegment::Line([4763, 295]),
    ChromeTextSegment::Line([4874, 295]),
    ChromeTextSegment::Line([4874, 984]),
    ChromeTextSegment::Quad {
        control: [4874, 1035],
        to: [4904, 1064],
    },
    ChromeTextSegment::Quad {
        control: [4933, 1092],
        to: [4980, 1092],
    },
];

const VERSION_CONTOUR_32: [ChromeTextSegment; 6] = [
    ChromeTextSegment::Quad {
        control: [5186, 1101],
        to: [5352, 1101],
    },
    ChromeTextSegment::Quad {
        control: [5431, 1101],
        to: [5475, 1038],
    },
    ChromeTextSegment::Quad {
        control: [5519, 975],
        to: [5519, 865],
    },
    ChromeTextSegment::Quad {
        control: [5519, 632],
        to: [5352, 632],
    },
    ChromeTextSegment::Quad {
        control: [5276, 632],
        to: [5232, 694],
    },
    ChromeTextSegment::Quad {
        control: [5186, 756],
        to: [5186, 865],
    },
];

const VERSION_CONTOUR_33: [ChromeTextSegment; 8] = [
    ChromeTextSegment::Quad {
        control: [5225, 541],
        to: [5352, 541],
    },
    ChromeTextSegment::Quad {
        control: [5487, 541],
        to: [5562, 627],
    },
    ChromeTextSegment::Quad {
        control: [5636, 712],
        to: [5636, 865],
    },
    ChromeTextSegment::Quad {
        control: [5636, 1017],
        to: [5560, 1105],
    },
    ChromeTextSegment::Quad {
        control: [5484, 1192],
        to: [5352, 1192],
    },
    ChromeTextSegment::Quad {
        control: [5219, 1192],
        to: [5144, 1104],
    },
    ChromeTextSegment::Quad {
        control: [5069, 1015],
        to: [5069, 865],
    },
    ChromeTextSegment::Quad {
        control: [5069, 719],
        to: [5147, 630],
    },
];

const VERSION_CONTOUR_34: [ChromeTextSegment; 11] = [
    ChromeTextSegment::Quad {
        control: [6059, 635],
        to: [6022, 635],
    },
    ChromeTextSegment::Quad {
        control: [5963, 635],
        to: [5919, 689],
    },
    ChromeTextSegment::Quad {
        control: [5874, 744],
        to: [5874, 820],
    },
    ChromeTextSegment::Line([5874, 1180]),
    ChromeTextSegment::Line([5763, 1180]),
    ChromeTextSegment::Line([5763, 553]),
    ChromeTextSegment::Line([5874, 553]),
    ChromeTextSegment::Line([5874, 653]),
    ChromeTextSegment::Quad {
        control: [5935, 541],
        to: [6056, 541],
    },
    ChromeTextSegment::Line([6141, 552]),
    ChromeTextSegment::Line([6096, 660]),
];

const VERSION_CONTOUR_35: [ChromeTextSegment; 16] = [
    ChromeTextSegment::Quad {
        control: [6712, 407],
        to: [6671, 431],
    },
    ChromeTextSegment::Quad {
        control: [6629, 456],
        to: [6610, 494],
    },
    ChromeTextSegment::Line([6535, 432]),
    ChromeTextSegment::Quad {
        control: [6555, 375],
        to: [6614, 341],
    },
    ChromeTextSegment::Quad {
        control: [6671, 307],
        to: [6754, 307],
    },
    ChromeTextSegment::Quad {
        control: [6877, 307],
        to: [6948, 364],
    },
    ChromeTextSegment::Quad {
        control: [7017, 421],
        to: [7017, 525],
    },
    ChromeTextSegment::Quad {
        control: [7017, 622],
        to: [6924, 769],
    },
    ChromeTextSegment::Line([6732, 1075]),
    ChromeTextSegment::Line([7068, 1075]),
    ChromeTextSegment::Line([7068, 1180]),
    ChromeTextSegment::Line([6541, 1180]),
    ChromeTextSegment::Line([6541, 1157]),
    ChromeTextSegment::Line([6809, 745]),
    ChromeTextSegment::Quad {
        control: [6894, 615],
        to: [6894, 525],
    },
    ChromeTextSegment::Quad {
        control: [6894, 407],
        to: [6760, 407],
    },
];

const VERSION_CONTOUR_36: [ChromeTextSegment; 7] = [
    ChromeTextSegment::Quad {
        control: [7285, 407],
        to: [7285, 736],
    },
    ChromeTextSegment::Quad {
        control: [7285, 1092],
        to: [7437, 1092],
    },
    ChromeTextSegment::Quad {
        control: [7525, 1092],
        to: [7566, 1014],
    },
    ChromeTextSegment::Quad {
        control: [7607, 934],
        to: [7607, 736],
    },
    ChromeTextSegment::Quad {
        control: [7607, 600],
        to: [7591, 535],
    },
    ChromeTextSegment::Quad {
        control: [7575, 472],
        to: [7540, 439],
    },
    ChromeTextSegment::Quad {
        control: [7505, 407],
        to: [7451, 407],
    },
];

const VERSION_CONTOUR_37: [ChromeTextSegment; 8] = [
    ChromeTextSegment::Quad {
        control: [7588, 307],
        to: [7656, 404],
    },
    ChromeTextSegment::Quad {
        control: [7724, 501],
        to: [7724, 744],
    },
    ChromeTextSegment::Quad {
        control: [7724, 953],
        to: [7653, 1072],
    },
    ChromeTextSegment::Quad {
        control: [7582, 1192],
        to: [7445, 1192],
    },
    ChromeTextSegment::Quad {
        control: [7309, 1192],
        to: [7239, 1096],
    },
    ChromeTextSegment::Quad {
        control: [7168, 998],
        to: [7168, 725],
    },
    ChromeTextSegment::Quad {
        control: [7168, 540],
        to: [7242, 423],
    },
    ChromeTextSegment::Quad {
        control: [7318, 307],
        to: [7453, 307],
    },
];

const VERSION_CONTOUR_38: [ChromeTextSegment; 7] = [
    ChromeTextSegment::Quad {
        control: [7915, 407],
        to: [7915, 736],
    },
    ChromeTextSegment::Quad {
        control: [7915, 1092],
        to: [8067, 1092],
    },
    ChromeTextSegment::Quad {
        control: [8155, 1092],
        to: [8196, 1014],
    },
    ChromeTextSegment::Quad {
        control: [8237, 934],
        to: [8237, 736],
    },
    ChromeTextSegment::Quad {
        control: [8237, 600],
        to: [8221, 535],
    },
    ChromeTextSegment::Quad {
        control: [8205, 472],
        to: [8170, 439],
    },
    ChromeTextSegment::Quad {
        control: [8135, 407],
        to: [8081, 407],
    },
];

const VERSION_CONTOUR_39: [ChromeTextSegment; 8] = [
    ChromeTextSegment::Quad {
        control: [8218, 307],
        to: [8286, 404],
    },
    ChromeTextSegment::Quad {
        control: [8354, 501],
        to: [8354, 744],
    },
    ChromeTextSegment::Quad {
        control: [8354, 953],
        to: [8283, 1072],
    },
    ChromeTextSegment::Quad {
        control: [8212, 1192],
        to: [8075, 1192],
    },
    ChromeTextSegment::Quad {
        control: [7939, 1192],
        to: [7869, 1096],
    },
    ChromeTextSegment::Quad {
        control: [7798, 998],
        to: [7798, 725],
    },
    ChromeTextSegment::Quad {
        control: [7798, 540],
        to: [7873, 423],
    },
    ChromeTextSegment::Quad {
        control: [7948, 307],
        to: [8083, 307],
    },
];

const VERSION_CONTOUR_40: [ChromeTextSegment; 8] = [
    ChromeTextSegment::Line([8799, 1180]),
    ChromeTextSegment::Line([8682, 1180]),
    ChromeTextSegment::Line([8682, 523]),
    ChromeTextSegment::Line([8507, 632]),
    ChromeTextSegment::Line([8507, 521]),
    ChromeTextSegment::Quad {
        control: [8574, 489],
        to: [8649, 430],
    },
    ChromeTextSegment::Quad {
        control: [8723, 371],
        to: [8764, 319],
    },
    ChromeTextSegment::Line([8799, 319]),
];

const VERSION_CONTOUR_41: [ChromeTextSegment; 4] = [
    ChromeTextSegment::Line([9114, 782]),
    ChromeTextSegment::Line([9365, 782]),
    ChromeTextSegment::Line([9365, 884]),
    ChromeTextSegment::Line([9114, 884]),
];

const VERSION_CONTOUR_42: [ChromeTextSegment; 16] = [
    ChromeTextSegment::Quad {
        control: [9672, 407],
        to: [9631, 431],
    },
    ChromeTextSegment::Quad {
        control: [9589, 456],
        to: [9570, 494],
    },
    ChromeTextSegment::Line([9495, 432]),
    ChromeTextSegment::Quad {
        control: [9515, 375],
        to: [9574, 341],
    },
    ChromeTextSegment::Quad {
        control: [9631, 307],
        to: [9714, 307],
    },
    ChromeTextSegment::Quad {
        control: [9837, 307],
        to: [9908, 364],
    },
    ChromeTextSegment::Quad {
        control: [9977, 421],
        to: [9977, 525],
    },
    ChromeTextSegment::Quad {
        control: [9977, 622],
        to: [9884, 769],
    },
    ChromeTextSegment::Line([9692, 1075]),
    ChromeTextSegment::Line([10028, 1075]),
    ChromeTextSegment::Line([10028, 1180]),
    ChromeTextSegment::Line([9501, 1180]),
    ChromeTextSegment::Line([9501, 1157]),
    ChromeTextSegment::Line([9769, 745]),
    ChromeTextSegment::Quad {
        control: [9854, 615],
        to: [9854, 525],
    },
    ChromeTextSegment::Quad {
        control: [9854, 407],
        to: [9720, 407],
    },
];

const VERSION_CONTOUR_43: [ChromeTextSegment; 18] = [
    ChromeTextSegment::Quad {
        control: [10861, 541],
        to: [10923, 603],
    },
    ChromeTextSegment::Quad {
        control: [10984, 666],
        to: [10984, 800],
    },
    ChromeTextSegment::Line([10984, 1025]),
    ChromeTextSegment::Quad {
        control: [10984, 1109],
        to: [11034, 1135],
    },
    ChromeTextSegment::Line([11034, 1192]),
    ChromeTextSegment::Quad {
        control: [10966, 1192],
        to: [10933, 1172],
    },
    ChromeTextSegment::Quad {
        control: [10899, 1153],
        to: [10884, 1109],
    },
    ChromeTextSegment::Quad {
        control: [10817, 1192],
        to: [10680, 1192],
    },
    ChromeTextSegment::Quad {
        control: [10606, 1192],
        to: [10552, 1139],
    },
    ChromeTextSegment::Quad {
        control: [10497, 1085],
        to: [10497, 1005],
    },
    ChromeTextSegment::Quad {
        control: [10497, 909],
        to: [10581, 844],
    },
    ChromeTextSegment::Quad {
        control: [10664, 778],
        to: [10793, 778],
    },
    ChromeTextSegment::Line([10873, 793]),
    ChromeTextSegment::Quad {
        control: [10873, 641],
        to: [10737, 641],
    },
    ChromeTextSegment::Quad {
        control: [10633, 641],
        to: [10577, 697],
    },
    ChromeTextSegment::Line([10530, 603]),
    ChromeTextSegment::Quad {
        control: [10561, 578],
        to: [10618, 560],
    },
    ChromeTextSegment::Quad {
        control: [10674, 541],
        to: [10724, 541],
    },
];

const VERSION_CONTOUR_44: [ChromeTextSegment; 6] = [
    ChromeTextSegment::Quad {
        control: [10715, 860],
        to: [10662, 903],
    },
    ChromeTextSegment::Quad {
        control: [10608, 947],
        to: [10608, 1007],
    },
    ChromeTextSegment::Quad {
        control: [10608, 1104],
        to: [10724, 1104],
    },
    ChromeTextSegment::Quad {
        control: [10809, 1104],
        to: [10873, 1024],
    },
    ChromeTextSegment::Line([10873, 872]),
    ChromeTextSegment::Line([10799, 860]),
];

const VERSION_CONTOUR_45: [ChromeTextSegment; 14] = [
    ChromeTextSegment::Line([11657, 1180]),
    ChromeTextSegment::Line([11545, 1180]),
    ChromeTextSegment::Line([11545, 816]),
    ChromeTextSegment::Quad {
        control: [11545, 715],
        to: [11516, 675],
    },
    ChromeTextSegment::Quad {
        control: [11485, 635],
        to: [11414, 635],
    },
    ChromeTextSegment::Quad {
        control: [11376, 635],
        to: [11334, 657],
    },
    ChromeTextSegment::Quad {
        control: [11293, 681],
        to: [11271, 714],
    },
    ChromeTextSegment::Line([11271, 1180]),
    ChromeTextSegment::Line([11160, 1180]),
    ChromeTextSegment::Line([11160, 553]),
    ChromeTextSegment::Line([11236, 553]),
    ChromeTextSegment::Line([11271, 634]),
    ChromeTextSegment::Quad {
        control: [11326, 541],
        to: [11450, 541],
    },
    ChromeTextSegment::Quad {
        control: [11657, 541],
        to: [11657, 792],
    },
];

const VERSION_CONTOUR_46: [ChromeTextSegment; 11] = [
    ChromeTextSegment::Line([12323, 295]),
    ChromeTextSegment::Line([12323, 1180]),
    ChromeTextSegment::Line([12212, 1180]),
    ChromeTextSegment::Line([12212, 1133]),
    ChromeTextSegment::Quad {
        control: [12155, 1192],
        to: [12043, 1192],
    },
    ChromeTextSegment::Quad {
        control: [11926, 1192],
        to: [11852, 1107],
    },
    ChromeTextSegment::Quad {
        control: [11780, 1023],
        to: [11780, 882],
    },
    ChromeTextSegment::Quad {
        control: [11780, 741],
        to: [11864, 641],
    },
    ChromeTextSegment::Quad {
        control: [11948, 541],
        to: [12064, 541],
    },
    ChromeTextSegment::Quad {
        control: [12162, 541],
        to: [12212, 587],
    },
    ChromeTextSegment::Line([12212, 295]),
];

const VERSION_CONTOUR_47: [ChromeTextSegment; 7] = [
    ChromeTextSegment::Quad {
        control: [12203, 1065],
        to: [12212, 1046],
    },
    ChromeTextSegment::Line([12212, 698]),
    ChromeTextSegment::Quad {
        control: [12170, 635],
        to: [12097, 635],
    },
    ChromeTextSegment::Quad {
        control: [12007, 635],
        to: [11952, 702],
    },
    ChromeTextSegment::Quad {
        control: [11897, 769],
        to: [11897, 872],
    },
    ChromeTextSegment::Quad {
        control: [11897, 1098],
        to: [12103, 1098],
    },
    ChromeTextSegment::Quad {
        control: [12129, 1098],
        to: [12166, 1082],
    },
];

const VERSION_CONTOUR_48: [ChromeTextSegment; 27] = [
    ChromeTextSegment::Quad {
        control: [12954, 615],
        to: [12976, 633],
    },
    ChromeTextSegment::Quad {
        control: [12998, 651],
        to: [13066, 684],
    },
    ChromeTextSegment::Line([13136, 717]),
    ChromeTextSegment::Quad {
        control: [13224, 759],
        to: [13260, 817],
    },
    ChromeTextSegment::Quad {
        control: [13295, 874],
        to: [13295, 963],
    },
    ChromeTextSegment::Quad {
        control: [13295, 1060],
        to: [13217, 1127],
    },
    ChromeTextSegment::Quad {
        control: [13140, 1195],
        to: [13010, 1195],
    },
    ChromeTextSegment::Quad {
        control: [12895, 1195],
        to: [12814, 1141],
    },
    ChromeTextSegment::Line([12858, 1034]),
    ChromeTextSegment::Quad {
        control: [12890, 1057],
        to: [12940, 1073],
    },
    ChromeTextSegment::Quad {
        control: [12988, 1090],
        to: [13026, 1090],
    },
    ChromeTextSegment::Quad {
        control: [13095, 1090],
        to: [13136, 1052],
    },
    ChromeTextSegment::Quad {
        control: [13179, 1015],
        to: [13179, 956],
    },
    ChromeTextSegment::Quad {
        control: [13179, 913],
        to: [13155, 876],
    },
    ChromeTextSegment::Quad {
        control: [13132, 839],
        to: [13039, 794],
    },
    ChromeTextSegment::Line([12970, 763]),
    ChromeTextSegment::Quad {
        control: [12882, 722],
        to: [12847, 666],
    },
    ChromeTextSegment::Quad {
        control: [12812, 609],
        to: [12812, 530],
    },
    ChromeTextSegment::Quad {
        control: [12812, 434],
        to: [12880, 370],
    },
    ChromeTextSegment::Quad {
        control: [12948, 307],
        to: [13054, 307],
    },
    ChromeTextSegment::Quad {
        control: [13197, 307],
        to: [13253, 354],
    },
    ChromeTextSegment::Line([13219, 456]),
    ChromeTextSegment::Quad {
        control: [13195, 438],
        to: [13147, 423],
    },
    ChromeTextSegment::Line([13058, 407]),
    ChromeTextSegment::Quad {
        control: [12998, 407],
        to: [12964, 441],
    },
    ChromeTextSegment::Quad {
        control: [12929, 475],
        to: [12929, 528],
    },
    ChromeTextSegment::Quad {
        control: [12929, 561],
        to: [12942, 588],
    },
];

const VERSION_CONTOUR_49: [ChromeTextSegment; 8] = [
    ChromeTextSegment::Quad {
        control: [13563, 315],
        to: [13584, 336],
    },
    ChromeTextSegment::Quad {
        control: [13604, 356],
        to: [13604, 384],
    },
    ChromeTextSegment::Quad {
        control: [13604, 412],
        to: [13584, 434],
    },
    ChromeTextSegment::Quad {
        control: [13563, 453],
        to: [13535, 453],
    },
    ChromeTextSegment::Quad {
        control: [13506, 453],
        to: [13486, 434],
    },
    ChromeTextSegment::Quad {
        control: [13465, 412],
        to: [13465, 384],
    },
    ChromeTextSegment::Quad {
        control: [13465, 355],
        to: [13485, 335],
    },
    ChromeTextSegment::Quad {
        control: [13505, 315],
        to: [13535, 315],
    },
];

const VERSION_CONTOUR_50: [ChromeTextSegment; 6] = [
    ChromeTextSegment::Line([13585, 1180]),
    ChromeTextSegment::Line([13474, 1180]),
    ChromeTextSegment::Line([13474, 647]),
    ChromeTextSegment::Line([13387, 647]),
    ChromeTextSegment::Line([13387, 553]),
    ChromeTextSegment::Line([13585, 553]),
];

const VERSION_CONTOUR_51: [ChromeTextSegment; 11] = [
    ChromeTextSegment::Quad {
        control: [14064, 635],
        to: [14027, 635],
    },
    ChromeTextSegment::Quad {
        control: [13968, 635],
        to: [13924, 689],
    },
    ChromeTextSegment::Quad {
        control: [13879, 744],
        to: [13879, 820],
    },
    ChromeTextSegment::Line([13879, 1180]),
    ChromeTextSegment::Line([13768, 1180]),
    ChromeTextSegment::Line([13768, 553]),
    ChromeTextSegment::Line([13879, 553]),
    ChromeTextSegment::Line([13879, 653]),
    ChromeTextSegment::Quad {
        control: [13940, 541],
        to: [14061, 541],
    },
    ChromeTextSegment::Line([14146, 552]),
    ChromeTextSegment::Line([14101, 660]),
];

const VERSION_CONTOUR_52: [ChromeTextSegment; 4] = [
    ChromeTextSegment::Line([14731, 1180]),
    ChromeTextSegment::Line([14614, 1180]),
    ChromeTextSegment::Line([14614, 322]),
    ChromeTextSegment::Line([14731, 322]),
];

const VERSION_CONTOUR_53: [ChromeTextSegment; 22] = [
    ChromeTextSegment::Line([15227, 688]),
    ChromeTextSegment::Quad {
        control: [15161, 635],
        to: [15094, 635],
    },
    ChromeTextSegment::Quad {
        control: [15054, 635],
        to: [15028, 654],
    },
    ChromeTextSegment::Quad {
        control: [14999, 673],
        to: [14999, 701],
    },
    ChromeTextSegment::Quad {
        control: [14999, 762],
        to: [15069, 792],
    },
    ChromeTextSegment::Line([15148, 828]),
    ChromeTextSegment::Quad {
        control: [15221, 862],
        to: [15255, 905],
    },
    ChromeTextSegment::Quad {
        control: [15288, 948],
        to: [15288, 1012],
    },
    ChromeTextSegment::Quad {
        control: [15288, 1097],
        to: [15229, 1145],
    },
    ChromeTextSegment::Quad {
        control: [15169, 1192],
        to: [15065, 1192],
    },
    ChromeTextSegment::Quad {
        control: [14965, 1192],
        to: [14879, 1142],
    },
    ChromeTextSegment::Line([14917, 1037]),
    ChromeTextSegment::Quad {
        control: [15011, 1098],
        to: [15067, 1098],
    },
    ChromeTextSegment::Quad {
        control: [15170, 1098],
        to: [15170, 1011],
    },
    ChromeTextSegment::Quad {
        control: [15170, 949],
        to: [15071, 905],
    },
    ChromeTextSegment::Quad {
        control: [14995, 869],
        to: [14968, 852],
    },
    ChromeTextSegment::Quad {
        control: [14941, 833],
        to: [14922, 811],
    },
    ChromeTextSegment::Quad {
        control: [14902, 787],
        to: [14893, 762],
    },
    ChromeTextSegment::Quad {
        control: [14882, 735],
        to: [14882, 705],
    },
    ChromeTextSegment::Quad {
        control: [14882, 628],
        to: [14938, 585],
    },
    ChromeTextSegment::Quad {
        control: [14995, 541],
        to: [15086, 541],
    },
    ChromeTextSegment::Quad {
        control: [15154, 541],
        to: [15258, 585],
    },
];

const VERSION_CONTOUR_54: [ChromeTextSegment; 18] = [
    ChromeTextSegment::Quad {
        control: [15736, 541],
        to: [15798, 603],
    },
    ChromeTextSegment::Quad {
        control: [15859, 666],
        to: [15859, 800],
    },
    ChromeTextSegment::Line([15859, 1025]),
    ChromeTextSegment::Quad {
        control: [15859, 1109],
        to: [15909, 1135],
    },
    ChromeTextSegment::Line([15909, 1192]),
    ChromeTextSegment::Quad {
        control: [15841, 1192],
        to: [15808, 1172],
    },
    ChromeTextSegment::Quad {
        control: [15774, 1153],
        to: [15759, 1109],
    },
    ChromeTextSegment::Quad {
        control: [15692, 1192],
        to: [15555, 1192],
    },
    ChromeTextSegment::Quad {
        control: [15481, 1192],
        to: [15427, 1139],
    },
    ChromeTextSegment::Quad {
        control: [15372, 1085],
        to: [15372, 1005],
    },
    ChromeTextSegment::Quad {
        control: [15372, 909],
        to: [15456, 844],
    },
    ChromeTextSegment::Quad {
        control: [15539, 778],
        to: [15668, 778],
    },
    ChromeTextSegment::Line([15748, 793]),
    ChromeTextSegment::Quad {
        control: [15748, 641],
        to: [15612, 641],
    },
    ChromeTextSegment::Quad {
        control: [15508, 641],
        to: [15452, 697],
    },
    ChromeTextSegment::Line([15405, 603]),
    ChromeTextSegment::Quad {
        control: [15436, 578],
        to: [15493, 560],
    },
    ChromeTextSegment::Quad {
        control: [15549, 541],
        to: [15599, 541],
    },
];

const VERSION_CONTOUR_55: [ChromeTextSegment; 6] = [
    ChromeTextSegment::Quad {
        control: [15590, 860],
        to: [15537, 903],
    },
    ChromeTextSegment::Quad {
        control: [15483, 947],
        to: [15483, 1007],
    },
    ChromeTextSegment::Quad {
        control: [15483, 1104],
        to: [15599, 1104],
    },
    ChromeTextSegment::Quad {
        control: [15684, 1104],
        to: [15748, 1024],
    },
    ChromeTextSegment::Line([15748, 872]),
    ChromeTextSegment::Line([15674, 860]),
];

const VERSION_CONTOUR_56: [ChromeTextSegment; 18] = [
    ChromeTextSegment::Quad {
        control: [16366, 541],
        to: [16428, 603],
    },
    ChromeTextSegment::Quad {
        control: [16489, 666],
        to: [16489, 800],
    },
    ChromeTextSegment::Line([16489, 1025]),
    ChromeTextSegment::Quad {
        control: [16489, 1109],
        to: [16539, 1135],
    },
    ChromeTextSegment::Line([16539, 1192]),
    ChromeTextSegment::Quad {
        control: [16471, 1192],
        to: [16438, 1172],
    },
    ChromeTextSegment::Quad {
        control: [16404, 1153],
        to: [16389, 1109],
    },
    ChromeTextSegment::Quad {
        control: [16322, 1192],
        to: [16185, 1192],
    },
    ChromeTextSegment::Quad {
        control: [16111, 1192],
        to: [16057, 1139],
    },
    ChromeTextSegment::Quad {
        control: [16002, 1085],
        to: [16002, 1005],
    },
    ChromeTextSegment::Quad {
        control: [16002, 909],
        to: [16086, 844],
    },
    ChromeTextSegment::Quad {
        control: [16169, 778],
        to: [16298, 778],
    },
    ChromeTextSegment::Line([16378, 793]),
    ChromeTextSegment::Quad {
        control: [16378, 641],
        to: [16242, 641],
    },
    ChromeTextSegment::Quad {
        control: [16138, 641],
        to: [16082, 697],
    },
    ChromeTextSegment::Line([16035, 603]),
    ChromeTextSegment::Quad {
        control: [16066, 578],
        to: [16123, 560],
    },
    ChromeTextSegment::Quad {
        control: [16179, 541],
        to: [16229, 541],
    },
];

const VERSION_CONTOUR_57: [ChromeTextSegment; 6] = [
    ChromeTextSegment::Quad {
        control: [16220, 860],
        to: [16167, 903],
    },
    ChromeTextSegment::Quad {
        control: [16113, 947],
        to: [16113, 1007],
    },
    ChromeTextSegment::Quad {
        control: [16113, 1104],
        to: [16229, 1104],
    },
    ChromeTextSegment::Quad {
        control: [16314, 1104],
        to: [16378, 1024],
    },
    ChromeTextSegment::Line([16378, 872]),
    ChromeTextSegment::Line([16304, 860]),
];

const VERSION_CONTOUR_58: [ChromeTextSegment; 16] = [
    ChromeTextSegment::Quad {
        control: [17111, 582],
        to: [17138, 603],
    },
    ChromeTextSegment::Line([17083, 682]),
    ChromeTextSegment::Quad {
        control: [17065, 666],
        to: [17023, 650],
    },
    ChromeTextSegment::Line([16938, 635]),
    ChromeTextSegment::Quad {
        control: [16848, 635],
        to: [16794, 698],
    },
    ChromeTextSegment::Quad {
        control: [16741, 762],
        to: [16741, 873],
    },
    ChromeTextSegment::Quad {
        control: [16741, 983],
        to: [16795, 1041],
    },
    ChromeTextSegment::Quad {
        control: [16850, 1098],
        to: [16946, 1098],
    },
    ChromeTextSegment::Quad {
        control: [17021, 1098],
        to: [17097, 1041],
    },
    ChromeTextSegment::Line([17142, 1134]),
    ChromeTextSegment::Quad {
        control: [17051, 1192],
        to: [16919, 1192],
    },
    ChromeTextSegment::Quad {
        control: [16791, 1192],
        to: [16707, 1106],
    },
    ChromeTextSegment::Quad {
        control: [16624, 1019],
        to: [16624, 873],
    },
    ChromeTextSegment::Quad {
        control: [16624, 723],
        to: [16710, 632],
    },
    ChromeTextSegment::Quad {
        control: [16797, 541],
        to: [16948, 541],
    },
    ChromeTextSegment::Line([17054, 561]),
];

const VERSION_CONTOUR_59: [ChromeTextSegment; 10] = [
    ChromeTextSegment::Line([18219, 1192]),
    ChromeTextSegment::Line([18183, 1192]),
    ChromeTextSegment::Line([17739, 565]),
    ChromeTextSegment::Line([17739, 1180]),
    ChromeTextSegment::Line([17628, 1180]),
    ChromeTextSegment::Line([17628, 322]),
    ChromeTextSegment::Line([17675, 322]),
    ChromeTextSegment::Line([18107, 915]),
    ChromeTextSegment::Line([18107, 322]),
    ChromeTextSegment::Line([18219, 322]),
];

const VERSION_CONTOUR_60: [ChromeTextSegment; 5] = [
    ChromeTextSegment::Quad {
        control: [18569, 635],
        to: [18518, 683],
    },
    ChromeTextSegment::Quad {
        control: [18470, 729],
        to: [18463, 797],
    },
    ChromeTextSegment::Line([18811, 797]),
    ChromeTextSegment::Quad {
        control: [18811, 729],
        to: [18769, 684],
    },
    ChromeTextSegment::Quad {
        control: [18722, 635],
        to: [18643, 635],
    },
];

const VERSION_CONTOUR_61: [ChromeTextSegment; 14] = [
    ChromeTextSegment::Quad {
        control: [18575, 1098],
        to: [18658, 1098],
    },
    ChromeTextSegment::Quad {
        control: [18754, 1098],
        to: [18817, 1043],
    },
    ChromeTextSegment::Line([18864, 1123]),
    ChromeTextSegment::Quad {
        control: [18838, 1148],
        to: [18785, 1167],
    },
    ChromeTextSegment::Quad {
        control: [18719, 1192],
        to: [18637, 1192],
    },
    ChromeTextSegment::Quad {
        control: [18518, 1192],
        to: [18435, 1112],
    },
    ChromeTextSegment::Quad {
        control: [18344, 1023],
        to: [18344, 874],
    },
    ChromeTextSegment::Quad {
        control: [18344, 718],
        to: [18437, 625],
    },
    ChromeTextSegment::Quad {
        control: [18522, 541],
        to: [18638, 541],
    },
    ChromeTextSegment::Quad {
        control: [18771, 541],
        to: [18848, 616],
    },
    ChromeTextSegment::Quad {
        control: [18921, 689],
        to: [18921, 810],
    },
    ChromeTextSegment::Quad {
        control: [18921, 846],
        to: [18913, 878],
    },
    ChromeTextSegment::Line([18461, 878]),
    ChromeTextSegment::Quad {
        control: [18461, 988],
        to: [18521, 1046],
    },
];

const VERSION_CONTOUR_62: [ChromeTextSegment; 13] = [
    ChromeTextSegment::Line([19620, 1192]),
    ChromeTextSegment::Line([19590, 1192]),
    ChromeTextSegment::Line([19406, 765]),
    ChromeTextSegment::Line([19223, 1192]),
    ChromeTextSegment::Line([19193, 1192]),
    ChromeTextSegment::Line([18969, 551]),
    ChromeTextSegment::Line([19088, 551]),
    ChromeTextSegment::Line([19223, 963]),
    ChromeTextSegment::Line([19389, 551]),
    ChromeTextSegment::Line([19418, 551]),
    ChromeTextSegment::Line([19590, 963]),
    ChromeTextSegment::Line([19735, 551]),
    ChromeTextSegment::Line([19845, 551]),
];

const VERSION_CONTOUR_63: [ChromeTextSegment; 18] = [
    ChromeTextSegment::Line([19975, 422]),
    ChromeTextSegment::Line([20086, 378]),
    ChromeTextSegment::Line([20086, 553]),
    ChromeTextSegment::Line([20258, 553]),
    ChromeTextSegment::Line([20258, 641]),
    ChromeTextSegment::Line([20086, 641]),
    ChromeTextSegment::Line([20086, 953]),
    ChromeTextSegment::Quad {
        control: [20086, 1031],
        to: [20113, 1065],
    },
    ChromeTextSegment::Quad {
        control: [20139, 1098],
        to: [20198, 1098],
    },
    ChromeTextSegment::Quad {
        control: [20241, 1098],
        to: [20286, 1077],
    },
    ChromeTextSegment::Line([20303, 1174]),
    ChromeTextSegment::Line([20151, 1192]),
    ChromeTextSegment::Quad {
        control: [20076, 1192],
        to: [20026, 1137],
    },
    ChromeTextSegment::Quad {
        control: [19975, 1082],
        to: [19975, 997],
    },
    ChromeTextSegment::Line([19975, 641]),
    ChromeTextSegment::Line([19902, 641]),
    ChromeTextSegment::Line([19902, 553]),
    ChromeTextSegment::Line([19975, 553]),
];

const VERSION_CONTOUR_64: [ChromeTextSegment; 6] = [
    ChromeTextSegment::Quad {
        control: [20486, 1101],
        to: [20652, 1101],
    },
    ChromeTextSegment::Quad {
        control: [20731, 1101],
        to: [20775, 1038],
    },
    ChromeTextSegment::Quad {
        control: [20819, 975],
        to: [20819, 865],
    },
    ChromeTextSegment::Quad {
        control: [20819, 632],
        to: [20652, 632],
    },
    ChromeTextSegment::Quad {
        control: [20576, 632],
        to: [20532, 694],
    },
    ChromeTextSegment::Quad {
        control: [20486, 756],
        to: [20486, 865],
    },
];

const VERSION_CONTOUR_65: [ChromeTextSegment; 8] = [
    ChromeTextSegment::Quad {
        control: [20525, 541],
        to: [20652, 541],
    },
    ChromeTextSegment::Quad {
        control: [20787, 541],
        to: [20862, 627],
    },
    ChromeTextSegment::Quad {
        control: [20936, 712],
        to: [20936, 865],
    },
    ChromeTextSegment::Quad {
        control: [20936, 1017],
        to: [20860, 1105],
    },
    ChromeTextSegment::Quad {
        control: [20784, 1192],
        to: [20652, 1192],
    },
    ChromeTextSegment::Quad {
        control: [20519, 1192],
        to: [20444, 1104],
    },
    ChromeTextSegment::Quad {
        control: [20369, 1015],
        to: [20369, 865],
    },
    ChromeTextSegment::Quad {
        control: [20369, 719],
        to: [20447, 630],
    },
];

const VERSION_CONTOUR_66: [ChromeTextSegment; 14] = [
    ChromeTextSegment::Line([21552, 1180]),
    ChromeTextSegment::Line([21440, 1180]),
    ChromeTextSegment::Line([21440, 816]),
    ChromeTextSegment::Quad {
        control: [21440, 715],
        to: [21411, 675],
    },
    ChromeTextSegment::Quad {
        control: [21380, 635],
        to: [21309, 635],
    },
    ChromeTextSegment::Quad {
        control: [21271, 635],
        to: [21229, 657],
    },
    ChromeTextSegment::Quad {
        control: [21188, 681],
        to: [21166, 714],
    },
    ChromeTextSegment::Line([21166, 1180]),
    ChromeTextSegment::Line([21055, 1180]),
    ChromeTextSegment::Line([21055, 553]),
    ChromeTextSegment::Line([21131, 553]),
    ChromeTextSegment::Line([21166, 634]),
    ChromeTextSegment::Quad {
        control: [21221, 541],
        to: [21345, 541],
    },
    ChromeTextSegment::Quad {
        control: [21552, 541],
        to: [21552, 792],
    },
];

const VERSION_CONTOUR_67: [ChromeTextSegment; 8] = [
    ChromeTextSegment::Line([22399, 1180]),
    ChromeTextSegment::Line([22282, 1180]),
    ChromeTextSegment::Line([22282, 523]),
    ChromeTextSegment::Line([22107, 632]),
    ChromeTextSegment::Line([22107, 521]),
    ChromeTextSegment::Quad {
        control: [22174, 489],
        to: [22249, 430],
    },
    ChromeTextSegment::Quad {
        control: [22323, 371],
        to: [22364, 319],
    },
    ChromeTextSegment::Line([22399, 319]),
];

const VERSION_CONTOUR_68: [ChromeTextSegment; 11] = [
    ChromeTextSegment::Quad {
        control: [23020, 390],
        to: [22922, 509],
    },
    ChromeTextSegment::Quad {
        control: [22824, 626],
        to: [22813, 690],
    },
    ChromeTextSegment::Quad {
        control: [22863, 643],
        to: [22949, 643],
    },
    ChromeTextSegment::Quad {
        control: [23063, 643],
        to: [23131, 716],
    },
    ChromeTextSegment::Quad {
        control: [23200, 789],
        to: [23200, 915],
    },
    ChromeTextSegment::Quad {
        control: [23200, 1042],
        to: [23130, 1118],
    },
    ChromeTextSegment::Quad {
        control: [23058, 1195],
        to: [22952, 1195],
    },
    ChromeTextSegment::Quad {
        control: [22669, 1195],
        to: [22669, 820],
    },
    ChromeTextSegment::Quad {
        control: [22669, 664],
        to: [22781, 501],
    },
    ChromeTextSegment::Quad {
        control: [22891, 339],
        to: [23014, 307],
    },
    ChromeTextSegment::Line([23071, 369]),
];

const VERSION_CONTOUR_69: [ChromeTextSegment; 6] = [
    ChromeTextSegment::Quad {
        control: [22792, 1096],
        to: [22943, 1096],
    },
    ChromeTextSegment::Quad {
        control: [23007, 1096],
        to: [23043, 1048],
    },
    ChromeTextSegment::Quad {
        control: [23079, 1000],
        to: [23079, 921],
    },
    ChromeTextSegment::Quad {
        control: [23079, 840],
        to: [23041, 791],
    },
    ChromeTextSegment::Quad {
        control: [23002, 743],
        to: [22941, 743],
    },
    ChromeTextSegment::Quad {
        control: [22792, 743],
        to: [22792, 912],
    },
];

const VERSION_CONTOUR_70: [ChromeTextSegment; 14] = [
    ChromeTextSegment::Quad {
        control: [23798, 571],
        to: [23761, 629],
    },
    ChromeTextSegment::Quad {
        control: [23725, 687],
        to: [23675, 715],
    },
    ChromeTextSegment::Quad {
        control: [23830, 801],
        to: [23830, 942],
    },
    ChromeTextSegment::Quad {
        control: [23830, 1062],
        to: [23759, 1127],
    },
    ChromeTextSegment::Quad {
        control: [23687, 1192],
        to: [23563, 1192],
    },
    ChromeTextSegment::Quad {
        control: [23303, 1192],
        to: [23303, 942],
    },
    ChromeTextSegment::Quad {
        control: [23303, 871],
        to: [23347, 800],
    },
    ChromeTextSegment::Quad {
        control: [23392, 731],
        to: [23457, 703],
    },
    ChromeTextSegment::Quad {
        control: [23402, 674],
        to: [23368, 621],
    },
    ChromeTextSegment::Quad {
        control: [23333, 569],
        to: [23333, 513],
    },
    ChromeTextSegment::Quad {
        control: [23333, 418],
        to: [23399, 362],
    },
    ChromeTextSegment::Quad {
        control: [23463, 307],
        to: [23565, 307],
    },
    ChromeTextSegment::Quad {
        control: [23677, 307],
        to: [23738, 362],
    },
    ChromeTextSegment::Quad {
        control: [23798, 418],
        to: [23798, 514],
    },
];

const VERSION_CONTOUR_71: [ChromeTextSegment; 5] = [
    ChromeTextSegment::Quad {
        control: [23450, 591],
        to: [23600, 667],
    },
    ChromeTextSegment::Quad {
        control: [23680, 592],
        to: [23680, 511],
    },
    ChromeTextSegment::Quad {
        control: [23680, 463],
        to: [23648, 435],
    },
    ChromeTextSegment::Quad {
        control: [23617, 407],
        to: [23565, 407],
    },
    ChromeTextSegment::Quad {
        control: [23450, 407],
        to: [23450, 512],
    },
];

const VERSION_CONTOUR_72: [ChromeTextSegment; 7] = [
    ChromeTextSegment::Quad {
        control: [23420, 824],
        to: [23420, 942],
    },
    ChromeTextSegment::Quad {
        control: [23420, 1007],
        to: [23460, 1050],
    },
    ChromeTextSegment::Quad {
        control: [23500, 1092],
        to: [23563, 1092],
    },
    ChromeTextSegment::Quad {
        control: [23629, 1092],
        to: [23671, 1050],
    },
    ChromeTextSegment::Quad {
        control: [23713, 1008],
        to: [23713, 942],
    },
    ChromeTextSegment::Quad {
        control: [23713, 895],
        to: [23684, 853],
    },
    ChromeTextSegment::Quad {
        control: [23654, 811],
        to: [23549, 753],
    },
];

const VERSION_CONTOUR_73: [ChromeTextSegment; 11] = [
    ChromeTextSegment::Quad {
        control: [24280, 390],
        to: [24182, 509],
    },
    ChromeTextSegment::Quad {
        control: [24084, 626],
        to: [24073, 690],
    },
    ChromeTextSegment::Quad {
        control: [24123, 643],
        to: [24209, 643],
    },
    ChromeTextSegment::Quad {
        control: [24323, 643],
        to: [24391, 716],
    },
    ChromeTextSegment::Quad {
        control: [24460, 789],
        to: [24460, 915],
    },
    ChromeTextSegment::Quad {
        control: [24460, 1042],
        to: [24390, 1118],
    },
    ChromeTextSegment::Quad {
        control: [24318, 1195],
        to: [24212, 1195],
    },
    ChromeTextSegment::Quad {
        control: [23929, 1195],
        to: [23929, 820],
    },
    ChromeTextSegment::Quad {
        control: [23929, 664],
        to: [24041, 501],
    },
    ChromeTextSegment::Quad {
        control: [24151, 339],
        to: [24274, 307],
    },
    ChromeTextSegment::Line([24331, 369]),
];

const VERSION_CONTOUR_74: [ChromeTextSegment; 6] = [
    ChromeTextSegment::Quad {
        control: [24052, 1096],
        to: [24203, 1096],
    },
    ChromeTextSegment::Quad {
        control: [24267, 1096],
        to: [24303, 1048],
    },
    ChromeTextSegment::Quad {
        control: [24339, 1000],
        to: [24339, 921],
    },
    ChromeTextSegment::Quad {
        control: [24339, 840],
        to: [24301, 791],
    },
    ChromeTextSegment::Quad {
        control: [24262, 743],
        to: [24201, 743],
    },
    ChromeTextSegment::Quad {
        control: [24052, 743],
        to: [24052, 912],
    },
];

const VERSION_CONTOURS: [ChromeTextContour; 75] = [
    ChromeTextContour {
        start: [-9529, 551],
        segments: &VERSION_CONTOUR_0,
    },
    ChromeTextContour {
        start: [-9182, 635],
        segments: &VERSION_CONTOUR_1,
    },
    ChromeTextContour {
        start: [-9304, 1046],
        segments: &VERSION_CONTOUR_2,
    },
    ChromeTextContour {
        start: [-8444, 660],
        segments: &VERSION_CONTOUR_3,
    },
    ChromeTextContour {
        start: [-7982, 585],
        segments: &VERSION_CONTOUR_4,
    },
    ChromeTextContour {
        start: [-7720, 315],
        segments: &VERSION_CONTOUR_5,
    },
    ChromeTextContour {
        start: [-7670, 553],
        segments: &VERSION_CONTOUR_6,
    },
    ChromeTextContour {
        start: [-7419, 865],
        segments: &VERSION_CONTOUR_7,
    },
    ChromeTextContour {
        start: [-7458, 630],
        segments: &VERSION_CONTOUR_8,
    },
    ChromeTextContour {
        start: [-6353, 792],
        segments: &VERSION_CONTOUR_9,
    },
    ChromeTextContour {
        start: [-5506, 319],
        segments: &VERSION_CONTOUR_10,
    },
    ChromeTextContour {
        start: [-5078, 1016],
        segments: &VERSION_CONTOUR_11,
    },
    ChromeTextContour {
        start: [-4393, 367],
        segments: &VERSION_CONTOUR_12,
    },
    ChromeTextContour {
        start: [-3763, 367],
        segments: &VERSION_CONTOUR_13,
    },
    ChromeTextContour {
        start: [-3131, 884],
        segments: &VERSION_CONTOUR_14,
    },
    ChromeTextContour {
        start: [-2063, 316],
        segments: &VERSION_CONTOUR_15,
    },
    ChromeTextContour {
        start: [-1516, 561],
        segments: &VERSION_CONTOUR_16,
    },
    ChromeTextContour {
        start: [-1028, 853],
        segments: &VERSION_CONTOUR_17,
    },
    ChromeTextContour {
        start: [-399, 295],
        segments: &VERSION_CONTOUR_18,
    },
    ChromeTextContour {
        start: [-399, 684],
        segments: &VERSION_CONTOUR_19,
    },
    ChromeTextContour {
        start: [665, 553],
        segments: &VERSION_CONTOUR_20,
    },
    ChromeTextContour {
        start: [1300, 567],
        segments: &VERSION_CONTOUR_21,
    },
    ChromeTextContour {
        start: [1237, 1016],
        segments: &VERSION_CONTOUR_22,
    },
    ChromeTextContour {
        start: [2047, 792],
        segments: &VERSION_CONTOUR_23,
    },
    ChromeTextContour {
        start: [2462, 635],
        segments: &VERSION_CONTOUR_24,
    },
    ChromeTextContour {
        start: [2341, 1046],
        segments: &VERSION_CONTOUR_25,
    },
    ChromeTextContour {
        start: [2936, 865],
        segments: &VERSION_CONTOUR_26,
    },
    ChromeTextContour {
        start: [2897, 630],
        segments: &VERSION_CONTOUR_27,
    },
    ChromeTextContour {
        start: [3785, 791],
        segments: &VERSION_CONTOUR_28,
    },
    ChromeTextContour {
        start: [4186, 865],
        segments: &VERSION_CONTOUR_29,
    },
    ChromeTextContour {
        start: [4147, 630],
        segments: &VERSION_CONTOUR_30,
    },
    ChromeTextContour {
        start: [4980, 1092],
        segments: &VERSION_CONTOUR_31,
    },
    ChromeTextContour {
        start: [5186, 865],
        segments: &VERSION_CONTOUR_32,
    },
    ChromeTextContour {
        start: [5147, 630],
        segments: &VERSION_CONTOUR_33,
    },
    ChromeTextContour {
        start: [6096, 660],
        segments: &VERSION_CONTOUR_34,
    },
    ChromeTextContour {
        start: [6760, 407],
        segments: &VERSION_CONTOUR_35,
    },
    ChromeTextContour {
        start: [7451, 407],
        segments: &VERSION_CONTOUR_36,
    },
    ChromeTextContour {
        start: [7453, 307],
        segments: &VERSION_CONTOUR_37,
    },
    ChromeTextContour {
        start: [8081, 407],
        segments: &VERSION_CONTOUR_38,
    },
    ChromeTextContour {
        start: [8083, 307],
        segments: &VERSION_CONTOUR_39,
    },
    ChromeTextContour {
        start: [8799, 319],
        segments: &VERSION_CONTOUR_40,
    },
    ChromeTextContour {
        start: [9114, 884],
        segments: &VERSION_CONTOUR_41,
    },
    ChromeTextContour {
        start: [9720, 407],
        segments: &VERSION_CONTOUR_42,
    },
    ChromeTextContour {
        start: [10724, 541],
        segments: &VERSION_CONTOUR_43,
    },
    ChromeTextContour {
        start: [10799, 860],
        segments: &VERSION_CONTOUR_44,
    },
    ChromeTextContour {
        start: [11657, 792],
        segments: &VERSION_CONTOUR_45,
    },
    ChromeTextContour {
        start: [12212, 295],
        segments: &VERSION_CONTOUR_46,
    },
    ChromeTextContour {
        start: [12166, 1082],
        segments: &VERSION_CONTOUR_47,
    },
    ChromeTextContour {
        start: [12942, 588],
        segments: &VERSION_CONTOUR_48,
    },
    ChromeTextContour {
        start: [13535, 315],
        segments: &VERSION_CONTOUR_49,
    },
    ChromeTextContour {
        start: [13585, 553],
        segments: &VERSION_CONTOUR_50,
    },
    ChromeTextContour {
        start: [14101, 660],
        segments: &VERSION_CONTOUR_51,
    },
    ChromeTextContour {
        start: [14731, 322],
        segments: &VERSION_CONTOUR_52,
    },
    ChromeTextContour {
        start: [15258, 585],
        segments: &VERSION_CONTOUR_53,
    },
    ChromeTextContour {
        start: [15599, 541],
        segments: &VERSION_CONTOUR_54,
    },
    ChromeTextContour {
        start: [15674, 860],
        segments: &VERSION_CONTOUR_55,
    },
    ChromeTextContour {
        start: [16229, 541],
        segments: &VERSION_CONTOUR_56,
    },
    ChromeTextContour {
        start: [16304, 860],
        segments: &VERSION_CONTOUR_57,
    },
    ChromeTextContour {
        start: [17054, 561],
        segments: &VERSION_CONTOUR_58,
    },
    ChromeTextContour {
        start: [18219, 322],
        segments: &VERSION_CONTOUR_59,
    },
    ChromeTextContour {
        start: [18643, 635],
        segments: &VERSION_CONTOUR_60,
    },
    ChromeTextContour {
        start: [18521, 1046],
        segments: &VERSION_CONTOUR_61,
    },
    ChromeTextContour {
        start: [19845, 551],
        segments: &VERSION_CONTOUR_62,
    },
    ChromeTextContour {
        start: [19975, 553],
        segments: &VERSION_CONTOUR_63,
    },
    ChromeTextContour {
        start: [20486, 865],
        segments: &VERSION_CONTOUR_64,
    },
    ChromeTextContour {
        start: [20447, 630],
        segments: &VERSION_CONTOUR_65,
    },
    ChromeTextContour {
        start: [21552, 792],
        segments: &VERSION_CONTOUR_66,
    },
    ChromeTextContour {
        start: [22399, 319],
        segments: &VERSION_CONTOUR_67,
    },
    ChromeTextContour {
        start: [23071, 369],
        segments: &VERSION_CONTOUR_68,
    },
    ChromeTextContour {
        start: [22792, 912],
        segments: &VERSION_CONTOUR_69,
    },
    ChromeTextContour {
        start: [23798, 514],
        segments: &VERSION_CONTOUR_70,
    },
    ChromeTextContour {
        start: [23450, 512],
        segments: &VERSION_CONTOUR_71,
    },
    ChromeTextContour {
        start: [23549, 753],
        segments: &VERSION_CONTOUR_72,
    },
    ChromeTextContour {
        start: [24331, 369],
        segments: &VERSION_CONTOUR_73,
    },
    ChromeTextContour {
        start: [24052, 912],
        segments: &VERSION_CONTOUR_74,
    },
];

pub const VERSION: ChromeTextDefinition = ChromeTextDefinition {
    text: "version 1.33 - (c) by :neokolor 2001-2 and Sir Isaac Newton 1686",
    define_text_id: 73,
    font_ids: &VERSION_FONT_IDS,
    color_rgb: [140, 140, 176],
    bounds_centipx: [-10105, 25035, 295, 1430],
    contours: &VERSION_CONTOURS,
};

const SPONSOR_UP_FONT_IDS: [u16; 1] = [54];

const SPONSOR_UP_CONTOUR_0: [ChromeTextSegment; 18] = [
    ChromeTextSegment::Line([102, 425]),
    ChromeTextSegment::Line([248, 371]),
    ChromeTextSegment::Line([248, 553]),
    ChromeTextSegment::Line([421, 553]),
    ChromeTextSegment::Line([421, 670]),
    ChromeTextSegment::Line([248, 670]),
    ChromeTextSegment::Line([248, 946]),
    ChromeTextSegment::Quad {
        control: [248, 1012],
        to: [270, 1041],
    },
    ChromeTextSegment::Quad {
        control: [291, 1069],
        to: [343, 1069],
    },
    ChromeTextSegment::Quad {
        control: [396, 1069],
        to: [442, 1039],
    },
    ChromeTextSegment::Line([442, 1174]),
    ChromeTextSegment::Quad {
        control: [390, 1192],
        to: [296, 1192],
    },
    ChromeTextSegment::Quad {
        control: [203, 1192],
        to: [152, 1139],
    },
    ChromeTextSegment::Quad {
        control: [102, 1086],
        to: [102, 988],
    },
    ChromeTextSegment::Line([102, 670]),
    ChromeTextSegment::Line([29, 670]),
    ChromeTextSegment::Line([29, 553]),
    ChromeTextSegment::Line([102, 553]),
];

const SPONSOR_UP_CONTOUR_1: [ChromeTextSegment; 6] = [
    ChromeTextSegment::Quad {
        control: [666, 1072],
        to: [815, 1072],
    },
    ChromeTextSegment::Quad {
        control: [884, 1072],
        to: [924, 1018],
    },
    ChromeTextSegment::Quad {
        control: [964, 964],
        to: [964, 865],
    },
    ChromeTextSegment::Quad {
        control: [964, 661],
        to: [815, 661],
    },
    ChromeTextSegment::Quad {
        control: [747, 661],
        to: [706, 715],
    },
    ChromeTextSegment::Quad {
        control: [666, 769],
        to: [666, 865],
    },
];

const SPONSOR_UP_CONTOUR_2: [ChromeTextSegment; 8] = [
    ChromeTextSegment::Quad {
        control: [679, 541],
        to: [815, 541],
    },
    ChromeTextSegment::Quad {
        control: [958, 541],
        to: [1038, 628],
    },
    ChromeTextSegment::Quad {
        control: [1116, 715],
        to: [1116, 865],
    },
    ChromeTextSegment::Quad {
        control: [1116, 1015],
        to: [1036, 1104],
    },
    ChromeTextSegment::Quad {
        control: [955, 1192],
        to: [815, 1192],
    },
    ChromeTextSegment::Quad {
        control: [672, 1192],
        to: [593, 1103],
    },
    ChromeTextSegment::Quad {
        control: [514, 1014],
        to: [514, 865],
    },
    ChromeTextSegment::Quad {
        control: [514, 722],
        to: [597, 632],
    },
];

const SPONSOR_UP_CONTOUR_3: [ChromeTextSegment; 15] = [
    ChromeTextSegment::Line([1727, 612]),
    ChromeTextSegment::Quad {
        control: [1787, 541],
        to: [1903, 541],
    },
    ChromeTextSegment::Quad {
        control: [2015, 541],
        to: [2080, 608],
    },
    ChromeTextSegment::Quad {
        control: [2144, 675],
        to: [2144, 796],
    },
    ChromeTextSegment::Line([2144, 1180]),
    ChromeTextSegment::Line([1998, 1180]),
    ChromeTextSegment::Line([1998, 818]),
    ChromeTextSegment::Quad {
        control: [1998, 737],
        to: [1967, 701],
    },
    ChromeTextSegment::Quad {
        control: [1937, 663],
        to: [1867, 663],
    },
    ChromeTextSegment::Quad {
        control: [1835, 663],
        to: [1799, 682],
    },
    ChromeTextSegment::Quad {
        control: [1761, 700],
        to: [1741, 726],
    },
    ChromeTextSegment::Line([1741, 1180]),
    ChromeTextSegment::Line([1595, 1180]),
    ChromeTextSegment::Line([1595, 553]),
    ChromeTextSegment::Line([1700, 553]),
];

const SPONSOR_UP_CONTOUR_4: [ChromeTextSegment; 3] = [
    ChromeTextSegment::Line([2730, 801]),
    ChromeTextSegment::Quad {
        control: [2715, 663],
        to: [2578, 663],
    },
    ChromeTextSegment::Quad {
        control: [2452, 663],
        to: [2422, 801],
    },
];

const SPONSOR_UP_CONTOUR_5: [ChromeTextSegment; 13] = [
    ChromeTextSegment::Quad {
        control: [2516, 1070],
        to: [2595, 1070],
    },
    ChromeTextSegment::Quad {
        control: [2696, 1070],
        to: [2748, 1017],
    },
    ChromeTextSegment::Line([2805, 1130]),
    ChromeTextSegment::Quad {
        control: [2728, 1192],
        to: [2574, 1192],
    },
    ChromeTextSegment::Quad {
        control: [2430, 1192],
        to: [2347, 1109],
    },
    ChromeTextSegment::Quad {
        control: [2264, 1024],
        to: [2264, 873],
    },
    ChromeTextSegment::Quad {
        control: [2264, 725],
        to: [2355, 633],
    },
    ChromeTextSegment::Quad {
        control: [2446, 541],
        to: [2575, 541],
    },
    ChromeTextSegment::Quad {
        control: [2711, 541],
        to: [2795, 622],
    },
    ChromeTextSegment::Quad {
        control: [2877, 704],
        to: [2877, 830],
    },
    ChromeTextSegment::Line([2865, 912]),
    ChromeTextSegment::Line([2416, 912]),
    ChromeTextSegment::Quad {
        control: [2421, 987],
        to: [2468, 1028],
    },
];

const SPONSOR_UP_CONTOUR_6: [ChromeTextSegment; 6] = [
    ChromeTextSegment::Quad {
        control: [3106, 1072],
        to: [3255, 1072],
    },
    ChromeTextSegment::Quad {
        control: [3324, 1072],
        to: [3364, 1018],
    },
    ChromeTextSegment::Quad {
        control: [3404, 964],
        to: [3404, 865],
    },
    ChromeTextSegment::Quad {
        control: [3404, 661],
        to: [3255, 661],
    },
    ChromeTextSegment::Quad {
        control: [3187, 661],
        to: [3146, 715],
    },
    ChromeTextSegment::Quad {
        control: [3106, 769],
        to: [3106, 865],
    },
];

const SPONSOR_UP_CONTOUR_7: [ChromeTextSegment; 8] = [
    ChromeTextSegment::Quad {
        control: [3119, 541],
        to: [3255, 541],
    },
    ChromeTextSegment::Quad {
        control: [3398, 541],
        to: [3478, 628],
    },
    ChromeTextSegment::Quad {
        control: [3556, 715],
        to: [3556, 865],
    },
    ChromeTextSegment::Quad {
        control: [3556, 1015],
        to: [3476, 1104],
    },
    ChromeTextSegment::Quad {
        control: [3395, 1192],
        to: [3255, 1192],
    },
    ChromeTextSegment::Quad {
        control: [3112, 1192],
        to: [3033, 1103],
    },
    ChromeTextSegment::Quad {
        control: [2954, 1014],
        to: [2954, 865],
    },
    ChromeTextSegment::Quad {
        control: [2954, 722],
        to: [3037, 632],
    },
];

const SPONSOR_UP_CONTOUR_8: [ChromeTextSegment; 12] = [
    ChromeTextSegment::Line([4224, 284]),
    ChromeTextSegment::Line([4224, 1180]),
    ChromeTextSegment::Line([4078, 1180]),
    ChromeTextSegment::Line([4078, 1142]),
    ChromeTextSegment::Quad {
        control: [4060, 1162],
        to: [4017, 1178],
    },
    ChromeTextSegment::Quad {
        control: [3974, 1192],
        to: [3927, 1192],
    },
    ChromeTextSegment::Quad {
        control: [3795, 1192],
        to: [3720, 1109],
    },
    ChromeTextSegment::Quad {
        control: [3645, 1025],
        to: [3645, 876],
    },
    ChromeTextSegment::Quad {
        control: [3645, 728],
        to: [3732, 635],
    },
    ChromeTextSegment::Quad {
        control: [3818, 541],
        to: [3948, 541],
    },
    ChromeTextSegment::Quad {
        control: [4019, 541],
        to: [4078, 571],
    },
    ChromeTextSegment::Line([4078, 319]),
];

const SPONSOR_UP_CONTOUR_9: [ChromeTextSegment; 7] = [
    ChromeTextSegment::Quad {
        control: [4068, 1043],
        to: [4078, 1030],
    },
    ChromeTextSegment::Line([4078, 703]),
    ChromeTextSegment::Quad {
        control: [4031, 666],
        to: [3981, 666],
    },
    ChromeTextSegment::Quad {
        control: [3893, 666],
        to: [3845, 719],
    },
    ChromeTextSegment::Quad {
        control: [3798, 772],
        to: [3798, 873],
    },
    ChromeTextSegment::Quad {
        control: [3798, 1069],
        to: [3986, 1069],
    },
    ChromeTextSegment::Quad {
        control: [4007, 1069],
        to: [4038, 1056],
    },
];

const SPONSOR_UP_CONTOUR_10: [ChromeTextSegment; 3] = [
    ChromeTextSegment::Line([4795, 801]),
    ChromeTextSegment::Quad {
        control: [4780, 663],
        to: [4643, 663],
    },
    ChromeTextSegment::Quad {
        control: [4517, 663],
        to: [4487, 801],
    },
];

const SPONSOR_UP_CONTOUR_11: [ChromeTextSegment; 13] = [
    ChromeTextSegment::Quad {
        control: [4581, 1070],
        to: [4660, 1070],
    },
    ChromeTextSegment::Quad {
        control: [4761, 1070],
        to: [4813, 1017],
    },
    ChromeTextSegment::Line([4870, 1130]),
    ChromeTextSegment::Quad {
        control: [4793, 1192],
        to: [4639, 1192],
    },
    ChromeTextSegment::Quad {
        control: [4495, 1192],
        to: [4412, 1109],
    },
    ChromeTextSegment::Quad {
        control: [4329, 1024],
        to: [4329, 873],
    },
    ChromeTextSegment::Quad {
        control: [4329, 725],
        to: [4420, 633],
    },
    ChromeTextSegment::Quad {
        control: [4511, 541],
        to: [4640, 541],
    },
    ChromeTextSegment::Quad {
        control: [4776, 541],
        to: [4860, 622],
    },
    ChromeTextSegment::Quad {
        control: [4942, 704],
        to: [4942, 830],
    },
    ChromeTextSegment::Line([4930, 912]),
    ChromeTextSegment::Line([4481, 912]),
    ChromeTextSegment::Quad {
        control: [4486, 987],
        to: [4533, 1028],
    },
];

const SPONSOR_UP_CONTOUR_12: [ChromeTextSegment; 6] = [
    ChromeTextSegment::Line([5231, 284]),
    ChromeTextSegment::Line([5231, 989]),
    ChromeTextSegment::Quad {
        control: [5231, 1105],
        to: [5300, 1127],
    },
    ChromeTextSegment::Quad {
        control: [5266, 1192],
        to: [5184, 1192],
    },
    ChromeTextSegment::Quad {
        control: [5084, 1192],
        to: [5084, 1053],
    },
    ChromeTextSegment::Line([5084, 319]),
];

const SPONSOR_UP_CONTOUR_13: [ChromeTextSegment; 8] = [
    ChromeTextSegment::Quad {
        control: [5494, 309],
        to: [5530, 309],
    },
    ChromeTextSegment::Quad {
        control: [5565, 309],
        to: [5589, 335],
    },
    ChromeTextSegment::Quad {
        control: [5614, 360],
        to: [5614, 395],
    },
    ChromeTextSegment::Quad {
        control: [5614, 430],
        to: [5589, 455],
    },
    ChromeTextSegment::Quad {
        control: [5565, 479],
        to: [5530, 479],
    },
    ChromeTextSegment::Quad {
        control: [5494, 479],
        to: [5470, 455],
    },
    ChromeTextSegment::Quad {
        control: [5444, 430],
        to: [5444, 395],
    },
    ChromeTextSegment::Quad {
        control: [5444, 360],
        to: [5470, 335],
    },
];

const SPONSOR_UP_CONTOUR_14: [ChromeTextSegment; 6] = [
    ChromeTextSegment::Line([5602, 1180]),
    ChromeTextSegment::Line([5453, 1180]),
    ChromeTextSegment::Line([5453, 673]),
    ChromeTextSegment::Line([5374, 673]),
    ChromeTextSegment::Line([5374, 553]),
    ChromeTextSegment::Line([5602, 553]),
];

const SPONSOR_UP_CONTOUR_15: [ChromeTextSegment; 31] = [
    ChromeTextSegment::Line([6211, 648]),
    ChromeTextSegment::Quad {
        control: [6247, 703],
        to: [6247, 778],
    },
    ChromeTextSegment::Quad {
        control: [6247, 885],
        to: [6181, 949],
    },
    ChromeTextSegment::Quad {
        control: [6116, 1014],
        to: [6016, 1014],
    },
    ChromeTextSegment::Line([5974, 1010]),
    ChromeTextSegment::Line([5950, 1007]),
    ChromeTextSegment::Line([5919, 1019]),
    ChromeTextSegment::Quad {
        control: [5892, 1032],
        to: [5892, 1045],
    },
    ChromeTextSegment::Quad {
        control: [5892, 1069],
        to: [5932, 1069],
    },
    ChromeTextSegment::Line([5993, 1060]),
    ChromeTextSegment::Line([6066, 1051]),
    ChromeTextSegment::Quad {
        control: [6280, 1051],
        to: [6280, 1223],
    },
    ChromeTextSegment::Quad {
        control: [6280, 1318],
        to: [6194, 1372],
    },
    ChromeTextSegment::Quad {
        control: [6109, 1426],
        to: [5988, 1426],
    },
    ChromeTextSegment::Quad {
        control: [5844, 1426],
        to: [5728, 1342],
    },
    ChromeTextSegment::Line([5820, 1227]),
    ChromeTextSegment::Quad {
        control: [5897, 1297],
        to: [5991, 1297],
    },
    ChromeTextSegment::Quad {
        control: [6055, 1297],
        to: [6096, 1278],
    },
    ChromeTextSegment::Quad {
        control: [6137, 1260],
        to: [6137, 1227],
    },
    ChromeTextSegment::Quad {
        control: [6137, 1172],
        to: [6045, 1172],
    },
    ChromeTextSegment::Line([5973, 1178]),
    ChromeTextSegment::Line([5899, 1184]),
    ChromeTextSegment::Quad {
        control: [5748, 1184],
        to: [5748, 1075],
    },
    ChromeTextSegment::Quad {
        control: [5748, 1044],
        to: [5774, 1014],
    },
    ChromeTextSegment::Quad {
        control: [5798, 984],
        to: [5832, 970],
    },
    ChromeTextSegment::Quad {
        control: [5724, 900],
        to: [5724, 772],
    },
    ChromeTextSegment::Quad {
        control: [5724, 671],
        to: [5798, 606],
    },
    ChromeTextSegment::Quad {
        control: [5872, 540],
        to: [5980, 540],
    },
    ChromeTextSegment::Quad {
        control: [6064, 540],
        to: [6122, 572],
    },
    ChromeTextSegment::Line([6179, 505]),
    ChromeTextSegment::Line([6280, 596]),
];

const SPONSOR_UP_CONTOUR_16: [ChromeTextSegment; 8] = [
    ChromeTextSegment::Quad {
        control: [5938, 660],
        to: [5906, 691],
    },
    ChromeTextSegment::Quad {
        control: [5874, 723],
        to: [5874, 773],
    },
    ChromeTextSegment::Quad {
        control: [5874, 828],
        to: [5905, 862],
    },
    ChromeTextSegment::Quad {
        control: [5935, 896],
        to: [5988, 896],
    },
    ChromeTextSegment::Quad {
        control: [6040, 896],
        to: [6068, 864],
    },
    ChromeTextSegment::Quad {
        control: [6096, 831],
        to: [6096, 773],
    },
    ChromeTextSegment::Quad {
        control: [6096, 725],
        to: [6066, 693],
    },
    ChromeTextSegment::Quad {
        control: [6036, 660],
        to: [5988, 660],
    },
];

const SPONSOR_UP_CONTOUR_17: [ChromeTextSegment; 15] = [
    ChromeTextSegment::Line([6524, 593]),
    ChromeTextSegment::Quad {
        control: [6580, 541],
        to: [6679, 541],
    },
    ChromeTextSegment::Quad {
        control: [6798, 541],
        to: [6862, 607],
    },
    ChromeTextSegment::Quad {
        control: [6928, 671],
        to: [6928, 793],
    },
    ChromeTextSegment::Line([6928, 1180]),
    ChromeTextSegment::Line([6780, 1180]),
    ChromeTextSegment::Line([6780, 793]),
    ChromeTextSegment::Quad {
        control: [6780, 736],
        to: [6744, 701],
    },
    ChromeTextSegment::Quad {
        control: [6707, 666],
        to: [6649, 666],
    },
    ChromeTextSegment::Quad {
        control: [6613, 666],
        to: [6576, 685],
    },
    ChromeTextSegment::Quad {
        control: [6539, 704],
        to: [6524, 730],
    },
    ChromeTextSegment::Line([6524, 1180]),
    ChromeTextSegment::Line([6375, 1180]),
    ChromeTextSegment::Line([6375, 319]),
    ChromeTextSegment::Line([6524, 284]),
];

const SPONSOR_UP_CONTOUR_18: [ChromeTextSegment; 18] = [
    ChromeTextSegment::Line([7107, 425]),
    ChromeTextSegment::Line([7253, 371]),
    ChromeTextSegment::Line([7253, 553]),
    ChromeTextSegment::Line([7426, 553]),
    ChromeTextSegment::Line([7426, 670]),
    ChromeTextSegment::Line([7253, 670]),
    ChromeTextSegment::Line([7253, 946]),
    ChromeTextSegment::Quad {
        control: [7253, 1012],
        to: [7275, 1041],
    },
    ChromeTextSegment::Quad {
        control: [7296, 1069],
        to: [7348, 1069],
    },
    ChromeTextSegment::Quad {
        control: [7401, 1069],
        to: [7447, 1039],
    },
    ChromeTextSegment::Line([7447, 1174]),
    ChromeTextSegment::Quad {
        control: [7395, 1192],
        to: [7301, 1192],
    },
    ChromeTextSegment::Quad {
        control: [7208, 1192],
        to: [7157, 1139],
    },
    ChromeTextSegment::Quad {
        control: [7107, 1086],
        to: [7107, 988],
    },
    ChromeTextSegment::Line([7107, 670]),
    ChromeTextSegment::Line([7034, 670]),
    ChromeTextSegment::Line([7034, 553]),
    ChromeTextSegment::Line([7107, 553]),
];

const SPONSOR_UP_CONTOURS: [ChromeTextContour; 19] = [
    ChromeTextContour {
        start: [102, 553],
        segments: &SPONSOR_UP_CONTOUR_0,
    },
    ChromeTextContour {
        start: [666, 865],
        segments: &SPONSOR_UP_CONTOUR_1,
    },
    ChromeTextContour {
        start: [597, 632],
        segments: &SPONSOR_UP_CONTOUR_2,
    },
    ChromeTextContour {
        start: [1700, 553],
        segments: &SPONSOR_UP_CONTOUR_3,
    },
    ChromeTextContour {
        start: [2422, 801],
        segments: &SPONSOR_UP_CONTOUR_4,
    },
    ChromeTextContour {
        start: [2468, 1028],
        segments: &SPONSOR_UP_CONTOUR_5,
    },
    ChromeTextContour {
        start: [3106, 865],
        segments: &SPONSOR_UP_CONTOUR_6,
    },
    ChromeTextContour {
        start: [3037, 632],
        segments: &SPONSOR_UP_CONTOUR_7,
    },
    ChromeTextContour {
        start: [4078, 319],
        segments: &SPONSOR_UP_CONTOUR_8,
    },
    ChromeTextContour {
        start: [4038, 1056],
        segments: &SPONSOR_UP_CONTOUR_9,
    },
    ChromeTextContour {
        start: [4487, 801],
        segments: &SPONSOR_UP_CONTOUR_10,
    },
    ChromeTextContour {
        start: [4533, 1028],
        segments: &SPONSOR_UP_CONTOUR_11,
    },
    ChromeTextContour {
        start: [5084, 319],
        segments: &SPONSOR_UP_CONTOUR_12,
    },
    ChromeTextContour {
        start: [5470, 335],
        segments: &SPONSOR_UP_CONTOUR_13,
    },
    ChromeTextContour {
        start: [5602, 553],
        segments: &SPONSOR_UP_CONTOUR_14,
    },
    ChromeTextContour {
        start: [6280, 596],
        segments: &SPONSOR_UP_CONTOUR_15,
    },
    ChromeTextContour {
        start: [5988, 660],
        segments: &SPONSOR_UP_CONTOUR_16,
    },
    ChromeTextContour {
        start: [6524, 284],
        segments: &SPONSOR_UP_CONTOUR_17,
    },
    ChromeTextContour {
        start: [7107, 553],
        segments: &SPONSOR_UP_CONTOUR_18,
    },
];

pub const SPONSOR_UP: ChromeTextDefinition = ChromeTextDefinition {
    text: "to neodelight",
    define_text_id: 95,
    font_ids: &SPONSOR_UP_FONT_IDS,
    color_rgb: [0, 0, 0],
    bounds_centipx: [30, 8005, 285, 1425],
    contours: &SPONSOR_UP_CONTOURS,
};

const SPONSOR_OVER_FONT_IDS: [u16; 1] = [54];

const SPONSOR_OVER_CONTOUR_0: [ChromeTextSegment; 18] = [
    ChromeTextSegment::Line([102, 425]),
    ChromeTextSegment::Line([248, 371]),
    ChromeTextSegment::Line([248, 553]),
    ChromeTextSegment::Line([421, 553]),
    ChromeTextSegment::Line([421, 670]),
    ChromeTextSegment::Line([248, 670]),
    ChromeTextSegment::Line([248, 946]),
    ChromeTextSegment::Quad {
        control: [248, 1012],
        to: [270, 1041],
    },
    ChromeTextSegment::Quad {
        control: [291, 1069],
        to: [343, 1069],
    },
    ChromeTextSegment::Quad {
        control: [396, 1069],
        to: [442, 1039],
    },
    ChromeTextSegment::Line([442, 1174]),
    ChromeTextSegment::Quad {
        control: [390, 1192],
        to: [296, 1192],
    },
    ChromeTextSegment::Quad {
        control: [203, 1192],
        to: [152, 1139],
    },
    ChromeTextSegment::Quad {
        control: [102, 1086],
        to: [102, 988],
    },
    ChromeTextSegment::Line([102, 670]),
    ChromeTextSegment::Line([29, 670]),
    ChromeTextSegment::Line([29, 553]),
    ChromeTextSegment::Line([102, 553]),
];

const SPONSOR_OVER_CONTOUR_1: [ChromeTextSegment; 6] = [
    ChromeTextSegment::Quad {
        control: [666, 1072],
        to: [815, 1072],
    },
    ChromeTextSegment::Quad {
        control: [884, 1072],
        to: [924, 1018],
    },
    ChromeTextSegment::Quad {
        control: [964, 964],
        to: [964, 865],
    },
    ChromeTextSegment::Quad {
        control: [964, 661],
        to: [815, 661],
    },
    ChromeTextSegment::Quad {
        control: [747, 661],
        to: [706, 715],
    },
    ChromeTextSegment::Quad {
        control: [666, 769],
        to: [666, 865],
    },
];

const SPONSOR_OVER_CONTOUR_2: [ChromeTextSegment; 8] = [
    ChromeTextSegment::Quad {
        control: [679, 541],
        to: [815, 541],
    },
    ChromeTextSegment::Quad {
        control: [958, 541],
        to: [1038, 628],
    },
    ChromeTextSegment::Quad {
        control: [1116, 715],
        to: [1116, 865],
    },
    ChromeTextSegment::Quad {
        control: [1116, 1015],
        to: [1036, 1104],
    },
    ChromeTextSegment::Quad {
        control: [955, 1192],
        to: [815, 1192],
    },
    ChromeTextSegment::Quad {
        control: [672, 1192],
        to: [593, 1103],
    },
    ChromeTextSegment::Quad {
        control: [514, 1014],
        to: [514, 865],
    },
    ChromeTextSegment::Quad {
        control: [514, 722],
        to: [597, 632],
    },
];

const SPONSOR_OVER_CONTOUR_3: [ChromeTextSegment; 15] = [
    ChromeTextSegment::Line([1727, 612]),
    ChromeTextSegment::Quad {
        control: [1787, 541],
        to: [1903, 541],
    },
    ChromeTextSegment::Quad {
        control: [2015, 541],
        to: [2080, 608],
    },
    ChromeTextSegment::Quad {
        control: [2144, 675],
        to: [2144, 796],
    },
    ChromeTextSegment::Line([2144, 1180]),
    ChromeTextSegment::Line([1998, 1180]),
    ChromeTextSegment::Line([1998, 818]),
    ChromeTextSegment::Quad {
        control: [1998, 737],
        to: [1967, 701],
    },
    ChromeTextSegment::Quad {
        control: [1937, 663],
        to: [1867, 663],
    },
    ChromeTextSegment::Quad {
        control: [1835, 663],
        to: [1799, 682],
    },
    ChromeTextSegment::Quad {
        control: [1761, 700],
        to: [1741, 726],
    },
    ChromeTextSegment::Line([1741, 1180]),
    ChromeTextSegment::Line([1595, 1180]),
    ChromeTextSegment::Line([1595, 553]),
    ChromeTextSegment::Line([1700, 553]),
];

const SPONSOR_OVER_CONTOUR_4: [ChromeTextSegment; 3] = [
    ChromeTextSegment::Line([2730, 801]),
    ChromeTextSegment::Quad {
        control: [2715, 663],
        to: [2578, 663],
    },
    ChromeTextSegment::Quad {
        control: [2452, 663],
        to: [2422, 801],
    },
];

const SPONSOR_OVER_CONTOUR_5: [ChromeTextSegment; 13] = [
    ChromeTextSegment::Quad {
        control: [2516, 1070],
        to: [2595, 1070],
    },
    ChromeTextSegment::Quad {
        control: [2696, 1070],
        to: [2748, 1017],
    },
    ChromeTextSegment::Line([2805, 1130]),
    ChromeTextSegment::Quad {
        control: [2728, 1192],
        to: [2574, 1192],
    },
    ChromeTextSegment::Quad {
        control: [2430, 1192],
        to: [2347, 1109],
    },
    ChromeTextSegment::Quad {
        control: [2264, 1024],
        to: [2264, 873],
    },
    ChromeTextSegment::Quad {
        control: [2264, 725],
        to: [2355, 633],
    },
    ChromeTextSegment::Quad {
        control: [2446, 541],
        to: [2575, 541],
    },
    ChromeTextSegment::Quad {
        control: [2711, 541],
        to: [2795, 622],
    },
    ChromeTextSegment::Quad {
        control: [2877, 704],
        to: [2877, 830],
    },
    ChromeTextSegment::Line([2865, 912]),
    ChromeTextSegment::Line([2416, 912]),
    ChromeTextSegment::Quad {
        control: [2421, 987],
        to: [2468, 1028],
    },
];

const SPONSOR_OVER_CONTOUR_6: [ChromeTextSegment; 6] = [
    ChromeTextSegment::Quad {
        control: [3106, 1072],
        to: [3255, 1072],
    },
    ChromeTextSegment::Quad {
        control: [3324, 1072],
        to: [3364, 1018],
    },
    ChromeTextSegment::Quad {
        control: [3404, 964],
        to: [3404, 865],
    },
    ChromeTextSegment::Quad {
        control: [3404, 661],
        to: [3255, 661],
    },
    ChromeTextSegment::Quad {
        control: [3187, 661],
        to: [3146, 715],
    },
    ChromeTextSegment::Quad {
        control: [3106, 769],
        to: [3106, 865],
    },
];

const SPONSOR_OVER_CONTOUR_7: [ChromeTextSegment; 8] = [
    ChromeTextSegment::Quad {
        control: [3119, 541],
        to: [3255, 541],
    },
    ChromeTextSegment::Quad {
        control: [3398, 541],
        to: [3478, 628],
    },
    ChromeTextSegment::Quad {
        control: [3556, 715],
        to: [3556, 865],
    },
    ChromeTextSegment::Quad {
        control: [3556, 1015],
        to: [3476, 1104],
    },
    ChromeTextSegment::Quad {
        control: [3395, 1192],
        to: [3255, 1192],
    },
    ChromeTextSegment::Quad {
        control: [3112, 1192],
        to: [3033, 1103],
    },
    ChromeTextSegment::Quad {
        control: [2954, 1014],
        to: [2954, 865],
    },
    ChromeTextSegment::Quad {
        control: [2954, 722],
        to: [3037, 632],
    },
];

const SPONSOR_OVER_CONTOUR_8: [ChromeTextSegment; 12] = [
    ChromeTextSegment::Line([4224, 284]),
    ChromeTextSegment::Line([4224, 1180]),
    ChromeTextSegment::Line([4078, 1180]),
    ChromeTextSegment::Line([4078, 1142]),
    ChromeTextSegment::Quad {
        control: [4060, 1162],
        to: [4017, 1178],
    },
    ChromeTextSegment::Quad {
        control: [3974, 1192],
        to: [3927, 1192],
    },
    ChromeTextSegment::Quad {
        control: [3795, 1192],
        to: [3720, 1109],
    },
    ChromeTextSegment::Quad {
        control: [3645, 1025],
        to: [3645, 876],
    },
    ChromeTextSegment::Quad {
        control: [3645, 728],
        to: [3732, 635],
    },
    ChromeTextSegment::Quad {
        control: [3818, 541],
        to: [3948, 541],
    },
    ChromeTextSegment::Quad {
        control: [4019, 541],
        to: [4078, 571],
    },
    ChromeTextSegment::Line([4078, 319]),
];

const SPONSOR_OVER_CONTOUR_9: [ChromeTextSegment; 7] = [
    ChromeTextSegment::Quad {
        control: [4068, 1043],
        to: [4078, 1030],
    },
    ChromeTextSegment::Line([4078, 703]),
    ChromeTextSegment::Quad {
        control: [4031, 666],
        to: [3981, 666],
    },
    ChromeTextSegment::Quad {
        control: [3893, 666],
        to: [3845, 719],
    },
    ChromeTextSegment::Quad {
        control: [3798, 772],
        to: [3798, 873],
    },
    ChromeTextSegment::Quad {
        control: [3798, 1069],
        to: [3986, 1069],
    },
    ChromeTextSegment::Quad {
        control: [4007, 1069],
        to: [4038, 1056],
    },
];

const SPONSOR_OVER_CONTOUR_10: [ChromeTextSegment; 3] = [
    ChromeTextSegment::Line([4795, 801]),
    ChromeTextSegment::Quad {
        control: [4780, 663],
        to: [4643, 663],
    },
    ChromeTextSegment::Quad {
        control: [4517, 663],
        to: [4487, 801],
    },
];

const SPONSOR_OVER_CONTOUR_11: [ChromeTextSegment; 13] = [
    ChromeTextSegment::Quad {
        control: [4581, 1070],
        to: [4660, 1070],
    },
    ChromeTextSegment::Quad {
        control: [4761, 1070],
        to: [4813, 1017],
    },
    ChromeTextSegment::Line([4870, 1130]),
    ChromeTextSegment::Quad {
        control: [4793, 1192],
        to: [4639, 1192],
    },
    ChromeTextSegment::Quad {
        control: [4495, 1192],
        to: [4412, 1109],
    },
    ChromeTextSegment::Quad {
        control: [4329, 1024],
        to: [4329, 873],
    },
    ChromeTextSegment::Quad {
        control: [4329, 725],
        to: [4420, 633],
    },
    ChromeTextSegment::Quad {
        control: [4511, 541],
        to: [4640, 541],
    },
    ChromeTextSegment::Quad {
        control: [4776, 541],
        to: [4860, 622],
    },
    ChromeTextSegment::Quad {
        control: [4942, 704],
        to: [4942, 830],
    },
    ChromeTextSegment::Line([4930, 912]),
    ChromeTextSegment::Line([4481, 912]),
    ChromeTextSegment::Quad {
        control: [4486, 987],
        to: [4533, 1028],
    },
];

const SPONSOR_OVER_CONTOUR_12: [ChromeTextSegment; 6] = [
    ChromeTextSegment::Line([5231, 284]),
    ChromeTextSegment::Line([5231, 989]),
    ChromeTextSegment::Quad {
        control: [5231, 1105],
        to: [5300, 1127],
    },
    ChromeTextSegment::Quad {
        control: [5266, 1192],
        to: [5184, 1192],
    },
    ChromeTextSegment::Quad {
        control: [5084, 1192],
        to: [5084, 1053],
    },
    ChromeTextSegment::Line([5084, 319]),
];

const SPONSOR_OVER_CONTOUR_13: [ChromeTextSegment; 8] = [
    ChromeTextSegment::Quad {
        control: [5494, 309],
        to: [5530, 309],
    },
    ChromeTextSegment::Quad {
        control: [5565, 309],
        to: [5589, 335],
    },
    ChromeTextSegment::Quad {
        control: [5614, 360],
        to: [5614, 395],
    },
    ChromeTextSegment::Quad {
        control: [5614, 430],
        to: [5589, 455],
    },
    ChromeTextSegment::Quad {
        control: [5565, 479],
        to: [5530, 479],
    },
    ChromeTextSegment::Quad {
        control: [5494, 479],
        to: [5470, 455],
    },
    ChromeTextSegment::Quad {
        control: [5444, 430],
        to: [5444, 395],
    },
    ChromeTextSegment::Quad {
        control: [5444, 360],
        to: [5470, 335],
    },
];

const SPONSOR_OVER_CONTOUR_14: [ChromeTextSegment; 6] = [
    ChromeTextSegment::Line([5602, 1180]),
    ChromeTextSegment::Line([5453, 1180]),
    ChromeTextSegment::Line([5453, 673]),
    ChromeTextSegment::Line([5374, 673]),
    ChromeTextSegment::Line([5374, 553]),
    ChromeTextSegment::Line([5602, 553]),
];

const SPONSOR_OVER_CONTOUR_15: [ChromeTextSegment; 31] = [
    ChromeTextSegment::Line([6211, 648]),
    ChromeTextSegment::Quad {
        control: [6247, 703],
        to: [6247, 778],
    },
    ChromeTextSegment::Quad {
        control: [6247, 885],
        to: [6181, 949],
    },
    ChromeTextSegment::Quad {
        control: [6116, 1014],
        to: [6016, 1014],
    },
    ChromeTextSegment::Line([5974, 1010]),
    ChromeTextSegment::Line([5950, 1007]),
    ChromeTextSegment::Line([5919, 1019]),
    ChromeTextSegment::Quad {
        control: [5892, 1032],
        to: [5892, 1045],
    },
    ChromeTextSegment::Quad {
        control: [5892, 1069],
        to: [5932, 1069],
    },
    ChromeTextSegment::Line([5993, 1060]),
    ChromeTextSegment::Line([6066, 1051]),
    ChromeTextSegment::Quad {
        control: [6280, 1051],
        to: [6280, 1223],
    },
    ChromeTextSegment::Quad {
        control: [6280, 1318],
        to: [6194, 1372],
    },
    ChromeTextSegment::Quad {
        control: [6109, 1426],
        to: [5988, 1426],
    },
    ChromeTextSegment::Quad {
        control: [5844, 1426],
        to: [5728, 1342],
    },
    ChromeTextSegment::Line([5820, 1227]),
    ChromeTextSegment::Quad {
        control: [5897, 1297],
        to: [5991, 1297],
    },
    ChromeTextSegment::Quad {
        control: [6055, 1297],
        to: [6096, 1278],
    },
    ChromeTextSegment::Quad {
        control: [6137, 1260],
        to: [6137, 1227],
    },
    ChromeTextSegment::Quad {
        control: [6137, 1172],
        to: [6045, 1172],
    },
    ChromeTextSegment::Line([5973, 1178]),
    ChromeTextSegment::Line([5899, 1184]),
    ChromeTextSegment::Quad {
        control: [5748, 1184],
        to: [5748, 1075],
    },
    ChromeTextSegment::Quad {
        control: [5748, 1044],
        to: [5774, 1014],
    },
    ChromeTextSegment::Quad {
        control: [5798, 984],
        to: [5832, 970],
    },
    ChromeTextSegment::Quad {
        control: [5724, 900],
        to: [5724, 772],
    },
    ChromeTextSegment::Quad {
        control: [5724, 671],
        to: [5798, 606],
    },
    ChromeTextSegment::Quad {
        control: [5872, 540],
        to: [5980, 540],
    },
    ChromeTextSegment::Quad {
        control: [6064, 540],
        to: [6122, 572],
    },
    ChromeTextSegment::Line([6179, 505]),
    ChromeTextSegment::Line([6280, 596]),
];

const SPONSOR_OVER_CONTOUR_16: [ChromeTextSegment; 8] = [
    ChromeTextSegment::Quad {
        control: [5938, 660],
        to: [5906, 691],
    },
    ChromeTextSegment::Quad {
        control: [5874, 723],
        to: [5874, 773],
    },
    ChromeTextSegment::Quad {
        control: [5874, 828],
        to: [5905, 862],
    },
    ChromeTextSegment::Quad {
        control: [5935, 896],
        to: [5988, 896],
    },
    ChromeTextSegment::Quad {
        control: [6040, 896],
        to: [6068, 864],
    },
    ChromeTextSegment::Quad {
        control: [6096, 831],
        to: [6096, 773],
    },
    ChromeTextSegment::Quad {
        control: [6096, 725],
        to: [6066, 693],
    },
    ChromeTextSegment::Quad {
        control: [6036, 660],
        to: [5988, 660],
    },
];

const SPONSOR_OVER_CONTOUR_17: [ChromeTextSegment; 15] = [
    ChromeTextSegment::Line([6524, 593]),
    ChromeTextSegment::Quad {
        control: [6580, 541],
        to: [6679, 541],
    },
    ChromeTextSegment::Quad {
        control: [6798, 541],
        to: [6862, 607],
    },
    ChromeTextSegment::Quad {
        control: [6928, 671],
        to: [6928, 793],
    },
    ChromeTextSegment::Line([6928, 1180]),
    ChromeTextSegment::Line([6780, 1180]),
    ChromeTextSegment::Line([6780, 793]),
    ChromeTextSegment::Quad {
        control: [6780, 736],
        to: [6744, 701],
    },
    ChromeTextSegment::Quad {
        control: [6707, 666],
        to: [6649, 666],
    },
    ChromeTextSegment::Quad {
        control: [6613, 666],
        to: [6576, 685],
    },
    ChromeTextSegment::Quad {
        control: [6539, 704],
        to: [6524, 730],
    },
    ChromeTextSegment::Line([6524, 1180]),
    ChromeTextSegment::Line([6375, 1180]),
    ChromeTextSegment::Line([6375, 319]),
    ChromeTextSegment::Line([6524, 284]),
];

const SPONSOR_OVER_CONTOUR_18: [ChromeTextSegment; 18] = [
    ChromeTextSegment::Line([7107, 425]),
    ChromeTextSegment::Line([7253, 371]),
    ChromeTextSegment::Line([7253, 553]),
    ChromeTextSegment::Line([7426, 553]),
    ChromeTextSegment::Line([7426, 670]),
    ChromeTextSegment::Line([7253, 670]),
    ChromeTextSegment::Line([7253, 946]),
    ChromeTextSegment::Quad {
        control: [7253, 1012],
        to: [7275, 1041],
    },
    ChromeTextSegment::Quad {
        control: [7296, 1069],
        to: [7348, 1069],
    },
    ChromeTextSegment::Quad {
        control: [7401, 1069],
        to: [7447, 1039],
    },
    ChromeTextSegment::Line([7447, 1174]),
    ChromeTextSegment::Quad {
        control: [7395, 1192],
        to: [7301, 1192],
    },
    ChromeTextSegment::Quad {
        control: [7208, 1192],
        to: [7157, 1139],
    },
    ChromeTextSegment::Quad {
        control: [7107, 1086],
        to: [7107, 988],
    },
    ChromeTextSegment::Line([7107, 670]),
    ChromeTextSegment::Line([7034, 670]),
    ChromeTextSegment::Line([7034, 553]),
    ChromeTextSegment::Line([7107, 553]),
];

const SPONSOR_OVER_CONTOURS: [ChromeTextContour; 19] = [
    ChromeTextContour {
        start: [102, 553],
        segments: &SPONSOR_OVER_CONTOUR_0,
    },
    ChromeTextContour {
        start: [666, 865],
        segments: &SPONSOR_OVER_CONTOUR_1,
    },
    ChromeTextContour {
        start: [597, 632],
        segments: &SPONSOR_OVER_CONTOUR_2,
    },
    ChromeTextContour {
        start: [1700, 553],
        segments: &SPONSOR_OVER_CONTOUR_3,
    },
    ChromeTextContour {
        start: [2422, 801],
        segments: &SPONSOR_OVER_CONTOUR_4,
    },
    ChromeTextContour {
        start: [2468, 1028],
        segments: &SPONSOR_OVER_CONTOUR_5,
    },
    ChromeTextContour {
        start: [3106, 865],
        segments: &SPONSOR_OVER_CONTOUR_6,
    },
    ChromeTextContour {
        start: [3037, 632],
        segments: &SPONSOR_OVER_CONTOUR_7,
    },
    ChromeTextContour {
        start: [4078, 319],
        segments: &SPONSOR_OVER_CONTOUR_8,
    },
    ChromeTextContour {
        start: [4038, 1056],
        segments: &SPONSOR_OVER_CONTOUR_9,
    },
    ChromeTextContour {
        start: [4487, 801],
        segments: &SPONSOR_OVER_CONTOUR_10,
    },
    ChromeTextContour {
        start: [4533, 1028],
        segments: &SPONSOR_OVER_CONTOUR_11,
    },
    ChromeTextContour {
        start: [5084, 319],
        segments: &SPONSOR_OVER_CONTOUR_12,
    },
    ChromeTextContour {
        start: [5470, 335],
        segments: &SPONSOR_OVER_CONTOUR_13,
    },
    ChromeTextContour {
        start: [5602, 553],
        segments: &SPONSOR_OVER_CONTOUR_14,
    },
    ChromeTextContour {
        start: [6280, 596],
        segments: &SPONSOR_OVER_CONTOUR_15,
    },
    ChromeTextContour {
        start: [5988, 660],
        segments: &SPONSOR_OVER_CONTOUR_16,
    },
    ChromeTextContour {
        start: [6524, 284],
        segments: &SPONSOR_OVER_CONTOUR_17,
    },
    ChromeTextContour {
        start: [7107, 553],
        segments: &SPONSOR_OVER_CONTOUR_18,
    },
];

pub const SPONSOR_OVER: ChromeTextDefinition = ChromeTextDefinition {
    text: "to neodelight",
    define_text_id: 96,
    font_ids: &SPONSOR_OVER_FONT_IDS,
    color_rgb: [204, 204, 204],
    bounds_centipx: [30, 8005, 285, 1425],
    contours: &SPONSOR_OVER_CONTOURS,
};

const SPONSOR_DOWN_FONT_IDS: [u16; 1] = [54];

const SPONSOR_DOWN_CONTOUR_0: [ChromeTextSegment; 18] = [
    ChromeTextSegment::Line([102, 425]),
    ChromeTextSegment::Line([248, 371]),
    ChromeTextSegment::Line([248, 553]),
    ChromeTextSegment::Line([421, 553]),
    ChromeTextSegment::Line([421, 670]),
    ChromeTextSegment::Line([248, 670]),
    ChromeTextSegment::Line([248, 946]),
    ChromeTextSegment::Quad {
        control: [248, 1012],
        to: [270, 1041],
    },
    ChromeTextSegment::Quad {
        control: [291, 1069],
        to: [343, 1069],
    },
    ChromeTextSegment::Quad {
        control: [396, 1069],
        to: [442, 1039],
    },
    ChromeTextSegment::Line([442, 1174]),
    ChromeTextSegment::Quad {
        control: [390, 1192],
        to: [296, 1192],
    },
    ChromeTextSegment::Quad {
        control: [203, 1192],
        to: [152, 1139],
    },
    ChromeTextSegment::Quad {
        control: [102, 1086],
        to: [102, 988],
    },
    ChromeTextSegment::Line([102, 670]),
    ChromeTextSegment::Line([29, 670]),
    ChromeTextSegment::Line([29, 553]),
    ChromeTextSegment::Line([102, 553]),
];

const SPONSOR_DOWN_CONTOUR_1: [ChromeTextSegment; 6] = [
    ChromeTextSegment::Quad {
        control: [666, 1072],
        to: [815, 1072],
    },
    ChromeTextSegment::Quad {
        control: [884, 1072],
        to: [924, 1018],
    },
    ChromeTextSegment::Quad {
        control: [964, 964],
        to: [964, 865],
    },
    ChromeTextSegment::Quad {
        control: [964, 661],
        to: [815, 661],
    },
    ChromeTextSegment::Quad {
        control: [747, 661],
        to: [706, 715],
    },
    ChromeTextSegment::Quad {
        control: [666, 769],
        to: [666, 865],
    },
];

const SPONSOR_DOWN_CONTOUR_2: [ChromeTextSegment; 8] = [
    ChromeTextSegment::Quad {
        control: [679, 541],
        to: [815, 541],
    },
    ChromeTextSegment::Quad {
        control: [958, 541],
        to: [1038, 628],
    },
    ChromeTextSegment::Quad {
        control: [1116, 715],
        to: [1116, 865],
    },
    ChromeTextSegment::Quad {
        control: [1116, 1015],
        to: [1036, 1104],
    },
    ChromeTextSegment::Quad {
        control: [955, 1192],
        to: [815, 1192],
    },
    ChromeTextSegment::Quad {
        control: [672, 1192],
        to: [593, 1103],
    },
    ChromeTextSegment::Quad {
        control: [514, 1014],
        to: [514, 865],
    },
    ChromeTextSegment::Quad {
        control: [514, 722],
        to: [597, 632],
    },
];

const SPONSOR_DOWN_CONTOUR_3: [ChromeTextSegment; 15] = [
    ChromeTextSegment::Line([1727, 612]),
    ChromeTextSegment::Quad {
        control: [1787, 541],
        to: [1903, 541],
    },
    ChromeTextSegment::Quad {
        control: [2015, 541],
        to: [2080, 608],
    },
    ChromeTextSegment::Quad {
        control: [2144, 675],
        to: [2144, 796],
    },
    ChromeTextSegment::Line([2144, 1180]),
    ChromeTextSegment::Line([1998, 1180]),
    ChromeTextSegment::Line([1998, 818]),
    ChromeTextSegment::Quad {
        control: [1998, 737],
        to: [1967, 701],
    },
    ChromeTextSegment::Quad {
        control: [1937, 663],
        to: [1867, 663],
    },
    ChromeTextSegment::Quad {
        control: [1835, 663],
        to: [1799, 682],
    },
    ChromeTextSegment::Quad {
        control: [1761, 700],
        to: [1741, 726],
    },
    ChromeTextSegment::Line([1741, 1180]),
    ChromeTextSegment::Line([1595, 1180]),
    ChromeTextSegment::Line([1595, 553]),
    ChromeTextSegment::Line([1700, 553]),
];

const SPONSOR_DOWN_CONTOUR_4: [ChromeTextSegment; 3] = [
    ChromeTextSegment::Line([2730, 801]),
    ChromeTextSegment::Quad {
        control: [2715, 663],
        to: [2578, 663],
    },
    ChromeTextSegment::Quad {
        control: [2452, 663],
        to: [2422, 801],
    },
];

const SPONSOR_DOWN_CONTOUR_5: [ChromeTextSegment; 13] = [
    ChromeTextSegment::Quad {
        control: [2516, 1070],
        to: [2595, 1070],
    },
    ChromeTextSegment::Quad {
        control: [2696, 1070],
        to: [2748, 1017],
    },
    ChromeTextSegment::Line([2805, 1130]),
    ChromeTextSegment::Quad {
        control: [2728, 1192],
        to: [2574, 1192],
    },
    ChromeTextSegment::Quad {
        control: [2430, 1192],
        to: [2347, 1109],
    },
    ChromeTextSegment::Quad {
        control: [2264, 1024],
        to: [2264, 873],
    },
    ChromeTextSegment::Quad {
        control: [2264, 725],
        to: [2355, 633],
    },
    ChromeTextSegment::Quad {
        control: [2446, 541],
        to: [2575, 541],
    },
    ChromeTextSegment::Quad {
        control: [2711, 541],
        to: [2795, 622],
    },
    ChromeTextSegment::Quad {
        control: [2877, 704],
        to: [2877, 830],
    },
    ChromeTextSegment::Line([2865, 912]),
    ChromeTextSegment::Line([2416, 912]),
    ChromeTextSegment::Quad {
        control: [2421, 987],
        to: [2468, 1028],
    },
];

const SPONSOR_DOWN_CONTOUR_6: [ChromeTextSegment; 6] = [
    ChromeTextSegment::Quad {
        control: [3106, 1072],
        to: [3255, 1072],
    },
    ChromeTextSegment::Quad {
        control: [3324, 1072],
        to: [3364, 1018],
    },
    ChromeTextSegment::Quad {
        control: [3404, 964],
        to: [3404, 865],
    },
    ChromeTextSegment::Quad {
        control: [3404, 661],
        to: [3255, 661],
    },
    ChromeTextSegment::Quad {
        control: [3187, 661],
        to: [3146, 715],
    },
    ChromeTextSegment::Quad {
        control: [3106, 769],
        to: [3106, 865],
    },
];

const SPONSOR_DOWN_CONTOUR_7: [ChromeTextSegment; 8] = [
    ChromeTextSegment::Quad {
        control: [3119, 541],
        to: [3255, 541],
    },
    ChromeTextSegment::Quad {
        control: [3398, 541],
        to: [3478, 628],
    },
    ChromeTextSegment::Quad {
        control: [3556, 715],
        to: [3556, 865],
    },
    ChromeTextSegment::Quad {
        control: [3556, 1015],
        to: [3476, 1104],
    },
    ChromeTextSegment::Quad {
        control: [3395, 1192],
        to: [3255, 1192],
    },
    ChromeTextSegment::Quad {
        control: [3112, 1192],
        to: [3033, 1103],
    },
    ChromeTextSegment::Quad {
        control: [2954, 1014],
        to: [2954, 865],
    },
    ChromeTextSegment::Quad {
        control: [2954, 722],
        to: [3037, 632],
    },
];

const SPONSOR_DOWN_CONTOUR_8: [ChromeTextSegment; 12] = [
    ChromeTextSegment::Line([4224, 284]),
    ChromeTextSegment::Line([4224, 1180]),
    ChromeTextSegment::Line([4078, 1180]),
    ChromeTextSegment::Line([4078, 1142]),
    ChromeTextSegment::Quad {
        control: [4060, 1162],
        to: [4017, 1178],
    },
    ChromeTextSegment::Quad {
        control: [3974, 1192],
        to: [3927, 1192],
    },
    ChromeTextSegment::Quad {
        control: [3795, 1192],
        to: [3720, 1109],
    },
    ChromeTextSegment::Quad {
        control: [3645, 1025],
        to: [3645, 876],
    },
    ChromeTextSegment::Quad {
        control: [3645, 728],
        to: [3732, 635],
    },
    ChromeTextSegment::Quad {
        control: [3818, 541],
        to: [3948, 541],
    },
    ChromeTextSegment::Quad {
        control: [4019, 541],
        to: [4078, 571],
    },
    ChromeTextSegment::Line([4078, 319]),
];

const SPONSOR_DOWN_CONTOUR_9: [ChromeTextSegment; 7] = [
    ChromeTextSegment::Quad {
        control: [4068, 1043],
        to: [4078, 1030],
    },
    ChromeTextSegment::Line([4078, 703]),
    ChromeTextSegment::Quad {
        control: [4031, 666],
        to: [3981, 666],
    },
    ChromeTextSegment::Quad {
        control: [3893, 666],
        to: [3845, 719],
    },
    ChromeTextSegment::Quad {
        control: [3798, 772],
        to: [3798, 873],
    },
    ChromeTextSegment::Quad {
        control: [3798, 1069],
        to: [3986, 1069],
    },
    ChromeTextSegment::Quad {
        control: [4007, 1069],
        to: [4038, 1056],
    },
];

const SPONSOR_DOWN_CONTOUR_10: [ChromeTextSegment; 3] = [
    ChromeTextSegment::Line([4795, 801]),
    ChromeTextSegment::Quad {
        control: [4780, 663],
        to: [4643, 663],
    },
    ChromeTextSegment::Quad {
        control: [4517, 663],
        to: [4487, 801],
    },
];

const SPONSOR_DOWN_CONTOUR_11: [ChromeTextSegment; 13] = [
    ChromeTextSegment::Quad {
        control: [4581, 1070],
        to: [4660, 1070],
    },
    ChromeTextSegment::Quad {
        control: [4761, 1070],
        to: [4813, 1017],
    },
    ChromeTextSegment::Line([4870, 1130]),
    ChromeTextSegment::Quad {
        control: [4793, 1192],
        to: [4639, 1192],
    },
    ChromeTextSegment::Quad {
        control: [4495, 1192],
        to: [4412, 1109],
    },
    ChromeTextSegment::Quad {
        control: [4329, 1024],
        to: [4329, 873],
    },
    ChromeTextSegment::Quad {
        control: [4329, 725],
        to: [4420, 633],
    },
    ChromeTextSegment::Quad {
        control: [4511, 541],
        to: [4640, 541],
    },
    ChromeTextSegment::Quad {
        control: [4776, 541],
        to: [4860, 622],
    },
    ChromeTextSegment::Quad {
        control: [4942, 704],
        to: [4942, 830],
    },
    ChromeTextSegment::Line([4930, 912]),
    ChromeTextSegment::Line([4481, 912]),
    ChromeTextSegment::Quad {
        control: [4486, 987],
        to: [4533, 1028],
    },
];

const SPONSOR_DOWN_CONTOUR_12: [ChromeTextSegment; 6] = [
    ChromeTextSegment::Line([5231, 284]),
    ChromeTextSegment::Line([5231, 989]),
    ChromeTextSegment::Quad {
        control: [5231, 1105],
        to: [5300, 1127],
    },
    ChromeTextSegment::Quad {
        control: [5266, 1192],
        to: [5184, 1192],
    },
    ChromeTextSegment::Quad {
        control: [5084, 1192],
        to: [5084, 1053],
    },
    ChromeTextSegment::Line([5084, 319]),
];

const SPONSOR_DOWN_CONTOUR_13: [ChromeTextSegment; 8] = [
    ChromeTextSegment::Quad {
        control: [5494, 309],
        to: [5530, 309],
    },
    ChromeTextSegment::Quad {
        control: [5565, 309],
        to: [5589, 335],
    },
    ChromeTextSegment::Quad {
        control: [5614, 360],
        to: [5614, 395],
    },
    ChromeTextSegment::Quad {
        control: [5614, 430],
        to: [5589, 455],
    },
    ChromeTextSegment::Quad {
        control: [5565, 479],
        to: [5530, 479],
    },
    ChromeTextSegment::Quad {
        control: [5494, 479],
        to: [5470, 455],
    },
    ChromeTextSegment::Quad {
        control: [5444, 430],
        to: [5444, 395],
    },
    ChromeTextSegment::Quad {
        control: [5444, 360],
        to: [5470, 335],
    },
];

const SPONSOR_DOWN_CONTOUR_14: [ChromeTextSegment; 6] = [
    ChromeTextSegment::Line([5602, 1180]),
    ChromeTextSegment::Line([5453, 1180]),
    ChromeTextSegment::Line([5453, 673]),
    ChromeTextSegment::Line([5374, 673]),
    ChromeTextSegment::Line([5374, 553]),
    ChromeTextSegment::Line([5602, 553]),
];

const SPONSOR_DOWN_CONTOUR_15: [ChromeTextSegment; 31] = [
    ChromeTextSegment::Line([6211, 648]),
    ChromeTextSegment::Quad {
        control: [6247, 703],
        to: [6247, 778],
    },
    ChromeTextSegment::Quad {
        control: [6247, 885],
        to: [6181, 949],
    },
    ChromeTextSegment::Quad {
        control: [6116, 1014],
        to: [6016, 1014],
    },
    ChromeTextSegment::Line([5974, 1010]),
    ChromeTextSegment::Line([5950, 1007]),
    ChromeTextSegment::Line([5919, 1019]),
    ChromeTextSegment::Quad {
        control: [5892, 1032],
        to: [5892, 1045],
    },
    ChromeTextSegment::Quad {
        control: [5892, 1069],
        to: [5932, 1069],
    },
    ChromeTextSegment::Line([5993, 1060]),
    ChromeTextSegment::Line([6066, 1051]),
    ChromeTextSegment::Quad {
        control: [6280, 1051],
        to: [6280, 1223],
    },
    ChromeTextSegment::Quad {
        control: [6280, 1318],
        to: [6194, 1372],
    },
    ChromeTextSegment::Quad {
        control: [6109, 1426],
        to: [5988, 1426],
    },
    ChromeTextSegment::Quad {
        control: [5844, 1426],
        to: [5728, 1342],
    },
    ChromeTextSegment::Line([5820, 1227]),
    ChromeTextSegment::Quad {
        control: [5897, 1297],
        to: [5991, 1297],
    },
    ChromeTextSegment::Quad {
        control: [6055, 1297],
        to: [6096, 1278],
    },
    ChromeTextSegment::Quad {
        control: [6137, 1260],
        to: [6137, 1227],
    },
    ChromeTextSegment::Quad {
        control: [6137, 1172],
        to: [6045, 1172],
    },
    ChromeTextSegment::Line([5973, 1178]),
    ChromeTextSegment::Line([5899, 1184]),
    ChromeTextSegment::Quad {
        control: [5748, 1184],
        to: [5748, 1075],
    },
    ChromeTextSegment::Quad {
        control: [5748, 1044],
        to: [5774, 1014],
    },
    ChromeTextSegment::Quad {
        control: [5798, 984],
        to: [5832, 970],
    },
    ChromeTextSegment::Quad {
        control: [5724, 900],
        to: [5724, 772],
    },
    ChromeTextSegment::Quad {
        control: [5724, 671],
        to: [5798, 606],
    },
    ChromeTextSegment::Quad {
        control: [5872, 540],
        to: [5980, 540],
    },
    ChromeTextSegment::Quad {
        control: [6064, 540],
        to: [6122, 572],
    },
    ChromeTextSegment::Line([6179, 505]),
    ChromeTextSegment::Line([6280, 596]),
];

const SPONSOR_DOWN_CONTOUR_16: [ChromeTextSegment; 8] = [
    ChromeTextSegment::Quad {
        control: [5938, 660],
        to: [5906, 691],
    },
    ChromeTextSegment::Quad {
        control: [5874, 723],
        to: [5874, 773],
    },
    ChromeTextSegment::Quad {
        control: [5874, 828],
        to: [5905, 862],
    },
    ChromeTextSegment::Quad {
        control: [5935, 896],
        to: [5988, 896],
    },
    ChromeTextSegment::Quad {
        control: [6040, 896],
        to: [6068, 864],
    },
    ChromeTextSegment::Quad {
        control: [6096, 831],
        to: [6096, 773],
    },
    ChromeTextSegment::Quad {
        control: [6096, 725],
        to: [6066, 693],
    },
    ChromeTextSegment::Quad {
        control: [6036, 660],
        to: [5988, 660],
    },
];

const SPONSOR_DOWN_CONTOUR_17: [ChromeTextSegment; 15] = [
    ChromeTextSegment::Line([6524, 593]),
    ChromeTextSegment::Quad {
        control: [6580, 541],
        to: [6679, 541],
    },
    ChromeTextSegment::Quad {
        control: [6798, 541],
        to: [6862, 607],
    },
    ChromeTextSegment::Quad {
        control: [6928, 671],
        to: [6928, 793],
    },
    ChromeTextSegment::Line([6928, 1180]),
    ChromeTextSegment::Line([6780, 1180]),
    ChromeTextSegment::Line([6780, 793]),
    ChromeTextSegment::Quad {
        control: [6780, 736],
        to: [6744, 701],
    },
    ChromeTextSegment::Quad {
        control: [6707, 666],
        to: [6649, 666],
    },
    ChromeTextSegment::Quad {
        control: [6613, 666],
        to: [6576, 685],
    },
    ChromeTextSegment::Quad {
        control: [6539, 704],
        to: [6524, 730],
    },
    ChromeTextSegment::Line([6524, 1180]),
    ChromeTextSegment::Line([6375, 1180]),
    ChromeTextSegment::Line([6375, 319]),
    ChromeTextSegment::Line([6524, 284]),
];

const SPONSOR_DOWN_CONTOUR_18: [ChromeTextSegment; 18] = [
    ChromeTextSegment::Line([7107, 425]),
    ChromeTextSegment::Line([7253, 371]),
    ChromeTextSegment::Line([7253, 553]),
    ChromeTextSegment::Line([7426, 553]),
    ChromeTextSegment::Line([7426, 670]),
    ChromeTextSegment::Line([7253, 670]),
    ChromeTextSegment::Line([7253, 946]),
    ChromeTextSegment::Quad {
        control: [7253, 1012],
        to: [7275, 1041],
    },
    ChromeTextSegment::Quad {
        control: [7296, 1069],
        to: [7348, 1069],
    },
    ChromeTextSegment::Quad {
        control: [7401, 1069],
        to: [7447, 1039],
    },
    ChromeTextSegment::Line([7447, 1174]),
    ChromeTextSegment::Quad {
        control: [7395, 1192],
        to: [7301, 1192],
    },
    ChromeTextSegment::Quad {
        control: [7208, 1192],
        to: [7157, 1139],
    },
    ChromeTextSegment::Quad {
        control: [7107, 1086],
        to: [7107, 988],
    },
    ChromeTextSegment::Line([7107, 670]),
    ChromeTextSegment::Line([7034, 670]),
    ChromeTextSegment::Line([7034, 553]),
    ChromeTextSegment::Line([7107, 553]),
];

const SPONSOR_DOWN_CONTOURS: [ChromeTextContour; 19] = [
    ChromeTextContour {
        start: [102, 553],
        segments: &SPONSOR_DOWN_CONTOUR_0,
    },
    ChromeTextContour {
        start: [666, 865],
        segments: &SPONSOR_DOWN_CONTOUR_1,
    },
    ChromeTextContour {
        start: [597, 632],
        segments: &SPONSOR_DOWN_CONTOUR_2,
    },
    ChromeTextContour {
        start: [1700, 553],
        segments: &SPONSOR_DOWN_CONTOUR_3,
    },
    ChromeTextContour {
        start: [2422, 801],
        segments: &SPONSOR_DOWN_CONTOUR_4,
    },
    ChromeTextContour {
        start: [2468, 1028],
        segments: &SPONSOR_DOWN_CONTOUR_5,
    },
    ChromeTextContour {
        start: [3106, 865],
        segments: &SPONSOR_DOWN_CONTOUR_6,
    },
    ChromeTextContour {
        start: [3037, 632],
        segments: &SPONSOR_DOWN_CONTOUR_7,
    },
    ChromeTextContour {
        start: [4078, 319],
        segments: &SPONSOR_DOWN_CONTOUR_8,
    },
    ChromeTextContour {
        start: [4038, 1056],
        segments: &SPONSOR_DOWN_CONTOUR_9,
    },
    ChromeTextContour {
        start: [4487, 801],
        segments: &SPONSOR_DOWN_CONTOUR_10,
    },
    ChromeTextContour {
        start: [4533, 1028],
        segments: &SPONSOR_DOWN_CONTOUR_11,
    },
    ChromeTextContour {
        start: [5084, 319],
        segments: &SPONSOR_DOWN_CONTOUR_12,
    },
    ChromeTextContour {
        start: [5470, 335],
        segments: &SPONSOR_DOWN_CONTOUR_13,
    },
    ChromeTextContour {
        start: [5602, 553],
        segments: &SPONSOR_DOWN_CONTOUR_14,
    },
    ChromeTextContour {
        start: [6280, 596],
        segments: &SPONSOR_DOWN_CONTOUR_15,
    },
    ChromeTextContour {
        start: [5988, 660],
        segments: &SPONSOR_DOWN_CONTOUR_16,
    },
    ChromeTextContour {
        start: [6524, 284],
        segments: &SPONSOR_DOWN_CONTOUR_17,
    },
    ChromeTextContour {
        start: [7107, 553],
        segments: &SPONSOR_DOWN_CONTOUR_18,
    },
];

pub const SPONSOR_DOWN: ChromeTextDefinition = ChromeTextDefinition {
    text: "to neodelight",
    define_text_id: 97,
    font_ids: &SPONSOR_DOWN_FONT_IDS,
    color_rgb: [255, 255, 255],
    bounds_centipx: [30, 8005, 285, 1425],
    contours: &SPONSOR_DOWN_CONTOURS,
};

const BACK_UP_FONT_IDS: [u16; 1] = [54];

const BACK_UP_CONTOUR_0: [ChromeTextSegment; 11] = [
    ChromeTextSegment::Line([214, 576]),
    ChromeTextSegment::Quad {
        control: [273, 541],
        to: [342, 541],
    },
    ChromeTextSegment::Quad {
        control: [479, 541],
        to: [564, 628],
    },
    ChromeTextSegment::Quad {
        control: [648, 715],
        to: [648, 851],
    },
    ChromeTextSegment::Quad {
        control: [648, 1009],
        to: [564, 1100],
    },
    ChromeTextSegment::Quad {
        control: [478, 1192],
        to: [333, 1192],
    },
    ChromeTextSegment::Quad {
        control: [246, 1192],
        to: [183, 1146],
    },
    ChromeTextSegment::Line([150, 1192]),
    ChromeTextSegment::Line([68, 1192]),
    ChromeTextSegment::Line([68, 319]),
    ChromeTextSegment::Line([214, 284]),
];

const BACK_UP_CONTOUR_1: [ChromeTextSegment; 6] = [
    ChromeTextSegment::Line([214, 1030]),
    ChromeTextSegment::Quad {
        control: [246, 1068],
        to: [301, 1068],
    },
    ChromeTextSegment::Quad {
        control: [408, 1068],
        to: [452, 1017],
    },
    ChromeTextSegment::Quad {
        control: [496, 967],
        to: [496, 857],
    },
    ChromeTextSegment::Quad {
        control: [496, 666],
        to: [313, 666],
    },
    ChromeTextSegment::Quad {
        control: [248, 666],
        to: [214, 702],
    },
];

const BACK_UP_CONTOUR_2: [ChromeTextSegment; 19] = [
    ChromeTextSegment::Quad {
        control: [1127, 541],
        to: [1193, 607],
    },
    ChromeTextSegment::Quad {
        control: [1260, 671],
        to: [1260, 854],
    },
    ChromeTextSegment::Line([1260, 988]),
    ChromeTextSegment::Quad {
        control: [1260, 1113],
        to: [1311, 1146],
    },
    ChromeTextSegment::Quad {
        control: [1293, 1178],
        to: [1271, 1185],
    },
    ChromeTextSegment::Line([1219, 1192]),
    ChromeTextSegment::Quad {
        control: [1188, 1192],
        to: [1163, 1168],
    },
    ChromeTextSegment::Quad {
        control: [1137, 1145],
        to: [1129, 1118],
    },
    ChromeTextSegment::Quad {
        control: [1109, 1151],
        to: [1060, 1172],
    },
    ChromeTextSegment::Quad {
        control: [1009, 1192],
        to: [955, 1192],
    },
    ChromeTextSegment::Quad {
        control: [855, 1192],
        to: [796, 1141],
    },
    ChromeTextSegment::Quad {
        control: [739, 1091],
        to: [739, 997],
    },
    ChromeTextSegment::Quad {
        control: [739, 888],
        to: [821, 827],
    },
    ChromeTextSegment::Quad {
        control: [902, 765],
        to: [1053, 765],
    },
    ChromeTextSegment::Line([1114, 775]),
    ChromeTextSegment::Quad {
        control: [1114, 663],
        to: [973, 663],
    },
    ChromeTextSegment::Quad {
        control: [891, 663],
        to: [835, 691],
    },
    ChromeTextSegment::Line([803, 578]),
    ChromeTextSegment::Quad {
        control: [879, 541],
        to: [984, 541],
    },
];

const BACK_UP_CONTOUR_3: [ChromeTextSegment; 5] = [
    ChromeTextSegment::Quad {
        control: [885, 878],
        to: [885, 991],
    },
    ChromeTextSegment::Quad {
        control: [885, 1076],
        to: [982, 1076],
    },
    ChromeTextSegment::Quad {
        control: [1115, 1076],
        to: [1115, 943],
    },
    ChromeTextSegment::Line([1115, 885]),
    ChromeTextSegment::Line([1059, 878]),
];

const BACK_UP_CONTOUR_4: [ChromeTextSegment; 14] = [
    ChromeTextSegment::Quad {
        control: [1531, 775],
        to: [1531, 872],
    },
    ChromeTextSegment::Quad {
        control: [1531, 1070],
        to: [1719, 1070],
    },
    ChromeTextSegment::Quad {
        control: [1801, 1070],
        to: [1863, 1016],
    },
    ChromeTextSegment::Line([1917, 1131]),
    ChromeTextSegment::Quad {
        control: [1852, 1171],
        to: [1805, 1181],
    },
    ChromeTextSegment::Line([1692, 1192]),
    ChromeTextSegment::Quad {
        control: [1546, 1192],
        to: [1463, 1107],
    },
    ChromeTextSegment::Quad {
        control: [1379, 1023],
        to: [1379, 872],
    },
    ChromeTextSegment::Quad {
        control: [1379, 724],
        to: [1470, 633],
    },
    ChromeTextSegment::Quad {
        control: [1563, 541],
        to: [1721, 541],
    },
    ChromeTextSegment::Quad {
        control: [1831, 541],
        to: [1912, 602],
    },
    ChromeTextSegment::Line([1849, 712]),
    ChromeTextSegment::Quad {
        control: [1797, 663],
        to: [1710, 663],
    },
    ChromeTextSegment::Quad {
        control: [1627, 663],
        to: [1579, 719],
    },
];

const BACK_UP_CONTOUR_5: [ChromeTextSegment; 12] = [
    ChromeTextSegment::Line([2610, 1180]),
    ChromeTextSegment::Line([2435, 1180]),
    ChromeTextSegment::Line([2251, 896]),
    ChromeTextSegment::Line([2181, 970]),
    ChromeTextSegment::Line([2181, 1180]),
    ChromeTextSegment::Line([2035, 1180]),
    ChromeTextSegment::Line([2035, 319]),
    ChromeTextSegment::Line([2181, 284]),
    ChromeTextSegment::Line([2181, 793]),
    ChromeTextSegment::Line([2390, 553]),
    ChromeTextSegment::Line([2567, 553]),
    ChromeTextSegment::Line([2355, 791]),
];

const BACK_UP_CONTOUR_6: [ChromeTextSegment; 18] = [
    ChromeTextSegment::Line([3072, 425]),
    ChromeTextSegment::Line([3218, 371]),
    ChromeTextSegment::Line([3218, 553]),
    ChromeTextSegment::Line([3391, 553]),
    ChromeTextSegment::Line([3391, 670]),
    ChromeTextSegment::Line([3218, 670]),
    ChromeTextSegment::Line([3218, 946]),
    ChromeTextSegment::Quad {
        control: [3218, 1012],
        to: [3240, 1041],
    },
    ChromeTextSegment::Quad {
        control: [3261, 1069],
        to: [3313, 1069],
    },
    ChromeTextSegment::Quad {
        control: [3366, 1069],
        to: [3412, 1039],
    },
    ChromeTextSegment::Line([3412, 1174]),
    ChromeTextSegment::Quad {
        control: [3360, 1192],
        to: [3266, 1192],
    },
    ChromeTextSegment::Quad {
        control: [3173, 1192],
        to: [3122, 1139],
    },
    ChromeTextSegment::Quad {
        control: [3072, 1086],
        to: [3072, 988],
    },
    ChromeTextSegment::Line([3072, 670]),
    ChromeTextSegment::Line([2999, 670]),
    ChromeTextSegment::Line([2999, 553]),
    ChromeTextSegment::Line([3072, 553]),
];

const BACK_UP_CONTOUR_7: [ChromeTextSegment; 6] = [
    ChromeTextSegment::Quad {
        control: [3636, 1072],
        to: [3785, 1072],
    },
    ChromeTextSegment::Quad {
        control: [3854, 1072],
        to: [3894, 1018],
    },
    ChromeTextSegment::Quad {
        control: [3934, 964],
        to: [3934, 865],
    },
    ChromeTextSegment::Quad {
        control: [3934, 661],
        to: [3785, 661],
    },
    ChromeTextSegment::Quad {
        control: [3717, 661],
        to: [3676, 715],
    },
    ChromeTextSegment::Quad {
        control: [3636, 769],
        to: [3636, 865],
    },
];

const BACK_UP_CONTOUR_8: [ChromeTextSegment; 8] = [
    ChromeTextSegment::Quad {
        control: [3649, 541],
        to: [3785, 541],
    },
    ChromeTextSegment::Quad {
        control: [3928, 541],
        to: [4008, 628],
    },
    ChromeTextSegment::Quad {
        control: [4086, 715],
        to: [4086, 865],
    },
    ChromeTextSegment::Quad {
        control: [4086, 1015],
        to: [4006, 1104],
    },
    ChromeTextSegment::Quad {
        control: [3925, 1192],
        to: [3785, 1192],
    },
    ChromeTextSegment::Quad {
        control: [3642, 1192],
        to: [3563, 1103],
    },
    ChromeTextSegment::Quad {
        control: [3484, 1014],
        to: [3484, 865],
    },
    ChromeTextSegment::Quad {
        control: [3484, 722],
        to: [3567, 632],
    },
];

const BACK_UP_CONTOUR_9: [ChromeTextSegment; 24] = [
    ChromeTextSegment::Quad {
        control: [5321, 541],
        to: [5379, 601],
    },
    ChromeTextSegment::Quad {
        control: [5438, 661],
        to: [5438, 766],
    },
    ChromeTextSegment::Line([5438, 1180]),
    ChromeTextSegment::Line([5291, 1180]),
    ChromeTextSegment::Line([5291, 787]),
    ChromeTextSegment::Quad {
        control: [5291, 663],
        to: [5180, 663],
    },
    ChromeTextSegment::Quad {
        control: [5147, 663],
        to: [5117, 683],
    },
    ChromeTextSegment::Quad {
        control: [5086, 702],
        to: [5074, 728],
    },
    ChromeTextSegment::Line([5074, 1180]),
    ChromeTextSegment::Line([4928, 1180]),
    ChromeTextSegment::Line([4928, 764]),
    ChromeTextSegment::Quad {
        control: [4928, 718],
        to: [4899, 691],
    },
    ChromeTextSegment::Quad {
        control: [4871, 663],
        to: [4818, 663],
    },
    ChromeTextSegment::Quad {
        control: [4791, 663],
        to: [4758, 684],
    },
    ChromeTextSegment::Quad {
        control: [4725, 704],
        to: [4711, 729],
    },
    ChromeTextSegment::Line([4711, 1180]),
    ChromeTextSegment::Line([4565, 1180]),
    ChromeTextSegment::Line([4565, 553]),
    ChromeTextSegment::Line([4665, 553]),
    ChromeTextSegment::Line([4695, 608]),
    ChromeTextSegment::Quad {
        control: [4756, 541],
        to: [4853, 541],
    },
    ChromeTextSegment::Quad {
        control: [4977, 541],
        to: [5040, 609],
    },
    ChromeTextSegment::Quad {
        control: [5069, 578],
        to: [5118, 560],
    },
    ChromeTextSegment::Quad {
        control: [5167, 541],
        to: [5216, 541],
    },
];

const BACK_UP_CONTOUR_10: [ChromeTextSegment; 3] = [
    ChromeTextSegment::Line([6020, 801]),
    ChromeTextSegment::Quad {
        control: [6005, 663],
        to: [5868, 663],
    },
    ChromeTextSegment::Quad {
        control: [5742, 663],
        to: [5712, 801],
    },
];

const BACK_UP_CONTOUR_11: [ChromeTextSegment; 13] = [
    ChromeTextSegment::Quad {
        control: [5806, 1070],
        to: [5885, 1070],
    },
    ChromeTextSegment::Quad {
        control: [5986, 1070],
        to: [6038, 1017],
    },
    ChromeTextSegment::Line([6095, 1130]),
    ChromeTextSegment::Quad {
        control: [6018, 1192],
        to: [5864, 1192],
    },
    ChromeTextSegment::Quad {
        control: [5720, 1192],
        to: [5637, 1109],
    },
    ChromeTextSegment::Quad {
        control: [5554, 1024],
        to: [5554, 873],
    },
    ChromeTextSegment::Quad {
        control: [5554, 725],
        to: [5645, 633],
    },
    ChromeTextSegment::Quad {
        control: [5736, 541],
        to: [5865, 541],
    },
    ChromeTextSegment::Quad {
        control: [6001, 541],
        to: [6085, 622],
    },
    ChromeTextSegment::Quad {
        control: [6167, 704],
        to: [6167, 830],
    },
    ChromeTextSegment::Line([6155, 912]),
    ChromeTextSegment::Line([5706, 912]),
    ChromeTextSegment::Quad {
        control: [5711, 987],
        to: [5758, 1028],
    },
];

const BACK_UP_CONTOUR_12: [ChromeTextSegment; 15] = [
    ChromeTextSegment::Line([6417, 612]),
    ChromeTextSegment::Quad {
        control: [6477, 541],
        to: [6593, 541],
    },
    ChromeTextSegment::Quad {
        control: [6705, 541],
        to: [6770, 608],
    },
    ChromeTextSegment::Quad {
        control: [6834, 675],
        to: [6834, 796],
    },
    ChromeTextSegment::Line([6834, 1180]),
    ChromeTextSegment::Line([6688, 1180]),
    ChromeTextSegment::Line([6688, 818]),
    ChromeTextSegment::Quad {
        control: [6688, 737],
        to: [6657, 701],
    },
    ChromeTextSegment::Quad {
        control: [6627, 663],
        to: [6557, 663],
    },
    ChromeTextSegment::Quad {
        control: [6525, 663],
        to: [6489, 682],
    },
    ChromeTextSegment::Quad {
        control: [6451, 700],
        to: [6431, 726],
    },
    ChromeTextSegment::Line([6431, 1180]),
    ChromeTextSegment::Line([6285, 1180]),
    ChromeTextSegment::Line([6285, 553]),
    ChromeTextSegment::Line([6390, 553]),
];

const BACK_UP_CONTOUR_13: [ChromeTextSegment; 14] = [
    ChromeTextSegment::Line([7545, 1181]),
    ChromeTextSegment::Line([7399, 1181]),
    ChromeTextSegment::Line([7399, 1128]),
    ChromeTextSegment::Quad {
        control: [7367, 1154],
        to: [7313, 1173],
    },
    ChromeTextSegment::Quad {
        control: [7258, 1192],
        to: [7213, 1192],
    },
    ChromeTextSegment::Quad {
        control: [6995, 1192],
        to: [6995, 960],
    },
    ChromeTextSegment::Line([6995, 553]),
    ChromeTextSegment::Line([7141, 553]),
    ChromeTextSegment::Line([7141, 949]),
    ChromeTextSegment::Quad {
        control: [7141, 1070],
        to: [7249, 1070],
    },
    ChromeTextSegment::Quad {
        control: [7299, 1070],
        to: [7342, 1044],
    },
    ChromeTextSegment::Quad {
        control: [7385, 1018],
        to: [7399, 984],
    },
    ChromeTextSegment::Line([7399, 553]),
    ChromeTextSegment::Line([7545, 553]),
];

const BACK_UP_CONTOURS: [ChromeTextContour; 14] = [
    ChromeTextContour {
        start: [214, 284],
        segments: &BACK_UP_CONTOUR_0,
    },
    ChromeTextContour {
        start: [214, 702],
        segments: &BACK_UP_CONTOUR_1,
    },
    ChromeTextContour {
        start: [984, 541],
        segments: &BACK_UP_CONTOUR_2,
    },
    ChromeTextContour {
        start: [1059, 878],
        segments: &BACK_UP_CONTOUR_3,
    },
    ChromeTextContour {
        start: [1579, 719],
        segments: &BACK_UP_CONTOUR_4,
    },
    ChromeTextContour {
        start: [2355, 791],
        segments: &BACK_UP_CONTOUR_5,
    },
    ChromeTextContour {
        start: [3072, 553],
        segments: &BACK_UP_CONTOUR_6,
    },
    ChromeTextContour {
        start: [3636, 865],
        segments: &BACK_UP_CONTOUR_7,
    },
    ChromeTextContour {
        start: [3567, 632],
        segments: &BACK_UP_CONTOUR_8,
    },
    ChromeTextContour {
        start: [5216, 541],
        segments: &BACK_UP_CONTOUR_9,
    },
    ChromeTextContour {
        start: [5712, 801],
        segments: &BACK_UP_CONTOUR_10,
    },
    ChromeTextContour {
        start: [5758, 1028],
        segments: &BACK_UP_CONTOUR_11,
    },
    ChromeTextContour {
        start: [6390, 553],
        segments: &BACK_UP_CONTOUR_12,
    },
    ChromeTextContour {
        start: [7545, 553],
        segments: &BACK_UP_CONTOUR_13,
    },
];

pub const BACK_UP: ChromeTextDefinition = ChromeTextDefinition {
    text: "back to menu",
    define_text_id: 120,
    font_ids: &BACK_UP_FONT_IDS,
    color_rgb: [0, 0, 0],
    bounds_centipx: [65, 8150, 285, 1190],
    contours: &BACK_UP_CONTOURS,
};

const BACK_OVER_FONT_IDS: [u16; 1] = [54];

const BACK_OVER_CONTOUR_0: [ChromeTextSegment; 11] = [
    ChromeTextSegment::Line([214, 576]),
    ChromeTextSegment::Quad {
        control: [273, 541],
        to: [342, 541],
    },
    ChromeTextSegment::Quad {
        control: [479, 541],
        to: [564, 628],
    },
    ChromeTextSegment::Quad {
        control: [648, 715],
        to: [648, 851],
    },
    ChromeTextSegment::Quad {
        control: [648, 1009],
        to: [564, 1100],
    },
    ChromeTextSegment::Quad {
        control: [478, 1192],
        to: [333, 1192],
    },
    ChromeTextSegment::Quad {
        control: [246, 1192],
        to: [183, 1146],
    },
    ChromeTextSegment::Line([150, 1192]),
    ChromeTextSegment::Line([68, 1192]),
    ChromeTextSegment::Line([68, 319]),
    ChromeTextSegment::Line([214, 284]),
];

const BACK_OVER_CONTOUR_1: [ChromeTextSegment; 6] = [
    ChromeTextSegment::Line([214, 1030]),
    ChromeTextSegment::Quad {
        control: [246, 1068],
        to: [301, 1068],
    },
    ChromeTextSegment::Quad {
        control: [408, 1068],
        to: [452, 1017],
    },
    ChromeTextSegment::Quad {
        control: [496, 967],
        to: [496, 857],
    },
    ChromeTextSegment::Quad {
        control: [496, 666],
        to: [313, 666],
    },
    ChromeTextSegment::Quad {
        control: [248, 666],
        to: [214, 702],
    },
];

const BACK_OVER_CONTOUR_2: [ChromeTextSegment; 19] = [
    ChromeTextSegment::Quad {
        control: [1127, 541],
        to: [1193, 607],
    },
    ChromeTextSegment::Quad {
        control: [1260, 671],
        to: [1260, 854],
    },
    ChromeTextSegment::Line([1260, 988]),
    ChromeTextSegment::Quad {
        control: [1260, 1113],
        to: [1311, 1146],
    },
    ChromeTextSegment::Quad {
        control: [1293, 1178],
        to: [1271, 1185],
    },
    ChromeTextSegment::Line([1219, 1192]),
    ChromeTextSegment::Quad {
        control: [1188, 1192],
        to: [1163, 1168],
    },
    ChromeTextSegment::Quad {
        control: [1137, 1145],
        to: [1129, 1118],
    },
    ChromeTextSegment::Quad {
        control: [1109, 1151],
        to: [1060, 1172],
    },
    ChromeTextSegment::Quad {
        control: [1009, 1192],
        to: [955, 1192],
    },
    ChromeTextSegment::Quad {
        control: [855, 1192],
        to: [796, 1141],
    },
    ChromeTextSegment::Quad {
        control: [739, 1091],
        to: [739, 997],
    },
    ChromeTextSegment::Quad {
        control: [739, 888],
        to: [821, 827],
    },
    ChromeTextSegment::Quad {
        control: [902, 765],
        to: [1053, 765],
    },
    ChromeTextSegment::Line([1114, 775]),
    ChromeTextSegment::Quad {
        control: [1114, 663],
        to: [973, 663],
    },
    ChromeTextSegment::Quad {
        control: [891, 663],
        to: [835, 691],
    },
    ChromeTextSegment::Line([803, 578]),
    ChromeTextSegment::Quad {
        control: [879, 541],
        to: [984, 541],
    },
];

const BACK_OVER_CONTOUR_3: [ChromeTextSegment; 5] = [
    ChromeTextSegment::Quad {
        control: [885, 878],
        to: [885, 991],
    },
    ChromeTextSegment::Quad {
        control: [885, 1076],
        to: [982, 1076],
    },
    ChromeTextSegment::Quad {
        control: [1115, 1076],
        to: [1115, 943],
    },
    ChromeTextSegment::Line([1115, 885]),
    ChromeTextSegment::Line([1059, 878]),
];

const BACK_OVER_CONTOUR_4: [ChromeTextSegment; 14] = [
    ChromeTextSegment::Quad {
        control: [1531, 775],
        to: [1531, 872],
    },
    ChromeTextSegment::Quad {
        control: [1531, 1070],
        to: [1719, 1070],
    },
    ChromeTextSegment::Quad {
        control: [1801, 1070],
        to: [1863, 1016],
    },
    ChromeTextSegment::Line([1917, 1131]),
    ChromeTextSegment::Quad {
        control: [1852, 1171],
        to: [1805, 1181],
    },
    ChromeTextSegment::Line([1692, 1192]),
    ChromeTextSegment::Quad {
        control: [1546, 1192],
        to: [1463, 1107],
    },
    ChromeTextSegment::Quad {
        control: [1379, 1023],
        to: [1379, 872],
    },
    ChromeTextSegment::Quad {
        control: [1379, 724],
        to: [1470, 633],
    },
    ChromeTextSegment::Quad {
        control: [1563, 541],
        to: [1721, 541],
    },
    ChromeTextSegment::Quad {
        control: [1831, 541],
        to: [1912, 602],
    },
    ChromeTextSegment::Line([1849, 712]),
    ChromeTextSegment::Quad {
        control: [1797, 663],
        to: [1710, 663],
    },
    ChromeTextSegment::Quad {
        control: [1627, 663],
        to: [1579, 719],
    },
];

const BACK_OVER_CONTOUR_5: [ChromeTextSegment; 12] = [
    ChromeTextSegment::Line([2610, 1180]),
    ChromeTextSegment::Line([2435, 1180]),
    ChromeTextSegment::Line([2251, 896]),
    ChromeTextSegment::Line([2181, 970]),
    ChromeTextSegment::Line([2181, 1180]),
    ChromeTextSegment::Line([2035, 1180]),
    ChromeTextSegment::Line([2035, 319]),
    ChromeTextSegment::Line([2181, 284]),
    ChromeTextSegment::Line([2181, 793]),
    ChromeTextSegment::Line([2390, 553]),
    ChromeTextSegment::Line([2567, 553]),
    ChromeTextSegment::Line([2355, 791]),
];

const BACK_OVER_CONTOUR_6: [ChromeTextSegment; 18] = [
    ChromeTextSegment::Line([3072, 425]),
    ChromeTextSegment::Line([3218, 371]),
    ChromeTextSegment::Line([3218, 553]),
    ChromeTextSegment::Line([3391, 553]),
    ChromeTextSegment::Line([3391, 670]),
    ChromeTextSegment::Line([3218, 670]),
    ChromeTextSegment::Line([3218, 946]),
    ChromeTextSegment::Quad {
        control: [3218, 1012],
        to: [3240, 1041],
    },
    ChromeTextSegment::Quad {
        control: [3261, 1069],
        to: [3313, 1069],
    },
    ChromeTextSegment::Quad {
        control: [3366, 1069],
        to: [3412, 1039],
    },
    ChromeTextSegment::Line([3412, 1174]),
    ChromeTextSegment::Quad {
        control: [3360, 1192],
        to: [3266, 1192],
    },
    ChromeTextSegment::Quad {
        control: [3173, 1192],
        to: [3122, 1139],
    },
    ChromeTextSegment::Quad {
        control: [3072, 1086],
        to: [3072, 988],
    },
    ChromeTextSegment::Line([3072, 670]),
    ChromeTextSegment::Line([2999, 670]),
    ChromeTextSegment::Line([2999, 553]),
    ChromeTextSegment::Line([3072, 553]),
];

const BACK_OVER_CONTOUR_7: [ChromeTextSegment; 6] = [
    ChromeTextSegment::Quad {
        control: [3636, 1072],
        to: [3785, 1072],
    },
    ChromeTextSegment::Quad {
        control: [3854, 1072],
        to: [3894, 1018],
    },
    ChromeTextSegment::Quad {
        control: [3934, 964],
        to: [3934, 865],
    },
    ChromeTextSegment::Quad {
        control: [3934, 661],
        to: [3785, 661],
    },
    ChromeTextSegment::Quad {
        control: [3717, 661],
        to: [3676, 715],
    },
    ChromeTextSegment::Quad {
        control: [3636, 769],
        to: [3636, 865],
    },
];

const BACK_OVER_CONTOUR_8: [ChromeTextSegment; 8] = [
    ChromeTextSegment::Quad {
        control: [3649, 541],
        to: [3785, 541],
    },
    ChromeTextSegment::Quad {
        control: [3928, 541],
        to: [4008, 628],
    },
    ChromeTextSegment::Quad {
        control: [4086, 715],
        to: [4086, 865],
    },
    ChromeTextSegment::Quad {
        control: [4086, 1015],
        to: [4006, 1104],
    },
    ChromeTextSegment::Quad {
        control: [3925, 1192],
        to: [3785, 1192],
    },
    ChromeTextSegment::Quad {
        control: [3642, 1192],
        to: [3563, 1103],
    },
    ChromeTextSegment::Quad {
        control: [3484, 1014],
        to: [3484, 865],
    },
    ChromeTextSegment::Quad {
        control: [3484, 722],
        to: [3567, 632],
    },
];

const BACK_OVER_CONTOUR_9: [ChromeTextSegment; 24] = [
    ChromeTextSegment::Quad {
        control: [5321, 541],
        to: [5379, 601],
    },
    ChromeTextSegment::Quad {
        control: [5438, 661],
        to: [5438, 766],
    },
    ChromeTextSegment::Line([5438, 1180]),
    ChromeTextSegment::Line([5291, 1180]),
    ChromeTextSegment::Line([5291, 787]),
    ChromeTextSegment::Quad {
        control: [5291, 663],
        to: [5180, 663],
    },
    ChromeTextSegment::Quad {
        control: [5147, 663],
        to: [5117, 683],
    },
    ChromeTextSegment::Quad {
        control: [5086, 702],
        to: [5074, 728],
    },
    ChromeTextSegment::Line([5074, 1180]),
    ChromeTextSegment::Line([4928, 1180]),
    ChromeTextSegment::Line([4928, 764]),
    ChromeTextSegment::Quad {
        control: [4928, 718],
        to: [4899, 691],
    },
    ChromeTextSegment::Quad {
        control: [4871, 663],
        to: [4818, 663],
    },
    ChromeTextSegment::Quad {
        control: [4791, 663],
        to: [4758, 684],
    },
    ChromeTextSegment::Quad {
        control: [4725, 704],
        to: [4711, 729],
    },
    ChromeTextSegment::Line([4711, 1180]),
    ChromeTextSegment::Line([4565, 1180]),
    ChromeTextSegment::Line([4565, 553]),
    ChromeTextSegment::Line([4665, 553]),
    ChromeTextSegment::Line([4695, 608]),
    ChromeTextSegment::Quad {
        control: [4756, 541],
        to: [4853, 541],
    },
    ChromeTextSegment::Quad {
        control: [4977, 541],
        to: [5040, 609],
    },
    ChromeTextSegment::Quad {
        control: [5069, 578],
        to: [5118, 560],
    },
    ChromeTextSegment::Quad {
        control: [5167, 541],
        to: [5216, 541],
    },
];

const BACK_OVER_CONTOUR_10: [ChromeTextSegment; 3] = [
    ChromeTextSegment::Line([6020, 801]),
    ChromeTextSegment::Quad {
        control: [6005, 663],
        to: [5868, 663],
    },
    ChromeTextSegment::Quad {
        control: [5742, 663],
        to: [5712, 801],
    },
];

const BACK_OVER_CONTOUR_11: [ChromeTextSegment; 13] = [
    ChromeTextSegment::Quad {
        control: [5806, 1070],
        to: [5885, 1070],
    },
    ChromeTextSegment::Quad {
        control: [5986, 1070],
        to: [6038, 1017],
    },
    ChromeTextSegment::Line([6095, 1130]),
    ChromeTextSegment::Quad {
        control: [6018, 1192],
        to: [5864, 1192],
    },
    ChromeTextSegment::Quad {
        control: [5720, 1192],
        to: [5637, 1109],
    },
    ChromeTextSegment::Quad {
        control: [5554, 1024],
        to: [5554, 873],
    },
    ChromeTextSegment::Quad {
        control: [5554, 725],
        to: [5645, 633],
    },
    ChromeTextSegment::Quad {
        control: [5736, 541],
        to: [5865, 541],
    },
    ChromeTextSegment::Quad {
        control: [6001, 541],
        to: [6085, 622],
    },
    ChromeTextSegment::Quad {
        control: [6167, 704],
        to: [6167, 830],
    },
    ChromeTextSegment::Line([6155, 912]),
    ChromeTextSegment::Line([5706, 912]),
    ChromeTextSegment::Quad {
        control: [5711, 987],
        to: [5758, 1028],
    },
];

const BACK_OVER_CONTOUR_12: [ChromeTextSegment; 15] = [
    ChromeTextSegment::Line([6417, 612]),
    ChromeTextSegment::Quad {
        control: [6477, 541],
        to: [6593, 541],
    },
    ChromeTextSegment::Quad {
        control: [6705, 541],
        to: [6770, 608],
    },
    ChromeTextSegment::Quad {
        control: [6834, 675],
        to: [6834, 796],
    },
    ChromeTextSegment::Line([6834, 1180]),
    ChromeTextSegment::Line([6688, 1180]),
    ChromeTextSegment::Line([6688, 818]),
    ChromeTextSegment::Quad {
        control: [6688, 737],
        to: [6657, 701],
    },
    ChromeTextSegment::Quad {
        control: [6627, 663],
        to: [6557, 663],
    },
    ChromeTextSegment::Quad {
        control: [6525, 663],
        to: [6489, 682],
    },
    ChromeTextSegment::Quad {
        control: [6451, 700],
        to: [6431, 726],
    },
    ChromeTextSegment::Line([6431, 1180]),
    ChromeTextSegment::Line([6285, 1180]),
    ChromeTextSegment::Line([6285, 553]),
    ChromeTextSegment::Line([6390, 553]),
];

const BACK_OVER_CONTOUR_13: [ChromeTextSegment; 14] = [
    ChromeTextSegment::Line([7545, 1181]),
    ChromeTextSegment::Line([7399, 1181]),
    ChromeTextSegment::Line([7399, 1128]),
    ChromeTextSegment::Quad {
        control: [7367, 1154],
        to: [7313, 1173],
    },
    ChromeTextSegment::Quad {
        control: [7258, 1192],
        to: [7213, 1192],
    },
    ChromeTextSegment::Quad {
        control: [6995, 1192],
        to: [6995, 960],
    },
    ChromeTextSegment::Line([6995, 553]),
    ChromeTextSegment::Line([7141, 553]),
    ChromeTextSegment::Line([7141, 949]),
    ChromeTextSegment::Quad {
        control: [7141, 1070],
        to: [7249, 1070],
    },
    ChromeTextSegment::Quad {
        control: [7299, 1070],
        to: [7342, 1044],
    },
    ChromeTextSegment::Quad {
        control: [7385, 1018],
        to: [7399, 984],
    },
    ChromeTextSegment::Line([7399, 553]),
    ChromeTextSegment::Line([7545, 553]),
];

const BACK_OVER_CONTOURS: [ChromeTextContour; 14] = [
    ChromeTextContour {
        start: [214, 284],
        segments: &BACK_OVER_CONTOUR_0,
    },
    ChromeTextContour {
        start: [214, 702],
        segments: &BACK_OVER_CONTOUR_1,
    },
    ChromeTextContour {
        start: [984, 541],
        segments: &BACK_OVER_CONTOUR_2,
    },
    ChromeTextContour {
        start: [1059, 878],
        segments: &BACK_OVER_CONTOUR_3,
    },
    ChromeTextContour {
        start: [1579, 719],
        segments: &BACK_OVER_CONTOUR_4,
    },
    ChromeTextContour {
        start: [2355, 791],
        segments: &BACK_OVER_CONTOUR_5,
    },
    ChromeTextContour {
        start: [3072, 553],
        segments: &BACK_OVER_CONTOUR_6,
    },
    ChromeTextContour {
        start: [3636, 865],
        segments: &BACK_OVER_CONTOUR_7,
    },
    ChromeTextContour {
        start: [3567, 632],
        segments: &BACK_OVER_CONTOUR_8,
    },
    ChromeTextContour {
        start: [5216, 541],
        segments: &BACK_OVER_CONTOUR_9,
    },
    ChromeTextContour {
        start: [5712, 801],
        segments: &BACK_OVER_CONTOUR_10,
    },
    ChromeTextContour {
        start: [5758, 1028],
        segments: &BACK_OVER_CONTOUR_11,
    },
    ChromeTextContour {
        start: [6390, 553],
        segments: &BACK_OVER_CONTOUR_12,
    },
    ChromeTextContour {
        start: [7545, 553],
        segments: &BACK_OVER_CONTOUR_13,
    },
];

pub const BACK_OVER: ChromeTextDefinition = ChromeTextDefinition {
    text: "back to menu",
    define_text_id: 121,
    font_ids: &BACK_OVER_FONT_IDS,
    color_rgb: [204, 204, 204],
    bounds_centipx: [65, 8150, 285, 1190],
    contours: &BACK_OVER_CONTOURS,
};

const BACK_DOWN_FONT_IDS: [u16; 1] = [54];

const BACK_DOWN_CONTOUR_0: [ChromeTextSegment; 11] = [
    ChromeTextSegment::Line([214, 576]),
    ChromeTextSegment::Quad {
        control: [273, 541],
        to: [342, 541],
    },
    ChromeTextSegment::Quad {
        control: [479, 541],
        to: [564, 628],
    },
    ChromeTextSegment::Quad {
        control: [648, 715],
        to: [648, 851],
    },
    ChromeTextSegment::Quad {
        control: [648, 1009],
        to: [564, 1100],
    },
    ChromeTextSegment::Quad {
        control: [478, 1192],
        to: [333, 1192],
    },
    ChromeTextSegment::Quad {
        control: [246, 1192],
        to: [183, 1146],
    },
    ChromeTextSegment::Line([150, 1192]),
    ChromeTextSegment::Line([68, 1192]),
    ChromeTextSegment::Line([68, 319]),
    ChromeTextSegment::Line([214, 284]),
];

const BACK_DOWN_CONTOUR_1: [ChromeTextSegment; 6] = [
    ChromeTextSegment::Line([214, 1030]),
    ChromeTextSegment::Quad {
        control: [246, 1068],
        to: [301, 1068],
    },
    ChromeTextSegment::Quad {
        control: [408, 1068],
        to: [452, 1017],
    },
    ChromeTextSegment::Quad {
        control: [496, 967],
        to: [496, 857],
    },
    ChromeTextSegment::Quad {
        control: [496, 666],
        to: [313, 666],
    },
    ChromeTextSegment::Quad {
        control: [248, 666],
        to: [214, 702],
    },
];

const BACK_DOWN_CONTOUR_2: [ChromeTextSegment; 19] = [
    ChromeTextSegment::Quad {
        control: [1127, 541],
        to: [1193, 607],
    },
    ChromeTextSegment::Quad {
        control: [1260, 671],
        to: [1260, 854],
    },
    ChromeTextSegment::Line([1260, 988]),
    ChromeTextSegment::Quad {
        control: [1260, 1113],
        to: [1311, 1146],
    },
    ChromeTextSegment::Quad {
        control: [1293, 1178],
        to: [1271, 1185],
    },
    ChromeTextSegment::Line([1219, 1192]),
    ChromeTextSegment::Quad {
        control: [1188, 1192],
        to: [1163, 1168],
    },
    ChromeTextSegment::Quad {
        control: [1137, 1145],
        to: [1129, 1118],
    },
    ChromeTextSegment::Quad {
        control: [1109, 1151],
        to: [1060, 1172],
    },
    ChromeTextSegment::Quad {
        control: [1009, 1192],
        to: [955, 1192],
    },
    ChromeTextSegment::Quad {
        control: [855, 1192],
        to: [796, 1141],
    },
    ChromeTextSegment::Quad {
        control: [739, 1091],
        to: [739, 997],
    },
    ChromeTextSegment::Quad {
        control: [739, 888],
        to: [821, 827],
    },
    ChromeTextSegment::Quad {
        control: [902, 765],
        to: [1053, 765],
    },
    ChromeTextSegment::Line([1114, 775]),
    ChromeTextSegment::Quad {
        control: [1114, 663],
        to: [973, 663],
    },
    ChromeTextSegment::Quad {
        control: [891, 663],
        to: [835, 691],
    },
    ChromeTextSegment::Line([803, 578]),
    ChromeTextSegment::Quad {
        control: [879, 541],
        to: [984, 541],
    },
];

const BACK_DOWN_CONTOUR_3: [ChromeTextSegment; 5] = [
    ChromeTextSegment::Quad {
        control: [885, 878],
        to: [885, 991],
    },
    ChromeTextSegment::Quad {
        control: [885, 1076],
        to: [982, 1076],
    },
    ChromeTextSegment::Quad {
        control: [1115, 1076],
        to: [1115, 943],
    },
    ChromeTextSegment::Line([1115, 885]),
    ChromeTextSegment::Line([1059, 878]),
];

const BACK_DOWN_CONTOUR_4: [ChromeTextSegment; 14] = [
    ChromeTextSegment::Quad {
        control: [1531, 775],
        to: [1531, 872],
    },
    ChromeTextSegment::Quad {
        control: [1531, 1070],
        to: [1719, 1070],
    },
    ChromeTextSegment::Quad {
        control: [1801, 1070],
        to: [1863, 1016],
    },
    ChromeTextSegment::Line([1917, 1131]),
    ChromeTextSegment::Quad {
        control: [1852, 1171],
        to: [1805, 1181],
    },
    ChromeTextSegment::Line([1692, 1192]),
    ChromeTextSegment::Quad {
        control: [1546, 1192],
        to: [1463, 1107],
    },
    ChromeTextSegment::Quad {
        control: [1379, 1023],
        to: [1379, 872],
    },
    ChromeTextSegment::Quad {
        control: [1379, 724],
        to: [1470, 633],
    },
    ChromeTextSegment::Quad {
        control: [1563, 541],
        to: [1721, 541],
    },
    ChromeTextSegment::Quad {
        control: [1831, 541],
        to: [1912, 602],
    },
    ChromeTextSegment::Line([1849, 712]),
    ChromeTextSegment::Quad {
        control: [1797, 663],
        to: [1710, 663],
    },
    ChromeTextSegment::Quad {
        control: [1627, 663],
        to: [1579, 719],
    },
];

const BACK_DOWN_CONTOUR_5: [ChromeTextSegment; 12] = [
    ChromeTextSegment::Line([2610, 1180]),
    ChromeTextSegment::Line([2435, 1180]),
    ChromeTextSegment::Line([2251, 896]),
    ChromeTextSegment::Line([2181, 970]),
    ChromeTextSegment::Line([2181, 1180]),
    ChromeTextSegment::Line([2035, 1180]),
    ChromeTextSegment::Line([2035, 319]),
    ChromeTextSegment::Line([2181, 284]),
    ChromeTextSegment::Line([2181, 793]),
    ChromeTextSegment::Line([2390, 553]),
    ChromeTextSegment::Line([2567, 553]),
    ChromeTextSegment::Line([2355, 791]),
];

const BACK_DOWN_CONTOUR_6: [ChromeTextSegment; 18] = [
    ChromeTextSegment::Line([3072, 425]),
    ChromeTextSegment::Line([3218, 371]),
    ChromeTextSegment::Line([3218, 553]),
    ChromeTextSegment::Line([3391, 553]),
    ChromeTextSegment::Line([3391, 670]),
    ChromeTextSegment::Line([3218, 670]),
    ChromeTextSegment::Line([3218, 946]),
    ChromeTextSegment::Quad {
        control: [3218, 1012],
        to: [3240, 1041],
    },
    ChromeTextSegment::Quad {
        control: [3261, 1069],
        to: [3313, 1069],
    },
    ChromeTextSegment::Quad {
        control: [3366, 1069],
        to: [3412, 1039],
    },
    ChromeTextSegment::Line([3412, 1174]),
    ChromeTextSegment::Quad {
        control: [3360, 1192],
        to: [3266, 1192],
    },
    ChromeTextSegment::Quad {
        control: [3173, 1192],
        to: [3122, 1139],
    },
    ChromeTextSegment::Quad {
        control: [3072, 1086],
        to: [3072, 988],
    },
    ChromeTextSegment::Line([3072, 670]),
    ChromeTextSegment::Line([2999, 670]),
    ChromeTextSegment::Line([2999, 553]),
    ChromeTextSegment::Line([3072, 553]),
];

const BACK_DOWN_CONTOUR_7: [ChromeTextSegment; 6] = [
    ChromeTextSegment::Quad {
        control: [3636, 1072],
        to: [3785, 1072],
    },
    ChromeTextSegment::Quad {
        control: [3854, 1072],
        to: [3894, 1018],
    },
    ChromeTextSegment::Quad {
        control: [3934, 964],
        to: [3934, 865],
    },
    ChromeTextSegment::Quad {
        control: [3934, 661],
        to: [3785, 661],
    },
    ChromeTextSegment::Quad {
        control: [3717, 661],
        to: [3676, 715],
    },
    ChromeTextSegment::Quad {
        control: [3636, 769],
        to: [3636, 865],
    },
];

const BACK_DOWN_CONTOUR_8: [ChromeTextSegment; 8] = [
    ChromeTextSegment::Quad {
        control: [3649, 541],
        to: [3785, 541],
    },
    ChromeTextSegment::Quad {
        control: [3928, 541],
        to: [4008, 628],
    },
    ChromeTextSegment::Quad {
        control: [4086, 715],
        to: [4086, 865],
    },
    ChromeTextSegment::Quad {
        control: [4086, 1015],
        to: [4006, 1104],
    },
    ChromeTextSegment::Quad {
        control: [3925, 1192],
        to: [3785, 1192],
    },
    ChromeTextSegment::Quad {
        control: [3642, 1192],
        to: [3563, 1103],
    },
    ChromeTextSegment::Quad {
        control: [3484, 1014],
        to: [3484, 865],
    },
    ChromeTextSegment::Quad {
        control: [3484, 722],
        to: [3567, 632],
    },
];

const BACK_DOWN_CONTOUR_9: [ChromeTextSegment; 24] = [
    ChromeTextSegment::Quad {
        control: [5321, 541],
        to: [5379, 601],
    },
    ChromeTextSegment::Quad {
        control: [5438, 661],
        to: [5438, 766],
    },
    ChromeTextSegment::Line([5438, 1180]),
    ChromeTextSegment::Line([5291, 1180]),
    ChromeTextSegment::Line([5291, 787]),
    ChromeTextSegment::Quad {
        control: [5291, 663],
        to: [5180, 663],
    },
    ChromeTextSegment::Quad {
        control: [5147, 663],
        to: [5117, 683],
    },
    ChromeTextSegment::Quad {
        control: [5086, 702],
        to: [5074, 728],
    },
    ChromeTextSegment::Line([5074, 1180]),
    ChromeTextSegment::Line([4928, 1180]),
    ChromeTextSegment::Line([4928, 764]),
    ChromeTextSegment::Quad {
        control: [4928, 718],
        to: [4899, 691],
    },
    ChromeTextSegment::Quad {
        control: [4871, 663],
        to: [4818, 663],
    },
    ChromeTextSegment::Quad {
        control: [4791, 663],
        to: [4758, 684],
    },
    ChromeTextSegment::Quad {
        control: [4725, 704],
        to: [4711, 729],
    },
    ChromeTextSegment::Line([4711, 1180]),
    ChromeTextSegment::Line([4565, 1180]),
    ChromeTextSegment::Line([4565, 553]),
    ChromeTextSegment::Line([4665, 553]),
    ChromeTextSegment::Line([4695, 608]),
    ChromeTextSegment::Quad {
        control: [4756, 541],
        to: [4853, 541],
    },
    ChromeTextSegment::Quad {
        control: [4977, 541],
        to: [5040, 609],
    },
    ChromeTextSegment::Quad {
        control: [5069, 578],
        to: [5118, 560],
    },
    ChromeTextSegment::Quad {
        control: [5167, 541],
        to: [5216, 541],
    },
];

const BACK_DOWN_CONTOUR_10: [ChromeTextSegment; 3] = [
    ChromeTextSegment::Line([6020, 801]),
    ChromeTextSegment::Quad {
        control: [6005, 663],
        to: [5868, 663],
    },
    ChromeTextSegment::Quad {
        control: [5742, 663],
        to: [5712, 801],
    },
];

const BACK_DOWN_CONTOUR_11: [ChromeTextSegment; 13] = [
    ChromeTextSegment::Quad {
        control: [5806, 1070],
        to: [5885, 1070],
    },
    ChromeTextSegment::Quad {
        control: [5986, 1070],
        to: [6038, 1017],
    },
    ChromeTextSegment::Line([6095, 1130]),
    ChromeTextSegment::Quad {
        control: [6018, 1192],
        to: [5864, 1192],
    },
    ChromeTextSegment::Quad {
        control: [5720, 1192],
        to: [5637, 1109],
    },
    ChromeTextSegment::Quad {
        control: [5554, 1024],
        to: [5554, 873],
    },
    ChromeTextSegment::Quad {
        control: [5554, 725],
        to: [5645, 633],
    },
    ChromeTextSegment::Quad {
        control: [5736, 541],
        to: [5865, 541],
    },
    ChromeTextSegment::Quad {
        control: [6001, 541],
        to: [6085, 622],
    },
    ChromeTextSegment::Quad {
        control: [6167, 704],
        to: [6167, 830],
    },
    ChromeTextSegment::Line([6155, 912]),
    ChromeTextSegment::Line([5706, 912]),
    ChromeTextSegment::Quad {
        control: [5711, 987],
        to: [5758, 1028],
    },
];

const BACK_DOWN_CONTOUR_12: [ChromeTextSegment; 15] = [
    ChromeTextSegment::Line([6417, 612]),
    ChromeTextSegment::Quad {
        control: [6477, 541],
        to: [6593, 541],
    },
    ChromeTextSegment::Quad {
        control: [6705, 541],
        to: [6770, 608],
    },
    ChromeTextSegment::Quad {
        control: [6834, 675],
        to: [6834, 796],
    },
    ChromeTextSegment::Line([6834, 1180]),
    ChromeTextSegment::Line([6688, 1180]),
    ChromeTextSegment::Line([6688, 818]),
    ChromeTextSegment::Quad {
        control: [6688, 737],
        to: [6657, 701],
    },
    ChromeTextSegment::Quad {
        control: [6627, 663],
        to: [6557, 663],
    },
    ChromeTextSegment::Quad {
        control: [6525, 663],
        to: [6489, 682],
    },
    ChromeTextSegment::Quad {
        control: [6451, 700],
        to: [6431, 726],
    },
    ChromeTextSegment::Line([6431, 1180]),
    ChromeTextSegment::Line([6285, 1180]),
    ChromeTextSegment::Line([6285, 553]),
    ChromeTextSegment::Line([6390, 553]),
];

const BACK_DOWN_CONTOUR_13: [ChromeTextSegment; 14] = [
    ChromeTextSegment::Line([7545, 1181]),
    ChromeTextSegment::Line([7399, 1181]),
    ChromeTextSegment::Line([7399, 1128]),
    ChromeTextSegment::Quad {
        control: [7367, 1154],
        to: [7313, 1173],
    },
    ChromeTextSegment::Quad {
        control: [7258, 1192],
        to: [7213, 1192],
    },
    ChromeTextSegment::Quad {
        control: [6995, 1192],
        to: [6995, 960],
    },
    ChromeTextSegment::Line([6995, 553]),
    ChromeTextSegment::Line([7141, 553]),
    ChromeTextSegment::Line([7141, 949]),
    ChromeTextSegment::Quad {
        control: [7141, 1070],
        to: [7249, 1070],
    },
    ChromeTextSegment::Quad {
        control: [7299, 1070],
        to: [7342, 1044],
    },
    ChromeTextSegment::Quad {
        control: [7385, 1018],
        to: [7399, 984],
    },
    ChromeTextSegment::Line([7399, 553]),
    ChromeTextSegment::Line([7545, 553]),
];

const BACK_DOWN_CONTOURS: [ChromeTextContour; 14] = [
    ChromeTextContour {
        start: [214, 284],
        segments: &BACK_DOWN_CONTOUR_0,
    },
    ChromeTextContour {
        start: [214, 702],
        segments: &BACK_DOWN_CONTOUR_1,
    },
    ChromeTextContour {
        start: [984, 541],
        segments: &BACK_DOWN_CONTOUR_2,
    },
    ChromeTextContour {
        start: [1059, 878],
        segments: &BACK_DOWN_CONTOUR_3,
    },
    ChromeTextContour {
        start: [1579, 719],
        segments: &BACK_DOWN_CONTOUR_4,
    },
    ChromeTextContour {
        start: [2355, 791],
        segments: &BACK_DOWN_CONTOUR_5,
    },
    ChromeTextContour {
        start: [3072, 553],
        segments: &BACK_DOWN_CONTOUR_6,
    },
    ChromeTextContour {
        start: [3636, 865],
        segments: &BACK_DOWN_CONTOUR_7,
    },
    ChromeTextContour {
        start: [3567, 632],
        segments: &BACK_DOWN_CONTOUR_8,
    },
    ChromeTextContour {
        start: [5216, 541],
        segments: &BACK_DOWN_CONTOUR_9,
    },
    ChromeTextContour {
        start: [5712, 801],
        segments: &BACK_DOWN_CONTOUR_10,
    },
    ChromeTextContour {
        start: [5758, 1028],
        segments: &BACK_DOWN_CONTOUR_11,
    },
    ChromeTextContour {
        start: [6390, 553],
        segments: &BACK_DOWN_CONTOUR_12,
    },
    ChromeTextContour {
        start: [7545, 553],
        segments: &BACK_DOWN_CONTOUR_13,
    },
];

pub const BACK_DOWN: ChromeTextDefinition = ChromeTextDefinition {
    text: "back to menu",
    define_text_id: 122,
    font_ids: &BACK_DOWN_FONT_IDS,
    color_rgb: [255, 255, 255],
    bounds_centipx: [65, 8150, 285, 1190],
    contours: &BACK_DOWN_CONTOURS,
};

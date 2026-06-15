// Generated from gravity_arcade.swf DefineShape 35 via reference/extract_shapes.py --contours.
// Coordinates are stored in SWF twips; divide by 20 for stage-local pixels.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape35Segment {
    Line([i16; 2]),
    Quad { control: [i16; 2], to: [i16; 2] },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Shape35Contour {
    pub(super) start: [i16; 2],
    pub(super) segments: &'static [Shape35Segment],
}

pub const BOUNDS_TWIPS: [i16; 4] = [-2286, 2342, -1007, 1013];
pub const FILL_RGB: [u8; 3] = [102, 204, 0];

const CONTOUR_0: [Shape35Segment; 13] = [
    Shape35Segment::Quad {
        control: [612, -946],
        to: [612, -907],
    },
    Shape35Segment::Quad {
        control: [612, -870],
        to: [600, -795],
    },
    Shape35Segment::Quad {
        control: [595, -726],
        to: [595, -648],
    },
    Shape35Segment::Line([603, -405]),
    Shape35Segment::Line([612, -163]),
    Shape35Segment::Line([606, -18]),
    Shape35Segment::Line([578, -3]),
    Shape35Segment::Quad {
        control: [572, -123],
        to: [562, -656],
    },
    Shape35Segment::Quad {
        control: [561, -703],
        to: [569, -791],
    },
    Shape35Segment::Line([578, -922]),
    Shape35Segment::Line([578, -964]),
    Shape35Segment::Quad {
        control: [583, -980],
        to: [594, -980],
    },
    Shape35Segment::Line([602, -974]),
];

const CONTOUR_1: [Shape35Segment; 18] = [
    Shape35Segment::Quad {
        control: [321, -384],
        to: [374, -351],
    },
    Shape35Segment::Quad {
        control: [427, -318],
        to: [454, -261],
    },
    Shape35Segment::Line([457, -248]),
    Shape35Segment::Line([460, -237]),
    Shape35Segment::Line([455, -225]),
    Shape35Segment::Quad {
        control: [248, -187],
        to: [24, -103],
    },
    Shape35Segment::Quad {
        control: [28, -77],
        to: [59, -48],
    },
    Shape35Segment::Line([160, -41]),
    Shape35Segment::Quad {
        control: [362, -41],
        to: [479, -142],
    },
    Shape35Segment::Line([486, -143]),
    Shape35Segment::Line([505, -130]),
    Shape35Segment::Quad {
        control: [384, 3],
        to: [132, 18],
    },
    Shape35Segment::Line([67, 2]),
    Shape35Segment::Quad {
        control: [31, -9],
        to: [10, -31],
    },
    Shape35Segment::Quad {
        control: [-22, -74],
        to: [-22, -138],
    },
    Shape35Segment::Line([-15, -194]),
    Shape35Segment::Quad {
        control: [95, -380],
        to: [230, -380],
    },
    Shape35Segment::Line([258, -378]),
];

const CONTOUR_2: [Shape35Segment; 7] = [
    Shape35Segment::Line([164, -309]),
    Shape35Segment::Quad {
        control: [109, -297],
        to: [69, -248],
    },
    Shape35Segment::Quad {
        control: [33, -205],
        to: [20, -152],
    },
    Shape35Segment::Quad {
        control: [74, -174],
        to: [112, -177],
    },
    Shape35Segment::Line([255, -229]),
    Shape35Segment::Quad {
        control: [341, -261],
        to: [397, -274],
    },
    Shape35Segment::Quad {
        control: [364, -325],
        to: [280, -325],
    },
];

const CONTOUR_3: [Shape35Segment; 8] = [
    Shape35Segment::Line([685, -430]),
    Shape35Segment::Quad {
        control: [663, -441],
        to: [663, -466],
    },
    Shape35Segment::Quad {
        control: [663, -479],
        to: [672, -491],
    },
    Shape35Segment::Quad {
        control: [701, -531],
        to: [757, -531],
    },
    Shape35Segment::Quad {
        control: [817, -531],
        to: [836, -486],
    },
    Shape35Segment::Quad {
        control: [840, -447],
        to: [814, -423],
    },
    Shape35Segment::Quad {
        control: [796, -413],
        to: [772, -413],
    },
    Shape35Segment::Line([728, -419]),
];

const CONTOUR_4: [Shape35Segment; 9] = [
    Shape35Segment::Quad {
        control: [799, -491],
        to: [792, -491],
    },
    Shape35Segment::Line([774, -485]),
    Shape35Segment::Line([750, -480]),
    Shape35Segment::Quad {
        control: [737, -480],
        to: [729, -492],
    },
    Shape35Segment::Line([711, -481]),
    Shape35Segment::Quad {
        control: [702, -474],
        to: [702, -467],
    },
    Shape35Segment::Quad {
        control: [742, -453],
        to: [760, -453],
    },
    Shape35Segment::Quad {
        control: [781, -453],
        to: [797, -466],
    },
    Shape35Segment::Line([799, -480]),
];

const CONTOUR_5: [Shape35Segment; 11] = [
    Shape35Segment::Quad {
        control: [778, -309],
        to: [778, -217],
    },
    Shape35Segment::Line([775, -152]),
    Shape35Segment::Line([772, -113]),
    Shape35Segment::Line([772, -74]),
    Shape35Segment::Quad {
        control: [774, -25],
        to: [755, -5],
    },
    Shape35Segment::Quad {
        control: [737, -46],
        to: [737, -100],
    },
    Shape35Segment::Line([741, -163]),
    Shape35Segment::Line([745, -227]),
    Shape35Segment::Line([740, -278]),
    Shape35Segment::Line([737, -319]),
    Shape35Segment::Quad {
        control: [738, -343],
        to: [753, -355],
    },
];

const CONTOUR_6: [Shape35Segment; 24] = [
    Shape35Segment::Line([1512, -974]),
    Shape35Segment::Quad {
        control: [1531, -626],
        to: [1531, -520],
    },
    Shape35Segment::Quad {
        control: [1531, -404],
        to: [1522, -310],
    },
    Shape35Segment::Quad {
        control: [1622, -387],
        to: [1703, -387],
    },
    Shape35Segment::Quad {
        control: [1735, -387],
        to: [1765, -376],
    },
    Shape35Segment::Quad {
        control: [1821, -362],
        to: [1865, -308],
    },
    Shape35Segment::Quad {
        control: [1885, -253],
        to: [1892, -167],
    },
    Shape35Segment::Line([1902, -23]),
    Shape35Segment::Line([1885, -9]),
    Shape35Segment::Quad {
        control: [1863, -51],
        to: [1857, -123],
    },
    Shape35Segment::Line([1847, -244]),
    Shape35Segment::Quad {
        control: [1839, -317],
        to: [1723, -353],
    },
    Shape35Segment::Quad {
        control: [1544, -323],
        to: [1521, -230],
    },
    Shape35Segment::Quad {
        control: [1516, -203],
        to: [1516, -158],
    },
    Shape35Segment::Line([1516, -126]),
    Shape35Segment::Line([1516, -94]),
    Shape35Segment::Quad {
        control: [1516, -32],
        to: [1502, 0],
    },
    Shape35Segment::Quad {
        control: [1470, -12],
        to: [1470, -39],
    },
    Shape35Segment::Line([1476, -72]),
    Shape35Segment::Line([1481, -106]),
    Shape35Segment::Line([1481, -118]),
    Shape35Segment::Line([1490, -390]),
    Shape35Segment::Line([1496, -665]),
    Shape35Segment::Quad {
        control: [1496, -817],
        to: [1486, -952],
    },
];

const CONTOUR_7: [Shape35Segment; 29] = [
    Shape35Segment::Quad {
        control: [888, -212],
        to: [911, -255],
    },
    Shape35Segment::Quad {
        control: [966, -355],
        to: [1111, -355],
    },
    Shape35Segment::Line([1287, -334]),
    Shape35Segment::Quad {
        control: [1344, -325],
        to: [1393, -284],
    },
    Shape35Segment::Line([1395, -253]),
    Shape35Segment::Quad {
        control: [1395, -201],
        to: [1370, -154],
    },
    Shape35Segment::Quad {
        control: [1346, -106],
        to: [1305, -83],
    },
    Shape35Segment::Line([1227, -37]),
    Shape35Segment::Quad {
        control: [1183, -16],
        to: [1138, -20],
    },
    Shape35Segment::Line([1138, 5]),
    Shape35Segment::Line([1297, 14]),
    Shape35Segment::Quad {
        control: [1464, 27],
        to: [1566, 72],
    },
    Shape35Segment::Quad {
        control: [1705, 133],
        to: [1753, 259],
    },
    Shape35Segment::Line([1755, 265]),
    Shape35Segment::Quad {
        control: [1779, 380],
        to: [1779, 476],
    },
    Shape35Segment::Quad {
        control: [1779, 702],
        to: [1639, 822],
    },
    Shape35Segment::Quad {
        control: [1527, 917],
        to: [1384, 965],
    },
    Shape35Segment::Quad {
        control: [1314, 989],
        to: [1241, 1001],
    },
    Shape35Segment::Line([1095, 1013]),
    Shape35Segment::Quad {
        control: [984, 1013],
        to: [878, 988],
    },
    Shape35Segment::Quad {
        control: [759, 929],
        to: [680, 833],
    },
    Shape35Segment::Quad {
        control: [589, 722],
        to: [592, 602],
    },
    Shape35Segment::Quad {
        control: [618, 415],
        to: [722, 270],
    },
    Shape35Segment::Quad {
        control: [830, 118],
        to: [998, 52],
    },
    Shape35Segment::Quad {
        control: [1048, 47],
        to: [1084, 16],
    },
    Shape35Segment::Quad {
        control: [1084, 13],
        to: [1095, 4],
    },
    Shape35Segment::Line([1101, -6]),
    Shape35Segment::Quad {
        control: [976, 1],
        to: [918, -65],
    },
    Shape35Segment::Quad {
        control: [888, -98],
        to: [888, -158],
    },
];

const CONTOUR_8: [Shape35Segment; 9] = [
    Shape35Segment::Line([1225, -79]),
    Shape35Segment::Line([1319, -137]),
    Shape35Segment::Quad {
        control: [1367, -177],
        to: [1368, -233],
    },
    Shape35Segment::Line([1359, -268]),
    Shape35Segment::Quad {
        control: [1256, -320],
        to: [1156, -320],
    },
    Shape35Segment::Quad {
        control: [1071, -320],
        to: [1002, -282],
    },
    Shape35Segment::Quad {
        control: [947, -234],
        to: [947, -102],
    },
    Shape35Segment::Quad {
        control: [990, -46],
        to: [1075, -53],
    },
    Shape35Segment::Line([1153, -66]),
];

const CONTOUR_9: [Shape35Segment; 29] = [
    Shape35Segment::Line([2116, -454]),
    Shape35Segment::Quad {
        control: [2115, -428],
        to: [2124, -416],
    },
    Shape35Segment::Quad {
        control: [2134, -405],
        to: [2161, -407],
    },
    Shape35Segment::Line([2252, -417]),
    Shape35Segment::Quad {
        control: [2306, -419],
        to: [2342, -397],
    },
    Shape35Segment::Quad {
        control: [2307, -378],
        to: [2230, -377],
    },
    Shape35Segment::Line([2174, -375]),
    Shape35Segment::Line([2121, -363]),
    Shape35Segment::Quad {
        control: [2118, -290],
        to: [2129, -187],
    },
    Shape35Segment::Line([2146, -14]),
    Shape35Segment::Quad {
        control: [2140, -14],
        to: [2138, -10],
    },
    Shape35Segment::Line([2132, -3]),
    Shape35Segment::Quad {
        control: [2102, -31],
        to: [2102, -86],
    },
    Shape35Segment::Line([2102, -137]),
    Shape35Segment::Line([2101, -178]),
    Shape35Segment::Quad {
        control: [2094, -196],
        to: [2092, -232],
    },
    Shape35Segment::Line([2090, -287]),
    Shape35Segment::Quad {
        control: [2086, -353],
        to: [2049, -365],
    },
    Shape35Segment::Quad {
        control: [2031, -354],
        to: [2008, -354],
    },
    Shape35Segment::Line([1978, -357]),
    Shape35Segment::Line([1946, -360]),
    Shape35Segment::Line([1919, -370]),
    Shape35Segment::Quad {
        control: [1918, -382],
        to: [1925, -389],
    },
    Shape35Segment::Line([1941, -400]),
    Shape35Segment::Line([2014, -407]),
    Shape35Segment::Line([2085, -423]),
    Shape35Segment::Line([2085, -480]),
    Shape35Segment::Quad {
        control: [2085, -514],
        to: [2094, -538],
    },
    Shape35Segment::Quad {
        control: [2113, -524],
        to: [2117, -498],
    },
];

const CONTOUR_10: [Shape35Segment; 19] = [
    Shape35Segment::Quad {
        control: [1665, 171],
        to: [1566, 115],
    },
    Shape35Segment::Quad {
        control: [1448, 48],
        to: [1296, 48],
    },
    Shape35Segment::Line([1207, 55]),
    Shape35Segment::Line([1192, 55]),
    Shape35Segment::Quad {
        control: [1169, 55],
        to: [1135, 81],
    },
    Shape35Segment::Quad {
        control: [1101, 108],
        to: [1085, 108],
    },
    Shape35Segment::Quad {
        control: [1071, 108],
        to: [1056, 94],
    },
    Shape35Segment::Quad {
        control: [985, 103],
        to: [916, 156],
    },
    Shape35Segment::Line([799, 256]),
    Shape35Segment::Quad {
        control: [648, 350],
        to: [648, 591],
    },
    Shape35Segment::Quad {
        control: [648, 662],
        to: [662, 737],
    },
    Shape35Segment::Line([715, 833]),
    Shape35Segment::Quad {
        control: [749, 886],
        to: [799, 896],
    },
    Shape35Segment::Quad {
        control: [919, 973],
        to: [1071, 973],
    },
    Shape35Segment::Quad {
        control: [1150, 973],
        to: [1241, 952],
    },
    Shape35Segment::Quad {
        control: [1381, 919],
        to: [1547, 835],
    },
    Shape35Segment::Quad {
        control: [1746, 734],
        to: [1746, 403],
    },
    Shape35Segment::Line([1743, 321]),
    Shape35Segment::Line([1716, 259]),
];

const CONTOUR_11: [Shape35Segment; 39] = [
    Shape35Segment::Quad {
        control: [418, 248],
        to: [418, 256],
    },
    Shape35Segment::Line([422, 264]),
    Shape35Segment::Line([438, 272]),
    Shape35Segment::Line([453, 279]),
    Shape35Segment::Line([476, 293]),
    Shape35Segment::Quad {
        control: [485, 301],
        to: [485, 312],
    },
    Shape35Segment::Quad {
        control: [485, 327],
        to: [473, 338],
    },
    Shape35Segment::Quad {
        control: [462, 349],
        to: [439, 349],
    },
    Shape35Segment::Line([417, 344]),
    Shape35Segment::Line([411, 346]),
    Shape35Segment::Quad {
        control: [404, 346],
        to: [400, 341],
    },
    Shape35Segment::Line([395, 330]),
    Shape35Segment::Line([400, 319]),
    Shape35Segment::Quad {
        control: [404, 314],
        to: [411, 314],
    },
    Shape35Segment::Line([422, 319]),
    Shape35Segment::Line([427, 330]),
    Shape35Segment::Line([427, 331]),
    Shape35Segment::Line([439, 333]),
    Shape35Segment::Line([453, 331]),
    Shape35Segment::Line([462, 325]),
    Shape35Segment::Line([466, 315]),
    Shape35Segment::Line([457, 301]),
    Shape35Segment::Line([432, 289]),
    Shape35Segment::Line([417, 282]),
    Shape35Segment::Line([406, 273]),
    Shape35Segment::Quad {
        control: [401, 267],
        to: [401, 258],
    },
    Shape35Segment::Line([405, 242]),
    Shape35Segment::Line([418, 230]),
    Shape35Segment::Line([439, 224]),
    Shape35Segment::Line([459, 228]),
    Shape35Segment::Quad {
        control: [463, 225],
        to: [467, 225],
    },
    Shape35Segment::Line([479, 230]),
    Shape35Segment::Quad {
        control: [483, 234],
        to: [483, 241],
    },
    Shape35Segment::Line([479, 252]),
    Shape35Segment::Line([467, 257]),
    Shape35Segment::Quad {
        control: [461, 257],
        to: [457, 253],
    },
    Shape35Segment::Line([451, 242]),
    Shape35Segment::Line([441, 240]),
    Shape35Segment::Line([424, 244]),
];

const CONTOUR_12: [Shape35Segment; 28] = [
    Shape35Segment::Line([229, 262]),
    Shape35Segment::Line([233, 279]),
    Shape35Segment::Line([231, 288]),
    Shape35Segment::Line([228, 294]),
    Shape35Segment::Line([224, 297]),
    Shape35Segment::Line([147, 297]),
    Shape35Segment::Line([152, 315]),
    Shape35Segment::Line([166, 330]),
    Shape35Segment::Line([184, 335]),
    Shape35Segment::Line([199, 332]),
    Shape35Segment::Line([199, 330]),
    Shape35Segment::Quad {
        control: [199, 323],
        to: [204, 319],
    },
    Shape35Segment::Quad {
        control: [209, 314],
        to: [215, 314],
    },
    Shape35Segment::Quad {
        control: [222, 314],
        to: [227, 319],
    },
    Shape35Segment::Line([231, 330]),
    Shape35Segment::Line([226, 341]),
    Shape35Segment::Line([215, 346]),
    Shape35Segment::Line([208, 344]),
    Shape35Segment::Line([184, 349]),
    Shape35Segment::Quad {
        control: [165, 349],
        to: [153, 340],
    },
    Shape35Segment::Quad {
        control: [140, 331],
        to: [135, 318],
    },
    Shape35Segment::Quad {
        control: [129, 304],
        to: [129, 291],
    },
    Shape35Segment::Quad {
        control: [129, 280],
        to: [132, 269],
    },
    Shape35Segment::Line([142, 248]),
    Shape35Segment::Line([158, 232]),
    Shape35Segment::Quad {
        control: [168, 226],
        to: [180, 226],
    },
    Shape35Segment::Line([203, 231]),
    Shape35Segment::Line([220, 245]),
];

const CONTOUR_13: [Shape35Segment; 8] = [
    Shape35Segment::Line([152, 259]),
    Shape35Segment::Quad {
        control: [148, 268],
        to: [147, 281],
    },
    Shape35Segment::Line([214, 281]),
    Shape35Segment::Line([212, 265]),
    Shape35Segment::Line([205, 252]),
    Shape35Segment::Line([194, 243]),
    Shape35Segment::Line([180, 240]),
    Shape35Segment::Line([164, 245]),
];

const CONTOUR_14: [Shape35Segment; 4] = [
    Shape35Segment::Line([643, 571]),
    Shape35Segment::Quad {
        control: [634, 615],
        to: [640, 677],
    },
    Shape35Segment::Line([643, 639]),
    Shape35Segment::Line([646, 615]),
];

const CONTOUR_15: [Shape35Segment; 27] = [
    Shape35Segment::Line([-2286, -345]),
    Shape35Segment::Line([-2286, -361]),
    Shape35Segment::Quad {
        control: [-2286, -382],
        to: [-2267, -386],
    },
    Shape35Segment::Quad {
        control: [-2253, -373],
        to: [-2253, -355],
    },
    Shape35Segment::Line([-2258, -320]),
    Shape35Segment::Quad {
        control: [-2262, -302],
        to: [-2253, -292],
    },
    Shape35Segment::Quad {
        control: [-2182, -367],
        to: [-2073, -367],
    },
    Shape35Segment::Quad {
        control: [-2008, -367],
        to: [-1945, -340],
    },
    Shape35Segment::Quad {
        control: [-1900, -300],
        to: [-1888, -229],
    },
    Shape35Segment::Quad {
        control: [-1883, -205],
        to: [-1881, -103],
    },
    Shape35Segment::Line([-1872, -18]),
    Shape35Segment::Quad {
        control: [-1876, -6],
        to: [-1885, -6],
    },
    Shape35Segment::Quad {
        control: [-1894, -6],
        to: [-1904, -16],
    },
    Shape35Segment::Quad {
        control: [-1912, -48],
        to: [-1913, -108],
    },
    Shape35Segment::Quad {
        control: [-1915, -181],
        to: [-1919, -202],
    },
    Shape35Segment::Quad {
        control: [-1933, -316],
        to: [-2014, -334],
    },
    Shape35Segment::Line([-2149, -324]),
    Shape35Segment::Quad {
        control: [-2224, -305],
        to: [-2251, -241],
    },
    Shape35Segment::Line([-2253, -219]),
    Shape35Segment::Line([-2244, -143]),
    Shape35Segment::Line([-2236, -67]),
    Shape35Segment::Quad {
        control: [-2236, -24],
        to: [-2250, 8],
    },
    Shape35Segment::Line([-2272, 8]),
    Shape35Segment::Quad {
        control: [-2269, -18],
        to: [-2269, -47],
    },
    Shape35Segment::Line([-2277, -162]),
    Shape35Segment::Line([-2286, -276]),
    Shape35Segment::Line([-2286, -328]),
];

const CONTOUR_16: [Shape35Segment; 7] = [
    Shape35Segment::Quad {
        control: [-1753, -205],
        to: [-1766, -152],
    },
    Shape35Segment::Quad {
        control: [-1712, -174],
        to: [-1674, -177],
    },
    Shape35Segment::Line([-1531, -229]),
    Shape35Segment::Quad {
        control: [-1445, -261],
        to: [-1389, -274],
    },
    Shape35Segment::Quad {
        control: [-1422, -325],
        to: [-1506, -325],
    },
    Shape35Segment::Line([-1622, -309]),
    Shape35Segment::Quad {
        control: [-1677, -297],
        to: [-1717, -248],
    },
];

const CONTOUR_17: [Shape35Segment; 18] = [
    Shape35Segment::Line([-1801, -194]),
    Shape35Segment::Quad {
        control: [-1691, -380],
        to: [-1556, -380],
    },
    Shape35Segment::Line([-1528, -378]),
    Shape35Segment::Quad {
        control: [-1465, -384],
        to: [-1412, -351],
    },
    Shape35Segment::Quad {
        control: [-1359, -318],
        to: [-1332, -261],
    },
    Shape35Segment::Line([-1329, -248]),
    Shape35Segment::Line([-1326, -237]),
    Shape35Segment::Quad {
        control: [-1326, -230],
        to: [-1331, -225],
    },
    Shape35Segment::Quad {
        control: [-1538, -187],
        to: [-1762, -103],
    },
    Shape35Segment::Quad {
        control: [-1758, -77],
        to: [-1727, -48],
    },
    Shape35Segment::Line([-1626, -41]),
    Shape35Segment::Quad {
        control: [-1424, -41],
        to: [-1307, -142],
    },
    Shape35Segment::Line([-1300, -143]),
    Shape35Segment::Line([-1281, -130]),
    Shape35Segment::Quad {
        control: [-1402, 3],
        to: [-1654, 18],
    },
    Shape35Segment::Line([-1719, 2]),
    Shape35Segment::Quad {
        control: [-1755, -9],
        to: [-1776, -31],
    },
    Shape35Segment::Quad {
        control: [-1808, -74],
        to: [-1808, -138],
    },
];

const CONTOUR_18: [Shape35Segment; 11] = [
    Shape35Segment::Quad {
        control: [-1226, -206],
        to: [-1202, -260],
    },
    Shape35Segment::Quad {
        control: [-1178, -314],
        to: [-1134, -347],
    },
    Shape35Segment::Quad {
        control: [-1031, -423],
        to: [-907, -423],
    },
    Shape35Segment::Quad {
        control: [-787, -423],
        to: [-710, -343],
    },
    Shape35Segment::Quad {
        control: [-687, -295],
        to: [-687, -252],
    },
    Shape35Segment::Quad {
        control: [-687, -222],
        to: [-699, -194],
    },
    Shape35Segment::Quad {
        control: [-735, -112],
        to: [-813, -56],
    },
    Shape35Segment::Quad {
        control: [-895, 3],
        to: [-984, 2],
    },
    Shape35Segment::Line([-1028, 5]),
    Shape35Segment::Quad {
        control: [-1157, 5],
        to: [-1218, -79],
    },
    Shape35Segment::Quad {
        control: [-1226, -111],
        to: [-1226, -146],
    },
];

const CONTOUR_19: [Shape35Segment; 12] = [
    Shape35Segment::Line([-1034, -355]),
    Shape35Segment::Line([-1083, -328]),
    Shape35Segment::Quad {
        control: [-1130, -306],
        to: [-1161, -253],
    },
    Shape35Segment::Quad {
        control: [-1192, -200],
        to: [-1191, -144],
    },
    Shape35Segment::Quad {
        control: [-1189, -83],
        to: [-1109, -39],
    },
    Shape35Segment::Line([-1034, -32]),
    Shape35Segment::Quad {
        control: [-953, -32],
        to: [-880, -64],
    },
    Shape35Segment::Quad {
        control: [-807, -96],
        to: [-755, -155],
    },
    Shape35Segment::Quad {
        control: [-720, -194],
        to: [-720, -248],
    },
    Shape35Segment::Quad {
        control: [-720, -285],
        to: [-737, -315],
    },
    Shape35Segment::Quad {
        control: [-780, -390],
        to: [-882, -390],
    },
    Shape35Segment::Quad {
        control: [-937, -390],
        to: [-980, -366],
    },
];

const CONTOUR_20: [Shape35Segment; 23] = [
    Shape35Segment::Quad {
        control: [-129, -969],
        to: [-97, -1007],
    },
    Shape35Segment::Line([-84, -989]),
    Shape35Segment::Quad {
        control: [-107, -826],
        to: [-110, -586],
    },
    Shape35Segment::Line([-110, -176]),
    Shape35Segment::Quad {
        control: [-119, -152],
        to: [-114, -82],
    },
    Shape35Segment::Line([-112, -56]),
    Shape35Segment::Quad {
        control: [-112, -16],
        to: [-134, 4],
    },
    Shape35Segment::Quad {
        control: [-148, -17],
        to: [-150, -50],
    },
    Shape35Segment::Line([-154, -107]),
    Shape35Segment::Quad {
        control: [-221, -59],
        to: [-264, -38],
    },
    Shape35Segment::Quad {
        control: [-326, -6],
        to: [-387, 2],
    },
    Shape35Segment::Line([-443, 12]),
    Shape35Segment::Quad {
        control: [-478, 12],
        to: [-522, -4],
    },
    Shape35Segment::Line([-598, -37]),
    Shape35Segment::Quad {
        control: [-635, -77],
        to: [-635, -138],
    },
    Shape35Segment::Line([-634, -159]),
    Shape35Segment::Quad {
        control: [-560, -310],
        to: [-300, -353],
    },
    Shape35Segment::Line([-218, -349]),
    Shape35Segment::Quad {
        control: [-168, -344],
        to: [-143, -320],
    },
    Shape35Segment::Quad {
        control: [-147, -387],
        to: [-141, -498],
    },
    Shape35Segment::Line([-135, -676]),
    Shape35Segment::Line([-139, -784]),
    Shape35Segment::Line([-135, -900]),
];

const CONTOUR_21: [Shape35Segment; 12] = [
    Shape35Segment::Line([-562, -70]),
    Shape35Segment::Line([-503, -67]),
    Shape35Segment::Quad {
        control: [-338, -67],
        to: [-194, -153],
    },
    Shape35Segment::Quad {
        control: [-184, -156],
        to: [-181, -176],
    },
    Shape35Segment::Quad {
        control: [-178, -196],
        to: [-163, -200],
    },
    Shape35Segment::Quad {
        control: [-156, -215],
        to: [-163, -236],
    },
    Shape35Segment::Line([-180, -274]),
    Shape35Segment::Line([-201, -303]),
    Shape35Segment::Line([-268, -314]),
    Shape35Segment::Quad {
        control: [-377, -314],
        to: [-519, -207],
    },
    Shape35Segment::Line([-550, -154]),
    Shape35Segment::Quad {
        control: [-571, -122],
        to: [-571, -101],
    },
];

const CONTOUR_22: [Shape35Segment; 53] = [
    Shape35Segment::Line([-172, 233]),
    Shape35Segment::Quad {
        control: [-161, 224],
        to: [-148, 224],
    },
    Shape35Segment::Line([-130, 229]),
    Shape35Segment::Quad {
        control: [-122, 235],
        to: [-117, 244],
    },
    Shape35Segment::Line([-100, 230]),
    Shape35Segment::Line([-80, 225]),
    Shape35Segment::Line([-65, 228]),
    Shape35Segment::Quad {
        control: [-58, 230],
        to: [-52, 235],
    },
    Shape35Segment::Line([-42, 247]),
    Shape35Segment::Quad {
        control: [-39, 253],
        to: [-39, 261],
    },
    Shape35Segment::Line([-39, 320]),
    Shape35Segment::Line([-33, 326]),
    Shape35Segment::Line([-31, 333]),
    Shape35Segment::Line([-36, 345]),
    Shape35Segment::Line([-47, 349]),
    Shape35Segment::Line([-58, 345]),
    Shape35Segment::Line([-63, 333]),
    Shape35Segment::Quad {
        control: [-63, 324],
        to: [-55, 319],
    },
    Shape35Segment::Line([-55, 269]),
    Shape35Segment::Line([-59, 254]),
    Shape35Segment::Line([-69, 245]),
    Shape35Segment::Line([-81, 241]),
    Shape35Segment::Line([-94, 245]),
    Shape35Segment::Line([-105, 256]),
    Shape35Segment::Line([-109, 274]),
    Shape35Segment::Line([-109, 320]),
    Shape35Segment::Line([-103, 326]),
    Shape35Segment::Line([-101, 334]),
    Shape35Segment::Line([-105, 345]),
    Shape35Segment::Line([-117, 350]),
    Shape35Segment::Line([-128, 345]),
    Shape35Segment::Line([-132, 334]),
    Shape35Segment::Quad {
        control: [-132, 325],
        to: [-125, 321],
    },
    Shape35Segment::Line([-125, 269]),
    Shape35Segment::Line([-129, 254]),
    Shape35Segment::Line([-138, 245]),
    Shape35Segment::Line([-151, 241]),
    Shape35Segment::Line([-164, 245]),
    Shape35Segment::Line([-175, 256]),
    Shape35Segment::Line([-180, 274]),
    Shape35Segment::Line([-180, 319]),
    Shape35Segment::Line([-174, 325]),
    Shape35Segment::Line([-171, 333]),
    Shape35Segment::Line([-176, 345]),
    Shape35Segment::Line([-187, 349]),
    Shape35Segment::Line([-198, 344]),
    Shape35Segment::Line([-203, 333]),
    Shape35Segment::Quad {
        control: [-203, 325],
        to: [-195, 320],
    },
    Shape35Segment::Line([-195, 250]),
    Shape35Segment::Line([-203, 237]),
    Shape35Segment::Line([-198, 226]),
    Shape35Segment::Line([-187, 221]),
    Shape35Segment::Line([-178, 224]),
];

const CONTOUR_23: [Shape35Segment; 24] = [
    Shape35Segment::Line([-366, 237]),
    Shape35Segment::Line([-368, 246]),
    Shape35Segment::Line([-375, 251]),
    Shape35Segment::Line([-375, 322]),
    Shape35Segment::Line([-368, 328]),
    Shape35Segment::Line([-366, 336]),
    Shape35Segment::Line([-371, 347]),
    Shape35Segment::Quad {
        control: [-375, 352],
        to: [-382, 352],
    },
    Shape35Segment::Line([-392, 348]),
    Shape35Segment::Line([-397, 340]),
    Shape35Segment::Line([-412, 347]),
    Shape35Segment::Line([-428, 349]),
    Shape35Segment::Line([-450, 342]),
    Shape35Segment::Line([-468, 321]),
    Shape35Segment::Line([-474, 288]),
    Shape35Segment::Line([-470, 260]),
    Shape35Segment::Line([-458, 240]),
    Shape35Segment::Line([-441, 228]),
    Shape35Segment::Line([-425, 224]),
    Shape35Segment::Line([-411, 227]),
    Shape35Segment::Quad {
        control: [-403, 230],
        to: [-398, 236],
    },
    Shape35Segment::Line([-392, 226]),
    Shape35Segment::Line([-382, 222]),
    Shape35Segment::Line([-371, 226]),
];

const CONTOUR_24: [Shape35Segment; 13] = [
    Shape35Segment::Line([-406, 246]),
    Shape35Segment::Quad {
        control: [-414, 240],
        to: [-425, 240],
    },
    Shape35Segment::Quad {
        control: [-436, 240],
        to: [-443, 247],
    },
    Shape35Segment::Line([-454, 264]),
    Shape35Segment::Line([-457, 288]),
    Shape35Segment::Line([-454, 311]),
    Shape35Segment::Line([-443, 329]),
    Shape35Segment::Line([-425, 335]),
    Shape35Segment::Line([-409, 331]),
    Shape35Segment::Line([-398, 320]),
    Shape35Segment::Line([-392, 304]),
    Shape35Segment::Line([-390, 288]),
    Shape35Segment::Line([-394, 262]),
];

const CONTOUR_25: [Shape35Segment; 43] = [
    Shape35Segment::Line([-753, 269]),
    Shape35Segment::Quad {
        control: [-753, 287],
        to: [-746, 301],
    },
    Shape35Segment::Line([-728, 324]),
    Shape35Segment::Quad {
        control: [-716, 332],
        to: [-700, 332],
    },
    Shape35Segment::Line([-675, 325]),
    Shape35Segment::Quad {
        control: [-664, 318],
        to: [-659, 306],
    },
    Shape35Segment::Quad {
        control: [-654, 295],
        to: [-654, 281],
    },
    Shape35Segment::Line([-654, 276]),
    Shape35Segment::Line([-654, 272]),
    Shape35Segment::Line([-663, 272]),
    Shape35Segment::Line([-668, 278]),
    Shape35Segment::Line([-676, 280]),
    Shape35Segment::Line([-688, 275]),
    Shape35Segment::Line([-692, 264]),
    Shape35Segment::Line([-688, 253]),
    Shape35Segment::Line([-676, 248]),
    Shape35Segment::Line([-668, 250]),
    Shape35Segment::Line([-663, 256]),
    Shape35Segment::Line([-651, 256]),
    Shape35Segment::Quad {
        control: [-642, 256],
        to: [-639, 261],
    },
    Shape35Segment::Line([-637, 271]),
    Shape35Segment::Quad {
        control: [-637, 310],
        to: [-655, 329],
    },
    Shape35Segment::Quad {
        control: [-673, 347],
        to: [-701, 347],
    },
    Shape35Segment::Line([-733, 341]),
    Shape35Segment::Line([-755, 324]),
    Shape35Segment::Line([-768, 298]),
    Shape35Segment::Line([-772, 269]),
    Shape35Segment::Line([-766, 236]),
    Shape35Segment::Line([-751, 208]),
    Shape35Segment::Line([-728, 191]),
    Shape35Segment::Quad {
        control: [-715, 184],
        to: [-700, 184],
    },
    Shape35Segment::Line([-684, 186]),
    Shape35Segment::Line([-668, 193]),
    Shape35Segment::Line([-661, 191]),
    Shape35Segment::Quad {
        control: [-654, 191],
        to: [-650, 196],
    },
    Shape35Segment::Quad {
        control: [-645, 200],
        to: [-645, 207],
    },
    Shape35Segment::Line([-650, 218]),
    Shape35Segment::Line([-661, 223]),
    Shape35Segment::Line([-672, 218]),
    Shape35Segment::Line([-677, 208]),
    Shape35Segment::Quad {
        control: [-689, 200],
        to: [-701, 200],
    },
    Shape35Segment::Quad {
        control: [-720, 201],
        to: [-732, 212],
    },
    Shape35Segment::Line([-748, 238]),
];

const CONTOUR_26: [Shape35Segment; 8] = [
    Shape35Segment::Line([-1004, 245]),
    Shape35Segment::Line([-1016, 259]),
    Shape35Segment::Line([-1021, 281]),
    Shape35Segment::Line([-954, 281]),
    Shape35Segment::Line([-956, 265]),
    Shape35Segment::Line([-963, 252]),
    Shape35Segment::Line([-974, 243]),
    Shape35Segment::Line([-988, 240]),
];

const CONTOUR_27: [Shape35Segment; 28] = [
    Shape35Segment::Line([-948, 245]),
    Shape35Segment::Quad {
        control: [-942, 253],
        to: [-939, 262],
    },
    Shape35Segment::Line([-935, 279]),
    Shape35Segment::Line([-937, 288]),
    Shape35Segment::Line([-940, 294]),
    Shape35Segment::Line([-944, 297]),
    Shape35Segment::Line([-1021, 297]),
    Shape35Segment::Line([-1016, 315]),
    Shape35Segment::Line([-1002, 330]),
    Shape35Segment::Line([-984, 335]),
    Shape35Segment::Line([-969, 332]),
    Shape35Segment::Line([-969, 330]),
    Shape35Segment::Line([-964, 319]),
    Shape35Segment::Line([-953, 314]),
    Shape35Segment::Quad {
        control: [-946, 314],
        to: [-941, 319],
    },
    Shape35Segment::Line([-937, 330]),
    Shape35Segment::Line([-942, 341]),
    Shape35Segment::Line([-953, 346]),
    Shape35Segment::Line([-960, 344]),
    Shape35Segment::Line([-984, 349]),
    Shape35Segment::Line([-1015, 340]),
    Shape35Segment::Quad {
        control: [-1028, 331],
        to: [-1033, 318],
    },
    Shape35Segment::Line([-1039, 291]),
    Shape35Segment::Line([-1036, 269]),
    Shape35Segment::Line([-1026, 248]),
    Shape35Segment::Line([-1010, 232]),
    Shape35Segment::Quad {
        control: [-1000, 226],
        to: [-988, 226],
    },
    Shape35Segment::Line([-965, 231]),
];

const CONTOUR_28: [Shape35Segment; 35] = [
    Shape35Segment::Line([-1225, 345]),
    Shape35Segment::Line([-1230, 334]),
    Shape35Segment::Line([-1228, 326]),
    Shape35Segment::Line([-1222, 320]),
    Shape35Segment::Line([-1222, 267]),
    Shape35Segment::Quad {
        control: [-1222, 255],
        to: [-1230, 248],
    },
    Shape35Segment::Quad {
        control: [-1237, 241],
        to: [-1249, 241],
    },
    Shape35Segment::Quad {
        control: [-1263, 242],
        to: [-1271, 250],
    },
    Shape35Segment::Quad {
        control: [-1280, 259],
        to: [-1280, 275],
    },
    Shape35Segment::Line([-1280, 320]),
    Shape35Segment::Line([-1274, 326]),
    Shape35Segment::Line([-1272, 334]),
    Shape35Segment::Line([-1276, 345]),
    Shape35Segment::Line([-1288, 350]),
    Shape35Segment::Line([-1299, 345]),
    Shape35Segment::Line([-1303, 334]),
    Shape35Segment::Line([-1301, 326]),
    Shape35Segment::Line([-1295, 320]),
    Shape35Segment::Line([-1295, 250]),
    Shape35Segment::Line([-1301, 244]),
    Shape35Segment::Line([-1303, 237]),
    Shape35Segment::Line([-1298, 226]),
    Shape35Segment::Quad {
        control: [-1293, 221],
        to: [-1287, 221],
    },
    Shape35Segment::Line([-1277, 224]),
    Shape35Segment::Line([-1271, 233]),
    Shape35Segment::Line([-1260, 226]),
    Shape35Segment::Line([-1247, 224]),
    Shape35Segment::Line([-1227, 229]),
    Shape35Segment::Line([-1212, 243]),
    Shape35Segment::Quad {
        control: [-1206, 253],
        to: [-1206, 265],
    },
    Shape35Segment::Line([-1206, 320]),
    Shape35Segment::Line([-1200, 326]),
    Shape35Segment::Line([-1198, 334]),
    Shape35Segment::Line([-1203, 345]),
    Shape35Segment::Line([-1214, 350]),
];

const CONTOUR_29: [Shape35Segment; 8] = [
    Shape35Segment::Line([-1472, 199]),
    Shape35Segment::Line([-1475, 206]),
    Shape35Segment::Quad {
        control: [-1479, 210],
        to: [-1483, 210],
    },
    Shape35Segment::Line([-1490, 206]),
    Shape35Segment::Line([-1494, 199]),
    Shape35Segment::Line([-1491, 191]),
    Shape35Segment::Line([-1483, 187]),
    Shape35Segment::Line([-1475, 191]),
];

const CONTOUR_30: [Shape35Segment; 17] = [
    Shape35Segment::Line([-1467, 231]),
    Shape35Segment::Line([-1465, 239]),
    Shape35Segment::Line([-1467, 247]),
    Shape35Segment::Line([-1474, 253]),
    Shape35Segment::Line([-1474, 319]),
    Shape35Segment::Line([-1467, 325]),
    Shape35Segment::Line([-1465, 333]),
    Shape35Segment::Line([-1470, 345]),
    Shape35Segment::Quad {
        control: [-1474, 349],
        to: [-1481, 349],
    },
    Shape35Segment::Line([-1492, 344]),
    Shape35Segment::Line([-1497, 333]),
    Shape35Segment::Quad {
        control: [-1497, 325],
        to: [-1489, 320],
    },
    Shape35Segment::Line([-1489, 252]),
    Shape35Segment::Quad {
        control: [-1497, 247],
        to: [-1497, 239],
    },
    Shape35Segment::Line([-1492, 228]),
    Shape35Segment::Quad {
        control: [-1487, 223],
        to: [-1481, 223],
    },
    Shape35Segment::Line([-1473, 225]),
];

const CONTOUR_31: [Shape35Segment; 19] = [
    Shape35Segment::Line([-1660, 186]),
    Shape35Segment::Line([-1662, 195]),
    Shape35Segment::Line([-1668, 201]),
    Shape35Segment::Line([-1668, 320]),
    Shape35Segment::Line([-1662, 326]),
    Shape35Segment::Line([-1660, 334]),
    Shape35Segment::Line([-1665, 345]),
    Shape35Segment::Line([-1676, 350]),
    Shape35Segment::Line([-1687, 345]),
    Shape35Segment::Line([-1692, 334]),
    Shape35Segment::Line([-1690, 326]),
    Shape35Segment::Line([-1684, 320]),
    Shape35Segment::Line([-1684, 200]),
    Shape35Segment::Line([-1689, 194]),
    Shape35Segment::Line([-1691, 186]),
    Shape35Segment::Line([-1687, 175]),
    Shape35Segment::Line([-1676, 171]),
    Shape35Segment::Line([-1668, 173]),
    Shape35Segment::Line([-1662, 179]),
];

const CONTOUR_32: [Shape35Segment; 35] = [
    Shape35Segment::Quad {
        control: [-1948, 221],
        to: [-1942, 221],
    },
    Shape35Segment::Quad {
        control: [-1937, 221],
        to: [-1932, 224],
    },
    Shape35Segment::Line([-1926, 233]),
    Shape35Segment::Line([-1915, 226]),
    Shape35Segment::Line([-1902, 224]),
    Shape35Segment::Line([-1882, 229]),
    Shape35Segment::Quad {
        control: [-1872, 234],
        to: [-1867, 243],
    },
    Shape35Segment::Quad {
        control: [-1861, 253],
        to: [-1861, 265],
    },
    Shape35Segment::Line([-1861, 320]),
    Shape35Segment::Line([-1855, 326]),
    Shape35Segment::Line([-1853, 334]),
    Shape35Segment::Line([-1858, 345]),
    Shape35Segment::Quad {
        control: [-1863, 350],
        to: [-1869, 350],
    },
    Shape35Segment::Line([-1880, 345]),
    Shape35Segment::Line([-1885, 334]),
    Shape35Segment::Line([-1883, 326]),
    Shape35Segment::Line([-1877, 320]),
    Shape35Segment::Line([-1877, 267]),
    Shape35Segment::Quad {
        control: [-1877, 255],
        to: [-1885, 248],
    },
    Shape35Segment::Quad {
        control: [-1892, 241],
        to: [-1904, 241],
    },
    Shape35Segment::Quad {
        control: [-1918, 242],
        to: [-1926, 250],
    },
    Shape35Segment::Quad {
        control: [-1935, 259],
        to: [-1935, 275],
    },
    Shape35Segment::Line([-1935, 320]),
    Shape35Segment::Line([-1929, 326]),
    Shape35Segment::Line([-1927, 334]),
    Shape35Segment::Line([-1931, 345]),
    Shape35Segment::Line([-1943, 350]),
    Shape35Segment::Line([-1954, 345]),
    Shape35Segment::Line([-1958, 334]),
    Shape35Segment::Line([-1956, 326]),
    Shape35Segment::Line([-1950, 320]),
    Shape35Segment::Line([-1950, 250]),
    Shape35Segment::Line([-1956, 244]),
    Shape35Segment::Line([-1958, 237]),
    Shape35Segment::Line([-1953, 226]),
];

const CONTOUR_33: [Shape35Segment; 15] = [
    Shape35Segment::Line([-2191, 201]),
    Shape35Segment::Quad {
        control: [-2206, 201],
        to: [-2216, 208],
    },
    Shape35Segment::Line([-2231, 224]),
    Shape35Segment::Line([-2239, 245]),
    Shape35Segment::Line([-2241, 267]),
    Shape35Segment::Line([-2234, 299]),
    Shape35Segment::Line([-2218, 322]),
    Shape35Segment::Quad {
        control: [-2208, 332],
        to: [-2191, 332],
    },
    Shape35Segment::Line([-2167, 326]),
    Shape35Segment::Line([-2152, 310]),
    Shape35Segment::Line([-2143, 289]),
    Shape35Segment::Line([-2140, 267]),
    Shape35Segment::Line([-2144, 240]),
    Shape35Segment::Quad {
        control: [-2148, 228],
        to: [-2155, 219],
    },
    Shape35Segment::Line([-2171, 206]),
];

const CONTOUR_34: [Shape35Segment; 15] = [
    Shape35Segment::Quad {
        control: [-2123, 242],
        to: [-2123, 267],
    },
    Shape35Segment::Line([-2127, 295]),
    Shape35Segment::Line([-2140, 321]),
    Shape35Segment::Line([-2161, 340]),
    Shape35Segment::Line([-2191, 347]),
    Shape35Segment::Line([-2216, 341]),
    Shape35Segment::Line([-2237, 325]),
    Shape35Segment::Line([-2253, 298]),
    Shape35Segment::Line([-2258, 266]),
    Shape35Segment::Line([-2254, 236]),
    Shape35Segment::Line([-2241, 210]),
    Shape35Segment::Line([-2220, 193]),
    Shape35Segment::Quad {
        control: [-2207, 186],
        to: [-2191, 186],
    },
    Shape35Segment::Quad {
        control: [-2172, 186],
        to: [-2156, 196],
    },
    Shape35Segment::Quad {
        control: [-2141, 205],
        to: [-2132, 223],
    },
];

pub const CONTOURS: [Shape35Contour; 35] = [
    Shape35Contour {
        start: [602, -974],
        segments: &CONTOUR_0,
    },
    Shape35Contour {
        start: [258, -378],
        segments: &CONTOUR_1,
    },
    Shape35Contour {
        start: [280, -325],
        segments: &CONTOUR_2,
    },
    Shape35Contour {
        start: [728, -419],
        segments: &CONTOUR_3,
    },
    Shape35Contour {
        start: [799, -480],
        segments: &CONTOUR_4,
    },
    Shape35Contour {
        start: [753, -355],
        segments: &CONTOUR_5,
    },
    Shape35Contour {
        start: [1486, -952],
        segments: &CONTOUR_6,
    },
    Shape35Contour {
        start: [888, -158],
        segments: &CONTOUR_7,
    },
    Shape35Contour {
        start: [1153, -66],
        segments: &CONTOUR_8,
    },
    Shape35Contour {
        start: [2117, -498],
        segments: &CONTOUR_9,
    },
    Shape35Contour {
        start: [1716, 259],
        segments: &CONTOUR_10,
    },
    Shape35Contour {
        start: [424, 244],
        segments: &CONTOUR_11,
    },
    Shape35Contour {
        start: [220, 245],
        segments: &CONTOUR_12,
    },
    Shape35Contour {
        start: [164, 245],
        segments: &CONTOUR_13,
    },
    Shape35Contour {
        start: [646, 615],
        segments: &CONTOUR_14,
    },
    Shape35Contour {
        start: [-2286, -328],
        segments: &CONTOUR_15,
    },
    Shape35Contour {
        start: [-1717, -248],
        segments: &CONTOUR_16,
    },
    Shape35Contour {
        start: [-1808, -138],
        segments: &CONTOUR_17,
    },
    Shape35Contour {
        start: [-1226, -146],
        segments: &CONTOUR_18,
    },
    Shape35Contour {
        start: [-980, -366],
        segments: &CONTOUR_19,
    },
    Shape35Contour {
        start: [-135, -900],
        segments: &CONTOUR_20,
    },
    Shape35Contour {
        start: [-571, -101],
        segments: &CONTOUR_21,
    },
    Shape35Contour {
        start: [-178, 224],
        segments: &CONTOUR_22,
    },
    Shape35Contour {
        start: [-371, 226],
        segments: &CONTOUR_23,
    },
    Shape35Contour {
        start: [-394, 262],
        segments: &CONTOUR_24,
    },
    Shape35Contour {
        start: [-748, 238],
        segments: &CONTOUR_25,
    },
    Shape35Contour {
        start: [-988, 240],
        segments: &CONTOUR_26,
    },
    Shape35Contour {
        start: [-965, 231],
        segments: &CONTOUR_27,
    },
    Shape35Contour {
        start: [-1214, 350],
        segments: &CONTOUR_28,
    },
    Shape35Contour {
        start: [-1475, 191],
        segments: &CONTOUR_29,
    },
    Shape35Contour {
        start: [-1473, 225],
        segments: &CONTOUR_30,
    },
    Shape35Contour {
        start: [-1662, 179],
        segments: &CONTOUR_31,
    },
    Shape35Contour {
        start: [-1953, 226],
        segments: &CONTOUR_32,
    },
    Shape35Contour {
        start: [-2171, 206],
        segments: &CONTOUR_33,
    },
    Shape35Contour {
        start: [-2132, 223],
        segments: &CONTOUR_34,
    },
];

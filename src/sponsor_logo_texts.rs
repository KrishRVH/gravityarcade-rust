// Generated from gravity_arcade.swf sponsor-logo DefineText tags.
// Coordinates are stored as SWF text-local centipixels; divide by 100 for pixels.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SponsorLogoTextSegment {
    Line([i16; 2]),
    Quad { control: [i16; 2], to: [i16; 2] },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SponsorLogoTextContour {
    pub(super) start: [i16; 2],
    pub(super) segments: &'static [SponsorLogoTextSegment],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SponsorLogoTextDefinition {
    pub(super) text: &'static str,
    pub(super) define_text_id: u16,
    pub(super) font_ids: &'static [u16],
    pub(super) color_rgb: [u8; 3],
    pub(super) bounds_centipx: [i16; 4],
    pub(super) contours: &'static [SponsorLogoTextContour],
}

const DARK_FONT_IDS: [u16; 1] = [32];

const DARK_CONTOUR_0: [SponsorLogoTextSegment; 27] = [
    SponsorLogoTextSegment::Quad {
        control: [325, 1079],
        to: [499, 1079],
    },
    SponsorLogoTextSegment::Quad {
        control: [602, 1079],
        to: [705, 1123],
    },
    SponsorLogoTextSegment::Quad {
        control: [780, 1189],
        to: [800, 1317],
    },
    SponsorLogoTextSegment::Quad {
        control: [807, 1342],
        to: [811, 1502],
    },
    SponsorLogoTextSegment::Quad {
        control: [814, 1582],
        to: [825, 1637],
    },
    SponsorLogoTextSegment::Quad {
        control: [813, 1665],
        to: [793, 1665],
    },
    SponsorLogoTextSegment::Quad {
        control: [773, 1665],
        to: [752, 1642],
    },
    SponsorLogoTextSegment::Quad {
        control: [739, 1590],
        to: [737, 1495],
    },
    SponsorLogoTextSegment::Line([729, 1351]),
    SponsorLogoTextSegment::Quad {
        control: [707, 1180],
        to: [591, 1154],
    },
    SponsorLogoTextSegment::Line([379, 1169]),
    SponsorLogoTextSegment::Quad {
        control: [272, 1197],
        to: [232, 1289],
    },
    SponsorLogoTextSegment::Line([231, 1321]),
    SponsorLogoTextSegment::Quad {
        control: [231, 1360],
        to: [245, 1439],
    },
    SponsorLogoTextSegment::Quad {
        control: [258, 1517],
        to: [258, 1558],
    },
    SponsorLogoTextSegment::Quad {
        control: [258, 1627],
        to: [231, 1687],
    },
    SponsorLogoTextSegment::Line([177, 1687]),
    SponsorLogoTextSegment::Line([183, 1590]),
    SponsorLogoTextSegment::Quad {
        control: [183, 1531],
        to: [170, 1412],
    },
    SponsorLogoTextSegment::Quad {
        control: [156, 1292],
        to: [156, 1231],
    },
    SponsorLogoTextSegment::Line([156, 1150]),
    SponsorLogoTextSegment::Line([156, 1125]),
    SponsorLogoTextSegment::Line([156, 1100]),
    SponsorLogoTextSegment::Quad {
        control: [156, 1059],
        to: [199, 1050],
    },
    SponsorLogoTextSegment::Quad {
        control: [231, 1075],
        to: [231, 1108],
    },
    SponsorLogoTextSegment::Line([223, 1153]),
    SponsorLogoTextSegment::Quad {
        control: [217, 1188],
        to: [219, 1195],
    },
];

const DARK_CONTOUR_1: [SponsorLogoTextSegment; 18] = [
    SponsorLogoTextSegment::Quad {
        control: [1467, 1051],
        to: [1554, 1105],
    },
    SponsorLogoTextSegment::Quad {
        control: [1639, 1159],
        to: [1684, 1253],
    },
    SponsorLogoTextSegment::Line([1688, 1273]),
    SponsorLogoTextSegment::Line([1693, 1293]),
    SponsorLogoTextSegment::Quad {
        control: [1693, 1307],
        to: [1680, 1323],
    },
    SponsorLogoTextSegment::Quad {
        control: [1353, 1381],
        to: [1013, 1509],
    },
    SponsorLogoTextSegment::Quad {
        control: [1019, 1537],
        to: [1061, 1577],
    },
    SponsorLogoTextSegment::Line([1214, 1586]),
    SponsorLogoTextSegment::Quad {
        control: [1526, 1586],
        to: [1706, 1430],
    },
    SponsorLogoTextSegment::Line([1722, 1427]),
    SponsorLogoTextSegment::Quad {
        control: [1734, 1427],
        to: [1763, 1460],
    },
    SponsorLogoTextSegment::Quad {
        control: [1569, 1679],
        to: [1168, 1701],
    },
    SponsorLogoTextSegment::Line([1066, 1676]),
    SponsorLogoTextSegment::Quad {
        control: [1006, 1658],
        to: [971, 1623],
    },
    SponsorLogoTextSegment::Quad {
        control: [919, 1551],
        to: [919, 1447],
    },
    SponsorLogoTextSegment::Quad {
        control: [919, 1401],
        to: [930, 1355],
    },
    SponsorLogoTextSegment::Quad {
        control: [1106, 1058],
        to: [1322, 1058],
    },
    SponsorLogoTextSegment::Line([1366, 1061]),
];

const DARK_CONTOUR_2: [SponsorLogoTextSegment; 7] = [
    SponsorLogoTextSegment::Quad {
        control: [1026, 1347],
        to: [1008, 1414],
    },
    SponsorLogoTextSegment::Quad {
        control: [1077, 1380],
        to: [1136, 1374],
    },
    SponsorLogoTextSegment::Line([1358, 1294]),
    SponsorLogoTextSegment::Quad {
        control: [1492, 1245],
        to: [1572, 1228],
    },
    SponsorLogoTextSegment::Quad {
        control: [1527, 1168],
        to: [1401, 1168],
    },
    SponsorLogoTextSegment::Line([1231, 1190]),
    SponsorLogoTextSegment::Quad {
        control: [1141, 1210],
        to: [1084, 1264],
    },
];

const DARK_CONTOUR_3: [SponsorLogoTextSegment; 12] = [
    SponsorLogoTextSegment::Quad {
        control: [2280, 1600],
        to: [2391, 1551],
    },
    SponsorLogoTextSegment::Quad {
        control: [2503, 1503],
        to: [2582, 1413],
    },
    SponsorLogoTextSegment::Quad {
        control: [2633, 1355],
        to: [2633, 1276],
    },
    SponsorLogoTextSegment::Quad {
        control: [2633, 1222],
        to: [2609, 1177],
    },
    SponsorLogoTextSegment::Quad {
        control: [2546, 1066],
        to: [2392, 1066],
    },
    SponsorLogoTextSegment::Quad {
        control: [2310, 1066],
        to: [2242, 1105],
    },
    SponsorLogoTextSegment::Quad {
        control: [2206, 1099],
        to: [2161, 1119],
    },
    SponsorLogoTextSegment::Line([2085, 1160]),
    SponsorLogoTextSegment::Quad {
        control: [2014, 1195],
        to: [1967, 1275],
    },
    SponsorLogoTextSegment::Quad {
        control: [1920, 1354],
        to: [1922, 1439],
    },
    SponsorLogoTextSegment::Quad {
        control: [1924, 1526],
        to: [2043, 1591],
    },
    SponsorLogoTextSegment::Line([2155, 1600]),
];

const DARK_CONTOUR_4: [SponsorLogoTextSegment; 11] = [
    SponsorLogoTextSegment::Quad {
        control: [2156, 991],
        to: [2353, 991],
    },
    SponsorLogoTextSegment::Quad {
        control: [2546, 991],
        to: [2670, 1121],
    },
    SponsorLogoTextSegment::Quad {
        control: [2708, 1201],
        to: [2708, 1270],
    },
    SponsorLogoTextSegment::Quad {
        control: [2708, 1320],
        to: [2686, 1368],
    },
    SponsorLogoTextSegment::Quad {
        control: [2631, 1495],
        to: [2492, 1587],
    },
    SponsorLogoTextSegment::Quad {
        control: [2376, 1679],
        to: [2234, 1676],
    },
    SponsorLogoTextSegment::Line([2165, 1682]),
    SponsorLogoTextSegment::Quad {
        control: [1957, 1682],
        to: [1858, 1543],
    },
    SponsorLogoTextSegment::Quad {
        control: [1845, 1491],
        to: [1845, 1435],
    },
    SponsorLogoTextSegment::Quad {
        control: [1845, 1339],
        to: [1883, 1252],
    },
    SponsorLogoTextSegment::Quad {
        control: [1921, 1166],
        to: [1992, 1113],
    },
];

const DARK_CONTOUR_5: [SponsorLogoTextSegment; 23] = [
    SponsorLogoTextSegment::Quad {
        control: [3630, 376],
        to: [3626, 737],
    },
    SponsorLogoTextSegment::Line([3626, 1389]),
    SponsorLogoTextSegment::Quad {
        control: [3611, 1428],
        to: [3619, 1532],
    },
    SponsorLogoTextSegment::Line([3623, 1576]),
    SponsorLogoTextSegment::Quad {
        control: [3623, 1642],
        to: [3572, 1680],
    },
    SponsorLogoTextSegment::Quad {
        control: [3543, 1639],
        to: [3540, 1586],
    },
    SponsorLogoTextSegment::Line([3535, 1508]),
    SponsorLogoTextSegment::Quad {
        control: [3447, 1580],
        to: [3386, 1609],
    },
    SponsorLogoTextSegment::Quad {
        control: [3281, 1664],
        to: [3185, 1676],
    },
    SponsorLogoTextSegment::Quad {
        control: [3142, 1693],
        to: [3094, 1693],
    },
    SponsorLogoTextSegment::Quad {
        control: [3037, 1693],
        to: [2968, 1668],
    },
    SponsorLogoTextSegment::Line([2846, 1616]),
    SponsorLogoTextSegment::Quad {
        control: [2784, 1548],
        to: [2784, 1447],
    },
    SponsorLogoTextSegment::Line([2786, 1410]),
    SponsorLogoTextSegment::Quad {
        control: [2903, 1168],
        to: [3316, 1100],
    },
    SponsorLogoTextSegment::Line([3447, 1107]),
    SponsorLogoTextSegment::Quad {
        control: [3528, 1115],
        to: [3550, 1151],
    },
    SponsorLogoTextSegment::Quad {
        control: [3544, 1059],
        to: [3554, 884],
    },
    SponsorLogoTextSegment::Line([3564, 608]),
    SponsorLogoTextSegment::Line([3557, 441]),
    SponsorLogoTextSegment::Line([3564, 259]),
    SponsorLogoTextSegment::Quad {
        control: [3572, 147],
        to: [3635, 82],
    },
    SponsorLogoTextSegment::Line([3665, 119]),
];

const DARK_CONTOUR_6: [SponsorLogoTextSegment; 12] = [
    SponsorLogoTextSegment::Quad {
        control: [3255, 1546],
        to: [3478, 1414],
    },
    SponsorLogoTextSegment::Quad {
        control: [3488, 1410],
        to: [3492, 1379],
    },
    SponsorLogoTextSegment::Quad {
        control: [3496, 1346],
        to: [3523, 1340],
    },
    SponsorLogoTextSegment::Quad {
        control: [3530, 1327],
        to: [3520, 1291],
    },
    SponsorLogoTextSegment::Line([3495, 1241]),
    SponsorLogoTextSegment::Line([3465, 1200]),
    SponsorLogoTextSegment::Line([3369, 1185]),
    SponsorLogoTextSegment::Quad {
        control: [3201, 1185],
        to: [2985, 1347],
    },
    SponsorLogoTextSegment::Quad {
        control: [2980, 1368],
        to: [2937, 1429],
    },
    SponsorLogoTextSegment::Quad {
        control: [2907, 1475],
        to: [2907, 1505],
    },
    SponsorLogoTextSegment::Quad {
        control: [2907, 1525],
        to: [2916, 1542],
    },
    SponsorLogoTextSegment::Line([3002, 1546]),
];

const DARK_CONTOUR_7: [SponsorLogoTextSegment; 18] = [
    SponsorLogoTextSegment::Quad {
        control: [4307, 1051],
        to: [4394, 1105],
    },
    SponsorLogoTextSegment::Quad {
        control: [4479, 1159],
        to: [4524, 1253],
    },
    SponsorLogoTextSegment::Line([4528, 1273]),
    SponsorLogoTextSegment::Line([4533, 1293]),
    SponsorLogoTextSegment::Quad {
        control: [4533, 1307],
        to: [4520, 1323],
    },
    SponsorLogoTextSegment::Quad {
        control: [4193, 1381],
        to: [3853, 1509],
    },
    SponsorLogoTextSegment::Quad {
        control: [3859, 1537],
        to: [3901, 1577],
    },
    SponsorLogoTextSegment::Line([4054, 1586]),
    SponsorLogoTextSegment::Quad {
        control: [4366, 1586],
        to: [4546, 1430],
    },
    SponsorLogoTextSegment::Line([4562, 1427]),
    SponsorLogoTextSegment::Quad {
        control: [4574, 1427],
        to: [4603, 1460],
    },
    SponsorLogoTextSegment::Quad {
        control: [4409, 1679],
        to: [4008, 1701],
    },
    SponsorLogoTextSegment::Line([3906, 1676]),
    SponsorLogoTextSegment::Quad {
        control: [3846, 1658],
        to: [3811, 1623],
    },
    SponsorLogoTextSegment::Quad {
        control: [3759, 1551],
        to: [3759, 1447],
    },
    SponsorLogoTextSegment::Quad {
        control: [3759, 1401],
        to: [3770, 1355],
    },
    SponsorLogoTextSegment::Quad {
        control: [3946, 1058],
        to: [4162, 1058],
    },
    SponsorLogoTextSegment::Line([4206, 1061]),
];

const DARK_CONTOUR_8: [SponsorLogoTextSegment; 7] = [
    SponsorLogoTextSegment::Quad {
        control: [3866, 1347],
        to: [3848, 1414],
    },
    SponsorLogoTextSegment::Quad {
        control: [3917, 1380],
        to: [3976, 1374],
    },
    SponsorLogoTextSegment::Line([4198, 1294]),
    SponsorLogoTextSegment::Quad {
        control: [4332, 1245],
        to: [4412, 1228],
    },
    SponsorLogoTextSegment::Quad {
        control: [4367, 1168],
        to: [4241, 1168],
    },
    SponsorLogoTextSegment::Line([4071, 1190]),
    SponsorLogoTextSegment::Quad {
        control: [3981, 1210],
        to: [3924, 1264],
    },
];

const DARK_CONTOUR_9: [SponsorLogoTextSegment; 13] = [
    SponsorLogoTextSegment::Quad {
        control: [4762, 529],
        to: [4762, 652],
    },
    SponsorLogoTextSegment::Quad {
        control: [4762, 784],
        to: [4776, 1034],
    },
    SponsorLogoTextSegment::Quad {
        control: [4789, 1285],
        to: [4789, 1408],
    },
    SponsorLogoTextSegment::Quad {
        control: [4789, 1529],
        to: [4779, 1640],
    },
    SponsorLogoTextSegment::Line([4713, 1669]),
    SponsorLogoTextSegment::Quad {
        control: [4703, 1471],
        to: [4687, 641],
    },
    SponsorLogoTextSegment::Quad {
        control: [4686, 567],
        to: [4699, 428],
    },
    SponsorLogoTextSegment::Quad {
        control: [4711, 291],
        to: [4711, 226],
    },
    SponsorLogoTextSegment::Line([4711, 159]),
    SponsorLogoTextSegment::Quad {
        control: [4723, 124],
        to: [4748, 124],
    },
    SponsorLogoTextSegment::Quad {
        control: [4761, 124],
        to: [4772, 139],
    },
    SponsorLogoTextSegment::Quad {
        control: [4789, 187],
        to: [4789, 248],
    },
    SponsorLogoTextSegment::Quad {
        control: [4789, 307],
        to: [4770, 426],
    },
];

const DARK_CONTOUR_10: [SponsorLogoTextSegment; 9] = [
    SponsorLogoTextSegment::Line([5034, 917]),
    SponsorLogoTextSegment::Quad {
        control: [5013, 925],
        to: [4997, 925],
    },
    SponsorLogoTextSegment::Quad {
        control: [4970, 925],
        to: [4958, 908],
    },
    SponsorLogoTextSegment::Line([4944, 919]),
    SponsorLogoTextSegment::Line([4935, 928]),
    SponsorLogoTextSegment::Quad {
        control: [4985, 944],
        to: [5011, 944],
    },
    SponsorLogoTextSegment::Quad {
        control: [5040, 944],
        to: [5059, 930],
    },
    SponsorLogoTextSegment::Line([5061, 914]),
    SponsorLogoTextSegment::Line([5062, 909]),
];

const DARK_CONTOUR_11: [SponsorLogoTextSegment; 8] = [
    SponsorLogoTextSegment::Line([4890, 1004]),
    SponsorLogoTextSegment::Quad {
        control: [4848, 983],
        to: [4848, 936],
    },
    SponsorLogoTextSegment::Quad {
        control: [4848, 911],
        to: [4864, 891],
    },
    SponsorLogoTextSegment::Quad {
        control: [4914, 822],
        to: [5007, 822],
    },
    SponsorLogoTextSegment::Quad {
        control: [5109, 822],
        to: [5141, 902],
    },
    SponsorLogoTextSegment::Quad {
        control: [5148, 971],
        to: [5103, 1012],
    },
    SponsorLogoTextSegment::Quad {
        control: [5071, 1031],
        to: [5031, 1031],
    },
    SponsorLogoTextSegment::Line([4960, 1020]),
];

const DARK_CONTOUR_12: [SponsorLogoTextSegment; 11] = [
    SponsorLogoTextSegment::Line([4964, 1166]),
    SponsorLogoTextSegment::Quad {
        control: [4966, 1122],
        to: [5004, 1096],
    },
    SponsorLogoTextSegment::Quad {
        control: [5052, 1177],
        to: [5052, 1325],
    },
    SponsorLogoTextSegment::Line([5047, 1429]),
    SponsorLogoTextSegment::Line([5021, 1495]),
    SponsorLogoTextSegment::Quad {
        control: [5032, 1477],
        to: [5042, 1556],
    },
    SponsorLogoTextSegment::Quad {
        control: [5045, 1627],
        to: [5000, 1666],
    },
    SponsorLogoTextSegment::Quad {
        control: [4964, 1592],
        to: [4964, 1507],
    },
    SponsorLogoTextSegment::Line([4971, 1407]),
    SponsorLogoTextSegment::Line([4977, 1309]),
    SponsorLogoTextSegment::Line([4970, 1231]),
];

const DARK_CONTOUR_13: [SponsorLogoTextSegment; 26] = [
    SponsorLogoTextSegment::Quad {
        control: [5205, 1328],
        to: [5243, 1259],
    },
    SponsorLogoTextSegment::Quad {
        control: [5331, 1096],
        to: [5564, 1096],
    },
    SponsorLogoTextSegment::Line([5843, 1130]),
    SponsorLogoTextSegment::Quad {
        control: [5934, 1144],
        to: [6016, 1214],
    },
    SponsorLogoTextSegment::Line([6019, 1269]),
    SponsorLogoTextSegment::Quad {
        control: [6019, 1352],
        to: [5979, 1428],
    },
    SponsorLogoTextSegment::Quad {
        control: [5939, 1505],
        to: [5873, 1544],
    },
    SponsorLogoTextSegment::Line([5750, 1617]),
    SponsorLogoTextSegment::Quad {
        control: [5679, 1648],
        to: [5619, 1644],
    },
    SponsorLogoTextSegment::Line([5619, 1659]),
    SponsorLogoTextSegment::Line([5856, 1673]),
    SponsorLogoTextSegment::Quad {
        control: [6118, 1693],
        to: [6279, 1763],
    },
    SponsorLogoTextSegment::Quad {
        control: [6505, 1862],
        to: [6579, 2071],
    },
    SponsorLogoTextSegment::Quad {
        control: [6618, 2254],
        to: [6618, 2404],
    },
    SponsorLogoTextSegment::Quad {
        control: [6618, 2763],
        to: [6394, 2953],
    },
    SponsorLogoTextSegment::Quad {
        control: [6218, 3101],
        to: [5999, 3175],
    },
    SponsorLogoTextSegment::Quad {
        control: [5775, 3254],
        to: [5540, 3254],
    },
    SponsorLogoTextSegment::Quad {
        control: [5364, 3254],
        to: [5196, 3213],
    },
    SponsorLogoTextSegment::Quad {
        control: [5009, 3120],
        to: [4885, 2968],
    },
    SponsorLogoTextSegment::Quad {
        control: [4739, 2791],
        to: [4743, 2599],
    },
    SponsorLogoTextSegment::Quad {
        control: [4786, 2305],
        to: [4949, 2076],
    },
    SponsorLogoTextSegment::Quad {
        control: [5119, 1836],
        to: [5386, 1733],
    },
    SponsorLogoTextSegment::Quad {
        control: [5461, 1725],
        to: [5510, 1682],
    },
    SponsorLogoTextSegment::Line([5526, 1666]),
    SponsorLogoTextSegment::Quad {
        control: [5345, 1673],
        to: [5255, 1569],
    },
    SponsorLogoTextSegment::Quad {
        control: [5205, 1514],
        to: [5205, 1415],
    },
];

const DARK_CONTOUR_14: [SponsorLogoTextSegment; 9] = [
    SponsorLogoTextSegment::Line([5738, 1528]),
    SponsorLogoTextSegment::Quad {
        control: [5844, 1470],
        to: [5883, 1440],
    },
    SponsorLogoTextSegment::Quad {
        control: [5951, 1380],
        to: [5953, 1301],
    },
    SponsorLogoTextSegment::Line([5941, 1252]),
    SponsorLogoTextSegment::Quad {
        control: [5788, 1175],
        to: [5635, 1175],
    },
    SponsorLogoTextSegment::Quad {
        control: [5505, 1175],
        to: [5401, 1232],
    },
    SponsorLogoTextSegment::Quad {
        control: [5321, 1304],
        to: [5321, 1500],
    },
    SponsorLogoTextSegment::Quad {
        control: [5380, 1579],
        to: [5507, 1568],
    },
    SponsorLogoTextSegment::Line([5629, 1548]),
];

const DARK_CONTOUR_15: [SponsorLogoTextSegment; 17] = [
    SponsorLogoTextSegment::Quad {
        control: [6086, 1749],
        to: [5853, 1749],
    },
    SponsorLogoTextSegment::Line([5716, 1760]),
    SponsorLogoTextSegment::Line([5691, 1760]),
    SponsorLogoTextSegment::Quad {
        control: [5658, 1760],
        to: [5605, 1801],
    },
    SponsorLogoTextSegment::Quad {
        control: [5553, 1842],
        to: [5525, 1842],
    },
    SponsorLogoTextSegment::Quad {
        control: [5499, 1842],
        to: [5475, 1821],
    },
    SponsorLogoTextSegment::Quad {
        control: [5373, 1835],
        to: [5268, 1916],
    },
    SponsorLogoTextSegment::Line([5085, 2071]),
    SponsorLogoTextSegment::Quad {
        control: [4855, 2214],
        to: [4855, 2584],
    },
    SponsorLogoTextSegment::Quad {
        control: [4855, 2694],
        to: [4875, 2807],
    },
    SponsorLogoTextSegment::Line([4957, 2956]),
    SponsorLogoTextSegment::Quad {
        control: [5007, 3034],
        to: [5083, 3049],
    },
    SponsorLogoTextSegment::Quad {
        control: [5268, 3167],
        to: [5503, 3167],
    },
    SponsorLogoTextSegment::Quad {
        control: [5815, 3167],
        to: [6232, 2961],
    },
    SponsorLogoTextSegment::Quad {
        control: [6543, 2799],
        to: [6543, 2291],
    },
    SponsorLogoTextSegment::Line([6538, 2165]),
    SponsorLogoTextSegment::Quad {
        control: [6469, 1965],
        to: [6268, 1851],
    },
];

const DARK_CONTOUR_16: [SponsorLogoTextSegment; 4] = [
    SponsorLogoTextSegment::Line([4830, 2705]),
    SponsorLogoTextSegment::Line([4832, 2564]),
    SponsorLogoTextSegment::Line([4827, 2619]),
    SponsorLogoTextSegment::Line([4824, 2656]),
];

const DARK_CONTOUR_17: [SponsorLogoTextSegment; 24] = [
    SponsorLogoTextSegment::Quad {
        control: [6250, 690],
        to: [6250, 853],
    },
    SponsorLogoTextSegment::Quad {
        control: [6250, 1032],
        to: [6236, 1167],
    },
    SponsorLogoTextSegment::Quad {
        control: [6375, 1047],
        to: [6505, 1047],
    },
    SponsorLogoTextSegment::Quad {
        control: [6557, 1047],
        to: [6604, 1066],
    },
    SponsorLogoTextSegment::Quad {
        control: [6695, 1087],
        to: [6768, 1177],
    },
    SponsorLogoTextSegment::Quad {
        control: [6801, 1265],
        to: [6811, 1401],
    },
    SponsorLogoTextSegment::Line([6828, 1632]),
    SponsorLogoTextSegment::Line([6785, 1661]),
    SponsorLogoTextSegment::Quad {
        control: [6743, 1586],
        to: [6733, 1473],
    },
    SponsorLogoTextSegment::Line([6719, 1284]),
    SponsorLogoTextSegment::Quad {
        control: [6706, 1177],
        to: [6535, 1123],
    },
    SponsorLogoTextSegment::Quad {
        control: [6266, 1170],
        to: [6232, 1309],
    },
    SponsorLogoTextSegment::Quad {
        control: [6225, 1346],
        to: [6225, 1416],
    },
    SponsorLogoTextSegment::Line([6225, 1464]),
    SponsorLogoTextSegment::Line([6225, 1516]),
    SponsorLogoTextSegment::Quad {
        control: [6225, 1614],
        to: [6198, 1673],
    },
    SponsorLogoTextSegment::Quad {
        control: [6130, 1652],
        to: [6130, 1601],
    },
    SponsorLogoTextSegment::Line([6140, 1548]),
    SponsorLogoTextSegment::Line([6148, 1496]),
    SponsorLogoTextSegment::Line([6148, 1478]),
    SponsorLogoTextSegment::Line([6161, 1054]),
    SponsorLogoTextSegment::Line([6171, 625]),
    SponsorLogoTextSegment::Quad {
        control: [6171, 390],
        to: [6155, 173],
    },
    SponsorLogoTextSegment::Line([6218, 133]),
];

const DARK_CONTOUR_18: [SponsorLogoTextSegment; 29] = [
    SponsorLogoTextSegment::Line([7254, 1091]),
    SponsorLogoTextSegment::Line([7182, 1103]),
    SponsorLogoTextSegment::Quad {
        control: [7176, 1209],
        to: [7194, 1368],
    },
    SponsorLogoTextSegment::Line([7221, 1652]),
    SponsorLogoTextSegment::Line([7204, 1657]),
    SponsorLogoTextSegment::Line([7188, 1669]),
    SponsorLogoTextSegment::Quad {
        control: [7128, 1619],
        to: [7128, 1528],
    },
    SponsorLogoTextSegment::Line([7128, 1448]),
    SponsorLogoTextSegment::Quad {
        control: [7132, 1408],
        to: [7127, 1387],
    },
    SponsorLogoTextSegment::Quad {
        control: [7117, 1359],
        to: [7113, 1303],
    },
    SponsorLogoTextSegment::Line([7111, 1217]),
    SponsorLogoTextSegment::Quad {
        control: [7105, 1121],
        to: [7059, 1106],
    },
    SponsorLogoTextSegment::Quad {
        control: [7033, 1122],
        to: [6995, 1122],
    },
    SponsorLogoTextSegment::Line([6945, 1118]),
    SponsorLogoTextSegment::Line([6895, 1113]),
    SponsorLogoTextSegment::Line([6845, 1094]),
    SponsorLogoTextSegment::Quad {
        control: [6842, 1061],
        to: [6856, 1047],
    },
    SponsorLogoTextSegment::Line([6886, 1026]),
    SponsorLogoTextSegment::Line([6998, 1017]),
    SponsorLogoTextSegment::Line([7120, 992]),
    SponsorLogoTextSegment::Quad {
        control: [7111, 983],
        to: [7102, 914],
    },
    SponsorLogoTextSegment::Quad {
        control: [7102, 859],
        to: [7122, 812],
    },
    SponsorLogoTextSegment::Quad {
        control: [7169, 839],
        to: [7175, 885],
    },
    SponsorLogoTextSegment::Line([7173, 955]),
    SponsorLogoTextSegment::Quad {
        control: [7172, 992],
        to: [7183, 1006],
    },
    SponsorLogoTextSegment::Quad {
        control: [7195, 1019],
        to: [7230, 1017],
    },
    SponsorLogoTextSegment::Line([7372, 1000]),
    SponsorLogoTextSegment::Quad {
        control: [7461, 997],
        to: [7527, 1044],
    },
    SponsorLogoTextSegment::Quad {
        control: [7463, 1085],
        to: [7339, 1087],
    },
];

const DARK_CONTOURS: [SponsorLogoTextContour; 19] = [
    SponsorLogoTextContour {
        start: [219, 1195],
        segments: &DARK_CONTOUR_0,
    },
    SponsorLogoTextContour {
        start: [1366, 1061],
        segments: &DARK_CONTOUR_1,
    },
    SponsorLogoTextContour {
        start: [1084, 1264],
        segments: &DARK_CONTOUR_2,
    },
    SponsorLogoTextContour {
        start: [2155, 1600],
        segments: &DARK_CONTOUR_3,
    },
    SponsorLogoTextContour {
        start: [1992, 1113],
        segments: &DARK_CONTOUR_4,
    },
    SponsorLogoTextContour {
        start: [3665, 119],
        segments: &DARK_CONTOUR_5,
    },
    SponsorLogoTextContour {
        start: [3002, 1546],
        segments: &DARK_CONTOUR_6,
    },
    SponsorLogoTextContour {
        start: [4206, 1061],
        segments: &DARK_CONTOUR_7,
    },
    SponsorLogoTextContour {
        start: [3924, 1264],
        segments: &DARK_CONTOUR_8,
    },
    SponsorLogoTextContour {
        start: [4770, 426],
        segments: &DARK_CONTOUR_9,
    },
    SponsorLogoTextContour {
        start: [5062, 909],
        segments: &DARK_CONTOUR_10,
    },
    SponsorLogoTextContour {
        start: [4960, 1020],
        segments: &DARK_CONTOUR_11,
    },
    SponsorLogoTextContour {
        start: [4970, 1231],
        segments: &DARK_CONTOUR_12,
    },
    SponsorLogoTextContour {
        start: [5205, 1415],
        segments: &DARK_CONTOUR_13,
    },
    SponsorLogoTextContour {
        start: [5629, 1548],
        segments: &DARK_CONTOUR_14,
    },
    SponsorLogoTextContour {
        start: [6268, 1851],
        segments: &DARK_CONTOUR_15,
    },
    SponsorLogoTextContour {
        start: [4824, 2656],
        segments: &DARK_CONTOUR_16,
    },
    SponsorLogoTextContour {
        start: [6218, 133],
        segments: &DARK_CONTOUR_17,
    },
    SponsorLogoTextContour {
        start: [7339, 1087],
        segments: &DARK_CONTOUR_18,
    },
];

pub const DARK: SponsorLogoTextDefinition = SponsorLogoTextDefinition {
    text: "neodelight",
    define_text_id: 33,
    font_ids: &DARK_FONT_IDS,
    color_rgb: [51, 102, 0],
    bounds_centipx: [155, 8045, 80, 3255],
    contours: &DARK_CONTOURS,
};

const OLIVE_FONT_IDS: [u16; 1] = [32];

const OLIVE_CONTOUR_0: [SponsorLogoTextSegment; 27] = [
    SponsorLogoTextSegment::Quad {
        control: [325, 1079],
        to: [499, 1079],
    },
    SponsorLogoTextSegment::Quad {
        control: [602, 1079],
        to: [705, 1123],
    },
    SponsorLogoTextSegment::Quad {
        control: [780, 1189],
        to: [800, 1317],
    },
    SponsorLogoTextSegment::Quad {
        control: [807, 1342],
        to: [811, 1502],
    },
    SponsorLogoTextSegment::Quad {
        control: [814, 1582],
        to: [825, 1637],
    },
    SponsorLogoTextSegment::Quad {
        control: [813, 1665],
        to: [793, 1665],
    },
    SponsorLogoTextSegment::Quad {
        control: [773, 1665],
        to: [752, 1642],
    },
    SponsorLogoTextSegment::Quad {
        control: [739, 1590],
        to: [737, 1495],
    },
    SponsorLogoTextSegment::Line([729, 1351]),
    SponsorLogoTextSegment::Quad {
        control: [707, 1180],
        to: [591, 1154],
    },
    SponsorLogoTextSegment::Line([379, 1169]),
    SponsorLogoTextSegment::Quad {
        control: [272, 1197],
        to: [232, 1289],
    },
    SponsorLogoTextSegment::Line([231, 1321]),
    SponsorLogoTextSegment::Quad {
        control: [231, 1360],
        to: [245, 1439],
    },
    SponsorLogoTextSegment::Quad {
        control: [258, 1517],
        to: [258, 1558],
    },
    SponsorLogoTextSegment::Quad {
        control: [258, 1627],
        to: [231, 1687],
    },
    SponsorLogoTextSegment::Line([177, 1687]),
    SponsorLogoTextSegment::Line([183, 1590]),
    SponsorLogoTextSegment::Quad {
        control: [183, 1531],
        to: [170, 1412],
    },
    SponsorLogoTextSegment::Quad {
        control: [156, 1292],
        to: [156, 1231],
    },
    SponsorLogoTextSegment::Line([156, 1150]),
    SponsorLogoTextSegment::Line([156, 1125]),
    SponsorLogoTextSegment::Line([156, 1100]),
    SponsorLogoTextSegment::Quad {
        control: [156, 1059],
        to: [199, 1050],
    },
    SponsorLogoTextSegment::Quad {
        control: [231, 1075],
        to: [231, 1108],
    },
    SponsorLogoTextSegment::Line([223, 1153]),
    SponsorLogoTextSegment::Quad {
        control: [217, 1188],
        to: [219, 1195],
    },
];

const OLIVE_CONTOUR_1: [SponsorLogoTextSegment; 18] = [
    SponsorLogoTextSegment::Quad {
        control: [1467, 1051],
        to: [1554, 1105],
    },
    SponsorLogoTextSegment::Quad {
        control: [1639, 1159],
        to: [1684, 1253],
    },
    SponsorLogoTextSegment::Line([1688, 1273]),
    SponsorLogoTextSegment::Line([1693, 1293]),
    SponsorLogoTextSegment::Quad {
        control: [1693, 1307],
        to: [1680, 1323],
    },
    SponsorLogoTextSegment::Quad {
        control: [1353, 1381],
        to: [1013, 1509],
    },
    SponsorLogoTextSegment::Quad {
        control: [1019, 1537],
        to: [1061, 1577],
    },
    SponsorLogoTextSegment::Line([1214, 1586]),
    SponsorLogoTextSegment::Quad {
        control: [1526, 1586],
        to: [1706, 1430],
    },
    SponsorLogoTextSegment::Line([1722, 1427]),
    SponsorLogoTextSegment::Quad {
        control: [1734, 1427],
        to: [1763, 1460],
    },
    SponsorLogoTextSegment::Quad {
        control: [1569, 1679],
        to: [1168, 1701],
    },
    SponsorLogoTextSegment::Line([1066, 1676]),
    SponsorLogoTextSegment::Quad {
        control: [1006, 1658],
        to: [971, 1623],
    },
    SponsorLogoTextSegment::Quad {
        control: [919, 1551],
        to: [919, 1447],
    },
    SponsorLogoTextSegment::Quad {
        control: [919, 1401],
        to: [930, 1355],
    },
    SponsorLogoTextSegment::Quad {
        control: [1106, 1058],
        to: [1322, 1058],
    },
    SponsorLogoTextSegment::Line([1366, 1061]),
];

const OLIVE_CONTOUR_2: [SponsorLogoTextSegment; 7] = [
    SponsorLogoTextSegment::Quad {
        control: [1026, 1347],
        to: [1008, 1414],
    },
    SponsorLogoTextSegment::Quad {
        control: [1077, 1380],
        to: [1136, 1374],
    },
    SponsorLogoTextSegment::Line([1358, 1294]),
    SponsorLogoTextSegment::Quad {
        control: [1492, 1245],
        to: [1572, 1228],
    },
    SponsorLogoTextSegment::Quad {
        control: [1527, 1168],
        to: [1401, 1168],
    },
    SponsorLogoTextSegment::Line([1231, 1190]),
    SponsorLogoTextSegment::Quad {
        control: [1141, 1210],
        to: [1084, 1264],
    },
];

const OLIVE_CONTOUR_3: [SponsorLogoTextSegment; 12] = [
    SponsorLogoTextSegment::Quad {
        control: [2280, 1600],
        to: [2391, 1551],
    },
    SponsorLogoTextSegment::Quad {
        control: [2503, 1503],
        to: [2582, 1413],
    },
    SponsorLogoTextSegment::Quad {
        control: [2633, 1355],
        to: [2633, 1276],
    },
    SponsorLogoTextSegment::Quad {
        control: [2633, 1222],
        to: [2609, 1177],
    },
    SponsorLogoTextSegment::Quad {
        control: [2546, 1066],
        to: [2392, 1066],
    },
    SponsorLogoTextSegment::Quad {
        control: [2310, 1066],
        to: [2242, 1105],
    },
    SponsorLogoTextSegment::Quad {
        control: [2206, 1099],
        to: [2161, 1119],
    },
    SponsorLogoTextSegment::Line([2085, 1160]),
    SponsorLogoTextSegment::Quad {
        control: [2014, 1195],
        to: [1967, 1275],
    },
    SponsorLogoTextSegment::Quad {
        control: [1920, 1354],
        to: [1922, 1439],
    },
    SponsorLogoTextSegment::Quad {
        control: [1924, 1526],
        to: [2043, 1591],
    },
    SponsorLogoTextSegment::Line([2155, 1600]),
];

const OLIVE_CONTOUR_4: [SponsorLogoTextSegment; 11] = [
    SponsorLogoTextSegment::Quad {
        control: [2156, 991],
        to: [2353, 991],
    },
    SponsorLogoTextSegment::Quad {
        control: [2546, 991],
        to: [2670, 1121],
    },
    SponsorLogoTextSegment::Quad {
        control: [2708, 1201],
        to: [2708, 1270],
    },
    SponsorLogoTextSegment::Quad {
        control: [2708, 1320],
        to: [2686, 1368],
    },
    SponsorLogoTextSegment::Quad {
        control: [2631, 1495],
        to: [2492, 1587],
    },
    SponsorLogoTextSegment::Quad {
        control: [2376, 1679],
        to: [2234, 1676],
    },
    SponsorLogoTextSegment::Line([2165, 1682]),
    SponsorLogoTextSegment::Quad {
        control: [1957, 1682],
        to: [1858, 1543],
    },
    SponsorLogoTextSegment::Quad {
        control: [1845, 1491],
        to: [1845, 1435],
    },
    SponsorLogoTextSegment::Quad {
        control: [1845, 1339],
        to: [1883, 1252],
    },
    SponsorLogoTextSegment::Quad {
        control: [1921, 1166],
        to: [1992, 1113],
    },
];

const OLIVE_CONTOUR_5: [SponsorLogoTextSegment; 23] = [
    SponsorLogoTextSegment::Quad {
        control: [3630, 376],
        to: [3626, 737],
    },
    SponsorLogoTextSegment::Line([3626, 1389]),
    SponsorLogoTextSegment::Quad {
        control: [3611, 1428],
        to: [3619, 1532],
    },
    SponsorLogoTextSegment::Line([3623, 1576]),
    SponsorLogoTextSegment::Quad {
        control: [3623, 1642],
        to: [3572, 1680],
    },
    SponsorLogoTextSegment::Quad {
        control: [3543, 1639],
        to: [3540, 1586],
    },
    SponsorLogoTextSegment::Line([3535, 1508]),
    SponsorLogoTextSegment::Quad {
        control: [3447, 1580],
        to: [3386, 1609],
    },
    SponsorLogoTextSegment::Quad {
        control: [3281, 1664],
        to: [3185, 1676],
    },
    SponsorLogoTextSegment::Quad {
        control: [3142, 1693],
        to: [3094, 1693],
    },
    SponsorLogoTextSegment::Quad {
        control: [3037, 1693],
        to: [2968, 1668],
    },
    SponsorLogoTextSegment::Line([2846, 1616]),
    SponsorLogoTextSegment::Quad {
        control: [2784, 1548],
        to: [2784, 1447],
    },
    SponsorLogoTextSegment::Line([2786, 1410]),
    SponsorLogoTextSegment::Quad {
        control: [2903, 1168],
        to: [3316, 1100],
    },
    SponsorLogoTextSegment::Line([3447, 1107]),
    SponsorLogoTextSegment::Quad {
        control: [3528, 1115],
        to: [3550, 1151],
    },
    SponsorLogoTextSegment::Quad {
        control: [3544, 1059],
        to: [3554, 884],
    },
    SponsorLogoTextSegment::Line([3564, 608]),
    SponsorLogoTextSegment::Line([3557, 441]),
    SponsorLogoTextSegment::Line([3564, 259]),
    SponsorLogoTextSegment::Quad {
        control: [3572, 147],
        to: [3635, 82],
    },
    SponsorLogoTextSegment::Line([3665, 119]),
];

const OLIVE_CONTOUR_6: [SponsorLogoTextSegment; 12] = [
    SponsorLogoTextSegment::Quad {
        control: [3255, 1546],
        to: [3478, 1414],
    },
    SponsorLogoTextSegment::Quad {
        control: [3488, 1410],
        to: [3492, 1379],
    },
    SponsorLogoTextSegment::Quad {
        control: [3496, 1346],
        to: [3523, 1340],
    },
    SponsorLogoTextSegment::Quad {
        control: [3530, 1327],
        to: [3520, 1291],
    },
    SponsorLogoTextSegment::Line([3495, 1241]),
    SponsorLogoTextSegment::Line([3465, 1200]),
    SponsorLogoTextSegment::Line([3369, 1185]),
    SponsorLogoTextSegment::Quad {
        control: [3201, 1185],
        to: [2985, 1347],
    },
    SponsorLogoTextSegment::Quad {
        control: [2980, 1368],
        to: [2937, 1429],
    },
    SponsorLogoTextSegment::Quad {
        control: [2907, 1475],
        to: [2907, 1505],
    },
    SponsorLogoTextSegment::Quad {
        control: [2907, 1525],
        to: [2916, 1542],
    },
    SponsorLogoTextSegment::Line([3002, 1546]),
];

const OLIVE_CONTOUR_7: [SponsorLogoTextSegment; 18] = [
    SponsorLogoTextSegment::Quad {
        control: [4307, 1051],
        to: [4394, 1105],
    },
    SponsorLogoTextSegment::Quad {
        control: [4479, 1159],
        to: [4524, 1253],
    },
    SponsorLogoTextSegment::Line([4528, 1273]),
    SponsorLogoTextSegment::Line([4533, 1293]),
    SponsorLogoTextSegment::Quad {
        control: [4533, 1307],
        to: [4520, 1323],
    },
    SponsorLogoTextSegment::Quad {
        control: [4193, 1381],
        to: [3853, 1509],
    },
    SponsorLogoTextSegment::Quad {
        control: [3859, 1537],
        to: [3901, 1577],
    },
    SponsorLogoTextSegment::Line([4054, 1586]),
    SponsorLogoTextSegment::Quad {
        control: [4366, 1586],
        to: [4546, 1430],
    },
    SponsorLogoTextSegment::Line([4562, 1427]),
    SponsorLogoTextSegment::Quad {
        control: [4574, 1427],
        to: [4603, 1460],
    },
    SponsorLogoTextSegment::Quad {
        control: [4409, 1679],
        to: [4008, 1701],
    },
    SponsorLogoTextSegment::Line([3906, 1676]),
    SponsorLogoTextSegment::Quad {
        control: [3846, 1658],
        to: [3811, 1623],
    },
    SponsorLogoTextSegment::Quad {
        control: [3759, 1551],
        to: [3759, 1447],
    },
    SponsorLogoTextSegment::Quad {
        control: [3759, 1401],
        to: [3770, 1355],
    },
    SponsorLogoTextSegment::Quad {
        control: [3946, 1058],
        to: [4162, 1058],
    },
    SponsorLogoTextSegment::Line([4206, 1061]),
];

const OLIVE_CONTOUR_8: [SponsorLogoTextSegment; 7] = [
    SponsorLogoTextSegment::Quad {
        control: [3866, 1347],
        to: [3848, 1414],
    },
    SponsorLogoTextSegment::Quad {
        control: [3917, 1380],
        to: [3976, 1374],
    },
    SponsorLogoTextSegment::Line([4198, 1294]),
    SponsorLogoTextSegment::Quad {
        control: [4332, 1245],
        to: [4412, 1228],
    },
    SponsorLogoTextSegment::Quad {
        control: [4367, 1168],
        to: [4241, 1168],
    },
    SponsorLogoTextSegment::Line([4071, 1190]),
    SponsorLogoTextSegment::Quad {
        control: [3981, 1210],
        to: [3924, 1264],
    },
];

const OLIVE_CONTOUR_9: [SponsorLogoTextSegment; 13] = [
    SponsorLogoTextSegment::Quad {
        control: [4762, 529],
        to: [4762, 652],
    },
    SponsorLogoTextSegment::Quad {
        control: [4762, 784],
        to: [4776, 1034],
    },
    SponsorLogoTextSegment::Quad {
        control: [4789, 1285],
        to: [4789, 1408],
    },
    SponsorLogoTextSegment::Quad {
        control: [4789, 1529],
        to: [4779, 1640],
    },
    SponsorLogoTextSegment::Line([4713, 1669]),
    SponsorLogoTextSegment::Quad {
        control: [4703, 1471],
        to: [4687, 641],
    },
    SponsorLogoTextSegment::Quad {
        control: [4686, 567],
        to: [4699, 428],
    },
    SponsorLogoTextSegment::Quad {
        control: [4711, 291],
        to: [4711, 226],
    },
    SponsorLogoTextSegment::Line([4711, 159]),
    SponsorLogoTextSegment::Quad {
        control: [4723, 124],
        to: [4748, 124],
    },
    SponsorLogoTextSegment::Quad {
        control: [4761, 124],
        to: [4772, 139],
    },
    SponsorLogoTextSegment::Quad {
        control: [4789, 187],
        to: [4789, 248],
    },
    SponsorLogoTextSegment::Quad {
        control: [4789, 307],
        to: [4770, 426],
    },
];

const OLIVE_CONTOUR_10: [SponsorLogoTextSegment; 9] = [
    SponsorLogoTextSegment::Line([5034, 917]),
    SponsorLogoTextSegment::Quad {
        control: [5013, 925],
        to: [4997, 925],
    },
    SponsorLogoTextSegment::Quad {
        control: [4970, 925],
        to: [4958, 908],
    },
    SponsorLogoTextSegment::Line([4944, 919]),
    SponsorLogoTextSegment::Line([4935, 928]),
    SponsorLogoTextSegment::Quad {
        control: [4985, 944],
        to: [5011, 944],
    },
    SponsorLogoTextSegment::Quad {
        control: [5040, 944],
        to: [5059, 930],
    },
    SponsorLogoTextSegment::Line([5061, 914]),
    SponsorLogoTextSegment::Line([5062, 909]),
];

const OLIVE_CONTOUR_11: [SponsorLogoTextSegment; 8] = [
    SponsorLogoTextSegment::Line([4890, 1004]),
    SponsorLogoTextSegment::Quad {
        control: [4848, 983],
        to: [4848, 936],
    },
    SponsorLogoTextSegment::Quad {
        control: [4848, 911],
        to: [4864, 891],
    },
    SponsorLogoTextSegment::Quad {
        control: [4914, 822],
        to: [5007, 822],
    },
    SponsorLogoTextSegment::Quad {
        control: [5109, 822],
        to: [5141, 902],
    },
    SponsorLogoTextSegment::Quad {
        control: [5148, 971],
        to: [5103, 1012],
    },
    SponsorLogoTextSegment::Quad {
        control: [5071, 1031],
        to: [5031, 1031],
    },
    SponsorLogoTextSegment::Line([4960, 1020]),
];

const OLIVE_CONTOUR_12: [SponsorLogoTextSegment; 11] = [
    SponsorLogoTextSegment::Line([4964, 1166]),
    SponsorLogoTextSegment::Quad {
        control: [4966, 1122],
        to: [5004, 1096],
    },
    SponsorLogoTextSegment::Quad {
        control: [5052, 1177],
        to: [5052, 1325],
    },
    SponsorLogoTextSegment::Line([5047, 1429]),
    SponsorLogoTextSegment::Line([5021, 1495]),
    SponsorLogoTextSegment::Quad {
        control: [5032, 1477],
        to: [5042, 1556],
    },
    SponsorLogoTextSegment::Quad {
        control: [5045, 1627],
        to: [5000, 1666],
    },
    SponsorLogoTextSegment::Quad {
        control: [4964, 1592],
        to: [4964, 1507],
    },
    SponsorLogoTextSegment::Line([4971, 1407]),
    SponsorLogoTextSegment::Line([4977, 1309]),
    SponsorLogoTextSegment::Line([4970, 1231]),
];

const OLIVE_CONTOUR_13: [SponsorLogoTextSegment; 26] = [
    SponsorLogoTextSegment::Quad {
        control: [5205, 1328],
        to: [5243, 1259],
    },
    SponsorLogoTextSegment::Quad {
        control: [5331, 1096],
        to: [5564, 1096],
    },
    SponsorLogoTextSegment::Line([5843, 1130]),
    SponsorLogoTextSegment::Quad {
        control: [5934, 1144],
        to: [6016, 1214],
    },
    SponsorLogoTextSegment::Line([6019, 1269]),
    SponsorLogoTextSegment::Quad {
        control: [6019, 1352],
        to: [5979, 1428],
    },
    SponsorLogoTextSegment::Quad {
        control: [5939, 1505],
        to: [5873, 1544],
    },
    SponsorLogoTextSegment::Line([5750, 1617]),
    SponsorLogoTextSegment::Quad {
        control: [5679, 1648],
        to: [5619, 1644],
    },
    SponsorLogoTextSegment::Line([5619, 1659]),
    SponsorLogoTextSegment::Line([5856, 1673]),
    SponsorLogoTextSegment::Quad {
        control: [6118, 1693],
        to: [6279, 1763],
    },
    SponsorLogoTextSegment::Quad {
        control: [6505, 1862],
        to: [6579, 2071],
    },
    SponsorLogoTextSegment::Quad {
        control: [6618, 2254],
        to: [6618, 2404],
    },
    SponsorLogoTextSegment::Quad {
        control: [6618, 2763],
        to: [6394, 2953],
    },
    SponsorLogoTextSegment::Quad {
        control: [6218, 3101],
        to: [5999, 3175],
    },
    SponsorLogoTextSegment::Quad {
        control: [5775, 3254],
        to: [5540, 3254],
    },
    SponsorLogoTextSegment::Quad {
        control: [5364, 3254],
        to: [5196, 3213],
    },
    SponsorLogoTextSegment::Quad {
        control: [5009, 3120],
        to: [4885, 2968],
    },
    SponsorLogoTextSegment::Quad {
        control: [4739, 2791],
        to: [4743, 2599],
    },
    SponsorLogoTextSegment::Quad {
        control: [4786, 2305],
        to: [4949, 2076],
    },
    SponsorLogoTextSegment::Quad {
        control: [5119, 1836],
        to: [5386, 1733],
    },
    SponsorLogoTextSegment::Quad {
        control: [5461, 1725],
        to: [5510, 1682],
    },
    SponsorLogoTextSegment::Line([5526, 1666]),
    SponsorLogoTextSegment::Quad {
        control: [5345, 1673],
        to: [5255, 1569],
    },
    SponsorLogoTextSegment::Quad {
        control: [5205, 1514],
        to: [5205, 1415],
    },
];

const OLIVE_CONTOUR_14: [SponsorLogoTextSegment; 9] = [
    SponsorLogoTextSegment::Line([5738, 1528]),
    SponsorLogoTextSegment::Quad {
        control: [5844, 1470],
        to: [5883, 1440],
    },
    SponsorLogoTextSegment::Quad {
        control: [5951, 1380],
        to: [5953, 1301],
    },
    SponsorLogoTextSegment::Line([5941, 1252]),
    SponsorLogoTextSegment::Quad {
        control: [5788, 1175],
        to: [5635, 1175],
    },
    SponsorLogoTextSegment::Quad {
        control: [5505, 1175],
        to: [5401, 1232],
    },
    SponsorLogoTextSegment::Quad {
        control: [5321, 1304],
        to: [5321, 1500],
    },
    SponsorLogoTextSegment::Quad {
        control: [5380, 1579],
        to: [5507, 1568],
    },
    SponsorLogoTextSegment::Line([5629, 1548]),
];

const OLIVE_CONTOUR_15: [SponsorLogoTextSegment; 17] = [
    SponsorLogoTextSegment::Quad {
        control: [6086, 1749],
        to: [5853, 1749],
    },
    SponsorLogoTextSegment::Line([5716, 1760]),
    SponsorLogoTextSegment::Line([5691, 1760]),
    SponsorLogoTextSegment::Quad {
        control: [5658, 1760],
        to: [5605, 1801],
    },
    SponsorLogoTextSegment::Quad {
        control: [5553, 1842],
        to: [5525, 1842],
    },
    SponsorLogoTextSegment::Quad {
        control: [5499, 1842],
        to: [5475, 1821],
    },
    SponsorLogoTextSegment::Quad {
        control: [5373, 1835],
        to: [5268, 1916],
    },
    SponsorLogoTextSegment::Line([5085, 2071]),
    SponsorLogoTextSegment::Quad {
        control: [4855, 2214],
        to: [4855, 2584],
    },
    SponsorLogoTextSegment::Quad {
        control: [4855, 2694],
        to: [4875, 2807],
    },
    SponsorLogoTextSegment::Line([4957, 2956]),
    SponsorLogoTextSegment::Quad {
        control: [5007, 3034],
        to: [5083, 3049],
    },
    SponsorLogoTextSegment::Quad {
        control: [5268, 3167],
        to: [5503, 3167],
    },
    SponsorLogoTextSegment::Quad {
        control: [5815, 3167],
        to: [6232, 2961],
    },
    SponsorLogoTextSegment::Quad {
        control: [6543, 2799],
        to: [6543, 2291],
    },
    SponsorLogoTextSegment::Line([6538, 2165]),
    SponsorLogoTextSegment::Quad {
        control: [6469, 1965],
        to: [6268, 1851],
    },
];

const OLIVE_CONTOUR_16: [SponsorLogoTextSegment; 4] = [
    SponsorLogoTextSegment::Line([4830, 2705]),
    SponsorLogoTextSegment::Line([4832, 2564]),
    SponsorLogoTextSegment::Line([4827, 2619]),
    SponsorLogoTextSegment::Line([4824, 2656]),
];

const OLIVE_CONTOUR_17: [SponsorLogoTextSegment; 24] = [
    SponsorLogoTextSegment::Quad {
        control: [6250, 690],
        to: [6250, 853],
    },
    SponsorLogoTextSegment::Quad {
        control: [6250, 1032],
        to: [6236, 1167],
    },
    SponsorLogoTextSegment::Quad {
        control: [6375, 1047],
        to: [6505, 1047],
    },
    SponsorLogoTextSegment::Quad {
        control: [6557, 1047],
        to: [6604, 1066],
    },
    SponsorLogoTextSegment::Quad {
        control: [6695, 1087],
        to: [6768, 1177],
    },
    SponsorLogoTextSegment::Quad {
        control: [6801, 1265],
        to: [6811, 1401],
    },
    SponsorLogoTextSegment::Line([6828, 1632]),
    SponsorLogoTextSegment::Line([6785, 1661]),
    SponsorLogoTextSegment::Quad {
        control: [6743, 1586],
        to: [6733, 1473],
    },
    SponsorLogoTextSegment::Line([6719, 1284]),
    SponsorLogoTextSegment::Quad {
        control: [6706, 1177],
        to: [6535, 1123],
    },
    SponsorLogoTextSegment::Quad {
        control: [6266, 1170],
        to: [6232, 1309],
    },
    SponsorLogoTextSegment::Quad {
        control: [6225, 1346],
        to: [6225, 1416],
    },
    SponsorLogoTextSegment::Line([6225, 1464]),
    SponsorLogoTextSegment::Line([6225, 1516]),
    SponsorLogoTextSegment::Quad {
        control: [6225, 1614],
        to: [6198, 1673],
    },
    SponsorLogoTextSegment::Quad {
        control: [6130, 1652],
        to: [6130, 1601],
    },
    SponsorLogoTextSegment::Line([6140, 1548]),
    SponsorLogoTextSegment::Line([6148, 1496]),
    SponsorLogoTextSegment::Line([6148, 1478]),
    SponsorLogoTextSegment::Line([6161, 1054]),
    SponsorLogoTextSegment::Line([6171, 625]),
    SponsorLogoTextSegment::Quad {
        control: [6171, 390],
        to: [6155, 173],
    },
    SponsorLogoTextSegment::Line([6218, 133]),
];

const OLIVE_CONTOUR_18: [SponsorLogoTextSegment; 29] = [
    SponsorLogoTextSegment::Line([7254, 1091]),
    SponsorLogoTextSegment::Line([7182, 1103]),
    SponsorLogoTextSegment::Quad {
        control: [7176, 1209],
        to: [7194, 1368],
    },
    SponsorLogoTextSegment::Line([7221, 1652]),
    SponsorLogoTextSegment::Line([7204, 1657]),
    SponsorLogoTextSegment::Line([7188, 1669]),
    SponsorLogoTextSegment::Quad {
        control: [7128, 1619],
        to: [7128, 1528],
    },
    SponsorLogoTextSegment::Line([7128, 1448]),
    SponsorLogoTextSegment::Quad {
        control: [7132, 1408],
        to: [7127, 1387],
    },
    SponsorLogoTextSegment::Quad {
        control: [7117, 1359],
        to: [7113, 1303],
    },
    SponsorLogoTextSegment::Line([7111, 1217]),
    SponsorLogoTextSegment::Quad {
        control: [7105, 1121],
        to: [7059, 1106],
    },
    SponsorLogoTextSegment::Quad {
        control: [7033, 1122],
        to: [6995, 1122],
    },
    SponsorLogoTextSegment::Line([6945, 1118]),
    SponsorLogoTextSegment::Line([6895, 1113]),
    SponsorLogoTextSegment::Line([6845, 1094]),
    SponsorLogoTextSegment::Quad {
        control: [6842, 1061],
        to: [6856, 1047],
    },
    SponsorLogoTextSegment::Line([6886, 1026]),
    SponsorLogoTextSegment::Line([6998, 1017]),
    SponsorLogoTextSegment::Line([7120, 992]),
    SponsorLogoTextSegment::Quad {
        control: [7111, 983],
        to: [7102, 914],
    },
    SponsorLogoTextSegment::Quad {
        control: [7102, 859],
        to: [7122, 812],
    },
    SponsorLogoTextSegment::Quad {
        control: [7169, 839],
        to: [7175, 885],
    },
    SponsorLogoTextSegment::Line([7173, 955]),
    SponsorLogoTextSegment::Quad {
        control: [7172, 992],
        to: [7183, 1006],
    },
    SponsorLogoTextSegment::Quad {
        control: [7195, 1019],
        to: [7230, 1017],
    },
    SponsorLogoTextSegment::Line([7372, 1000]),
    SponsorLogoTextSegment::Quad {
        control: [7461, 997],
        to: [7527, 1044],
    },
    SponsorLogoTextSegment::Quad {
        control: [7463, 1085],
        to: [7339, 1087],
    },
];

const OLIVE_CONTOURS: [SponsorLogoTextContour; 19] = [
    SponsorLogoTextContour {
        start: [219, 1195],
        segments: &OLIVE_CONTOUR_0,
    },
    SponsorLogoTextContour {
        start: [1366, 1061],
        segments: &OLIVE_CONTOUR_1,
    },
    SponsorLogoTextContour {
        start: [1084, 1264],
        segments: &OLIVE_CONTOUR_2,
    },
    SponsorLogoTextContour {
        start: [2155, 1600],
        segments: &OLIVE_CONTOUR_3,
    },
    SponsorLogoTextContour {
        start: [1992, 1113],
        segments: &OLIVE_CONTOUR_4,
    },
    SponsorLogoTextContour {
        start: [3665, 119],
        segments: &OLIVE_CONTOUR_5,
    },
    SponsorLogoTextContour {
        start: [3002, 1546],
        segments: &OLIVE_CONTOUR_6,
    },
    SponsorLogoTextContour {
        start: [4206, 1061],
        segments: &OLIVE_CONTOUR_7,
    },
    SponsorLogoTextContour {
        start: [3924, 1264],
        segments: &OLIVE_CONTOUR_8,
    },
    SponsorLogoTextContour {
        start: [4770, 426],
        segments: &OLIVE_CONTOUR_9,
    },
    SponsorLogoTextContour {
        start: [5062, 909],
        segments: &OLIVE_CONTOUR_10,
    },
    SponsorLogoTextContour {
        start: [4960, 1020],
        segments: &OLIVE_CONTOUR_11,
    },
    SponsorLogoTextContour {
        start: [4970, 1231],
        segments: &OLIVE_CONTOUR_12,
    },
    SponsorLogoTextContour {
        start: [5205, 1415],
        segments: &OLIVE_CONTOUR_13,
    },
    SponsorLogoTextContour {
        start: [5629, 1548],
        segments: &OLIVE_CONTOUR_14,
    },
    SponsorLogoTextContour {
        start: [6268, 1851],
        segments: &OLIVE_CONTOUR_15,
    },
    SponsorLogoTextContour {
        start: [4824, 2656],
        segments: &OLIVE_CONTOUR_16,
    },
    SponsorLogoTextContour {
        start: [6218, 133],
        segments: &OLIVE_CONTOUR_17,
    },
    SponsorLogoTextContour {
        start: [7339, 1087],
        segments: &OLIVE_CONTOUR_18,
    },
];

pub const OLIVE: SponsorLogoTextDefinition = SponsorLogoTextDefinition {
    text: "neodelight",
    define_text_id: 34,
    font_ids: &OLIVE_FONT_IDS,
    color_rgb: [153, 153, 0],
    bounds_centipx: [155, 8045, 80, 3255],
    contours: &OLIVE_CONTOURS,
};

// Generated from gravity_arcade.swf DefineEditText 154 and DefineFont2 153.
// Coordinates are stored as SWF text-local centipixels; divide by 100 for pixels.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoundNumberSegment {
    Line([i16; 2]),
    Quad { control: [i16; 2], to: [i16; 2] },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoundNumberContour {
    pub(super) start: [i16; 2],
    pub(super) segments: &'static [RoundNumberSegment],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RoundNumberDefinition {
    pub(super) text: &'static str,
    pub(super) define_edit_text_id: u16,
    pub(super) font_id: u16,
    pub(super) color_rgb: [u8; 3],
    pub(super) bounds_centipx: [i16; 4],
    pub(super) contours: &'static [RoundNumberContour],
}

const ONE_CONTOUR_0: [RoundNumberSegment; 21] = [
    RoundNumberSegment::Line([2490, 642]),
    RoundNumberSegment::Line([2487, 734]),
    RoundNumberSegment::Line([2487, 1080]),
    RoundNumberSegment::Quad {
        control: [2487, 1102],
        to: [2497, 1118],
    },
    RoundNumberSegment::Quad {
        control: [2506, 1133],
        to: [2522, 1141],
    },
    RoundNumberSegment::Line([2573, 1153]),
    RoundNumberSegment::Line([2624, 1159]),
    RoundNumberSegment::Line([2624, 1200]),
    RoundNumberSegment::Line([2229, 1200]),
    RoundNumberSegment::Line([2229, 1159]),
    RoundNumberSegment::Line([2284, 1154]),
    RoundNumberSegment::Line([2335, 1146]),
    RoundNumberSegment::Quad {
        control: [2354, 1140],
        to: [2363, 1127],
    },
    RoundNumberSegment::Quad {
        control: [2372, 1113],
        to: [2372, 1088],
    },
    RoundNumberSegment::Line([2372, 676]),
    RoundNumberSegment::Line([2227, 676]),
    RoundNumberSegment::Line([2227, 627]),
    RoundNumberSegment::Line([2259, 627]),
    RoundNumberSegment::Quad {
        control: [2322, 627],
        to: [2372, 604],
    },
    RoundNumberSegment::Quad {
        control: [2423, 580],
        to: [2444, 553],
    },
    RoundNumberSegment::Line([2493, 553]),
];

const ONE_CONTOURS: [RoundNumberContour; 1] = [RoundNumberContour {
    start: [2493, 553],
    segments: &ONE_CONTOUR_0,
}];

pub const ONE: RoundNumberDefinition = RoundNumberDefinition {
    text: "1",
    define_edit_text_id: 154,
    font_id: 153,
    color_rgb: [255, 255, 255],
    bounds_centipx: [-200, 5015, -200, 1820],
    contours: &ONE_CONTOURS,
};

const TWO_CONTOUR_0: [RoundNumberSegment; 34] = [
    RoundNumberSegment::Quad {
        control: [2514, 553],
        to: [2586, 606],
    },
    RoundNumberSegment::Quad {
        control: [2656, 660],
        to: [2656, 750],
    },
    RoundNumberSegment::Quad {
        control: [2656, 792],
        to: [2643, 827],
    },
    RoundNumberSegment::Quad {
        control: [2630, 862],
        to: [2602, 889],
    },
    RoundNumberSegment::Quad {
        control: [2572, 919],
        to: [2529, 941],
    },
    RoundNumberSegment::Line([2403, 1000]),
    RoundNumberSegment::Line([2308, 1048]),
    RoundNumberSegment::Quad {
        control: [2263, 1072],
        to: [2235, 1096],
    },
    RoundNumberSegment::Line([2686, 1096]),
    RoundNumberSegment::Line([2686, 1200]),
    RoundNumberSegment::Line([2137, 1200]),
    RoundNumberSegment::Line([2137, 1120]),
    RoundNumberSegment::Line([2181, 1079]),
    RoundNumberSegment::Line([2248, 1030]),
    RoundNumberSegment::Line([2304, 996]),
    RoundNumberSegment::Line([2378, 955]),
    RoundNumberSegment::Quad {
        control: [2425, 930],
        to: [2450, 912],
    },
    RoundNumberSegment::Line([2495, 866]),
    RoundNumberSegment::Quad {
        control: [2508, 847],
        to: [2516, 820],
    },
    RoundNumberSegment::Quad {
        control: [2525, 793],
        to: [2525, 743],
    },
    RoundNumberSegment::Quad {
        control: [2525, 669],
        to: [2486, 635],
    },
    RoundNumberSegment::Quad {
        control: [2446, 601],
        to: [2385, 601],
    },
    RoundNumberSegment::Quad {
        control: [2347, 601],
        to: [2313, 616],
    },
    RoundNumberSegment::Quad {
        control: [2279, 630],
        to: [2262, 657],
    },
    RoundNumberSegment::Line([2273, 702]),
    RoundNumberSegment::Line([2279, 752]),
    RoundNumberSegment::Quad {
        control: [2279, 775],
        to: [2260, 792],
    },
    RoundNumberSegment::Quad {
        control: [2241, 809],
        to: [2205, 809],
    },
    RoundNumberSegment::Quad {
        control: [2175, 809],
        to: [2160, 789],
    },
    RoundNumberSegment::Quad {
        control: [2144, 769],
        to: [2144, 735],
    },
    RoundNumberSegment::Quad {
        control: [2144, 703],
        to: [2161, 671],
    },
    RoundNumberSegment::Quad {
        control: [2179, 639],
        to: [2212, 612],
    },
    RoundNumberSegment::Quad {
        control: [2245, 586],
        to: [2291, 570],
    },
    RoundNumberSegment::Line([2395, 553]),
];

const TWO_CONTOURS: [RoundNumberContour; 1] = [RoundNumberContour {
    start: [2395, 553],
    segments: &TWO_CONTOUR_0,
}];

pub const TWO: RoundNumberDefinition = RoundNumberDefinition {
    text: "2",
    define_edit_text_id: 154,
    font_id: 153,
    color_rgb: [255, 255, 255],
    bounds_centipx: [-200, 5015, -200, 1820],
    contours: &TWO_CONTOURS,
};

const THREE_CONTOUR_0: [RoundNumberSegment; 58] = [
    RoundNumberSegment::Quad {
        control: [2571, 595],
        to: [2598, 626],
    },
    RoundNumberSegment::Quad {
        control: [2624, 656],
        to: [2634, 689],
    },
    RoundNumberSegment::Quad {
        control: [2644, 722],
        to: [2644, 745],
    },
    RoundNumberSegment::Quad {
        control: [2644, 775],
        to: [2635, 804],
    },
    RoundNumberSegment::Quad {
        control: [2627, 833],
        to: [2608, 859],
    },
    RoundNumberSegment::Quad {
        control: [2589, 886],
        to: [2555, 909],
    },
    RoundNumberSegment::Quad {
        control: [2521, 932],
        to: [2470, 946],
    },
    RoundNumberSegment::Line([2470, 955]),
    RoundNumberSegment::Line([2541, 969]),
    RoundNumberSegment::Line([2607, 1004]),
    RoundNumberSegment::Quad {
        control: [2638, 1028],
        to: [2658, 1066],
    },
    RoundNumberSegment::Quad {
        control: [2678, 1104],
        to: [2678, 1161],
    },
    RoundNumberSegment::Quad {
        control: [2678, 1274],
        to: [2587, 1344],
    },
    RoundNumberSegment::Quad {
        control: [2497, 1414],
        to: [2362, 1414],
    },
    RoundNumberSegment::Line([2262, 1400]),
    RoundNumberSegment::Quad {
        control: [2216, 1386],
        to: [2183, 1363],
    },
    RoundNumberSegment::Quad {
        control: [2150, 1339],
        to: [2133, 1309],
    },
    RoundNumberSegment::Quad {
        control: [2114, 1279],
        to: [2114, 1247],
    },
    RoundNumberSegment::Quad {
        control: [2114, 1218],
        to: [2128, 1195],
    },
    RoundNumberSegment::Quad {
        control: [2142, 1172],
        to: [2174, 1172],
    },
    RoundNumberSegment::Quad {
        control: [2209, 1172],
        to: [2228, 1189],
    },
    RoundNumberSegment::Quad {
        control: [2245, 1206],
        to: [2245, 1228],
    },
    RoundNumberSegment::Line([2239, 1276]),
    RoundNumberSegment::Line([2230, 1317]),
    RoundNumberSegment::Line([2245, 1331]),
    RoundNumberSegment::Line([2271, 1346]),
    RoundNumberSegment::Quad {
        control: [2290, 1356],
        to: [2311, 1361],
    },
    RoundNumberSegment::Line([2368, 1366]),
    RoundNumberSegment::Line([2436, 1355]),
    RoundNumberSegment::Quad {
        control: [2469, 1343],
        to: [2494, 1317],
    },
    RoundNumberSegment::Quad {
        control: [2519, 1290],
        to: [2533, 1254],
    },
    RoundNumberSegment::Quad {
        control: [2549, 1218],
        to: [2549, 1159],
    },
    RoundNumberSegment::Line([2542, 1097]),
    RoundNumberSegment::Quad {
        control: [2535, 1065],
        to: [2517, 1043],
    },
    RoundNumberSegment::Quad {
        control: [2498, 1020],
        to: [2467, 1008],
    },
    RoundNumberSegment::Line([2387, 995]),
    RoundNumberSegment::Line([2311, 995]),
    RoundNumberSegment::Line([2311, 927]),
    RoundNumberSegment::Line([2361, 927]),
    RoundNumberSegment::Quad {
        control: [2443, 927],
        to: [2478, 882],
    },
    RoundNumberSegment::Quad {
        control: [2515, 838],
        to: [2515, 749],
    },
    RoundNumberSegment::Quad {
        control: [2515, 675],
        to: [2480, 639],
    },
    RoundNumberSegment::Quad {
        control: [2446, 601],
        to: [2381, 601],
    },
    RoundNumberSegment::Line([2332, 608]),
    RoundNumberSegment::Line([2299, 622]),
    RoundNumberSegment::Line([2276, 641]),
    RoundNumberSegment::Line([2262, 655]),
    RoundNumberSegment::Line([2271, 702]),
    RoundNumberSegment::Line([2277, 754]),
    RoundNumberSegment::Quad {
        control: [2277, 775],
        to: [2259, 792],
    },
    RoundNumberSegment::Quad {
        control: [2241, 810],
        to: [2204, 810],
    },
    RoundNumberSegment::Quad {
        control: [2174, 810],
        to: [2160, 788],
    },
    RoundNumberSegment::Quad {
        control: [2144, 765],
        to: [2144, 736],
    },
    RoundNumberSegment::Quad {
        control: [2144, 705],
        to: [2162, 674],
    },
    RoundNumberSegment::Quad {
        control: [2178, 642],
        to: [2212, 614],
    },
    RoundNumberSegment::Quad {
        control: [2245, 587],
        to: [2293, 570],
    },
    RoundNumberSegment::Line([2405, 553]),
    RoundNumberSegment::Quad {
        control: [2476, 553],
        to: [2524, 574],
    },
];

const THREE_CONTOURS: [RoundNumberContour; 1] = [RoundNumberContour {
    start: [2524, 574],
    segments: &THREE_CONTOUR_0,
}];

pub const THREE: RoundNumberDefinition = RoundNumberDefinition {
    text: "3",
    define_edit_text_id: 154,
    font_id: 153,
    color_rgb: [255, 255, 255],
    bounds_centipx: [-200, 5015, -200, 1820],
    contours: &THREE_CONTOURS,
};

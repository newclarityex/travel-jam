use bevy::prelude::*;

pub const HILL_COORDS: &'static [Vec2] = &[
    // // Bottom Right
    // Vec2::new(600., -270.),
    // // Bottom Left
    // Vec2::new(-600., -270.),
    // // Top Left
    // Vec2::new(-600., 238.),
    // // Ledge
    // Vec2::new(-343., 238.),
    // Vec2::new(-337., 237.),
    // Vec2::new(-331., 235.),
    // Vec2::new(-329., 234.),
    // Vec2::new(-200., 107.),
    // Vec2::new(-129., 48.),
    // Vec2::new(-49., -9.),
    // Vec2::new(109., -93.),
    // Vec2::new(257., -150.),
    // // Vec2::new(-49.5, -500.),
    // Vec2::new(470., -208.),
    // Vec2::new(600., -270.),
    // new
    Vec2::new(-727.0, 269.0),
    Vec2::new(-726.0, -302.0),
    Vec2::new(728.0, -301.0),
    Vec2::new(612.0, -180.0),
    // Vec2::new(595.0, -173.0),
    // Vec2::new(591.0, -172.0),
    // Vec2::new(586.0, -171.0),
    Vec2::new(582.0, -170.0),
    // Vec2::new(578.0, -169.0),
    // Vec2::new(574.0, -168.0),
    // Vec2::new(570.0, -167.0),
    // Vec2::new(564.0, -166.0),
    // Vec2::new(560.0, -165.0),
    Vec2::new(556.0, -164.0),
    // Vec2::new(552.0, -163.0),
    // Vec2::new(548.0, -162.0),
    // Vec2::new(544.0, -161.0),
    // Vec2::new(540.0, -160.0),
    // Vec2::new(536.0, -159.0),
    Vec2::new(532.0, -158.0),
    // Vec2::new(528.0, -157.0),
    // Vec2::new(524.0, -156.0),
    // Vec2::new(520.0, -155.0),
    // Vec2::new(516.0, -154.0),
    Vec2::new(512.0, -153.0),
    // Vec2::new(508.0, -152.0),
    // Vec2::new(504.0, -151.0),
    // Vec2::new(501.0, -150.0),
    // Vec2::new(497.0, -149.0),
    // Vec2::new(493.0, -148.0),
    // Vec2::new(489.0, -147.0),
    // Vec2::new(485.0, -146.0),
    // Vec2::new(482.0, -145.0),
    // Vec2::new(479.0, -144.0),
    // Vec2::new(475.0, -143.0),
    // Vec2::new(472.0, -142.0),
    // Vec2::new(467.0, -141.0),
    // Vec2::new(463.0, -140.0),
    // Vec2::new(459.0, -139.0),
    // Vec2::new(456.0, -138.0),
    // Vec2::new(452.0, -137.0),
    // Vec2::new(448.0, -136.0),
    // Vec2::new(444.0, -135.0),
    // Vec2::new(441.0, -134.0),
    // Vec2::new(437.0, -133.0),
    // Vec2::new(435.0, -132.0),
    // Vec2::new(431.0, -131.0),
    // Vec2::new(427.0, -130.0),
    // Vec2::new(424.0, -129.0),
    // Vec2::new(420.0, -128.0),
    // Vec2::new(416.0, -127.0),
    // Vec2::new(414.0, -126.0),
    // Vec2::new(410.0, -125.0),
    // Vec2::new(407.0, -124.0),
    // Vec2::new(405.0, -123.0),
    // Vec2::new(401.0, -122.0),
    // Vec2::new(397.0, -121.0),
    // Vec2::new(393.0, -120.0),
    // Vec2::new(391.0, -119.0),
    // Vec2::new(388.0, -118.0),
    // Vec2::new(384.0, -117.0),
    // Vec2::new(382.0, -116.0),
    Vec2::new(378.0, -115.0),
    // Vec2::new(375.0, -114.0),
    // Vec2::new(373.0, -113.0),
    // Vec2::new(369.0, -112.0),
    // Vec2::new(367.0, -111.0),
    // Vec2::new(364.0, -110.0),
    // Vec2::new(360.0, -109.0),
    // Vec2::new(358.0, -108.0),
    // Vec2::new(355.0, -107.0),
    // Vec2::new(353.0, -106.0),
    // Vec2::new(349.0, -105.0),
    // Vec2::new(346.0, -104.0),
    // Vec2::new(344.0, -103.0),
    // Vec2::new(341.0, -102.0),
    // Vec2::new(339.0, -101.0),
    // Vec2::new(335.0, -100.0),
    // Vec2::new(332.0, -99.0),
    // Vec2::new(330.0, -98.0),
    // Vec2::new(326.0, -97.0),
    // Vec2::new(324.0, -96.0),
    // Vec2::new(321.0, -95.0),
    // Vec2::new(319.0, -94.0),
    // Vec2::new(315.0, -93.0),
    // Vec2::new(314.0, -92.0),
    // Vec2::new(312.0, -91.0),
    // Vec2::new(308.0, -90.0),
    // Vec2::new(306.0, -89.0),
    // Vec2::new(303.0, -88.0),
    // Vec2::new(301.0, -87.0),
    // Vec2::new(299.0, -86.0),
    // Vec2::new(296.0, -85.0),
    // Vec2::new(294.0, -84.0),
    // Vec2::new(292.0, -83.0),
    // Vec2::new(289.0, -82.0),
    // Vec2::new(287.0, -81.0),
    Vec2::new(284.0, -80.0),
    // Vec2::new(282.0, -79.0),
    // Vec2::new(280.0, -78.0),
    // Vec2::new(277.0, -77.0),
    // Vec2::new(275.0, -76.0),
    // Vec2::new(272.0, -75.0),
    // Vec2::new(270.0, -74.0),
    // Vec2::new(268.0, -73.0),
    // Vec2::new(265.0, -72.0),
    // Vec2::new(263.0, -71.0),
    // Vec2::new(261.0, -70.0),
    // Vec2::new(259.0, -69.0),
    // Vec2::new(257.0, -68.0),
    // Vec2::new(254.0, -67.0),
    // Vec2::new(252.0, -66.0),
    // Vec2::new(249.0, -65.0),
    // Vec2::new(247.0, -64.0),
    // Vec2::new(245.0, -63.0),
    // Vec2::new(242.0, -62.0),
    // Vec2::new(240.0, -61.0),
    // Vec2::new(237.0, -60.0),
    // Vec2::new(235.0, -59.0),
    // Vec2::new(233.0, -58.0),
    // Vec2::new(231.0, -57.0),
    // Vec2::new(229.0, -56.0),
    // Vec2::new(227.0, -55.0),
    // Vec2::new(225.0, -54.0),
    Vec2::new(222.0, -53.0),
    // Vec2::new(220.0, -52.0),
    // Vec2::new(218.0, -51.0),
    // Vec2::new(215.0, -50.0),
    // Vec2::new(213.0, -49.0),
    // Vec2::new(211.0, -48.0),
    // Vec2::new(209.0, -47.0),
    // Vec2::new(207.0, -46.0),
    // Vec2::new(205.0, -45.0),
    // Vec2::new(203.0, -44.0),
    // Vec2::new(201.0, -43.0),
    // Vec2::new(199.0, -42.0),
    // Vec2::new(197.0, -41.0),
    // Vec2::new(195.0, -40.0),
    // Vec2::new(193.0, -39.0),
    // Vec2::new(191.0, -38.0),
    // Vec2::new(189.0, -37.0),
    // Vec2::new(187.0, -36.0),
    // Vec2::new(185.0, -35.0),
    // Vec2::new(183.0, -34.0),
    // Vec2::new(181.0, -33.0),
    // Vec2::new(179.0, -32.0),
    Vec2::new(177.0, -31.0),
    // Vec2::new(175.0, -30.0),
    // Vec2::new(173.0, -29.0),
    // Vec2::new(171.0, -28.0),
    // Vec2::new(169.0, -27.0),
    // Vec2::new(167.0, -26.0),
    // Vec2::new(165.0, -25.0),
    // Vec2::new(163.0, -24.0),
    // Vec2::new(161.0, -23.0),
    // Vec2::new(159.0, -22.0),
    // Vec2::new(158.0, -21.0),
    // Vec2::new(156.0, -20.0),
    // Vec2::new(154.0, -19.0),
    // Vec2::new(152.0, -18.0),
    // Vec2::new(150.0, -17.0),
    // Vec2::new(148.0, -16.0),
    // Vec2::new(147.0, -15.0),
    // Vec2::new(145.0, -14.0),
    // Vec2::new(143.0, -13.0),
    Vec2::new(141.0, -12.0),
    // Vec2::new(139.0, -11.0),
    // Vec2::new(137.0, -10.0),
    // Vec2::new(136.0, -9.0),
    // Vec2::new(134.0, -8.0),
    // Vec2::new(132.0, -7.0),
    // Vec2::new(130.0, -6.0),
    // Vec2::new(128.0, -5.0),
    // Vec2::new(127.0, -4.0),
    // Vec2::new(125.0, -3.0),
    // Vec2::new(124.0, -2.0),
    // Vec2::new(122.0, -1.0),
    // Vec2::new(121.0, 0.0),
    // Vec2::new(119.0, 1.0),
    // Vec2::new(117.0, 2.0),
    // Vec2::new(115.0, 3.0),
    // Vec2::new(113.0, 4.0),
    // Vec2::new(112.0, 5.0),
    // Vec2::new(110.0, 6.0),
    // Vec2::new(109.0, 7.0),
    // Vec2::new(107.0, 8.0),
    // Vec2::new(105.0, 9.0),
    // Vec2::new(103.0, 10.0),
    // Vec2::new(101.0, 11.0),
    // Vec2::new(100.0, 12.0),
    // Vec2::new(98.0, 13.0),
    // Vec2::new(97.0, 14.0),
    // Vec2::new(95.0, 15.0),
    Vec2::new(93.0, 16.0),
    // Vec2::new(91.0, 17.0),
    // Vec2::new(89.0, 18.0),
    Vec2::new(88.0, 19.0),
    // Vec2::new(86.0, 20.0),
    // Vec2::new(85.0, 21.0),
    // Vec2::new(83.0, 22.0),
    // Vec2::new(81.0, 23.0),
    // Vec2::new(79.0, 24.0),
    // Vec2::new(78.0, 25.0),
    // Vec2::new(76.0, 26.0),
    // Vec2::new(74.0, 27.0),
    // Vec2::new(73.0, 28.0),
    // Vec2::new(71.0, 29.0),
    // Vec2::new(69.0, 30.0),
    // Vec2::new(68.0, 31.0),
    // Vec2::new(66.0, 32.0),
    // Vec2::new(65.0, 33.0),
    // Vec2::new(64.0, 34.0),
    // Vec2::new(62.0, 35.0),
    // Vec2::new(61.0, 36.0),
    // Vec2::new(59.0, 37.0),
    // Vec2::new(58.0, 38.0),
    // Vec2::new(57.0, 39.0),
    Vec2::new(55.0, 40.0),
    // Vec2::new(53.0, 41.0),
    // Vec2::new(51.0, 42.0),
    // Vec2::new(50.0, 43.0),
    // Vec2::new(48.0, 44.0),
    // Vec2::new(47.0, 45.0),
    // Vec2::new(46.0, 46.0),
    // Vec2::new(44.0, 47.0),
    // Vec2::new(43.0, 48.0),
    // Vec2::new(41.0, 49.0),
    // Vec2::new(40.0, 50.0),
    // Vec2::new(39.0, 51.0),
    // Vec2::new(37.0, 52.0),
    // Vec2::new(35.0, 53.0),
    // Vec2::new(33.0, 54.0),
    // Vec2::new(32.0, 55.0),
    // Vec2::new(31.0, 56.0),
    // Vec2::new(29.0, 57.0),
    // Vec2::new(28.0, 58.0),
    // Vec2::new(27.0, 59.0),
    // Vec2::new(25.0, 60.0),
    Vec2::new(24.0, 61.0),
    // Vec2::new(23.0, 62.0),
    // Vec2::new(21.0, 63.0),
    // Vec2::new(20.0, 64.0),
    // Vec2::new(19.0, 65.0),
    // Vec2::new(17.0, 66.0),
    // Vec2::new(16.0, 67.0),
    // Vec2::new(15.0, 68.0),
    // Vec2::new(14.0, 69.0),
    // Vec2::new(12.0, 70.0),
    // Vec2::new(11.0, 71.0),
    // Vec2::new(10.0, 72.0),
    // Vec2::new(8.0, 73.0),
    // Vec2::new(6.0, 74.0),
    // Vec2::new(4.0, 75.0),
    // Vec2::new(3.0, 76.0),
    // Vec2::new(2.0, 77.0),
    Vec2::new(0.0, 78.0),
    // Vec2::new(-1.0, 79.0),
    // Vec2::new(-2.0, 80.0),
    // Vec2::new(-3.0, 81.0),
    // Vec2::new(-4.0, 82.0),
    // Vec2::new(-5.0, 83.0),
    // Vec2::new(-6.0, 84.0),
    // Vec2::new(-7.0, 85.0),
    // Vec2::new(-9.0, 86.0),
    // Vec2::new(-10.0, 87.0),
    // Vec2::new(-11.0, 88.0),
    // Vec2::new(-13.0, 89.0),
    // Vec2::new(-14.0, 90.0),
    // Vec2::new(-15.0, 91.0),
    // Vec2::new(-16.0, 92.0),
    // Vec2::new(-17.0, 93.0),
    // Vec2::new(-18.0, 94.0),
    // Vec2::new(-19.0, 95.0),
    // Vec2::new(-20.0, 96.0),
    // Vec2::new(-21.0, 97.0),
    // Vec2::new(-23.0, 98.0),
    // Vec2::new(-24.0, 99.0),
    // Vec2::new(-25.0, 100.0),
    // Vec2::new(-27.0, 101.0),
    // Vec2::new(-28.0, 102.0),
    // Vec2::new(-29.0, 103.0),
    // Vec2::new(-30.0, 104.0),
    // Vec2::new(-31.0, 105.0),
    // Vec2::new(-32.0, 106.0),
    // Vec2::new(-33.0, 107.0),
    // Vec2::new(-34.0, 108.0),
    // Vec2::new(-35.0, 109.0),
    // Vec2::new(-37.0, 110.0),
    // Vec2::new(-38.0, 111.0),
    // Vec2::new(-39.0, 112.0),
    // Vec2::new(-41.0, 113.0),
    // Vec2::new(-42.0, 114.0),
    // Vec2::new(-43.0, 115.0),
    // Vec2::new(-44.0, 116.0),
    // Vec2::new(-46.0, 117.0),
    Vec2::new(-47.0, 118.0),
    // Vec2::new(-48.0, 119.0),
    // Vec2::new(-49.0, 120.0),
    // Vec2::new(-51.0, 121.0),
    // Vec2::new(-52.0, 122.0),
    // Vec2::new(-53.0, 123.0),
    // Vec2::new(-54.0, 124.0),
    // Vec2::new(-55.0, 125.0),
    // Vec2::new(-56.0, 126.0),
    // Vec2::new(-57.0, 127.0),
    // Vec2::new(-58.0, 128.0),
    // Vec2::new(-60.0, 129.0),
    // Vec2::new(-61.0, 130.0),
    // Vec2::new(-62.0, 131.0),
    // Vec2::new(-64.0, 132.0),
    // Vec2::new(-65.0, 133.0),
    // Vec2::new(-66.0, 134.0),
    // Vec2::new(-67.0, 135.0),
    // Vec2::new(-69.0, 136.0),
    // Vec2::new(-70.0, 137.0),
    // Vec2::new(-71.0, 138.0),
    // Vec2::new(-72.0, 139.0),
    // Vec2::new(-74.0, 140.0),
    // Vec2::new(-75.0, 141.0),
    // Vec2::new(-76.0, 142.0),
    // Vec2::new(-77.0, 143.0),
    // Vec2::new(-78.0, 144.0),
    // Vec2::new(-79.0, 145.0),
    // Vec2::new(-80.0, 146.0),
    // Vec2::new(-81.0, 147.0),
    // Vec2::new(-82.0, 148.0),
    // Vec2::new(-83.0, 149.0),
    // Vec2::new(-84.0, 150.0),
    // Vec2::new(-85.0, 151.0),
    // Vec2::new(-87.0, 152.0),
    // Vec2::new(-89.0, 153.0),
    // Vec2::new(-90.0, 154.0),
    // Vec2::new(-91.0, 155.0),
    // Vec2::new(-92.0, 156.0),
    // Vec2::new(-93.0, 157.0),
    // Vec2::new(-94.0, 158.0),
    // Vec2::new(-95.0, 159.0),
    // Vec2::new(-96.0, 160.0),
    // Vec2::new(-97.0, 161.0),
    // Vec2::new(-98.0, 162.0),
    // Vec2::new(-99.0, 163.0),
    // Vec2::new(-100.0, 164.0),
    // Vec2::new(-101.0, 165.0),
    // Vec2::new(-102.0, 166.0),
    // Vec2::new(-103.0, 167.0),
    // Vec2::new(-104.0, 168.0),
    // Vec2::new(-105.0, 169.0),
    // Vec2::new(-106.0, 170.0),
    // Vec2::new(-107.0, 171.0),
    // Vec2::new(-108.0, 172.0),
    // Vec2::new(-109.0, 173.0),
    // Vec2::new(-110.0, 174.0),
    // Vec2::new(-111.0, 175.0),
    // Vec2::new(-112.0, 176.0),
    // Vec2::new(-113.0, 177.0),
    // Vec2::new(-114.0, 178.0),
    // Vec2::new(-115.0, 179.0),
    // Vec2::new(-116.0, 180.0),
    // Vec2::new(-117.0, 181.0),
    // Vec2::new(-118.0, 182.0),
    Vec2::new(-119.0, 183.0),
    // Vec2::new(-120.0, 184.0),
    // Vec2::new(-121.0, 185.0),
    // Vec2::new(-122.0, 186.0),
    // Vec2::new(-123.0, 187.0),
    // Vec2::new(-124.0, 188.0),
    // Vec2::new(-125.0, 189.0),
    // Vec2::new(-126.0, 190.0),
    // Vec2::new(-127.0, 191.0),
    // Vec2::new(-128.0, 192.0),
    // Vec2::new(-129.0, 193.0),
    // Vec2::new(-130.0, 194.0),
    // Vec2::new(-131.0, 195.0),
    // Vec2::new(-132.0, 196.0),
    // Vec2::new(-133.0, 197.0),
    // Vec2::new(-134.0, 198.0),
    // Vec2::new(-135.0, 199.0),
    // Vec2::new(-136.0, 200.0),
    // Vec2::new(-137.0, 201.0),
    // Vec2::new(-138.0, 202.0),
    // Vec2::new(-139.0, 203.0),
    // Vec2::new(-140.0, 204.0),
    // Vec2::new(-141.0, 205.0),
    // Vec2::new(-142.0, 206.0),
    // Vec2::new(-143.0, 207.0),
    // Vec2::new(-144.0, 208.0),
    // Vec2::new(-145.0, 209.0),
    // Vec2::new(-146.0, 210.0),
    // Vec2::new(-147.0, 211.0),
    // Vec2::new(-148.0, 212.0),
    // Vec2::new(-149.0, 213.0),
    // Vec2::new(-150.0, 214.0),
    // Vec2::new(-151.0, 215.0),
    // Vec2::new(-152.0, 216.0),
    // Vec2::new(-153.0, 217.0),
    // Vec2::new(-154.0, 218.0),
    // Vec2::new(-155.0, 219.0),
    // Vec2::new(-156.0, 220.0),
    // Vec2::new(-157.0, 221.0),
    // Vec2::new(-158.0, 222.0),
    Vec2::new(-159.0, 223.0),
    // Vec2::new(-160.0, 224.0),
    // Vec2::new(-161.0, 225.0),
    // Vec2::new(-162.0, 226.0),
    // Vec2::new(-163.0, 227.0),
    // Vec2::new(-164.0, 228.0),
    // Vec2::new(-165.0, 229.0),
    // Vec2::new(-166.0, 230.0),
    // Vec2::new(-167.0, 231.0),
    // Vec2::new(-168.0, 232.0),
    // Vec2::new(-169.0, 233.0),
    // Vec2::new(-170.0, 234.0),
    // Vec2::new(-171.0, 235.0),
    // Vec2::new(-172.0, 236.0),
    // Vec2::new(-173.0, 237.0),
    // Vec2::new(-174.0, 238.0),
    // Vec2::new(-175.0, 239.0),
    // Vec2::new(-176.0, 240.0),
    // Vec2::new(-177.0, 241.0),
    // Vec2::new(-178.0, 242.0),
    Vec2::new(-179.0, 243.0),
    // Vec2::new(-180.0, 244.0),
    // Vec2::new(-181.0, 245.0),
    // Vec2::new(-182.0, 246.0),
    // Vec2::new(-183.0, 247.0),
    // Vec2::new(-184.0, 248.0),
    // Vec2::new(-185.0, 249.0),
    // Vec2::new(-186.0, 250.0),
    // Vec2::new(-187.0, 251.0),
    Vec2::new(-188.0, 252.0),
    // Vec2::new(-189.0, 253.0),
    // Vec2::new(-190.0, 254.0),
    // Vec2::new(-191.0, 255.0),
    // Vec2::new(-192.0, 256.0),
    // Vec2::new(-193.0, 257.0),
    // Vec2::new(-194.0, 258.0),
    Vec2::new(-195.0, 259.0),
    // Vec2::new(-196.0, 260.0),
    // Vec2::new(-197.0, 261.0),
    // Vec2::new(-198.0, 262.0),
    // Vec2::new(-199.0, 263.0),
    // Vec2::new(-200.0, 264.0),
    // Vec2::new(-201.0, 265.0),
    // Vec2::new(-203.0, 266.0),
    // Vec2::new(-206.0, 267.0),
    Vec2::new(-209.0, 268.0),
    Vec2::new(-215.0, 269.0),
    Vec2::new(-727.0, 269.0),
];

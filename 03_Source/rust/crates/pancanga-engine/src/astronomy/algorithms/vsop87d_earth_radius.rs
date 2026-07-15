//! VSOP87D Earth radius vector coefficients.
//!
//! Generated from the official IMCCE file `VSOP87D.ear`.
//! Only variable 3 (radius vector, R) is included here.
//! The tuple fields are `(A, B, C)` for `A * cos(B + C * tau)`.
//! Coefficients are copied with the same decimal spelling used by IMCCE.

#![allow(clippy::approx_constant, clippy::excessive_precision)]

#[derive(Debug, Clone, Copy)]
pub(super) struct Vsop87Term {
    pub(super) amplitude: f64,
    pub(super) phase: f64,
    pub(super) frequency: f64,
}

pub(super) const R0: &[Vsop87Term] = &[
    Vsop87Term {
        amplitude: 1.00013988799,
        phase: 0.00000000000,
        frequency: 0.00000000000,
    },
    Vsop87Term {
        amplitude: 0.01670699626,
        phase: 3.09846350771,
        frequency: 6283.07584999140,
    },
    Vsop87Term {
        amplitude: 0.00013956023,
        phase: 3.05524609620,
        frequency: 12566.15169998280,
    },
    Vsop87Term {
        amplitude: 0.00003083720,
        phase: 5.19846674381,
        frequency: 77713.77146812050,
    },
    Vsop87Term {
        amplitude: 0.00001628461,
        phase: 1.17387749012,
        frequency: 5753.38488489680,
    },
    Vsop87Term {
        amplitude: 0.00001575568,
        phase: 2.84685245825,
        frequency: 7860.41939243920,
    },
    Vsop87Term {
        amplitude: 0.00000924799,
        phase: 5.45292234084,
        frequency: 11506.76976979360,
    },
    Vsop87Term {
        amplitude: 0.00000542444,
        phase: 4.56409149777,
        frequency: 3930.20969621960,
    },
    Vsop87Term {
        amplitude: 0.00000472110,
        phase: 3.66100022149,
        frequency: 5884.92684658320,
    },
    Vsop87Term {
        amplitude: 0.00000328780,
        phase: 5.89983646482,
        frequency: 5223.69391980220,
    },
    Vsop87Term {
        amplitude: 0.00000345983,
        phase: 0.96368617687,
        frequency: 5507.55323866740,
    },
    Vsop87Term {
        amplitude: 0.00000306784,
        phase: 0.29867139512,
        frequency: 5573.14280143310,
    },
    Vsop87Term {
        amplitude: 0.00000174844,
        phase: 3.01193636534,
        frequency: 18849.22754997420,
    },
    Vsop87Term {
        amplitude: 0.00000243189,
        phase: 4.27349536153,
        frequency: 11790.62908865880,
    },
    Vsop87Term {
        amplitude: 0.00000211829,
        phase: 5.84714540314,
        frequency: 1577.34354244780,
    },
    Vsop87Term {
        amplitude: 0.00000185752,
        phase: 5.02194447178,
        frequency: 10977.07880469900,
    },
    Vsop87Term {
        amplitude: 0.00000109835,
        phase: 5.05510636285,
        frequency: 5486.77784317500,
    },
    Vsop87Term {
        amplitude: 0.00000098316,
        phase: 0.88681311277,
        frequency: 6069.77675455340,
    },
    Vsop87Term {
        amplitude: 0.00000086499,
        phase: 5.68959778254,
        frequency: 15720.83878487840,
    },
    Vsop87Term {
        amplitude: 0.00000085825,
        phase: 1.27083733351,
        frequency: 161000.68573767410,
    },
    Vsop87Term {
        amplitude: 0.00000062916,
        phase: 0.92177108832,
        frequency: 529.69096509460,
    },
    Vsop87Term {
        amplitude: 0.00000057056,
        phase: 2.01374292014,
        frequency: 83996.84731811189,
    },
    Vsop87Term {
        amplitude: 0.00000064903,
        phase: 0.27250613787,
        frequency: 17260.15465469040,
    },
    Vsop87Term {
        amplitude: 0.00000049384,
        phase: 3.24501240359,
        frequency: 2544.31441988340,
    },
    Vsop87Term {
        amplitude: 0.00000055736,
        phase: 5.24159798933,
        frequency: 71430.69561812909,
    },
    Vsop87Term {
        amplitude: 0.00000042515,
        phase: 6.01110242003,
        frequency: 6275.96230299060,
    },
    Vsop87Term {
        amplitude: 0.00000046963,
        phase: 2.57805070386,
        frequency: 775.52261132400,
    },
    Vsop87Term {
        amplitude: 0.00000038968,
        phase: 5.36071738169,
        frequency: 4694.00295470760,
    },
    Vsop87Term {
        amplitude: 0.00000044661,
        phase: 5.53715807302,
        frequency: 9437.76293488700,
    },
    Vsop87Term {
        amplitude: 0.00000035660,
        phase: 1.67468058995,
        frequency: 12036.46073488820,
    },
    Vsop87Term {
        amplitude: 0.00000031921,
        phase: 0.18368229781,
        frequency: 5088.62883976680,
    },
    Vsop87Term {
        amplitude: 0.00000031846,
        phase: 1.77775642085,
        frequency: 398.14900340820,
    },
    Vsop87Term {
        amplitude: 0.00000033193,
        phase: 0.24370300098,
        frequency: 7084.89678111520,
    },
    Vsop87Term {
        amplitude: 0.00000038245,
        phase: 2.39255343974,
        frequency: 8827.39026987480,
    },
    Vsop87Term {
        amplitude: 0.00000028464,
        phase: 1.21344868176,
        frequency: 6286.59896834040,
    },
    Vsop87Term {
        amplitude: 0.00000037490,
        phase: 0.82952922332,
        frequency: 19651.04848109800,
    },
    Vsop87Term {
        amplitude: 0.00000036957,
        phase: 4.90107591914,
        frequency: 12139.55350910680,
    },
    Vsop87Term {
        amplitude: 0.00000034537,
        phase: 1.84270693282,
        frequency: 2942.46342329160,
    },
    Vsop87Term {
        amplitude: 0.00000026275,
        phase: 4.58896850401,
        frequency: 10447.38783960440,
    },
    Vsop87Term {
        amplitude: 0.00000024596,
        phase: 3.78660875483,
        frequency: 8429.24126646660,
    },
    Vsop87Term {
        amplitude: 0.00000023587,
        phase: 0.26866117066,
        frequency: 796.29800681640,
    },
    Vsop87Term {
        amplitude: 0.00000027793,
        phase: 1.89934330904,
        frequency: 6279.55273164240,
    },
    Vsop87Term {
        amplitude: 0.00000023927,
        phase: 4.99598548138,
        frequency: 5856.47765911540,
    },
    Vsop87Term {
        amplitude: 0.00000020349,
        phase: 4.65267995431,
        frequency: 2146.16541647520,
    },
    Vsop87Term {
        amplitude: 0.00000023287,
        phase: 2.80783650928,
        frequency: 14143.49524243060,
    },
    Vsop87Term {
        amplitude: 0.00000022103,
        phase: 1.95004702988,
        frequency: 3154.68708489560,
    },
    Vsop87Term {
        amplitude: 0.00000019506,
        phase: 5.38227371393,
        frequency: 2352.86615377180,
    },
    Vsop87Term {
        amplitude: 0.00000017958,
        phase: 0.19871379385,
        frequency: 6812.76681508600,
    },
    Vsop87Term {
        amplitude: 0.00000017174,
        phase: 4.43315560735,
        frequency: 10213.28554621100,
    },
    Vsop87Term {
        amplitude: 0.00000016190,
        phase: 5.23160507859,
        frequency: 17789.84561978500,
    },
    Vsop87Term {
        amplitude: 0.00000017314,
        phase: 6.15200787916,
        frequency: 16730.46368959580,
    },
    Vsop87Term {
        amplitude: 0.00000013814,
        phase: 5.18962074032,
        frequency: 8031.09226305840,
    },
    Vsop87Term {
        amplitude: 0.00000018833,
        phase: 0.67306674027,
        frequency: 149854.40013480789,
    },
    Vsop87Term {
        amplitude: 0.00000018331,
        phase: 2.25348733734,
        frequency: 23581.25817731760,
    },
    Vsop87Term {
        amplitude: 0.00000013641,
        phase: 3.68516118804,
        frequency: 4705.73230754360,
    },
    Vsop87Term {
        amplitude: 0.00000013139,
        phase: 0.65289581324,
        frequency: 13367.97263110660,
    },
    Vsop87Term {
        amplitude: 0.00000010414,
        phase: 4.33285688538,
        frequency: 11769.85369316640,
    },
    Vsop87Term {
        amplitude: 0.00000009978,
        phase: 4.20126336355,
        frequency: 6309.37416979120,
    },
    Vsop87Term {
        amplitude: 0.00000010169,
        phase: 1.59390681369,
        frequency: 4690.47983635860,
    },
    Vsop87Term {
        amplitude: 0.00000007564,
        phase: 2.62560597390,
        frequency: 6256.77753019160,
    },
    Vsop87Term {
        amplitude: 0.00000009661,
        phase: 3.67586791220,
        frequency: 27511.46787353720,
    },
    Vsop87Term {
        amplitude: 0.00000006743,
        phase: 0.56270332741,
        frequency: 3340.61242669980,
    },
    Vsop87Term {
        amplitude: 0.00000008743,
        phase: 6.06359123461,
        frequency: 1748.01641306700,
    },
    Vsop87Term {
        amplitude: 0.00000007786,
        phase: 3.67371235637,
        frequency: 12168.00269657460,
    },
    Vsop87Term {
        amplitude: 0.00000006633,
        phase: 5.66149277792,
        frequency: 11371.70468975820,
    },
    Vsop87Term {
        amplitude: 0.00000007712,
        phase: 0.31242577789,
        frequency: 7632.94325965020,
    },
    Vsop87Term {
        amplitude: 0.00000006592,
        phase: 3.13576266188,
        frequency: 801.82093112380,
    },
    Vsop87Term {
        amplitude: 0.00000007460,
        phase: 5.64757188143,
        frequency: 11926.25441366880,
    },
    Vsop87Term {
        amplitude: 0.00000006933,
        phase: 2.92384586400,
        frequency: 6681.22485339960,
    },
    Vsop87Term {
        amplitude: 0.00000006802,
        phase: 1.42329806420,
        frequency: 23013.53953958720,
    },
    Vsop87Term {
        amplitude: 0.00000006115,
        phase: 5.13393615454,
        frequency: 1194.44701022460,
    },
    Vsop87Term {
        amplitude: 0.00000006477,
        phase: 2.64986648492,
        frequency: 19804.82729158280,
    },
    Vsop87Term {
        amplitude: 0.00000005233,
        phase: 4.62434053374,
        frequency: 6438.49624942560,
    },
    Vsop87Term {
        amplitude: 0.00000006147,
        phase: 3.02863936662,
        frequency: 233141.31440436149,
    },
    Vsop87Term {
        amplitude: 0.00000004608,
        phase: 1.72194702724,
        frequency: 7234.79425624200,
    },
    Vsop87Term {
        amplitude: 0.00000004221,
        phase: 1.55697533729,
        frequency: 7238.67559160000,
    },
    Vsop87Term {
        amplitude: 0.00000005314,
        phase: 2.40716580847,
        frequency: 11499.65622279280,
    },
    Vsop87Term {
        amplitude: 0.00000005128,
        phase: 5.32398965690,
        frequency: 11513.88331679440,
    },
    Vsop87Term {
        amplitude: 0.00000004770,
        phase: 0.25554312006,
        frequency: 11856.21865142450,
    },
    Vsop87Term {
        amplitude: 0.00000005519,
        phase: 2.09089154502,
        frequency: 17298.18232732620,
    },
    Vsop87Term {
        amplitude: 0.00000005625,
        phase: 4.34052903053,
        frequency: 90955.55169449610,
    },
    Vsop87Term {
        amplitude: 0.00000004578,
        phase: 4.46569641570,
        frequency: 5746.27133789600,
    },
    Vsop87Term {
        amplitude: 0.00000003788,
        phase: 4.90729383510,
        frequency: 4164.31198961300,
    },
    Vsop87Term {
        amplitude: 0.00000005337,
        phase: 5.09957905104,
        frequency: 31441.67756975680,
    },
    Vsop87Term {
        amplitude: 0.00000003967,
        phase: 1.20054555174,
        frequency: 1349.86740965880,
    },
    Vsop87Term {
        amplitude: 0.00000004008,
        phase: 3.03007204392,
        frequency: 1059.38193018920,
    },
    Vsop87Term {
        amplitude: 0.00000003476,
        phase: 0.76080277030,
        frequency: 10973.55568635000,
    },
    Vsop87Term {
        amplitude: 0.00000004232,
        phase: 1.05485713117,
        frequency: 5760.49843189760,
    },
    Vsop87Term {
        amplitude: 0.00000004582,
        phase: 3.76570026763,
        frequency: 6386.16862421000,
    },
    Vsop87Term {
        amplitude: 0.00000003335,
        phase: 3.13829943354,
        frequency: 6836.64525283380,
    },
    Vsop87Term {
        amplitude: 0.00000003418,
        phase: 3.00072390334,
        frequency: 4292.33083295040,
    },
    Vsop87Term {
        amplitude: 0.00000003598,
        phase: 5.70718084323,
        frequency: 5643.17856367740,
    },
    Vsop87Term {
        amplitude: 0.00000003237,
        phase: 4.16448773994,
        frequency: 9917.69687450980,
    },
    Vsop87Term {
        amplitude: 0.00000004154,
        phase: 2.59941292162,
        frequency: 7058.59846131540,
    },
    Vsop87Term {
        amplitude: 0.00000003362,
        phase: 4.54577697964,
        frequency: 4732.03062734340,
    },
    Vsop87Term {
        amplitude: 0.00000002978,
        phase: 1.30561268820,
        frequency: 6283.14316029419,
    },
    Vsop87Term {
        amplitude: 0.00000002765,
        phase: 0.51311975679,
        frequency: 26.29831979980,
    },
    Vsop87Term {
        amplitude: 0.00000002802,
        phase: 5.66263240521,
        frequency: 8635.94200376320,
    },
    Vsop87Term {
        amplitude: 0.00000002927,
        phase: 5.73787481548,
        frequency: 16200.77272450120,
    },
    Vsop87Term {
        amplitude: 0.00000003164,
        phase: 1.69140262657,
        frequency: 11015.10647733480,
    },
    Vsop87Term {
        amplitude: 0.00000002598,
        phase: 2.96244118586,
        frequency: 25132.30339996560,
    },
    Vsop87Term {
        amplitude: 0.00000003519,
        phase: 3.62639325753,
        frequency: 244287.60000722769,
    },
    Vsop87Term {
        amplitude: 0.00000002676,
        phase: 4.20725700850,
        frequency: 18073.70493865020,
    },
    Vsop87Term {
        amplitude: 0.00000002978,
        phase: 1.74971565805,
        frequency: 6283.00853968860,
    },
    Vsop87Term {
        amplitude: 0.00000002287,
        phase: 1.06975704977,
        frequency: 14314.16811304980,
    },
    Vsop87Term {
        amplitude: 0.00000002863,
        phase: 5.92838131397,
        frequency: 14712.31711645800,
    },
    Vsop87Term {
        amplitude: 0.00000003071,
        phase: 0.23793217002,
        frequency: 35371.88726597640,
    },
    Vsop87Term {
        amplitude: 0.00000002656,
        phase: 0.89959301780,
        frequency: 12352.85260454480,
    },
    Vsop87Term {
        amplitude: 0.00000002415,
        phase: 2.79975176257,
        frequency: 709.93304855830,
    },
    Vsop87Term {
        amplitude: 0.00000002814,
        phase: 3.51488206882,
        frequency: 21228.39202354580,
    },
    Vsop87Term {
        amplitude: 0.00000001977,
        phase: 2.61358297550,
        frequency: 951.71840625060,
    },
    Vsop87Term {
        amplitude: 0.00000002548,
        phase: 2.47684686575,
        frequency: 6208.29425142410,
    },
    Vsop87Term {
        amplitude: 0.00000001999,
        phase: 0.56090388160,
        frequency: 7079.37385680780,
    },
    Vsop87Term {
        amplitude: 0.00000002305,
        phase: 1.05376461628,
        frequency: 22483.84857449259,
    },
    Vsop87Term {
        amplitude: 0.00000001855,
        phase: 2.86090681163,
        frequency: 5216.58037280140,
    },
    Vsop87Term {
        amplitude: 0.00000002157,
        phase: 1.31396741861,
        frequency: 154717.60988768269,
    },
    Vsop87Term {
        amplitude: 0.00000001970,
        phase: 4.36929875289,
        frequency: 167283.76158766549,
    },
    Vsop87Term {
        amplitude: 0.00000001635,
        phase: 5.85571606764,
        frequency: 10984.19235169980,
    },
    Vsop87Term {
        amplitude: 0.00000001754,
        phase: 2.14452408833,
        frequency: 6290.18939699220,
    },
    Vsop87Term {
        amplitude: 0.00000002154,
        phase: 6.03828341543,
        frequency: 10873.98603048040,
    },
    Vsop87Term {
        amplitude: 0.00000001714,
        phase: 3.70157691113,
        frequency: 1592.59601363280,
    },
    Vsop87Term {
        amplitude: 0.00000001541,
        phase: 6.21598380732,
        frequency: 23543.23050468179,
    },
    Vsop87Term {
        amplitude: 0.00000001611,
        phase: 1.99824499377,
        frequency: 10969.96525769820,
    },
    Vsop87Term {
        amplitude: 0.00000001712,
        phase: 1.34295663542,
        frequency: 3128.38876509580,
    },
    Vsop87Term {
        amplitude: 0.00000001642,
        phase: 5.55026665339,
        frequency: 6496.37494542940,
    },
    Vsop87Term {
        amplitude: 0.00000001502,
        phase: 5.43948825854,
        frequency: 155.42039943420,
    },
    Vsop87Term {
        amplitude: 0.00000001827,
        phase: 5.91227480261,
        frequency: 3738.76143010800,
    },
    Vsop87Term {
        amplitude: 0.00000001726,
        phase: 2.16764983583,
        frequency: 10575.40668294180,
    },
    Vsop87Term {
        amplitude: 0.00000001532,
        phase: 5.35683107070,
        frequency: 13521.75144159140,
    },
    Vsop87Term {
        amplitude: 0.00000001829,
        phase: 1.66006148731,
        frequency: 39302.09696219600,
    },
    Vsop87Term {
        amplitude: 0.00000001605,
        phase: 1.90928637633,
        frequency: 6133.51265285680,
    },
    Vsop87Term {
        amplitude: 0.00000001282,
        phase: 2.46014880418,
        frequency: 13916.01910964160,
    },
    Vsop87Term {
        amplitude: 0.00000001211,
        phase: 4.41360631550,
        frequency: 3894.18182954220,
    },
    Vsop87Term {
        amplitude: 0.00000001394,
        phase: 1.77801929354,
        frequency: 9225.53927328300,
    },
    Vsop87Term {
        amplitude: 0.00000001571,
        phase: 4.95512957592,
        frequency: 25158.60171976540,
    },
    Vsop87Term {
        amplitude: 0.00000001205,
        phase: 1.19212540615,
        frequency: 3.52311834900,
    },
    Vsop87Term {
        amplitude: 0.00000001132,
        phase: 2.69830084955,
        frequency: 6040.34724601740,
    },
    Vsop87Term {
        amplitude: 0.00000001504,
        phase: 5.77002730341,
        frequency: 18209.33026366019,
    },
    Vsop87Term {
        amplitude: 0.00000001393,
        phase: 1.62621805428,
        frequency: 5120.60114558360,
    },
    Vsop87Term {
        amplitude: 0.00000001077,
        phase: 2.93931554233,
        frequency: 17256.63153634140,
    },
    Vsop87Term {
        amplitude: 0.00000001232,
        phase: 0.71655165307,
        frequency: 143571.32428481648,
    },
    Vsop87Term {
        amplitude: 0.00000001087,
        phase: 0.99769687939,
        frequency: 955.59974160860,
    },
    Vsop87Term {
        amplitude: 0.00000001068,
        phase: 5.28472576231,
        frequency: 65147.61976813770,
    },
    Vsop87Term {
        amplitude: 0.00000000980,
        phase: 5.10949204607,
        frequency: 6172.86952877200,
    },
    Vsop87Term {
        amplitude: 0.00000001169,
        phase: 3.11664290862,
        frequency: 14945.31617355440,
    },
    Vsop87Term {
        amplitude: 0.00000001202,
        phase: 4.02992510402,
        frequency: 553.56940284240,
    },
    Vsop87Term {
        amplitude: 0.00000000979,
        phase: 2.00000879212,
        frequency: 15110.46611986620,
    },
    Vsop87Term {
        amplitude: 0.00000000962,
        phase: 4.02380771400,
        frequency: 6282.09552892320,
    },
    Vsop87Term {
        amplitude: 0.00000000999,
        phase: 3.62643002790,
        frequency: 6262.30045449900,
    },
    Vsop87Term {
        amplitude: 0.00000001030,
        phase: 5.84989900289,
        frequency: 213.29909543800,
    },
    Vsop87Term {
        amplitude: 0.00000001014,
        phase: 2.84221578218,
        frequency: 8662.24032356300,
    },
    Vsop87Term {
        amplitude: 0.00000001185,
        phase: 1.51330541132,
        frequency: 17654.78053974960,
    },
    Vsop87Term {
        amplitude: 0.00000000967,
        phase: 2.67081017562,
        frequency: 5650.29211067820,
    },
    Vsop87Term {
        amplitude: 0.00000001222,
        phase: 2.65423784904,
        frequency: 88860.05707098669,
    },
    Vsop87Term {
        amplitude: 0.00000000981,
        phase: 2.36370360283,
        frequency: 6206.80977871580,
    },
    Vsop87Term {
        amplitude: 0.00000001033,
        phase: 0.13874927606,
        frequency: 11712.95531823080,
    },
    Vsop87Term {
        amplitude: 0.00000001103,
        phase: 3.08477302937,
        frequency: 43232.30665841560,
    },
    Vsop87Term {
        amplitude: 0.00000000781,
        phase: 2.53372735932,
        frequency: 16496.36139620240,
    },
    Vsop87Term {
        amplitude: 0.00000001019,
        phase: 3.04569392376,
        frequency: 6037.24420376200,
    },
    Vsop87Term {
        amplitude: 0.00000000795,
        phase: 5.80662989111,
        frequency: 5230.80746680300,
    },
    Vsop87Term {
        amplitude: 0.00000000813,
        phase: 3.57710279439,
        frequency: 10177.25767953360,
    },
    Vsop87Term {
        amplitude: 0.00000000962,
        phase: 5.31470594766,
        frequency: 6284.05617105960,
    },
    Vsop87Term {
        amplitude: 0.00000000721,
        phase: 5.96264301567,
        frequency: 12559.03815298200,
    },
    Vsop87Term {
        amplitude: 0.00000000966,
        phase: 2.74714939953,
        frequency: 6244.94281435360,
    },
    Vsop87Term {
        amplitude: 0.00000000921,
        phase: 0.10155275926,
        frequency: 29088.81141598500,
    },
    Vsop87Term {
        amplitude: 0.00000000692,
        phase: 3.89764447548,
        frequency: 1589.07289528380,
    },
    Vsop87Term {
        amplitude: 0.00000000719,
        phase: 5.91791450402,
        frequency: 4136.91043351620,
    },
    Vsop87Term {
        amplitude: 0.00000000772,
        phase: 4.05505682353,
        frequency: 6127.65545055720,
    },
    Vsop87Term {
        amplitude: 0.00000000712,
        phase: 5.49291532439,
        frequency: 22003.91463486980,
    },
    Vsop87Term {
        amplitude: 0.00000000672,
        phase: 1.60700490811,
        frequency: 11087.28512591840,
    },
    Vsop87Term {
        amplitude: 0.00000000690,
        phase: 4.50539825563,
        frequency: 426.59819087600,
    },
    Vsop87Term {
        amplitude: 0.00000000854,
        phase: 3.26104981596,
        frequency: 20426.57109242200,
    },
    Vsop87Term {
        amplitude: 0.00000000656,
        phase: 4.32410182940,
        frequency: 16858.48253293320,
    },
    Vsop87Term {
        amplitude: 0.00000000840,
        phase: 2.59572585222,
        frequency: 28766.92442448400,
    },
    Vsop87Term {
        amplitude: 0.00000000692,
        phase: 0.61650089011,
        frequency: 11403.67699557500,
    },
    Vsop87Term {
        amplitude: 0.00000000700,
        phase: 3.40901167143,
        frequency: 7.11354700080,
    },
    Vsop87Term {
        amplitude: 0.00000000726,
        phase: 0.04243053594,
        frequency: 5481.25491886760,
    },
    Vsop87Term {
        amplitude: 0.00000000557,
        phase: 4.78317696534,
        frequency: 20199.09495963300,
    },
    Vsop87Term {
        amplitude: 0.00000000649,
        phase: 1.04027912958,
        frequency: 6062.66320755260,
    },
    Vsop87Term {
        amplitude: 0.00000000633,
        phase: 5.70229959167,
        frequency: 45892.73043315699,
    },
    Vsop87Term {
        amplitude: 0.00000000592,
        phase: 6.11836729658,
        frequency: 9623.68827669120,
    },
    Vsop87Term {
        amplitude: 0.00000000523,
        phase: 3.62840021266,
        frequency: 5333.90024102160,
    },
    Vsop87Term {
        amplitude: 0.00000000604,
        phase: 5.57734696185,
        frequency: 10344.29506538580,
    },
    Vsop87Term {
        amplitude: 0.00000000496,
        phase: 2.21023499449,
        frequency: 1990.74501704100,
    },
    Vsop87Term {
        amplitude: 0.00000000691,
        phase: 1.96071732602,
        frequency: 12416.58850284820,
    },
    Vsop87Term {
        amplitude: 0.00000000640,
        phase: 1.59074172032,
        frequency: 18319.53658487960,
    },
    Vsop87Term {
        amplitude: 0.00000000625,
        phase: 3.82362791378,
        frequency: 13517.87010623340,
    },
    Vsop87Term {
        amplitude: 0.00000000663,
        phase: 5.08444996779,
        frequency: 283.85931886520,
    },
    Vsop87Term {
        amplitude: 0.00000000475,
        phase: 1.17025894287,
        frequency: 12569.67481833180,
    },
    Vsop87Term {
        amplitude: 0.00000000664,
        phase: 4.50029469969,
        frequency: 47162.51635463520,
    },
    Vsop87Term {
        amplitude: 0.00000000569,
        phase: 0.16310365162,
        frequency: 17267.26820169119,
    },
    Vsop87Term {
        amplitude: 0.00000000568,
        phase: 3.86100969474,
        frequency: 6076.89030155420,
    },
    Vsop87Term {
        amplitude: 0.00000000539,
        phase: 4.83282276086,
        frequency: 18422.62935909819,
    },
    Vsop87Term {
        amplitude: 0.00000000466,
        phase: 0.75872342878,
        frequency: 7342.45778018060,
    },
    Vsop87Term {
        amplitude: 0.00000000541,
        phase: 3.07212190507,
        frequency: 226858.23855437008,
    },
    Vsop87Term {
        amplitude: 0.00000000458,
        phase: 0.26774483096,
        frequency: 4590.91018048900,
    },
    Vsop87Term {
        amplitude: 0.00000000610,
        phase: 1.53597051291,
        frequency: 33019.02111220460,
    },
    Vsop87Term {
        amplitude: 0.00000000617,
        phase: 2.62356328726,
        frequency: 11190.37790013700,
    },
    Vsop87Term {
        amplitude: 0.00000000548,
        phase: 4.55798855791,
        frequency: 18875.52586977400,
    },
    Vsop87Term {
        amplitude: 0.00000000633,
        phase: 4.60110281228,
        frequency: 66567.48586525429,
    },
    Vsop87Term {
        amplitude: 0.00000000596,
        phase: 5.78202396722,
        frequency: 632.78373931320,
    },
    Vsop87Term {
        amplitude: 0.00000000533,
        phase: 5.01786882904,
        frequency: 12132.43996210600,
    },
    Vsop87Term {
        amplitude: 0.00000000603,
        phase: 5.38458554802,
        frequency: 316428.22867391503,
    },
    Vsop87Term {
        amplitude: 0.00000000469,
        phase: 0.59168241917,
        frequency: 21954.15760939799,
    },
    Vsop87Term {
        amplitude: 0.00000000548,
        phase: 3.50613163558,
        frequency: 17253.04110768959,
    },
    Vsop87Term {
        amplitude: 0.00000000502,
        phase: 0.98804327589,
        frequency: 11609.86254401220,
    },
    Vsop87Term {
        amplitude: 0.00000000568,
        phase: 1.98497313089,
        frequency: 7668.63742494250,
    },
    Vsop87Term {
        amplitude: 0.00000000482,
        phase: 1.62141803864,
        frequency: 12146.66705610760,
    },
    Vsop87Term {
        amplitude: 0.00000000391,
        phase: 3.68718382989,
        frequency: 18052.92954315780,
    },
    Vsop87Term {
        amplitude: 0.00000000457,
        phase: 3.77205737340,
        frequency: 156137.47598479928,
    },
    Vsop87Term {
        amplitude: 0.00000000401,
        phase: 5.28260651958,
        frequency: 15671.08175940660,
    },
    Vsop87Term {
        amplitude: 0.00000000469,
        phase: 1.80963184268,
        frequency: 12562.62858163380,
    },
    Vsop87Term {
        amplitude: 0.00000000508,
        phase: 3.36399024699,
        frequency: 20597.24396304120,
    },
    Vsop87Term {
        amplitude: 0.00000000450,
        phase: 5.66054299250,
        frequency: 10454.50138660520,
    },
    Vsop87Term {
        amplitude: 0.00000000375,
        phase: 4.98534633105,
        frequency: 9779.10867612540,
    },
    Vsop87Term {
        amplitude: 0.00000000523,
        phase: 0.97215560834,
        frequency: 155427.54293624099,
    },
    Vsop87Term {
        amplitude: 0.00000000403,
        phase: 5.13939866506,
        frequency: 1551.04522264800,
    },
    Vsop87Term {
        amplitude: 0.00000000372,
        phase: 3.69883738807,
        frequency: 9388.00590941520,
    },
    Vsop87Term {
        amplitude: 0.00000000367,
        phase: 4.43875659716,
        frequency: 4535.05943692440,
    },
    Vsop87Term {
        amplitude: 0.00000000406,
        phase: 4.20863156600,
        frequency: 12592.45001978260,
    },
    Vsop87Term {
        amplitude: 0.00000000360,
        phase: 2.53924644657,
        frequency: 242.72860397400,
    },
    Vsop87Term {
        amplitude: 0.00000000471,
        phase: 4.61907324819,
        frequency: 5436.99301524020,
    },
    Vsop87Term {
        amplitude: 0.00000000441,
        phase: 5.83872966262,
        frequency: 3496.03282613400,
    },
    Vsop87Term {
        amplitude: 0.00000000385,
        phase: 4.94496680973,
        frequency: 24356.78078864160,
    },
    Vsop87Term {
        amplitude: 0.00000000349,
        phase: 6.15018231784,
        frequency: 19800.94595622480,
    },
    Vsop87Term {
        amplitude: 0.00000000355,
        phase: 0.21895678106,
        frequency: 5429.87946823940,
    },
    Vsop87Term {
        amplitude: 0.00000000344,
        phase: 5.62993724928,
        frequency: 2379.16447357160,
    },
    Vsop87Term {
        amplitude: 0.00000000380,
        phase: 2.72105213143,
        frequency: 11933.36796066960,
    },
    Vsop87Term {
        amplitude: 0.00000000432,
        phase: 0.24221790536,
        frequency: 17996.03116822220,
    },
    Vsop87Term {
        amplitude: 0.00000000378,
        phase: 5.22517556974,
        frequency: 7477.52286021600,
    },
    Vsop87Term {
        amplitude: 0.00000000337,
        phase: 5.10888041439,
        frequency: 5849.36411211460,
    },
    Vsop87Term {
        amplitude: 0.00000000315,
        phase: 0.57827745123,
        frequency: 10557.59416082380,
    },
    Vsop87Term {
        amplitude: 0.00000000318,
        phase: 4.49953141399,
        frequency: 3634.62102451840,
    },
    Vsop87Term {
        amplitude: 0.00000000323,
        phase: 1.54274281393,
        frequency: 10440.27429260360,
    },
    Vsop87Term {
        amplitude: 0.00000000309,
        phase: 5.76839284397,
        frequency: 20.77539549240,
    },
    Vsop87Term {
        amplitude: 0.00000000301,
        phase: 2.34727604008,
        frequency: 4686.88940770680,
    },
    Vsop87Term {
        amplitude: 0.00000000414,
        phase: 5.93237602310,
        frequency: 51092.72605085480,
    },
    Vsop87Term {
        amplitude: 0.00000000361,
        phase: 2.16398609550,
        frequency: 28237.23345938940,
    },
    Vsop87Term {
        amplitude: 0.00000000288,
        phase: 0.18376252189,
        frequency: 13095.84266507740,
    },
    Vsop87Term {
        amplitude: 0.00000000277,
        phase: 5.12952205045,
        frequency: 13119.72110282519,
    },
    Vsop87Term {
        amplitude: 0.00000000327,
        phase: 6.19222146204,
        frequency: 6268.84875598980,
    },
    Vsop87Term {
        amplitude: 0.00000000273,
        phase: 0.30522428863,
        frequency: 23141.55838292460,
    },
    Vsop87Term {
        amplitude: 0.00000000267,
        phase: 5.76152585786,
        frequency: 5966.68398033480,
    },
    Vsop87Term {
        amplitude: 0.00000000308,
        phase: 5.99280509979,
        frequency: 22805.73556599360,
    },
    Vsop87Term {
        amplitude: 0.00000000345,
        phase: 2.92489919444,
        frequency: 36949.23080842420,
    },
    Vsop87Term {
        amplitude: 0.00000000253,
        phase: 5.20995219509,
        frequency: 24072.92146977640,
    },
    Vsop87Term {
        amplitude: 0.00000000342,
        phase: 5.72702586209,
        frequency: 16460.33352952499,
    },
    Vsop87Term {
        amplitude: 0.00000000261,
        phase: 2.00304796059,
        frequency: 6148.01076995600,
    },
    Vsop87Term {
        amplitude: 0.00000000238,
        phase: 5.08264392839,
        frequency: 6915.85958930460,
    },
    Vsop87Term {
        amplitude: 0.00000000249,
        phase: 2.94762789744,
        frequency: 135.06508003540,
    },
    Vsop87Term {
        amplitude: 0.00000000306,
        phase: 3.89764686987,
        frequency: 10988.80815753500,
    },
    Vsop87Term {
        amplitude: 0.00000000305,
        phase: 0.05827812117,
        frequency: 4701.11650170840,
    },
    Vsop87Term {
        amplitude: 0.00000000319,
        phase: 2.95712862064,
        frequency: 163096.18036118349,
    },
    Vsop87Term {
        amplitude: 0.00000000209,
        phase: 4.43768461442,
        frequency: 6546.15977336420,
    },
    Vsop87Term {
        amplitude: 0.00000000270,
        phase: 2.06643178717,
        frequency: 4804.20927592700,
    },
    Vsop87Term {
        amplitude: 0.00000000217,
        phase: 0.73691592312,
        frequency: 6303.85124548380,
    },
    Vsop87Term {
        amplitude: 0.00000000206,
        phase: 0.32075959415,
        frequency: 25934.12433108940,
    },
    Vsop87Term {
        amplitude: 0.00000000218,
        phase: 0.18428135264,
        frequency: 28286.99048486120,
    },
    Vsop87Term {
        amplitude: 0.00000000205,
        phase: 5.21312087405,
        frequency: 20995.39296644940,
    },
    Vsop87Term {
        amplitude: 0.00000000199,
        phase: 0.44384292491,
        frequency: 16737.57723659660,
    },
    Vsop87Term {
        amplitude: 0.00000000230,
        phase: 6.06567392849,
        frequency: 6287.00800325450,
    },
    Vsop87Term {
        amplitude: 0.00000000219,
        phase: 1.29194216300,
        frequency: 5326.78669402080,
    },
    Vsop87Term {
        amplitude: 0.00000000201,
        phase: 1.74700937253,
        frequency: 22743.40937951640,
    },
    Vsop87Term {
        amplitude: 0.00000000207,
        phase: 4.45440927276,
        frequency: 6279.48542133960,
    },
    Vsop87Term {
        amplitude: 0.00000000269,
        phase: 6.05640445030,
        frequency: 64471.99124174489,
    },
    Vsop87Term {
        amplitude: 0.00000000190,
        phase: 0.99256176518,
        frequency: 29296.61538957860,
    },
    Vsop87Term {
        amplitude: 0.00000000238,
        phase: 5.42471431221,
        frequency: 39609.65458316560,
    },
    Vsop87Term {
        amplitude: 0.00000000262,
        phase: 5.26961924198,
        frequency: 522.57741809380,
    },
    Vsop87Term {
        amplitude: 0.00000000210,
        phase: 4.68618183158,
        frequency: 6254.62666252360,
    },
    Vsop87Term {
        amplitude: 0.00000000197,
        phase: 2.80624554080,
        frequency: 4933.20844033260,
    },
    Vsop87Term {
        amplitude: 0.00000000252,
        phase: 4.36220154608,
        frequency: 40879.44050464380,
    },
    Vsop87Term {
        amplitude: 0.00000000261,
        phase: 1.07241516738,
        frequency: 55022.93574707440,
    },
    Vsop87Term {
        amplitude: 0.00000000189,
        phase: 3.82966734476,
        frequency: 419.48464387520,
    },
    Vsop87Term {
        amplitude: 0.00000000185,
        phase: 4.14324541379,
        frequency: 5642.19824260920,
    },
    Vsop87Term {
        amplitude: 0.00000000247,
        phase: 3.44855612987,
        frequency: 6702.56049386660,
    },
    Vsop87Term {
        amplitude: 0.00000000205,
        phase: 4.04424043223,
        frequency: 536.80451209540,
    },
    Vsop87Term {
        amplitude: 0.00000000191,
        phase: 3.14082686083,
        frequency: 16723.35014259500,
    },
    Vsop87Term {
        amplitude: 0.00000000222,
        phase: 5.16263907319,
        frequency: 23539.70738633280,
    },
    Vsop87Term {
        amplitude: 0.00000000180,
        phase: 4.56214752149,
        frequency: 6489.26139842860,
    },
    Vsop87Term {
        amplitude: 0.00000000219,
        phase: 0.80382553358,
        frequency: 16627.37091537720,
    },
    Vsop87Term {
        amplitude: 0.00000000227,
        phase: 0.60156339452,
        frequency: 5905.70224207560,
    },
    Vsop87Term {
        amplitude: 0.00000000168,
        phase: 0.88753528161,
        frequency: 16062.18452611680,
    },
    Vsop87Term {
        amplitude: 0.00000000158,
        phase: 0.92127725775,
        frequency: 23937.85638974100,
    },
    Vsop87Term {
        amplitude: 0.00000000157,
        phase: 4.69607868164,
        frequency: 6805.65326808520,
    },
    Vsop87Term {
        amplitude: 0.00000000207,
        phase: 4.88410451334,
        frequency: 6286.66627864320,
    },
    Vsop87Term {
        amplitude: 0.00000000160,
        phase: 4.95943826846,
        frequency: 10021.83728009940,
    },
    Vsop87Term {
        amplitude: 0.00000000166,
        phase: 0.97126433565,
        frequency: 3097.88382272579,
    },
    Vsop87Term {
        amplitude: 0.00000000209,
        phase: 5.75663411805,
        frequency: 3646.35037735440,
    },
    Vsop87Term {
        amplitude: 0.00000000175,
        phase: 6.12762824412,
        frequency: 239424.39025435288,
    },
    Vsop87Term {
        amplitude: 0.00000000173,
        phase: 3.13887234973,
        frequency: 6179.98307577280,
    },
    Vsop87Term {
        amplitude: 0.00000000157,
        phase: 3.62822058179,
        frequency: 18451.07854656599,
    },
    Vsop87Term {
        amplitude: 0.00000000157,
        phase: 4.67695912235,
        frequency: 6709.67404086740,
    },
    Vsop87Term {
        amplitude: 0.00000000146,
        phase: 3.09506069735,
        frequency: 4907.30205014560,
    },
    Vsop87Term {
        amplitude: 0.00000000165,
        phase: 2.27139128760,
        frequency: 10660.68693504240,
    },
    Vsop87Term {
        amplitude: 0.00000000201,
        phase: 1.67701267433,
        frequency: 2107.03450754240,
    },
    Vsop87Term {
        amplitude: 0.00000000144,
        phase: 3.96947747592,
        frequency: 6019.99192661860,
    },
    Vsop87Term {
        amplitude: 0.00000000171,
        phase: 5.91302216729,
        frequency: 6058.73105428950,
    },
    Vsop87Term {
        amplitude: 0.00000000144,
        phase: 2.13155655120,
        frequency: 26084.02180621620,
    },
    Vsop87Term {
        amplitude: 0.00000000151,
        phase: 0.67417383554,
        frequency: 2388.89402044920,
    },
    Vsop87Term {
        amplitude: 0.00000000189,
        phase: 5.07122281033,
        frequency: 263.08392337280,
    },
    Vsop87Term {
        amplitude: 0.00000000146,
        phase: 5.10373877968,
        frequency: 10770.89325626180,
    },
    Vsop87Term {
        amplitude: 0.00000000187,
        phase: 1.23915444627,
        frequency: 19402.79695281660,
    },
    Vsop87Term {
        amplitude: 0.00000000174,
        phase: 0.08407293391,
        frequency: 9380.95967271720,
    },
    Vsop87Term {
        amplitude: 0.00000000137,
        phase: 1.26247412309,
        frequency: 12566.21901028560,
    },
    Vsop87Term {
        amplitude: 0.00000000137,
        phase: 3.52826010842,
        frequency: 639.89728631400,
    },
    Vsop87Term {
        amplitude: 0.00000000148,
        phase: 1.76124372592,
        frequency: 5888.44996493220,
    },
    Vsop87Term {
        amplitude: 0.00000000164,
        phase: 2.39195095081,
        frequency: 6357.85744855870,
    },
    Vsop87Term {
        amplitude: 0.00000000146,
        phase: 2.43675816553,
        frequency: 5881.40372823420,
    },
    Vsop87Term {
        amplitude: 0.00000000161,
        phase: 1.15721259372,
        frequency: 26735.94526221320,
    },
    Vsop87Term {
        amplitude: 0.00000000131,
        phase: 2.51859277344,
        frequency: 6599.46771964800,
    },
    Vsop87Term {
        amplitude: 0.00000000153,
        phase: 5.85203687779,
        frequency: 6281.59137728310,
    },
    Vsop87Term {
        amplitude: 0.00000000151,
        phase: 3.72338532649,
        frequency: 12669.24447420140,
    },
    Vsop87Term {
        amplitude: 0.00000000132,
        phase: 2.38417741883,
        frequency: 6525.80445396540,
    },
    Vsop87Term {
        amplitude: 0.00000000129,
        phase: 0.75556744143,
        frequency: 5017.50837136500,
    },
    Vsop87Term {
        amplitude: 0.00000000127,
        phase: 0.00254936441,
        frequency: 10027.90319572920,
    },
    Vsop87Term {
        amplitude: 0.00000000148,
        phase: 2.85102145528,
        frequency: 6418.14093002680,
    },
    Vsop87Term {
        amplitude: 0.00000000143,
        phase: 5.74460279367,
        frequency: 26087.90314157420,
    },
    Vsop87Term {
        amplitude: 0.00000000172,
        phase: 0.41289962240,
        frequency: 174242.46596404970,
    },
    Vsop87Term {
        amplitude: 0.00000000136,
        phase: 4.15497742275,
        frequency: 6311.52503745920,
    },
    Vsop87Term {
        amplitude: 0.00000000170,
        phase: 5.98194913129,
        frequency: 327574.51427678125,
    },
    Vsop87Term {
        amplitude: 0.00000000124,
        phase: 1.65497607604,
        frequency: 32217.20018108080,
    },
    Vsop87Term {
        amplitude: 0.00000000136,
        phase: 2.48430783417,
        frequency: 13341.67431130680,
    },
    Vsop87Term {
        amplitude: 0.00000000165,
        phase: 2.49667924600,
        frequency: 58953.14544329400,
    },
    Vsop87Term {
        amplitude: 0.00000000123,
        phase: 3.45660563754,
        frequency: 6277.55292568400,
    },
    Vsop87Term {
        amplitude: 0.00000000117,
        phase: 0.86065134175,
        frequency: 6245.04817735560,
    },
    Vsop87Term {
        amplitude: 0.00000000149,
        phase: 5.61358280963,
        frequency: 5729.50644714900,
    },
    Vsop87Term {
        amplitude: 0.00000000153,
        phase: 0.26860029950,
        frequency: 245.83164622940,
    },
    Vsop87Term {
        amplitude: 0.00000000128,
        phase: 0.71204006588,
        frequency: 103.09277421860,
    },
    Vsop87Term {
        amplitude: 0.00000000159,
        phase: 2.43166592149,
        frequency: 221995.02880149524,
    },
    Vsop87Term {
        amplitude: 0.00000000130,
        phase: 2.80707316718,
        frequency: 6016.46880826960,
    },
    Vsop87Term {
        amplitude: 0.00000000137,
        phase: 1.70657709294,
        frequency: 12566.08438968000,
    },
    Vsop87Term {
        amplitude: 0.00000000111,
        phase: 1.56305648432,
        frequency: 17782.73207278420,
    },
    Vsop87Term {
        amplitude: 0.00000000113,
        phase: 3.58302904101,
        frequency: 25685.87280280800,
    },
    Vsop87Term {
        amplitude: 0.00000000109,
        phase: 3.26403795962,
        frequency: 6819.88036208680,
    },
    Vsop87Term {
        amplitude: 0.00000000122,
        phase: 0.34120688217,
        frequency: 1162.47470440780,
    },
    Vsop87Term {
        amplitude: 0.00000000119,
        phase: 5.84644718278,
        frequency: 12721.57209941700,
    },
    Vsop87Term {
        amplitude: 0.00000000144,
        phase: 2.28899679126,
        frequency: 12489.88562870720,
    },
    Vsop87Term {
        amplitude: 0.00000000137,
        phase: 5.82029768354,
        frequency: 44809.65020086340,
    },
    Vsop87Term {
        amplitude: 0.00000000107,
        phase: 2.42818544140,
        frequency: 5547.19933645960,
    },
    Vsop87Term {
        amplitude: 0.00000000134,
        phase: 1.26539982939,
        frequency: 5331.35744374080,
    },
    Vsop87Term {
        amplitude: 0.00000000103,
        phase: 5.96518130595,
        frequency: 6321.10352262720,
    },
    Vsop87Term {
        amplitude: 0.00000000109,
        phase: 0.33808549034,
        frequency: 11300.58422135640,
    },
    Vsop87Term {
        amplitude: 0.00000000129,
        phase: 5.89187277327,
        frequency: 12029.34718788740,
    },
    Vsop87Term {
        amplitude: 0.00000000122,
        phase: 5.77325634636,
        frequency: 11919.14086666800,
    },
    Vsop87Term {
        amplitude: 0.00000000107,
        phase: 6.24998989350,
        frequency: 77690.75950573849,
    },
    Vsop87Term {
        amplitude: 0.00000000107,
        phase: 1.00535580713,
        frequency: 77736.78343050249,
    },
    Vsop87Term {
        amplitude: 0.00000000143,
        phase: 0.24122178432,
        frequency: 4214.06901508480,
    },
    Vsop87Term {
        amplitude: 0.00000000143,
        phase: 0.88529649733,
        frequency: 7576.56007357400,
    },
    Vsop87Term {
        amplitude: 0.00000000107,
        phase: 2.92124030496,
        frequency: 31415.37924995700,
    },
    Vsop87Term {
        amplitude: 0.00000000099,
        phase: 5.70862227072,
        frequency: 5540.08578945880,
    },
    Vsop87Term {
        amplitude: 0.00000000110,
        phase: 0.37528037383,
        frequency: 5863.59120611620,
    },
    Vsop87Term {
        amplitude: 0.00000000104,
        phase: 4.44107178366,
        frequency: 2118.76386037840,
    },
    Vsop87Term {
        amplitude: 0.00000000098,
        phase: 5.95877916706,
        frequency: 4061.21921539440,
    },
    Vsop87Term {
        amplitude: 0.00000000113,
        phase: 1.24206857385,
        frequency: 84672.47584450469,
    },
    Vsop87Term {
        amplitude: 0.00000000124,
        phase: 2.55619029867,
        frequency: 12539.85338018300,
    },
    Vsop87Term {
        amplitude: 0.00000000110,
        phase: 3.66952094329,
        frequency: 238004.52415723629,
    },
    Vsop87Term {
        amplitude: 0.00000000112,
        phase: 4.32512422943,
        frequency: 97238.62754448749,
    },
    Vsop87Term {
        amplitude: 0.00000000097,
        phase: 3.70151541181,
        frequency: 11720.06886523160,
    },
    Vsop87Term {
        amplitude: 0.00000000120,
        phase: 1.26895630252,
        frequency: 12043.57428188900,
    },
    Vsop87Term {
        amplitude: 0.00000000094,
        phase: 2.56461130309,
        frequency: 19004.64794940840,
    },
    Vsop87Term {
        amplitude: 0.00000000117,
        phase: 3.65425622684,
        frequency: 34520.30930938080,
    },
    Vsop87Term {
        amplitude: 0.00000000098,
        phase: 0.13589994287,
        frequency: 11080.17157891760,
    },
    Vsop87Term {
        amplitude: 0.00000000097,
        phase: 5.38330115253,
        frequency: 7834.12107263940,
    },
    Vsop87Term {
        amplitude: 0.00000000097,
        phase: 2.46722096722,
        frequency: 71980.63357473118,
    },
    Vsop87Term {
        amplitude: 0.00000000095,
        phase: 5.36958330451,
        frequency: 6288.59877429880,
    },
    Vsop87Term {
        amplitude: 0.00000000111,
        phase: 5.01961920313,
        frequency: 11823.16163945020,
    },
    Vsop87Term {
        amplitude: 0.00000000090,
        phase: 2.72299804525,
        frequency: 26880.31981303260,
    },
    Vsop87Term {
        amplitude: 0.00000000099,
        phase: 0.90164266377,
        frequency: 18635.92845453620,
    },
    Vsop87Term {
        amplitude: 0.00000000126,
        phase: 4.78722177847,
        frequency: 305281.94307104882,
    },
    Vsop87Term {
        amplitude: 0.00000000093,
        phase: 0.21240380046,
        frequency: 18139.29450141590,
    },
    Vsop87Term {
        amplitude: 0.00000000124,
        phase: 5.00979495566,
        frequency: 172146.97134054029,
    },
    Vsop87Term {
        amplitude: 0.00000000099,
        phase: 5.67090026475,
        frequency: 16522.65971600220,
    },
    Vsop87Term {
        amplitude: 0.00000000092,
        phase: 2.28180963676,
        frequency: 12491.37010141550,
    },
    Vsop87Term {
        amplitude: 0.00000000090,
        phase: 4.50544881196,
        frequency: 40077.61957352000,
    },
    Vsop87Term {
        amplitude: 0.00000000100,
        phase: 2.00639461612,
        frequency: 12323.42309600880,
    },
    Vsop87Term {
        amplitude: 0.00000000095,
        phase: 5.68801979087,
        frequency: 14919.01785375460,
    },
    Vsop87Term {
        amplitude: 0.00000000087,
        phase: 1.86043406047,
        frequency: 27707.54249429480,
    },
    Vsop87Term {
        amplitude: 0.00000000105,
        phase: 3.02903468417,
        frequency: 22345.26037610820,
    },
    Vsop87Term {
        amplitude: 0.00000000087,
        phase: 5.43970168638,
        frequency: 6272.03014972750,
    },
    Vsop87Term {
        amplitude: 0.00000000089,
        phase: 1.63389387182,
        frequency: 33326.57873317420,
    },
    Vsop87Term {
        amplitude: 0.00000000082,
        phase: 5.58298993353,
        frequency: 10241.20229116720,
    },
    Vsop87Term {
        amplitude: 0.00000000094,
        phase: 5.47749711149,
        frequency: 9924.81042151060,
    },
    Vsop87Term {
        amplitude: 0.00000000082,
        phase: 4.71988314145,
        frequency: 15141.39079431200,
    },
    Vsop87Term {
        amplitude: 0.00000000097,
        phase: 5.61458778738,
        frequency: 2787.04302385740,
    },
    Vsop87Term {
        amplitude: 0.00000000096,
        phase: 3.89073946348,
        frequency: 6379.05507720920,
    },
    Vsop87Term {
        amplitude: 0.00000000081,
        phase: 3.13038482444,
        frequency: 36147.40987730040,
    },
    Vsop87Term {
        amplitude: 0.00000000110,
        phase: 4.89978492291,
        frequency: 72140.62866668739,
    },
    Vsop87Term {
        amplitude: 0.00000000097,
        phase: 5.20764563059,
        frequency: 6303.43116939020,
    },
    Vsop87Term {
        amplitude: 0.00000000082,
        phase: 5.26342716139,
        frequency: 9814.60410029120,
    },
    Vsop87Term {
        amplitude: 0.00000000109,
        phase: 2.35555589770,
        frequency: 83286.91426955358,
    },
    Vsop87Term {
        amplitude: 0.00000000097,
        phase: 2.58492958057,
        frequency: 30666.15495843280,
    },
    Vsop87Term {
        amplitude: 0.00000000093,
        phase: 1.32651591333,
        frequency: 23020.65308658799,
    },
    Vsop87Term {
        amplitude: 0.00000000078,
        phase: 3.99588630754,
        frequency: 11293.47067435560,
    },
    Vsop87Term {
        amplitude: 0.00000000090,
        phase: 0.57771932738,
        frequency: 26482.17080962440,
    },
    Vsop87Term {
        amplitude: 0.00000000106,
        phase: 3.92012705073,
        frequency: 62883.35513951360,
    },
    Vsop87Term {
        amplitude: 0.00000000098,
        phase: 2.94397773524,
        frequency: 316.39186965660,
    },
    Vsop87Term {
        amplitude: 0.00000000076,
        phase: 3.96310417608,
        frequency: 29026.48522950779,
    },
    Vsop87Term {
        amplitude: 0.00000000078,
        phase: 1.97068529306,
        frequency: 90279.92316810328,
    },
    Vsop87Term {
        amplitude: 0.00000000076,
        phase: 0.23027966596,
        frequency: 21424.46664430340,
    },
    Vsop87Term {
        amplitude: 0.00000000080,
        phase: 2.23099742212,
        frequency: 266.60704172180,
    },
    Vsop87Term {
        amplitude: 0.00000000079,
        phase: 1.46227790922,
        frequency: 8982.81066930900,
    },
    Vsop87Term {
        amplitude: 0.00000000102,
        phase: 4.92129953565,
        frequency: 5621.84292321040,
    },
    Vsop87Term {
        amplitude: 0.00000000100,
        phase: 0.39243148321,
        frequency: 24279.10701821359,
    },
    Vsop87Term {
        amplitude: 0.00000000071,
        phase: 1.52014858474,
        frequency: 33794.54372352860,
    },
    Vsop87Term {
        amplitude: 0.00000000076,
        phase: 0.22880641443,
        frequency: 57375.80190084620,
    },
    Vsop87Term {
        amplitude: 0.00000000091,
        phase: 0.96515913904,
        frequency: 48739.85989708300,
    },
    Vsop87Term {
        amplitude: 0.00000000075,
        phase: 2.77638585157,
        frequency: 12964.30070339100,
    },
    Vsop87Term {
        amplitude: 0.00000000077,
        phase: 5.18846946344,
        frequency: 11520.99686379520,
    },
    Vsop87Term {
        amplitude: 0.00000000068,
        phase: 0.50006599129,
        frequency: 4274.51831083240,
    },
    Vsop87Term {
        amplitude: 0.00000000075,
        phase: 2.07323762803,
        frequency: 15664.03552270859,
    },
    Vsop87Term {
        amplitude: 0.00000000074,
        phase: 1.01884134928,
        frequency: 6393.28217121080,
    },
    Vsop87Term {
        amplitude: 0.00000000077,
        phase: 0.46665178780,
        frequency: 16207.88627150200,
    },
    Vsop87Term {
        amplitude: 0.00000000081,
        phase: 4.10452219483,
        frequency: 161710.61878623239,
    },
    Vsop87Term {
        amplitude: 0.00000000067,
        phase: 3.83840630887,
        frequency: 6262.72053059260,
    },
    Vsop87Term {
        amplitude: 0.00000000071,
        phase: 3.91415523291,
        frequency: 7875.67186362420,
    },
    Vsop87Term {
        amplitude: 0.00000000081,
        phase: 0.91938383237,
        frequency: 74.78159856730,
    },
    Vsop87Term {
        amplitude: 0.00000000083,
        phase: 4.69916218791,
        frequency: 23006.42599258639,
    },
    Vsop87Term {
        amplitude: 0.00000000063,
        phase: 2.32556465878,
        frequency: 6279.19451463340,
    },
    Vsop87Term {
        amplitude: 0.00000000065,
        phase: 5.41938745446,
        frequency: 28628.33622609960,
    },
    Vsop87Term {
        amplitude: 0.00000000065,
        phase: 3.02336771694,
        frequency: 5959.57043333400,
    },
    Vsop87Term {
        amplitude: 0.00000000064,
        phase: 3.31033198370,
        frequency: 2636.72547263700,
    },
    Vsop87Term {
        amplitude: 0.00000000064,
        phase: 0.18375587519,
        frequency: 1066.49547719000,
    },
    Vsop87Term {
        amplitude: 0.00000000080,
        phase: 5.81239171612,
        frequency: 12341.80690428090,
    },
    Vsop87Term {
        amplitude: 0.00000000066,
        phase: 2.15105504851,
        frequency: 38.02767263580,
    },
    Vsop87Term {
        amplitude: 0.00000000062,
        phase: 2.43313614978,
        frequency: 10138.10951694860,
    },
    Vsop87Term {
        amplitude: 0.00000000060,
        phase: 3.16153906470,
        frequency: 5490.30096152400,
    },
    Vsop87Term {
        amplitude: 0.00000000069,
        phase: 0.30764736334,
        frequency: 7018.95236352320,
    },
    Vsop87Term {
        amplitude: 0.00000000068,
        phase: 2.24442548639,
        frequency: 24383.07910844140,
    },
    Vsop87Term {
        amplitude: 0.00000000078,
        phase: 1.39649386463,
        frequency: 9411.46461508720,
    },
    Vsop87Term {
        amplitude: 0.00000000063,
        phase: 0.72976362625,
        frequency: 6286.95718534940,
    },
    Vsop87Term {
        amplitude: 0.00000000073,
        phase: 4.95125917731,
        frequency: 6453.74872061060,
    },
    Vsop87Term {
        amplitude: 0.00000000078,
        phase: 0.32736023459,
        frequency: 6528.90749622080,
    },
    Vsop87Term {
        amplitude: 0.00000000059,
        phase: 4.95362151577,
        frequency: 35707.71008290740,
    },
    Vsop87Term {
        amplitude: 0.00000000070,
        phase: 2.37962727525,
        frequency: 15508.61512327440,
    },
    Vsop87Term {
        amplitude: 0.00000000073,
        phase: 1.35229143111,
        frequency: 5327.47610838280,
    },
    Vsop87Term {
        amplitude: 0.00000000072,
        phase: 5.91833527334,
        frequency: 10881.09957748120,
    },
    Vsop87Term {
        amplitude: 0.00000000059,
        phase: 5.36231868425,
        frequency: 10239.58386601080,
    },
    Vsop87Term {
        amplitude: 0.00000000059,
        phase: 1.63156134967,
        frequency: 61306.01159706580,
    },
    Vsop87Term {
        amplitude: 0.00000000054,
        phase: 4.29491690425,
        frequency: 21947.11137270000,
    },
    Vsop87Term {
        amplitude: 0.00000000057,
        phase: 5.89190132575,
        frequency: 34513.26307268280,
    },
    Vsop87Term {
        amplitude: 0.00000000074,
        phase: 1.38235845304,
        frequency: 9967.45389998160,
    },
    Vsop87Term {
        amplitude: 0.00000000053,
        phase: 3.86543309344,
        frequency: 32370.97899156560,
    },
    Vsop87Term {
        amplitude: 0.00000000055,
        phase: 4.51794544854,
        frequency: 34911.41207609100,
    },
    Vsop87Term {
        amplitude: 0.00000000063,
        phase: 5.41479412056,
        frequency: 11502.83761653050,
    },
    Vsop87Term {
        amplitude: 0.00000000063,
        phase: 2.34416220742,
        frequency: 11510.70192305670,
    },
    Vsop87Term {
        amplitude: 0.00000000068,
        phase: 0.77493931112,
        frequency: 29864.33402730900,
    },
    Vsop87Term {
        amplitude: 0.00000000060,
        phase: 5.57024703495,
        frequency: 5756.90800324580,
    },
    Vsop87Term {
        amplitude: 0.00000000072,
        phase: 2.80863088166,
        frequency: 10866.87248347960,
    },
    Vsop87Term {
        amplitude: 0.00000000061,
        phase: 2.69736991384,
        frequency: 82576.98122099529,
    },
    Vsop87Term {
        amplitude: 0.00000000063,
        phase: 5.32068807257,
        frequency: 3116.65941225980,
    },
    Vsop87Term {
        amplitude: 0.00000000052,
        phase: 1.02278758099,
        frequency: 6272.43918464160,
    },
    Vsop87Term {
        amplitude: 0.00000000069,
        phase: 5.00698550308,
        frequency: 25287.72379939980,
    },
    Vsop87Term {
        amplitude: 0.00000000066,
        phase: 6.12047940728,
        frequency: 12074.48840752400,
    },
    Vsop87Term {
        amplitude: 0.00000000051,
        phase: 2.59519527563,
        frequency: 11396.56344857420,
    },
    Vsop87Term {
        amplitude: 0.00000000056,
        phase: 2.57995973521,
        frequency: 17892.93839400359,
    },
    Vsop87Term {
        amplitude: 0.00000000059,
        phase: 0.44167237620,
        frequency: 250570.67585721909,
    },
    Vsop87Term {
        amplitude: 0.00000000059,
        phase: 3.84070143543,
        frequency: 5483.25472482600,
    },
    Vsop87Term {
        amplitude: 0.00000000049,
        phase: 0.54704693048,
        frequency: 22594.05489571199,
    },
    Vsop87Term {
        amplitude: 0.00000000065,
        phase: 2.38423614501,
        frequency: 52670.06959330260,
    },
    Vsop87Term {
        amplitude: 0.00000000069,
        phase: 5.34363738671,
        frequency: 66813.56483573320,
    },
    Vsop87Term {
        amplitude: 0.00000000057,
        phase: 5.42770501007,
        frequency: 310145.15282392364,
    },
    Vsop87Term {
        amplitude: 0.00000000053,
        phase: 1.17760296075,
        frequency: 149.56319713460,
    },
    Vsop87Term {
        amplitude: 0.00000000061,
        phase: 4.02090887211,
        frequency: 34596.36465465240,
    },
    Vsop87Term {
        amplitude: 0.00000000049,
        phase: 4.18361320516,
        frequency: 18606.49894600020,
    },
    Vsop87Term {
        amplitude: 0.00000000055,
        phase: 0.83886167974,
        frequency: 20452.86941222180,
    },
    Vsop87Term {
        amplitude: 0.00000000050,
        phase: 1.46327331958,
        frequency: 37455.72649597440,
    },
    Vsop87Term {
        amplitude: 0.00000000048,
        phase: 4.53854727167,
        frequency: 29822.78323632420,
    },
    Vsop87Term {
        amplitude: 0.00000000058,
        phase: 3.34847975377,
        frequency: 33990.61834428620,
    },
    Vsop87Term {
        amplitude: 0.00000000065,
        phase: 1.45522693982,
        frequency: 76251.32777062019,
    },
    Vsop87Term {
        amplitude: 0.00000000056,
        phase: 2.35650663692,
        frequency: 37724.75341974820,
    },
    Vsop87Term {
        amplitude: 0.00000000052,
        phase: 2.61551081496,
        frequency: 5999.21653112620,
    },
    Vsop87Term {
        amplitude: 0.00000000053,
        phase: 0.17334326094,
        frequency: 77717.29458646949,
    },
    Vsop87Term {
        amplitude: 0.00000000053,
        phase: 0.79879700631,
        frequency: 77710.24834977149,
    },
    Vsop87Term {
        amplitude: 0.00000000047,
        phase: 0.43240779709,
        frequency: 735.87651353180,
    },
    Vsop87Term {
        amplitude: 0.00000000053,
        phase: 4.58763261686,
        frequency: 11616.97609101300,
    },
    Vsop87Term {
        amplitude: 0.00000000048,
        phase: 6.20230111054,
        frequency: 4171.42553661380,
    },
    Vsop87Term {
        amplitude: 0.00000000052,
        phase: 1.09723616404,
        frequency: 640.87760738220,
    },
    Vsop87Term {
        amplitude: 0.00000000057,
        phase: 3.42008310383,
        frequency: 50317.20343953080,
    },
    Vsop87Term {
        amplitude: 0.00000000053,
        phase: 1.01528448581,
        frequency: 149144.46708624958,
    },
    Vsop87Term {
        amplitude: 0.00000000047,
        phase: 3.00924906195,
        frequency: 52175.80628314840,
    },
    Vsop87Term {
        amplitude: 0.00000000052,
        phase: 2.03254070404,
        frequency: 6293.71251534120,
    },
    Vsop87Term {
        amplitude: 0.00000000048,
        phase: 0.12356889734,
        frequency: 13362.44970679920,
    },
    Vsop87Term {
        amplitude: 0.00000000045,
        phase: 3.37963782356,
        frequency: 10763.77970926100,
    },
    Vsop87Term {
        amplitude: 0.00000000047,
        phase: 5.50981287869,
        frequency: 12779.45079542080,
    },
    Vsop87Term {
        amplitude: 0.00000000062,
        phase: 5.45209070099,
        frequency: 949.17560896980,
    },
    Vsop87Term {
        amplitude: 0.00000000061,
        phase: 2.93237974631,
        frequency: 5791.41255753260,
    },
    Vsop87Term {
        amplitude: 0.00000000044,
        phase: 2.87440620802,
        frequency: 8584.66166590080,
    },
    Vsop87Term {
        amplitude: 0.00000000046,
        phase: 4.03141796560,
        frequency: 10667.80048204320,
    },
    Vsop87Term {
        amplitude: 0.00000000047,
        phase: 3.89902931422,
        frequency: 3903.91137641980,
    },
    Vsop87Term {
        amplitude: 0.00000000046,
        phase: 2.75700467329,
        frequency: 6993.00889854970,
    },
    Vsop87Term {
        amplitude: 0.00000000045,
        phase: 1.93386293300,
        frequency: 206.18554843720,
    },
    Vsop87Term {
        amplitude: 0.00000000047,
        phase: 2.57670800912,
        frequency: 11492.54267579200,
    },
    Vsop87Term {
        amplitude: 0.00000000044,
        phase: 3.62570223167,
        frequency: 63658.87775083760,
    },
    Vsop87Term {
        amplitude: 0.00000000051,
        phase: 0.84536826273,
        frequency: 12345.73905754400,
    },
    Vsop87Term {
        amplitude: 0.00000000043,
        phase: 0.01524970172,
        frequency: 37853.87549938260,
    },
    Vsop87Term {
        amplitude: 0.00000000041,
        phase: 3.27146326065,
        frequency: 8858.31494432060,
    },
    Vsop87Term {
        amplitude: 0.00000000045,
        phase: 3.03765521215,
        frequency: 65236.22129328540,
    },
    Vsop87Term {
        amplitude: 0.00000000047,
        phase: 1.44447548944,
        frequency: 21393.54196985760,
    },
    Vsop87Term {
        amplitude: 0.00000000058,
        phase: 5.45843180927,
        frequency: 1975.49254585600,
    },
    Vsop87Term {
        amplitude: 0.00000000050,
        phase: 2.13285524146,
        frequency: 12573.26524698360,
    },
    Vsop87Term {
        amplitude: 0.00000000041,
        phase: 1.32190847146,
        frequency: 2547.83753823240,
    },
    Vsop87Term {
        amplitude: 0.00000000047,
        phase: 3.67579608544,
        frequency: 28313.28880466100,
    },
    Vsop87Term {
        amplitude: 0.00000000041,
        phase: 2.24013475126,
        frequency: 8273.82086703240,
    },
    Vsop87Term {
        amplitude: 0.00000000047,
        phase: 6.21438985953,
        frequency: 10991.30589870060,
    },
    Vsop87Term {
        amplitude: 0.00000000042,
        phase: 3.01631817350,
        frequency: 853.19638175200,
    },
    Vsop87Term {
        amplitude: 0.00000000056,
        phase: 1.09773690181,
        frequency: 77376.20102240759,
    },
    Vsop87Term {
        amplitude: 0.00000000040,
        phase: 2.35698541041,
        frequency: 2699.73481931760,
    },
    Vsop87Term {
        amplitude: 0.00000000043,
        phase: 5.28030898459,
        frequency: 17796.95916678580,
    },
    Vsop87Term {
        amplitude: 0.00000000054,
        phase: 2.59175932091,
        frequency: 22910.44676536859,
    },
    Vsop87Term {
        amplitude: 0.00000000054,
        phase: 0.88027764102,
        frequency: 71960.38658322369,
    },
    Vsop87Term {
        amplitude: 0.00000000055,
        phase: 0.07988899477,
        frequency: 83467.15635301729,
    },
    Vsop87Term {
        amplitude: 0.00000000039,
        phase: 1.12867321442,
        frequency: 9910.58332750900,
    },
    Vsop87Term {
        amplitude: 0.00000000040,
        phase: 1.35670430524,
        frequency: 27177.85152920020,
    },
    Vsop87Term {
        amplitude: 0.00000000039,
        phase: 4.39624220245,
        frequency: 5618.31980486140,
    },
    Vsop87Term {
        amplitude: 0.00000000042,
        phase: 4.78798367468,
        frequency: 7856.89627409019,
    },
    Vsop87Term {
        amplitude: 0.00000000047,
        phase: 2.75482175292,
        frequency: 18202.21671665939,
    },
    Vsop87Term {
        amplitude: 0.00000000039,
        phase: 1.97008298629,
        frequency: 24491.42579258340,
    },
    Vsop87Term {
        amplitude: 0.00000000042,
        phase: 4.04346599946,
        frequency: 7863.94251078820,
    },
    Vsop87Term {
        amplitude: 0.00000000038,
        phase: 0.49178679251,
        frequency: 38650.17350619900,
    },
    Vsop87Term {
        amplitude: 0.00000000036,
        phase: 4.86047906533,
        frequency: 4157.19844261220,
    },
    Vsop87Term {
        amplitude: 0.00000000043,
        phase: 5.64354880978,
        frequency: 1062.90504853820,
    },
    Vsop87Term {
        amplitude: 0.00000000036,
        phase: 3.98066313627,
        frequency: 12565.17137891460,
    },
    Vsop87Term {
        amplitude: 0.00000000042,
        phase: 2.30753932657,
        frequency: 6549.68289171320,
    },
    Vsop87Term {
        amplitude: 0.00000000040,
        phase: 5.39694918320,
        frequency: 9498.21223063460,
    },
    Vsop87Term {
        amplitude: 0.00000000040,
        phase: 3.30603243754,
        frequency: 23536.11695768099,
    },
    Vsop87Term {
        amplitude: 0.00000000050,
        phase: 6.15760345261,
        frequency: 78051.34191383338,
    },
];

pub(super) const R1: &[Vsop87Term] = &[
    Vsop87Term {
        amplitude: 0.00103018608,
        phase: 1.10748969588,
        frequency: 6283.07584999140,
    },
    Vsop87Term {
        amplitude: 0.00001721238,
        phase: 1.06442301418,
        frequency: 12566.15169998280,
    },
    Vsop87Term {
        amplitude: 0.00000702215,
        phase: 3.14159265359,
        frequency: 0.00000000000,
    },
    Vsop87Term {
        amplitude: 0.00000032346,
        phase: 1.02169059149,
        frequency: 18849.22754997420,
    },
    Vsop87Term {
        amplitude: 0.00000030799,
        phase: 2.84353804832,
        frequency: 5507.55323866740,
    },
    Vsop87Term {
        amplitude: 0.00000024971,
        phase: 1.31906709482,
        frequency: 5223.69391980220,
    },
    Vsop87Term {
        amplitude: 0.00000018485,
        phase: 1.42429748614,
        frequency: 1577.34354244780,
    },
    Vsop87Term {
        amplitude: 0.00000010078,
        phase: 5.91378194648,
        frequency: 10977.07880469900,
    },
    Vsop87Term {
        amplitude: 0.00000008634,
        phase: 0.27146150602,
        frequency: 5486.77784317500,
    },
    Vsop87Term {
        amplitude: 0.00000008654,
        phase: 1.42046854427,
        frequency: 6275.96230299060,
    },
    Vsop87Term {
        amplitude: 0.00000005069,
        phase: 1.68613426734,
        frequency: 5088.62883976680,
    },
    Vsop87Term {
        amplitude: 0.00000004985,
        phase: 6.01401770704,
        frequency: 6286.59896834040,
    },
    Vsop87Term {
        amplitude: 0.00000004669,
        phase: 5.98724494073,
        frequency: 529.69096509460,
    },
    Vsop87Term {
        amplitude: 0.00000004395,
        phase: 0.51800238019,
        frequency: 4694.00295470760,
    },
    Vsop87Term {
        amplitude: 0.00000003872,
        phase: 4.74969833437,
        frequency: 2544.31441988340,
    },
    Vsop87Term {
        amplitude: 0.00000003750,
        phase: 5.07097685568,
        frequency: 796.29800681640,
    },
    Vsop87Term {
        amplitude: 0.00000004100,
        phase: 1.08424786092,
        frequency: 9437.76293488700,
    },
    Vsop87Term {
        amplitude: 0.00000003518,
        phase: 0.02290216272,
        frequency: 83996.84731811189,
    },
    Vsop87Term {
        amplitude: 0.00000003436,
        phase: 0.94937019624,
        frequency: 71430.69561812909,
    },
    Vsop87Term {
        amplitude: 0.00000003221,
        phase: 6.15628775313,
        frequency: 2146.16541647520,
    },
    Vsop87Term {
        amplitude: 0.00000003414,
        phase: 5.41218322538,
        frequency: 775.52261132400,
    },
    Vsop87Term {
        amplitude: 0.00000002863,
        phase: 5.48432847146,
        frequency: 10447.38783960440,
    },
    Vsop87Term {
        amplitude: 0.00000002520,
        phase: 0.24276941146,
        frequency: 398.14900340820,
    },
    Vsop87Term {
        amplitude: 0.00000002201,
        phase: 4.95216196651,
        frequency: 6812.76681508600,
    },
    Vsop87Term {
        amplitude: 0.00000002186,
        phase: 0.41991743105,
        frequency: 8031.09226305840,
    },
    Vsop87Term {
        amplitude: 0.00000002838,
        phase: 3.42034351366,
        frequency: 2352.86615377180,
    },
    Vsop87Term {
        amplitude: 0.00000002554,
        phase: 6.13241878525,
        frequency: 6438.49624942560,
    },
    Vsop87Term {
        amplitude: 0.00000001932,
        phase: 5.31374608366,
        frequency: 8429.24126646660,
    },
    Vsop87Term {
        amplitude: 0.00000002429,
        phase: 3.09164528262,
        frequency: 4690.47983635860,
    },
    Vsop87Term {
        amplitude: 0.00000001730,
        phase: 1.53686208550,
        frequency: 4705.73230754360,
    },
    Vsop87Term {
        amplitude: 0.00000002250,
        phase: 3.68863633842,
        frequency: 7084.89678111520,
    },
    Vsop87Term {
        amplitude: 0.00000002093,
        phase: 1.28191783032,
        frequency: 1748.01641306700,
    },
    Vsop87Term {
        amplitude: 0.00000001441,
        phase: 0.81656250862,
        frequency: 14143.49524243060,
    },
    Vsop87Term {
        amplitude: 0.00000001483,
        phase: 3.22225357771,
        frequency: 7234.79425624200,
    },
    Vsop87Term {
        amplitude: 0.00000001754,
        phase: 3.22883705112,
        frequency: 6279.55273164240,
    },
    Vsop87Term {
        amplitude: 0.00000001583,
        phase: 4.09702349428,
        frequency: 11499.65622279280,
    },
    Vsop87Term {
        amplitude: 0.00000001575,
        phase: 5.53890170575,
        frequency: 3154.68708489560,
    },
    Vsop87Term {
        amplitude: 0.00000001847,
        phase: 1.82040335363,
        frequency: 7632.94325965020,
    },
    Vsop87Term {
        amplitude: 0.00000001504,
        phase: 3.63293385726,
        frequency: 11513.88331679440,
    },
    Vsop87Term {
        amplitude: 0.00000001337,
        phase: 4.64440864339,
        frequency: 6836.64525283380,
    },
    Vsop87Term {
        amplitude: 0.00000001275,
        phase: 2.69341415363,
        frequency: 1349.86740965880,
    },
    Vsop87Term {
        amplitude: 0.00000001352,
        phase: 6.15101580257,
        frequency: 5746.27133789600,
    },
    Vsop87Term {
        amplitude: 0.00000001125,
        phase: 3.35673439497,
        frequency: 17789.84561978500,
    },
    Vsop87Term {
        amplitude: 0.00000001470,
        phase: 3.65282991755,
        frequency: 1194.44701022460,
    },
    Vsop87Term {
        amplitude: 0.00000001177,
        phase: 2.57676109092,
        frequency: 13367.97263110660,
    },
    Vsop87Term {
        amplitude: 0.00000001101,
        phase: 4.49748696552,
        frequency: 4292.33083295040,
    },
    Vsop87Term {
        amplitude: 0.00000001234,
        phase: 5.65036509521,
        frequency: 5760.49843189760,
    },
    Vsop87Term {
        amplitude: 0.00000000984,
        phase: 0.65517395136,
        frequency: 5856.47765911540,
    },
    Vsop87Term {
        amplitude: 0.00000000928,
        phase: 2.32420318751,
        frequency: 10213.28554621100,
    },
    Vsop87Term {
        amplitude: 0.00000001077,
        phase: 5.82812169132,
        frequency: 12036.46073488820,
    },
    Vsop87Term {
        amplitude: 0.00000000916,
        phase: 0.76613009583,
        frequency: 16730.46368959580,
    },
    Vsop87Term {
        amplitude: 0.00000000877,
        phase: 1.50137505051,
        frequency: 11926.25441366880,
    },
    Vsop87Term {
        amplitude: 0.00000001023,
        phase: 5.62076589825,
        frequency: 6256.77753019160,
    },
    Vsop87Term {
        amplitude: 0.00000000851,
        phase: 0.65709335533,
        frequency: 155.42039943420,
    },
    Vsop87Term {
        amplitude: 0.00000000802,
        phase: 4.10519132088,
        frequency: 951.71840625060,
    },
    Vsop87Term {
        amplitude: 0.00000000857,
        phase: 1.41661697538,
        frequency: 5753.38488489680,
    },
    Vsop87Term {
        amplitude: 0.00000000994,
        phase: 1.14418521187,
        frequency: 1059.38193018920,
    },
    Vsop87Term {
        amplitude: 0.00000000813,
        phase: 1.63948433322,
        frequency: 6681.22485339960,
    },
    Vsop87Term {
        amplitude: 0.00000000662,
        phase: 4.55200452260,
        frequency: 5216.58037280140,
    },
    Vsop87Term {
        amplitude: 0.00000000644,
        phase: 4.19478168733,
        frequency: 6040.34724601740,
    },
    Vsop87Term {
        amplitude: 0.00000000626,
        phase: 1.50767713598,
        frequency: 5643.17856367740,
    },
    Vsop87Term {
        amplitude: 0.00000000590,
        phase: 6.18277145205,
        frequency: 4164.31198961300,
    },
    Vsop87Term {
        amplitude: 0.00000000635,
        phase: 0.52413263542,
        frequency: 6290.18939699220,
    },
    Vsop87Term {
        amplitude: 0.00000000650,
        phase: 0.97935690350,
        frequency: 25132.30339996560,
    },
    Vsop87Term {
        amplitude: 0.00000000568,
        phase: 2.30125315873,
        frequency: 10973.55568635000,
    },
    Vsop87Term {
        amplitude: 0.00000000547,
        phase: 5.27256412213,
        frequency: 3340.61242669980,
    },
    Vsop87Term {
        amplitude: 0.00000000547,
        phase: 2.20144422886,
        frequency: 1592.59601363280,
    },
    Vsop87Term {
        amplitude: 0.00000000526,
        phase: 0.92464258226,
        frequency: 11371.70468975820,
    },
    Vsop87Term {
        amplitude: 0.00000000490,
        phase: 5.90951388655,
        frequency: 3894.18182954220,
    },
    Vsop87Term {
        amplitude: 0.00000000478,
        phase: 1.66857963179,
        frequency: 12168.00269657460,
    },
    Vsop87Term {
        amplitude: 0.00000000516,
        phase: 3.59803483887,
        frequency: 10969.96525769820,
    },
    Vsop87Term {
        amplitude: 0.00000000518,
        phase: 3.97914412373,
        frequency: 17298.18232732620,
    },
    Vsop87Term {
        amplitude: 0.00000000534,
        phase: 5.03740926442,
        frequency: 9917.69687450980,
    },
    Vsop87Term {
        amplitude: 0.00000000487,
        phase: 2.50545369269,
        frequency: 6127.65545055720,
    },
    Vsop87Term {
        amplitude: 0.00000000416,
        phase: 4.04828175503,
        frequency: 10984.19235169980,
    },
    Vsop87Term {
        amplitude: 0.00000000538,
        phase: 5.54081539805,
        frequency: 553.56940284240,
    },
    Vsop87Term {
        amplitude: 0.00000000402,
        phase: 2.16544019233,
        frequency: 7860.41939243920,
    },
    Vsop87Term {
        amplitude: 0.00000000553,
        phase: 2.32177369366,
        frequency: 11506.76976979360,
    },
    Vsop87Term {
        amplitude: 0.00000000367,
        phase: 3.39152532250,
        frequency: 6496.37494542940,
    },
    Vsop87Term {
        amplitude: 0.00000000360,
        phase: 5.34379853282,
        frequency: 7079.37385680780,
    },
    Vsop87Term {
        amplitude: 0.00000000337,
        phase: 3.61563704045,
        frequency: 11790.62908865880,
    },
    Vsop87Term {
        amplitude: 0.00000000456,
        phase: 0.30754294809,
        frequency: 801.82093112380,
    },
    Vsop87Term {
        amplitude: 0.00000000417,
        phase: 3.70009308674,
        frequency: 10575.40668294180,
    },
    Vsop87Term {
        amplitude: 0.00000000381,
        phase: 5.82033971802,
        frequency: 7058.59846131540,
    },
    Vsop87Term {
        amplitude: 0.00000000321,
        phase: 0.31988767355,
        frequency: 16200.77272450120,
    },
    Vsop87Term {
        amplitude: 0.00000000364,
        phase: 1.08414306177,
        frequency: 6309.37416979120,
    },
    Vsop87Term {
        amplitude: 0.00000000294,
        phase: 4.54798604957,
        frequency: 11856.21865142450,
    },
    Vsop87Term {
        amplitude: 0.00000000290,
        phase: 1.26473978562,
        frequency: 8635.94200376320,
    },
    Vsop87Term {
        amplitude: 0.00000000399,
        phase: 4.16998866302,
        frequency: 26.29831979980,
    },
    Vsop87Term {
        amplitude: 0.00000000262,
        phase: 5.08316906342,
        frequency: 10177.25767953360,
    },
    Vsop87Term {
        amplitude: 0.00000000243,
        phase: 2.25746091190,
        frequency: 11712.95531823080,
    },
    Vsop87Term {
        amplitude: 0.00000000237,
        phase: 1.05070575346,
        frequency: 242.72860397400,
    },
    Vsop87Term {
        amplitude: 0.00000000275,
        phase: 3.45319481756,
        frequency: 5884.92684658320,
    },
    Vsop87Term {
        amplitude: 0.00000000255,
        phase: 5.38496831087,
        frequency: 21228.39202354580,
    },
    Vsop87Term {
        amplitude: 0.00000000307,
        phase: 4.24313526604,
        frequency: 3738.76143010800,
    },
    Vsop87Term {
        amplitude: 0.00000000216,
        phase: 3.46037894728,
        frequency: 213.29909543800,
    },
    Vsop87Term {
        amplitude: 0.00000000196,
        phase: 0.69029243914,
        frequency: 1990.74501704100,
    },
    Vsop87Term {
        amplitude: 0.00000000198,
        phase: 5.16301829964,
        frequency: 12352.85260454480,
    },
    Vsop87Term {
        amplitude: 0.00000000214,
        phase: 3.91876200279,
        frequency: 13916.01910964160,
    },
    Vsop87Term {
        amplitude: 0.00000000212,
        phase: 4.00861198517,
        frequency: 5230.80746680300,
    },
    Vsop87Term {
        amplitude: 0.00000000184,
        phase: 5.59805976614,
        frequency: 6283.14316029419,
    },
    Vsop87Term {
        amplitude: 0.00000000184,
        phase: 2.85275392124,
        frequency: 7238.67559160000,
    },
    Vsop87Term {
        amplitude: 0.00000000179,
        phase: 2.54259058334,
        frequency: 14314.16811304980,
    },
    Vsop87Term {
        amplitude: 0.00000000225,
        phase: 1.64458698399,
        frequency: 4732.03062734340,
    },
    Vsop87Term {
        amplitude: 0.00000000236,
        phase: 5.58826125715,
        frequency: 6069.77675455340,
    },
    Vsop87Term {
        amplitude: 0.00000000187,
        phase: 2.72805985443,
        frequency: 6062.66320755260,
    },
    Vsop87Term {
        amplitude: 0.00000000184,
        phase: 6.04216273598,
        frequency: 6283.00853968860,
    },
    Vsop87Term {
        amplitude: 0.00000000230,
        phase: 3.62591335086,
        frequency: 6284.05617105960,
    },
    Vsop87Term {
        amplitude: 0.00000000163,
        phase: 2.19117396803,
        frequency: 18073.70493865020,
    },
    Vsop87Term {
        amplitude: 0.00000000172,
        phase: 0.97612950740,
        frequency: 3930.20969621960,
    },
    Vsop87Term {
        amplitude: 0.00000000215,
        phase: 1.04672844028,
        frequency: 3496.03282613400,
    },
    Vsop87Term {
        amplitude: 0.00000000169,
        phase: 4.75084479006,
        frequency: 17267.26820169119,
    },
    Vsop87Term {
        amplitude: 0.00000000152,
        phase: 0.19390712179,
        frequency: 9779.10867612540,
    },
    Vsop87Term {
        amplitude: 0.00000000182,
        phase: 5.16288118255,
        frequency: 17253.04110768959,
    },
    Vsop87Term {
        amplitude: 0.00000000149,
        phase: 0.80944184260,
        frequency: 709.93304855830,
    },
    Vsop87Term {
        amplitude: 0.00000000163,
        phase: 2.19209570390,
        frequency: 6076.89030155420,
    },
    Vsop87Term {
        amplitude: 0.00000000186,
        phase: 5.01159497089,
        frequency: 11015.10647733480,
    },
    Vsop87Term {
        amplitude: 0.00000000134,
        phase: 0.97765485759,
        frequency: 65147.61976813770,
    },
    Vsop87Term {
        amplitude: 0.00000000141,
        phase: 4.38421981312,
        frequency: 4136.91043351620,
    },
    Vsop87Term {
        amplitude: 0.00000000158,
        phase: 4.60974280627,
        frequency: 9623.68827669120,
    },
    Vsop87Term {
        amplitude: 0.00000000133,
        phase: 3.30508592837,
        frequency: 154717.60988768269,
    },
    Vsop87Term {
        amplitude: 0.00000000163,
        phase: 6.11782626245,
        frequency: 3.52311834900,
    },
    Vsop87Term {
        amplitude: 0.00000000174,
        phase: 1.58078542187,
        frequency: 7.11354700080,
    },
    Vsop87Term {
        amplitude: 0.00000000141,
        phase: 0.49976927274,
        frequency: 25158.60171976540,
    },
    Vsop87Term {
        amplitude: 0.00000000124,
        phase: 6.03440460031,
        frequency: 9225.53927328300,
    },
    Vsop87Term {
        amplitude: 0.00000000150,
        phase: 5.30166336812,
        frequency: 13517.87010623340,
    },
    Vsop87Term {
        amplitude: 0.00000000127,
        phase: 1.92389511438,
        frequency: 22483.84857449259,
    },
    Vsop87Term {
        amplitude: 0.00000000121,
        phase: 2.37813129011,
        frequency: 167283.76158766549,
    },
    Vsop87Term {
        amplitude: 0.00000000120,
        phase: 3.98423684853,
        frequency: 4686.88940770680,
    },
    Vsop87Term {
        amplitude: 0.00000000117,
        phase: 5.81072642211,
        frequency: 12569.67481833180,
    },
    Vsop87Term {
        amplitude: 0.00000000122,
        phase: 5.60973054224,
        frequency: 5642.19824260920,
    },
    Vsop87Term {
        amplitude: 0.00000000157,
        phase: 3.40236426002,
        frequency: 16496.36139620240,
    },
    Vsop87Term {
        amplitude: 0.00000000129,
        phase: 2.10705116371,
        frequency: 1589.07289528380,
    },
    Vsop87Term {
        amplitude: 0.00000000116,
        phase: 0.55839966736,
        frequency: 5849.36411211460,
    },
    Vsop87Term {
        amplitude: 0.00000000123,
        phase: 1.52961392771,
        frequency: 12559.03815298200,
    },
    Vsop87Term {
        amplitude: 0.00000000111,
        phase: 0.44848279675,
        frequency: 6172.86952877200,
    },
    Vsop87Term {
        amplitude: 0.00000000123,
        phase: 5.81645568991,
        frequency: 6282.09552892320,
    },
    Vsop87Term {
        amplitude: 0.00000000150,
        phase: 4.26278409223,
        frequency: 3128.38876509580,
    },
    Vsop87Term {
        amplitude: 0.00000000106,
        phase: 2.27437761356,
        frequency: 5429.87946823940,
    },
    Vsop87Term {
        amplitude: 0.00000000104,
        phase: 4.42743707728,
        frequency: 23543.23050468179,
    },
    Vsop87Term {
        amplitude: 0.00000000121,
        phase: 0.39459045915,
        frequency: 12132.43996210600,
    },
    Vsop87Term {
        amplitude: 0.00000000104,
        phase: 2.41842602527,
        frequency: 426.59819087600,
    },
    Vsop87Term {
        amplitude: 0.00000000110,
        phase: 5.80381480447,
        frequency: 16858.48253293320,
    },
    Vsop87Term {
        amplitude: 0.00000000100,
        phase: 2.93805577485,
        frequency: 4535.05943692440,
    },
    Vsop87Term {
        amplitude: 0.00000000097,
        phase: 3.97935904984,
        frequency: 6133.51265285680,
    },
    Vsop87Term {
        amplitude: 0.00000000110,
        phase: 6.22339014386,
        frequency: 12146.66705610760,
    },
    Vsop87Term {
        amplitude: 0.00000000098,
        phase: 0.87576563709,
        frequency: 6525.80445396540,
    },
    Vsop87Term {
        amplitude: 0.00000000098,
        phase: 3.15248421301,
        frequency: 10440.27429260360,
    },
    Vsop87Term {
        amplitude: 0.00000000095,
        phase: 2.46168411100,
        frequency: 3097.88382272579,
    },
    Vsop87Term {
        amplitude: 0.00000000088,
        phase: 0.23371480284,
        frequency: 13119.72110282519,
    },
    Vsop87Term {
        amplitude: 0.00000000098,
        phase: 5.77016493489,
        frequency: 7342.45778018060,
    },
    Vsop87Term {
        amplitude: 0.00000000092,
        phase: 6.03915555063,
        frequency: 20426.57109242200,
    },
    Vsop87Term {
        amplitude: 0.00000000096,
        phase: 5.56909292561,
        frequency: 2388.89402044920,
    },
    Vsop87Term {
        amplitude: 0.00000000081,
        phase: 1.32131147691,
        frequency: 5650.29211067820,
    },
    Vsop87Term {
        amplitude: 0.00000000086,
        phase: 3.94529200528,
        frequency: 10454.50138660520,
    },
    Vsop87Term {
        amplitude: 0.00000000076,
        phase: 2.70729716925,
        frequency: 143571.32428481648,
    },
    Vsop87Term {
        amplitude: 0.00000000091,
        phase: 5.64100034152,
        frequency: 8827.39026987480,
    },
    Vsop87Term {
        amplitude: 0.00000000076,
        phase: 1.80783856698,
        frequency: 28286.99048486120,
    },
    Vsop87Term {
        amplitude: 0.00000000081,
        phase: 1.90858992196,
        frequency: 29088.81141598500,
    },
    Vsop87Term {
        amplitude: 0.00000000075,
        phase: 3.40955892978,
        frequency: 5481.25491886760,
    },
    Vsop87Term {
        amplitude: 0.00000000069,
        phase: 4.49936170873,
        frequency: 17256.63153634140,
    },
    Vsop87Term {
        amplitude: 0.00000000088,
        phase: 1.10098454357,
        frequency: 11769.85369316640,
    },
    Vsop87Term {
        amplitude: 0.00000000066,
        phase: 2.78285801977,
        frequency: 536.80451209540,
    },
    Vsop87Term {
        amplitude: 0.00000000068,
        phase: 3.88179770758,
        frequency: 17260.15465469040,
    },
    Vsop87Term {
        amplitude: 0.00000000084,
        phase: 1.59303306354,
        frequency: 9380.95967271720,
    },
    Vsop87Term {
        amplitude: 0.00000000088,
        phase: 3.88076636762,
        frequency: 7477.52286021600,
    },
    Vsop87Term {
        amplitude: 0.00000000061,
        phase: 6.17558202197,
        frequency: 11087.28512591840,
    },
    Vsop87Term {
        amplitude: 0.00000000060,
        phase: 4.34824715818,
        frequency: 6206.80977871580,
    },
    Vsop87Term {
        amplitude: 0.00000000082,
        phase: 4.59843208943,
        frequency: 9388.00590941520,
    },
    Vsop87Term {
        amplitude: 0.00000000079,
        phase: 1.63131230601,
        frequency: 4933.20844033260,
    },
    Vsop87Term {
        amplitude: 0.00000000078,
        phase: 4.20905757484,
        frequency: 5729.50644714900,
    },
    Vsop87Term {
        amplitude: 0.00000000057,
        phase: 5.48157926651,
        frequency: 18319.53658487960,
    },
    Vsop87Term {
        amplitude: 0.00000000060,
        phase: 1.01261781084,
        frequency: 12721.57209941700,
    },
    Vsop87Term {
        amplitude: 0.00000000056,
        phase: 1.63031935692,
        frequency: 15720.83878487840,
    },
    Vsop87Term {
        amplitude: 0.00000000055,
        phase: 0.24926735018,
        frequency: 15110.46611986620,
    },
    Vsop87Term {
        amplitude: 0.00000000061,
        phase: 5.93059279661,
        frequency: 12539.85338018300,
    },
    Vsop87Term {
        amplitude: 0.00000000055,
        phase: 4.84298966314,
        frequency: 13095.84266507740,
    },
    Vsop87Term {
        amplitude: 0.00000000067,
        phase: 6.11690589247,
        frequency: 8662.24032356300,
    },
    Vsop87Term {
        amplitude: 0.00000000054,
        phase: 5.73750638571,
        frequency: 3634.62102451840,
    },
    Vsop87Term {
        amplitude: 0.00000000074,
        phase: 1.05466745829,
        frequency: 16460.33352952499,
    },
    Vsop87Term {
        amplitude: 0.00000000053,
        phase: 2.29084335688,
        frequency: 16062.18452611680,
    },
    Vsop87Term {
        amplitude: 0.00000000064,
        phase: 2.13513767927,
        frequency: 7875.67186362420,
    },
    Vsop87Term {
        amplitude: 0.00000000067,
        phase: 0.07096807518,
        frequency: 14945.31617355440,
    },
    Vsop87Term {
        amplitude: 0.00000000051,
        phase: 2.31511194429,
        frequency: 6262.72053059260,
    },
    Vsop87Term {
        amplitude: 0.00000000057,
        phase: 5.77055471237,
        frequency: 12043.57428188900,
    },
    Vsop87Term {
        amplitude: 0.00000000056,
        phase: 4.41980790431,
        frequency: 4701.11650170840,
    },
    Vsop87Term {
        amplitude: 0.00000000059,
        phase: 5.87963500073,
        frequency: 5331.35744374080,
    },
    Vsop87Term {
        amplitude: 0.00000000058,
        phase: 2.30546168628,
        frequency: 955.59974160860,
    },
    Vsop87Term {
        amplitude: 0.00000000049,
        phase: 1.93839278478,
        frequency: 5333.90024102160,
    },
    Vsop87Term {
        amplitude: 0.00000000048,
        phase: 2.69973662261,
        frequency: 6709.67404086740,
    },
    Vsop87Term {
        amplitude: 0.00000000064,
        phase: 1.64379897981,
        frequency: 6262.30045449900,
    },
    Vsop87Term {
        amplitude: 0.00000000046,
        phase: 3.98449608961,
        frequency: 98068.53671630539,
    },
    Vsop87Term {
        amplitude: 0.00000000050,
        phase: 3.68875893005,
        frequency: 12323.42309600880,
    },
    Vsop87Term {
        amplitude: 0.00000000045,
        phase: 3.30068569697,
        frequency: 22003.91463486980,
    },
    Vsop87Term {
        amplitude: 0.00000000047,
        phase: 1.26317154881,
        frequency: 11919.14086666800,
    },
    Vsop87Term {
        amplitude: 0.00000000045,
        phase: 0.89150445122,
        frequency: 51868.24866217880,
    },
    Vsop87Term {
        amplitude: 0.00000000043,
        phase: 1.61526242998,
        frequency: 6277.55292568400,
    },
    Vsop87Term {
        amplitude: 0.00000000043,
        phase: 5.74295325645,
        frequency: 11403.67699557500,
    },
    Vsop87Term {
        amplitude: 0.00000000044,
        phase: 3.43070646822,
        frequency: 10021.83728009940,
    },
    Vsop87Term {
        amplitude: 0.00000000056,
        phase: 0.02481833774,
        frequency: 15671.08175940660,
    },
    Vsop87Term {
        amplitude: 0.00000000055,
        phase: 3.14274403422,
        frequency: 33019.02111220460,
    },
    Vsop87Term {
        amplitude: 0.00000000045,
        phase: 3.00877289177,
        frequency: 8982.81066930900,
    },
    Vsop87Term {
        amplitude: 0.00000000046,
        phase: 0.73303568429,
        frequency: 6303.43116939020,
    },
    Vsop87Term {
        amplitude: 0.00000000049,
        phase: 1.60455690285,
        frequency: 6303.85124548380,
    },
    Vsop87Term {
        amplitude: 0.00000000045,
        phase: 0.40210030323,
        frequency: 6805.65326808520,
    },
    Vsop87Term {
        amplitude: 0.00000000053,
        phase: 0.94869680175,
        frequency: 10988.80815753500,
    },
    Vsop87Term {
        amplitude: 0.00000000041,
        phase: 1.61122384329,
        frequency: 6819.88036208680,
    },
    Vsop87Term {
        amplitude: 0.00000000055,
        phase: 0.89439119424,
        frequency: 11933.36796066960,
    },
    Vsop87Term {
        amplitude: 0.00000000045,
        phase: 3.88495384656,
        frequency: 60530.48898574180,
    },
    Vsop87Term {
        amplitude: 0.00000000040,
        phase: 4.75740908001,
        frequency: 38526.57435087200,
    },
    Vsop87Term {
        amplitude: 0.00000000040,
        phase: 1.49921251887,
        frequency: 18451.07854656599,
    },
    Vsop87Term {
        amplitude: 0.00000000040,
        phase: 3.77498297228,
        frequency: 26087.90314157420,
    },
    Vsop87Term {
        amplitude: 0.00000000051,
        phase: 1.70258603562,
        frequency: 1551.04522264800,
    },
    Vsop87Term {
        amplitude: 0.00000000039,
        phase: 2.97100699926,
        frequency: 2118.76386037840,
    },
    Vsop87Term {
        amplitude: 0.00000000053,
        phase: 5.19854123078,
        frequency: 77713.77146812050,
    },
    Vsop87Term {
        amplitude: 0.00000000047,
        phase: 4.26356628717,
        frequency: 21424.46664430340,
    },
    Vsop87Term {
        amplitude: 0.00000000037,
        phase: 0.62902722802,
        frequency: 24356.78078864160,
    },
    Vsop87Term {
        amplitude: 0.00000000036,
        phase: 0.11087914947,
        frequency: 10344.29506538580,
    },
    Vsop87Term {
        amplitude: 0.00000000036,
        phase: 0.77037556319,
        frequency: 12029.34718788740,
    },
    Vsop87Term {
        amplitude: 0.00000000035,
        phase: 3.30933994515,
        frequency: 24072.92146977640,
    },
    Vsop87Term {
        amplitude: 0.00000000035,
        phase: 5.93650887012,
        frequency: 31570.79964939120,
    },
    Vsop87Term {
        amplitude: 0.00000000036,
        phase: 2.15108874765,
        frequency: 30774.50164257480,
    },
    Vsop87Term {
        amplitude: 0.00000000036,
        phase: 1.75078825382,
        frequency: 16207.88627150200,
    },
    Vsop87Term {
        amplitude: 0.00000000033,
        phase: 5.06264177921,
        frequency: 226858.23855437008,
    },
    Vsop87Term {
        amplitude: 0.00000000034,
        phase: 6.16891378800,
        frequency: 24491.42579258340,
    },
    Vsop87Term {
        amplitude: 0.00000000035,
        phase: 3.19120695549,
        frequency: 32217.20018108080,
    },
    Vsop87Term {
        amplitude: 0.00000000034,
        phase: 2.31528650443,
        frequency: 55798.45835839840,
    },
    Vsop87Term {
        amplitude: 0.00000000032,
        phase: 4.21446357042,
        frequency: 15664.03552270859,
    },
    Vsop87Term {
        amplitude: 0.00000000039,
        phase: 1.24979117796,
        frequency: 6418.14093002680,
    },
    Vsop87Term {
        amplitude: 0.00000000037,
        phase: 4.11943655770,
        frequency: 2787.04302385740,
    },
    Vsop87Term {
        amplitude: 0.00000000032,
        phase: 1.62887710890,
        frequency: 639.89728631400,
    },
    Vsop87Term {
        amplitude: 0.00000000038,
        phase: 5.89832942685,
        frequency: 640.87760738220,
    },
    Vsop87Term {
        amplitude: 0.00000000032,
        phase: 1.72442327688,
        frequency: 27433.88921587499,
    },
    Vsop87Term {
        amplitude: 0.00000000031,
        phase: 2.78828943753,
        frequency: 12139.55350910680,
    },
    Vsop87Term {
        amplitude: 0.00000000035,
        phase: 4.44608896525,
        frequency: 18202.21671665939,
    },
    Vsop87Term {
        amplitude: 0.00000000034,
        phase: 3.96287980676,
        frequency: 18216.44381066100,
    },
    Vsop87Term {
        amplitude: 0.00000000033,
        phase: 4.73611335874,
        frequency: 16723.35014259500,
    },
    Vsop87Term {
        amplitude: 0.00000000034,
        phase: 1.43910280005,
        frequency: 49515.38250840700,
    },
    Vsop87Term {
        amplitude: 0.00000000031,
        phase: 0.23302920161,
        frequency: 23581.25817731760,
    },
    Vsop87Term {
        amplitude: 0.00000000029,
        phase: 2.02633840220,
        frequency: 11609.86254401220,
    },
    Vsop87Term {
        amplitude: 0.00000000030,
        phase: 2.54923230240,
        frequency: 9924.81042151060,
    },
    Vsop87Term {
        amplitude: 0.00000000032,
        phase: 4.91793198558,
        frequency: 11300.58422135640,
    },
    Vsop87Term {
        amplitude: 0.00000000028,
        phase: 0.26187189577,
        frequency: 13521.75144159140,
    },
    Vsop87Term {
        amplitude: 0.00000000028,
        phase: 3.84568936822,
        frequency: 2699.73481931760,
    },
    Vsop87Term {
        amplitude: 0.00000000029,
        phase: 1.83149729794,
        frequency: 29822.78323632420,
    },
    Vsop87Term {
        amplitude: 0.00000000033,
        phase: 4.60320094415,
        frequency: 19004.64794940840,
    },
    Vsop87Term {
        amplitude: 0.00000000027,
        phase: 4.46183450287,
        frequency: 6702.56049386660,
    },
    Vsop87Term {
        amplitude: 0.00000000030,
        phase: 4.46494072240,
        frequency: 36147.40987730040,
    },
    Vsop87Term {
        amplitude: 0.00000000027,
        phase: 0.03211931363,
        frequency: 6279.78949257360,
    },
    Vsop87Term {
        amplitude: 0.00000000026,
        phase: 5.46497324333,
        frequency: 6245.04817735560,
    },
    Vsop87Term {
        amplitude: 0.00000000035,
        phase: 4.52695674113,
        frequency: 36949.23080842420,
    },
    Vsop87Term {
        amplitude: 0.00000000027,
        phase: 3.52528177609,
        frequency: 10770.89325626180,
    },
    Vsop87Term {
        amplitude: 0.00000000026,
        phase: 1.48499438453,
        frequency: 11080.17157891760,
    },
    Vsop87Term {
        amplitude: 0.00000000035,
        phase: 2.82154380962,
        frequency: 19402.79695281660,
    },
    Vsop87Term {
        amplitude: 0.00000000025,
        phase: 2.46339998836,
        frequency: 6279.48542133960,
    },
    Vsop87Term {
        amplitude: 0.00000000026,
        phase: 4.97688894643,
        frequency: 16737.57723659660,
    },
    Vsop87Term {
        amplitude: 0.00000000026,
        phase: 2.36136541526,
        frequency: 17996.03116822220,
    },
    Vsop87Term {
        amplitude: 0.00000000029,
        phase: 4.15148654061,
        frequency: 45892.73043315699,
    },
    Vsop87Term {
        amplitude: 0.00000000026,
        phase: 4.50714272714,
        frequency: 17796.95916678580,
    },
    Vsop87Term {
        amplitude: 0.00000000027,
        phase: 4.72625223674,
        frequency: 1066.49547719000,
    },
    Vsop87Term {
        amplitude: 0.00000000025,
        phase: 2.89309528854,
        frequency: 6286.66627864320,
    },
    Vsop87Term {
        amplitude: 0.00000000027,
        phase: 0.37462444357,
        frequency: 12964.30070339100,
    },
    Vsop87Term {
        amplitude: 0.00000000029,
        phase: 4.94860010533,
        frequency: 5863.59120611620,
    },
    Vsop87Term {
        amplitude: 0.00000000031,
        phase: 3.93096113577,
        frequency: 29864.33402730900,
    },
    Vsop87Term {
        amplitude: 0.00000000024,
        phase: 6.14987193584,
        frequency: 18606.49894600020,
    },
    Vsop87Term {
        amplitude: 0.00000000024,
        phase: 3.74225964547,
        frequency: 29026.48522950779,
    },
    Vsop87Term {
        amplitude: 0.00000000025,
        phase: 5.70460621565,
        frequency: 27707.54249429480,
    },
    Vsop87Term {
        amplitude: 0.00000000025,
        phase: 5.33928840652,
        frequency: 15141.39079431200,
    },
    Vsop87Term {
        amplitude: 0.00000000027,
        phase: 3.02320897140,
        frequency: 6286.36220740920,
    },
    Vsop87Term {
        amplitude: 0.00000000023,
        phase: 0.28364955406,
        frequency: 5327.47610838280,
    },
    Vsop87Term {
        amplitude: 0.00000000026,
        phase: 1.34240461687,
        frequency: 18875.52586977400,
    },
    Vsop87Term {
        amplitude: 0.00000000024,
        phase: 1.33998410121,
        frequency: 19800.94595622480,
    },
    Vsop87Term {
        amplitude: 0.00000000025,
        phase: 6.00172494004,
        frequency: 6489.26139842860,
    },
    Vsop87Term {
        amplitude: 0.00000000022,
        phase: 1.81777974484,
        frequency: 6288.59877429880,
    },
    Vsop87Term {
        amplitude: 0.00000000022,
        phase: 3.58603606640,
        frequency: 6915.85958930460,
    },
    Vsop87Term {
        amplitude: 0.00000000029,
        phase: 2.09564449439,
        frequency: 15265.88651930040,
    },
    Vsop87Term {
        amplitude: 0.00000000022,
        phase: 1.02173599251,
        frequency: 11925.27409260060,
    },
    Vsop87Term {
        amplitude: 0.00000000022,
        phase: 4.74660932338,
        frequency: 28230.18722269139,
    },
    Vsop87Term {
        amplitude: 0.00000000021,
        phase: 2.30688751432,
        frequency: 5999.21653112620,
    },
    Vsop87Term {
        amplitude: 0.00000000021,
        phase: 3.22654944430,
        frequency: 25934.12433108940,
    },
    Vsop87Term {
        amplitude: 0.00000000021,
        phase: 3.04956726238,
        frequency: 6566.93516885660,
    },
    Vsop87Term {
        amplitude: 0.00000000027,
        phase: 5.35653084499,
        frequency: 33794.54372352860,
    },
    Vsop87Term {
        amplitude: 0.00000000028,
        phase: 3.91168324815,
        frequency: 18208.34994259200,
    },
    Vsop87Term {
        amplitude: 0.00000000020,
        phase: 1.52296293311,
        frequency: 135.06508003540,
    },
    Vsop87Term {
        amplitude: 0.00000000022,
        phase: 4.66462839521,
        frequency: 13362.44970679920,
    },
    Vsop87Term {
        amplitude: 0.00000000019,
        phase: 1.78121167862,
        frequency: 156137.47598479928,
    },
    Vsop87Term {
        amplitude: 0.00000000019,
        phase: 2.99969102221,
        frequency: 19651.04848109800,
    },
    Vsop87Term {
        amplitude: 0.00000000019,
        phase: 2.86664273362,
        frequency: 18422.62935909819,
    },
    Vsop87Term {
        amplitude: 0.00000000025,
        phase: 0.94995632141,
        frequency: 31415.37924995700,
    },
    Vsop87Term {
        amplitude: 0.00000000019,
        phase: 4.71432851499,
        frequency: 77690.75950573849,
    },
    Vsop87Term {
        amplitude: 0.00000000019,
        phase: 2.54227398241,
        frequency: 77736.78343050249,
    },
    Vsop87Term {
        amplitude: 0.00000000020,
        phase: 5.91915117116,
        frequency: 48739.85989708300,
    },
];

pub(super) const R2: &[Vsop87Term] = &[
    Vsop87Term {
        amplitude: 0.00004359385,
        phase: 5.78455133738,
        frequency: 6283.07584999140,
    },
    Vsop87Term {
        amplitude: 0.00000123633,
        phase: 5.57934722157,
        frequency: 12566.15169998280,
    },
    Vsop87Term {
        amplitude: 0.00000012341,
        phase: 3.14159265359,
        frequency: 0.00000000000,
    },
    Vsop87Term {
        amplitude: 0.00000008792,
        phase: 3.62777733395,
        frequency: 77713.77146812050,
    },
    Vsop87Term {
        amplitude: 0.00000005689,
        phase: 1.86958905084,
        frequency: 5573.14280143310,
    },
    Vsop87Term {
        amplitude: 0.00000003301,
        phase: 5.47027913302,
        frequency: 18849.22754997420,
    },
    Vsop87Term {
        amplitude: 0.00000001471,
        phase: 4.48028885617,
        frequency: 5507.55323866740,
    },
    Vsop87Term {
        amplitude: 0.00000001013,
        phase: 2.81456417694,
        frequency: 5223.69391980220,
    },
    Vsop87Term {
        amplitude: 0.00000000854,
        phase: 3.10878241236,
        frequency: 1577.34354244780,
    },
    Vsop87Term {
        amplitude: 0.00000001102,
        phase: 2.84173992403,
        frequency: 161000.68573767410,
    },
    Vsop87Term {
        amplitude: 0.00000000648,
        phase: 5.47349498544,
        frequency: 775.52261132400,
    },
    Vsop87Term {
        amplitude: 0.00000000609,
        phase: 1.37969434104,
        frequency: 6438.49624942560,
    },
    Vsop87Term {
        amplitude: 0.00000000499,
        phase: 4.41649242250,
        frequency: 6286.59896834040,
    },
    Vsop87Term {
        amplitude: 0.00000000417,
        phase: 0.90242451175,
        frequency: 10977.07880469900,
    },
    Vsop87Term {
        amplitude: 0.00000000402,
        phase: 3.20376585290,
        frequency: 5088.62883976680,
    },
    Vsop87Term {
        amplitude: 0.00000000351,
        phase: 1.81079227770,
        frequency: 5486.77784317500,
    },
    Vsop87Term {
        amplitude: 0.00000000467,
        phase: 3.65753702738,
        frequency: 7084.89678111520,
    },
    Vsop87Term {
        amplitude: 0.00000000458,
        phase: 5.38585314743,
        frequency: 149854.40013480789,
    },
    Vsop87Term {
        amplitude: 0.00000000304,
        phase: 3.51701098693,
        frequency: 796.29800681640,
    },
    Vsop87Term {
        amplitude: 0.00000000266,
        phase: 6.17413982699,
        frequency: 6836.64525283380,
    },
    Vsop87Term {
        amplitude: 0.00000000279,
        phase: 1.84120501086,
        frequency: 4694.00295470760,
    },
    Vsop87Term {
        amplitude: 0.00000000260,
        phase: 1.41629543251,
        frequency: 2146.16541647520,
    },
    Vsop87Term {
        amplitude: 0.00000000266,
        phase: 3.13832905677,
        frequency: 71430.69561812909,
    },
    Vsop87Term {
        amplitude: 0.00000000321,
        phase: 5.35313367048,
        frequency: 3154.68708489560,
    },
    Vsop87Term {
        amplitude: 0.00000000238,
        phase: 2.17720020018,
        frequency: 155.42039943420,
    },
    Vsop87Term {
        amplitude: 0.00000000293,
        phase: 4.61501268144,
        frequency: 4690.47983635860,
    },
    Vsop87Term {
        amplitude: 0.00000000229,
        phase: 4.75969588070,
        frequency: 7234.79425624200,
    },
    Vsop87Term {
        amplitude: 0.00000000211,
        phase: 0.21868065485,
        frequency: 4705.73230754360,
    },
    Vsop87Term {
        amplitude: 0.00000000201,
        phase: 4.21905743357,
        frequency: 1349.86740965880,
    },
    Vsop87Term {
        amplitude: 0.00000000195,
        phase: 4.57808285364,
        frequency: 529.69096509460,
    },
    Vsop87Term {
        amplitude: 0.00000000253,
        phase: 2.81496293039,
        frequency: 1748.01641306700,
    },
    Vsop87Term {
        amplitude: 0.00000000182,
        phase: 5.70454011389,
        frequency: 6040.34724601740,
    },
    Vsop87Term {
        amplitude: 0.00000000179,
        phase: 6.02897097053,
        frequency: 4292.33083295040,
    },
    Vsop87Term {
        amplitude: 0.00000000186,
        phase: 1.58690991244,
        frequency: 6309.37416979120,
    },
    Vsop87Term {
        amplitude: 0.00000000170,
        phase: 2.90220009715,
        frequency: 9437.76293488700,
    },
    Vsop87Term {
        amplitude: 0.00000000166,
        phase: 1.99984925026,
        frequency: 8031.09226305840,
    },
    Vsop87Term {
        amplitude: 0.00000000158,
        phase: 0.04783713552,
        frequency: 2544.31441988340,
    },
    Vsop87Term {
        amplitude: 0.00000000197,
        phase: 2.01083639502,
        frequency: 1194.44701022460,
    },
    Vsop87Term {
        amplitude: 0.00000000165,
        phase: 5.78372596778,
        frequency: 83996.84731811189,
    },
    Vsop87Term {
        amplitude: 0.00000000214,
        phase: 3.38285934319,
        frequency: 7632.94325965020,
    },
    Vsop87Term {
        amplitude: 0.00000000140,
        phase: 0.36401486094,
        frequency: 10447.38783960440,
    },
    Vsop87Term {
        amplitude: 0.00000000151,
        phase: 0.95153163031,
        frequency: 6127.65545055720,
    },
    Vsop87Term {
        amplitude: 0.00000000136,
        phase: 1.48426306582,
        frequency: 2352.86615377180,
    },
    Vsop87Term {
        amplitude: 0.00000000127,
        phase: 5.48475435134,
        frequency: 951.71840625060,
    },
    Vsop87Term {
        amplitude: 0.00000000126,
        phase: 5.26866506592,
        frequency: 6279.55273164240,
    },
    Vsop87Term {
        amplitude: 0.00000000125,
        phase: 3.75754889288,
        frequency: 6812.76681508600,
    },
    Vsop87Term {
        amplitude: 0.00000000101,
        phase: 4.95015746147,
        frequency: 398.14900340820,
    },
    Vsop87Term {
        amplitude: 0.00000000102,
        phase: 0.68468295277,
        frequency: 1592.59601363280,
    },
    Vsop87Term {
        amplitude: 0.00000000100,
        phase: 1.14568935785,
        frequency: 3894.18182954220,
    },
    Vsop87Term {
        amplitude: 0.00000000129,
        phase: 0.76540016965,
        frequency: 553.56940284240,
    },
    Vsop87Term {
        amplitude: 0.00000000109,
        phase: 5.41063597567,
        frequency: 6256.77753019160,
    },
    Vsop87Term {
        amplitude: 0.00000000075,
        phase: 5.84804322893,
        frequency: 242.72860397400,
    },
    Vsop87Term {
        amplitude: 0.00000000095,
        phase: 1.94452244083,
        frequency: 11856.21865142450,
    },
    Vsop87Term {
        amplitude: 0.00000000077,
        phase: 0.69373708195,
        frequency: 8429.24126646660,
    },
    Vsop87Term {
        amplitude: 0.00000000100,
        phase: 5.19725292131,
        frequency: 244287.60000722769,
    },
    Vsop87Term {
        amplitude: 0.00000000080,
        phase: 6.18440483705,
        frequency: 1059.38193018920,
    },
    Vsop87Term {
        amplitude: 0.00000000069,
        phase: 5.25699888595,
        frequency: 14143.49524243060,
    },
    Vsop87Term {
        amplitude: 0.00000000085,
        phase: 5.39484725499,
        frequency: 25132.30339996560,
    },
    Vsop87Term {
        amplitude: 0.00000000066,
        phase: 0.51779993906,
        frequency: 801.82093112380,
    },
    Vsop87Term {
        amplitude: 0.00000000055,
        phase: 5.16878202461,
        frequency: 7058.59846131540,
    },
    Vsop87Term {
        amplitude: 0.00000000051,
        phase: 3.88759155247,
        frequency: 12036.46073488820,
    },
    Vsop87Term {
        amplitude: 0.00000000050,
        phase: 5.57636570536,
        frequency: 6290.18939699220,
    },
    Vsop87Term {
        amplitude: 0.00000000061,
        phase: 2.24359003264,
        frequency: 8635.94200376320,
    },
    Vsop87Term {
        amplitude: 0.00000000050,
        phase: 5.54441900966,
        frequency: 1990.74501704100,
    },
    Vsop87Term {
        amplitude: 0.00000000056,
        phase: 4.00301078040,
        frequency: 13367.97263110660,
    },
    Vsop87Term {
        amplitude: 0.00000000052,
        phase: 4.13138898038,
        frequency: 7860.41939243920,
    },
    Vsop87Term {
        amplitude: 0.00000000052,
        phase: 3.90943054011,
        frequency: 26.29831979980,
    },
    Vsop87Term {
        amplitude: 0.00000000041,
        phase: 3.57128482780,
        frequency: 7079.37385680780,
    },
    Vsop87Term {
        amplitude: 0.00000000056,
        phase: 2.76959005761,
        frequency: 90955.55169449610,
    },
    Vsop87Term {
        amplitude: 0.00000000042,
        phase: 1.91461189199,
        frequency: 7477.52286021600,
    },
    Vsop87Term {
        amplitude: 0.00000000042,
        phase: 0.42728171713,
        frequency: 10213.28554621100,
    },
    Vsop87Term {
        amplitude: 0.00000000042,
        phase: 1.09413724455,
        frequency: 709.93304855830,
    },
    Vsop87Term {
        amplitude: 0.00000000039,
        phase: 3.93298068961,
        frequency: 10973.55568635000,
    },
    Vsop87Term {
        amplitude: 0.00000000038,
        phase: 6.17935925345,
        frequency: 9917.69687450980,
    },
    Vsop87Term {
        amplitude: 0.00000000049,
        phase: 0.83021145241,
        frequency: 11506.76976979360,
    },
    Vsop87Term {
        amplitude: 0.00000000053,
        phase: 1.45828359397,
        frequency: 233141.31440436149,
    },
    Vsop87Term {
        amplitude: 0.00000000047,
        phase: 6.21568666789,
        frequency: 6681.22485339960,
    },
    Vsop87Term {
        amplitude: 0.00000000037,
        phase: 0.36359309980,
        frequency: 10177.25767953360,
    },
    Vsop87Term {
        amplitude: 0.00000000035,
        phase: 3.33024911524,
        frequency: 5643.17856367740,
    },
    Vsop87Term {
        amplitude: 0.00000000034,
        phase: 5.63446915337,
        frequency: 6525.80445396540,
    },
    Vsop87Term {
        amplitude: 0.00000000035,
        phase: 5.36033855038,
        frequency: 25158.60171976540,
    },
    Vsop87Term {
        amplitude: 0.00000000034,
        phase: 5.36319798321,
        frequency: 4933.20844033260,
    },
    Vsop87Term {
        amplitude: 0.00000000033,
        phase: 4.24722336872,
        frequency: 12569.67481833180,
    },
    Vsop87Term {
        amplitude: 0.00000000043,
        phase: 5.26370903404,
        frequency: 10575.40668294180,
    },
    Vsop87Term {
        amplitude: 0.00000000042,
        phase: 5.08837645072,
        frequency: 11015.10647733480,
    },
    Vsop87Term {
        amplitude: 0.00000000040,
        phase: 1.98334703186,
        frequency: 6284.05617105960,
    },
    Vsop87Term {
        amplitude: 0.00000000042,
        phase: 4.22496037505,
        frequency: 88860.05707098669,
    },
    Vsop87Term {
        amplitude: 0.00000000029,
        phase: 3.19088628170,
        frequency: 11926.25441366880,
    },
    Vsop87Term {
        amplitude: 0.00000000029,
        phase: 0.15217616684,
        frequency: 12168.00269657460,
    },
    Vsop87Term {
        amplitude: 0.00000000030,
        phase: 1.61904744136,
        frequency: 9779.10867612540,
    },
    Vsop87Term {
        amplitude: 0.00000000027,
        phase: 0.76388991416,
        frequency: 1589.07289528380,
    },
    Vsop87Term {
        amplitude: 0.00000000036,
        phase: 2.74712003443,
        frequency: 3738.76143010800,
    },
    Vsop87Term {
        amplitude: 0.00000000033,
        phase: 3.08807829566,
        frequency: 3930.20969621960,
    },
    Vsop87Term {
        amplitude: 0.00000000031,
        phase: 5.34906619513,
        frequency: 143571.32428481648,
    },
    Vsop87Term {
        amplitude: 0.00000000025,
        phase: 0.10240267494,
        frequency: 22483.84857449259,
    },
    Vsop87Term {
        amplitude: 0.00000000030,
        phase: 3.47110495524,
        frequency: 14945.31617355440,
    },
    Vsop87Term {
        amplitude: 0.00000000024,
        phase: 1.10425016019,
        frequency: 4535.05943692440,
    },
    Vsop87Term {
        amplitude: 0.00000000024,
        phase: 1.58037259780,
        frequency: 6496.37494542940,
    },
    Vsop87Term {
        amplitude: 0.00000000023,
        phase: 3.87710321433,
        frequency: 6275.96230299060,
    },
    Vsop87Term {
        amplitude: 0.00000000025,
        phase: 3.94529778970,
        frequency: 3128.38876509580,
    },
    Vsop87Term {
        amplitude: 0.00000000023,
        phase: 3.44685609601,
        frequency: 4136.91043351620,
    },
    Vsop87Term {
        amplitude: 0.00000000023,
        phase: 3.83156029849,
        frequency: 5753.38488489680,
    },
    Vsop87Term {
        amplitude: 0.00000000022,
        phase: 1.86956128067,
        frequency: 16730.46368959580,
    },
    Vsop87Term {
        amplitude: 0.00000000025,
        phase: 2.42188933855,
        frequency: 5729.50644714900,
    },
    Vsop87Term {
        amplitude: 0.00000000020,
        phase: 1.78208352927,
        frequency: 17789.84561978500,
    },
    Vsop87Term {
        amplitude: 0.00000000021,
        phase: 4.30363087400,
        frequency: 16858.48253293320,
    },
    Vsop87Term {
        amplitude: 0.00000000021,
        phase: 0.49258939822,
        frequency: 29088.81141598500,
    },
    Vsop87Term {
        amplitude: 0.00000000025,
        phase: 1.33030250444,
        frequency: 6282.09552892320,
    },
    Vsop87Term {
        amplitude: 0.00000000027,
        phase: 2.54785812264,
        frequency: 3496.03282613400,
    },
    Vsop87Term {
        amplitude: 0.00000000022,
        phase: 1.11232521950,
        frequency: 12721.57209941700,
    },
    Vsop87Term {
        amplitude: 0.00000000021,
        phase: 5.97759081637,
        frequency: 7.11354700080,
    },
    Vsop87Term {
        amplitude: 0.00000000019,
        phase: 0.80292033311,
        frequency: 16062.18452611680,
    },
    Vsop87Term {
        amplitude: 0.00000000023,
        phase: 4.12454848769,
        frequency: 2388.89402044920,
    },
    Vsop87Term {
        amplitude: 0.00000000022,
        phase: 4.92663152168,
        frequency: 18875.52586977400,
    },
    Vsop87Term {
        amplitude: 0.00000000023,
        phase: 5.68902059771,
        frequency: 16460.33352952499,
    },
    Vsop87Term {
        amplitude: 0.00000000023,
        phase: 4.97346265647,
        frequency: 17260.15465469040,
    },
    Vsop87Term {
        amplitude: 0.00000000023,
        phase: 3.03021283729,
        frequency: 66567.48586525429,
    },
    Vsop87Term {
        amplitude: 0.00000000016,
        phase: 3.89740925257,
        frequency: 5331.35744374080,
    },
    Vsop87Term {
        amplitude: 0.00000000017,
        phase: 3.08268671348,
        frequency: 154717.60988768269,
    },
    Vsop87Term {
        amplitude: 0.00000000016,
        phase: 3.95085099736,
        frequency: 3097.88382272579,
    },
    Vsop87Term {
        amplitude: 0.00000000016,
        phase: 3.99041783945,
        frequency: 6283.14316029419,
    },
    Vsop87Term {
        amplitude: 0.00000000020,
        phase: 6.10644140189,
        frequency: 167283.76158766549,
    },
    Vsop87Term {
        amplitude: 0.00000000015,
        phase: 4.09775914607,
        frequency: 11712.95531823080,
    },
    Vsop87Term {
        amplitude: 0.00000000016,
        phase: 5.71769940700,
        frequency: 17298.18232732620,
    },
    Vsop87Term {
        amplitude: 0.00000000016,
        phase: 3.28894009404,
        frequency: 5884.92684658320,
    },
    Vsop87Term {
        amplitude: 0.00000000015,
        phase: 5.64785377164,
        frequency: 12559.03815298200,
    },
    Vsop87Term {
        amplitude: 0.00000000016,
        phase: 4.43452080930,
        frequency: 6283.00853968860,
    },
    Vsop87Term {
        amplitude: 0.00000000014,
        phase: 2.31721603062,
        frequency: 5481.25491886760,
    },
    Vsop87Term {
        amplitude: 0.00000000014,
        phase: 4.43479032305,
        frequency: 13517.87010623340,
    },
    Vsop87Term {
        amplitude: 0.00000000014,
        phase: 4.73209312936,
        frequency: 7342.45778018060,
    },
    Vsop87Term {
        amplitude: 0.00000000012,
        phase: 0.64705975463,
        frequency: 18073.70493865020,
    },
    Vsop87Term {
        amplitude: 0.00000000011,
        phase: 1.51443332200,
        frequency: 16200.77272450120,
    },
    Vsop87Term {
        amplitude: 0.00000000011,
        phase: 0.88708889185,
        frequency: 21228.39202354580,
    },
    Vsop87Term {
        amplitude: 0.00000000014,
        phase: 4.50116508534,
        frequency: 640.87760738220,
    },
    Vsop87Term {
        amplitude: 0.00000000011,
        phase: 4.64339996198,
        frequency: 11790.62908865880,
    },
    Vsop87Term {
        amplitude: 0.00000000011,
        phase: 1.31064298246,
        frequency: 4164.31198961300,
    },
    Vsop87Term {
        amplitude: 0.00000000009,
        phase: 3.02238989305,
        frequency: 23543.23050468179,
    },
    Vsop87Term {
        amplitude: 0.00000000009,
        phase: 2.04999402381,
        frequency: 22003.91463486980,
    },
    Vsop87Term {
        amplitude: 0.00000000009,
        phase: 4.91488110218,
        frequency: 213.29909543800,
    },
];

pub(super) const R3: &[Vsop87Term] = &[
    Vsop87Term {
        amplitude: 0.00000144595,
        phase: 4.27319435148,
        frequency: 6283.07584999140,
    },
    Vsop87Term {
        amplitude: 0.00000006729,
        phase: 3.91697608662,
        frequency: 12566.15169998280,
    },
    Vsop87Term {
        amplitude: 0.00000000774,
        phase: 0.00000000000,
        frequency: 0.00000000000,
    },
    Vsop87Term {
        amplitude: 0.00000000247,
        phase: 3.73019298781,
        frequency: 18849.22754997420,
    },
    Vsop87Term {
        amplitude: 0.00000000036,
        phase: 2.80081409050,
        frequency: 6286.59896834040,
    },
    Vsop87Term {
        amplitude: 0.00000000033,
        phase: 5.62216602775,
        frequency: 6127.65545055720,
    },
    Vsop87Term {
        amplitude: 0.00000000019,
        phase: 3.71292621802,
        frequency: 6438.49624942560,
    },
    Vsop87Term {
        amplitude: 0.00000000016,
        phase: 4.26011484232,
        frequency: 6525.80445396540,
    },
    Vsop87Term {
        amplitude: 0.00000000016,
        phase: 3.50416887054,
        frequency: 6256.77753019160,
    },
    Vsop87Term {
        amplitude: 0.00000000014,
        phase: 3.62127621114,
        frequency: 25132.30339996560,
    },
    Vsop87Term {
        amplitude: 0.00000000011,
        phase: 4.39200958819,
        frequency: 4705.73230754360,
    },
    Vsop87Term {
        amplitude: 0.00000000011,
        phase: 5.22327127059,
        frequency: 6040.34724601740,
    },
    Vsop87Term {
        amplitude: 0.00000000010,
        phase: 4.28045254647,
        frequency: 83996.84731811189,
    },
    Vsop87Term {
        amplitude: 0.00000000009,
        phase: 1.56864096494,
        frequency: 5507.55323866740,
    },
    Vsop87Term {
        amplitude: 0.00000000011,
        phase: 1.37795688024,
        frequency: 6309.37416979120,
    },
    Vsop87Term {
        amplitude: 0.00000000010,
        phase: 5.19937959068,
        frequency: 71430.69561812909,
    },
    Vsop87Term {
        amplitude: 0.00000000009,
        phase: 0.47275199930,
        frequency: 6279.55273164240,
    },
    Vsop87Term {
        amplitude: 0.00000000009,
        phase: 0.74642756529,
        frequency: 5729.50644714900,
    },
    Vsop87Term {
        amplitude: 0.00000000007,
        phase: 2.97374891560,
        frequency: 775.52261132400,
    },
    Vsop87Term {
        amplitude: 0.00000000007,
        phase: 3.28615691021,
        frequency: 7058.59846131540,
    },
    Vsop87Term {
        amplitude: 0.00000000007,
        phase: 2.19184402142,
        frequency: 6812.76681508600,
    },
    Vsop87Term {
        amplitude: 0.00000000005,
        phase: 3.15419034438,
        frequency: 529.69096509460,
    },
    Vsop87Term {
        amplitude: 0.00000000006,
        phase: 4.54725567047,
        frequency: 1059.38193018920,
    },
    Vsop87Term {
        amplitude: 0.00000000005,
        phase: 1.51104406936,
        frequency: 7079.37385680780,
    },
    Vsop87Term {
        amplitude: 0.00000000007,
        phase: 2.98052059053,
        frequency: 6681.22485339960,
    },
    Vsop87Term {
        amplitude: 0.00000000005,
        phase: 2.30961231391,
        frequency: 12036.46073488820,
    },
    Vsop87Term {
        amplitude: 0.00000000005,
        phase: 3.71102966917,
        frequency: 6290.18939699220,
    },
];

pub(super) const R4: &[Vsop87Term] = &[
    Vsop87Term {
        amplitude: 0.00000003858,
        phase: 2.56384387339,
        frequency: 6283.07584999140,
    },
    Vsop87Term {
        amplitude: 0.00000000306,
        phase: 2.26769501230,
        frequency: 12566.15169998280,
    },
    Vsop87Term {
        amplitude: 0.00000000053,
        phase: 3.44031471924,
        frequency: 5573.14280143310,
    },
    Vsop87Term {
        amplitude: 0.00000000015,
        phase: 2.04794573436,
        frequency: 18849.22754997420,
    },
    Vsop87Term {
        amplitude: 0.00000000013,
        phase: 2.05688873673,
        frequency: 77713.77146812050,
    },
    Vsop87Term {
        amplitude: 0.00000000007,
        phase: 4.41218854480,
        frequency: 161000.68573767410,
    },
    Vsop87Term {
        amplitude: 0.00000000005,
        phase: 5.26154653107,
        frequency: 6438.49624942560,
    },
    Vsop87Term {
        amplitude: 0.00000000005,
        phase: 4.07695126049,
        frequency: 6127.65545055720,
    },
    Vsop87Term {
        amplitude: 0.00000000006,
        phase: 3.81514213664,
        frequency: 149854.40013480789,
    },
    Vsop87Term {
        amplitude: 0.00000000003,
        phase: 1.28175749811,
        frequency: 6286.59896834040,
    },
];

pub(super) const R5: &[Vsop87Term] = &[
    Vsop87Term {
        amplitude: 0.00000000086,
        phase: 1.21579741687,
        frequency: 6283.07584999140,
    },
    Vsop87Term {
        amplitude: 0.00000000012,
        phase: 0.65617264033,
        frequency: 12566.15169998280,
    },
    Vsop87Term {
        amplitude: 0.00000000001,
        phase: 0.38068797142,
        frequency: 18849.22754997420,
    },
];

pub(super) const RADIUS_SERIES: [&[Vsop87Term]; 6] = [R0, R1, R2, R3, R4, R5];

//! VSOP87D Earth heliocentric longitude coefficients.
//!
//! Generated from the official IMCCE file `VSOP87D.ear`.
//! Only variable 1 (heliocentric longitude, L) is included here.
//! The tuple fields are `(A, B, C)` for `A * cos(B + C * tau)`.
//! Coefficients are copied with the same decimal spelling used by IMCCE.

#![allow(clippy::approx_constant, clippy::excessive_precision)]

#[derive(Debug, Clone, Copy)]
pub(super) struct Vsop87Term {
    pub(super) amplitude: f64,
    pub(super) phase: f64,
    pub(super) frequency: f64,
}

pub(super) const L0: &[Vsop87Term] = &[
    Vsop87Term {
        amplitude: 1.75347045673,
        phase: 0.00000000000,
        frequency: 0.00000000000,
    },
    Vsop87Term {
        amplitude: 0.03341656456,
        phase: 4.66925680417,
        frequency: 6283.07584999140,
    },
    Vsop87Term {
        amplitude: 0.00034894275,
        phase: 4.62610241759,
        frequency: 12566.15169998280,
    },
    Vsop87Term {
        amplitude: 0.00003417571,
        phase: 2.82886579606,
        frequency: 3.52311834900,
    },
    Vsop87Term {
        amplitude: 0.00003497056,
        phase: 2.74411800971,
        frequency: 5753.38488489680,
    },
    Vsop87Term {
        amplitude: 0.00003135896,
        phase: 3.62767041758,
        frequency: 77713.77146812050,
    },
    Vsop87Term {
        amplitude: 0.00002676218,
        phase: 4.41808351397,
        frequency: 7860.41939243920,
    },
    Vsop87Term {
        amplitude: 0.00002342687,
        phase: 6.13516237631,
        frequency: 3930.20969621960,
    },
    Vsop87Term {
        amplitude: 0.00001273166,
        phase: 2.03709655772,
        frequency: 529.69096509460,
    },
    Vsop87Term {
        amplitude: 0.00001324292,
        phase: 0.74246356352,
        frequency: 11506.76976979360,
    },
    Vsop87Term {
        amplitude: 0.00000901855,
        phase: 2.04505443513,
        frequency: 26.29831979980,
    },
    Vsop87Term {
        amplitude: 0.00001199167,
        phase: 1.10962944315,
        frequency: 1577.34354244780,
    },
    Vsop87Term {
        amplitude: 0.00000857223,
        phase: 3.50849156957,
        frequency: 398.14900340820,
    },
    Vsop87Term {
        amplitude: 0.00000779786,
        phase: 1.17882652114,
        frequency: 5223.69391980220,
    },
    Vsop87Term {
        amplitude: 0.00000990250,
        phase: 5.23268129594,
        frequency: 5884.92684658320,
    },
    Vsop87Term {
        amplitude: 0.00000753141,
        phase: 2.53339053818,
        frequency: 5507.55323866740,
    },
    Vsop87Term {
        amplitude: 0.00000505264,
        phase: 4.58292563052,
        frequency: 18849.22754997420,
    },
    Vsop87Term {
        amplitude: 0.00000492379,
        phase: 4.20506639861,
        frequency: 775.52261132400,
    },
    Vsop87Term {
        amplitude: 0.00000356655,
        phase: 2.91954116867,
        frequency: 0.06731030280,
    },
    Vsop87Term {
        amplitude: 0.00000284125,
        phase: 1.89869034186,
        frequency: 796.29800681640,
    },
    Vsop87Term {
        amplitude: 0.00000242810,
        phase: 0.34481140906,
        frequency: 5486.77784317500,
    },
    Vsop87Term {
        amplitude: 0.00000317087,
        phase: 5.84901952218,
        frequency: 11790.62908865880,
    },
    Vsop87Term {
        amplitude: 0.00000271039,
        phase: 0.31488607649,
        frequency: 10977.07880469900,
    },
    Vsop87Term {
        amplitude: 0.00000206160,
        phase: 4.80646606059,
        frequency: 2544.31441988340,
    },
    Vsop87Term {
        amplitude: 0.00000205385,
        phase: 1.86947813692,
        frequency: 5573.14280143310,
    },
    Vsop87Term {
        amplitude: 0.00000202261,
        phase: 2.45767795458,
        frequency: 6069.77675455340,
    },
    Vsop87Term {
        amplitude: 0.00000126184,
        phase: 1.08302630210,
        frequency: 20.77539549240,
    },
    Vsop87Term {
        amplitude: 0.00000155516,
        phase: 0.83306073807,
        frequency: 213.29909543800,
    },
    Vsop87Term {
        amplitude: 0.00000115132,
        phase: 0.64544911683,
        frequency: 0.98032106820,
    },
    Vsop87Term {
        amplitude: 0.00000102851,
        phase: 0.63599846727,
        frequency: 4694.00295470760,
    },
    Vsop87Term {
        amplitude: 0.00000101724,
        phase: 4.26679821365,
        frequency: 7.11354700080,
    },
    Vsop87Term {
        amplitude: 0.00000099206,
        phase: 6.20992940258,
        frequency: 2146.16541647520,
    },
    Vsop87Term {
        amplitude: 0.00000132212,
        phase: 3.41118275555,
        frequency: 2942.46342329160,
    },
    Vsop87Term {
        amplitude: 0.00000097607,
        phase: 0.68101272270,
        frequency: 155.42039943420,
    },
    Vsop87Term {
        amplitude: 0.00000085128,
        phase: 1.29870743025,
        frequency: 6275.96230299060,
    },
    Vsop87Term {
        amplitude: 0.00000074651,
        phase: 1.75508916159,
        frequency: 5088.62883976680,
    },
    Vsop87Term {
        amplitude: 0.00000101895,
        phase: 0.97569221824,
        frequency: 15720.83878487840,
    },
    Vsop87Term {
        amplitude: 0.00000084711,
        phase: 3.67080093025,
        frequency: 71430.69561812909,
    },
    Vsop87Term {
        amplitude: 0.00000073547,
        phase: 4.67926565481,
        frequency: 801.82093112380,
    },
    Vsop87Term {
        amplitude: 0.00000073874,
        phase: 3.50319443167,
        frequency: 3154.68708489560,
    },
    Vsop87Term {
        amplitude: 0.00000078756,
        phase: 3.03698313141,
        frequency: 12036.46073488820,
    },
    Vsop87Term {
        amplitude: 0.00000079637,
        phase: 1.80791330700,
        frequency: 17260.15465469040,
    },
    Vsop87Term {
        amplitude: 0.00000085803,
        phase: 5.98322631256,
        frequency: 161000.68573767410,
    },
    Vsop87Term {
        amplitude: 0.00000056963,
        phase: 2.78430398043,
        frequency: 6286.59896834040,
    },
    Vsop87Term {
        amplitude: 0.00000061148,
        phase: 1.81839811024,
        frequency: 7084.89678111520,
    },
    Vsop87Term {
        amplitude: 0.00000069627,
        phase: 0.83297596966,
        frequency: 9437.76293488700,
    },
    Vsop87Term {
        amplitude: 0.00000056116,
        phase: 4.38694880779,
        frequency: 14143.49524243060,
    },
    Vsop87Term {
        amplitude: 0.00000062449,
        phase: 3.97763880587,
        frequency: 8827.39026987480,
    },
    Vsop87Term {
        amplitude: 0.00000051145,
        phase: 0.28306864501,
        frequency: 5856.47765911540,
    },
    Vsop87Term {
        amplitude: 0.00000055577,
        phase: 3.47006009062,
        frequency: 6279.55273164240,
    },
    Vsop87Term {
        amplitude: 0.00000041036,
        phase: 5.36817351402,
        frequency: 8429.24126646660,
    },
    Vsop87Term {
        amplitude: 0.00000051605,
        phase: 1.33282746983,
        frequency: 1748.01641306700,
    },
    Vsop87Term {
        amplitude: 0.00000051992,
        phase: 0.18914945834,
        frequency: 12139.55350910680,
    },
    Vsop87Term {
        amplitude: 0.00000049000,
        phase: 0.48735065033,
        frequency: 1194.44701022460,
    },
    Vsop87Term {
        amplitude: 0.00000039200,
        phase: 6.16832995016,
        frequency: 10447.38783960440,
    },
    Vsop87Term {
        amplitude: 0.00000035566,
        phase: 1.77597314691,
        frequency: 6812.76681508600,
    },
    Vsop87Term {
        amplitude: 0.00000036770,
        phase: 6.04133859347,
        frequency: 10213.28554621100,
    },
    Vsop87Term {
        amplitude: 0.00000036596,
        phase: 2.56955238628,
        frequency: 1059.38193018920,
    },
    Vsop87Term {
        amplitude: 0.00000033291,
        phase: 0.59309499459,
        frequency: 17789.84561978500,
    },
    Vsop87Term {
        amplitude: 0.00000035954,
        phase: 1.70876111898,
        frequency: 2352.86615377180,
    },
    Vsop87Term {
        amplitude: 0.00000040938,
        phase: 2.39850881707,
        frequency: 19651.04848109800,
    },
    Vsop87Term {
        amplitude: 0.00000030047,
        phase: 2.73975123935,
        frequency: 1349.86740965880,
    },
    Vsop87Term {
        amplitude: 0.00000030412,
        phase: 0.44294464135,
        frequency: 83996.84731811189,
    },
    Vsop87Term {
        amplitude: 0.00000023663,
        phase: 0.48473567763,
        frequency: 8031.09226305840,
    },
    Vsop87Term {
        amplitude: 0.00000023574,
        phase: 2.06527720049,
        frequency: 3340.61242669980,
    },
    Vsop87Term {
        amplitude: 0.00000021089,
        phase: 4.14825464101,
        frequency: 951.71840625060,
    },
    Vsop87Term {
        amplitude: 0.00000024738,
        phase: 0.21484762138,
        frequency: 3.59042865180,
    },
    Vsop87Term {
        amplitude: 0.00000025352,
        phase: 3.16470953405,
        frequency: 4690.47983635860,
    },
    Vsop87Term {
        amplitude: 0.00000022820,
        phase: 5.22197888032,
        frequency: 4705.73230754360,
    },
    Vsop87Term {
        amplitude: 0.00000021419,
        phase: 1.42563735525,
        frequency: 16730.46368959580,
    },
    Vsop87Term {
        amplitude: 0.00000021891,
        phase: 5.55594302562,
        frequency: 553.56940284240,
    },
    Vsop87Term {
        amplitude: 0.00000017481,
        phase: 4.56052900359,
        frequency: 135.06508003540,
    },
    Vsop87Term {
        amplitude: 0.00000019925,
        phase: 5.22208471269,
        frequency: 12168.00269657460,
    },
    Vsop87Term {
        amplitude: 0.00000019860,
        phase: 5.77470167653,
        frequency: 6309.37416979120,
    },
    Vsop87Term {
        amplitude: 0.00000020300,
        phase: 0.37133792946,
        frequency: 283.85931886520,
    },
    Vsop87Term {
        amplitude: 0.00000014421,
        phase: 4.19315332546,
        frequency: 242.72860397400,
    },
    Vsop87Term {
        amplitude: 0.00000016225,
        phase: 5.98837722564,
        frequency: 11769.85369316640,
    },
    Vsop87Term {
        amplitude: 0.00000015077,
        phase: 4.19567181073,
        frequency: 6256.77753019160,
    },
    Vsop87Term {
        amplitude: 0.00000019124,
        phase: 3.82219996949,
        frequency: 23581.25817731760,
    },
    Vsop87Term {
        amplitude: 0.00000018888,
        phase: 5.38626880969,
        frequency: 149854.40013480789,
    },
    Vsop87Term {
        amplitude: 0.00000014346,
        phase: 3.72355084422,
        frequency: 38.02767263580,
    },
    Vsop87Term {
        amplitude: 0.00000017898,
        phase: 2.21490735647,
        frequency: 13367.97263110660,
    },
    Vsop87Term {
        amplitude: 0.00000012054,
        phase: 2.62229588349,
        frequency: 955.59974160860,
    },
    Vsop87Term {
        amplitude: 0.00000011287,
        phase: 0.17739328092,
        frequency: 4164.31198961300,
    },
    Vsop87Term {
        amplitude: 0.00000013971,
        phase: 4.40138139996,
        frequency: 6681.22485339960,
    },
    Vsop87Term {
        amplitude: 0.00000013621,
        phase: 1.88934471407,
        frequency: 7632.94325965020,
    },
    Vsop87Term {
        amplitude: 0.00000012503,
        phase: 1.13052412208,
        frequency: 5.52292430740,
    },
    Vsop87Term {
        amplitude: 0.00000010498,
        phase: 5.35909518669,
        frequency: 1592.59601363280,
    },
    Vsop87Term {
        amplitude: 0.00000009803,
        phase: 0.99947478995,
        frequency: 11371.70468975820,
    },
    Vsop87Term {
        amplitude: 0.00000009220,
        phase: 4.57138609781,
        frequency: 4292.33083295040,
    },
    Vsop87Term {
        amplitude: 0.00000010327,
        phase: 6.19982566125,
        frequency: 6438.49624942560,
    },
    Vsop87Term {
        amplitude: 0.00000012003,
        phase: 1.00351456700,
        frequency: 632.78373931320,
    },
    Vsop87Term {
        amplitude: 0.00000010827,
        phase: 0.32734520222,
        frequency: 103.09277421860,
    },
    Vsop87Term {
        amplitude: 0.00000008356,
        phase: 4.53902685948,
        frequency: 25132.30339996560,
    },
    Vsop87Term {
        amplitude: 0.00000010005,
        phase: 6.02914963280,
        frequency: 5746.27133789600,
    },
    Vsop87Term {
        amplitude: 0.00000008409,
        phase: 3.29946744189,
        frequency: 7234.79425624200,
    },
    Vsop87Term {
        amplitude: 0.00000008006,
        phase: 5.82145271907,
        frequency: 28.44918746780,
    },
    Vsop87Term {
        amplitude: 0.00000010523,
        phase: 0.93871805506,
        frequency: 11926.25441366880,
    },
    Vsop87Term {
        amplitude: 0.00000007686,
        phase: 3.12142363172,
        frequency: 7238.67559160000,
    },
    Vsop87Term {
        amplitude: 0.00000009378,
        phase: 2.62414241032,
        frequency: 5760.49843189760,
    },
    Vsop87Term {
        amplitude: 0.00000008127,
        phase: 6.11228001785,
        frequency: 4732.03062734340,
    },
    Vsop87Term {
        amplitude: 0.00000009232,
        phase: 0.48343968736,
        frequency: 522.57741809380,
    },
    Vsop87Term {
        amplitude: 0.00000009802,
        phase: 5.24413991147,
        frequency: 27511.46787353720,
    },
    Vsop87Term {
        amplitude: 0.00000007871,
        phase: 0.99590177926,
        frequency: 5643.17856367740,
    },
    Vsop87Term {
        amplitude: 0.00000008123,
        phase: 6.27053013650,
        frequency: 426.59819087600,
    },
    Vsop87Term {
        amplitude: 0.00000009048,
        phase: 5.33686335897,
        frequency: 6386.16862421000,
    },
    Vsop87Term {
        amplitude: 0.00000008620,
        phase: 4.16538210888,
        frequency: 7058.59846131540,
    },
    Vsop87Term {
        amplitude: 0.00000006297,
        phase: 4.71724819317,
        frequency: 6836.64525283380,
    },
    Vsop87Term {
        amplitude: 0.00000007575,
        phase: 3.97382858911,
        frequency: 11499.65622279280,
    },
    Vsop87Term {
        amplitude: 0.00000007756,
        phase: 2.95729056763,
        frequency: 23013.53953958720,
    },
    Vsop87Term {
        amplitude: 0.00000007314,
        phase: 0.60652505806,
        frequency: 11513.88331679440,
    },
    Vsop87Term {
        amplitude: 0.00000005955,
        phase: 2.87641047971,
        frequency: 6283.14316029419,
    },
    Vsop87Term {
        amplitude: 0.00000006534,
        phase: 5.79072926033,
        frequency: 18073.70493865020,
    },
    Vsop87Term {
        amplitude: 0.00000007188,
        phase: 3.99831508699,
        frequency: 74.78159856730,
    },
    Vsop87Term {
        amplitude: 0.00000007346,
        phase: 4.38582365437,
        frequency: 316.39186965660,
    },
    Vsop87Term {
        amplitude: 0.00000005413,
        phase: 5.39199024641,
        frequency: 419.48464387520,
    },
    Vsop87Term {
        amplitude: 0.00000005127,
        phase: 2.36062848786,
        frequency: 10973.55568635000,
    },
    Vsop87Term {
        amplitude: 0.00000007056,
        phase: 0.32258441903,
        frequency: 263.08392337280,
    },
    Vsop87Term {
        amplitude: 0.00000006625,
        phase: 3.66475158672,
        frequency: 17298.18232732620,
    },
    Vsop87Term {
        amplitude: 0.00000006762,
        phase: 5.91132535899,
        frequency: 90955.55169449610,
    },
    Vsop87Term {
        amplitude: 0.00000004938,
        phase: 5.73672165674,
        frequency: 9917.69687450980,
    },
    Vsop87Term {
        amplitude: 0.00000005547,
        phase: 2.45152597661,
        frequency: 12352.85260454480,
    },
    Vsop87Term {
        amplitude: 0.00000005958,
        phase: 3.32051344676,
        frequency: 6283.00853968860,
    },
    Vsop87Term {
        amplitude: 0.00000004471,
        phase: 2.06385999536,
        frequency: 7079.37385680780,
    },
    Vsop87Term {
        amplitude: 0.00000006153,
        phase: 1.45823331144,
        frequency: 233141.31440436149,
    },
    Vsop87Term {
        amplitude: 0.00000004348,
        phase: 4.42342175480,
        frequency: 5216.58037280140,
    },
    Vsop87Term {
        amplitude: 0.00000006123,
        phase: 1.07494905258,
        frequency: 19804.82729158280,
    },
    Vsop87Term {
        amplitude: 0.00000004488,
        phase: 3.65285037150,
        frequency: 206.18554843720,
    },
    Vsop87Term {
        amplitude: 0.00000004020,
        phase: 0.83995823171,
        frequency: 20.35531939880,
    },
    Vsop87Term {
        amplitude: 0.00000005188,
        phase: 4.06503864016,
        frequency: 6208.29425142410,
    },
    Vsop87Term {
        amplitude: 0.00000005307,
        phase: 0.38217636096,
        frequency: 31441.67756975680,
    },
    Vsop87Term {
        amplitude: 0.00000003785,
        phase: 2.34369213733,
        frequency: 3.88133535800,
    },
    Vsop87Term {
        amplitude: 0.00000004497,
        phase: 3.27230796845,
        frequency: 11015.10647733480,
    },
    Vsop87Term {
        amplitude: 0.00000004132,
        phase: 0.92128915753,
        frequency: 3738.76143010800,
    },
    Vsop87Term {
        amplitude: 0.00000003521,
        phase: 5.97844807108,
        frequency: 3894.18182954220,
    },
    Vsop87Term {
        amplitude: 0.00000004215,
        phase: 1.90601120623,
        frequency: 245.83164622940,
    },
    Vsop87Term {
        amplitude: 0.00000003701,
        phase: 5.03069397926,
        frequency: 536.80451209540,
    },
    Vsop87Term {
        amplitude: 0.00000003865,
        phase: 1.82634360607,
        frequency: 11856.21865142450,
    },
    Vsop87Term {
        amplitude: 0.00000003652,
        phase: 1.01838584934,
        frequency: 16200.77272450120,
    },
    Vsop87Term {
        amplitude: 0.00000003390,
        phase: 0.97785123922,
        frequency: 8635.94200376320,
    },
    Vsop87Term {
        amplitude: 0.00000003737,
        phase: 2.95380107829,
        frequency: 3128.38876509580,
    },
    Vsop87Term {
        amplitude: 0.00000003507,
        phase: 3.71291946325,
        frequency: 6290.18939699220,
    },
    Vsop87Term {
        amplitude: 0.00000003086,
        phase: 3.64646921512,
        frequency: 10.63666534980,
    },
    Vsop87Term {
        amplitude: 0.00000003397,
        phase: 1.10590684017,
        frequency: 14712.31711645800,
    },
    Vsop87Term {
        amplitude: 0.00000003334,
        phase: 0.83684924911,
        frequency: 6496.37494542940,
    },
    Vsop87Term {
        amplitude: 0.00000002805,
        phase: 2.58504514144,
        frequency: 14314.16811304980,
    },
    Vsop87Term {
        amplitude: 0.00000003650,
        phase: 1.08344142571,
        frequency: 88860.05707098669,
    },
    Vsop87Term {
        amplitude: 0.00000003388,
        phase: 3.20185096055,
        frequency: 5120.60114558360,
    },
    Vsop87Term {
        amplitude: 0.00000003252,
        phase: 3.47859752062,
        frequency: 6133.51265285680,
    },
    Vsop87Term {
        amplitude: 0.00000002553,
        phase: 3.94869034189,
        frequency: 1990.74501704100,
    },
    Vsop87Term {
        amplitude: 0.00000003520,
        phase: 2.05559692878,
        frequency: 244287.60000722769,
    },
    Vsop87Term {
        amplitude: 0.00000002565,
        phase: 1.56071784900,
        frequency: 23543.23050468179,
    },
    Vsop87Term {
        amplitude: 0.00000002621,
        phase: 3.85639359951,
        frequency: 266.60704172180,
    },
    Vsop87Term {
        amplitude: 0.00000002955,
        phase: 3.39692949667,
        frequency: 9225.53927328300,
    },
    Vsop87Term {
        amplitude: 0.00000002876,
        phase: 6.02635617464,
        frequency: 154717.60988768269,
    },
    Vsop87Term {
        amplitude: 0.00000002395,
        phase: 1.16131956403,
        frequency: 10984.19235169980,
    },
    Vsop87Term {
        amplitude: 0.00000003161,
        phase: 1.32798718453,
        frequency: 10873.98603048040,
    },
    Vsop87Term {
        amplitude: 0.00000003163,
        phase: 5.08946464629,
        frequency: 21228.39202354580,
    },
    Vsop87Term {
        amplitude: 0.00000002361,
        phase: 4.27212906992,
        frequency: 6040.34724601740,
    },
    Vsop87Term {
        amplitude: 0.00000003030,
        phase: 1.80209931347,
        frequency: 35371.88726597640,
    },
    Vsop87Term {
        amplitude: 0.00000002343,
        phase: 3.57689860500,
        frequency: 10969.96525769820,
    },
    Vsop87Term {
        amplitude: 0.00000002618,
        phase: 2.57870156528,
        frequency: 22483.84857449259,
    },
    Vsop87Term {
        amplitude: 0.00000002113,
        phase: 3.71393780256,
        frequency: 65147.61976813770,
    },
    Vsop87Term {
        amplitude: 0.00000002019,
        phase: 0.81393923319,
        frequency: 170.67287061920,
    },
    Vsop87Term {
        amplitude: 0.00000002003,
        phase: 0.38091017375,
        frequency: 6172.86952877200,
    },
    Vsop87Term {
        amplitude: 0.00000002506,
        phase: 3.74379142438,
        frequency: 10575.40668294180,
    },
    Vsop87Term {
        amplitude: 0.00000002381,
        phase: 0.10581361289,
        frequency: 7.04623669800,
    },
    Vsop87Term {
        amplitude: 0.00000001949,
        phase: 4.86892513469,
        frequency: 36.02786667740,
    },
    Vsop87Term {
        amplitude: 0.00000002074,
        phase: 4.22794774570,
        frequency: 5650.29211067820,
    },
    Vsop87Term {
        amplitude: 0.00000001924,
        phase: 5.59460549860,
        frequency: 6282.09552892320,
    },
    Vsop87Term {
        amplitude: 0.00000001949,
        phase: 1.07002512703,
        frequency: 5230.80746680300,
    },
    Vsop87Term {
        amplitude: 0.00000001988,
        phase: 5.19736046771,
        frequency: 6262.30045449900,
    },
    Vsop87Term {
        amplitude: 0.00000001887,
        phase: 3.74365662683,
        frequency: 23.87843774780,
    },
    Vsop87Term {
        amplitude: 0.00000001787,
        phase: 1.25929682929,
        frequency: 12559.03815298200,
    },
    Vsop87Term {
        amplitude: 0.00000001883,
        phase: 1.90364058477,
        frequency: 15.25247118500,
    },
    Vsop87Term {
        amplitude: 0.00000001816,
        phase: 3.68083868442,
        frequency: 15110.46611986620,
    },
    Vsop87Term {
        amplitude: 0.00000001701,
        phase: 4.41105895380,
        frequency: 110.20632121940,
    },
    Vsop87Term {
        amplitude: 0.00000001990,
        phase: 3.93295788548,
        frequency: 6206.80977871580,
    },
    Vsop87Term {
        amplitude: 0.00000002103,
        phase: 0.75354917468,
        frequency: 13521.75144159140,
    },
    Vsop87Term {
        amplitude: 0.00000001774,
        phase: 0.48747535361,
        frequency: 1551.04522264800,
    },
    Vsop87Term {
        amplitude: 0.00000001882,
        phase: 0.86684493432,
        frequency: 22003.91463486980,
    },
    Vsop87Term {
        amplitude: 0.00000001924,
        phase: 1.22898324132,
        frequency: 709.93304855830,
    },
    Vsop87Term {
        amplitude: 0.00000002009,
        phase: 4.62850921980,
        frequency: 6037.24420376200,
    },
    Vsop87Term {
        amplitude: 0.00000001924,
        phase: 0.60231842508,
        frequency: 6284.05617105960,
    },
    Vsop87Term {
        amplitude: 0.00000001596,
        phase: 3.98332956992,
        frequency: 13916.01910964160,
    },
    Vsop87Term {
        amplitude: 0.00000001664,
        phase: 4.41939715469,
        frequency: 8662.24032356300,
    },
    Vsop87Term {
        amplitude: 0.00000001971,
        phase: 1.04560500503,
        frequency: 18209.33026366019,
    },
    Vsop87Term {
        amplitude: 0.00000001942,
        phase: 4.31335979989,
        frequency: 6244.94281435360,
    },
    Vsop87Term {
        amplitude: 0.00000001476,
        phase: 0.93271367331,
        frequency: 2379.16447357160,
    },
    Vsop87Term {
        amplitude: 0.00000001810,
        phase: 0.49112137707,
        frequency: 1.48447270830,
    },
    Vsop87Term {
        amplitude: 0.00000001346,
        phase: 1.51574702235,
        frequency: 4136.91043351620,
    },
    Vsop87Term {
        amplitude: 0.00000001528,
        phase: 5.61835711404,
        frequency: 6127.65545055720,
    },
    Vsop87Term {
        amplitude: 0.00000001791,
        phase: 3.22187270126,
        frequency: 39302.09696219600,
    },
    Vsop87Term {
        amplitude: 0.00000001747,
        phase: 3.05638656738,
        frequency: 18319.53658487960,
    },
    Vsop87Term {
        amplitude: 0.00000001431,
        phase: 4.51153808594,
        frequency: 20426.57109242200,
    },
    Vsop87Term {
        amplitude: 0.00000001695,
        phase: 0.22047718414,
        frequency: 25158.60171976540,
    },
    Vsop87Term {
        amplitude: 0.00000001242,
        phase: 4.46665769933,
        frequency: 17256.63153634140,
    },
    Vsop87Term {
        amplitude: 0.00000001463,
        phase: 4.69242679213,
        frequency: 14945.31617355440,
    },
    Vsop87Term {
        amplitude: 0.00000001205,
        phase: 1.86912144659,
        frequency: 4590.91018048900,
    },
    Vsop87Term {
        amplitude: 0.00000001192,
        phase: 2.74227166898,
        frequency: 12569.67481833180,
    },
    Vsop87Term {
        amplitude: 0.00000001222,
        phase: 5.18120087482,
        frequency: 5333.90024102160,
    },
    Vsop87Term {
        amplitude: 0.00000001390,
        phase: 5.42894648983,
        frequency: 143571.32428481648,
    },
    Vsop87Term {
        amplitude: 0.00000001473,
        phase: 1.70479245805,
        frequency: 11712.95531823080,
    },
    Vsop87Term {
        amplitude: 0.00000001362,
        phase: 2.61069503292,
        frequency: 6062.66320755260,
    },
    Vsop87Term {
        amplitude: 0.00000001148,
        phase: 6.03001800540,
        frequency: 3634.62102451840,
    },
    Vsop87Term {
        amplitude: 0.00000001198,
        phase: 5.15294130422,
        frequency: 10177.25767953360,
    },
    Vsop87Term {
        amplitude: 0.00000001266,
        phase: 0.11421493643,
        frequency: 18422.62935909819,
    },
    Vsop87Term {
        amplitude: 0.00000001411,
        phase: 1.09908857534,
        frequency: 3496.03282613400,
    },
    Vsop87Term {
        amplitude: 0.00000001349,
        phase: 2.99805109633,
        frequency: 17654.78053974960,
    },
    Vsop87Term {
        amplitude: 0.00000001253,
        phase: 2.79850152848,
        frequency: 167283.76158766549,
    },
    Vsop87Term {
        amplitude: 0.00000001311,
        phase: 1.60942984879,
        frequency: 5481.25491886760,
    },
    Vsop87Term {
        amplitude: 0.00000001079,
        phase: 6.20304501787,
        frequency: 3.28635741780,
    },
    Vsop87Term {
        amplitude: 0.00000001181,
        phase: 1.20653776978,
        frequency: 131.54196168640,
    },
    Vsop87Term {
        amplitude: 0.00000001254,
        phase: 5.45103277798,
        frequency: 6076.89030155420,
    },
    Vsop87Term {
        amplitude: 0.00000001035,
        phase: 2.32142722747,
        frequency: 7342.45778018060,
    },
    Vsop87Term {
        amplitude: 0.00000001117,
        phase: 0.38838354256,
        frequency: 949.17560896980,
    },
    Vsop87Term {
        amplitude: 0.00000000966,
        phase: 3.18341890851,
        frequency: 11087.28512591840,
    },
    Vsop87Term {
        amplitude: 0.00000001171,
        phase: 3.39635049962,
        frequency: 12562.62858163380,
    },
    Vsop87Term {
        amplitude: 0.00000001121,
        phase: 0.72627490378,
        frequency: 220.41264243880,
    },
    Vsop87Term {
        amplitude: 0.00000001024,
        phase: 2.19378315386,
        frequency: 11403.67699557500,
    },
    Vsop87Term {
        amplitude: 0.00000000888,
        phase: 3.91173199285,
        frequency: 4686.88940770680,
    },
    Vsop87Term {
        amplitude: 0.00000000910,
        phase: 1.98802695087,
        frequency: 735.87651353180,
    },
    Vsop87Term {
        amplitude: 0.00000000830,
        phase: 0.48984915507,
        frequency: 24072.92146977640,
    },
    Vsop87Term {
        amplitude: 0.00000001096,
        phase: 6.17377835617,
        frequency: 5436.99301524020,
    },
    Vsop87Term {
        amplitude: 0.00000000908,
        phase: 0.44959639433,
        frequency: 7477.52286021600,
    },
    Vsop87Term {
        amplitude: 0.00000000974,
        phase: 1.52996238356,
        frequency: 9623.68827669120,
    },
    Vsop87Term {
        amplitude: 0.00000000840,
        phase: 1.79543266333,
        frequency: 5429.87946823940,
    },
    Vsop87Term {
        amplitude: 0.00000000778,
        phase: 6.17699177946,
        frequency: 38.13303563780,
    },
    Vsop87Term {
        amplitude: 0.00000000776,
        phase: 4.09855402433,
        frequency: 14.22709400160,
    },
    Vsop87Term {
        amplitude: 0.00000001068,
        phase: 4.64200173735,
        frequency: 43232.30665841560,
    },
    Vsop87Term {
        amplitude: 0.00000000954,
        phase: 1.49988435748,
        frequency: 1162.47470440780,
    },
    Vsop87Term {
        amplitude: 0.00000000907,
        phase: 0.86986870809,
        frequency: 10344.29506538580,
    },
    Vsop87Term {
        amplitude: 0.00000000931,
        phase: 4.06044689031,
        frequency: 28766.92442448400,
    },
    Vsop87Term {
        amplitude: 0.00000000739,
        phase: 5.04368197372,
        frequency: 639.89728631400,
    },
    Vsop87Term {
        amplitude: 0.00000000937,
        phase: 3.46884698960,
        frequency: 1589.07289528380,
    },
    Vsop87Term {
        amplitude: 0.00000000763,
        phase: 5.86304932998,
        frequency: 16858.48253293320,
    },
    Vsop87Term {
        amplitude: 0.00000000953,
        phase: 4.20801492835,
        frequency: 11190.37790013700,
    },
    Vsop87Term {
        amplitude: 0.00000000708,
        phase: 1.72899988940,
        frequency: 13095.84266507740,
    },
    Vsop87Term {
        amplitude: 0.00000000969,
        phase: 1.64439522215,
        frequency: 29088.81141598500,
    },
    Vsop87Term {
        amplitude: 0.00000000717,
        phase: 0.16688678895,
        frequency: 11.72935283600,
    },
    Vsop87Term {
        amplitude: 0.00000000962,
        phase: 3.53092337542,
        frequency: 12416.58850284820,
    },
    Vsop87Term {
        amplitude: 0.00000000747,
        phase: 5.77866940346,
        frequency: 12592.45001978260,
    },
    Vsop87Term {
        amplitude: 0.00000000672,
        phase: 1.91095796194,
        frequency: 3.93215326310,
    },
    Vsop87Term {
        amplitude: 0.00000000671,
        phase: 5.46240843677,
        frequency: 18052.92954315780,
    },
    Vsop87Term {
        amplitude: 0.00000000675,
        phase: 6.28311558823,
        frequency: 4535.05943692440,
    },
    Vsop87Term {
        amplitude: 0.00000000684,
        phase: 0.39975012080,
        frequency: 5849.36411211460,
    },
    Vsop87Term {
        amplitude: 0.00000000799,
        phase: 0.29851185294,
        frequency: 12132.43996210600,
    },
    Vsop87Term {
        amplitude: 0.00000000758,
        phase: 0.96370823331,
        frequency: 1052.26838318840,
    },
    Vsop87Term {
        amplitude: 0.00000000782,
        phase: 5.33878339919,
        frequency: 13517.87010623340,
    },
    Vsop87Term {
        amplitude: 0.00000000730,
        phase: 1.70106160291,
        frequency: 17267.26820169119,
    },
    Vsop87Term {
        amplitude: 0.00000000749,
        phase: 2.59599901875,
        frequency: 11609.86254401220,
    },
    Vsop87Term {
        amplitude: 0.00000000734,
        phase: 2.78417782952,
        frequency: 640.87760738220,
    },
    Vsop87Term {
        amplitude: 0.00000000688,
        phase: 5.15048287468,
        frequency: 16496.36139620240,
    },
    Vsop87Term {
        amplitude: 0.00000000770,
        phase: 1.62469589333,
        frequency: 4701.11650170840,
    },
    Vsop87Term {
        amplitude: 0.00000000633,
        phase: 2.20587893893,
        frequency: 25934.12433108940,
    },
    Vsop87Term {
        amplitude: 0.00000000760,
        phase: 4.21317219403,
        frequency: 377.37360791580,
    },
    Vsop87Term {
        amplitude: 0.00000000584,
        phase: 2.13420121623,
        frequency: 10557.59416082380,
    },
    Vsop87Term {
        amplitude: 0.00000000574,
        phase: 0.24250054587,
        frequency: 9779.10867612540,
    },
    Vsop87Term {
        amplitude: 0.00000000573,
        phase: 3.16435264609,
        frequency: 533.21408344360,
    },
    Vsop87Term {
        amplitude: 0.00000000685,
        phase: 3.19344289472,
        frequency: 12146.66705610760,
    },
    Vsop87Term {
        amplitude: 0.00000000675,
        phase: 0.96179233959,
        frequency: 10454.50138660520,
    },
    Vsop87Term {
        amplitude: 0.00000000648,
        phase: 1.46327342555,
        frequency: 6268.84875598980,
    },
    Vsop87Term {
        amplitude: 0.00000000589,
        phase: 2.50543543638,
        frequency: 3097.88382272579,
    },
    Vsop87Term {
        amplitude: 0.00000000551,
        phase: 5.28099026956,
        frequency: 9388.00590941520,
    },
    Vsop87Term {
        amplitude: 0.00000000696,
        phase: 3.65342150016,
        frequency: 4804.20927592700,
    },
    Vsop87Term {
        amplitude: 0.00000000669,
        phase: 2.51030077026,
        frequency: 2388.89402044920,
    },
    Vsop87Term {
        amplitude: 0.00000000550,
        phase: 0.06883864342,
        frequency: 20199.09495963300,
    },
    Vsop87Term {
        amplitude: 0.00000000629,
        phase: 4.13350995675,
        frequency: 45892.73043315699,
    },
    Vsop87Term {
        amplitude: 0.00000000678,
        phase: 6.09190163533,
        frequency: 135.62532501000,
    },
    Vsop87Term {
        amplitude: 0.00000000593,
        phase: 1.50136257618,
        frequency: 226858.23855437008,
    },
    Vsop87Term {
        amplitude: 0.00000000542,
        phase: 3.58573645173,
        frequency: 6148.01076995600,
    },
    Vsop87Term {
        amplitude: 0.00000000682,
        phase: 5.02203067788,
        frequency: 17253.04110768959,
    },
    Vsop87Term {
        amplitude: 0.00000000565,
        phase: 4.29309238610,
        frequency: 11933.36796066960,
    },
    Vsop87Term {
        amplitude: 0.00000000486,
        phase: 0.77746204893,
        frequency: 27.40155609680,
    },
    Vsop87Term {
        amplitude: 0.00000000503,
        phase: 0.58963565969,
        frequency: 15671.08175940660,
    },
    Vsop87Term {
        amplitude: 0.00000000616,
        phase: 4.06539884128,
        frequency: 227.47613278900,
    },
    Vsop87Term {
        amplitude: 0.00000000583,
        phase: 6.12695541996,
        frequency: 18875.52586977400,
    },
    Vsop87Term {
        amplitude: 0.00000000537,
        phase: 2.15056440980,
        frequency: 21954.15760939799,
    },
    Vsop87Term {
        amplitude: 0.00000000669,
        phase: 6.06986269566,
        frequency: 47162.51635463520,
    },
    Vsop87Term {
        amplitude: 0.00000000475,
        phase: 0.40343842110,
        frequency: 6915.85958930460,
    },
    Vsop87Term {
        amplitude: 0.00000000540,
        phase: 2.83444222174,
        frequency: 5326.78669402080,
    },
    Vsop87Term {
        amplitude: 0.00000000530,
        phase: 5.26359885263,
        frequency: 10988.80815753500,
    },
    Vsop87Term {
        amplitude: 0.00000000582,
        phase: 3.24533095664,
        frequency: 153.77881048480,
    },
    Vsop87Term {
        amplitude: 0.00000000641,
        phase: 3.24711791371,
        frequency: 2107.03450754240,
    },
    Vsop87Term {
        amplitude: 0.00000000621,
        phase: 3.09698523779,
        frequency: 33019.02111220460,
    },
    Vsop87Term {
        amplitude: 0.00000000466,
        phase: 3.14982372198,
        frequency: 10440.27429260360,
    },
    Vsop87Term {
        amplitude: 0.00000000466,
        phase: 0.90708835657,
        frequency: 5966.68398033480,
    },
    Vsop87Term {
        amplitude: 0.00000000528,
        phase: 0.81926454470,
        frequency: 813.55028395980,
    },
    Vsop87Term {
        amplitude: 0.00000000603,
        phase: 3.81378921927,
        frequency: 316428.22867391503,
    },
    Vsop87Term {
        amplitude: 0.00000000559,
        phase: 1.81894804124,
        frequency: 17996.03116822220,
    },
    Vsop87Term {
        amplitude: 0.00000000437,
        phase: 2.28625594435,
        frequency: 6303.85124548380,
    },
    Vsop87Term {
        amplitude: 0.00000000518,
        phase: 4.86069178322,
        frequency: 20597.24396304120,
    },
    Vsop87Term {
        amplitude: 0.00000000424,
        phase: 6.23520018693,
        frequency: 6489.26139842860,
    },
    Vsop87Term {
        amplitude: 0.00000000518,
        phase: 6.17617826756,
        frequency: 0.24381748350,
    },
    Vsop87Term {
        amplitude: 0.00000000404,
        phase: 5.72804304258,
        frequency: 5642.19824260920,
    },
    Vsop87Term {
        amplitude: 0.00000000458,
        phase: 1.34117773915,
        frequency: 6287.00800325450,
    },
    Vsop87Term {
        amplitude: 0.00000000548,
        phase: 5.68454458320,
        frequency: 155427.54293624099,
    },
    Vsop87Term {
        amplitude: 0.00000000547,
        phase: 1.03391472061,
        frequency: 3646.35037735440,
    },
    Vsop87Term {
        amplitude: 0.00000000428,
        phase: 4.69800981138,
        frequency: 846.08283475120,
    },
    Vsop87Term {
        amplitude: 0.00000000413,
        phase: 6.02520699406,
        frequency: 6279.48542133960,
    },
    Vsop87Term {
        amplitude: 0.00000000534,
        phase: 3.03030638223,
        frequency: 66567.48586525429,
    },
    Vsop87Term {
        amplitude: 0.00000000383,
        phase: 1.49056949125,
        frequency: 19800.94595622480,
    },
    Vsop87Term {
        amplitude: 0.00000000410,
        phase: 5.28319622279,
        frequency: 18451.07854656599,
    },
    Vsop87Term {
        amplitude: 0.00000000352,
        phase: 4.68891600359,
        frequency: 4907.30205014560,
    },
    Vsop87Term {
        amplitude: 0.00000000480,
        phase: 5.36572651091,
        frequency: 348.92442044800,
    },
    Vsop87Term {
        amplitude: 0.00000000344,
        phase: 5.89157452896,
        frequency: 6546.15977336420,
    },
    Vsop87Term {
        amplitude: 0.00000000340,
        phase: 0.37557426440,
        frequency: 13119.72110282519,
    },
    Vsop87Term {
        amplitude: 0.00000000434,
        phase: 4.98417785901,
        frequency: 6702.56049386660,
    },
    Vsop87Term {
        amplitude: 0.00000000332,
        phase: 2.68902519126,
        frequency: 29296.61538957860,
    },
    Vsop87Term {
        amplitude: 0.00000000448,
        phase: 2.16478480251,
        frequency: 5905.70224207560,
    },
    Vsop87Term {
        amplitude: 0.00000000344,
        phase: 2.06546633735,
        frequency: 49.75702547180,
    },
    Vsop87Term {
        amplitude: 0.00000000315,
        phase: 1.24023811803,
        frequency: 4061.21921539440,
    },
    Vsop87Term {
        amplitude: 0.00000000324,
        phase: 2.30897526929,
        frequency: 5017.50837136500,
    },
    Vsop87Term {
        amplitude: 0.00000000413,
        phase: 0.17171692962,
        frequency: 6286.66627864320,
    },
    Vsop87Term {
        amplitude: 0.00000000431,
        phase: 3.86601101393,
        frequency: 12489.88562870720,
    },
    Vsop87Term {
        amplitude: 0.00000000349,
        phase: 4.55372342974,
        frequency: 4933.20844033260,
    },
    Vsop87Term {
        amplitude: 0.00000000323,
        phase: 0.41971136084,
        frequency: 10770.89325626180,
    },
    Vsop87Term {
        amplitude: 0.00000000341,
        phase: 2.68612860807,
        frequency: 11.04570026390,
    },
    Vsop87Term {
        amplitude: 0.00000000316,
        phase: 3.52936906658,
        frequency: 17782.73207278420,
    },
    Vsop87Term {
        amplitude: 0.00000000315,
        phase: 5.63357264999,
        frequency: 568.82187402740,
    },
    Vsop87Term {
        amplitude: 0.00000000340,
        phase: 3.83571212349,
        frequency: 10660.68693504240,
    },
    Vsop87Term {
        amplitude: 0.00000000297,
        phase: 0.62691416712,
        frequency: 20995.39296644940,
    },
    Vsop87Term {
        amplitude: 0.00000000405,
        phase: 1.00085779471,
        frequency: 16460.33352952499,
    },
    Vsop87Term {
        amplitude: 0.00000000414,
        phase: 1.21998752076,
        frequency: 51092.72605085480,
    },
    Vsop87Term {
        amplitude: 0.00000000336,
        phase: 4.71465945226,
        frequency: 6179.98307577280,
    },
    Vsop87Term {
        amplitude: 0.00000000361,
        phase: 3.71227508354,
        frequency: 28237.23345938940,
    },
    Vsop87Term {
        amplitude: 0.00000000385,
        phase: 6.21925225757,
        frequency: 24356.78078864160,
    },
    Vsop87Term {
        amplitude: 0.00000000327,
        phase: 1.05606504715,
        frequency: 11919.14086666800,
    },
    Vsop87Term {
        amplitude: 0.00000000327,
        phase: 6.14222420989,
        frequency: 6254.62666252360,
    },
    Vsop87Term {
        amplitude: 0.00000000268,
        phase: 2.47224339737,
        frequency: 664.75604513000,
    },
    Vsop87Term {
        amplitude: 0.00000000269,
        phase: 1.86207884109,
        frequency: 23141.55838292460,
    },
    Vsop87Term {
        amplitude: 0.00000000345,
        phase: 0.93461290184,
        frequency: 6058.73105428950,
    },
    Vsop87Term {
        amplitude: 0.00000000296,
        phase: 4.51687557180,
        frequency: 6418.14093002680,
    },
    Vsop87Term {
        amplitude: 0.00000000353,
        phase: 4.50033653082,
        frequency: 36949.23080842420,
    },
    Vsop87Term {
        amplitude: 0.00000000260,
        phase: 4.04963546305,
        frequency: 6525.80445396540,
    },
    Vsop87Term {
        amplitude: 0.00000000298,
        phase: 2.20046722622,
        frequency: 156137.47598479928,
    },
    Vsop87Term {
        amplitude: 0.00000000253,
        phase: 3.49900838384,
        frequency: 29864.33402730900,
    },
    Vsop87Term {
        amplitude: 0.00000000254,
        phase: 2.44901693835,
        frequency: 5331.35744374080,
    },
    Vsop87Term {
        amplitude: 0.00000000296,
        phase: 0.84347588787,
        frequency: 5729.50644714900,
    },
    Vsop87Term {
        amplitude: 0.00000000298,
        phase: 1.29194706125,
        frequency: 22805.73556599360,
    },
    Vsop87Term {
        amplitude: 0.00000000241,
        phase: 2.00721280805,
        frequency: 16737.57723659660,
    },
    Vsop87Term {
        amplitude: 0.00000000311,
        phase: 1.23668016334,
        frequency: 6281.59137728310,
    },
    Vsop87Term {
        amplitude: 0.00000000240,
        phase: 2.51650377121,
        frequency: 6245.04817735560,
    },
    Vsop87Term {
        amplitude: 0.00000000332,
        phase: 3.55576945724,
        frequency: 7668.63742494250,
    },
    Vsop87Term {
        amplitude: 0.00000000264,
        phase: 4.44052061202,
        frequency: 12964.30070339100,
    },
    Vsop87Term {
        amplitude: 0.00000000257,
        phase: 1.79654471948,
        frequency: 11080.17157891760,
    },
    Vsop87Term {
        amplitude: 0.00000000260,
        phase: 3.33077598420,
        frequency: 5888.44996493220,
    },
    Vsop87Term {
        amplitude: 0.00000000285,
        phase: 0.30886361430,
        frequency: 11823.16163945020,
    },
    Vsop87Term {
        amplitude: 0.00000000290,
        phase: 5.70141882483,
        frequency: 77.67377042800,
    },
    Vsop87Term {
        amplitude: 0.00000000255,
        phase: 4.00939664440,
        frequency: 5881.40372823420,
    },
    Vsop87Term {
        amplitude: 0.00000000253,
        phase: 4.73318493678,
        frequency: 16723.35014259500,
    },
    Vsop87Term {
        amplitude: 0.00000000228,
        phase: 0.95333661324,
        frequency: 5540.08578945880,
    },
    Vsop87Term {
        amplitude: 0.00000000319,
        phase: 1.38633229189,
        frequency: 163096.18036118349,
    },
    Vsop87Term {
        amplitude: 0.00000000224,
        phase: 1.65156322696,
        frequency: 10027.90319572920,
    },
    Vsop87Term {
        amplitude: 0.00000000226,
        phase: 0.34106460604,
        frequency: 17796.95916678580,
    },
    Vsop87Term {
        amplitude: 0.00000000236,
        phase: 4.19817431922,
        frequency: 19.66976089979,
    },
    Vsop87Term {
        amplitude: 0.00000000280,
        phase: 4.14080268970,
        frequency: 12539.85338018300,
    },
    Vsop87Term {
        amplitude: 0.00000000275,
        phase: 5.50306930248,
        frequency: 32.53255079140,
    },
    Vsop87Term {
        amplitude: 0.00000000223,
        phase: 5.23334210294,
        frequency: 56.89837493560,
    },
    Vsop87Term {
        amplitude: 0.00000000217,
        phase: 6.08587881787,
        frequency: 6805.65326808520,
    },
    Vsop87Term {
        amplitude: 0.00000000280,
        phase: 4.52472044653,
        frequency: 6016.46880826960,
    },
    Vsop87Term {
        amplitude: 0.00000000227,
        phase: 5.06509843737,
        frequency: 6277.55292568400,
    },
    Vsop87Term {
        amplitude: 0.00000000226,
        phase: 5.17755154305,
        frequency: 11720.06886523160,
    },
    Vsop87Term {
        amplitude: 0.00000000245,
        phase: 3.96486270306,
        frequency: 22.77520145080,
    },
    Vsop87Term {
        amplitude: 0.00000000220,
        phase: 4.72078081970,
        frequency: 6.62855890001,
    },
    Vsop87Term {
        amplitude: 0.00000000207,
        phase: 5.71701403951,
        frequency: 41.55079098480,
    },
    Vsop87Term {
        amplitude: 0.00000000204,
        phase: 3.91227411250,
        frequency: 2699.73481931760,
    },
    Vsop87Term {
        amplitude: 0.00000000209,
        phase: 0.86881969011,
        frequency: 6321.10352262720,
    },
    Vsop87Term {
        amplitude: 0.00000000200,
        phase: 2.11984445273,
        frequency: 4274.51831083240,
    },
    Vsop87Term {
        amplitude: 0.00000000200,
        phase: 5.39839888163,
        frequency: 6019.99192661860,
    },
    Vsop87Term {
        amplitude: 0.00000000209,
        phase: 5.67606291663,
        frequency: 11293.47067435560,
    },
    Vsop87Term {
        amplitude: 0.00000000252,
        phase: 1.64965729351,
        frequency: 9380.95967271720,
    },
    Vsop87Term {
        amplitude: 0.00000000275,
        phase: 5.04826903506,
        frequency: 73.29712585900,
    },
    Vsop87Term {
        amplitude: 0.00000000208,
        phase: 1.88207277133,
        frequency: 11300.58422135640,
    },
    Vsop87Term {
        amplitude: 0.00000000272,
        phase: 0.74640926842,
        frequency: 1975.49254585600,
    },
    Vsop87Term {
        amplitude: 0.00000000199,
        phase: 3.30836672397,
        frequency: 22743.40937951640,
    },
    Vsop87Term {
        amplitude: 0.00000000269,
        phase: 4.48560812155,
        frequency: 64471.99124174489,
    },
    Vsop87Term {
        amplitude: 0.00000000192,
        phase: 2.17464236325,
        frequency: 5863.59120611620,
    },
    Vsop87Term {
        amplitude: 0.00000000228,
        phase: 5.85373115869,
        frequency: 128.01884333740,
    },
    Vsop87Term {
        amplitude: 0.00000000261,
        phase: 2.64321183295,
        frequency: 55022.93574707440,
    },
    Vsop87Term {
        amplitude: 0.00000000220,
        phase: 5.75012110079,
        frequency: 29.42950853600,
    },
    Vsop87Term {
        amplitude: 0.00000000187,
        phase: 4.03230554718,
        frequency: 467.96499035440,
    },
    Vsop87Term {
        amplitude: 0.00000000200,
        phase: 5.60556112058,
        frequency: 1066.49547719000,
    },
    Vsop87Term {
        amplitude: 0.00000000231,
        phase: 1.09802712785,
        frequency: 12341.80690428090,
    },
    Vsop87Term {
        amplitude: 0.00000000199,
        phase: 0.29500625200,
        frequency: 149.56319713460,
    },
    Vsop87Term {
        amplitude: 0.00000000249,
        phase: 5.10473210814,
        frequency: 7875.67186362420,
    },
    Vsop87Term {
        amplitude: 0.00000000208,
        phase: 0.93013835019,
        frequency: 14919.01785375460,
    },
    Vsop87Term {
        amplitude: 0.00000000179,
        phase: 0.87104393079,
        frequency: 12721.57209941700,
    },
    Vsop87Term {
        amplitude: 0.00000000203,
        phase: 1.56920753653,
        frequency: 28286.99048486120,
    },
    Vsop87Term {
        amplitude: 0.00000000179,
        phase: 2.47036386443,
        frequency: 16062.18452611680,
    },
    Vsop87Term {
        amplitude: 0.00000000198,
        phase: 3.54061588502,
        frequency: 30.91412563500,
    },
    Vsop87Term {
        amplitude: 0.00000000171,
        phase: 3.45356518113,
        frequency: 5327.47610838280,
    },
    Vsop87Term {
        amplitude: 0.00000000183,
        phase: 0.72325421604,
        frequency: 6272.03014972750,
    },
    Vsop87Term {
        amplitude: 0.00000000216,
        phase: 2.97174580686,
        frequency: 19402.79695281660,
    },
    Vsop87Term {
        amplitude: 0.00000000168,
        phase: 2.51550550242,
        frequency: 23937.85638974100,
    },
    Vsop87Term {
        amplitude: 0.00000000195,
        phase: 0.09045393425,
        frequency: 156.40072050240,
    },
    Vsop87Term {
        amplitude: 0.00000000179,
        phase: 4.49471798090,
        frequency: 31415.37924995700,
    },
    Vsop87Term {
        amplitude: 0.00000000216,
        phase: 0.42177594328,
        frequency: 23539.70738633280,
    },
    Vsop87Term {
        amplitude: 0.00000000189,
        phase: 0.37542530191,
        frequency: 9814.60410029120,
    },
    Vsop87Term {
        amplitude: 0.00000000218,
        phase: 2.36835880025,
        frequency: 16627.37091537720,
    },
    Vsop87Term {
        amplitude: 0.00000000166,
        phase: 4.23182968446,
        frequency: 16840.67001081519,
    },
    Vsop87Term {
        amplitude: 0.00000000200,
        phase: 2.02153258098,
        frequency: 16097.67995028260,
    },
    Vsop87Term {
        amplitude: 0.00000000169,
        phase: 0.91318727000,
        frequency: 95.97922721780,
    },
    Vsop87Term {
        amplitude: 0.00000000211,
        phase: 5.73370637657,
        frequency: 151.89728108520,
    },
    Vsop87Term {
        amplitude: 0.00000000204,
        phase: 0.42643085174,
        frequency: 515.46387109300,
    },
    Vsop87Term {
        amplitude: 0.00000000212,
        phase: 3.00233538977,
        frequency: 12043.57428188900,
    },
    Vsop87Term {
        amplitude: 0.00000000192,
        phase: 5.46153589821,
        frequency: 6379.05507720920,
    },
    Vsop87Term {
        amplitude: 0.00000000165,
        phase: 1.38698167064,
        frequency: 4171.42553661380,
    },
    Vsop87Term {
        amplitude: 0.00000000160,
        phase: 6.23798383332,
        frequency: 202.25339517410,
    },
    Vsop87Term {
        amplitude: 0.00000000215,
        phase: 0.20889073407,
        frequency: 5621.84292321040,
    },
    Vsop87Term {
        amplitude: 0.00000000181,
        phase: 4.12439203622,
        frequency: 13341.67431130680,
    },
    Vsop87Term {
        amplitude: 0.00000000153,
        phase: 1.24460848836,
        frequency: 29826.30635467320,
    },
    Vsop87Term {
        amplitude: 0.00000000150,
        phase: 3.12999753018,
        frequency: 799.82112516540,
    },
    Vsop87Term {
        amplitude: 0.00000000175,
        phase: 4.55671604437,
        frequency: 239424.39025435288,
    },
    Vsop87Term {
        amplitude: 0.00000000192,
        phase: 1.33928820063,
        frequency: 394.62588505920,
    },
    Vsop87Term {
        amplitude: 0.00000000149,
        phase: 2.65697593276,
        frequency: 21.33564046700,
    },
    Vsop87Term {
        amplitude: 0.00000000146,
        phase: 5.58021191726,
        frequency: 412.37109687440,
    },
    Vsop87Term {
        amplitude: 0.00000000156,
        phase: 3.75650175503,
        frequency: 12323.42309600880,
    },
    Vsop87Term {
        amplitude: 0.00000000143,
        phase: 3.75708566606,
        frequency: 58864.54391814630,
    },
    Vsop87Term {
        amplitude: 0.00000000143,
        phase: 3.28248547724,
        frequency: 29.82143814880,
    },
    Vsop87Term {
        amplitude: 0.00000000144,
        phase: 1.07862546598,
        frequency: 1265.56747862640,
    },
    Vsop87Term {
        amplitude: 0.00000000148,
        phase: 0.23389236655,
        frequency: 10021.83728009940,
    },
    Vsop87Term {
        amplitude: 0.00000000193,
        phase: 5.92751083086,
        frequency: 40879.44050464380,
    },
    Vsop87Term {
        amplitude: 0.00000000140,
        phase: 4.97612440269,
        frequency: 158.94351778320,
    },
    Vsop87Term {
        amplitude: 0.00000000148,
        phase: 2.61640453469,
        frequency: 17157.06188047180,
    },
    Vsop87Term {
        amplitude: 0.00000000141,
        phase: 3.66871308723,
        frequency: 26084.02180621620,
    },
    Vsop87Term {
        amplitude: 0.00000000147,
        phase: 5.09968173403,
        frequency: 661.23292678100,
    },
    Vsop87Term {
        amplitude: 0.00000000146,
        phase: 4.96885605695,
        frequency: 57375.80190084620,
    },
    Vsop87Term {
        amplitude: 0.00000000142,
        phase: 0.78678347839,
        frequency: 12779.45079542080,
    },
    Vsop87Term {
        amplitude: 0.00000000134,
        phase: 4.79432636012,
        frequency: 111.18664228760,
    },
    Vsop87Term {
        amplitude: 0.00000000140,
        phase: 1.27748013377,
        frequency: 107.66352393860,
    },
    Vsop87Term {
        amplitude: 0.00000000169,
        phase: 2.74893543762,
        frequency: 26735.94526221320,
    },
    Vsop87Term {
        amplitude: 0.00000000165,
        phase: 3.95288000638,
        frequency: 6357.85744855870,
    },
    Vsop87Term {
        amplitude: 0.00000000183,
        phase: 5.43418358741,
        frequency: 369.69981594040,
    },
    Vsop87Term {
        amplitude: 0.00000000134,
        phase: 3.09132862833,
        frequency: 17.81252211800,
    },
    Vsop87Term {
        amplitude: 0.00000000132,
        phase: 3.05633896779,
        frequency: 22490.96212149340,
    },
    Vsop87Term {
        amplitude: 0.00000000134,
        phase: 4.09472795832,
        frequency: 6599.46771964800,
    },
    Vsop87Term {
        amplitude: 0.00000000181,
        phase: 4.22950689891,
        frequency: 966.97087743560,
    },
    Vsop87Term {
        amplitude: 0.00000000152,
        phase: 5.28885894415,
        frequency: 12669.24447420140,
    },
    Vsop87Term {
        amplitude: 0.00000000150,
        phase: 5.86819430908,
        frequency: 97238.62754448749,
    },
    Vsop87Term {
        amplitude: 0.00000000142,
        phase: 5.87266532526,
        frequency: 22476.73502749179,
    },
    Vsop87Term {
        amplitude: 0.00000000145,
        phase: 5.07330784304,
        frequency: 87.30820453981,
    },
    Vsop87Term {
        amplitude: 0.00000000133,
        phase: 5.65471067133,
        frequency: 31.97230581680,
    },
    Vsop87Term {
        amplitude: 0.00000000124,
        phase: 2.83326217072,
        frequency: 12566.21901028560,
    },
    Vsop87Term {
        amplitude: 0.00000000135,
        phase: 3.12861731644,
        frequency: 32217.20018108080,
    },
    Vsop87Term {
        amplitude: 0.00000000137,
        phase: 0.86487461904,
        frequency: 9924.81042151060,
    },
    Vsop87Term {
        amplitude: 0.00000000172,
        phase: 1.98369595114,
        frequency: 174242.46596404970,
    },
    Vsop87Term {
        amplitude: 0.00000000170,
        phase: 4.41115280254,
        frequency: 327574.51427678125,
    },
    Vsop87Term {
        amplitude: 0.00000000151,
        phase: 0.46542099527,
        frequency: 39609.65458316560,
    },
    Vsop87Term {
        amplitude: 0.00000000148,
        phase: 2.13439571118,
        frequency: 491.66329245880,
    },
    Vsop87Term {
        amplitude: 0.00000000153,
        phase: 3.78801830344,
        frequency: 17363.24742890899,
    },
    Vsop87Term {
        amplitude: 0.00000000165,
        phase: 5.31654110459,
        frequency: 16943.76278503380,
    },
    Vsop87Term {
        amplitude: 0.00000000165,
        phase: 4.06747587817,
        frequency: 58953.14544329400,
    },
    Vsop87Term {
        amplitude: 0.00000000118,
        phase: 0.63846333239,
        frequency: 6.06591562980,
    },
    Vsop87Term {
        amplitude: 0.00000000159,
        phase: 0.86086959274,
        frequency: 221995.02880149524,
    },
    Vsop87Term {
        amplitude: 0.00000000119,
        phase: 5.96432932413,
        frequency: 1385.89527633620,
    },
    Vsop87Term {
        amplitude: 0.00000000114,
        phase: 5.16516114595,
        frequency: 25685.87280280800,
    },
    Vsop87Term {
        amplitude: 0.00000000112,
        phase: 3.39403722178,
        frequency: 21393.54196985760,
    },
    Vsop87Term {
        amplitude: 0.00000000112,
        phase: 4.92889233335,
        frequency: 56.80326216980,
    },
    Vsop87Term {
        amplitude: 0.00000000119,
        phase: 2.40637635942,
        frequency: 18635.92845453620,
    },
    Vsop87Term {
        amplitude: 0.00000000115,
        phase: 0.23374479051,
        frequency: 418.92439890060,
    },
    Vsop87Term {
        amplitude: 0.00000000122,
        phase: 0.93575234049,
        frequency: 24492.40611365159,
    },
    Vsop87Term {
        amplitude: 0.00000000115,
        phase: 4.58880032176,
        frequency: 26709.64694241340,
    },
    Vsop87Term {
        amplitude: 0.00000000130,
        phase: 4.85539251000,
        frequency: 22345.26037610820,
    },
    Vsop87Term {
        amplitude: 0.00000000140,
        phase: 1.09413073202,
        frequency: 44809.65020086340,
    },
    Vsop87Term {
        amplitude: 0.00000000112,
        phase: 6.05401806281,
        frequency: 433.71173787680,
    },
    Vsop87Term {
        amplitude: 0.00000000104,
        phase: 1.54931540602,
        frequency: 127.95153303460,
    },
    Vsop87Term {
        amplitude: 0.00000000105,
        phase: 4.82620858888,
        frequency: 33794.54372352860,
    },
    Vsop87Term {
        amplitude: 0.00000000102,
        phase: 4.12448497391,
        frequency: 15664.03552270859,
    },
    Vsop87Term {
        amplitude: 0.00000000107,
        phase: 4.67919356465,
        frequency: 77690.75950573849,
    },
    Vsop87Term {
        amplitude: 0.00000000118,
        phase: 4.52320170120,
        frequency: 19004.64794940840,
    },
    Vsop87Term {
        amplitude: 0.00000000107,
        phase: 5.71774478555,
        frequency: 77736.78343050249,
    },
    Vsop87Term {
        amplitude: 0.00000000143,
        phase: 1.81201813018,
        frequency: 4214.06901508480,
    },
    Vsop87Term {
        amplitude: 0.00000000125,
        phase: 1.14419195615,
        frequency: 625.67019231240,
    },
    Vsop87Term {
        amplitude: 0.00000000124,
        phase: 3.27736514057,
        frequency: 12566.08438968000,
    },
    Vsop87Term {
        amplitude: 0.00000000110,
        phase: 1.08682570828,
        frequency: 2787.04302385740,
    },
    Vsop87Term {
        amplitude: 0.00000000105,
        phase: 1.78318141871,
        frequency: 18139.29450141590,
    },
    Vsop87Term {
        amplitude: 0.00000000102,
        phase: 4.75119578149,
        frequency: 12242.64628332540,
    },
    Vsop87Term {
        amplitude: 0.00000000137,
        phase: 1.43510636754,
        frequency: 86464.61331683119,
    },
    Vsop87Term {
        amplitude: 0.00000000101,
        phase: 4.91289409429,
        frequency: 401.67212175720,
    },
    Vsop87Term {
        amplitude: 0.00000000129,
        phase: 1.23567904485,
        frequency: 12029.34718788740,
    },
    Vsop87Term {
        amplitude: 0.00000000138,
        phase: 2.45654707999,
        frequency: 7576.56007357400,
    },
    Vsop87Term {
        amplitude: 0.00000000103,
        phase: 0.40004073416,
        frequency: 90279.92316810328,
    },
    Vsop87Term {
        amplitude: 0.00000000108,
        phase: 0.98989774940,
        frequency: 5636.06501667660,
    },
    Vsop87Term {
        amplitude: 0.00000000117,
        phase: 5.17362872063,
        frequency: 34520.30930938080,
    },
    Vsop87Term {
        amplitude: 0.00000000100,
        phase: 3.95534628189,
        frequency: 5547.19933645960,
    },
    Vsop87Term {
        amplitude: 0.00000000098,
        phase: 1.28118280598,
        frequency: 21548.96236929180,
    },
    Vsop87Term {
        amplitude: 0.00000000097,
        phase: 3.34717130592,
        frequency: 16310.97904572060,
    },
    Vsop87Term {
        amplitude: 0.00000000098,
        phase: 4.37041908717,
        frequency: 34513.26307268280,
    },
    Vsop87Term {
        amplitude: 0.00000000125,
        phase: 2.72164432960,
        frequency: 24065.80792277559,
    },
    Vsop87Term {
        amplitude: 0.00000000102,
        phase: 0.66938025772,
        frequency: 10239.58386601080,
    },
    Vsop87Term {
        amplitude: 0.00000000119,
        phase: 1.21689479331,
        frequency: 1478.86657406440,
    },
    Vsop87Term {
        amplitude: 0.00000000094,
        phase: 1.99595224256,
        frequency: 13362.44970679920,
    },
    Vsop87Term {
        amplitude: 0.00000000094,
        phase: 4.30965982872,
        frequency: 26880.31981303260,
    },
    Vsop87Term {
        amplitude: 0.00000000095,
        phase: 2.89807657534,
        frequency: 34911.41207609100,
    },
    Vsop87Term {
        amplitude: 0.00000000106,
        phase: 1.00156653590,
        frequency: 16522.65971600220,
    },
    Vsop87Term {
        amplitude: 0.00000000097,
        phase: 0.89642320201,
        frequency: 71980.63357473118,
    },
    Vsop87Term {
        amplitude: 0.00000000116,
        phase: 4.19967201116,
        frequency: 206.70073729660,
    },
    Vsop87Term {
        amplitude: 0.00000000099,
        phase: 1.37437847718,
        frequency: 1039.02661079040,
    },
    Vsop87Term {
        amplitude: 0.00000000126,
        phase: 3.21642544972,
        frequency: 305281.94307104882,
    },
    Vsop87Term {
        amplitude: 0.00000000094,
        phase: 0.68997876060,
        frequency: 7834.12107263940,
    },
    Vsop87Term {
        amplitude: 0.00000000094,
        phase: 5.58132218606,
        frequency: 3104.93005942380,
    },
    Vsop87Term {
        amplitude: 0.00000000095,
        phase: 3.03823741110,
        frequency: 8982.81066930900,
    },
    Vsop87Term {
        amplitude: 0.00000000108,
        phase: 0.52696637156,
        frequency: 276.74577186440,
    },
    Vsop87Term {
        amplitude: 0.00000000124,
        phase: 3.43899862683,
        frequency: 172146.97134054029,
    },
    Vsop87Term {
        amplitude: 0.00000000102,
        phase: 1.04031728553,
        frequency: 95143.13292097810,
    },
    Vsop87Term {
        amplitude: 0.00000000104,
        phase: 3.39218586218,
        frequency: 290.97286586600,
    },
    Vsop87Term {
        amplitude: 0.00000000110,
        phase: 3.68205877433,
        frequency: 22380.75580027400,
    },
    Vsop87Term {
        amplitude: 0.00000000117,
        phase: 0.78475956902,
        frequency: 83286.91426955358,
    },
    Vsop87Term {
        amplitude: 0.00000000083,
        phase: 0.18241793425,
        frequency: 15141.39079431200,
    },
    Vsop87Term {
        amplitude: 0.00000000089,
        phase: 4.45371820659,
        frequency: 792.77488846740,
    },
    Vsop87Term {
        amplitude: 0.00000000082,
        phase: 4.80703651241,
        frequency: 6819.88036208680,
    },
    Vsop87Term {
        amplitude: 0.00000000087,
        phase: 3.43122851097,
        frequency: 27707.54249429480,
    },
    Vsop87Term {
        amplitude: 0.00000000101,
        phase: 5.32081603011,
        frequency: 2301.58581590939,
    },
    Vsop87Term {
        amplitude: 0.00000000082,
        phase: 0.87060089842,
        frequency: 10241.20229116720,
    },
    Vsop87Term {
        amplitude: 0.00000000086,
        phase: 4.61919461931,
        frequency: 36147.40987730040,
    },
    Vsop87Term {
        amplitude: 0.00000000095,
        phase: 2.87032884659,
        frequency: 23020.65308658799,
    },
    Vsop87Term {
        amplitude: 0.00000000088,
        phase: 3.21133165690,
        frequency: 33326.57873317420,
    },
    Vsop87Term {
        amplitude: 0.00000000080,
        phase: 1.84900424847,
        frequency: 21424.46664430340,
    },
    Vsop87Term {
        amplitude: 0.00000000101,
        phase: 4.18796434479,
        frequency: 30666.15495843280,
    },
    Vsop87Term {
        amplitude: 0.00000000107,
        phase: 5.77864921649,
        frequency: 34115.11406927460,
    },
    Vsop87Term {
        amplitude: 0.00000000104,
        phase: 1.08739495962,
        frequency: 6288.59877429880,
    },
    Vsop87Term {
        amplitude: 0.00000000110,
        phase: 3.32898859416,
        frequency: 72140.62866668739,
    },
    Vsop87Term {
        amplitude: 0.00000000087,
        phase: 4.40657711727,
        frequency: 142.17862703620,
    },
    Vsop87Term {
        amplitude: 0.00000000109,
        phase: 1.94546030825,
        frequency: 24279.10701821359,
    },
    Vsop87Term {
        amplitude: 0.00000000087,
        phase: 4.32472045435,
        frequency: 742.99006053260,
    },
    Vsop87Term {
        amplitude: 0.00000000107,
        phase: 4.91580912547,
        frequency: 277.03499374140,
    },
    Vsop87Term {
        amplitude: 0.00000000088,
        phase: 2.10180220766,
        frequency: 26482.17080962440,
    },
    Vsop87Term {
        amplitude: 0.00000000086,
        phase: 4.01887374432,
        frequency: 12491.37010141550,
    },
    Vsop87Term {
        amplitude: 0.00000000106,
        phase: 5.49092372854,
        frequency: 62883.35513951360,
    },
    Vsop87Term {
        amplitude: 0.00000000080,
        phase: 6.19781316983,
        frequency: 6709.67404086740,
    },
    Vsop87Term {
        amplitude: 0.00000000088,
        phase: 2.09872810657,
        frequency: 238004.52415723629,
    },
    Vsop87Term {
        amplitude: 0.00000000083,
        phase: 4.90662164029,
        frequency: 51.28033786241,
    },
    Vsop87Term {
        amplitude: 0.00000000095,
        phase: 4.13387406591,
        frequency: 18216.44381066100,
    },
    Vsop87Term {
        amplitude: 0.00000000078,
        phase: 6.06949391680,
        frequency: 148434.53403769129,
    },
    Vsop87Term {
        amplitude: 0.00000000079,
        phase: 3.03048221644,
        frequency: 838.96928775040,
    },
    Vsop87Term {
        amplitude: 0.00000000074,
        phase: 5.49813051211,
        frequency: 29026.48522950779,
    },
    Vsop87Term {
        amplitude: 0.00000000073,
        phase: 3.05008665738,
        frequency: 567.71863773040,
    },
    Vsop87Term {
        amplitude: 0.00000000084,
        phase: 0.46604373274,
        frequency: 45.14121963660,
    },
    Vsop87Term {
        amplitude: 0.00000000093,
        phase: 2.52267536308,
        frequency: 48739.85989708300,
    },
    Vsop87Term {
        amplitude: 0.00000000076,
        phase: 1.76418124905,
        frequency: 41654.96311596780,
    },
    Vsop87Term {
        amplitude: 0.00000000067,
        phase: 5.77851227793,
        frequency: 6311.52503745920,
    },
    Vsop87Term {
        amplitude: 0.00000000062,
        phase: 3.32967880172,
        frequency: 15508.61512327440,
    },
    Vsop87Term {
        amplitude: 0.00000000079,
        phase: 5.59773841328,
        frequency: 71960.38658322369,
    },
    Vsop87Term {
        amplitude: 0.00000000057,
        phase: 3.90629505268,
        frequency: 5999.21653112620,
    },
    Vsop87Term {
        amplitude: 0.00000000061,
        phase: 0.05695043232,
        frequency: 7856.89627409019,
    },
    Vsop87Term {
        amplitude: 0.00000000061,
        phase: 5.63297958433,
        frequency: 7863.94251078820,
    },
    Vsop87Term {
        amplitude: 0.00000000065,
        phase: 3.72178394016,
        frequency: 12573.26524698360,
    },
    Vsop87Term {
        amplitude: 0.00000000057,
        phase: 4.18217219541,
        frequency: 26087.90314157420,
    },
    Vsop87Term {
        amplitude: 0.00000000066,
        phase: 3.92262333487,
        frequency: 69853.35207568129,
    },
    Vsop87Term {
        amplitude: 0.00000000053,
        phase: 5.51119362045,
        frequency: 77710.24834977149,
    },
    Vsop87Term {
        amplitude: 0.00000000053,
        phase: 4.88573986961,
        frequency: 77717.29458646949,
    },
    Vsop87Term {
        amplitude: 0.00000000062,
        phase: 2.88876342225,
        frequency: 9411.46461508720,
    },
    Vsop87Term {
        amplitude: 0.00000000051,
        phase: 1.12657183874,
        frequency: 82576.98122099529,
    },
    Vsop87Term {
        amplitude: 0.00000000045,
        phase: 2.95671076719,
        frequency: 24602.61243487099,
    },
    Vsop87Term {
        amplitude: 0.00000000040,
        phase: 5.55145719241,
        frequency: 12565.17137891460,
    },
    Vsop87Term {
        amplitude: 0.00000000039,
        phase: 1.20838190039,
        frequency: 18842.11400297339,
    },
    Vsop87Term {
        amplitude: 0.00000000045,
        phase: 3.18590558749,
        frequency: 45585.17281218740,
    },
    Vsop87Term {
        amplitude: 0.00000000049,
        phase: 2.44790934886,
        frequency: 13613.80427733600,
    },
];

pub(super) const L1: &[Vsop87Term] = &[
    Vsop87Term {
        amplitude: 6283.31966747491,
        phase: 0.00000000000,
        frequency: 0.00000000000,
    },
    Vsop87Term {
        amplitude: 0.00206058863,
        phase: 2.67823455584,
        frequency: 6283.07584999140,
    },
    Vsop87Term {
        amplitude: 0.00004303430,
        phase: 2.63512650414,
        frequency: 12566.15169998280,
    },
    Vsop87Term {
        amplitude: 0.00000425264,
        phase: 1.59046980729,
        frequency: 3.52311834900,
    },
    Vsop87Term {
        amplitude: 0.00000108977,
        phase: 2.96618001993,
        frequency: 1577.34354244780,
    },
    Vsop87Term {
        amplitude: 0.00000093478,
        phase: 2.59212835365,
        frequency: 18849.22754997420,
    },
    Vsop87Term {
        amplitude: 0.00000119261,
        phase: 5.79557487799,
        frequency: 26.29831979980,
    },
    Vsop87Term {
        amplitude: 0.00000072122,
        phase: 1.13846158196,
        frequency: 529.69096509460,
    },
    Vsop87Term {
        amplitude: 0.00000067768,
        phase: 1.87472304791,
        frequency: 398.14900340820,
    },
    Vsop87Term {
        amplitude: 0.00000067327,
        phase: 4.40918235168,
        frequency: 5507.55323866740,
    },
    Vsop87Term {
        amplitude: 0.00000059027,
        phase: 2.88797038460,
        frequency: 5223.69391980220,
    },
    Vsop87Term {
        amplitude: 0.00000055976,
        phase: 2.17471680261,
        frequency: 155.42039943420,
    },
    Vsop87Term {
        amplitude: 0.00000045407,
        phase: 0.39803079805,
        frequency: 796.29800681640,
    },
    Vsop87Term {
        amplitude: 0.00000036369,
        phase: 0.46624739835,
        frequency: 775.52261132400,
    },
    Vsop87Term {
        amplitude: 0.00000028958,
        phase: 2.64707383882,
        frequency: 7.11354700080,
    },
    Vsop87Term {
        amplitude: 0.00000019097,
        phase: 1.84628332577,
        frequency: 5486.77784317500,
    },
    Vsop87Term {
        amplitude: 0.00000020844,
        phase: 5.34138275149,
        frequency: 0.98032106820,
    },
    Vsop87Term {
        amplitude: 0.00000018508,
        phase: 4.96855124577,
        frequency: 213.29909543800,
    },
    Vsop87Term {
        amplitude: 0.00000016233,
        phase: 0.03216483047,
        frequency: 2544.31441988340,
    },
    Vsop87Term {
        amplitude: 0.00000017293,
        phase: 2.99116864949,
        frequency: 6275.96230299060,
    },
    Vsop87Term {
        amplitude: 0.00000015832,
        phase: 1.43049285325,
        frequency: 2146.16541647520,
    },
    Vsop87Term {
        amplitude: 0.00000014615,
        phase: 1.20532366323,
        frequency: 10977.07880469900,
    },
    Vsop87Term {
        amplitude: 0.00000011877,
        phase: 3.25804815607,
        frequency: 5088.62883976680,
    },
    Vsop87Term {
        amplitude: 0.00000011514,
        phase: 2.07502418155,
        frequency: 4694.00295470760,
    },
    Vsop87Term {
        amplitude: 0.00000009721,
        phase: 4.23925472239,
        frequency: 1349.86740965880,
    },
    Vsop87Term {
        amplitude: 0.00000009969,
        phase: 1.30262991097,
        frequency: 6286.59896834040,
    },
    Vsop87Term {
        amplitude: 0.00000009452,
        phase: 2.69957062864,
        frequency: 242.72860397400,
    },
    Vsop87Term {
        amplitude: 0.00000012461,
        phase: 2.83432285512,
        frequency: 1748.01641306700,
    },
    Vsop87Term {
        amplitude: 0.00000011808,
        phase: 5.27379790480,
        frequency: 1194.44701022460,
    },
    Vsop87Term {
        amplitude: 0.00000008577,
        phase: 5.64475868067,
        frequency: 951.71840625060,
    },
    Vsop87Term {
        amplitude: 0.00000010641,
        phase: 0.76614199202,
        frequency: 553.56940284240,
    },
    Vsop87Term {
        amplitude: 0.00000007576,
        phase: 5.30062664886,
        frequency: 2352.86615377180,
    },
    Vsop87Term {
        amplitude: 0.00000005834,
        phase: 1.76649917904,
        frequency: 1059.38193018920,
    },
    Vsop87Term {
        amplitude: 0.00000006385,
        phase: 2.65033984967,
        frequency: 9437.76293488700,
    },
    Vsop87Term {
        amplitude: 0.00000005223,
        phase: 5.66135767624,
        frequency: 71430.69561812909,
    },
    Vsop87Term {
        amplitude: 0.00000005305,
        phase: 0.90857521574,
        frequency: 3154.68708489560,
    },
    Vsop87Term {
        amplitude: 0.00000006101,
        phase: 4.66632584188,
        frequency: 4690.47983635860,
    },
    Vsop87Term {
        amplitude: 0.00000004330,
        phase: 0.24102555403,
        frequency: 6812.76681508600,
    },
    Vsop87Term {
        amplitude: 0.00000005041,
        phase: 1.42490103709,
        frequency: 6438.49624942560,
    },
    Vsop87Term {
        amplitude: 0.00000004259,
        phase: 0.77355900599,
        frequency: 10447.38783960440,
    },
    Vsop87Term {
        amplitude: 0.00000005198,
        phase: 1.85353197345,
        frequency: 801.82093112380,
    },
    Vsop87Term {
        amplitude: 0.00000003744,
        phase: 2.00119516488,
        frequency: 8031.09226305840,
    },
    Vsop87Term {
        amplitude: 0.00000003558,
        phase: 2.42901552681,
        frequency: 14143.49524243060,
    },
    Vsop87Term {
        amplitude: 0.00000003372,
        phase: 3.86210700128,
        frequency: 1592.59601363280,
    },
    Vsop87Term {
        amplitude: 0.00000003374,
        phase: 0.88776219727,
        frequency: 12036.46073488820,
    },
    Vsop87Term {
        amplitude: 0.00000003175,
        phase: 3.18785710594,
        frequency: 4705.73230754360,
    },
    Vsop87Term {
        amplitude: 0.00000003221,
        phase: 0.61599835472,
        frequency: 8429.24126646660,
    },
    Vsop87Term {
        amplitude: 0.00000004132,
        phase: 5.23992859705,
        frequency: 7084.89678111520,
    },
    Vsop87Term {
        amplitude: 0.00000002970,
        phase: 6.07026318493,
        frequency: 4292.33083295040,
    },
    Vsop87Term {
        amplitude: 0.00000002900,
        phase: 2.32464208411,
        frequency: 20.35531939880,
    },
    Vsop87Term {
        amplitude: 0.00000003504,
        phase: 4.79975694359,
        frequency: 6279.55273164240,
    },
    Vsop87Term {
        amplitude: 0.00000002950,
        phase: 1.43108874817,
        frequency: 5746.27133789600,
    },
    Vsop87Term {
        amplitude: 0.00000002697,
        phase: 4.80368225199,
        frequency: 7234.79425624200,
    },
    Vsop87Term {
        amplitude: 0.00000002531,
        phase: 6.22290682655,
        frequency: 6836.64525283380,
    },
    Vsop87Term {
        amplitude: 0.00000002745,
        phase: 0.93466065396,
        frequency: 5760.49843189760,
    },
    Vsop87Term {
        amplitude: 0.00000003250,
        phase: 3.39954640038,
        frequency: 7632.94325965020,
    },
    Vsop87Term {
        amplitude: 0.00000002277,
        phase: 5.00277837672,
        frequency: 17789.84561978500,
    },
    Vsop87Term {
        amplitude: 0.00000002075,
        phase: 3.95534978634,
        frequency: 10213.28554621100,
    },
    Vsop87Term {
        amplitude: 0.00000002061,
        phase: 2.22411683077,
        frequency: 5856.47765911540,
    },
    Vsop87Term {
        amplitude: 0.00000002252,
        phase: 5.67166499885,
        frequency: 11499.65622279280,
    },
    Vsop87Term {
        amplitude: 0.00000002148,
        phase: 5.20184578235,
        frequency: 11513.88331679440,
    },
    Vsop87Term {
        amplitude: 0.00000001886,
        phase: 0.53198320577,
        frequency: 3340.61242669980,
    },
    Vsop87Term {
        amplitude: 0.00000001875,
        phase: 4.73511970207,
        frequency: 83996.84731811189,
    },
    Vsop87Term {
        amplitude: 0.00000002060,
        phase: 2.54987293999,
        frequency: 25132.30339996560,
    },
    Vsop87Term {
        amplitude: 0.00000001794,
        phase: 1.47435409831,
        frequency: 4164.31198961300,
    },
    Vsop87Term {
        amplitude: 0.00000001778,
        phase: 3.02473091781,
        frequency: 5.52292430740,
    },
    Vsop87Term {
        amplitude: 0.00000002029,
        phase: 0.90960209983,
        frequency: 6256.77753019160,
    },
    Vsop87Term {
        amplitude: 0.00000002075,
        phase: 2.26767270157,
        frequency: 522.57741809380,
    },
    Vsop87Term {
        amplitude: 0.00000001772,
        phase: 3.02622802353,
        frequency: 5753.38488489680,
    },
    Vsop87Term {
        amplitude: 0.00000001569,
        phase: 6.12410242782,
        frequency: 5216.58037280140,
    },
    Vsop87Term {
        amplitude: 0.00000001590,
        phase: 4.63713748247,
        frequency: 3.28635741780,
    },
    Vsop87Term {
        amplitude: 0.00000001542,
        phase: 4.20004448567,
        frequency: 13367.97263110660,
    },
    Vsop87Term {
        amplitude: 0.00000001427,
        phase: 1.19088061711,
        frequency: 3894.18182954220,
    },
    Vsop87Term {
        amplitude: 0.00000001375,
        phase: 3.09301252193,
        frequency: 135.06508003540,
    },
    Vsop87Term {
        amplitude: 0.00000001359,
        phase: 4.24532506641,
        frequency: 426.59819087600,
    },
    Vsop87Term {
        amplitude: 0.00000001340,
        phase: 5.76511818622,
        frequency: 6040.34724601740,
    },
    Vsop87Term {
        amplitude: 0.00000001284,
        phase: 3.08524663344,
        frequency: 5643.17856367740,
    },
    Vsop87Term {
        amplitude: 0.00000001250,
        phase: 3.07748157144,
        frequency: 11926.25441366880,
    },
    Vsop87Term {
        amplitude: 0.00000001551,
        phase: 3.07665451458,
        frequency: 6681.22485339960,
    },
    Vsop87Term {
        amplitude: 0.00000001268,
        phase: 2.09196018331,
        frequency: 6290.18939699220,
    },
    Vsop87Term {
        amplitude: 0.00000001144,
        phase: 3.24444699514,
        frequency: 12168.00269657460,
    },
    Vsop87Term {
        amplitude: 0.00000001248,
        phase: 3.44504937285,
        frequency: 536.80451209540,
    },
    Vsop87Term {
        amplitude: 0.00000001118,
        phase: 2.31829670425,
        frequency: 16730.46368959580,
    },
    Vsop87Term {
        amplitude: 0.00000001105,
        phase: 5.31966001019,
        frequency: 23.87843774780,
    },
    Vsop87Term {
        amplitude: 0.00000001051,
        phase: 3.75015946014,
        frequency: 7860.41939243920,
    },
    Vsop87Term {
        amplitude: 0.00000001025,
        phase: 2.44688534235,
        frequency: 1990.74501704100,
    },
    Vsop87Term {
        amplitude: 0.00000000962,
        phase: 0.81771017882,
        frequency: 3.88133535800,
    },
    Vsop87Term {
        amplitude: 0.00000000910,
        phase: 0.41727865299,
        frequency: 7079.37385680780,
    },
    Vsop87Term {
        amplitude: 0.00000000883,
        phase: 5.16833917651,
        frequency: 11790.62908865880,
    },
    Vsop87Term {
        amplitude: 0.00000000957,
        phase: 4.07673573735,
        frequency: 6127.65545055720,
    },
    Vsop87Term {
        amplitude: 0.00000001110,
        phase: 3.90096793825,
        frequency: 11506.76976979360,
    },
    Vsop87Term {
        amplitude: 0.00000000802,
        phase: 3.88778875582,
        frequency: 10973.55568635000,
    },
    Vsop87Term {
        amplitude: 0.00000000780,
        phase: 2.39934293755,
        frequency: 1589.07289528380,
    },
    Vsop87Term {
        amplitude: 0.00000000758,
        phase: 1.30034364248,
        frequency: 103.09277421860,
    },
    Vsop87Term {
        amplitude: 0.00000000749,
        phase: 4.96275803300,
        frequency: 6496.37494542940,
    },
    Vsop87Term {
        amplitude: 0.00000000765,
        phase: 3.36312388424,
        frequency: 36.02786667740,
    },
    Vsop87Term {
        amplitude: 0.00000000915,
        phase: 5.41543742089,
        frequency: 206.18554843720,
    },
    Vsop87Term {
        amplitude: 0.00000000776,
        phase: 2.57589093871,
        frequency: 11371.70468975820,
    },
    Vsop87Term {
        amplitude: 0.00000000772,
        phase: 3.98369209464,
        frequency: 955.59974160860,
    },
    Vsop87Term {
        amplitude: 0.00000000749,
        phase: 5.17890001805,
        frequency: 10969.96525769820,
    },
    Vsop87Term {
        amplitude: 0.00000000806,
        phase: 0.34218864254,
        frequency: 9917.69687450980,
    },
    Vsop87Term {
        amplitude: 0.00000000728,
        phase: 5.20962563787,
        frequency: 38.02767263580,
    },
    Vsop87Term {
        amplitude: 0.00000000685,
        phase: 2.77592961854,
        frequency: 20.77539549240,
    },
    Vsop87Term {
        amplitude: 0.00000000636,
        phase: 4.28242193632,
        frequency: 28.44918746780,
    },
    Vsop87Term {
        amplitude: 0.00000000608,
        phase: 5.63278508906,
        frequency: 10984.19235169980,
    },
    Vsop87Term {
        amplitude: 0.00000000704,
        phase: 5.60738823665,
        frequency: 3738.76143010800,
    },
    Vsop87Term {
        amplitude: 0.00000000685,
        phase: 0.38876148682,
        frequency: 15.25247118500,
    },
    Vsop87Term {
        amplitude: 0.00000000601,
        phase: 0.73489602442,
        frequency: 419.48464387520,
    },
    Vsop87Term {
        amplitude: 0.00000000716,
        phase: 2.65279791438,
        frequency: 6309.37416979120,
    },
    Vsop87Term {
        amplitude: 0.00000000584,
        phase: 5.54502568227,
        frequency: 17298.18232732620,
    },
    Vsop87Term {
        amplitude: 0.00000000650,
        phase: 1.13379656406,
        frequency: 7058.59846131540,
    },
    Vsop87Term {
        amplitude: 0.00000000688,
        phase: 2.59683891779,
        frequency: 3496.03282613400,
    },
    Vsop87Term {
        amplitude: 0.00000000485,
        phase: 0.44467180946,
        frequency: 12352.85260454480,
    },
    Vsop87Term {
        amplitude: 0.00000000528,
        phase: 2.74936967681,
        frequency: 3930.20969621960,
    },
    Vsop87Term {
        amplitude: 0.00000000597,
        phase: 5.27668281777,
        frequency: 10575.40668294180,
    },
    Vsop87Term {
        amplitude: 0.00000000583,
        phase: 3.18929067810,
        frequency: 4732.03062734340,
    },
    Vsop87Term {
        amplitude: 0.00000000526,
        phase: 5.01697321546,
        frequency: 5884.92684658320,
    },
    Vsop87Term {
        amplitude: 0.00000000540,
        phase: 1.29175137075,
        frequency: 640.87760738220,
    },
    Vsop87Term {
        amplitude: 0.00000000473,
        phase: 5.49953306970,
        frequency: 5230.80746680300,
    },
    Vsop87Term {
        amplitude: 0.00000000406,
        phase: 5.21248452189,
        frequency: 220.41264243880,
    },
    Vsop87Term {
        amplitude: 0.00000000395,
        phase: 1.87474483222,
        frequency: 16200.77272450120,
    },
    Vsop87Term {
        amplitude: 0.00000000370,
        phase: 3.84921354713,
        frequency: 18073.70493865020,
    },
    Vsop87Term {
        amplitude: 0.00000000367,
        phase: 0.88533542778,
        frequency: 6283.14316029419,
    },
    Vsop87Term {
        amplitude: 0.00000000379,
        phase: 0.37983009325,
        frequency: 10177.25767953360,
    },
    Vsop87Term {
        amplitude: 0.00000000356,
        phase: 3.84145204913,
        frequency: 11712.95531823080,
    },
    Vsop87Term {
        amplitude: 0.00000000374,
        phase: 5.01577520608,
        frequency: 7.04623669800,
    },
    Vsop87Term {
        amplitude: 0.00000000381,
        phase: 4.30250406634,
        frequency: 6062.66320755260,
    },
    Vsop87Term {
        amplitude: 0.00000000471,
        phase: 0.86381834647,
        frequency: 6069.77675455340,
    },
    Vsop87Term {
        amplitude: 0.00000000367,
        phase: 1.32943839763,
        frequency: 6283.00853968860,
    },
    Vsop87Term {
        amplitude: 0.00000000460,
        phase: 5.19667219575,
        frequency: 6284.05617105960,
    },
    Vsop87Term {
        amplitude: 0.00000000333,
        phase: 5.54256205741,
        frequency: 4686.88940770680,
    },
    Vsop87Term {
        amplitude: 0.00000000341,
        phase: 4.36522989934,
        frequency: 7238.67559160000,
    },
    Vsop87Term {
        amplitude: 0.00000000336,
        phase: 4.00205876835,
        frequency: 3097.88382272579,
    },
    Vsop87Term {
        amplitude: 0.00000000359,
        phase: 6.22679790284,
        frequency: 245.83164622940,
    },
    Vsop87Term {
        amplitude: 0.00000000307,
        phase: 2.35299010924,
        frequency: 170.67287061920,
    },
    Vsop87Term {
        amplitude: 0.00000000343,
        phase: 3.77164927143,
        frequency: 6076.89030155420,
    },
    Vsop87Term {
        amplitude: 0.00000000296,
        phase: 5.44152227481,
        frequency: 17260.15465469040,
    },
    Vsop87Term {
        amplitude: 0.00000000328,
        phase: 0.13837875384,
        frequency: 11015.10647733480,
    },
    Vsop87Term {
        amplitude: 0.00000000268,
        phase: 1.13904550630,
        frequency: 12569.67481833180,
    },
    Vsop87Term {
        amplitude: 0.00000000263,
        phase: 0.00538633678,
        frequency: 4136.91043351620,
    },
    Vsop87Term {
        amplitude: 0.00000000282,
        phase: 5.04399837480,
        frequency: 7477.52286021600,
    },
    Vsop87Term {
        amplitude: 0.00000000288,
        phase: 3.13401177517,
        frequency: 12559.03815298200,
    },
    Vsop87Term {
        amplitude: 0.00000000259,
        phase: 0.93882269387,
        frequency: 5642.19824260920,
    },
    Vsop87Term {
        amplitude: 0.00000000292,
        phase: 1.98420020514,
        frequency: 12132.43996210600,
    },
    Vsop87Term {
        amplitude: 0.00000000247,
        phase: 3.84244798532,
        frequency: 5429.87946823940,
    },
    Vsop87Term {
        amplitude: 0.00000000245,
        phase: 5.70467521726,
        frequency: 65147.61976813770,
    },
    Vsop87Term {
        amplitude: 0.00000000241,
        phase: 0.99480969552,
        frequency: 3634.62102451840,
    },
    Vsop87Term {
        amplitude: 0.00000000246,
        phase: 3.06168069935,
        frequency: 110.20632121940,
    },
    Vsop87Term {
        amplitude: 0.00000000239,
        phase: 6.11855909114,
        frequency: 11856.21865142450,
    },
    Vsop87Term {
        amplitude: 0.00000000263,
        phase: 0.66348415419,
        frequency: 21228.39202354580,
    },
    Vsop87Term {
        amplitude: 0.00000000262,
        phase: 1.51070507866,
        frequency: 12146.66705610760,
    },
    Vsop87Term {
        amplitude: 0.00000000230,
        phase: 1.75927314884,
        frequency: 9779.10867612540,
    },
    Vsop87Term {
        amplitude: 0.00000000223,
        phase: 2.00967043606,
        frequency: 6172.86952877200,
    },
    Vsop87Term {
        amplitude: 0.00000000246,
        phase: 1.10411690865,
        frequency: 6282.09552892320,
    },
    Vsop87Term {
        amplitude: 0.00000000221,
        phase: 3.03945240854,
        frequency: 8635.94200376320,
    },
    Vsop87Term {
        amplitude: 0.00000000214,
        phase: 4.03840869663,
        frequency: 14314.16811304980,
    },
    Vsop87Term {
        amplitude: 0.00000000236,
        phase: 5.46915070580,
        frequency: 13916.01910964160,
    },
    Vsop87Term {
        amplitude: 0.00000000224,
        phase: 4.68408089456,
        frequency: 24072.92146977640,
    },
    Vsop87Term {
        amplitude: 0.00000000212,
        phase: 2.13695625494,
        frequency: 5849.36411211460,
    },
    Vsop87Term {
        amplitude: 0.00000000207,
        phase: 3.07724246401,
        frequency: 11.72935283600,
    },
    Vsop87Term {
        amplitude: 0.00000000207,
        phase: 6.10306282747,
        frequency: 23543.23050468179,
    },
    Vsop87Term {
        amplitude: 0.00000000266,
        phase: 1.00709566823,
        frequency: 2388.89402044920,
    },
    Vsop87Term {
        amplitude: 0.00000000217,
        phase: 6.27837036335,
        frequency: 17267.26820169119,
    },
    Vsop87Term {
        amplitude: 0.00000000204,
        phase: 2.34615348695,
        frequency: 266.60704172180,
    },
    Vsop87Term {
        amplitude: 0.00000000195,
        phase: 5.55015549753,
        frequency: 6133.51265285680,
    },
    Vsop87Term {
        amplitude: 0.00000000188,
        phase: 2.52667166175,
        frequency: 6525.80445396540,
    },
    Vsop87Term {
        amplitude: 0.00000000185,
        phase: 0.90960768344,
        frequency: 18319.53658487960,
    },
    Vsop87Term {
        amplitude: 0.00000000177,
        phase: 1.73429218289,
        frequency: 154717.60988768269,
    },
    Vsop87Term {
        amplitude: 0.00000000187,
        phase: 4.76483647432,
        frequency: 4535.05943692440,
    },
    Vsop87Term {
        amplitude: 0.00000000186,
        phase: 4.63080493407,
        frequency: 10440.27429260360,
    },
    Vsop87Term {
        amplitude: 0.00000000215,
        phase: 2.81255454560,
        frequency: 7342.45778018060,
    },
    Vsop87Term {
        amplitude: 0.00000000172,
        phase: 1.45551888559,
        frequency: 9225.53927328300,
    },
    Vsop87Term {
        amplitude: 0.00000000162,
        phase: 3.30661909388,
        frequency: 639.89728631400,
    },
    Vsop87Term {
        amplitude: 0.00000000168,
        phase: 2.17671416605,
        frequency: 27.40155609680,
    },
    Vsop87Term {
        amplitude: 0.00000000160,
        phase: 1.68164180475,
        frequency: 15110.46611986620,
    },
    Vsop87Term {
        amplitude: 0.00000000158,
        phase: 0.13519771874,
        frequency: 13095.84266507740,
    },
    Vsop87Term {
        amplitude: 0.00000000183,
        phase: 0.56281322071,
        frequency: 13517.87010623340,
    },
    Vsop87Term {
        amplitude: 0.00000000179,
        phase: 3.58450811616,
        frequency: 87.30820453981,
    },
    Vsop87Term {
        amplitude: 0.00000000152,
        phase: 2.84070476818,
        frequency: 5650.29211067820,
    },
    Vsop87Term {
        amplitude: 0.00000000182,
        phase: 0.44065530624,
        frequency: 17253.04110768959,
    },
    Vsop87Term {
        amplitude: 0.00000000160,
        phase: 5.95767264171,
        frequency: 4701.11650170840,
    },
    Vsop87Term {
        amplitude: 0.00000000142,
        phase: 1.46290137520,
        frequency: 11087.28512591840,
    },
    Vsop87Term {
        amplitude: 0.00000000142,
        phase: 2.04464036087,
        frequency: 20426.57109242200,
    },
    Vsop87Term {
        amplitude: 0.00000000131,
        phase: 5.40912137746,
        frequency: 2699.73481931760,
    },
    Vsop87Term {
        amplitude: 0.00000000144,
        phase: 2.07312090485,
        frequency: 25158.60171976540,
    },
    Vsop87Term {
        amplitude: 0.00000000147,
        phase: 6.15106982168,
        frequency: 9623.68827669120,
    },
    Vsop87Term {
        amplitude: 0.00000000141,
        phase: 5.55739979498,
        frequency: 10454.50138660520,
    },
    Vsop87Term {
        amplitude: 0.00000000135,
        phase: 0.06098110407,
        frequency: 16723.35014259500,
    },
    Vsop87Term {
        amplitude: 0.00000000124,
        phase: 5.81218025669,
        frequency: 17256.63153634140,
    },
    Vsop87Term {
        amplitude: 0.00000000124,
        phase: 2.36293551623,
        frequency: 4933.20844033260,
    },
    Vsop87Term {
        amplitude: 0.00000000126,
        phase: 3.47435905118,
        frequency: 22483.84857449259,
    },
    Vsop87Term {
        amplitude: 0.00000000159,
        phase: 5.63954754618,
        frequency: 5729.50644714900,
    },
    Vsop87Term {
        amplitude: 0.00000000123,
        phase: 3.92815963256,
        frequency: 17996.03116822220,
    },
    Vsop87Term {
        amplitude: 0.00000000148,
        phase: 3.02509280598,
        frequency: 1551.04522264800,
    },
    Vsop87Term {
        amplitude: 0.00000000120,
        phase: 5.91904349732,
        frequency: 6206.80977871580,
    },
    Vsop87Term {
        amplitude: 0.00000000134,
        phase: 3.11122937825,
        frequency: 21954.15760939799,
    },
    Vsop87Term {
        amplitude: 0.00000000119,
        phase: 5.52141123450,
        frequency: 709.93304855830,
    },
    Vsop87Term {
        amplitude: 0.00000000122,
        phase: 3.00813429479,
        frequency: 19800.94595622480,
    },
    Vsop87Term {
        amplitude: 0.00000000127,
        phase: 1.37618620001,
        frequency: 14945.31617355440,
    },
    Vsop87Term {
        amplitude: 0.00000000141,
        phase: 2.56889468729,
        frequency: 1052.26838318840,
    },
    Vsop87Term {
        amplitude: 0.00000000123,
        phase: 2.83671175442,
        frequency: 11919.14086666800,
    },
    Vsop87Term {
        amplitude: 0.00000000118,
        phase: 0.81934438215,
        frequency: 5331.35744374080,
    },
    Vsop87Term {
        amplitude: 0.00000000151,
        phase: 2.68731829165,
        frequency: 11769.85369316640,
    },
    Vsop87Term {
        amplitude: 0.00000000119,
        phase: 5.08835797638,
        frequency: 5481.25491886760,
    },
    Vsop87Term {
        amplitude: 0.00000000153,
        phase: 2.46021790779,
        frequency: 11933.36796066960,
    },
    Vsop87Term {
        amplitude: 0.00000000108,
        phase: 1.04936452145,
        frequency: 11403.67699557500,
    },
    Vsop87Term {
        amplitude: 0.00000000128,
        phase: 0.99794735107,
        frequency: 8827.39026987480,
    },
    Vsop87Term {
        amplitude: 0.00000000144,
        phase: 2.54869747042,
        frequency: 227.47613278900,
    },
    Vsop87Term {
        amplitude: 0.00000000150,
        phase: 4.50631437136,
        frequency: 2379.16447357160,
    },
    Vsop87Term {
        amplitude: 0.00000000107,
        phase: 1.79272017026,
        frequency: 13119.72110282519,
    },
    Vsop87Term {
        amplitude: 0.00000000107,
        phase: 4.43556814486,
        frequency: 18422.62935909819,
    },
    Vsop87Term {
        amplitude: 0.00000000109,
        phase: 0.29269062317,
        frequency: 16737.57723659660,
    },
    Vsop87Term {
        amplitude: 0.00000000141,
        phase: 3.18979826258,
        frequency: 6262.30045449900,
    },
    Vsop87Term {
        amplitude: 0.00000000122,
        phase: 4.23040027813,
        frequency: 29.42950853600,
    },
    Vsop87Term {
        amplitude: 0.00000000111,
        phase: 5.16954029551,
        frequency: 17782.73207278420,
    },
    Vsop87Term {
        amplitude: 0.00000000100,
        phase: 3.52213872761,
        frequency: 18052.92954315780,
    },
    Vsop87Term {
        amplitude: 0.00000000108,
        phase: 1.08514212991,
        frequency: 16858.48253293320,
    },
    Vsop87Term {
        amplitude: 0.00000000106,
        phase: 1.96085248410,
        frequency: 74.78159856730,
    },
    Vsop87Term {
        amplitude: 0.00000000110,
        phase: 2.30582372873,
        frequency: 16460.33352952499,
    },
    Vsop87Term {
        amplitude: 0.00000000097,
        phase: 3.50918940210,
        frequency: 5333.90024102160,
    },
    Vsop87Term {
        amplitude: 0.00000000099,
        phase: 3.56417337974,
        frequency: 735.87651353180,
    },
    Vsop87Term {
        amplitude: 0.00000000094,
        phase: 5.01857894228,
        frequency: 3128.38876509580,
    },
    Vsop87Term {
        amplitude: 0.00000000097,
        phase: 1.65579893894,
        frequency: 533.21408344360,
    },
    Vsop87Term {
        amplitude: 0.00000000092,
        phase: 0.89217162285,
        frequency: 29296.61538957860,
    },
    Vsop87Term {
        amplitude: 0.00000000123,
        phase: 3.16062050433,
        frequency: 9380.95967271720,
    },
    Vsop87Term {
        amplitude: 0.00000000102,
        phase: 1.20493500565,
        frequency: 23020.65308658799,
    },
    Vsop87Term {
        amplitude: 0.00000000088,
        phase: 2.21296088224,
        frequency: 12721.57209941700,
    },
    Vsop87Term {
        amplitude: 0.00000000089,
        phase: 1.54264720310,
        frequency: 20199.09495963300,
    },
    Vsop87Term {
        amplitude: 0.00000000113,
        phase: 4.83320707870,
        frequency: 16496.36139620240,
    },
    Vsop87Term {
        amplitude: 0.00000000121,
        phase: 6.19860353182,
        frequency: 9388.00590941520,
    },
    Vsop87Term {
        amplitude: 0.00000000089,
        phase: 4.08082274765,
        frequency: 22805.73556599360,
    },
    Vsop87Term {
        amplitude: 0.00000000098,
        phase: 1.09181832830,
        frequency: 12043.57428188900,
    },
    Vsop87Term {
        amplitude: 0.00000000086,
        phase: 1.13655027605,
        frequency: 143571.32428481648,
    },
    Vsop87Term {
        amplitude: 0.00000000088,
        phase: 5.96980472191,
        frequency: 107.66352393860,
    },
    Vsop87Term {
        amplitude: 0.00000000082,
        phase: 5.01340404594,
        frequency: 22003.91463486980,
    },
    Vsop87Term {
        amplitude: 0.00000000094,
        phase: 1.69615700473,
        frequency: 23006.42599258639,
    },
    Vsop87Term {
        amplitude: 0.00000000081,
        phase: 3.00657814365,
        frequency: 2118.76386037840,
    },
    Vsop87Term {
        amplitude: 0.00000000098,
        phase: 1.39215287161,
        frequency: 8662.24032356300,
    },
    Vsop87Term {
        amplitude: 0.00000000077,
        phase: 3.33555190840,
        frequency: 15720.83878487840,
    },
    Vsop87Term {
        amplitude: 0.00000000082,
        phase: 5.86880116464,
        frequency: 2787.04302385740,
    },
    Vsop87Term {
        amplitude: 0.00000000076,
        phase: 5.67183650604,
        frequency: 14.22709400160,
    },
    Vsop87Term {
        amplitude: 0.00000000081,
        phase: 6.16619455699,
        frequency: 1039.02661079040,
    },
    Vsop87Term {
        amplitude: 0.00000000076,
        phase: 3.21449884756,
        frequency: 111.18664228760,
    },
    Vsop87Term {
        amplitude: 0.00000000078,
        phase: 1.37531518377,
        frequency: 21947.11137270000,
    },
    Vsop87Term {
        amplitude: 0.00000000074,
        phase: 3.58814195051,
        frequency: 11609.86254401220,
    },
    Vsop87Term {
        amplitude: 0.00000000077,
        phase: 4.84846488388,
        frequency: 22743.40937951640,
    },
    Vsop87Term {
        amplitude: 0.00000000090,
        phase: 1.48869013606,
        frequency: 15671.08175940660,
    },
    Vsop87Term {
        amplitude: 0.00000000082,
        phase: 3.48618399109,
        frequency: 29088.81141598500,
    },
    Vsop87Term {
        amplitude: 0.00000000069,
        phase: 3.55746476593,
        frequency: 4590.91018048900,
    },
    Vsop87Term {
        amplitude: 0.00000000069,
        phase: 1.93625656075,
        frequency: 135.62532501000,
    },
    Vsop87Term {
        amplitude: 0.00000000070,
        phase: 2.66548322237,
        frequency: 18875.52586977400,
    },
    Vsop87Term {
        amplitude: 0.00000000069,
        phase: 5.41478093731,
        frequency: 26735.94526221320,
    },
    Vsop87Term {
        amplitude: 0.00000000079,
        phase: 5.15154513662,
        frequency: 12323.42309600880,
    },
    Vsop87Term {
        amplitude: 0.00000000094,
        phase: 3.62899392448,
        frequency: 77713.77146812050,
    },
    Vsop87Term {
        amplitude: 0.00000000078,
        phase: 4.17011182047,
        frequency: 1066.49547719000,
    },
    Vsop87Term {
        amplitude: 0.00000000071,
        phase: 3.89435637865,
        frequency: 22779.43724619380,
    },
    Vsop87Term {
        amplitude: 0.00000000063,
        phase: 4.53968787714,
        frequency: 8982.81066930900,
    },
    Vsop87Term {
        amplitude: 0.00000000069,
        phase: 0.96028230548,
        frequency: 14919.01785375460,
    },
    Vsop87Term {
        amplitude: 0.00000000076,
        phase: 3.29092216589,
        frequency: 2942.46342329160,
    },
    Vsop87Term {
        amplitude: 0.00000000063,
        phase: 4.09167842893,
        frequency: 16062.18452611680,
    },
    Vsop87Term {
        amplitude: 0.00000000065,
        phase: 3.34580407184,
        frequency: 51.28033786241,
    },
    Vsop87Term {
        amplitude: 0.00000000065,
        phase: 5.75757544877,
        frequency: 52670.06959330260,
    },
    Vsop87Term {
        amplitude: 0.00000000068,
        phase: 5.75884067555,
        frequency: 21424.46664430340,
    },
    Vsop87Term {
        amplitude: 0.00000000057,
        phase: 5.45122399850,
        frequency: 12592.45001978260,
    },
    Vsop87Term {
        amplitude: 0.00000000057,
        phase: 5.25043362558,
        frequency: 20995.39296644940,
    },
    Vsop87Term {
        amplitude: 0.00000000073,
        phase: 0.53299090807,
        frequency: 2301.58581590939,
    },
    Vsop87Term {
        amplitude: 0.00000000070,
        phase: 4.31243357502,
        frequency: 19402.79695281660,
    },
    Vsop87Term {
        amplitude: 0.00000000067,
        phase: 2.53852336668,
        frequency: 377.37360791580,
    },
    Vsop87Term {
        amplitude: 0.00000000056,
        phase: 3.20816844695,
        frequency: 24889.57479599160,
    },
    Vsop87Term {
        amplitude: 0.00000000053,
        phase: 3.17816599142,
        frequency: 18451.07854656599,
    },
    Vsop87Term {
        amplitude: 0.00000000053,
        phase: 3.61529270216,
        frequency: 77.67377042800,
    },
    Vsop87Term {
        amplitude: 0.00000000053,
        phase: 0.45467549335,
        frequency: 30666.15495843280,
    },
    Vsop87Term {
        amplitude: 0.00000000061,
        phase: 0.14807288453,
        frequency: 23013.53953958720,
    },
    Vsop87Term {
        amplitude: 0.00000000051,
        phase: 3.32803972907,
        frequency: 56.89837493560,
    },
    Vsop87Term {
        amplitude: 0.00000000052,
        phase: 3.41177624177,
        frequency: 23141.55838292460,
    },
    Vsop87Term {
        amplitude: 0.00000000058,
        phase: 3.13638677202,
        frequency: 309.27832265580,
    },
    Vsop87Term {
        amplitude: 0.00000000070,
        phase: 2.50592323465,
        frequency: 31415.37924995700,
    },
    Vsop87Term {
        amplitude: 0.00000000052,
        phase: 5.10673376738,
        frequency: 17796.95916678580,
    },
    Vsop87Term {
        amplitude: 0.00000000067,
        phase: 6.27917920454,
        frequency: 22345.26037610820,
    },
    Vsop87Term {
        amplitude: 0.00000000050,
        phase: 0.42577644151,
        frequency: 25685.87280280800,
    },
    Vsop87Term {
        amplitude: 0.00000000048,
        phase: 0.70204553333,
        frequency: 1162.47470440780,
    },
    Vsop87Term {
        amplitude: 0.00000000066,
        phase: 3.64350022359,
        frequency: 15265.88651930040,
    },
    Vsop87Term {
        amplitude: 0.00000000050,
        phase: 5.74382917440,
        frequency: 19.66976089979,
    },
    Vsop87Term {
        amplitude: 0.00000000050,
        phase: 4.69825387775,
        frequency: 28237.23345938940,
    },
    Vsop87Term {
        amplitude: 0.00000000047,
        phase: 5.74015846442,
        frequency: 12139.55350910680,
    },
    Vsop87Term {
        amplitude: 0.00000000054,
        phase: 1.97301333704,
        frequency: 23581.25817731760,
    },
    Vsop87Term {
        amplitude: 0.00000000049,
        phase: 4.98223579027,
        frequency: 10021.83728009940,
    },
    Vsop87Term {
        amplitude: 0.00000000046,
        phase: 5.41431705539,
        frequency: 33019.02111220460,
    },
    Vsop87Term {
        amplitude: 0.00000000051,
        phase: 1.23882053879,
        frequency: 12539.85338018300,
    },
    Vsop87Term {
        amplitude: 0.00000000046,
        phase: 2.41369976086,
        frequency: 98068.53671630539,
    },
    Vsop87Term {
        amplitude: 0.00000000044,
        phase: 0.80750593746,
        frequency: 167283.76158766549,
    },
    Vsop87Term {
        amplitude: 0.00000000045,
        phase: 4.39613584445,
        frequency: 433.71173787680,
    },
    Vsop87Term {
        amplitude: 0.00000000044,
        phase: 2.57358208785,
        frequency: 12964.30070339100,
    },
    Vsop87Term {
        amplitude: 0.00000000046,
        phase: 0.26142733448,
        frequency: 11.04570026390,
    },
    Vsop87Term {
        amplitude: 0.00000000045,
        phase: 2.46230645202,
        frequency: 51868.24866217880,
    },
    Vsop87Term {
        amplitude: 0.00000000048,
        phase: 0.89551707131,
        frequency: 56600.27928952220,
    },
    Vsop87Term {
        amplitude: 0.00000000057,
        phase: 1.86416707010,
        frequency: 25287.72379939980,
    },
    Vsop87Term {
        amplitude: 0.00000000042,
        phase: 5.26377513431,
        frequency: 26084.02180621620,
    },
    Vsop87Term {
        amplitude: 0.00000000049,
        phase: 3.17757670611,
        frequency: 6303.85124548380,
    },
    Vsop87Term {
        amplitude: 0.00000000052,
        phase: 3.65266055509,
        frequency: 7872.14874527520,
    },
    Vsop87Term {
        amplitude: 0.00000000040,
        phase: 1.81891629936,
        frequency: 34596.36465465240,
    },
    Vsop87Term {
        amplitude: 0.00000000043,
        phase: 1.94164978061,
        frequency: 1903.43681250120,
    },
    Vsop87Term {
        amplitude: 0.00000000041,
        phase: 0.74461854136,
        frequency: 23937.85638974100,
    },
    Vsop87Term {
        amplitude: 0.00000000048,
        phase: 6.26034008181,
        frequency: 28286.99048486120,
    },
    Vsop87Term {
        amplitude: 0.00000000045,
        phase: 5.45575017530,
        frequency: 60530.48898574180,
    },
    Vsop87Term {
        amplitude: 0.00000000040,
        phase: 2.92105728682,
        frequency: 21548.96236929180,
    },
    Vsop87Term {
        amplitude: 0.00000000040,
        phase: 0.04502010161,
        frequency: 38526.57435087200,
    },
    Vsop87Term {
        amplitude: 0.00000000053,
        phase: 3.64791042082,
        frequency: 11925.27409260060,
    },
    Vsop87Term {
        amplitude: 0.00000000041,
        phase: 5.04048954693,
        frequency: 27832.03821928320,
    },
    Vsop87Term {
        amplitude: 0.00000000042,
        phase: 5.19292937193,
        frequency: 19004.64794940840,
    },
    Vsop87Term {
        amplitude: 0.00000000040,
        phase: 2.57120233428,
        frequency: 24356.78078864160,
    },
    Vsop87Term {
        amplitude: 0.00000000038,
        phase: 3.49190341464,
        frequency: 226858.23855437008,
    },
    Vsop87Term {
        amplitude: 0.00000000039,
        phase: 4.61184303844,
        frequency: 95.97922721780,
    },
    Vsop87Term {
        amplitude: 0.00000000043,
        phase: 2.20648228147,
        frequency: 13521.75144159140,
    },
    Vsop87Term {
        amplitude: 0.00000000040,
        phase: 5.83461945819,
        frequency: 16193.65917750039,
    },
    Vsop87Term {
        amplitude: 0.00000000045,
        phase: 3.73714372195,
        frequency: 7875.67186362420,
    },
    Vsop87Term {
        amplitude: 0.00000000043,
        phase: 1.14078465002,
        frequency: 49.75702547180,
    },
    Vsop87Term {
        amplitude: 0.00000000037,
        phase: 1.29390383811,
        frequency: 310.84079886840,
    },
    Vsop87Term {
        amplitude: 0.00000000038,
        phase: 0.95970925950,
        frequency: 664.75604513000,
    },
    Vsop87Term {
        amplitude: 0.00000000037,
        phase: 4.27532649462,
        frequency: 6709.67404086740,
    },
    Vsop87Term {
        amplitude: 0.00000000038,
        phase: 2.20108541046,
        frequency: 28628.33622609960,
    },
    Vsop87Term {
        amplitude: 0.00000000039,
        phase: 0.85957361635,
        frequency: 16522.65971600220,
    },
    Vsop87Term {
        amplitude: 0.00000000040,
        phase: 4.35214003837,
        frequency: 48739.85989708300,
    },
    Vsop87Term {
        amplitude: 0.00000000036,
        phase: 1.68167662194,
        frequency: 10344.29506538580,
    },
    Vsop87Term {
        amplitude: 0.00000000040,
        phase: 5.13217319067,
        frequency: 15664.03552270859,
    },
    Vsop87Term {
        amplitude: 0.00000000036,
        phase: 3.72187132496,
        frequency: 30774.50164257480,
    },
    Vsop87Term {
        amplitude: 0.00000000036,
        phase: 3.32158458257,
        frequency: 16207.88627150200,
    },
    Vsop87Term {
        amplitude: 0.00000000045,
        phase: 3.94202418608,
        frequency: 10988.80815753500,
    },
    Vsop87Term {
        amplitude: 0.00000000039,
        phase: 1.51948786199,
        frequency: 12029.34718788740,
    },
    Vsop87Term {
        amplitude: 0.00000000026,
        phase: 3.87685883180,
        frequency: 6262.72053059260,
    },
    Vsop87Term {
        amplitude: 0.00000000024,
        phase: 4.91804163466,
        frequency: 19651.04848109800,
    },
    Vsop87Term {
        amplitude: 0.00000000023,
        phase: 0.29300197709,
        frequency: 13362.44970679920,
    },
    Vsop87Term {
        amplitude: 0.00000000021,
        phase: 3.18605672363,
        frequency: 6277.55292568400,
    },
    Vsop87Term {
        amplitude: 0.00000000021,
        phase: 6.07546891132,
        frequency: 18139.29450141590,
    },
    Vsop87Term {
        amplitude: 0.00000000022,
        phase: 2.31199937177,
        frequency: 6303.43116939020,
    },
    Vsop87Term {
        amplitude: 0.00000000021,
        phase: 3.58418394393,
        frequency: 18209.33026366019,
    },
    Vsop87Term {
        amplitude: 0.00000000026,
        phase: 2.06801296900,
        frequency: 12573.26524698360,
    },
    Vsop87Term {
        amplitude: 0.00000000021,
        phase: 1.56857722317,
        frequency: 13341.67431130680,
    },
    Vsop87Term {
        amplitude: 0.00000000024,
        phase: 5.72605158675,
        frequency: 29864.33402730900,
    },
    Vsop87Term {
        amplitude: 0.00000000024,
        phase: 1.40237993205,
        frequency: 14712.31711645800,
    },
    Vsop87Term {
        amplitude: 0.00000000025,
        phase: 5.71466092822,
        frequency: 25934.12433108940,
    },
];

pub(super) const L2: &[Vsop87Term] = &[
    Vsop87Term {
        amplitude: 0.00052918870,
        phase: 0.00000000000,
        frequency: 0.00000000000,
    },
    Vsop87Term {
        amplitude: 0.00008719837,
        phase: 1.07209665242,
        frequency: 6283.07584999140,
    },
    Vsop87Term {
        amplitude: 0.00000309125,
        phase: 0.86728818832,
        frequency: 12566.15169998280,
    },
    Vsop87Term {
        amplitude: 0.00000027339,
        phase: 0.05297871691,
        frequency: 3.52311834900,
    },
    Vsop87Term {
        amplitude: 0.00000016334,
        phase: 5.18826691036,
        frequency: 26.29831979980,
    },
    Vsop87Term {
        amplitude: 0.00000015752,
        phase: 3.68457889430,
        frequency: 155.42039943420,
    },
    Vsop87Term {
        amplitude: 0.00000009541,
        phase: 0.75742297675,
        frequency: 18849.22754997420,
    },
    Vsop87Term {
        amplitude: 0.00000008937,
        phase: 2.05705419118,
        frequency: 77713.77146812050,
    },
    Vsop87Term {
        amplitude: 0.00000006952,
        phase: 0.82673305410,
        frequency: 775.52261132400,
    },
    Vsop87Term {
        amplitude: 0.00000005064,
        phase: 4.66284525271,
        frequency: 1577.34354244780,
    },
    Vsop87Term {
        amplitude: 0.00000004061,
        phase: 1.03057162962,
        frequency: 7.11354700080,
    },
    Vsop87Term {
        amplitude: 0.00000003463,
        phase: 5.14074632811,
        frequency: 796.29800681640,
    },
    Vsop87Term {
        amplitude: 0.00000003169,
        phase: 6.05291851171,
        frequency: 5507.55323866740,
    },
    Vsop87Term {
        amplitude: 0.00000003020,
        phase: 1.19246506441,
        frequency: 242.72860397400,
    },
    Vsop87Term {
        amplitude: 0.00000002886,
        phase: 6.11652627155,
        frequency: 529.69096509460,
    },
    Vsop87Term {
        amplitude: 0.00000003810,
        phase: 3.44050803490,
        frequency: 5573.14280143310,
    },
    Vsop87Term {
        amplitude: 0.00000002714,
        phase: 0.30637881025,
        frequency: 398.14900340820,
    },
    Vsop87Term {
        amplitude: 0.00000002371,
        phase: 4.38118838167,
        frequency: 5223.69391980220,
    },
    Vsop87Term {
        amplitude: 0.00000002538,
        phase: 2.27992810679,
        frequency: 553.56940284240,
    },
    Vsop87Term {
        amplitude: 0.00000002079,
        phase: 3.75435330484,
        frequency: 0.98032106820,
    },
    Vsop87Term {
        amplitude: 0.00000001675,
        phase: 0.90216407959,
        frequency: 951.71840625060,
    },
    Vsop87Term {
        amplitude: 0.00000001534,
        phase: 5.75900462759,
        frequency: 1349.86740965880,
    },
    Vsop87Term {
        amplitude: 0.00000001224,
        phase: 2.97328088405,
        frequency: 2146.16541647520,
    },
    Vsop87Term {
        amplitude: 0.00000001449,
        phase: 4.36415913970,
        frequency: 1748.01641306700,
    },
    Vsop87Term {
        amplitude: 0.00000001341,
        phase: 3.72061130861,
        frequency: 1194.44701022460,
    },
    Vsop87Term {
        amplitude: 0.00000001254,
        phase: 2.94846826628,
        frequency: 6438.49624942560,
    },
    Vsop87Term {
        amplitude: 0.00000000999,
        phase: 5.98640014468,
        frequency: 6286.59896834040,
    },
    Vsop87Term {
        amplitude: 0.00000000917,
        phase: 4.79788687522,
        frequency: 5088.62883976680,
    },
    Vsop87Term {
        amplitude: 0.00000000828,
        phase: 3.31321076572,
        frequency: 213.29909543800,
    },
    Vsop87Term {
        amplitude: 0.00000001103,
        phase: 1.27104454479,
        frequency: 161000.68573767410,
    },
    Vsop87Term {
        amplitude: 0.00000000762,
        phase: 3.41582762988,
        frequency: 5486.77784317500,
    },
    Vsop87Term {
        amplitude: 0.00000001044,
        phase: 0.60409577691,
        frequency: 3154.68708489560,
    },
    Vsop87Term {
        amplitude: 0.00000000887,
        phase: 5.23465144638,
        frequency: 7084.89678111520,
    },
    Vsop87Term {
        amplitude: 0.00000000645,
        phase: 1.60096192515,
        frequency: 2544.31441988340,
    },
    Vsop87Term {
        amplitude: 0.00000000681,
        phase: 3.43155669169,
        frequency: 4694.00295470760,
    },
    Vsop87Term {
        amplitude: 0.00000000605,
        phase: 2.47806340546,
        frequency: 10977.07880469900,
    },
    Vsop87Term {
        amplitude: 0.00000000706,
        phase: 6.19393222575,
        frequency: 4690.47983635860,
    },
    Vsop87Term {
        amplitude: 0.00000000643,
        phase: 1.98042503148,
        frequency: 801.82093112380,
    },
    Vsop87Term {
        amplitude: 0.00000000502,
        phase: 1.44394375363,
        frequency: 6836.64525283380,
    },
    Vsop87Term {
        amplitude: 0.00000000490,
        phase: 2.34129524194,
        frequency: 1592.59601363280,
    },
    Vsop87Term {
        amplitude: 0.00000000458,
        phase: 1.30876448575,
        frequency: 4292.33083295040,
    },
    Vsop87Term {
        amplitude: 0.00000000431,
        phase: 0.03526421494,
        frequency: 7234.79425624200,
    },
    Vsop87Term {
        amplitude: 0.00000000379,
        phase: 3.17030522615,
        frequency: 6309.37416979120,
    },
    Vsop87Term {
        amplitude: 0.00000000348,
        phase: 0.99049550009,
        frequency: 6040.34724601740,
    },
    Vsop87Term {
        amplitude: 0.00000000386,
        phase: 1.57019797263,
        frequency: 71430.69561812909,
    },
    Vsop87Term {
        amplitude: 0.00000000347,
        phase: 0.67013291338,
        frequency: 1059.38193018920,
    },
    Vsop87Term {
        amplitude: 0.00000000458,
        phase: 3.81499443681,
        frequency: 149854.40013480789,
    },
    Vsop87Term {
        amplitude: 0.00000000302,
        phase: 1.91760044838,
        frequency: 10447.38783960440,
    },
    Vsop87Term {
        amplitude: 0.00000000307,
        phase: 3.55343347416,
        frequency: 8031.09226305840,
    },
    Vsop87Term {
        amplitude: 0.00000000395,
        phase: 4.93701776616,
        frequency: 7632.94325965020,
    },
    Vsop87Term {
        amplitude: 0.00000000314,
        phase: 3.18093696547,
        frequency: 2352.86615377180,
    },
    Vsop87Term {
        amplitude: 0.00000000282,
        phase: 4.41936437052,
        frequency: 9437.76293488700,
    },
    Vsop87Term {
        amplitude: 0.00000000276,
        phase: 2.71314254553,
        frequency: 3894.18182954220,
    },
    Vsop87Term {
        amplitude: 0.00000000298,
        phase: 2.52037474210,
        frequency: 6127.65545055720,
    },
    Vsop87Term {
        amplitude: 0.00000000230,
        phase: 1.37790215549,
        frequency: 4705.73230754360,
    },
    Vsop87Term {
        amplitude: 0.00000000252,
        phase: 0.55330133471,
        frequency: 6279.55273164240,
    },
    Vsop87Term {
        amplitude: 0.00000000255,
        phase: 5.26570187369,
        frequency: 6812.76681508600,
    },
    Vsop87Term {
        amplitude: 0.00000000275,
        phase: 0.67264264272,
        frequency: 25132.30339996560,
    },
    Vsop87Term {
        amplitude: 0.00000000178,
        phase: 0.92820785174,
        frequency: 1990.74501704100,
    },
    Vsop87Term {
        amplitude: 0.00000000221,
        phase: 0.63897368842,
        frequency: 6256.77753019160,
    },
    Vsop87Term {
        amplitude: 0.00000000155,
        phase: 0.77319790838,
        frequency: 14143.49524243060,
    },
    Vsop87Term {
        amplitude: 0.00000000150,
        phase: 2.40470465561,
        frequency: 426.59819087600,
    },
    Vsop87Term {
        amplitude: 0.00000000196,
        phase: 6.06877865012,
        frequency: 640.87760738220,
    },
    Vsop87Term {
        amplitude: 0.00000000137,
        phase: 2.21679460145,
        frequency: 8429.24126646660,
    },
    Vsop87Term {
        amplitude: 0.00000000127,
        phase: 3.26094223174,
        frequency: 17789.84561978500,
    },
    Vsop87Term {
        amplitude: 0.00000000128,
        phase: 5.47237279946,
        frequency: 12036.46073488820,
    },
    Vsop87Term {
        amplitude: 0.00000000122,
        phase: 2.16291082757,
        frequency: 10213.28554621100,
    },
    Vsop87Term {
        amplitude: 0.00000000118,
        phase: 0.45789822268,
        frequency: 7058.59846131540,
    },
    Vsop87Term {
        amplitude: 0.00000000141,
        phase: 2.34932647403,
        frequency: 11506.76976979360,
    },
    Vsop87Term {
        amplitude: 0.00000000100,
        phase: 0.85621569847,
        frequency: 6290.18939699220,
    },
    Vsop87Term {
        amplitude: 0.00000000092,
        phase: 5.10587476002,
        frequency: 7079.37385680780,
    },
    Vsop87Term {
        amplitude: 0.00000000126,
        phase: 2.65428307012,
        frequency: 88860.05707098669,
    },
    Vsop87Term {
        amplitude: 0.00000000106,
        phase: 5.85646710022,
        frequency: 7860.41939243920,
    },
    Vsop87Term {
        amplitude: 0.00000000084,
        phase: 3.57457554262,
        frequency: 16730.46368959580,
    },
    Vsop87Term {
        amplitude: 0.00000000089,
        phase: 4.21433259618,
        frequency: 83996.84731811189,
    },
    Vsop87Term {
        amplitude: 0.00000000097,
        phase: 5.57938280855,
        frequency: 13367.97263110660,
    },
    Vsop87Term {
        amplitude: 0.00000000102,
        phase: 2.05853060226,
        frequency: 87.30820453981,
    },
    Vsop87Term {
        amplitude: 0.00000000080,
        phase: 4.73792651816,
        frequency: 11926.25441366880,
    },
    Vsop87Term {
        amplitude: 0.00000000080,
        phase: 5.41418965044,
        frequency: 10973.55568635000,
    },
    Vsop87Term {
        amplitude: 0.00000000106,
        phase: 4.10978997399,
        frequency: 3496.03282613400,
    },
    Vsop87Term {
        amplitude: 0.00000000102,
        phase: 3.62650006043,
        frequency: 244287.60000722769,
    },
    Vsop87Term {
        amplitude: 0.00000000075,
        phase: 4.89483161769,
        frequency: 5643.17856367740,
    },
    Vsop87Term {
        amplitude: 0.00000000087,
        phase: 0.42863750683,
        frequency: 11015.10647733480,
    },
    Vsop87Term {
        amplitude: 0.00000000069,
        phase: 1.88908760720,
        frequency: 10177.25767953360,
    },
    Vsop87Term {
        amplitude: 0.00000000089,
        phase: 1.35567273119,
        frequency: 6681.22485339960,
    },
    Vsop87Term {
        amplitude: 0.00000000066,
        phase: 0.99455837265,
        frequency: 6525.80445396540,
    },
    Vsop87Term {
        amplitude: 0.00000000067,
        phase: 5.51240997070,
        frequency: 3097.88382272579,
    },
    Vsop87Term {
        amplitude: 0.00000000076,
        phase: 2.72016814799,
        frequency: 4164.31198961300,
    },
    Vsop87Term {
        amplitude: 0.00000000063,
        phase: 1.44349902540,
        frequency: 9917.69687450980,
    },
    Vsop87Term {
        amplitude: 0.00000000078,
        phase: 3.51469733747,
        frequency: 11856.21865142450,
    },
    Vsop87Term {
        amplitude: 0.00000000085,
        phase: 0.50956043858,
        frequency: 10575.40668294180,
    },
    Vsop87Term {
        amplitude: 0.00000000067,
        phase: 3.62043033405,
        frequency: 16496.36139620240,
    },
    Vsop87Term {
        amplitude: 0.00000000055,
        phase: 5.24637517308,
        frequency: 3340.61242669980,
    },
    Vsop87Term {
        amplitude: 0.00000000048,
        phase: 5.43966777314,
        frequency: 20426.57109242200,
    },
    Vsop87Term {
        amplitude: 0.00000000064,
        phase: 5.79535817813,
        frequency: 2388.89402044920,
    },
    Vsop87Term {
        amplitude: 0.00000000046,
        phase: 5.43499966519,
        frequency: 6275.96230299060,
    },
    Vsop87Term {
        amplitude: 0.00000000050,
        phase: 3.86263598617,
        frequency: 5729.50644714900,
    },
    Vsop87Term {
        amplitude: 0.00000000044,
        phase: 1.52269529228,
        frequency: 12168.00269657460,
    },
    Vsop87Term {
        amplitude: 0.00000000057,
        phase: 4.96352373486,
        frequency: 14945.31617355440,
    },
    Vsop87Term {
        amplitude: 0.00000000045,
        phase: 1.00861230160,
        frequency: 8635.94200376320,
    },
    Vsop87Term {
        amplitude: 0.00000000043,
        phase: 3.30685683359,
        frequency: 9779.10867612540,
    },
    Vsop87Term {
        amplitude: 0.00000000042,
        phase: 0.63481258930,
        frequency: 2699.73481931760,
    },
    Vsop87Term {
        amplitude: 0.00000000041,
        phase: 5.67996766641,
        frequency: 11712.95531823080,
    },
    Vsop87Term {
        amplitude: 0.00000000056,
        phase: 4.34024451468,
        frequency: 90955.55169449610,
    },
    Vsop87Term {
        amplitude: 0.00000000041,
        phase: 5.81722212845,
        frequency: 709.93304855830,
    },
    Vsop87Term {
        amplitude: 0.00000000053,
        phase: 6.17052087143,
        frequency: 233141.31440436149,
    },
    Vsop87Term {
        amplitude: 0.00000000037,
        phase: 3.12495025087,
        frequency: 16200.77272450120,
    },
    Vsop87Term {
        amplitude: 0.00000000035,
        phase: 5.76973458495,
        frequency: 12569.67481833180,
    },
    Vsop87Term {
        amplitude: 0.00000000037,
        phase: 0.31656444326,
        frequency: 24356.78078864160,
    },
    Vsop87Term {
        amplitude: 0.00000000035,
        phase: 0.96229051027,
        frequency: 17298.18232732620,
    },
    Vsop87Term {
        amplitude: 0.00000000033,
        phase: 5.23130355867,
        frequency: 5331.35744374080,
    },
    Vsop87Term {
        amplitude: 0.00000000035,
        phase: 0.62517020593,
        frequency: 25158.60171976540,
    },
    Vsop87Term {
        amplitude: 0.00000000035,
        phase: 0.80004512129,
        frequency: 13916.01910964160,
    },
    Vsop87Term {
        amplitude: 0.00000000037,
        phase: 2.89336088688,
        frequency: 12721.57209941700,
    },
    Vsop87Term {
        amplitude: 0.00000000030,
        phase: 4.50198402401,
        frequency: 23543.23050468179,
    },
    Vsop87Term {
        amplitude: 0.00000000030,
        phase: 5.31355708693,
        frequency: 18319.53658487960,
    },
    Vsop87Term {
        amplitude: 0.00000000029,
        phase: 3.47275229977,
        frequency: 13119.72110282519,
    },
    Vsop87Term {
        amplitude: 0.00000000029,
        phase: 3.11002782516,
        frequency: 4136.91043351620,
    },
    Vsop87Term {
        amplitude: 0.00000000032,
        phase: 5.52273255667,
        frequency: 5753.38488489680,
    },
    Vsop87Term {
        amplitude: 0.00000000035,
        phase: 3.79699996680,
        frequency: 143571.32428481648,
    },
    Vsop87Term {
        amplitude: 0.00000000026,
        phase: 1.50634201907,
        frequency: 154717.60988768269,
    },
    Vsop87Term {
        amplitude: 0.00000000030,
        phase: 3.53519084118,
        frequency: 6284.05617105960,
    },
    Vsop87Term {
        amplitude: 0.00000000023,
        phase: 4.41808025967,
        frequency: 5884.92684658320,
    },
    Vsop87Term {
        amplitude: 0.00000000025,
        phase: 1.38477355808,
        frequency: 65147.61976813770,
    },
    Vsop87Term {
        amplitude: 0.00000000023,
        phase: 3.49782549797,
        frequency: 7477.52286021600,
    },
    Vsop87Term {
        amplitude: 0.00000000019,
        phase: 3.14329413716,
        frequency: 6496.37494542940,
    },
    Vsop87Term {
        amplitude: 0.00000000019,
        phase: 2.20135125199,
        frequency: 18073.70493865020,
    },
    Vsop87Term {
        amplitude: 0.00000000019,
        phase: 4.95020255309,
        frequency: 3930.20969621960,
    },
    Vsop87Term {
        amplitude: 0.00000000019,
        phase: 0.57998702747,
        frequency: 31415.37924995700,
    },
    Vsop87Term {
        amplitude: 0.00000000021,
        phase: 1.75474323399,
        frequency: 12139.55350910680,
    },
    Vsop87Term {
        amplitude: 0.00000000019,
        phase: 3.92233070499,
        frequency: 19651.04848109800,
    },
    Vsop87Term {
        amplitude: 0.00000000014,
        phase: 0.98131213224,
        frequency: 12559.03815298200,
    },
    Vsop87Term {
        amplitude: 0.00000000019,
        phase: 4.93309333729,
        frequency: 2942.46342329160,
    },
    Vsop87Term {
        amplitude: 0.00000000016,
        phase: 5.55997534558,
        frequency: 8827.39026987480,
    },
    Vsop87Term {
        amplitude: 0.00000000013,
        phase: 1.68808165516,
        frequency: 4535.05943692440,
    },
    Vsop87Term {
        amplitude: 0.00000000013,
        phase: 0.33982116161,
        frequency: 4933.20844033260,
    },
    Vsop87Term {
        amplitude: 0.00000000012,
        phase: 1.85426309994,
        frequency: 5856.47765911540,
    },
    Vsop87Term {
        amplitude: 0.00000000010,
        phase: 4.82763996845,
        frequency: 13095.84266507740,
    },
    Vsop87Term {
        amplitude: 0.00000000011,
        phase: 5.38005490571,
        frequency: 11790.62908865880,
    },
    Vsop87Term {
        amplitude: 0.00000000010,
        phase: 1.40815507226,
        frequency: 10988.80815753500,
    },
    Vsop87Term {
        amplitude: 0.00000000011,
        phase: 3.05005267431,
        frequency: 17260.15465469040,
    },
    Vsop87Term {
        amplitude: 0.00000000010,
        phase: 4.93364992366,
        frequency: 12352.85260454480,
    },
];

pub(super) const L3: &[Vsop87Term] = &[
    Vsop87Term {
        amplitude: 0.00000289226,
        phase: 5.84384198723,
        frequency: 6283.07584999140,
    },
    Vsop87Term {
        amplitude: 0.00000034955,
        phase: 0.00000000000,
        frequency: 0.00000000000,
    },
    Vsop87Term {
        amplitude: 0.00000016819,
        phase: 5.48766912348,
        frequency: 12566.15169998280,
    },
    Vsop87Term {
        amplitude: 0.00000002962,
        phase: 5.19577265202,
        frequency: 155.42039943420,
    },
    Vsop87Term {
        amplitude: 0.00000001288,
        phase: 4.72200252235,
        frequency: 3.52311834900,
    },
    Vsop87Term {
        amplitude: 0.00000000635,
        phase: 5.96925937141,
        frequency: 242.72860397400,
    },
    Vsop87Term {
        amplitude: 0.00000000714,
        phase: 5.30045809128,
        frequency: 18849.22754997420,
    },
    Vsop87Term {
        amplitude: 0.00000000402,
        phase: 3.78682982419,
        frequency: 553.56940284240,
    },
    Vsop87Term {
        amplitude: 0.00000000072,
        phase: 4.29768126180,
        frequency: 6286.59896834040,
    },
    Vsop87Term {
        amplitude: 0.00000000067,
        phase: 0.90721687647,
        frequency: 6127.65545055720,
    },
    Vsop87Term {
        amplitude: 0.00000000036,
        phase: 5.24029648014,
        frequency: 6438.49624942560,
    },
    Vsop87Term {
        amplitude: 0.00000000024,
        phase: 5.16003960716,
        frequency: 25132.30339996560,
    },
    Vsop87Term {
        amplitude: 0.00000000023,
        phase: 3.01921570335,
        frequency: 6309.37416979120,
    },
    Vsop87Term {
        amplitude: 0.00000000017,
        phase: 5.82863573502,
        frequency: 6525.80445396540,
    },
    Vsop87Term {
        amplitude: 0.00000000017,
        phase: 3.67772863930,
        frequency: 71430.69561812909,
    },
    Vsop87Term {
        amplitude: 0.00000000009,
        phase: 4.58467294499,
        frequency: 1577.34354244780,
    },
    Vsop87Term {
        amplitude: 0.00000000008,
        phase: 1.40626662824,
        frequency: 11856.21865142450,
    },
    Vsop87Term {
        amplitude: 0.00000000008,
        phase: 5.07561257196,
        frequency: 6256.77753019160,
    },
    Vsop87Term {
        amplitude: 0.00000000007,
        phase: 2.82473374405,
        frequency: 83996.84731811189,
    },
    Vsop87Term {
        amplitude: 0.00000000005,
        phase: 2.71488713339,
        frequency: 10977.07880469900,
    },
    Vsop87Term {
        amplitude: 0.00000000005,
        phase: 3.76879847273,
        frequency: 12036.46073488820,
    },
    Vsop87Term {
        amplitude: 0.00000000005,
        phase: 4.28412873331,
        frequency: 6275.96230299060,
    },
];

pub(super) const L4: &[Vsop87Term] = &[
    Vsop87Term {
        amplitude: 0.00000114084,
        phase: 3.14159265359,
        frequency: 0.00000000000,
    },
    Vsop87Term {
        amplitude: 0.00000007717,
        phase: 4.13446589358,
        frequency: 6283.07584999140,
    },
    Vsop87Term {
        amplitude: 0.00000000765,
        phase: 3.83803776214,
        frequency: 12566.15169998280,
    },
    Vsop87Term {
        amplitude: 0.00000000420,
        phase: 0.41925861858,
        frequency: 155.42039943420,
    },
    Vsop87Term {
        amplitude: 0.00000000040,
        phase: 3.59847585840,
        frequency: 18849.22754997420,
    },
    Vsop87Term {
        amplitude: 0.00000000041,
        phase: 3.14398414077,
        frequency: 3.52311834900,
    },
    Vsop87Term {
        amplitude: 0.00000000035,
        phase: 5.00298940826,
        frequency: 5573.14280143310,
    },
    Vsop87Term {
        amplitude: 0.00000000013,
        phase: 0.48794833701,
        frequency: 77713.77146812050,
    },
    Vsop87Term {
        amplitude: 0.00000000010,
        phase: 5.64801766350,
        frequency: 6127.65545055720,
    },
    Vsop87Term {
        amplitude: 0.00000000008,
        phase: 2.84160570605,
        frequency: 161000.68573767410,
    },
    Vsop87Term {
        amplitude: 0.00000000002,
        phase: 0.54912904658,
        frequency: 6438.49624942560,
    },
];

pub(super) const L5: &[Vsop87Term] = &[
    Vsop87Term {
        amplitude: 0.00000000878,
        phase: 3.14159265359,
        frequency: 0.00000000000,
    },
    Vsop87Term {
        amplitude: 0.00000000172,
        phase: 2.76579069510,
        frequency: 6283.07584999140,
    },
    Vsop87Term {
        amplitude: 0.00000000050,
        phase: 2.01353298182,
        frequency: 155.42039943420,
    },
    Vsop87Term {
        amplitude: 0.00000000028,
        phase: 2.21496423926,
        frequency: 12566.15169998280,
    },
    Vsop87Term {
        amplitude: 0.00000000005,
        phase: 1.75600058765,
        frequency: 18849.22754997420,
    },
];

pub(super) const LONGITUDE_SERIES: [&[Vsop87Term]; 6] = [L0, L1, L2, L3, L4, L5];

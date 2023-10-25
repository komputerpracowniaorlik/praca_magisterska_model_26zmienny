use std::{
    fs::OpenOptions,
    io::{self, BufWriter, Write},
};

//czarna magia
struct BoxedFunction {
    f: Box<dyn Fn([f64; 27], [f64; 31]) -> f64>,
}
impl BoxedFunction {
    fn new<F>(f: F) -> BoxedFunction
    where
        F: Fn([f64; 27], [f64; 31]) -> f64 + 'static,
    {
        BoxedFunction { f: Box::new(f) }
    }
}

macro_rules! zip {
    ($x: expr) => ($x);
    ($x: expr, $($y: expr), +) => (
        $x.iter().zip(
            zip!($($y), +))
    )
}

fn main() {
    let _path1 =
        "/home/kartonrealista/actual_code/praca_magisterska_model_26zmienny/ptau1000.csv";
    let _path2 =
        "/home/kartonrealista/actual_code/praca_magisterska_model_26zmienny/stezenia.csv";
    let _path1win = r"C:\Users\admin\Desktop\MTHOMAS\x\model26zmienny\ptau.csv";
    let path2win =
        r"C:\Users\admin\Desktop\MTHOMAS\x\model26zmienny\stezenia.csv";

    //stałe
    let mut km = [0.0; 31];
    km[1] = 10.0_f64.powf(5.0);
    km[3] = 10.0_f64.powf(-2.0);
    km[4] = 20.0;
    km[5] = 7.5 * 10.0_f64.powf(-4.0);
    km[6] = 0.3;
    km[7] = 5.0 * 10.0_f64.powf(2.0);
    km[8] = 10.0_f64.powf(3.0);
    km[9] = 1.0;
    km[10] = 0.1;
    km[11] = 10.0_f64.powf(-3.0);
    km[12] = 5.0 * 10.0_f64.powf(-2.0);
    km[13] = 10.0_f64.powf(9.0);
    km[14] = 25.1;
    km[15] = 10.0_f64.powf(9.0);
    km[16] = 1.62 * 10.0_f64.powf(6.0);
    km[17] = 10.0_f64.powf(9.0);
    km[18] = 3.0 * 10.0_f64.powf(3.0);
    km[19] = 2.0 * 10.0_f64.powf(6.0);
    km[20] = 10.0_f64.powf(5.0);
    km[21] = 2.0 * 10.0_f64.powf(3.0);
    km[22] = 1.5 * 10.0_f64.powf(5.0);
    km[23] = 10.0_f64.powf(8.0);
    km[24] = 0.2;
    km[25] = 10.0_f64.powf(6.0);
    km[26] = 10.0_f64.powf(9.0);
    km[27] = 5.0 * 10.0_f64.powf(6.0);
    km[28] = 10.0_f64.powf(3.0);
    km[29] = 30.0;
    km[30] = 2.0 * 10.0_f64.powf(9.0);

    //równania
    let eq1 = |c: [f64; 27], km: [f64; 31]| {
        let sum = - km[1] * c[1] * c[2] * c[3] /*M1*/
        + km[2] * c[4] /*M2*/
        + km[20] * c[11] * c[19] /*M20*/
        + km[22] * c[20] * c[11] /*M22*/
        - km[26]  * c[1] * c[23] * c[23] /*M26*/
        + km[29] * c[2] * c[25]; /*M29*/
        if c[1] + sum > 0.0 {
            sum
        } else {
            -c[1]
        }
    };
    let eq2 = |c: [f64; 27], km: [f64; 31]| {
        let sum = - km[1] * c[1] * c[2] * c[3] /*M1*/
        + km[2] * c[4] /*M2*/
        + km[4] * c[7] * c[7] /*M4*/
        - km[5] * c[2] * c[5] /*M5*/
        - km[6] * c[2] * c[8] /*M6*/
        - km[24] * c[2] * c[21] /*M24*/
        - km[28] * c[2] * c[26] /*M28*/
        - km[29] * c[2] * c[25] /*M29*/
        - km[30] * c[2] * c[6]; /*M30*/
        if c[2] + sum > 0.0 {
            sum
        } else {
            -c[2]
        }
    };
    let eq3 = |c: [f64; 27], km: [f64; 31]| {
        let sum = - km[1] * c[1] * c[2] * c[3] /*M1*/
        + km[2] * c[4] /*M2*/
        - km[8] * c[9] * c[10] /*M8*/
        - km[9] * c[10] * c[10] /*M9*/
        + km[12] * c[15] /*M12*/
        - km[13] * c[3] * c[12] /*M13*/
        + km[14] * c[13] /*M14*/
        - km[15] * c[3] * c[17] /*M15*/
        + km[16] * c[16] /*M16*/
        - km[17] * c[3] * c[14] /*M17*/
        + 2.0 * km[18] * c[10] * c[18] /*M18*/
        - 2.0 * km[19] * c[19] * c[19] /*M19*/
        - 2.0 * km[23] * c[20] * c[20] /*M23*/
        - km[25] * c[9] * 10.0_f64.powf(-14.0) / c[3] /*M25*/
        - 2.0 * km[26]  * c[1] * c[23] * c[23] /*M26*/
        + 2.0 * km[29] * c[2] * c[25]; /*M29*/
        if c[3] + sum > 0.0 {
            sum
        } else {
            -c[3]
        }
    };
    let eq4 = |c: [f64; 27], km: [f64; 31]| {
        let sum = km[1] * c[1] * c[2] * c[3] /*M1*/
        - km[2] * c[4] /*M2*/
        - km[3] * c[4] /*M3*/
        - km[20] * c[11] * c[19] /*M20*/
        - km[22] * c[11] * c[20]; /*M22*/
        if c[4] + sum > 0.0 {
            sum
        } else {
            -c[4]
        }
    };
    let eq5 = |c: [f64; 27], km: [f64; 31]| {
        let sum = -km[5] * c[2] * c[5]; /*M5*/
        if c[5] + sum > 0.0 {
            sum
        } else {
            -c[5]
        }
    };
    let eq6 = |c: [f64; 27], km: [f64; 31]| {
        let sum = km[29] * c[2] * c[25] /*M29*/
        - km[30] * c[2] * c[6]; /*M30*/
        if c[6] + sum > 0.0 {
            sum
        } else {
            -c[6]
        }
    };
    let eq7 = |c: [f64; 27], km: [f64; 31]| {
        let sum = km [3] * c[4] /*M3*/
        - 2.0 * km[4] * c[7] * c[7] /*M4*/
        - km[21] * c[7] * c[18] /*M21*/
        + km[30] * c[2] * c[6]; /*M30*/
        if c[7] + sum > 0.0 {
            sum
        } else {
            -c[7]
        }
    };
    let eq8 = |c: [f64; 27], km: [f64; 31]| {
        let sum = km[5] * c[2] * c[5] /*M5*/
        - km[6] * c[2] * c[8] /*M6*/
        + km[7] * c[9] * c[9] /*M7*/
        + km[8] * c[9] * c[10]; /*M8*/
        if c[8] + sum > 0.0 {
            sum
        } else {
            -c[8]
        }
    };
    let eq9 = |c: [f64; 27], km: [f64; 31]| {
        let sum = km[6] * c[2] * c[8] /*M6*/
        - 2.0 * km[7] * c[9] * c[9] /*M7*/
        - km[8] * c[9] * c[10] /*M8*/
        - km[25] * c[9] * 10.0_f64.powf(-14.0) / c[3]; /*M25*/
        if c[9] + sum > 0.0 {
            sum
        } else {
            -c[9]
        }
    };
    let eq10 = |c: [f64; 27], km: [f64; 31]| {
        let sum = km[7] * c[9] * c[9] /*M7*/
        - km[8] * c[9] * c[10] /*M8*/
        - 2.0 * km[9] * c[10] * c[10] /*M9*/
        - km[18] * c[10] * c[18] /*M18*/
        + km[19] * c[19] * c[19]; /*M19*/
        if c[10] + sum > 0.0 {
            sum
        } else {
            -c[10]
        }
    };
    let eq11 = |c: [f64; 27], km: [f64; 31]| {
        let sum = km[3] * c[4] /*M3*/
        - km[20] * c[11] * c[19] /*M20*/
        - km[22] * c[11] * c[20]; /*M22*/
        if c[11] + sum > 0.0 {
            sum
        } else {
            -c[11]
        }
    };
    let eq12 = |c: [f64; 27], km: [f64; 31]| {
        let sum = km[8] * c[9] * c[10] /*M8*/
        + km[9] * c[10] * c[10] /*M9*/
        - km[10] * c[12] /*M10*/
        + km[12] * c[15] /*M12*/
        - km[13] * c[3] * c[12] /*M13*/
        + km[21] * c[7] * c[18] /*M21*/
        + 2.0 * km[28] * c[2] * c[26]; /*M28*/
        if c[12] + sum > 0.0 {
            sum
        } else {
            -c[12]
        }
    };
    let eq13 = |c: [f64; 27], km: [f64; 31]| {
        let sum = km[10] * c [12] /*M10*/
        + km[11] * c[15] /*M11*/
        - km[14] * c[13] /*M14*/
        + km[15] * c[3] * c[17]; /*M15*/
        if c[13] + sum > 0.0 {
            sum
        } else {
            -c[13]
        }
    };
    let eq14 = |c: [f64; 27], km: [f64; 31]| {
        let sum = km[10] * c [12] /*M10*/
        + km[16] * c[16] /*M16*/
        - km[17] * c[3] * c[14]; /*M17*/
        if c[14] + sum > 0.0 {
            sum
        } else {
            -c[14]
        }
    };
    let eq15 = |c: [f64; 27], km: [f64; 31]| {
        let sum = - km[11] * c[15] /*M11*/
        - km[12] * c[15] /*M12*/
        + km[13] * c[3] * c[12]; /*M13*/
        if c[15] + sum > 0.0 {
            sum
        } else {
            -c[15]
        }
    };
    let eq16 = |c: [f64; 27], km: [f64; 31]| {
        let sum = km[11] * c[15] /*M11*/
        -km[16] * c[16] /*M16*/
        + km[17] * c[3] * c[14]; /*M17*/
        if c[16] + sum > 0.0 {
            sum
        } else {
            -c[16]
        }
    };
    let eq17 = |c: [f64; 27], km: [f64; 31]| {
        let sum = km[14] * c[13] /*M14*/
        - km[15] * c[3] * c[17]; /*M15*/
        if c[17] + sum > 0.0 {
            sum
        } else {
            -c[17]
        }
    };
    let eq18 = |c: [f64; 27], km: [f64; 31]| {
        let sum = km[9] * c[10] * c[10] /*M9*/
        - km[18] * c[10] * c[18] /*M18*/
        + km[19] * c[19] * c[19] /*M19*/
        + km[20] * c[11] * c[19] /*M20*/
        - km[21] * c[7] * c[18]; /*M21*/
        if c[18] + sum > 0.0 {
            sum
        } else {
            -c[18]
        }
    };
    let eq19 = |c: [f64; 27], km: [f64; 31]| {
        let sum = 2.0 * km[18] * c[10] * c[18] /*M18*/
        - 2.0 * km[19] * c[19] * c[19] /*M19*/
        - km[20] * c[11] * c[19]; /*M20*/
        if c[19] + sum > 0.0 {
            sum
        } else {
            -c[19]
        }
    };
    let eq20 = |c: [f64; 27], km: [f64; 31]| {
        let sum = km[21] * c[7] * c[18] /*M21*/
        - km[22] * c[20] * c[11] /*M22*/
        - 2.0 * km[23] * c[20] * c[20]; /*M23*/
        if c[20] + sum > 0.0 {
            sum
        } else {
            -c[20]
        }
    };
    let eq21 = |c: [f64; 27], km: [f64; 31]| {
        let sum = km[22] * c[20] * c[11] /*M22*/
        + km[23] * c[20] * c[20] /*M23*/
        - km[24] * c[2] * c[21] /*M24*/
        + km[25] * c[9] * 10.0_f64.powf(-14.0) / c[3]; /*M25*/
        if c[21] + sum > 0.0 {
            sum
        } else {
            -c[21]
        }
    };
    let eq22 = |_: [f64; 27], _k: [f64; 31]| 0.0;
    let eq23 = |c: [f64; 27], km: [f64; 31]| {
        let sum = km[25] * c[9] * 10.0_f64.powf(-14.0) / c[3] /*M25*/
        - 2.0 * km[26]  * c[1] * c[23] * c[23] /*M26*/
        + km[29] * c[2] * c[25]; /*M29*/
        if c[23] + sum > 0.0 {
            sum
        } else {
            -c[23]
        }
    };
    let eq24 = |c: [f64; 27], km: [f64; 31]| {
        let sum = km[26]  * c[1] * c[23] * c[23] /*M26*/
        - 2.0 * km[27] * c[24] * c[24]; /*M27*/
        if c[24] + sum > 0.0 {
            sum
        } else {
            -c[24]
        }
    };
    let eq25 = |c: [f64; 27], km: [f64; 31]| {
        let sum = km[27] * c[24] * c[24] /*M27*/
        - km[29] * c[2] * c[25]; /*M29*/
        if c[25] + sum > 0.0 {
            sum
        } else {
            -c[25]
        }
    };
    let eq26 = |c: [f64; 27], km: [f64; 31]| {
        let sum = km[27] * c[24] * c[24] /*M27*/
        - km[28] * c[2] * c[26]; /*M28*/
        if c[26] + sum > 0.0 {
            sum
        } else {
            -c[26]
        }
    };

    let mut eqs = Vec::new();
    let mut pushinator =
        |eq: fn([f64; 27], [f64; 31]) -> f64| eqs.push(BoxedFunction::new(eq));
    pushinator(|_, _| 0.0); // niczemu nie służy, tylko po to,
                            // żeby liczba elementów była taka sama jak w conc poniżej
    pushinator(eq1);
    pushinator(eq2);
    pushinator(eq3);
    pushinator(eq4);
    pushinator(eq5);
    pushinator(eq6);
    pushinator(eq7);
    pushinator(eq8);
    pushinator(eq9);
    pushinator(eq10);
    pushinator(eq11);
    pushinator(eq12);
    pushinator(eq13);
    pushinator(eq14);
    pushinator(eq15);
    pushinator(eq16);
    pushinator(eq17);
    pushinator(eq18);
    pushinator(eq19);
    pushinator(eq20);
    pushinator(eq21);
    pushinator(eq22);
    pushinator(eq23);
    pushinator(eq24);
    pushinator(eq25);
    pushinator(eq26);

    let mut c_cu_poczatkowe = String::new();
    println!("c_cu [uM]:");
    io::stdin()
        .read_line(&mut c_cu_poczatkowe)
        .expect("stdin failed - stezenie poczatkowe");

    //stezenia
    let mut h;
    let mut t = 0.0;
    let mut conc = [0.0*10.0_f64.powf(-8.0); 27];
    conc[4] = c_cu_poczatkowe.trim_end().parse::<f64>().unwrap()
        * 10.0_f64.powf(-6.0);
    println!(
        "...c_cu = {}",
        c_cu_poczatkowe.trim_end().parse::<f64>().unwrap()
    );
    conc[2] = 0.25;
    conc[3] = 0.025;
    conc[5] = 0.025;
    //conc[25] = 10.0_f64.powf(-8.0);

    //let mut pot = potencjal_mieszany(c_ho2min, conc[5], conc[2], conc[1]);
    //println!("{t},{},{}", (pot.0), (pot.1));

    // let f = OpenOptions::new()
    //     .append(true)
    //     .open(path1win)
    //     .expect("Unable to open file");
    // let mut f = BufWriter::new(f);
    let stezenia_plik = OpenOptions::new()
        .append(true)
        .open(path2win)
        .expect("Unable to open file");
    let mut stezenia_plik = BufWriter::new(stezenia_plik);
    // f.write_all("t,Au,Pt\n".as_bytes()).expect("tragedia");
    // f.write_all(format!("{},{},{}\n", t / 60.0, pot.0, pot.1).as_bytes())
    //     .expect("tragedia");
    stezenia_plik
        .write_all("t,c1,c2,c3,c4,c5,c6,c7,c8,c9,c10,c11,c12,c13,c14,c15,c16,c17,c18,c19,c20,c21,c22,c23,c24,c25,c26\n".as_bytes())
        .expect("tragedia stezenia");
    let zapisy_na_sekunde = 10.0;
    

    //let mut switch = true;
    while t < 10000.0 {
        if t < 5.0 * 10.0_f64.powf(0.0) {
            h = 10.0_f64.powf(-8.0)
        } else if t < 10_f64.powf(2.0) {
            h = 10.0_f64.powf(-7.0)
        } else {
            h = 10.0_f64.powf(-7.0)
        }
        t += h;
        // if t > 2400.0 && switch {
        //     conc[11] = 3.0 * 10.0_f64.powf(-5.0);
        //     h = 10.0_f64.powf(-5.0);
        //     switch = false;
        // } else if t > 2700.0 {
        //     if h < 10.0_f64.powf(-3.0) {
        //         h = h * 1.1;
        //         //println!("{h}");
        //     }
        // }

        conc = rk4(conc, h, &eqs, &mut km);
        if conc[1] > 0.001 || conc[1].is_nan() {
            println!("{}, {}, {}", t / 60.0, conc[1], km[2]);
            break;
        }
        if (zapisy_na_sekunde * (t + h)).floor()
            >= (zapisy_na_sekunde * t).ceil()
        {
            //println!("{}, {}, {}", t / 60.0, conc[1], km[2]);
            //println!("{t},{},{},{:?}", (pot.0), (pot.1), conc);
            // f.write_all(format!("{},{},{}\n", t / 60.0, pot.0, pot.1).as_bytes())
            // .expect("tragedia");
            stezenia_plik
                .write_all(
                    format!(
                        "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},\n",
                        t / 60.0,
                        conc[1],
                        conc[2],
                        conc[3],
                        conc[4],
                        conc[5],
                        conc[6],
                        conc[7],
                        conc[8],
                        conc[9],
                        conc[10],
                        conc[11],
                        conc[12],
                        conc[13],
                        conc[14],
                        conc[15],
                        conc[16],
                        conc[17],
                        conc[18],
                        conc[19],
                        conc[20],
                        conc[21],
                        conc[22],
                        conc[23],
                        conc[24],
                        conc[25],
                        conc[26]
                    )
                    .as_bytes(),
                )
                .expect("tragedia stezenia");
        }
    }
}

// fn _euler(conc: [f64; 27], h: f64, eqs: &A, c_h2o2: f64, c_ho2min: f64, c_scn: f64) -> [f64; 27] {
//     let k1s: Vec<f64> = (0usize..27usize)
//         .map(|i| h * (*eqs.b_vec[i].f)(conc))
//         .collect();
//     let new_concs = zip!(conc, k1s)
//         .map(|(c, k1)| c + k1)
//         .collect::<Vec<_>>()
//         .try_into()
//         .unwrap();
//     new_concs
// }

fn rk4(
    conc: [f64; 27],
    h: f64,
    eqs: &[BoxedFunction],
    km: &mut [f64; 31],
) -> [f64; 27] {
    km[2] = km[1]
        / ((2.4 * 100.0 * 4.8 * 10.0_f64.powf(17.0) * conc[3].powf(4.0)
            + 3.3 * 10.0_f64.powf(17.0) * conc[3].powf(3.0))
            / (1.0 + 2.4 * 100.0 * conc[3]));
    let kxconculator = |kxs: &Vec<f64>, multiplier| {
        conc.iter()
            .zip(kxs)
            .map(|(c, k)| c + k * multiplier)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    };
    let kxer = |kxconcs: [f64; 27]| {
        (0usize..27usize)
            .map(|i| h * (*eqs[i].f)(kxconcs, *km))
            .collect::<Vec<f64>>()
    };
    let k1s: Vec<f64> = kxer(conc);
    let k2concs = kxconculator(&k1s, 0.5);
    let k2s: Vec<f64> = kxer(k2concs);
    let k3concs = kxconculator(&k2s, 0.5);
    let k3s: Vec<f64> = kxer(k3concs);
    let k4concs = kxconculator(&k3s, 1.0);
    let k4s: Vec<f64> = kxer(k4concs);

    let mut new_concs: [f64; 27] = zip!(conc, k1s, k2s, k3s, k4s)
        .map(|(c, (k1, (k2, (k3, k4))))| {
            c + (k1 + 2.0 * k2 + 2.0 * k3 + k4) / 6.0
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    new_concs[22] = 10.0_f64.powf(-14.0) / new_concs[3];
    new_concs
}

// fn potencjal_mieszany(c_ho2min: f64, c_ho2rod: f64, c_cuoh3: f64, c_cuoh2: f64) -> (f64, f64) {
//     const F: f64 = 96485.3321;
//     const R: f64 = 8.314462;
//     const T: f64 = 283.15;

//     let e1 = -0.18 + R * T / F * (c_cuoh3 / c_cuoh2).ln();
//     let e2 = 0.22 + R * T / F * (c_ho2rod / c_ho2min).ln();
//     let i01 = F * c_cuoh3.powf(0.5) * c_cuoh2.powf(0.5);
//     let i02pt = 10.0_f64.powf(-5.0) * F * c_ho2min.powf(0.5) * c_ho2rod.powf(0.5);
//     let i02au = 10.0_f64.powf(-8.0) * F * c_ho2min.powf(0.5) * c_ho2rod.powf(0.5);

//     let e_pt = R * T / F
//         * ((i01 * (F * e1 / 2.0 / R / T).exp() + i02pt * (F * e2 / 2.0 / R / T).exp())
//             / (i01 * (-F * e1 / 2.0 / R / T).exp() + i02pt * (-F * e2 / 2.0 / R / T).exp()))
//         .ln();
//     let e_au = R * T / F
//         * ((i01 * (F * e1 / 2.0 / R / T).exp() + i02au * (F * e2 / 2.0 / R / T).exp())
//             / (i01 * (-F * e1 / 2.0 / R / T).exp() + i02au * (-F * e2 / 2.0 / R / T).exp()))
//         .ln();
//     (e_au, e_pt)
// }

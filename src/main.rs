use std::fs::OpenOptions;
use std::io::{self, BufWriter, Write};
use ode_solvers::Dop853;
type State = ode_solvers::SVector<f64, 27>;
type Time = f64;

struct Oscillator {
    km1: f64,
    km2: f64,
    km3: f64,
    km4: f64,
    km5: f64,
    km6: f64,
    km7: f64,
    km8: f64,
    km9: f64,
    km10: f64,
    km11: f64,
    km12: f64,
    km13: f64,
    km14: f64,
    km15: f64,
    km16: f64,
    km17: f64,
    km18: f64,
    km19: f64,
    km20: f64,
    km21: f64,
    km22: f64,
    km23: f64,
    km24: f64,
    km25: f64,
    km26: f64,
    km27: f64,
    km28: f64,
    km29: f64,
    km30: f64,
}
impl ode_solvers::System<State> for Oscillator {
    // Equations of motion of the system
    fn system(&self, _t: Time, c: &State, dc: &mut State) {
        let km_2 = self.km1
            / ((2.4 * 100.0 * 4.8 * 10.0_f64.powf(17.0) * c[3].powf(4.0)
                + 3.3 * 10.0_f64.powf(17.0) * c[3].powf(3.0))
                / (1.0 + 2.4 * 100.0 * c[3]));

        dc[0] = - self.km1 * c[1] * c[2] * c[3] /*M1*/
        + km_2 * c[4] /*M2*/
        + self.km20 * c[11] * c[19] /*M20*/
        + self.km22 * c[20] * c[11] /*M22*/
        - self.km26  * c[1] * c[23] * c[23] /*M26*/
        + self.km29 * c[2] * c[25]; /*M29*/
        dc[1] = - self.km1 * c[1] * c[2] * c[3] /*M1*/
        + km_2 * c[4] /*M2*/
        + self.km20 * c[11] * c[19] /*M20*/
        + self.km22 * c[20] * c[11] /*M22*/
        - self.km26  * c[1] * c[23] * c[23] /*M26*/
        + self.km29 * c[2] * c[25]; /*M29*/
        dc[2] = - self.km1 * c[1] * c[2] * c[3] /*M1*/
            + km_2 * c[4] /*M2*/
            + self.km4 * c[7] * c[7] /*M4*/
            - self.km5 * c[2] * c[5] /*M5*/
            - self.km6 * c[2] * c[8] /*M6*/
            - self.km24 * c[2] * c[21] /*M24*/
            - self.km28 * c[2] * c[26] /*M28*/
            - self.km29 * c[2] * c[25] /*M29*/
            - self.km30 * c[2] * c[6]; /*M30*/
        dc[3] = - self.km1 * c[1] * c[2] * c[3] /*M1*/
            + km_2 * c[4] /*M2*/
            - self.km8 * c[9] * c[10] /*M8*/
            - self.km9 * c[10] * c[10] /*M9*/
            + self.km12 * c[15] /*M12*/
            - self.km13 * c[3] * c[12] /*M13*/
            + self.km14 * c[13] /*M14*/
            - self.km15 * c[3] * c[17] /*M15*/
            + self.km16 * c[16] /*M16*/
            - self.km17 * c[3] * c[14] /*M17*/
            + 2.0 * self.km18 * c[10] * c[18] /*M18*/
            - 2.0 * self.km19 * c[19] * c[19] /*M19*/
            - 2.0 * self.km23 * c[20] * c[20] /*M23*/
            - self.km25 * c[9] * 10.0_f64.powf(-14.0) / c[3] /*M25*/
            - 2.0 * self.km26  * c[1] * c[23] * c[23] /*M26*/
            + 2.0 * self.km29 * c[2] * c[25]; /*M29*/
        dc[4] = self.km1 * c[1] * c[2] * c[3] /*M1*/
            - km_2 * c[4] /*M2*/
            - self.km3 * c[4] /*M3*/
            - self.km20 * c[11] * c[19] /*M20*/
            - self.km22 * c[11] * c[20]; /*M22*/
        dc[5] = -self.km5 * c[2] * c[5]; /*M5*/
        dc[6] = self.km29 * c[2] * c[25] /*M29*/
            - self.km30 * c[2] * c[6]; /*M30*/
        dc[7] = self.km3 * c[4] /*M3*/
            - 2.0 * self.km4 * c[7] * c[7] /*M4*/
            - self.km21 * c[7] * c[18] /*M21*/
            + self.km30 * c[2] * c[6]; /*M30*/
        dc[8] = self.km5 * c[2] * c[5] /*M5*/
            - self.km6 * c[2] * c[8] /*M6*/
            + self.km7 * c[9] * c[9] /*M7*/
            + self.km8 * c[9] * c[10]; /*M8*/
        dc[9] = self.km6 * c[2] * c[8] /*M6*/
            - 2.0 * self.km7 * c[9] * c[9] /*M7*/
            - self.km8 * c[9] * c[10] /*M8*/
            - self.km25 * c[9] * 10.0_f64.powf(-14.0) / c[3]; /*M25*/
        dc[10] = self.km7 * c[9] * c[9] /*M7*/
            - self.km8 * c[9] * c[10] /*M8*/
            - 2.0 * self.km9 * c[10] * c[10] /*M9*/
            - self.km18 * c[10] * c[18] /*M18*/
            + self.km19 * c[19] * c[19]; /*M19*/
        dc[11] = self.km3 * c[4] /*M3*/
            - self.km20 * c[11] * c[19] /*M20*/
            - self.km22 * c[11] * c[20]; /*M22*/
        dc[12] = self.km8 * c[9] * c[10] /*M8*/
            + self.km9 * c[10] * c[10] /*M9*/
            - self.km10 * c[12] /*M10*/
            + self.km12 * c[15] /*M12*/
            - self.km13 * c[3] * c[12] /*M13*/
            + self.km21 * c[7] * c[18] /*M21*/
            + 2.0 * self.km28 * c[2] * c[26]; /*M28*/
        dc[13] = self.km10 * c [12] /*M10*/
            + self.km11 * c[15] /*M11*/
            - self.km14 * c[13] /*M14*/
            + self.km15 * c[3] * c[17]; /*M15*/
        dc[14] = self.km10 * c [12] /*M10*/
            + self.km16 * c[16] /*M16*/
            - self.km17 * c[3] * c[14]; /*M17*/
        dc[15] = - self.km11 * c[15] /*M11*/
            - self.km12 * c[15] /*M12*/
            + self.km13 * c[3] * c[12]; /*M13*/
        dc[16] = self.km11 * c[15] /*M11*/
            -self.km16 * c[16] /*M16*/
            + self.km17 * c[3] * c[14]; /*M17*/
        dc[17] = self.km14 * c[13] /*M14*/
            - self.km15 * c[3] * c[17]; /*M15*/
        dc[18] = self.km9 * c[10] * c[10] /*M9*/
            - self.km18 * c[10] * c[18] /*M18*/
            + self.km19 * c[19] * c[19] /*M19*/
            + self.km20 * c[11] * c[19] /*M20*/
            - self.km21 * c[7] * c[18]; /*M21*/
        dc[19] = 2.0 * self.km18 * c[10] * c[18] /*M18*/
            - 2.0 * self.km19 * c[19] * c[19] /*M19*/
            - self.km20 * c[11] * c[19]; /*M20*/
        dc[20] = self.km21 * c[7] * c[18] /*M21*/
            - self.km22 * c[20] * c[11] /*M22*/
            - 2.0 * self.km23 * c[20] * c[20]; /*M23*/
        dc[21] = self.km22 * c[20] * c[11] /*M22*/
            + self.km23 * c[20] * c[20] /*M23*/
            - self.km24 * c[2] * c[21] /*M24*/
            + self.km25 * c[9] * 10.0_f64.powf(-14.0) / c[3]; /*M25*/
        dc[22] = 0.0;
        dc[23] = self.km25 * c[9] * 10.0_f64.powf(-14.0) / c[3] /*M25*/
            - 2.0 * self.km26  * c[1] * c[23] * c[23] /*M26*/
            + self.km29 * c[2] * c[25]; /*M29*/
        dc[24] = self.km26  * c[1] * c[23] * c[23] /*M26*/
            - 2.0 * self.km27 * c[24] * c[24]; /*M27*/
        dc[25] = self.km27 * c[24] * c[24] /*M27*/
            - self.km29 * c[2] * c[25]; /*M29*/
        dc[26] = self.km27 * c[24] * c[24] /*M27*/
            - self.km28 * c[2] * c[26]; /*M28*/
    }
}
fn main() {
    //stałe
    let km1 = 10.0_f64.powf(5.0);
    let km2 = 0.0;
    let km3 = 10.0_f64.powf(-2.0);
    let km4 = 20.0;
    let km5 = 7.5 * 10.0_f64.powf(-4.0);
    let km6 = 0.3;
    let km7 = 5.0 * 10.0_f64.powf(2.0);
    let km8 = 10.0_f64.powf(3.0);
    let km9 = 1.0;
    let km10 = 0.1;
    let km11 = 10.0_f64.powf(-3.0);
    let km12 = 5.0 * 10.0_f64.powf(-2.0);
    let km13 = 10.0_f64.powf(9.0);
    let km14 = 25.1;
    let km15 = 10.0_f64.powf(9.0);
    let km16 = 1.62 * 10.0_f64.powf(6.0);
    let km17 = 10.0_f64.powf(9.0);
    let km18 = 3.0 * 10.0_f64.powf(3.0);
    let km19 = 2.0 * 10.0_f64.powf(6.0);
    let km20 = 10.0_f64.powf(5.0);
    let km21 = 2.0 * 10.0_f64.powf(3.0);
    let km22 = 1.5 * 10.0_f64.powf(5.0);
    let km23 = 10.0_f64.powf(8.0);
    let km24 = 0.2;
    let km25 = 10.0_f64.powf(6.0);
    let km26 = 10.0_f64.powf(9.0);
    let km27 = 5.0 * 10.0_f64.powf(6.0);
    let km28 = 10.0_f64.powf(3.0);
    let km29 = 30.0;
    let km30 = 2.0 * 10.0_f64.powf(9.0);

    let system = Oscillator {
        km1,
        km2,
        km3,
        km4,
        km5,
        km6,
        km7,
        km8,
        km9,
        km10,
        km11,
        km12,
        km13,
        km14,
        km15,
        km16,
        km17,
        km18,
        km19,
        km20,
        km21,
        km22,
        km23,
        km24,
        km25,
        km26,
        km27,
        km28,
        km29,
        km30,
    };

    let mut c_cu_poczatkowe = String::new();
    println!("c_cu [uM]:");
    io::stdin()
        .read_line(&mut c_cu_poczatkowe)
        .expect("stdin failed - stezenie poczatkowe");

    let mut conc = [10.0_f64.powf(-19.0); 27];
    conc[1] = c_cu_poczatkowe.trim_end().parse::<f64>().unwrap() * 10.0_f64.powf(-6.0);
    println!(
        "...c_cu = {}",
        c_cu_poczatkowe.trim_end().parse::<f64>().unwrap()
    );
    conc[2] = 0.25;
    conc[3] = 0.025;
    conc[5] = 0.025;
    let c0 = State::from(conc);

    let mut stepper = Dop853::from_param(system, 0.0, 1000.0, 1.0, c0, 1.0e-14, 1.0e-14, 0.9, 0.0, 0.333,6.0,0.001,10.0_f64.powf(-11.0),10_u32.pow(9),1000,ode_solvers::dop_shared::OutputType::Dense);
    let res = stepper.integrate();

    let path2win = r"C:\Users\admin\Desktop\MTHOMAS\x\model26zmienny\stezenia.csv";

    let stezenia_plik = OpenOptions::new()
        .append(true)
        .open(path2win)
        .expect("Unable to open file");

    let mut stezenia_plik = BufWriter::new(stezenia_plik);
    stezenia_plik
    .write_all("t,c1,c2,c3,c4,c5,c6,c7,c8,c9,c10,c11,c12,c13,c14,c15,c16,c17,c18,c19,c20,c21,c22,c23,c24,c25,c26\n".as_bytes())
    .expect("tragedia stezenia");

    // Handle result
    match res {
        Ok(stats) => {
            stezenia_plik
            .write_all(format!("{:?}", stepper.x_out()).as_bytes())
            .expect("tragedia stezenia");

            // Do something with the output...
            // let path = Path::new("./outputs/kepler_orbit_dopri5.dat");
            // save(stepper.x_out(), stepper.y_out(), path);
            // println!("Results saved in: {:?}", path);
        }
        Err(_) => println!("An error    occured."),
    }
}

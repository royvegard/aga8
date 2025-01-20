use aga8::composition::Composition;
use aga8::detail::Detail;
use aga8::gerg2008::Gerg2008;
use std::fs;

#[macro_use]
extern crate assert_float_eq;

#[test]
fn binary_mixtures() {
    let mut composition: Composition;
    let mut detail = Detail::new();
    let mut gerg = Gerg2008::new();

    for l in fs::read_to_string("tests/test_data.csv").unwrap().lines() {
        //println!("{}", l);

        if l.starts_with("###") {
            // Skip comments
            continue;
        } else if l.starts_with("# ") {
            // Compositions start with one #
            composition = Composition {
                ..Default::default()
            };

            let comp: Vec<&str> = l.split(' ').nth(1).unwrap().split(',').collect();

            for c in comp {
                match c {
                    "methane" => composition.methane = 0.5,
                    "nitrogen" => composition.nitrogen = 0.5,
                    "carbon_dioxide" => composition.carbon_dioxide = 0.5,
                    "ethane" => composition.ethane = 0.5,
                    "propane" => composition.propane = 0.5,
                    "isobutane" => composition.isobutane = 0.5,
                    "n_butane" => composition.n_butane = 0.5,
                    "isopentane" => composition.isopentane = 0.5,
                    "n_pentane" => composition.n_pentane = 0.5,
                    "hexane" => composition.hexane = 0.5,
                    "heptane" => composition.heptane = 0.5,
                    "octane" => composition.octane = 0.5,
                    "nonane" => composition.nonane = 0.5,
                    "decane" => composition.decane = 0.5,
                    "hydrogen" => composition.hydrogen = 0.5,
                    "oxygen" => composition.oxygen = 0.5,
                    "carbon_monoxide" => composition.carbon_monoxide = 0.5,
                    "water" => composition.water = 0.5,
                    "hydrogen_sulfide" => composition.hydrogen_sulfide = 0.5,
                    "helium" => composition.helium = 0.5,
                    "argon" => composition.argon = 0.5,
                    _ => panic!("Invalid component {}", c),
                }
            }
            gerg.set_composition(&composition).unwrap();
            detail.set_composition(&composition).unwrap();
            continue;
        } else {
            let mut case = l.split(',');
            let t = case.next().unwrap().parse::<f64>().unwrap();
            let d = case.next().unwrap().parse::<f64>().unwrap();
            case.next();

            let gerg_p = case.next().unwrap().parse::<f64>().unwrap();
            let gerg_cv = case.next().unwrap().parse::<f64>().unwrap();
            let gerg_cp = case.next().unwrap().parse::<f64>().unwrap();
            let gerg_w = case.next().unwrap().parse::<f64>().unwrap();
            case.next();

            let detail_p = case.next().unwrap().parse::<f64>().unwrap();
            let detail_cv = case.next().unwrap().parse::<f64>().unwrap();
            let detail_cp = case.next().unwrap().parse::<f64>().unwrap();
            let detail_w = case.next().unwrap().parse::<f64>().unwrap();

            println!("t: {}, d: {}", t, d);

            gerg.t = t;
            gerg.d = d;
            gerg.p = gerg.pressure();
            gerg.properties();

            assert_float_relative_eq!(gerg.p / 1000.0, gerg_p, 1.0e-5);
            assert_float_relative_eq!(gerg.cv, gerg_cv, 1.0e-5);
            assert_float_relative_eq!(gerg.cp, gerg_cp, 1.0e-5);
            assert_float_relative_eq!(gerg.w, gerg_w, 1.0e-5);

            detail.t = t;
            detail.d = d;
            detail.p = detail.pressure();
            detail.properties();

            assert_float_relative_eq!(detail.p / 1000.0, detail_p, 1.0e-5);
            assert_float_relative_eq!(detail.cv, detail_cv, 1.0e-5);
            assert_float_relative_eq!(detail.cp, detail_cp, 1.0e-5);
            assert_float_relative_eq!(detail.w, detail_w, 1.0e-5);
        }
    }
}

//#[test]
fn natural_gas_calculations() {
    let mut compositions: Vec<Composition> = Vec::new();
    let mut detail = Detail::new();
    let mut gerg = Gerg2008::new();
    const DETAIL_EPSILON: f64 = 1.0e-3;
    const GERG_EPSILON: f64 = 1.0e-3;

    for l in fs::read_to_string("tests/natural_gas_compositions.csv")
        .unwrap()
        .lines()
    {
        if l.starts_with('#') {
            continue;
        }

        let comp: Vec<f64> = l.split(',').map(|n| n.parse::<f64>().unwrap()).collect();
        let mut composition = Composition {
            methane: comp[0],
            nitrogen: comp[1],
            carbon_dioxide: comp[2],
            ethane: comp[3],
            propane: comp[4],
            isobutane: comp[5],
            n_butane: comp[6],
            isopentane: comp[7],
            n_pentane: comp[8],
            hexane: comp[9],
            heptane: comp[10],
            octane: comp[11],
            nonane: comp[12],
            decane: comp[13],
            hydrogen_sulfide: comp[14],
            helium: comp[15],
            water: comp[16],
            oxygen: comp[17],
            argon: comp[18],
            hydrogen: comp[19],
            carbon_monoxide: comp[20],
        };
        composition.normalize().unwrap();
        compositions.push(composition);
    }

    //println!("{:?}", compositions);

    for l in fs::read_to_string("tests/natural_gas_calculations.csv")
        .unwrap()
        .lines()
    {
        //println!("{}", l);

        if l.starts_with("###") {
            continue;
        }

        let mut case = l.split(',');
        let index = case.next().unwrap().parse::<usize>().unwrap();
        let t = case.next().unwrap().parse::<f64>().unwrap();
        let d = case.next().unwrap().parse::<f64>().unwrap();
        case.next();

        let gerg_p = case.next().unwrap().parse::<f64>().unwrap();
        let gerg_cv = case.next().unwrap().parse::<f64>().unwrap();
        let gerg_cp = case.next().unwrap().parse::<f64>().unwrap();
        let gerg_w = case.next().unwrap().parse::<f64>().unwrap();
        case.next();

        let detail_p = case.next().unwrap().parse::<f64>().unwrap();
        let detail_cv = case.next().unwrap().parse::<f64>().unwrap();
        let detail_cp = case.next().unwrap().parse::<f64>().unwrap();
        let detail_w = case.next().unwrap().parse::<f64>().unwrap();
        //println!("index: {}, t: {}, d: {}", index, t, d);

        gerg.set_composition(&compositions[index - 2]).unwrap();
        gerg.t = t;
        gerg.d = d;
        gerg.p = gerg.pressure();
        gerg.properties();
        // assert_float_relative_eq!(gerg.p / 1000.0, gerg_p, GERG_EPSILON);
        // assert_float_relative_eq!(gerg.cv, gerg_cv, GERG_EPSILON);
        // assert_float_relative_eq!(gerg.cp, gerg_cp, GERG_EPSILON);
        // assert_float_relative_eq!(gerg.w, gerg_w, GERG_EPSILON);

        let scale_max = 0.50;
        println!("comp: {index}");
        println!("gerg:");
        println!(" p: {}", gerg.p / 1000.0 - gerg_p);
        //let diff = (gerg.p / 1000.0 - gerg_p).abs() / gerg_p;
        println!(
            "{}",
            print_bar((gerg.p / 1000.0 - gerg_p).abs() / gerg_p, 0.0, scale_max)
        );
        println!(" cv: {}", gerg.cv - gerg_cv);
        println!(
            "{}",
            print_bar((gerg.cv - gerg_cv).abs() / gerg_cv, 0.0, scale_max)
        );
        println!(" cp: {}", gerg.cp - gerg_cp);
        println!(
            "{}",
            print_bar((gerg.cp - gerg_cp).abs() / gerg_cp, 0.0, scale_max)
        );
        println!(" w: {}", gerg.w - gerg_w);
        println!(
            "{}",
            print_bar((gerg.w - gerg_w).abs() / gerg_w, 0.0, scale_max)
        );

        detail.set_composition(&compositions[index - 2]).unwrap();
        detail.t = t;
        detail.d = d;
        detail.p = detail.pressure();
        detail.properties();
        // assert_float_relative_eq!(detail.p / 1000.0, detail_p, DETAIL_EPSILON);
        // assert_float_relative_eq!(detail.cv, detail_cv, DETAIL_EPSILON);
        // assert_float_relative_eq!(detail.cp, detail_cp, DETAIL_EPSILON);
        // assert_float_relative_eq!(detail.w, detail_w, DETAIL_EPSILON);

        println!("detail:");
        println!(" p: {}", detail.p / 1000.0 - detail_p);
        println!(" cv: {}", detail.cv - detail_cv);
        println!(" cp: {}", detail.cp - detail_cp);
        println!(" w: {}", detail.w - detail_w);
    }
    assert!(false);
}

fn print_bar(value: f64, scale_min: f64, scale_max: f64) -> String {
    let mut bar: String = "".to_string();
    let columns = 225;
    let range = scale_max - scale_min;
    let step = range / columns as f64;
    let mut length = (value.abs() / step) as u16;
    if length > columns {
        length = columns;
    }

    for i in 0..length {
        bar.push('#');
        if i == columns - 1 {
            bar.push('>');
        }
    }

    bar
}

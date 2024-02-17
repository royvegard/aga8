use aga8::composition::Composition;
use aga8::detail::Detail;
use aga8::gerg2008::Gerg2008;
use std::fs;

#[macro_use]
extern crate assert_float_eq;

#[test]
fn test_cases() {
    let mut composition: Composition;
    let mut detail = Detail::new();
    let mut gerg = Gerg2008::new();

    for l in fs::read_to_string("tests/test_data.csv").unwrap().lines() {
        //println!("{}", l);

        if l.starts_with("###") {
            continue;
        } else if l.starts_with("# ") {
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
            println!("{:?}", composition);
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

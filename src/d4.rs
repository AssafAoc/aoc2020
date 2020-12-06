use std::iter;
use std::collections::HashMap;

#[allow(dead_code)]
const TEST: &str = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#;

#[allow(dead_code)]
const TEST2: &str = r#"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719

eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"#;

#[allow(dead_code)]
pub fn run() {
    let input = super::get_input(4, "").collect::<Vec<_>>();
    // let input = TEST.lines().map(|s| s.to_owned()).collect::<Vec<_>>();
    // let input = TEST2.lines().map(|s| s.to_owned()).collect::<Vec<_>>();

    let passports = input.split(|l| l.len() == 0);

    let mut good_passports_a = 0;
    let mut good_passports_b = 0;

    for passport_info in passports {
        let mut it: Box<dyn Iterator<Item=&str>> = Box::new(iter::empty());
        for passport_line in passport_info {
            let splitted = passport_line.split(' ');
            let new = it.chain(splitted);
            it = Box::new(new);
        }
        let mut passport_infoids = HashMap::with_capacity(8);
        for infoid in it {
            let mut infoid_it = infoid.split(':');
            let infoid_name = infoid_it.next().unwrap();
            passport_infoids.insert(infoid_name, infoid_it.next().unwrap());
        }
        if let Some(byr) = passport_infoids.get("byr") {
            if let Some(iyr) = passport_infoids.get("iyr") {
                if let Some(eyr) = passport_infoids.get("eyr") {
                    if let Some(hgt) = passport_infoids.get("hgt") {
                        if let Some(hcl) = passport_infoids.get("hcl") {
                            if let Some(ecl) = passport_infoids.get("ecl") {
                                if let Some(pid) = passport_infoids.get("pid") {
                                    good_passports_a += 1;
                                    if let Ok(parsed_byr) = byr.parse::<u16>() {
                                        if parsed_byr >= 1920 && parsed_byr <= 2002 {
                                            if let Ok(parsed_iyr) = iyr.parse::<u16>() {
                                                if parsed_iyr >= 2010 && parsed_iyr <= 2020 {
                                                    if let Ok(parsed_hgt) = hgt[..hgt.len() - 2].parse::<u16>() {
                                                        if (&hgt[hgt.len() - 2..] == "cm" && parsed_hgt >= 150 && parsed_hgt <= 193) || (&hgt[hgt.len() - 2..] == "in" && parsed_hgt >= 59 && parsed_hgt <= 76) {
                                                            if let Ok(parsed_eyr) = eyr.parse::<u16>() {
                                                                if parsed_eyr >= 2020 && parsed_eyr <= 2030 {
                                                                    let mut hcl_chars = hcl.chars();
                                                                    if hcl_chars.next().unwrap() == '#' && u32::from_str_radix(hcl_chars.as_str(), 16).is_ok() {
                                                                        if ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(ecl) {
                                                                            if pid.len() == 9 && pid.parse::<u32>().is_ok() {
                                                                                good_passports_b += 1;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("a: {}", good_passports_a);
    println!("b: {}", good_passports_b);
}

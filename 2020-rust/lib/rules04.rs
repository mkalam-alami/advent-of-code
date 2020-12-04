use regex::Regex;

lazy_static! {
  static ref HCL: Regex = Regex::new("^#[0-9a-z]{1,6}$").unwrap();
  static ref PID: Regex = Regex::new("^[0-9]{9}$").unwrap();
}


// byr (Birth Year) - four digits; at least 1920 and at most 2002.
pub fn is_byr_valid(byr: &str) -> bool {
    let year = byr.parse::<i64>().unwrap_or(-1);
    return year >= 1920 && year <= 2002;
}

// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
pub fn is_iyr_valid(iyr: &str) -> bool {
    let year = iyr.parse::<i32>().unwrap_or(-1);
    return year >= 2010 && year <= 2020;
}

// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
pub fn is_eyr_valid(eyr: &str) -> bool {
    let year = eyr.parse::<i32>().unwrap_or(-1);
    return year >= 2020 && year <= 2030;
}

// hgt (Height) - a number followed by either cm or in:
// If cm, the number must be at least 150 and at most 193.
// If in, the number must be at least 59 and at most 76.
pub fn is_hgt_valid(hgt: &str) -> bool {
    if hgt.len() > 2 {
        let height = hgt[..(hgt.len() - 2)].parse::<i64>().unwrap_or(-1);
        if hgt.ends_with("cm") {
            return height >= 150 && height <= 193;
        } else if hgt.ends_with("in") {
            return height >= 59 && height <= 76;
        }
    }
    return false;
}

// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
pub fn is_hcl_valid(hcl: &str) -> bool {
  return HCL.is_match(hcl);
}

// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
pub fn is_ecl_valid(ecl: &str) -> bool {
  return ["amb","blu","brn","gry","grn","hzl","oth"].contains(&ecl);
}

// pid (Passport ID) - a nine-digit number, including leading zeroes.
pub fn is_pid_valid(pid: &str) -> bool {
  return PID.is_match(pid);
}

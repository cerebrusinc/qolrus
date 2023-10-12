use rand::Rng;

fn to_rgb(hex: String) -> [u32; 3] {
    let r = u32::from_str_radix(&hex[0..2], 16).expect("[to_rgb] => failed to parse 'r'");
    let g = u32::from_str_radix(&hex[2..4], 16).expect("[to_rgb] => failed to parse 'g'");
    let b = u32::from_str_radix(&hex[4..6], 16).expect("[to_rgb] => failed to parse 'b'");

    [r, g, b]
}

#[derive(Debug, PartialEq, Eq)]
enum ColourSetting {
    CMYK,
    HS,
}

fn compute_color(r: u32, g: u32, b: u32, setting: ColourSetting) -> [u32; 5] {
    let res_array = match setting {
        ColourSetting::CMYK => {
            let cc = (1 - r) / 255;
            let cm = (1 - g) / 255;
            let cy = (1 - b) / 255;
            let ck = std::cmp::min(cc, std::cmp::min(cm, cy));

            [cc, cm, cy, ck, 0]
        }
        ColourSetting::HS => {
            let cr = r / 255;
            let cg = g / 255;
            let cb = b / 255;
            let min = std::cmp::min(cr, std::cmp::min(cg, cb));
            let max = std::cmp::min(cr, std::cmp::min(cg, cb));

            [cr, cg, cb, min, max]
        }
    };

    res_array
}

fn to_cmyk(hex: String) -> [String; 4] {
    let [r, g, b] = to_rgb(hex);

    let mut res_value = [
        "0%".to_owned(),
        "0%".to_owned(),
        "0%".to_owned(),
        "100%".to_owned(),
    ];

    if r != 0 && g != 0 && b != 0 {
        let [cc, cm, cy, ck, _] = compute_color(r, g, b, ColourSetting::CMYK);

        let dc = (cc - ck) / (1 - ck);
        let dm = (cm - ck) / (1 - ck);
        let dy = (cy - ck) / (1 - ck);

        let c = dc * 100;
        let m = dm * 100;
        let y = dy * 100;
        let k = ck * 100;

        res_value = [
            format!("{}%", c),
            format!("{}%", m),
            format!("{}%", y),
            format!("{}%", k),
        ]
    }

    res_value
}

fn to_hsv(hex: String) -> [String; 3] {
    let [r, g, b] = to_rgb(hex);
    let [cr, cg, cb, min, cv] = compute_color(r, g, b, ColourSetting::HS);

    let mut return_value = ["0".to_owned(), "0%".to_owned(), format!("{}%", min)];

    if min != cv {
        let x = match cr == min {
            true => cg - cb,
            _ => {
                if cb == min {
                    cr - cg
                } else {
                    cb - cr
                }
            }
        };

        let y = match cr == min {
            true => 3,
            _ => {
                if cb == min {
                    1
                } else {
                    5
                }
            }
        };

        let h = 60 * (y - x / (cv - min));
        let cs = (cv - min) / cv;

        let s = cs * 100;
        let v = cv * 100;

        return_value = [format!("{}", h), format!("{}%", s), format!("{}%", v)]
    }

    return_value
}

fn to_hsl(hex: String) -> [String; 3] {
    let [r, g, b] = to_rgb(hex);
    let [cr, cg, cb, min, max] = compute_color(r, g, b, ColourSetting::HS);

    let cl = min + max;
    let l = cl * 50;

    let mut return_value = ["0".to_owned(), "0%".to_owned(), format!("{}%", l)];

    if min != max {
        let x = match cr == min {
            true => cg - cb,
            _ => {
                if cb == min {
                    cr - cg
                } else {
                    cb - cr
                }
            }
        };

        let y = match cr == min {
            true => 3,
            _ => {
                if cb == min {
                    1
                } else {
                    5
                }
            }
        };

        let condition = f64::from(cl / 2);

        let h = 60 * (y - x / (max - min));
        let cs = match condition > 0.5 {
            true => (max - min) / (2 - max - min),
            _ => (max - min) / (max + min),
        };

        let s = cs * 100;

        return_value = [format!("{}", h), format!("{}%", s), format!("{}%", l)]
    }

    return_value
}

///
/// This `enum` is your main interface with the `random_colour` function. It also serves as a neat way to show all the colour types we can do.
///
/// #### Interface
///
/// ```rs
/// enum ColourType {
///     HEX,
///     RGB,
///     CMYK,
///     HSV,
///     HSL,
/// }
/// ```
///
#[derive(Debug, PartialEq, Eq)]
pub enum ColourType {
    HEX,
    RGB,
    CMYK,
    HSV,
    HSL,
}

///
/// Generate a random colour.
///
/// #### Example
///
/// ```rs
/// use qolpy::{random_colour, ColourType};
///
/// fn main() {
///     let colour: String = random_colour(ColourType::HEX);
///     println!("The colour is: '{}'", colour)
///     // The colour is: '#f7f7f7'
/// }
/// ```
///
/// #### Args
/// - setting: `ColourType`
///
/// #### Returns
/// a `String` in the colour format of your choice
///
pub fn random_colour(setting: ColourType) -> String {
    let mut rng = rand::thread_rng();
    let mut h = String::new();

    for _ in 0..6 {
        let num: u8 = rng.gen_range(0..=16);
        h.push_str(&format!("{:x}", num));
    }

    let return_value = match setting {
        ColourType::HEX => format!("#{}", h),
        ColourType::RGB => {
            let [r, g, b] = to_rgb(h);
            format!("rgb({}, {}, {})", r, g, b)
        }
        ColourType::CMYK => {
            let [c, m, y, k] = to_cmyk(h);
            format!("cmyk({}, {}, {}, {})", c, m, y, k)
        }
        ColourType::HSV => {
            let [h, s, v] = to_hsv(h);
            format!("hsv({}, {}, {})", h, s, v)
        }
        ColourType::HSL => {
            let [h, s, l] = to_hsl(h);
            format!("hsl({}, {}, {})", h, s, l)
        }
    };

    return_value
}

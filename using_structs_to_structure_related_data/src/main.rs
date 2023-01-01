use std::{
    cmp,
    f64::consts::{FRAC_PI_6, PI},
    time::Instant,
};

// FIXME: this program cause segmentation fault on debug build.

#[derive(Debug, Clone, Copy)]
struct ColorHSV(f64, f64, f64);
impl ColorHSV {
    fn to_rgb(&self) -> ColorRGB {
        let h = self.0;
        let h_degree = (h / PI * 360.0) as isize;
        let s = self.1;
        let v = self.2;

        // conversion
        let c = s * v;
        let x = c * (1 - ((h_degree / 60) % 2 - 1)).abs() as f64;
        let m = v - c;

        let (r, g, b) = match h_degree {
            0..=59 => (c, x, 0.0),
            60..=119 => (x, c, 0.0),
            120..=179 => (0.0, c, x),
            180..=239 => (0.0, x, c),
            240..=299 => (x, 0.0, c),
            300..=359 => (c, 0.0, x),
            _ => panic!("calculation error"),
        };

        ColorRGB(
            ((r + m) * 255.0) as u8,
            ((g + m) * 255.0) as u8,
            ((b + m) * 255.0) as u8,
        )
    }
}

#[derive(Debug, Clone, Copy)]
struct ColorRGB(u8, u8, u8);
impl ColorRGB {
    fn to_hsv(&self) -> ColorHSV {
        // conversion 0 - 255 to 0.0 - 1.0
        let r = self.0 as f64 / 255.0;
        let g = self.1 as f64 / 255.0;
        let b = self.2 as f64 / 255.0;
        let max = cmp::max(cmp::max(self.0, self.1), self.2) as f64 / 255.0;
        let min = cmp::min(cmp::min(self.0, self.1), self.2) as f64 / 255.0;
        let delta = max - min;

        // calculate hsv
        let mut h = if delta == 0.0 {
            0.0
        } else if max == r {
            FRAC_PI_6 * ((g - b) / delta + 0.0)
        } else if max == g {
            FRAC_PI_6 * ((b - r) / delta + 2.0)
        } else {
            FRAC_PI_6 * ((r - g) / delta + 4.0)
        };

        // h is possibly negative
        if h < 0.0 {
            h = PI + h;
        }
        let s = if max == 0.0 { 0.0 } else { delta / max };
        let v = max;

        ColorHSV(h, s, v)
    }
}

// a program for conversion between rgb and hsv
fn main() {
    // let now = Instant::now();

    let mut colors_rgb = [Some(ColorRGB(0, 0, 0)); 256 * 256 * 256];

    for r in 0..255 {
        for g in 0..255 {
            for b in 0..255 {
                let c = ColorRGB(r, g, b);
                let i = r as usize * (256 * 256) + g as usize * (256) + b as usize;
                colors_rgb[i] = Some(c.to_hsv().to_rgb());
            }
        }
    }

    // let elapsed = now.elapsed();
    // println!("{} ms", elapsed.as_millis());
    println!("{} count", colors_rgb.len());
}

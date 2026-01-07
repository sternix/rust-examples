// https://twitter.com/SegHaxx/status/1494188117309288452

fn mandel(w: u16, h: u16, cx0: f32, cy0: f32, cx1: f32, cy1: f32) {
    let sx: f32 = (cx1 - cx0) / w as f32;
    let sy: f32 = (cy1 - cy0) / h as f32;
    let mut y0: f32 = cy0 - (sy * 0.5);

    for _ in 0..h {
        let mut x0 = cx0;
        y0 += sy;
        for _ in 0..w {
            x0 += sx;
            let mut c = 0;
            let (mut x, mut y, mut x2, mut y2) = (0.0, 0.0, 0.0, 0.0);
            for i in 0..256 {
                y = (x + x) * y + y0;
                x = x2 - y2 + x0;
                x2 = x * x;
                y2 = y * y;
                if x2 + y2 > 4.0 {
                    c = (i & 0xf) + 1;
                    break;
                }
            }
            print!("{}", r##" .,-="^!:;x+X$#&@"##.chars().nth(c).unwrap());
        }
        println!();
    }
}

fn main() {
    mandel(80, 25, -2.05, -1.0, 0.6, 1.0);
}

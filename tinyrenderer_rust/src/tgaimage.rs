// use std::f64;

pub struct TGAHeader {
    id_length: u8,
    color_map_type: u8,
    data_type_code: u8,
    color_map_origin: u16,
    color_map_length: u16,
    color_map_depth: u8,
    x_origin: u16,
    y_origin: u16,
    width: u16,
    height: u16,
    bits_per_pixel: u8,
    image_descriptor: u8,
}

pub struct TGAColor {
    bgra: [u8; 4],
    bytespp: u8,
}

impl std::ops::Index<usize> for TGAColor {
    type Output = u8;

    fn index(&self, rhs: usize) -> &u8 {
        &self.bgra[rhs]
    }
}

impl std::ops::Mul<f64> for TGAColor {
    type Output = TGAColor;

    fn mul(self, intensity: f64) -> TGAColor {
        let clamped = {
            let intensity = f64::min(intensity, 1.0);
            f64::max(0.0, intensity)
        };
        let mut tgac = TGAColor {
            bgra: [0, 0, 0, 0],
            bytespp: 0,
        };
        let mut i = 0;
        while i < self.bgra.len() {
            tgac.bgra[i] = ((self.bgra[i] as f64) * clamped) as u8;
            i = i + 1;
        }
        tgac
    }
}

pub struct TGAImage {
    width: i32,
    height: i32,
    bytespp: i32,
    data: Vec<u8>,
}

impl TGAImage {
    fn read_tga_file(self, filename: String) -> std::io::Result<()> {
        let filedata = std::fs::read(filename)?;
        let mut curr = 0;
        let tgaheader = self.parse_tga_header(filedata, curr);

        self.width = tgaheader.width as i32;
        self.height = tgaheader.height as i32;
        self.bytespp = (tgaheader.bits_per_pixel >> 3) as i32;

        if self.width <= 0
            || self.height <= 0
            || (self.bytespp != 1 && self.bytespp != 3 && self.bytespp != 4)
        {
            //error
        }

        let nbytes = self.width * self.height * self.bytespp;
        self.data.resize(nbytes as usize, 0);
        if tgaheader.data_type_code == 2 || tgaheader.data_type_code == 3 {
        } else if tgaheader.data_type_code == 11 || tgaheader.data_type_code == 10 {
        } else {
        }

        if tgaheader.image_descriptor & 0x20 == 0 {
            self.flip_vertically();
        }
        if tgaheader.image_descriptor & 0x10 != 0 {
            self.flip_horizontally();
        }

        Ok(())
    }

    fn write_tga_file(self, filename: String) -> std::io::Result<()> {
        Ok(())
    }

    fn flip_vertically(self) {}
    fn flip_horizontally(self) {}
    fn scale(self, w: i32, h: i32) {}
    fn get(self, x: i32, y: i32) -> TGAColor {}
    fn set(self, x: i32, y: i32, cl: TGAColor) {}
    fn get_width(self) -> i32 {}
    fn get_height(self) -> i32 {}
    fn get_bytespp(self) -> i32 {}
    fn buffer(self) {}
    fn clear(self) {}

    fn parse_tga_header(self, data: Vec<u8>, point: usize) -> TGAHeader {
        let mut TGAHeader;
    }
}

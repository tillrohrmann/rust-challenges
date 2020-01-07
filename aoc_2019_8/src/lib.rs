use std::{fs, io};

pub struct Image {
    width: u32,
    height: u32,
    layers: Vec<Vec<u32>>,
}

impl Image {
    pub fn load_from_file(width: u32, height: u32, filename: &str) -> io::Result<Image> {
        let content = fs::read_to_string(filename)?;

        let content: String = content.chars().filter(|chr| '\n' != *chr).collect();
        let x = content.trim().chars().map(|chr| chr.to_digit(10).ok_or(io::Error::from(io::ErrorKind::Other)));
        let digits = x.collect::<io::Result<Vec<u32>>>()?;

        let layers: Vec<Vec<u32>> = digits.chunks(width as usize).map(|chunk| chunk.to_vec()).collect();

        Ok(Image {
            width,
            height,
            layers,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_loading() {
        let image = Image::load_from_file(25, 6, "input.txt").unwrap();
    }
}
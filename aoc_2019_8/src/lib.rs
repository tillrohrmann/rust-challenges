use std::{fs, io};

pub struct Image {
    width: u32,
    height: u32,
    layers: Vec<Vec<u32>>,
}

impl Image {
    pub fn load_from_file(width: u32, height: u32, filename: &str) -> io::Result<Image> {
        let content = fs::read_to_string(filename)?;

        Image::load_from_str(width, height, &content)
    }

    fn load_from_str(width: u32, height: u32, content: &str) -> io::Result<Image> {
        let content: String = content.chars().filter(|chr| '\n' != *chr).collect();
        let x = content.trim().chars().map(|chr| chr.to_digit(10).ok_or(io::Error::from(io::ErrorKind::Other)));
        let digits = x.collect::<io::Result<Vec<u32>>>()?;
        let layers: Vec<Vec<u32>> = digits.chunks((width * height) as usize).map(|chunk| chunk.to_vec()).collect();

        Ok(Image {
            width,
            height,
            layers,
        })
    }

    pub fn fold_layer<B, F>(&self, idx: usize, init: B, function: F) -> Option<B> where F: FnMut(B, &u32) -> B {
        self.layers.get(idx).map(|layer| layer.iter().fold(init, function))
    }

    pub fn find_layer(&self, fold_function: fn(&Vec<u32>) -> usize) -> Option<usize> {
        let result = self.layers.iter().enumerate().map(|(idx, layer)| (idx, fold_function(layer))).max_by_key(|(idx, value)| *value);

        result.map(|(idx, _)| idx)
    }

    pub fn check_sum(&self) -> Result<u32, String> {
        let layer_with_fewest_zeros = self.find_layer(Image::count_non_zeros).ok_or(format!("Could not find layer with fewest zeros."))?;

        let number_ones = self.fold_layer(layer_with_fewest_zeros, 0, Image::ones).ok_or(format!("Could not count ones for layer {}.", layer_with_fewest_zeros))?;
        let number_twos = self.fold_layer(layer_with_fewest_zeros, 0, Image::twos).ok_or(format!("Could not count twos for layer {}.", layer_with_fewest_zeros))?;

        Ok(number_ones * number_twos)
    }

    fn count_non_zeros(input: &Vec<u32>) -> usize {
        input.iter().filter(|&v| *v != 0).count()
    }

    fn count_digit(acc: u32, other: &u32, digit: u32) -> u32 {
        if *other == digit {
            acc + 1
        } else {
            acc
        }
    }

    fn ones(acc: u32, other: &u32) -> u32 {
        Image::count_digit(acc, other, 1)
    }

    fn twos(acc: u32, other: &u32) -> u32 {
        Image::count_digit(acc, other, 2)
    }

    fn zeros(acc: u32, other: &u32) -> u32 {
        Image::count_digit(acc, other, 0)
    }

    pub fn size(&self) -> usize {
        (self.width * self.height) as usize
    }

    pub fn render_image(&self) -> Vec<u32> {
        let mut result = vec![0; self.size()];

        for idx in 0..self.size() {
            result[idx] = 2;

            for layer_idx in 0..self.layers.len() {
                if self.layers[layer_idx][idx] != 2 {
                    result[idx] = self.layers[layer_idx][idx];
                    break;
                }
            }
        }

        result
    }

    pub fn print_rendered_image(&self) {
        let rendered_image = self.render_image();

        for line in rendered_image.chunks(self.width as usize) {
            for &x in line {
                if x == 0 {
                    print!(" ");
                } else {
                    print!("#");
                }
            }
            print!("\n");
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_decoding() {
        let image = Image::load_from_str(2, 2, "0222112222120000").unwrap();

        assert_eq!(image.render_image(), vec![0, 1, 1, 0])
    }
}
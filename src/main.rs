use rayoneweek::image;
use ndarray::{Array3, Array};
fn main() {
    const WIDTH:usize = 256;
    const HEIGHT:usize = 256;

    let mut array:Array3<u8> = Array::zeros((HEIGHT,WIDTH,3));

    for ((y,x,z),v) in array.indexed_iter_mut() {
        *v = match z{
            0 => x as u8,
            1 => (HEIGHT - y) as u8,
            2 => 64,
            _=>unreachable!(),
        }
    }

    let img = image::array_to_image(array);
    img.save("output.png").expect("fail");
    println!("End!");
}

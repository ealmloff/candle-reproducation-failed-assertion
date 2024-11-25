use candle_core::{Device, Tensor};

fn main() {
    let tensor = Tensor::new(vec![1.0f32, 2.0, 3.0], &Device::new_metal(0).unwrap()).unwrap();

    loop {
        let tensor = tensor.clone();
        std::thread::spawn(move || {
            let new = tensor.add(&tensor).unwrap();
            let vec: Vec<f32> = new.to_vec1().unwrap();
            assert!(vec[0] > 1.9 && vec[0] < 2.1);
            assert!(vec[1] > 3.9 && vec[1] < 4.1);
            assert!(vec[2] > 5.9 && vec[2] < 6.1);
        });
    }
}

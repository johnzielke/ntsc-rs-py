use image::RgbImage;
use ntscrs::settings;
use ntscrs::settings::NtscEffect;
use numpy::ndarray::{Array4, ArrayD, ArrayView4, ArrayViewD, ArrayViewMutD};
use numpy::{IntoPyArray, PyArray4, PyArrayDyn, PyReadonlyArray4, PyReadonlyArrayDyn};
use pyo3::prelude::*;
use pyo3::{pymodule, types::PyModule, PyResult, Python};
fn ntsc_rs(x: ArrayView4<'_, u8>) -> Array4<u8> {
    // Check if the input is a 4D array (frame, width, height, channel)
    if x.ndim() != 4 {
        panic!("Input must be a 4D array");
    }
    let mut output = Array4::<u8>::zeros((x.shape()[0], x.shape()[1], x.shape()[2], x.shape()[3]));
    let mut ntsc_effect = NtscEffect::default();

    // FIXME: Without setting the intensity to 0.0, the apply_effect function will panic
    // Set ntsc_effect.vhs_settings.edge_wave.intensity to 0.0 to disable the effect
    // ntsc_effect.vhs_settings = Some(settings::VHSSettings {
    //     edge_wave: Some(settings::VHSEdgeWaveSettings {
    //         intensity: 0.0,
    //         ..ntsc_effect.vhs_settings.clone().expect("").edge_wave.unwrap()
    //     }),
    //     ..ntsc_effect.vhs_settings.expect("")
    // });
    for i in 0..x.shape()[0] {
        let mut image = RgbImage::from_raw(
            x.shape()[1] as u32,
            x.shape()[2] as u32,
            x.to_owned().into_raw_vec(),
        )
        .unwrap();
        ntsc_effect.apply_effect(&mut image, i as usize);

        for j in 0..x.shape()[1] {
            for k in 0..x.shape()[2] {
                for l in 0..x.shape()[3] {
                    output[[i, j, k, l]] = image.get_pixel(j as u32, k as u32)[l];
                }
            }
        }
    }
    output
}
#[pymodule]
fn ntsc_rs_py<'py>(_py: Python<'py>, m: &'py PyModule) -> PyResult<()> {
    // wrapper of `ntsc_rs`
    #[pyfn(m)]
    #[pyo3(name = "ntsc_rs")]
    fn ntsc_rs_py<'py>(py: Python<'py>, x: PyReadonlyArray4<'py, u8>) -> &'py PyArray4<u8> {
        let x = x.as_array();

        let z = ntsc_rs(x);
        z.into_pyarray(py)
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use numpy::ndarray::Array4;

    #[test]
    fn it_works() {
        let image = Array4::<u8>::zeros((3, 400, 400, 3));
        let output = super::ntsc_rs(image.view());
    }
}

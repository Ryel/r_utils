use ndarray::{Array1, Array2, Axis};

fn adjust_time_resolution(quantized: Array1<f32>, mel: Array2<f32>) -> Result<(Array1<f32>, Array2<f32>), String> {
    if mel.nrows() == 0 {
        return Err("mel matrix has no rows".to_string());
    }

    let upsample_factor = quantized.len() / mel.nrows();
    let mut upsampled_mel = Array2::zeros((quantized.len(), mel.ncols()));
    // TODO: reverse sampling
    for (i, row) in mel.axis_iter(Axis(0)).enumerate() {
        for j in 0..upsample_factor {
            let index = i * upsample_factor + j;
            if index < upsampled_mel.nrows() {
                upsampled_mel.row_mut(index).assign(&row);
            }
        }
    }

    // TODO: implement
    let (start, end) = start_and_end_indices(&quantized, hparams.silence_threshold);
    Ok((quantized.slice(s![start..end]).to_owned(), upsampled_mel.slice(s![start..end, ..]).to_owned()))
}

extern crate tulip_rust_wrapper;

use tulip_rust_wrapper::{ti_macd, ti_macd_start, TC_REAL};

fn macd(short_period :u32, long_period :u32, signal_period :u32, close_prices: &Vec<TC_REAL>) -> (Vec<TC_REAL>,Vec<TC_REAL>,Vec<TC_REAL>,i32) {
    let mut macdVec :Vec<TC_REAL>;
    let mut signalVec :Vec<TC_REAL>;
    let mut histVec :Vec<TC_REAL>;

    let mut out: [*mut f64;3];

    let options :Vec<f64> = vec![short_period as f64,long_period as f64,signal_period as f64];
    let const_out_ptr;
    let const_input_ptr : *const f64 = close_prices.as_ptr();
    let start;
    let ret_code ;
    let output_length;

    let result_macd;
    let result_macd_signal;
    let result_macd_histogram;

    unsafe{
        start = ti_macd_start(options.as_ptr());
        output_length = close_prices.len()- start as usize;
        macdVec = Vec::with_capacity(close_prices.len());
        signalVec = Vec::with_capacity(close_prices.len());
        histVec = Vec::with_capacity(close_prices.len());


        out = [macdVec.as_mut_ptr(),signalVec.as_mut_ptr(),histVec.as_mut_ptr()];
        const_out_ptr = out;

        ret_code = ti_macd(
            close_prices.len() as i32,
            &const_input_ptr,
            options.as_ptr() ,
            const_out_ptr.as_ptr()
        );
        result_macd = std::slice::from_raw_parts_mut(const_out_ptr[0], output_length).to_vec();
        result_macd_signal = std::slice::from_raw_parts_mut(const_out_ptr[1], output_length).to_vec();
        result_macd_histogram = std::slice::from_raw_parts_mut(const_out_ptr[2], output_length).to_vec();
    }
    (result_macd,result_macd_signal,result_macd_histogram,ret_code)

}
fn main() {
    let close_prices: Vec<TC_REAL> = vec![
        81.59,81.06,82.87,83.00,83.61,83.15,82.84,83.99,84.55,
        84.36,85.53,86.54,86.89,87.77,87.29];

    let (macd_vec,macdsingal_vec,macdhistogram_vec,ret_code) = macd(2,5,9, &close_prices);

    for (index, value) in macd_vec.iter().enumerate() {
        println!("index = {} ,macd = {}",index, value);
    }
    for (index, value) in macdsingal_vec.iter().enumerate() {
        println!("index = {} ,macdsignal = {}",index, value);
    }
    for (index, value) in macdhistogram_vec.iter().enumerate() {
        println!("index = {} ,macdhistogram = {}",index, value);
    }
}

extern crate tulip_rust_wrapper;

use tulip_rust_wrapper::{ti_sma,ti_sma_start,TC_REAL};

fn sma(period: u32, close_prices: &Vec<TC_REAL>) -> (Vec<TC_REAL>,i32) {
    let mut out: Vec<TC_REAL>;
    let options :Vec<f64> = vec![period as f64];
    let const_out_ptr;
    let const_input_ptr : *const f64 = &close_prices[0];
    let start;
    let ret_code ;
    let output_length;

    unsafe{
        start = ti_sma_start(options.as_ptr()) as usize;
        output_length = close_prices.len() - start;
        out = Vec::with_capacity(output_length);
        const_out_ptr = out.as_mut_ptr();
        ret_code = ti_sma(
            close_prices.len() as i32,
            &const_input_ptr,
            &options[0] ,
            &const_out_ptr
        );
        out = std::slice::from_raw_parts_mut(const_out_ptr, output_length).to_vec();
    }
    (out,ret_code)

}
fn main() {
    let close_prices: Vec<TC_REAL> = vec![
        1.087010, 1.087120, 1.087080, 1.087170, 1.087110, 1.087010, 1.087100, 1.087120, 1.087110,
        1.087080, 1.087000, 1.086630, 1.086630, 1.086610, 1.086630, 1.086640, 1.086650, 1.086650,
        1.086670, 1.086630
    ];

    let (sma_values,ret_code) = sma(3, &close_prices);

    for (index, value) in sma_values.iter().enumerate() {
        println!("index = {} ,Close = {}",index, value);
    }
}

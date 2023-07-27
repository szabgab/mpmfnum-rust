// mpmfnum: a numbers library in Rust
// Brett Saiki <bksaiki(at)gmail.com>
// 2023

// sandbox.rs
//
// Sandboxing

#[test]
fn sandbox() {
    let qnan = f32::from_bits(0x7FC00000);
    let snan = f32::from_bits(0x7f800001);
    let zero = 0.0_f32;

    let sum1 = qnan + zero;
    let sum2 = snan + zero;
    let sum3 = qnan + snan;
    let sum4 = snan + qnan;

    println!(
        "{:X} {:X} {:X} {:X}",
        sum1.to_bits(),
        sum2.to_bits(),
        sum3.to_bits(),
        sum4.to_bits()
    );
}

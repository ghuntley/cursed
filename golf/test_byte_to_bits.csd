// Test byte_to_bits function
fn byte_to_bits(byte_val) {
    let b0 = (byte_val / 128) % 2;
    let b1 = (byte_val / 64) % 2;
    let b2 = (byte_val / 32) % 2;
    let b3 = (byte_val / 16) % 2;
    let b4 = (byte_val / 8) % 2;
    let b5 = (byte_val / 4) % 2;
    let b6 = (byte_val / 2) % 2;
    let b7 = byte_val % 2;
    return [b0, b1, b2, b3, b4, b5, b6, b7];
}

// Test cases
let bits170 = byte_to_bits(170); // 10101010
vibez.spill("170:", bits170[0], bits170[1], bits170[2], bits170[3], bits170[4], bits170[5], bits170[6], bits170[7]);

let bits85 = byte_to_bits(85); // 01010101  
vibez.spill("85: ", bits85[0], bits85[1], bits85[2], bits85[3], bits85[4], bits85[5], bits85[6], bits85[7]);

let bits255 = byte_to_bits(255); // 11111111
vibez.spill("255:", bits255[0], bits255[1], bits255[2], bits255[3], bits255[4], bits255[5], bits255[6], bits255[7]);

let bits0 = byte_to_bits(0); // 00000000
vibez.spill("0:  ", bits0[0], bits0[1], bits0[2], bits0[3], bits0[4], bits0[5], bits0[6], bits0[7]);

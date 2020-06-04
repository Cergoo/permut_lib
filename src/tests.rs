#[cfg(test)]
use crate::permut::*;

#[test]
fn create_permut_t1() {
    let t1: Vec<&[u8]> = vec![
        "A".as_bytes(),
        "B".as_bytes(),
        "C".as_bytes(),
        "D".as_bytes(),
        "E".as_bytes(),
        "F".as_bytes(),
        "G".as_bytes(),
        "H".as_bytes(),
        "I".as_bytes(),
        "J".as_bytes(),
        "K".as_bytes(),
        "L".as_bytes(),
        "M".as_bytes(),
        "N".as_bytes(),
        "O".as_bytes(),
        "P".as_bytes(),
        "Q".as_bytes(),
        "R".as_bytes(),
        "S".as_bytes(),
        "T".as_bytes(),
        "U".as_bytes(),
        "V".as_bytes(),
        "W".as_bytes(),
        "X".as_bytes(),
        "Y".as_bytes(),
        "Z".as_bytes(),
        "AA".as_bytes(),
        "AB".as_bytes(),
        "AC".as_bytes(),
        "AD".as_bytes(),
        "AE".as_bytes(),
        "AF".as_bytes(),
        "AG".as_bytes(),
        "AH".as_bytes(),
        "AI".as_bytes(),
        "AJ".as_bytes(),
        "AK".as_bytes(),
        "AL".as_bytes(),
        "AM".as_bytes(),
        "AN".as_bytes(),
        "AO".as_bytes(),
        "AP".as_bytes(),
        "AQ".as_bytes(),
        "AR".as_bytes(),
        "AS".as_bytes(),
        "AT".as_bytes(),
        "AU".as_bytes(),
        "AV".as_bytes(),
        "AW".as_bytes(),
        "AX".as_bytes(),
        "AY".as_bytes(),
        "AZ".as_bytes(),
        "BA".as_bytes(),
        "BB".as_bytes(),
    ];

    let r = create_permut(CHARS_CAPS_LATIN, 54);
    let r1: Vec<&[u8]> = r.iter().map(|a| a.as_slice()).collect();

    assert_eq!(t1, r1);
}

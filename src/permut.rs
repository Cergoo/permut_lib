pub const CHARS_CUPS_LATIN: &[u8] = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes();
pub const CHARS_LOW_LATIN: &[u8] = "abcdefghijklmnopqrstuvwxyz".as_bytes();

/// Universal function for generate unic sequences of a u8 chars
/// # Parameters
/// - 'chars' some u8 dictonary set
/// - 'count' count elements to must be generated
/// # Examples
/// ```
/// use permut_lib::permut::*;
/// create_permut(CHARS_CUPS_LATIN, 195); // result: "A", "B", "C", ... , "AA", "AB" ...  
/// ```
pub fn create_permut(chars: &[u8], count: usize) -> Vec<Vec<u8>> {
    let base: (usize, usize) = (count / chars.len(), count % chars.len());
    let cap = count + if base.1 > 0 { chars.len() - base.1 } else { 0 };
    let mut rezult: Vec<Vec<u8>> = Vec::with_capacity(cap);

    let chars_iter = chars.iter().cloned().map(|a| {
        let mut n = Vec::new();
        n.push(a);
        n
    });

    rezult.extend(chars_iter);

    let ch_iter = chars.iter().cloned();
    for n in 0..base.0 {
        let base_result = &rezult[n].clone();
        rezult.extend(ch_iter.clone().map(|a| {
            let mut tmp = base_result.clone();
            tmp.push(a);
            tmp
        }));
    }

    rezult.truncate(count);
    rezult
}

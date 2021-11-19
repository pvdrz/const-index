use const_index::{crange, ConstGet};

#[test]
fn it_works() {
    let bytes: Vec<u8> = (0..100).collect();
    let b: &[u8; 10] = bytes.cget(crange!(0..10)).unwrap();
    let c: &[u8; 10] = bytes.cget(crange!(0..=9)).unwrap();
    let d: &[u8; 10] = bytes.cget(crange!(..10)).unwrap();
    let e: &[u8; 10] = bytes.cget(crange!(..=9)).unwrap();

    assert_eq!(b, c);
    assert_eq!(b, d);
    assert_eq!(b, e);
}

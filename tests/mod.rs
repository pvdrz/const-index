use const_index::{cindex, ConstGet};

#[test]
fn it_works() {
    let bytes: Vec<u8> = (0..100).collect();
    let b: &[u8; 10] = bytes.cget(cindex!(0..10)).unwrap();
    let c: &[u8; 10] = bytes.cget(cindex!(0..=9)).unwrap();
    let d: &[u8; 10] = bytes.cget(cindex!(..10)).unwrap();
    let e: &[u8; 10] = bytes.cget(cindex!(..=9)).unwrap();
    let f: &u8 = bytes.cget(cindex!(0)).unwrap();
    let (g, h): (&[u8; 10], &[u8]) = bytes.csplit_at(cindex!(10));

    assert_eq!(b, c);
    assert_eq!(b, d);
    assert_eq!(b, e);
    assert_eq!(&b[0], f);
    assert_eq!(b, g);
    assert_eq!(&bytes[10..], h);

    assert_eq!(b, bytes.cindex(cindex!(0..10)));
    assert_eq!(c, bytes.cindex(cindex!(0..=9)));
    assert_eq!(d, bytes.cindex(cindex!(..10)));
    assert_eq!(e, bytes.cindex(cindex!(..=9)));
}

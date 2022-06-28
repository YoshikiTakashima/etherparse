//proptest! {
//    #[test]
//    fn u32_u16_comparison(
//        data in proptest::collection::vec(any::<u8>(), 0..0xfffusize)
//    ) {
//        use super::etherparse::checksum::*;
//
//        let u32_oc = u32_16bit_word::ones_complement(
//            u32_16bit_word::add_slice(0, &data)
//        );
//        let u64_oc = u64_16bit_word::ones_complement(
//            u64_16bit_word::add_slice(0, &data)
//        );
//        assert_eq!(u32_oc, u64_oc);
//
//        let struct_oc = Sum16BitWords::new()
//            .add_slice(&data)
//            .ones_complement();
//        assert_eq!(u32_oc, struct_oc);
//    }
//}

#[kani::unwind( 50)]
#[kani::proof]
fn u32_u16_comparison(
) {
    use etherparse::checksum::*;
    let kani_any_slice: [u8;0xfusize] = kani::any();
    let data: Vec<u8> = kani_any_slice.to_vec();

    let u32_oc = u32_16bit_word::ones_complement(
        u32_16bit_word::add_slice(0, &data)
    );
    let u64_oc = u64_16bit_word::ones_complement(
        u64_16bit_word::add_slice(0, &data)
    );
    assert_eq!(u32_oc, u64_oc);

    let struct_oc = Sum16BitWords::new()
        .add_slice(&data)
        .ones_complement();
    assert_eq!(u32_oc, struct_oc);
}

//const CBMC_FOUND: [u8; 8] = [0, 0b01001110, 0, 0, 0, 0, 0, 0];
//#[test]
//fn replay_u32_u16_comparison(
//) {
//    use etherparse::checksum::*;
//    //let kani_any_slice: [u8;0xfffusize] = CBMC_FOUND;
//    let data: Vec<u8> = CBMC_FOUND.clone().to_vec();
////kani_any_slice.to_vec();
//
//    let u32_oc = u32_16bit_word::ones_complement(
//        u32_16bit_word::add_slice(0, &data)
//    );
//    let u64_oc = u64_16bit_word::ones_complement(
//        u64_16bit_word::add_slice(0, &data)
//    );
//    assert_eq!(u32_oc, u64_oc);
//
//    let struct_oc = Sum16BitWords::new()
//        .add_slice(&data)
//        .ones_complement();
//    assert_eq!(u32_oc, struct_oc);
//}

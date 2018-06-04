use core::*;
use std::mem;

#[test]
fn generic_test() {
	let db = DB::init("test.lmdb", 1024 * 100).unwrap();

	{
		let res: Vec<(_, Vec<u8>)> = db.read_range(&[0u8; 1], &unsafe { mem::transmute::<u32, [u8; 4]>(0xFFFFFFFF) }).unwrap();
		assert!(res.is_empty());
	}

	{
		let test_value = 69;
		let test_key = &[test_value];

		let _ = db.write(test_key, test_value).unwrap();

		let res: u8 = db.read(test_key).unwrap();
		assert_eq!(res, test_value);
	}

	{
		let set: Vec<(&[u8], u16)> = vec![
			(&[0xAAu8, 0xAAu8][..], 0xAAAAu16),
			(&[0xAAu8, 0xABu8][..], 0xAAABu16),
			(&[0xAAu8, 0xACu8][..], 0xAAACu16),
			(&[0xAAu8, 0xADu8][..], 0xAAADu16),
		];

		db.write_bulk(set.clone()).unwrap();

		let mut res: Vec<(_, u16)> = db.read_range_u64(0xAAAA, 0xAAAE).unwrap();

		res.reverse();

		for (k, v) in set {
			if let Some((r_k, r_v)) = res.pop() {
				assert_eq!(r_k, k);
				assert_eq!(r_v, v);
			} else {
				panic!("failed to unwrap pop");
			}
		}
	}
}

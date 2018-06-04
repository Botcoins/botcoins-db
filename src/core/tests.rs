use core::*;
use std::fs;

#[test]
fn generic_test() {
	let dir = "test.lmdb";

	let _ = fs::remove_dir_all(dir);
	{
		let db = DB::init(dir, 1024 * 100).unwrap();

		{
			let res: Vec<(_, u8)> = db.read_range(&[0u8; 1], &[0xFFu8; 4]).unwrap();
			assert!(res.is_empty());
		}

		{
			let res: Vec<(_, Vec<u8>)> = db.read_range(&[0u8; 1], &[0xFFu8; 4]).unwrap();
			assert!(res.is_empty());
		}

		{
			assert!(db.read::<Vec<u8>>(&[0u8; 1]).is_err());
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
				(&[0x01u8, 0xAAu8][..], 0xAAAAu16),
				(&[0x01u8, 0xABu8][..], 0xAAABu16),
				(&[0x01u8, 0xACu8][..], 0xAAACu16),
				(&[0x01u8, 0xADu8][..], 0xAAADu16),
			];

			db.write_bulk(set.clone()).unwrap();

			let mut res: Vec<(_, u16)> = db.read_range(set[0].0, set[3].0).unwrap();

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

		{
			let res: Vec<(_, u16)> = db.read_range(&[0x01u8, 0xAAu8], &[0x01u8, 0xAFu8]).unwrap();
			assert_eq!(res.len(), 4);
		}

		{
			let set: Vec<(&[u8], Vec<u8>)> = vec![
				(&[0x02u8, 0xAAu8][..], vec![0xFFu8; 4]),
				(&[0x02u8, 0xABu8][..], vec![0xFFu8; 4]),
				(&[0x02u8, 0xACu8][..], vec![0xFFu8; 4]),
				(&[0x02u8, 0xADu8][..], vec![0xFFu8; 4]),
			];

			db.write_bulk(set).unwrap();

			let res: Vec<(_, Vec<u8>)> = db.read_range(&[0x02u8, 0xAAu8], &[0x02u8, 0xAFu8]).unwrap();
			assert_eq!(res.len(), 4);
		}
	}
	let _ = fs::remove_dir_all(dir);
}

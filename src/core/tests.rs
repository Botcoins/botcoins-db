use core::*;

#[test]
fn test_init_db() {
	let db = DB::init("test.lmdb", 1024 * 100);
}
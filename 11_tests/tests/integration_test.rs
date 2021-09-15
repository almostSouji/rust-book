use tests;
// running specific integration test file:
// cargo test --test integration_test

#[test]
fn it_adds_two() {
	common::setup(); // common functions, not a test file, hence placed in /tests/common/mod.rs
	assert_eq!(4, tets::add_two(2));
}

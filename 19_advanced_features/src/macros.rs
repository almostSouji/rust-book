#[macro_export] // macro can be brought into scope
macro_rules! vecc {  // name without !
	// similar to match expression, one arm with pattern ( $($x:expr), * )
	( $($x:expr), * ) => {
		{
			let mut temp_vec = Vec::new();
			$(
				temp_vec.push($x)
			)*
			temp_vec
		}
	};
}

#[cfg(test)]
pub mod tests {
    use kzg_bench::tests::poly::*;
    use zkcrypto::poly::{ZPoly};
	use zkcrypto::zkfr::blsScalar;
	
    #[test]
	fn create_poly_of_length_ten_() {
		create_poly_of_length_ten::<blsScalar, ZPoly>();
	}
	
	#[test]
	fn poly_eval_check_() {
		poly_eval_check::<blsScalar, ZPoly>();
	}

	#[test]
	fn poly_eval_0_check_() {
		poly_eval_0_check::<blsScalar, ZPoly>();
	}

	#[test]
	fn poly_eval_nil_check_() {
		poly_eval_nil_check::<blsScalar, ZPoly>();
	}

	#[test]
	fn poly_inverse_simple_0_() {
		poly_inverse_simple_0::<blsScalar, ZPoly>();
	}

	#[test]
	fn poly_inverse_simple_1_() {
		poly_inverse_simple_1::<blsScalar, ZPoly>();
	}
	
}
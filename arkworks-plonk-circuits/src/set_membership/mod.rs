use ark_ec::models::TEModelParameters;
use ark_ff::PrimeField;
use ark_std::{marker::PhantomData, vec::Vec};
use plonk_core::{
	circuit::Circuit, constraint_system::StandardComposer, error::Error, prelude::Variable,
};

/// A function whose output is 1 if `member` belongs to `set`
/// and 0 otherwise.  Contraints are added to a StandardComposer
/// and the output is added as a variable to the StandardComposer.
/// The set is assumed to consist of private inputs.
fn check_private_set_membership<F, P>(
	composer: &mut StandardComposer<F, P>,
	set: &Vec<F>,
	member: Variable,
) -> Variable
where
	F: PrimeField,
	P: TEModelParameters<BaseField = F>,
{
	let set: Vec<Variable> = set.iter().map(|x| composer.add_input(*x)).collect();

	// Compute all differences between `member` and set elements
	let mut diffs = Vec::new();
	for x in set.iter() {
		let diff = composer
			.arithmetic_gate(|gate| gate.witness(member, *x, None).add(F::one(), -F::one()));
		diffs.push(diff);
	}

	// Accumulate the product of all differences
	let mut accumulator = composer.add_witness_to_circuit_description(F::one());
	for diff in diffs {
		accumulator =
			composer.arithmetic_gate(|gate| gate.witness(accumulator, diff, None).mul(F::one()));
	}

	composer.is_zero_with_output(accumulator)
}

/// A function whose output is 1 if `member` belongs to `set`
/// and 0 otherwise.  Contraints are added to a StandardComposer
/// and the output is added as a variable to the StandardComposer.
/// The set is assumed to consist of public inputs, which reduces
/// the number of variables in the circuit.
fn check_public_set_membership<F, P>(
	composer: &mut StandardComposer<F, P>,
	set: &Vec<F>,
	member: Variable,
) -> Variable
where
	F: PrimeField,
	P: TEModelParameters<BaseField = F>,
{
	// Compute all differences between `member` and set elements
	let mut diffs = Vec::new();
	for x in set.iter() {
		let diff = composer.arithmetic_gate(|gate| {
			gate.witness(member, member, None)
				.add(F::zero(), F::one())
				.pi(-*x)
		});
		diffs.push(diff);
	}

	// Accumulate the product of all differences
	let mut accumulator = composer.add_witness_to_circuit_description(F::one());
	for diff in diffs {
		accumulator =
			composer.arithmetic_gate(|gate| gate.witness(accumulator, diff, None).mul(F::one()));
	}

	composer.is_zero_with_output(accumulator)
}

/// A function whose output is 1 if `member` belongs to `set`
/// and 0 otherwise.  Contraints are added to a StandardComposer
/// and the output is added as a variable to the StandardComposer.
/// The set is assumed to consist of public constants, which
/// may not be appropriate.  This cuts the number of gates in half, however.
fn check_constant_set_membership<F, P>(
	composer: &mut StandardComposer<F, P>,
	set: &Vec<F>,
	member: Variable,
) -> Variable
where
	F: PrimeField,
	P: TEModelParameters<BaseField = F>,
{
	// iterate through set, multiplying an accumulated value by the next
	// difference (x - s)
	let mut accumulated = composer.add_witness_to_circuit_description(F::one());
	for s in set.iter() {
		accumulated = composer.arithmetic_gate(|gate| {
			gate.witness(member, accumulated, None)
				.add(F::zero(), -*s)
				.mul(F::one())
		});
	}
	composer.is_zero_with_output(accumulated)
}

#[cfg(test)]
pub(crate) mod tests {
	//copied from ark-plonk
	use super::*;
	use ark_bn254::Bn254;
	use ark_ec::{models::TEModelParameters, PairingEngine};
	use ark_ed_on_bn254::{EdwardsParameters as JubjubParameters, Fq};
	use ark_poly::polynomial::univariate::DensePolynomial;
	use ark_poly_commit::{kzg10::KZG10, sonic_pc::SonicKZG10, PolynomialCommitment};
	use ark_std::test_rng;
	use plonk_core::proof_system::{Prover, Verifier};

	#[test]
	fn test_verify_set_membership_functions() {
		let set = vec![Fq::from(1u32), Fq::from(2u32), Fq::from(3u32)];

		// Check private version
		{
			let mut composer = StandardComposer::<Fq, JubjubParameters>::new();
			let one = composer.add_input(Fq::from(1u32));
			let member = composer.add_input(Fq::from(2u32));

			let result_private = check_private_set_membership(&mut composer, &set, member);
			composer.assert_equal(result_private, one);
			composer.check_circuit_satisfied();
		}

		// Check public version
		{
			let mut composer = StandardComposer::<Fq, JubjubParameters>::new();
			let one = composer.add_input(Fq::from(1u32));
			let member = composer.add_input(Fq::from(2u32));

			let result_private = check_public_set_membership(&mut composer, &set, member);
			composer.assert_equal(result_private, one);
			composer.check_circuit_satisfied();
		}

		// Check constant version
		{
			let mut composer = StandardComposer::<Fq, JubjubParameters>::new();
			let one = composer.add_input(Fq::from(1u32));
			let member = composer.add_input(Fq::from(2u32));

			let result_private = check_constant_set_membership(&mut composer, &set, member);
			composer.assert_equal(result_private, one);
			composer.check_circuit_satisfied();
		}
	}

	#[test]
	fn test_fail_to_verify_set_membership_functions() {
		let set = vec![Fq::from(1u32), Fq::from(2u32), Fq::from(3u32)];

		// Check private version
		{
			let mut composer = StandardComposer::<Fq, JubjubParameters>::new();
			let zero = composer.zero_var();
			let member = composer.add_input(Fq::from(4u32));

			let result_private = check_private_set_membership(&mut composer, &set, member);
			composer.assert_equal(result_private, zero);
			composer.check_circuit_satisfied();
		}

		// Check public version
		{
			let mut composer = StandardComposer::<Fq, JubjubParameters>::new();
			let zero = composer.zero_var();
			let member = composer.add_input(Fq::from(4u32));

			let result_private = check_public_set_membership(&mut composer, &set, member);
			composer.assert_equal(result_private, zero);
			composer.check_circuit_satisfied();
		}

		// Check constant version
		{
			let mut composer = StandardComposer::<Fq, JubjubParameters>::new();
			let zero = composer.zero_var();
			let member = composer.add_input(Fq::from(4u32));

			let result_private = check_constant_set_membership(&mut composer, &set, member);
			composer.assert_equal(result_private, zero);
			composer.check_circuit_satisfied();
		}
	}

	// This helper function is still used elsewhere in crate.  Should move it to
	// mixer
	pub(crate) fn gadget_tester<
		E: PairingEngine,
		P: TEModelParameters<BaseField = E::Fr>,
		C: Circuit<E::Fr, P>,
	>(
		circuit: &mut C,
		n: usize,
	) -> Result<(), Error> {
		let rng = &mut test_rng();
		// Common View
		let universal_params = KZG10::<E, DensePolynomial<E::Fr>>::setup(2 * n, false, rng)?;
		// Provers View
		let (proof, public_inputs) = {
			// Create a prover struct
			let mut prover: Prover<
				E::Fr,
				P,
				SonicKZG10<E, DensePolynomial<<E as PairingEngine>::Fr>>,
			> = Prover::new(b"demo");

			// Additionally key the transcript
			prover.key_transcript(b"key", b"additional seed information");

			// Add gadgets
			circuit.gadget(&mut prover.mut_cs())?;

			// Commit Key
			let (ck, _) = SonicKZG10::<E, DensePolynomial<E::Fr>>::trim(
				&universal_params,
				prover.circuit_size().next_power_of_two() + 6,
				0,
				None,
			)
			.unwrap();
			// Preprocess circuit
			prover.preprocess(&ck)?;

			// Once the prove method is called, the public inputs are cleared
			// So pre-fetch these before calling Prove
			let public_inputs = prover.mut_cs().construct_dense_pi_vec();
			//? let lookup_table = prover.mut_cs().lookup_table.clone();

			// Compute Proof
			(prover.prove(&ck)?, public_inputs)
		};
		// Verifiers view
		//
		// Create a Verifier object
		let mut verifier = Verifier::new(b"demo");

		// Additionally key the transcript
		verifier.key_transcript(b"key", b"additional seed information");

		// Add gadgets
		circuit.gadget(&mut verifier.mut_cs())?;

		// Compute Commit and Verifier Key
		let (sonic_ck, sonic_vk) = SonicKZG10::<E, DensePolynomial<E::Fr>>::trim(
			&universal_params,
			verifier.circuit_size().next_power_of_two(),
			0,
			None,
		)
		.unwrap();

		// Preprocess circuit
		verifier.preprocess(&sonic_ck)?;

		// Verify proof
		Ok(verifier.verify(&proof, &sonic_vk, &public_inputs)?)
	}
}

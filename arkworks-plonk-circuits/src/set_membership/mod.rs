use ark_ec::models::TEModelParameters;
use ark_ff::PrimeField;
use ark_std::{marker::PhantomData, vec::Vec};
use plonk_core::{
	circuit::Circuit, constraint_system::StandardComposer, error::Error, prelude::Variable,
};

#[derive(Debug, Default)]
struct SetMembershipCircuit<F: PrimeField, P: TEModelParameters<BaseField = F>> {
	pub roots: Vec<F>,
	pub target: F,
	pub _te: PhantomData<P>,
}

impl<F: PrimeField, P: TEModelParameters<BaseField = F>> Circuit<F, P>
	for SetMembershipCircuit<F, P>
{
	const CIRCUIT_ID: [u8; 32] = [0xff; 32];

	fn gadget(&mut self, composer: &mut StandardComposer<F, P>) -> Result<(), Error> {
		let roots: Vec<Variable> = self.roots.iter().map(|x| composer.add_input(*x)).collect();
		let target = composer.add_input(self.target);

		let mut diffs = Vec::new();
		for x in roots {
			let diff = composer
				.arithmetic_gate(|gate| gate.witness(target, x, None).add(-F::one(), F::one()));
			diffs.push(diff);
		}

		let mut sum = composer.add_input(F::one());

		for diff in diffs {
			sum = composer.arithmetic_gate(|gate| gate.witness(sum, diff, None).mul(F::one()));
		}

		composer.assert_equal(sum, composer.zero_var());

		Ok(())
	}

	fn padded_circuit_size(&self) -> usize {
		1 << 11
	}
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

	#[test]
	fn test_verify_set_membership() {
		let roots = vec![Fq::from(1u32), Fq::from(2u32), Fq::from(3u32)];
		let target = Fq::from(2u32);
		let mut circuit = SetMembershipCircuit::<Fq, JubjubParameters> {
			roots,
			target,
			_te: PhantomData,
		};

		let res = gadget_tester::<Bn254, JubjubParameters, _>(&mut circuit, 2000);
		assert!(res.is_ok(), "{:?}", res.err().unwrap());
	}

	#[test]
	fn test_fail_to_verify_invalid_set_membership() {
		let roots = vec![Fq::from(1u32), Fq::from(2u32), Fq::from(3u32)];
		// Not in the set
		let target = Fq::from(4u32);
		let mut circuit = SetMembershipCircuit::<Fq, JubjubParameters> {
			roots,
			target,
			_te: PhantomData,
		};

		let res = gadget_tester::<Bn254, JubjubParameters, _>(&mut circuit, 2000);
		assert!(res.is_err());
	}
}

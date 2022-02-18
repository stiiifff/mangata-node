use super::*;
use mock::{new_test_ext, roll_to_while_minting, BlocksPerRound, Issuance, System, Test, Tokens};
use sp_runtime::SaturatedConversion;

#[test]
fn linear_issuance_works() {
	new_test_ext().execute_with(|| {
		let session_number = System::block_number().saturated_into::<u32>() / BlocksPerRound::get();
		let session_issuance = <Issuance as GetIssuance>::get_all_issuance(session_number)
			.expect("session issuance is always populated in advance");
		let block_issuance = (session_issuance.0 + session_issuance.1 + session_issuance.2) /
			(BlocksPerRound::get().saturated_into::<u128>());

		// Mint in block 1
		// We are not minting in block 0, but that's okay
		assert_eq!(450044, (session_issuance.0 + session_issuance.1 + session_issuance.2));
		assert_eq!(90008, block_issuance);

		roll_to_while_minting(22218, Some(90008));

		assert_eq!(3999995536, Tokens::total_issuance(0u32) as Balance);

		// This the point the next session's issuance will be calculated and minted
		// on the basis of total_issuance
		roll_to_while_minting(22219, Some(90008));

		assert_eq!(4000000000, Tokens::total_issuance(0u32) as Balance);
	});
}

#[test]
fn linear_issuance_doesnt_change_upon_burn() {
	new_test_ext().execute_with(|| {
		roll_to_while_minting(15000, Some(90008));

		orml_tokens::MultiTokenCurrencyAdapter::<Test>::burn_and_settle(
			0u32.into(),
			&0u128,
			100_000_000,
		)
		.unwrap();

		assert_eq!(3250582044, Tokens::total_issuance(0u32) as Balance);

		roll_to_while_minting(22218, Some(90008));

		assert_eq!(3899995536, Tokens::total_issuance(0u32) as Balance);

		// This the point the next session's issuance will be calculated and minted
		// on the basis of total_issuance
		roll_to_while_minting(22219, Some(90008));

		assert_eq!(3900445580, Tokens::total_issuance(0u32) as Balance);
	});
}

#[test]
fn issuance_stops_upon_reaching_cap() {
	new_test_ext().execute_with(|| {
		// This the point the next session's issuance will be calculated and minted
		// on the basis of total_issuance

		// At this point the entirety of the missing issuance will be allocated to the next session
		roll_to_while_minting(22219, Some(90008));

		assert_eq!(4000000000, Tokens::total_issuance(0u32) as Balance);

		roll_to_while_minting(22224, Some(892));

		assert_eq!(4000000000, Tokens::total_issuance(0u32) as Balance);

		// Now there is not enough missing issuance to issue so no more mga will be issued

		roll_to_while_minting(23000, Some(0));

		assert_eq!(4000000000, Tokens::total_issuance(0u32) as Balance);
	});
}

#[test]
fn issuance_does_not_stop_upon_burn() {
	new_test_ext().execute_with(|| {
		// This the point the next session's issuance will be calculated and minted
		// on the basis of total_issuance
		roll_to_while_minting(22219, Some(90008));

		assert_eq!(4000000000, Tokens::total_issuance(0u32) as Balance);

		roll_to_while_minting(22221, Some(892));

		orml_tokens::MultiTokenCurrencyAdapter::<Test>::burn_and_settle(
			0u32.into(),
			&0u128,
			100_000,
		)
		.unwrap();

		// At this point the entirety of the missing issuance will be allocated to the next session

		roll_to_while_minting(22224, Some(892));

		assert_eq!(4000000000, Tokens::total_issuance(0u32) as Balance);

		roll_to_while_minting(22229, Some(20000));

		assert_eq!(4000000000, Tokens::total_issuance(0u32) as Balance);

		roll_to_while_minting(24001, Some(0));

		assert_eq!(4000000000, Tokens::total_issuance(0u32) as Balance);
	});
}

#[test]
fn issuance_restarts_upon_burn() {
	new_test_ext().execute_with(|| {
		// This the point the next session's issuance will be calculated and minted
		// on the basis of total_issuance
		roll_to_while_minting(22219, Some(90008));

		assert_eq!(4000000000, Tokens::total_issuance(0u32) as Balance);

		// At this point the entirety of the missing issuance will be allocated to the next session

		roll_to_while_minting(22224, Some(892));

		assert_eq!(4000000000, Tokens::total_issuance(0u32) as Balance);

		// Now there is not enough missing issuance to issue so no more mga will be issued

		roll_to_while_minting(23002, Some(0));

		assert_eq!(4000000000, Tokens::total_issuance(0u32) as Balance);

		orml_tokens::MultiTokenCurrencyAdapter::<Test>::burn_and_settle(
			0u32.into(),
			&0u128,
			100_000,
		)
		.unwrap();

		assert_eq!(3999900000, Tokens::total_issuance(0u32) as Balance);

		roll_to_while_minting(23004, Some(0));

		roll_to_while_minting(23009, Some(20000));

		assert_eq!(4000000000, Tokens::total_issuance(0u32) as Balance);

		roll_to_while_minting(24001, Some(0));

		assert_eq!(4000000000, Tokens::total_issuance(0u32) as Balance);
	});
}

#[test]
fn issuance_after_linear_period_never_execeeds_linear() {
	new_test_ext().execute_with(|| {
		// This the point the next session's issuance will be calculated and minted
		// on the basis of total_issuance
		roll_to_while_minting(22219, Some(90008));

		assert_eq!(4000000000, Tokens::total_issuance(0u32) as Balance);

		// At this point the entirety of the missing issuance will be allocated to the next session

		roll_to_while_minting(22224, Some(892));

		assert_eq!(4000000000, Tokens::total_issuance(0u32) as Balance);

		// Now there is not enough missing issuance to issue so no more mga will be issued

		roll_to_while_minting(23002, Some(0));

		assert_eq!(4000000000, Tokens::total_issuance(0u32) as Balance);

		orml_tokens::MultiTokenCurrencyAdapter::<Test>::burn_and_settle(
			0u32.into(),
			&0u128,
			100_000,
		)
		.unwrap();

		assert_eq!(3999900000, Tokens::total_issuance(0u32) as Balance);

		roll_to_while_minting(23004, Some(0));

		roll_to_while_minting(23009, Some(20000));

		assert_eq!(4000000000, Tokens::total_issuance(0u32) as Balance);

		roll_to_while_minting(23023, Some(0));

		assert_eq!(4000000000, Tokens::total_issuance(0u32) as Balance);

		orml_tokens::MultiTokenCurrencyAdapter::<Test>::burn_and_settle(
			0u32.into(),
			&0u128,
			100_000_000,
		)
		.unwrap();

		assert_eq!(3900000000, Tokens::total_issuance(0u32) as Balance);

		roll_to_while_minting(23024, Some(0));

		roll_to_while_minting(23051, Some(90008));

		assert_eq!(3902700264, Tokens::total_issuance(0u32) as Balance);
	});
}
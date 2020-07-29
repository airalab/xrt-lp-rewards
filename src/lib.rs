///! Simple XRT LP Rewads calculator.

use sp_runtime::curve::PiecewiseLinear;
use sp_runtime::Perbill;

#[cfg(feature = "browser")]
use wasm_bindgen::prelude::*;

pallet_staking_reward_curve::build! {
    const LPI_XRT: PiecewiseLinear<'static> = curve!(
        min_inflation: 0_050_000,
        max_inflation: 0_250_000,
        ideal_stake: 0_500_000,
        falloff: 0_050_000,
        max_piece_count: 1_000,
        test_precision: 0_005_000,
    );
}

#[cfg_attr(feature = "browser", wasm_bindgen)]
pub fn total_payout(
	lp_token_staked: u64,
	total_tokens: u64,
) -> u64 {
    compute_total_payout(lp_token_staked, total_tokens).0
}

pub fn compute_total_payout(
	lp_token_staked: u64,
	total_tokens: u64,
) -> (u64, u64) {
	// Milliseconds per year for the Julian year (365.25 days).
	const MILLISECONDS_PER_YEAR: u64 = 1000 * 3600 * 24 * 36525 / 100;
	// Milliseconds per week for.
	const MILLISECONDS_PER_WEEK: u64 = 1000 * 3600 * 24 * 7 / 100;

	let portion = Perbill::from_rational_approximation(
        MILLISECONDS_PER_WEEK,
        MILLISECONDS_PER_YEAR,
    );
	let payout = portion * LPI_XRT.calculate_for_fraction_times_denominator(
		lp_token_staked,
		total_tokens,
	);
	let maximum = portion * (LPI_XRT.maximum * total_tokens);
	(payout, maximum)
}

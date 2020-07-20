///! Simple XRT LP Rewads calculator.

use sp_runtime::curve::PiecewiseLinear;
use sp_runtime::Perbill;
use structopt::StructOpt;

#[cfg(feature = "browser")]
use wasm_bindgen::prelude::*;

#[derive(StructOpt)]
#[structopt(name = "xrt-lp-rewards", about = "Robonomics (XRT) LP reward calculator.")]
struct Opt {
    /// LP staked amount in Wn (1 XRT = 10^9 Wn).
    staked: u64,

    /// Circulation amount in Wn (1 XRT = 10^9 Wn). 
    total: u64,
}

pallet_staking_reward_curve::build! {
    const LPI_XRT: PiecewiseLinear<'static> = curve!(
        min_inflation: 0_500_000,
        max_inflation: 0_990_000,
        ideal_stake: 0_800_000,
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

fn compute_total_payout(
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
	(payout * 100, maximum * 100)
}

fn wn2xrt(wn: u64) -> f32 {
    (wn as f32) / 1_000_000_000f32
}

fn main() {
    let opt = Opt::from_args();
    let (payout, maximum) = compute_total_payout(opt.staked, opt.total);
    let ratio = Perbill::from_rational_approximation(opt.staked, opt.total) * 100u32;
    println!(
        "Staked: {}%\nPayout: {} XRT ({} Wn)\nMaximum: {} XRT ({} Wn)",
        ratio,
        wn2xrt(payout), payout,
        wn2xrt(maximum), maximum,
    );
}

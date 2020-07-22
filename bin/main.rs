use sp_runtime::Perbill;
use structopt::StructOpt;
use xrt_lp_rewards::compute_total_payout;

#[derive(StructOpt)]
#[structopt(name = "xrt-lp-rewards", about = "Robonomics (XRT) LP reward calculator.")]
struct Opt {
    /// LP staked amount in Wn (1 XRT = 10^9 Wn).
    staked: u64,

    /// Circulation amount in Wn (1 XRT = 10^9 Wn). 
    total: u64,
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

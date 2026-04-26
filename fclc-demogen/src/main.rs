//! fclc-demogen — synthetic OMOP-compatible demo data generator.
//!
//! Rust port of `scripts/generate_demo_data.py`. Produces
//! `data/clinic_node{N}_demo.csv` for testing FCLC without real patient data.
//!
//! Usage:
//!     fclc-demogen --nodes 3 --records 500 --seed 42
//!     fclc-demogen --out data/

use clap::Parser;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, Normal};
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

const CURRENT_YEAR: i32 = 2026;

const AGE_BINS: &[i32] = &[30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80];
const AGE_WEIGHTS: &[f64] = &[0.10, 0.10, 0.12, 0.13, 0.14, 0.13, 0.12, 0.10, 0.07, 0.05, 0.04];

const HBA1C_MEAN: f64 = 8.0;
const HBA1C_SD: f64 = 1.5;
const HBA1C_MIN: f64 = 5.5;
const HBA1C_MAX: f64 = 14.0;

const BMI_MEAN: f64 = 32.0;
const BMI_SD: f64 = 6.0;
const BMI_MIN: f64 = 18.0;
const BMI_MAX: f64 = 55.0;

const NEPHROPATHY_BASE: f64 = 0.22;
const RETINOPATHY_BASE: f64 = 0.28;
const HOSP_LAST_RATE: f64 = 0.18;

const NEPHROPATHY_HOSP_OR: f64 = 2.5;
const RETINOPATHY_HOSP_OR: f64 = 1.8;
const AGE_HOSP_SLOPE: f64 = 0.012;

#[derive(Parser, Debug)]
#[command(name = "fclc-demogen", about = "Generate synthetic OMOP CSV for FCLC demos")]
struct Cli {
    #[arg(long, default_value_t = 3)]
    nodes: usize,
    #[arg(long, default_value_t = 500)]
    records: usize,
    #[arg(long, default_value_t = 42)]
    seed: u64,
    #[arg(long, default_value = "data")]
    out: PathBuf,
}

#[derive(Debug, Serialize)]
struct OmopRecord {
    age_years: i32,
    sex: String,
    diabetes_year: i32,
    hba1c: f64,
    bmi: f64,
    has_nephropathy: u8,
    has_retinopathy: u8,
    hospitalized_last_12m: u8,
    hospitalized_next_12m: u8,
}

fn clipped_normal(rng: &mut ChaCha8Rng, mean: f64, sd: f64, lo: f64, hi: f64) -> f64 {
    let n = Normal::new(mean, sd).unwrap();
    n.sample(rng).max(lo).min(hi)
}

fn weighted_age(rng: &mut ChaCha8Rng) -> i32 {
    let dist = WeightedIndex::new(AGE_WEIGHTS).unwrap();
    let idx = dist.sample(rng);
    let jitter: i32 = rng.gen_range(-2..=2);
    AGE_BINS[idx] + jitter
}

fn generate_record(rng: &mut ChaCha8Rng, node_idx: usize) -> OmopRecord {
    let age = weighted_age(rng).max(18).min(90);
    let sex = if rng.gen::<f64>() < 0.48 { "M" } else { "F" }.to_string();

    // exponentially-distributed years since diagnosis
    let years_since_dx = (-(rng.gen::<f64>().ln()) / 0.12).round() as i32 + 1;
    let years_since_dx = years_since_dx.min(40).max(1);
    let diabetes_year = CURRENT_YEAR - years_since_dx;

    let node_hba1c_shift = (node_idx as f64 - 1.0) * 0.4;
    let hba1c = (clipped_normal(rng, HBA1C_MEAN + node_hba1c_shift, HBA1C_SD, HBA1C_MIN, HBA1C_MAX) * 10.0).round() / 10.0;
    let bmi = (clipped_normal(rng, BMI_MEAN, BMI_SD, BMI_MIN, BMI_MAX) * 10.0).round() / 10.0;

    let node_risk = [0.8, 1.0, 1.3][node_idx % 3];
    let nephropathy: u8 = (rng.gen::<f64>() < NEPHROPATHY_BASE * node_risk) as u8;
    let retinopathy: u8 = (rng.gen::<f64>() < RETINOPATHY_BASE * node_risk) as u8;
    let hosp_last: u8 = (rng.gen::<f64>() < HOSP_LAST_RATE * node_risk) as u8;

    let log_odds = -3.5
        + AGE_HOSP_SLOPE * age as f64
        + NEPHROPATHY_HOSP_OR.ln() * nephropathy as f64
        + RETINOPATHY_HOSP_OR.ln() * retinopathy as f64
        + 0.15 * (hba1c - 7.0)
        + 0.8 * hosp_last as f64
        + 0.1 * (node_idx as f64 - 1.0);
    let prob_hosp_next = 1.0 / (1.0 + (-log_odds).exp());
    let hosp_next: u8 = (rng.gen::<f64>() < prob_hosp_next) as u8;

    OmopRecord {
        age_years: age,
        sex,
        diabetes_year,
        hba1c,
        bmi,
        has_nephropathy: nephropathy,
        has_retinopathy: retinopathy,
        hospitalized_last_12m: hosp_last,
        hospitalized_next_12m: hosp_next,
    }
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    fs::create_dir_all(&cli.out)?;

    for node_idx in 1..=cli.nodes {
        let node_seed = cli.seed + (node_idx as u64) * 31337;
        let mut rng = ChaCha8Rng::seed_from_u64(node_seed);

        let records: Vec<OmopRecord> = (0..cli.records).map(|_| generate_record(&mut rng, node_idx)).collect();

        let path = cli.out.join(format!("clinic_node{}_demo.csv", node_idx));
        let mut wtr = csv::Writer::from_path(&path)?;
        for r in &records {
            wtr.serialize(r)?;
        }
        wtr.flush()?;
        println!("  ✓ {} ({} records)", path.display(), records.len());
    }
    Ok(())
}

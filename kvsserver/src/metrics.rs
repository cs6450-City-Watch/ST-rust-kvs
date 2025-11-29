//! Performance metrics module for KVS operations
//!
//! This module provides thread-safe atomic counters for tracking KVS operations
//! and calculating throughput rates in operations per second.

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

/// Global atomic counters for tracking operation counts
pub static GETS_COUNT: AtomicU64 = AtomicU64::new(0);
pub static PUTS_COUNT: AtomicU64 = AtomicU64::new(0);
pub static OPS_COUNT: AtomicU64 = AtomicU64::new(0);
pub static COMMITS_COUNT: AtomicU64 = AtomicU64::new(0);
pub static ABORTS_COUNT: AtomicU64 = AtomicU64::new(0);

/// Increments the get operation counter
pub fn increment_gets() {
    GETS_COUNT.fetch_add(1, Ordering::Relaxed);
}

/// Increments the put operation counter
pub fn increment_puts() {
    PUTS_COUNT.fetch_add(1, Ordering::Relaxed);
}

/// Increments the ops (begin) operation counter
pub fn increment_ops() {
    OPS_COUNT.fetch_add(1, Ordering::Relaxed);
}

/// Increments the commit operation counter
pub fn increment_commits() {
    COMMITS_COUNT.fetch_add(1, Ordering::Relaxed);
}

/// Increments the abort operation counter
pub fn increment_aborts() {
    ABORTS_COUNT.fetch_add(1, Ordering::Relaxed);
}

/// Resets all counters and returns the previous values along with elapsed time
pub fn reset_and_get_counts() -> (u64, u64, u64, u64, u64) {
    let gets = GETS_COUNT.swap(0, Ordering::Relaxed);
    let puts = PUTS_COUNT.swap(0, Ordering::Relaxed);
    let ops = OPS_COUNT.swap(0, Ordering::Relaxed);
    let commits = COMMITS_COUNT.swap(0, Ordering::Relaxed);
    let aborts = ABORTS_COUNT.swap(0, Ordering::Relaxed);

    (gets, puts, ops, commits, aborts)
}

/// Calculates and prints the current throughput rates
pub fn print_rates(gets: u64, puts: u64, ops: u64, commits: u64, aborts: u64, elapsed: Duration) {
    let elapsed_secs = elapsed.as_secs_f64();

    if elapsed_secs > 0.0 {
        let gets_per_sec = gets as f64 / elapsed_secs;
        let puts_per_sec = puts as f64 / elapsed_secs;
        let ops_per_sec = ops as f64 / elapsed_secs;
        let commits_per_sec = commits as f64 / elapsed_secs;
        let aborts_per_sec = aborts as f64 / elapsed_secs;

        println!(
            "gets/s: {gets_per_sec:.2}, puts/s: {puts_per_sec:.2}, ops/s: {ops_per_sec:.2}, commits/s: {commits_per_sec:.2}, aborts/s: {aborts_per_sec:.2}"
        );
    }
}

/// Starts the background metrics reporting task
pub async fn start_metrics_reporting() {
    let mut last_time = Instant::now();
    let mut interval = tokio::time::interval(Duration::from_secs(1));

    loop {
        interval.tick().await;

        let now = Instant::now();
        let elapsed = now.duration_since(last_time);
        last_time = now;

        let (gets, puts, ops, commits, aborts) = reset_and_get_counts();
        print_rates(gets, puts, ops, commits, aborts, elapsed);
    }
}

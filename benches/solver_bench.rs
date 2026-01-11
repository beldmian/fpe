// MCCFR Solver Benchmarks
//
// Benchmarks for the Monte Carlo CFR solver implementation.
// These measure performance across different scenarios and iteration counts.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use fpe::models::{Card, GameState, Hand, Position, Range};
use fpe::solver::solve;
use std::str::FromStr;

/// Benchmark: River decision with nuts vs range (100 iterations)
fn benchmark_solver_river_nuts_100(c: &mut Criterion) {
    let hero = Hand::from_str("AhKh").unwrap();
    let board = vec![
        Card::from_str("Qh").unwrap(),
        Card::from_str("Jh").unwrap(),
        Card::from_str("Th").unwrap(),
        Card::from_str("2s").unwrap(),
        Card::from_str("3d").unwrap(),
    ];
    let villain_range = Range::from_notation("22+").unwrap();

    let state = GameState::new(hero, board, 10.0, 100.0, 0.0, Position::IP, villain_range).unwrap();

    c.bench_function("solve_river_nuts_100_iter", |b| {
        b.iter(|| solve(state.clone(), 100))
    });
}

/// Benchmark: River decision with nuts vs range (1000 iterations)
fn benchmark_solver_river_nuts_1000(c: &mut Criterion) {
    let hero = Hand::from_str("AhKh").unwrap();
    let board = vec![
        Card::from_str("Qh").unwrap(),
        Card::from_str("Jh").unwrap(),
        Card::from_str("Th").unwrap(),
        Card::from_str("2s").unwrap(),
        Card::from_str("3d").unwrap(),
    ];
    let villain_range = Range::from_notation("22+").unwrap();

    let state = GameState::new(hero, board, 10.0, 100.0, 0.0, Position::IP, villain_range).unwrap();

    c.bench_function("solve_river_nuts_1000_iter", |b| {
        b.iter(|| solve(state.clone(), 1000))
    });
}

/// Benchmark: River decision with polarized range (default iterations)
fn benchmark_solver_river_polarized(c: &mut Criterion) {
    let hero = Hand::from_str("AsKd").unwrap();
    let board = vec![
        Card::from_str("Ah").unwrap(),
        Card::from_str("Kh").unwrap(),
        Card::from_str("Qh").unwrap(),
        Card::from_str("2s").unwrap(),
        Card::from_str("3d").unwrap(),
    ];
    // Polarized range: nuts and air
    let villain_range = Range::from_notation("JJ+,AK,72o,83o").unwrap();

    let state = GameState::new(hero, board, 10.0, 100.0, 0.0, Position::IP, villain_range).unwrap();

    c.bench_function("solve_river_polarized_1000_iter", |b| {
        b.iter(|| solve(state.clone(), 1000))
    });
}

/// Benchmark: Flop decision with medium SPR
fn benchmark_solver_flop_medium_spr(c: &mut Criterion) {
    let hero = Hand::from_str("AsAd").unwrap();
    let board = vec![
        Card::from_str("Kh").unwrap(),
        Card::from_str("9s").unwrap(),
        Card::from_str("5c").unwrap(),
    ];
    let villain_range = Range::from_notation("22+,AK,AQ,KQ").unwrap();

    let state = GameState::new(hero, board, 15.0, 75.0, 0.0, Position::OOP, villain_range).unwrap();

    c.bench_function("solve_flop_medium_spr_100_iter", |b| {
        b.iter(|| solve(state.clone(), 100))
    });
}

/// Benchmark: Iteration count scaling
fn benchmark_iteration_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("iteration_scaling");

    let hero = Hand::from_str("AsKd").unwrap();
    let board = vec![
        Card::from_str("Ah").unwrap(),
        Card::from_str("Kh").unwrap(),
        Card::from_str("Qh").unwrap(),
        Card::from_str("2s").unwrap(),
        Card::from_str("3d").unwrap(),
    ];
    let villain_range = Range::from_notation("22+,AK,KQ").unwrap();

    for iterations in [100, 500, 1000, 5000].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(iterations),
            iterations,
            |b, &iter_count| {
                let state = GameState::new(
                    hero.clone(),
                    board.clone(),
                    10.0,
                    100.0,
                    0.0,
                    Position::IP,
                    villain_range.clone(),
                ).unwrap();
                b.iter(|| solve(state.clone(), iter_count))
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    benchmark_solver_river_nuts_100,
    benchmark_solver_river_nuts_1000,
    benchmark_solver_river_polarized,
    benchmark_solver_flop_medium_spr,
    benchmark_iteration_scaling
);
criterion_main!(benches);

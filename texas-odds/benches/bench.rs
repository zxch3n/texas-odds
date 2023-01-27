use criterion::{criterion_group, criterion_main, Criterion};
use texas_odds::odds::Stage;

pub fn run(c: &mut Criterion) {
    let mut b = c.benchmark_group("texas");
    b.sample_size(10);
    b.bench_function("run without community cards", |b| {
        b.iter(|| {
            let stage = Stage::new(["12".into(), "22".into()], &[]);
            stage.win_rate_with_n_players(5);
        })
    });

    b.bench_function("run with 3 community cards", |b| {
        b.iter(|| {
            let stage = Stage::new(
                ["12".into(), "22".into()],
                &["23".into(), "45".into(), "15".into()],
            );
            stage.win_rate_with_n_players(5);
        })
    });

    b.bench_function("run with 4 community cards", |b| {
        b.iter(|| {
            let stage = Stage::new(
                ["12".into(), "22".into()],
                &["23".into(), "45".into(), "15".into(), "22".into()],
            );
            stage.win_rate_with_n_players(5);
        })
    });

    b.bench_function("run with 5 community cards", |b| {
        b.iter(|| {
            let stage = Stage::new(
                ["12".into(), "22".into()],
                &[
                    "23".into(),
                    "45".into(),
                    "15".into(),
                    "22".into(),
                    "DK".into(),
                ],
            );
            stage.win_rate_with_n_players(5);
        })
    });
}

criterion_group!(benches, run);
criterion_main!(benches);

# Obvious

Just my simple little logic solver and calculator....after all logic is all obvious isn't it?

## Usage

Most of the features of this crate are showcased in `examples/`.

For the performance of the parallel vs the non parallel calculator for truth tables you can check out
`benches/truth_table_benchmark.rs`. The results on my machine look as follows:

TODO: Add image

So it appears the parallel calculator outperform the non parallel one by a factor of 2-ish.

use std::time::Instant;
use inlining_experiment::inlining_experiment::InliningExperiment;

const NUMBER_OF_ITERATIONS: u32 = 1_000_000;

fn main()
{
    let mut experiment = InliningExperiment::new();

    experiment = experimental_run(experiment, true);

    experimental_run(experiment, false);
}

#[inline(always)]
fn experimental_run(mut experiment: InliningExperiment, inline: bool) -> InliningExperiment
{
    let start_time = Instant::now();

    println!("Starting run with {} iterations.  Inline is set to {}.", NUMBER_OF_ITERATIONS, inline);

    let mut counter: u32 = 0;
    while counter < NUMBER_OF_ITERATIONS
    {
        experiment.flip_array(inline);
        counter = counter + 1;
    }

    let finish_time = Instant::now();
    let elapsed_time = finish_time - start_time;

    println!("Run finished.  Elapsed time is {} ms.", elapsed_time.as_millis());

    return experiment;
}
extern crate inlining_experiment;

use inlining_experiment::inlining_experiment::InliningExperiment;
const ARRAY_SIZE: usize = 10_000;

#[test]
fn returns_large_array()
{
    let experiment = InliningExperiment::new();
    let mut test_array: [u16; ARRAY_SIZE] = [0; ARRAY_SIZE];

    for counter in 0..ARRAY_SIZE
    {
        test_array[counter] = counter as u16;
    }

    assert_eq!(test_array, experiment.get_array());
}

#[test]
fn flips_large_array_inlined()
{
    let mut experiment = InliningExperiment::new();
    let mut test_array: [u16; ARRAY_SIZE] = [0; ARRAY_SIZE];

    for counter in 0..ARRAY_SIZE
    {
        test_array[ARRAY_SIZE - counter - 1] = counter as u16;
    }

    experiment.flip_array(true);

    assert_eq!(test_array, experiment.get_array());
}

#[test]
fn flips_large_array_outlined()
{
    let mut experiment = InliningExperiment::new();
    let mut test_array: [u16; ARRAY_SIZE] = [0; ARRAY_SIZE];

    for counter in 0..ARRAY_SIZE
    {
        test_array[ARRAY_SIZE - counter - 1] = counter as u16;
    }

    experiment.flip_array(false);

    assert_eq!(test_array, experiment.get_array());
}
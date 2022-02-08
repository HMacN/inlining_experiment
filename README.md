inlining_experiment

A very simple little experiment where I played about with Rust's "inline" tags.  The program creates a 10,000 element array of integers and flips it 1,000,000 times.  It does it once with the loop body function inlined, and then repeats it with the loop body function not inlined.  On my machine I got a consistent speed decrease with the non-inlined function, of about 0.5%.

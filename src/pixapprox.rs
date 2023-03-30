use rand::{rngs::StdRng, SeedableRng};
use rayon::prelude::*;
use std::time::{Duration, Instant};
use std::{fs::File, io::Write, iter::zip};

use crate::eval::eval;
use crate::{
    expr::Program,
    mutate::mutate,
    myimage::{GrayScaleImage, MyRgbImage},
    population::{Individual, Population},
    state::State,
};

/// Set to true if you want a <number>.txt written containing the code for the image
const OUTPUT_CODE: bool = false;

/// Set to a number n to keep the previous generations n best individuals
const USE_ELITISM: usize = 0;

/// The number of mutations done on each individual
const NUMBER_OF_MUTATIONS: usize = 1;

/// The number of the best individuals the next generation will be based on
const NBEST: usize = 10;

/// The maxmimum number of generations before the program ends
const NGENERATIONS: u32 = 15000;

/// The number of individuals in each generation.
/// Higher number is slower but not always better.
/// 20-1000 seems like good values depending on the image.
/// Lower value means the code size will increase at a faster rate.
const POPULATION_SIZE: usize = 50;

/// Number of variables, 2 means x and y
const NVARS: usize = 2;

pub fn approx_pic() {
    let mut rng = StdRng::from_rng(rand::thread_rng()).unwrap();

    //
    // The larger versions will quickly take a lot of time.
    // When experimenting, prefer the use of the _small.png ones.
    //

    // These pictures converge pretty fast:
    let file_name = "images/filled_circle.png";
    // let file_name = "images/mona_lisa.png";
    // let file_name = "images/mona_lisa_small.png";
    // let file_name = "images/filled_thing.png";
    // let file_name = "images/heavy.png";
    // let file_name = "images/heavy_small.png";
    // let file_name = "images/cornell.png";
    // let file_name = "images/cornell_small.png";
    let file_name = "images/zebra_skin_by_photolight.png";

    // These converge slow, seems impossible to get good:
    // Straight lines and squares are not easy to calculate
    // let file_name = "images/mondriaan.png";
    // let file_name = "images/mondriaan_small.png";
    // let file_name = "images/red_apple.png";

    let goal_image = MyRgbImage::load_rgb_image(file_name)
        .unwrap()
        .to_gray_scale_image();

    let npixels = goal_image.data.len() as u64;

    let mut last_error: u64 = u64::MAX;
    let mut population = Population::random(&rng, POPULATION_SIZE);
    let mut file_number = 0u64;
    for gen in 0..NGENERATIONS {
        let start_time = Instant::now();

        simulate(&goal_image, &mut population);

        let simulate_time = start_time.elapsed();

        let best_ind_error = population.individuals[0].error.unwrap();
        if best_ind_error < last_error {
            file_number += 1;
            save_best(&goal_image, &mut population, file_number);
            last_error = best_ind_error;
        }
        print_best_info(&population, gen, npixels, simulate_time);

        population = evolve(gen, population, &mut rng, NVARS);
    }
}

fn print_best_info(population: &Population, gen: u32, npixels: u64, duration: Duration) {
    let best_ind = &population.individuals[0];

    let best_ind_error = best_ind.error.unwrap() as f32;
    let code_size = best_ind.prg.code.len();
    let error_per_pixel = best_ind_error / (npixels as f32);
    let time = duration.as_millis();

    println!("Gen: {gen}, Code: {code_size}, Error: {error_per_pixel:.4}, Time: {time} ms");
}

fn evolve(gen: u32, population: Population, rng: &mut StdRng, nvars: usize) -> Population {
    let mut new_population = Population {
        individuals: Vec::with_capacity(POPULATION_SIZE),
    };

    // New population is a mutated version of the NBEST individuals from previous generation
    for i in 0..POPULATION_SIZE {
        let mut individual = population.individuals[i % NBEST].clone();

        for _ in 0..NUMBER_OF_MUTATIONS {
            // Mutate
            mutate(rng, &mut individual.prg, nvars);
        }

        new_population.individuals.push(individual);
    }

    // Elitism - remember the best individuals from the previous generation
    #[allow(clippy::reversed_empty_ranges)]
    for i in 0..USE_ELITISM {
        new_population.individuals[i] = population.individuals[i].clone();
    }

    new_population
}

fn simulate(goal_image: &GrayScaleImage, population: &mut Population) {
    // Eval all exprs
    population
        .individuals
        .par_iter_mut()
        .for_each(|individual| eval_individual(goal_image, individual));

    // Sort by error
    population
        .individuals
        .sort_by(|a, b| a.error.unwrap().cmp(&b.error.unwrap()));
}

fn save_best(goal_image: &GrayScaleImage, population: &mut Population, file_number: u64) {
    let best_ind = &population.individuals[0];

    // Save image result
    let filename = format!("result/{:05}.png", file_number);
    let generated_image = eval_into_image(goal_image, &best_ind.prg);
    save_comparison_image(goal_image, &generated_image, filename.as_str());

    if OUTPUT_CODE {
        // Save code result
        let filename = format!("result/{:05}.txt", file_number);
        let mut output = File::create(filename).unwrap();
        let line = format!("{}", best_ind.prg);
        output.write_all(line.as_bytes()).unwrap();
    }
}

fn eval_individual(goal_image: &GrayScaleImage, individual: &mut Individual) {
    let generated_image = eval_into_image(goal_image, &individual.prg);
    let error_sum = calc_image_error(goal_image, &generated_image);

    individual.error = Some(error_sum);
}

fn eval_into_image(goal_image: &GrayScaleImage, prg: &Program) -> GrayScaleImage {
    let mut image = GrayScaleImage::with_dimensions(goal_image.width, goal_image.height);

    // State is where x and y are stored
    let mut state = State::new(NVARS);

    for y in 0..image.height {
        for x in 0..image.width {
            // Convert width and height from
            //   0..height/width
            // to
            //   -1.0 to +1.0
            state.vars[0] = (x as f32) / (image.width as f32) * 2.0 - 1.0;
            state.vars[1] = (y as f32) / (image.height as f32) * 2.0 - 1.0;

            let mut result = eval(prg, &state);

            // Limit the output to stay between -1.0 and 1.0
            result = result.min(1.0).max(-1.0);

            // Rescale the value to be from 0-255
            result = result * 127.0 + 128.0;

            let pix = result.trunc() as u8;
            image.data.push(pix);
        }
    }

    image
}

pub fn save_comparison_image(goal: &GrayScaleImage, generated: &GrayScaleImage, filename: &str) {
    assert_eq!(goal.width, generated.width);
    assert_eq!(goal.height, generated.height);

    let mut image = GrayScaleImage::with_dimensions(goal.width * 2, goal.height);

    let mut gen_bytes = generated.data.iter();
    let mut goal_bytes = goal.data.iter();
    for y in 0..goal.height {
        for x in 0..goal.width {
            let b = *goal_bytes.next().unwrap();
            image.data.push(b);
        }
        for x in 0..goal.width {
            let b = *gen_bytes.next().unwrap();
            image.data.push(b);
        }
    }

    image.save_file(filename).unwrap();
}

pub fn calc_image_error(goal_image: &GrayScaleImage, generated_image: &GrayScaleImage) -> u64 {
    let mut sum_error = 0;

    for (p1, p2) in zip(&goal_image.data, &generated_image.data) {
        let error = (*p1 as u64).abs_diff(*p2 as u64);
        sum_error += error * error;
    }

    sum_error
}

// ffmpeg -i result/result_gen_%05d.png out.mp4
use rand::{rngs::StdRng, SeedableRng};
use rayon::prelude::*;
use std::time::{Duration, Instant};
use std::{fs::File, io::Write, iter::zip};

use crate::{
    expr::{eval, Program},
    mutate::mutate,
    myimage::{GrayScaleImage, MyRgbImage},
    population::{Individual, Population},
    state::State,
};

const NGENERATIONS: u32 = 15000;
const NVARS: usize = 2;
const POPULATION_SIZE: usize = 70;

pub fn approx_pic() {
    let mut rng = StdRng::from_rng(rand::thread_rng()).unwrap();

    //
    // The larger versions will quickly take a lot of time.
    // When experimenting, prefer the use of the _small.png ones.
    //

    // These pictures converge pretty fast:
    // let file_name = "images/filled_circle.png";
    // let file_name = "images/mona_lisa.png";
    let file_name = "images/mona_lisa_small.png";
    // let file_name = "images/filled_thing.png";
    // let file_name = "images/heavy.png";
    // let file_name = "images/heavy_small.png";
    // let file_name = "images/cornell.png";
    // let file_name = "images/cornell_small.png";

    // These converge slow, seems impossible to get good:
    // let file_name = "images/mondriaan.png";
    // let file_name = "images/mondriaan_small.png";
    // let file_name = "images/red_apple.png";

    let goal_image = MyRgbImage::load_rgb_image(file_name)
        .unwrap()
        .to_gray_scale_image();

    let npixels = goal_image.data.len() as u64;

    let mut last_error: u64 = u64::MAX;
    let mut population = Population::random(&rng, POPULATION_SIZE);

    for gen in 0..NGENERATIONS {
        let start_time = Instant::now();

        simulate(&goal_image, &mut population);

        let simulate_time = start_time.elapsed();

        let best_ind_error = population.individuals[0].error.unwrap();
        if best_ind_error < last_error {
            save_best(gen, &goal_image, &mut population);
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

    println!("Gen: {gen}, Population: {POPULATION_SIZE}, Code: {code_size}, Error: {error_per_pixel},\tTime: {time} ms");
}

fn evolve(gen: u32, population: Population, rng: &mut StdRng, nvars: usize) -> Population {
    let mut new_population = Population {
        individuals: Vec::with_capacity(POPULATION_SIZE),
    };

    // New population is a mutated version of the nbest versions from previous generation
    let nbest = 10;
    for i in 0..POPULATION_SIZE {
        let mut individual = population.individuals[i % nbest].clone();

        // Mutate
        mutate(rng, &mut individual.prg, nvars);
        mutate(rng, &mut individual.prg, nvars);

        new_population.individuals.push(individual);
    }

    // Elitism - remember the 2 best individuals from the previous generation
    for i in 0..2 {
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

fn save_best(gen: u32, goal_image: &GrayScaleImage, population: &mut Population) {
    let best_ind = &population.individuals[0];

    // Save image result
    let filename = format!("result/result_gen_{:05}.png", gen);
    let generated_image = eval_into_image(goal_image, &best_ind.prg);
    save_comparison_image(goal_image, &generated_image, filename.as_str());

    // Save code result
    let filename = format!("result/result_gen_{:05}.txt", gen);
    let mut output = File::create(filename).unwrap();
    let line = format!("{}", best_ind.prg);
    output.write_all(line.as_bytes()).unwrap();
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
            image.data.push(*goal_bytes.next().unwrap());
        }
        for x in 0..goal.width {
            image.data.push(*gen_bytes.next().unwrap());
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

use rand::Rng;
use std::env;
use std::time::Instant;

const ROWS: usize = 600;
const COLS: usize = 600;

fn main() {
    //check command line arguments
    let print_result = false;
    let mut option:usize = 1;

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {

        match args[1].parse::<usize>() {
            Ok(number) => {
                option = number;
            }
            Err(_) => {
                eprintln!("Parameter passed to run the program is not a valid number");
                return;
            }
        }    
    }

    //print welcome message
    print_welcome_message();

    //declare matrices using arrays
    let mut a_matrix : [[i32; COLS]; ROWS] = [[0; COLS];ROWS];
    let mut b_matrix : [[i32; COLS]; ROWS] = [[0; COLS];ROWS];
    let mut result_matrix_basic: [[i32; COLS]; ROWS] = [[0; COLS];ROWS];
    let mut result_matrix_inverted_loop: [[i32; COLS]; ROWS] = [[0; COLS];ROWS];
    let mut result_matrix_tiled: [[i32; COLS]; ROWS] = [[0; COLS];ROWS];

    //populate matrices a and b
    matrix_populate(&mut a_matrix);
    matrix_populate(&mut b_matrix);


    //multiply the matrices based on the option passed as a parameter during program spawn
    if option == 1 {
        println!(
            "You have chosen option {} : Basic matrix multiplication",
            option
        );
        println!("-----------------------------------------------------------------------");
        basic_matrix_multiply(&a_matrix, &b_matrix, &mut result_matrix_basic);
    }
    else if option == 2 {
        println!(
            "You have chosen option {} : Inverted loop matrix multiplication",
            option
        );
        println!("-----------------------------------------------------------------------");
        inverted_loop_matrix_multiply(&a_matrix, &b_matrix, &mut result_matrix_inverted_loop);
    }
    else {
        println!(
            "You have chosen option {} : Tiled matrix multiplication",
            option
        );
        tiled_matrix_mutiply(&a_matrix, &b_matrix, &mut result_matrix_tiled);
    }
    
    //print results
    if print_result {
        println!("Printing result_matrix_basic");
        matrix_print(&result_matrix_basic);

        println!("Printing result_matrix_inverted_loop");
        matrix_print(&result_matrix_inverted_loop);

        println!("Printing result_matrix_tiled");
        matrix_print(&result_matrix_tiled);
    }
}

fn matrix_populate(matrix: &mut [[i32; COLS];ROWS]) {
    let mut rng = rand::thread_rng();
    for i in 0..ROWS {
        for j in 0..COLS {
            matrix[i][j] = rng.gen_range(0..=100); // Adjust the range as needed
        }
    }
}

fn print_welcome_message() {

    println!("-----------------------------------------------------------------------");
    println!("Welcome to the rusty matrix multiplication implementation using Arrays");
    println!("-----------------------------------------------------------------------");
    println!(
        "Our input and result matrices have {} rows and {} columns ",
        ROWS, COLS
    );
    println!("-----------------------------------------------------------------------");
}

fn matrix_print(matrix: & [[i32; COLS];ROWS]) {
    println!("...................................................");
    for i in 0..ROWS {
        for j in 0..COLS {
            print!("  {}", matrix[i][j]);
        }
        println!("");
    }
}

fn basic_matrix_multiply(
    a_matrix: &[[i32; COLS];ROWS],
    b_matrix: &[[i32; COLS];ROWS],
    result_matrix: &mut [[i32; COLS];ROWS],
) {
    let start_time = Instant::now();

    for i in 0..ROWS {
        for j in 0..COLS {
            for k in 0..COLS {
                result_matrix[i][j] += a_matrix[i][k] * b_matrix[k][j];
            }
        }
    }

    let elapsed_time = start_time.elapsed();

    println!(
        "Elapsed time basic_matrix_multiply: {}s {}ms",
        elapsed_time.as_secs(),
        elapsed_time.subsec_millis()
    );
}

fn inverted_loop_matrix_multiply(
    a_matrix: &[[i32; COLS];ROWS],
    b_matrix: &[[i32; COLS];ROWS],
    result_matrix: &mut [[i32; COLS];ROWS],
) {
    let start_time = Instant::now();

    for i in 0..ROWS {
        for k in 0..COLS {
            for j in 0..COLS {
                result_matrix[i][j] += a_matrix[i][k] * b_matrix[k][j];
            }
        }
    }

    let elapsed_time = start_time.elapsed();

    println!(
        "Elapsed time inverted_loop_matrix_multiply: {}s {}ms",
        elapsed_time.as_secs(),
        elapsed_time.subsec_millis()
    );
}

fn tiled_matrix_mutiply(
    a_matrix: &[[i32; COLS];ROWS],
    b_matrix: &[[i32; COLS];ROWS],
    result_matrix: &mut [[i32; COLS];ROWS],
) {
    let start_time = Instant::now();
    let tile_size = 4;

    for ii in (0..ROWS).step_by(tile_size) {
        for kk in (0..ROWS).step_by(tile_size) {
            for jj in (0..ROWS).step_by(tile_size) {
                for i in ii..ii + tile_size {
                    for k in kk..kk + tile_size {
                        for j in jj..jj + tile_size {
                            if i < ROWS && j < ROWS && k < ROWS {
                                result_matrix[i][j] += a_matrix[i][k] * b_matrix[k][j];
                            }
                        }
                    }
                }
            }
        }
    }

    let elapsed_time = start_time.elapsed();

    println!(
        "Elapsed time tiled_matrix_mutiply: {}s {}ms",
        elapsed_time.as_secs(),
        elapsed_time.subsec_millis()
    );
}

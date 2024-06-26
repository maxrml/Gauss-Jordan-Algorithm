/*
How can you implement the Gauss-Jordan-Algorithm to bring a matrix in their row echelon form?

With this program I want to offer a quite easy way to do so, since there is not the one right algorithm,
there are many ways to accomplish the Gauss-Elimination. My way might not be the best in terms of time complexity
[O(m * n * log(n))] but for the sake of general complexity "coding-difficulty-like" this might be a good choice!

The idea is quite simple and works in a few steps:
1. The Matrix, represented by a Vector of i32 Vectors, gets sorted such that rows with the more zeros in
the front are lower sorted.
2. Starting at row 1, you iterate through the row and look for the first element != 0 (Pivot Element)
3. You search in the lower rows if in row 2, 3... there are also pivot elements at the same space
if yes: eliminate the pivot elements in the lower rows by elementary line transformation
if not: go a row lower and search for the pivot element and continue

This process goes on till you reach the last line of the matrix!
 */



fn main() {
    // First part: Terminal handling of the input into a matrix
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    let mut input = String::new();
    println!("Enter the matrix you want to get in row echelon form!");
    println!("Please give your inputs row by row(just i32), seperated by a semicolon in the following form:");
    println!("1 2 3 ; 2 3 4 ; 3 4 5");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error by reading your Input!");
    let unedited_rows: Vec<&str> = input.split(';').collect(); //Split the input by semicolons
    for unedited_row in unedited_rows { //Iterate through the by semicolon split strings
        let edited_row: Vec<i32> = unedited_row //Create an instance for the edited rows, which should include each one i32
            .trim()//Delete whitespace in the end and beginning and line breaks
            .split_whitespace()//Split the remaining numbers by whitespace
            .map(|num| num.parse()//Use for each split i32 the closure num.parse() such that each string number gets an i32 value
                .expect("Error by parsing your number!"))
            .collect();//Safe each i32 in the vec edited_row
        matrix.push(edited_row);
    }
    // Second part: From the input matrix to the matrix in row echelon form
    let solved_matrix = gauss_elimination(&mut matrix);
    // Third part: Print the Vec<Vec<i32>> as an actual matrix
    let mut counter = 0;
    // Create a counter such that the matrix uses line breaks at the right point
    let row_length = solved_matrix[0].len();
    for row in solved_matrix.iter() {
        for element in row.iter() {
            //For each element printed the counter goes up by 1, when the counter equals the row length set it back to 0 and do a line break
            counter += 1;
            if counter == row_length {
                println!("{element:?}]");
                counter = 0;
            } else {
                if counter == 1 { print!("[") } else {}
                print!("{:?} ", element);
            }
        }
    }
}

fn gauss_elimination(matrix: &mut Vec<Vec<i32>>) -> &mut Vec<Vec<i32>> {
    let current_row = 1;
    while current_row < matrix.len() {
        sort(matrix);
        //Find pivots
        let mut pivot_space = matrix[current_row].iter().position(|&x| x != 0);

        match pivot_space {
            //Check if there is a pivot
            Some(pivot_index) => {
                let mut n = 0;
                // Iterate through n rows below and check if there are als pivots on the same space
                    while current_row + n < matrix.len() && matrix[current_row + n].iter().position(|&x| x != 0) == Some(pivot_index) {
                        n += 1;
                    }
                // Eliminate the pivot elements on the n rows below
                while n != 0 {
                    let new_row = row_add(&mut matrix[current_row].clone(), &mut matrix[current_row+n-1].clone(), pivot_space.unwrap());
                    let _ = std::mem::replace(&mut matrix[current_row + n - 1], new_row);
                    n -= 1;
                }
                return sort(matrix)
            }
            None => return sort(matrix)
        }
    }
    matrix
}

fn sort(matrix: &mut Vec<Vec<i32>>) -> &mut Vec<Vec<i32>>{
    //Sort the rows such that rows with many zeros in the front are down
    matrix.sort_by_key(|vec| count_leading_zeros(vec));

    matrix
}

fn count_leading_zeros(vec: &Vec<i32>) -> usize {
    //Function needed as an argument for the sort_by_key
    vec.iter()
        .take_while(|&&x| x==0)
        .count()
}




fn row_add(row1: &mut Vec<i32>, row2: &mut Vec<i32>, pivot_space: usize) -> Vec<i32> {
    let elimination_factor = -(row1[pivot_space] as f64 / row2[pivot_space] as f64) as i32;
    for element in 0..row1.len() {
        row1[element] += elimination_factor * row2[element];
    }
    row1.clone()
}


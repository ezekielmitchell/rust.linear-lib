use std::fs;

#[derive(Debug)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Matrix {
        let data = vec![vec![0.0; cols]; rows];
        Matrix { rows, cols, data }
    }

    pub fn from_file(path: &str) -> Matrix { // this is a constructor method for Matrix
        let content: String = fs::read_to_string(path).unwrap_or_else(|e|panic!("{e}")); // read file to string
        let mut matrix: Vec<Vec<f64>> = Vec::new(); // create a vector of vectors
        for rows in content.lines() { // iterate over lines in the file
            let mut row: Vec<f64> = Vec::new(); // create a vector for each row
            let entries: Vec<&str> = rows // split each row into a vector of strings
                .split_whitespace() // split on whitespace
                .collect(); // collect into a vector
            for ent in entries { // iterate over entries in the row
                row.push(ent.parse::<f64>().unwrap()); // parse each entry as a float and push to row vector
            }
        }
        let r = matrix.len(); // get number of rows
        let c = matrix[0].len(); // get number of columns
        Matrix {rows: r, cols: c, data: matrix } // return a new Matrix
    }
}
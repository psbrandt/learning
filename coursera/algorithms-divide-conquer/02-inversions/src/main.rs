use std::io::{BufReader, ErrorKind, Error, BufRead};
use std::fs::File;

#[derive(Debug)]
struct SortedCount {
    sorted: Vec<i64>,
    num_inversions: i64
}

fn merge_and_count_split(a: &Vec<i64>, b: &Vec<i64>) -> SortedCount {
    let mut i = 0;
    let mut j = 0;
    let mut num_inversions: i64 = 0;
    let mut sorted = Vec::<i64>::new();

    let output_size = a.len() + b.len();

    while i + j < output_size {
        if i < a.len() && j < b.len() {
            // there are remaining items in both lists
            if a.get(i) < b.get(j) {
                sorted.push(*a.get(i).unwrap());
                i = i + 1;
            } else if a.get(i) > b.get(j) {
                sorted.push(*b.get(j).unwrap());
                j = j + 1;

                num_inversions = num_inversions + (a.len() - i) as i64;
            } else {
                panic!("Repeated items in list!")
            }
        } else if i < a.len() {
            // there are remaining items only in the first list
            sorted.extend(a[i..].to_vec());
            i = a.len();
        } else {
            // there are remaining items only in the second list
            sorted.extend(b[j..].to_vec());
            j = b.len();
        }
    }

    SortedCount {
        sorted,
        num_inversions
    }
}

fn sort_and_count(v: &Vec<i64>) -> SortedCount {
    if v.len() <= 1 {
        return SortedCount {
            sorted: v.clone(),
            num_inversions: 0
        };
    }

    let mid = v.len() / 2;

    let left = sort_and_count(&v[..mid].to_vec());
    let right = sort_and_count(&v[mid..].to_vec());
    let merged = merge_and_count_split(&left.sorted, &right.sorted);

    SortedCount {
        sorted: merged.sorted,
        num_inversions: left.num_inversions + right.num_inversions + merged.num_inversions
    }
}

fn read_file(name: &str) -> Result<Vec<i64>, Error> {
    let io = File::open(name)?;

    let br = BufReader::new(io);
    let mut v = vec![];
    for line in br.lines() {
        v.push(line?
        .trim()
        .parse()
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))?);
    }

    Ok(v)
}

fn main() {
    let v = read_file("./integers.txt").unwrap();

    let s = sort_and_count(&v);

    println!("Number of inversions: {:#?}", s.num_inversions);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tiny() {
        let s = sort_and_count(&vec![1, 3, 2]);

        assert_eq!(s.num_inversions, 1);
    }

    #[test]
    fn test_from_slides() {
        let s = sort_and_count(&vec![1, 3, 5, 2, 4, 6]);

        assert_eq!(s.num_inversions, 3);
    }

    fn test_max() {
        let s = sort_and_count(&vec![6, 5, 4, 3, 2, 1]);

        assert_eq!(s.num_inversions, 15);
    }
}
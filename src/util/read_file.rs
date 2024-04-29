use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;
use std::fs::File;

pub fn read_file(path: &PathBuf) -> io::Result<Vec<String>> {
    let mut lines: Vec<String> = vec![];
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if line.len() < 1 { continue }
        lines.push(line);
    }

    Ok(lines)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_test_file() {
        let target_vec = vec![
            "01,02,03,04,05,06-07x2".to_string(),
            "10,11,12,13,14,15-16x3".to_string(),
        ];
        let mut path_buf = PathBuf::new();
        path_buf.push("./test_lottos.txt");
        let test_file_content = read_file(&path_buf).unwrap();
        assert_eq!(target_vec, test_file_content);
    }
}

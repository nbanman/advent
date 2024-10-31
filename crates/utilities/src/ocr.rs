/*pub fn ocr(letters: &[bool], width: usize, height: usize) -> String {
    let mut rotated_letters = vec!['\n'; width * (height + 1)];
    for y in 0..height {
        for x in 0..width {
            let c = if letters[y * width + x] { '#' } else { '.' };
            rotated_letters[x * (height + 1) + y] = c;
        }
    }
    let string: String = rotated_letters.into_iter().collect();
    let string = string.replace(
        "....##\n...#..\n###...\n...#..\n....##",
        "....##\n...#..\n###...\n...#..\n....##\n......\n"
    );

    "hi".to_string()
}*/
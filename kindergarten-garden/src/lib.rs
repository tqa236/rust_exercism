pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let rows: Vec<&str> = diagram.split('\n').collect();
    let first_row = rows[0];
    let second_row = rows[1];

    let students = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];

    let student_index = students.iter().position(|&s| s == student).unwrap();

    let student_plants = vec![
        first_row.chars().nth(student_index * 2).unwrap(),
        first_row.chars().nth(student_index * 2 + 1).unwrap(),
        second_row.chars().nth(student_index * 2).unwrap(),
        second_row.chars().nth(student_index * 2 + 1).unwrap(),
    ];

    student_plants
        .into_iter()
        .map(|plant| match plant {
            'G' => "grass",
            'C' => "clover",
            'R' => "radishes",
            'V' => "violets",
            _ => unreachable!(),
        })
        .collect()
}

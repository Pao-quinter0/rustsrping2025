struct Student {
    major: String,
}

fn assign_major(s: &mut Student, major: String) {
    s.major = major;
}

fn update_majors(
    mut collection: Vec<Student>,
    majors: Vec<String>,
    behavior: fn(&mut Student, String),
) -> Vec<Student> {
    for (student, major) in collection.iter_mut().zip(majors.into_iter()) {
        behavior(student, major);
    }
    collection
}

fn main() {
    let students = vec![
        Student { major: "".to_string() },
        Student { major: "".to_string() },
        Student { major: "".to_string() },
    ];
    let majors = vec![
        "Physics".to_string(),
        "Computer Science".to_string(),
        "Biology".to_string(),
    ];

    let updated_students = update_majors(students, majors, assign_major);

    for (i, student) in updated_students.iter().enumerate() {
        println!("Student {}: Major = {}", i + 1, student.major);
    }
}

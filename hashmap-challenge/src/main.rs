// # Hashmaps Challenge

// Students Gradees Tracker

// The program should:

// 1. Add grades for multiple students
// 2. Find a specific student's grade
// 3. Update a student's grade
// 4. Remove a student
// 5. Print all students and their grade

use std::collections::HashMap;

type StudentMap = HashMap<&'static str, [f32; 5]>;

fn main() {
    let mut students: StudentMap = HashMap::new();

    // Add grades for multiple students
    students.insert("Ana", [7.5, 3.2, 9.1, 5.0, 6.8]);
    students.insert("Maria", [4.5, 8.8, 6.1, 7.9, 3.0]);
    students.insert("Fernando", [9.9, 2.0, 7.3, 5.5, 8.0]);
    students.insert("Eduardo", [6.0, 4.1, 8.5, 9.2, 1.5]);
    students.insert("Theo", [7.7, 6.3, 3.5, 8.9, 2.2]);
    students.insert("Pedro", [5.1, 7.0, 4.3, 6.6, 9.4]);
    students.insert("Enzo", [8.2, 1.0, 3.8, 7.1, 5.9]);

    // find the specific student by name
    let specific_student_name: &str = "Ana";

    match students.get(specific_student_name) {
        Some(notes) => {
            println!("{} Notes: {:?}", specific_student_name, notes);
        }
        None => {
            println!("Student not found");
        }
    }

    // Update student's grade
    students.insert(specific_student_name, [10.0, 10.0, 10.0, 10.0, 10.]);

    match students.get(specific_student_name) {
        Some(notes) => {
            println!(
                "Notas de {} após modificação: {:?}",
                specific_student_name, notes
            );
        }
        None => {}
    }

    // Remove a student by name
    students.remove(specific_student_name);

    println!("List após remover {}", specific_student_name);
    println!("{:?}", &students);

    // print all students and their grades
    // compute mean
    for (name, notes) in &students {
        let mut sum: f32 = 0.0;

        for note in notes {
            sum += note;
        }

        let mean: f32 = sum / 5.0;

        println!(
            "Estudante: {:<12} | Média: {:<10.2} | Notas: {:?}",
            name, mean, notes
        );
    }
}

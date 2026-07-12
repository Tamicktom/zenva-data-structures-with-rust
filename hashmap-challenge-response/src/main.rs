use std::collections::HashMap;

fn main() {
    let mut student_grades = HashMap::new();

    student_grades.insert(String::from("Alice"), 85);
    student_grades.insert(String::from("Bob"), 90);

    line_breaker();
    print_students(&student_grades);

    line_breaker();
    update_student_grade("Bob", 100, &mut student_grades);

    line_breaker();
    find_student_grade("Alice", &student_grades);

    line_breaker();
    remove_student("Alice", &mut student_grades);

    line_breaker();
    print_students(&student_grades);
}

fn find_student_grade(student: &str, grades: &HashMap<String, i32>) -> () {
    if let Some(&grade) = grades.get(student) {
        println!("{}'s grade: {}", student, grade);
    } else {
        println!("Student not found.");
    }
}

fn update_student_grade(student: &str, new_grade: i32, grades: &mut HashMap<String, i32>) {
    println!("Updating student's grade");
    grades.insert(student.to_string(), new_grade);
}

fn remove_student(student: &str, grades: &mut HashMap<String, i32>) {
    println!("Removing student...");
    grades.remove(student);
}

fn print_students(grades: &HashMap<String, i32>) -> () {
    for (student, grade) in grades {
        println!("Student: {:?} | Grades: {:?}", student, grade);
    }
}

fn line_breaker() -> () {
    println!("--------------------");
}

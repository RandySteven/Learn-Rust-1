use std::collections::HashMap;

const APPLICATION_NAME:&str = "Rust Application";

fn make_user(name:&str, age:i32){
    println!("{name} \n{age}")
}

fn add(a:i32, b:i32) -> i32{
    return a + b;
}

enum Gender {
    MALE,
    FEMALE
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Gender::MALE => write!(f, "Male"),
            Gender::FEMALE => write!(f, "Female")
        }
    }
}


struct Student {
    name:String,
    major:String,
    age:u32,
    gender:Gender
}

fn main() {
    let name = "Randy";
    let age = 20;
    let sum = add(2, 1);
    println!("{APPLICATION_NAME}");
    println!("{sum}");
    make_user(name, age);

    let user:(&str, i32, bool) = ("Randy", 20, false);
    println!("{:?}", user);

    // let teachers = ["Test", "False", "Last"];
    // let names:[&str;2] = ["Randy", "Steven"];

    // for name in names.iter() {
    //     println!("value is : {}", name);
    // }

    // for teacher in teachers.iter(){
    //     println!("teacher is : {}", teacher)
    // }

    let mut numbers = [1, 2, 3, 4];
    update(&mut numbers);
    println!("Main : {:?}", numbers);

    let v = vec![1, 2, 3];
    let v2 = v;
    // println!("{:?}", &v);
    println!("{:?}", &v2);
    let mut students = Vec::new();

    let student1 = Student{
        name:String::from("Randy"),
        major:String::from("Information System"),
        age:23,
        gender:Gender::MALE
    };
    let student2 = Student{
        name:String::from("Kevin"),
        major:String::from("Management"),
        age: 22,
        gender:Gender::MALE
    };
    // let student3 = find_the_elder(student1, student2);
    // print_student(student3);
    students.push(student1);
    students.push(student2);
    display_students(students);

    let mut student_map:HashMap<String, Student> = HashMap::new();
    student_map.insert(String::from("Test"), Student { 
        name: String::from("User"), 
        major: String::from("Teknik Industri"), 
        age: 39, 
        gender: Gender::FEMALE }
    );
}

fn update(arr:&mut [i32;4]){
    for i in 0..4 {
        arr[i] = 0
    }
    println!("Update : {:?}", arr)
}

fn display_students(students:Vec<Student>) {
    for student in &students {
        println!("Name : {} \nMajor : {}\nAge : {}\nGender : {}", 
            student.name, 
            student.major, 
            student.age,
            student.gender.to_string()
        );
    }
}

fn print_student(student:Student){
    println!("Name : {} \nMajor : {}\nAge : {}\n", 
        student.name, 
        student.major, 
        student.age,
    );
    println!("{:?}", student.gender.to_string())
}


fn find_the_elder(student1:Student, student2:Student) -> Student {
    if student1.age > student2.age {
        return student1;
    }
    return student2;
}
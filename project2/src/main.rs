//John Zavisa
//Project 2
//Theory of programming languages
//Rust

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec::Vec;
use std::str::SplitWhitespace;
use std::io::{stdin,stdout,Write};
use std::iter::FromIterator;
use std::collections::HashMap;
use std::io;

#[derive(Default,Debug)]

//student struct
pub struct Student{
    c_number: String,
    cla: u32,
    ola: u32,
    quiz: u32,
    exam: u32,
    final_exam: u32,
    letter: String,
}

//method implementation
impl Student {

	//getters
    fn get_c_number(&self) -> &String {
        &self.c_number
    }
    fn get_cla(&self) -> u32 {
     self.cla
    }
    fn get_ola(&self) -> u32 {
        self.ola
    }
    fn get_quiz(&self) -> u32 {
        self.quiz
    }
    fn get_exam(&self) -> u32 {
        self.exam
    }
    fn get_final(&self) -> u32 {
        self.final_exam
    }

    //setter
    fn new(c_number: String, cla: u32, ola: u32, quiz: u32, exam: u32, final_exam: u32) -> Student {
        let letter = Student::final_calc(cla, ola, quiz, exam, final_exam);

        Student {
            c_number,
            cla,
            ola,
            quiz,
            exam,
            final_exam,
            letter,
        }
    }

    //letter grade method
    fn final_calc(cla: u32, ola: u32, quiz: u32, exam: u32, final_exam: u32) -> String {
        let total = cla + ola + quiz + exam + final_exam;

        let mut letter : String = " ".to_string();

        if total >= 90 {
            letter = "A".to_string();
        }else if total >= 87 && total < 90{
            letter = "B+".to_string();
        }else if total >= 83 && total < 87{
            letter = "B".to_string();
        }else if total >= 80 && total < 83{
            letter = "B-".to_string();
        }else if total >= 77 && total < 80{
            letter = "C+".to_string();
        }else if total >= 73 && total < 77{
            letter = "C".to_string();
        }else if total >= 70 && total < 73{
            letter = "C-".to_string();
        }else if total >= 67 && total < 70{
            letter = "D+".to_string();
        }else if total >= 63 && total < 67{
            letter = "D".to_string();
        }else if total >= 60 && total < 63{
            letter = "D-".to_string();
        }else if total < 60{
            letter = "F".to_string();
        }

        letter
    }  
}

//main
fn main() {

	//get user input for data file
    let mut s=String::new();
    let mut k = 0;
    print!("Please enter some file name: ");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    
    //read in file
    let mut v: Vec<String> = Vec::new();
    let f = File::open(s).expect("Unable to open file");
    let f = BufReader::new(f);
    let mut count = 0;
   
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        v.push(line);
        count += 1;
    }

    //pop off first line
    v.remove(0);

    //creat hashmap
    let mut students = HashMap::new();

    //parse by whitespace 
    for line in v{
        let mut split = line.split_whitespace();
        let student_info: Vec<String> = Vec::from_iter(line.split_whitespace().map(String::from));

        //scores from string to int
        let cNum = student_info.iter().nth(0).unwrap().clone();
        let cla_Int = match student_info[1].parse::<u32>() {Ok(cla) => cla, Err(_) => panic!("{:?}","Didnt work" ),};
        let ola_Int = match student_info[2].parse::<u32>() {Ok(cla) => cla, Err(_) => panic!("{:?}","Didnt work" ),};
        let quiz_Int = match student_info[3].parse::<u32>() {Ok(cla) => cla, Err(_) => panic!("{:?}","Didnt work" ),};
        let exam_Int = match student_info[4].parse::<u32>() {Ok(cla) => cla, Err(_) => panic!("{:?}","Didnt work" ),};
        let final_Int = match student_info[5].parse::<u32>() {Ok(cla) => cla, Err(_) => panic!("{:?}","Didnt work" ),}; 
        
        //student object create
        let student = Student::new(cNum, cla_Int, ola_Int, quiz_Int, exam_Int, final_Int);

        //insert student object into hashmap
        students.insert(student.get_c_number().clone(), student);
    }

    //two query prompts 
    for k in 0..2 {  
        println!("Enter the C# to search for.");
        let mut c_number_search = String::new();
        io::stdin().read_line(&mut c_number_search)
        .expect("Unable to read the C#");
        c_number_search = c_number_search.trim().to_string();
        match students.get(&c_number_search) {
            Some(ref student) => println!("{:?}", student),
            None => println!("Student not found")
    	}
	}
    // a variable for each score
    let mut max_cla = 0;
    let mut max_ola = 0;
    let mut max_quiz = 0;
    let mut max_exam = 0;
    let mut max_final_exam = 0;

    //print all students 
    for (c_number, student) in &students {
        println!("{:?}", student);

        // store the score so we don't have to get it twice
        let cla = student.get_cla();
        let ola = student.get_ola();
        let quiz = student.get_quiz();
        let exam = student.get_exam();
        let final_exam = student.get_final();

        //determine max scores
        if cla > max_cla {
            max_cla = cla;
        }
        if ola > max_ola {
            max_ola = ola;
        }
        if quiz > max_quiz {
            max_quiz = quiz;
        }
        if exam > max_final_exam {
            max_exam = exam;
        }
        if final_exam > max_final_exam {
            max_final_exam = final_exam;
        }   
    }
    //print max scores
    println!("The max cla score was: {}", max_cla);
    println!("The max ola score was: {}", max_ola);
    println!("The max quiz score was: {}", max_quiz);
    println!("The max exam  was: {}", max_exam);
    println!("The max final exam score was: {}", max_final_exam);
    
    let mut n = 0.0;
    let mut cla_sum = 0.0;
    let mut ola_sum = 0.0;
    let mut quiz_sum = 0.0;
    let mut exam_sum = 0.0;
    let mut final_sum = 0.0;

    //calculate sum of scores
    for (c_number, student) in &students {
        cla_sum += student.get_cla() as f64;
        ola_sum += student.get_ola() as f64;
        quiz_sum += student.get_quiz() as f64;
        exam_sum += student.get_exam() as f64;
        final_sum += student.get_final() as f64;

        n += 1 as f64;

    }

    //calculate average with sum / number of students
    let mut cla_average = cla_sum / n; 
    let mut ola_average = ola_sum / n; 
    let mut quiz_average = quiz_sum / n; 
    let mut exam_average = exam_sum / n; 
    let mut final_average = final_sum / n; 

    //print average scores
    println!("The average cla score was: {}", cla_average);
    println!("The average ola score was: {}", ola_average);
    println!("The average quiz score was: {}", quiz_average);
    println!("The average exam score was: {}", exam_average);
    println!("The average final exam score was: {}", final_average);

}


    
    



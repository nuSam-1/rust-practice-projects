// Make an application which takes various questions from a file, picked randomly, and puts
// together a quiz for students. Reads a key to grade the quizzes.

use std::fs;
use std::error::Error;
use rand::Rng;
use std::io;

const NUM_OF_QUESTIONS: u32 = 10;

fn main() {
    // Read questions from the question file.
    let mut question_file_string = String::new();
    match read_question_file() {
        Ok(file_contents) => question_file_string = file_contents,
        Err(_) => println!("Error while opening question file")
    }

    let quiz_questions = generate_quiz_questions(&question_file_string);
    let score: u32 = take_quiz(quiz_questions);
    let grade: Grade = calculate_grade(score);
    print_results(score, grade);
}

// Starts the quiz
// Asks all the questions in the quiz, and returns a score.
fn take_quiz(quiz_questions: Vec<QuizQuestion>) -> u32 {
    let mut score = 0;
    println!("QUIZ START");
   
    // Iterate over all questions in the quiz
    // Print question, and await answer from user
    for question in quiz_questions {
        println!("{}", question.question);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read_line");
                  
        // Checks user answer, and increment score if correct.
        if check_answer(input.to_string(), question) {
            score += 1;
        }
    }

    score
}

// Checks the user's answer against the correct answer.
fn check_answer(input_answer: String, question: QuizQuestion) -> bool {
    if input_answer.trim().to_lowercase() == question.answer.to_lowercase() {
        true
    } else {
        false
    }
}

// Calculates the grade using the score achieved
// Returns an enum containing the grade
fn calculate_grade(score: u32) -> Grade {
    let score_percentage = (score as f64 / NUM_OF_QUESTIONS as f64) * 100.0;
    if score_percentage >= 90.0 {
        Grade::AStar
    } else if score_percentage >= 80.0 {
        Grade::A
    } else if score_percentage >= 70.0 {
        Grade::B
    } else if score_percentage >= 60.0 {
        Grade::C
    } else if score_percentage >= 50.0 {
        Grade::D
    } else {
        Grade::F
    }
}

// Prints the users final score & grade
fn print_results(score: u32, grade: Grade) {
    println!("You scored {} out of {} on the quiz.", score, NUM_OF_QUESTIONS);
    println!("This score achieved the grade: {:?}", grade);
}

// Reads question file to a String
fn read_question_file() -> Result<String, Box<dyn Error>> {
    let question_file = "questions.txt";
    let message = fs::read_to_string(question_file)?;
    Ok(message)
}

// Parses the string of questions read from the question file into a vector of QuizQuestions
fn parse_questions_to_vector(question_file_string: &String) -> Vec<QuizQuestion> {
    let questions: Vec<&str> = question_file_string.split("\r\n").collect();
    let mut question_pool: Vec<QuizQuestion> = Vec::new();

    for i in (0..questions.len()).step_by(2) {
        let question = match questions.get(i) {
            Some(q) => q,
            None => ""
        };

        let answer = match questions.get(i+1) {
            Some(a) => a,
            None => ""
        };
       
        if question != "" && answer != "" {
            let question_and_answer = QuizQuestion {
                question: question.to_string(),
                answer: answer.to_string()
            };

            question_pool.push(question_and_answer);
        };
    }

    question_pool
}

// Selects a number of questions of the list of questions
fn generate_quiz_questions(question_file_string: &String) -> Vec<QuizQuestion> { 
    let mut quiz_questions: Vec<QuizQuestion> = Vec::new();

    // Parse string of questions into vector
    let mut question_pool: Vec<QuizQuestion> = parse_questions_to_vector(question_file_string);

    for _i in 0..NUM_OF_QUESTIONS {
        // Pick a random question from the pool
        let choice = rand::thread_rng().gen_range(0..question_pool.len()-1);
        let question = question_pool.remove(choice); // Extract & remove the question from pool
        quiz_questions.push(question); // Add question to quiz
    }

    quiz_questions
}

#[derive(Debug)]
struct QuizQuestion {
    question: String,
    answer: String
}

#[derive(Debug)]
enum Grade {
    AStar,
    A,
    B,
    C,
    D,
    F
}

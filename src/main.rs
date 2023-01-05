use std::{env, io};

use console::Term;

use crate::questions::QuestionList;

mod questions;

fn main() {
    let mut ql = QuestionList::default();

    if ql.questions.is_empty() {
        println!("I don't have any questions to ask you :(");
        println!("Please give me a .txt file containing some :)");
        println!(
            "(Note that my current directory is `{:?}`)",
            env::current_dir().unwrap(),
        );
        let stdin = io::stdin();
        let mut dir = String::new();
        stdin.read_line(&mut dir).unwrap();
        dir = dir.trim().to_string();

        ql.read_questions_from_txt(dir);

        println!();
    }

    let initial = ql.get_tot();

    let mut done = false;

    while !done {
        println!("Here's a question to answer: ");
        let mut q = ql.extract_random();
        println!("{}\n\n", q.get_text());
        println!("Commands:");
        println!("\t+: answered well");
        println!("\t-: answered badly");
        println!("\t=: answered so-so");
        println!("\tr: reset priorities");
        println!("\tq: save and quit");

        let term = Term::stdout();

        let mut valid_input = false;

        while !valid_input {
            valid_input = true;

            let command = term.read_char().unwrap();

            match command {
                '+' => {
                    q.good_answer();
                    println!("Well done!\n\n");
                }
                '-' => {
                    q.bad_answer();
                    println!("You'll get it next time.\n\n")
                }
                '=' => {
                    q.so_so_answer();
                    println!("Not bad, keep practising.\n\n")
                }
                'r' => {
                    q.reset();
                    println!("Resetting priorities...\n\n");
                    for question in ql.questions.iter_mut() {
                        question.reset();
                    }
                }
                'q' => {
                    let new_tot = ql.get_tot() + q.get_priority();
                    println!("Quitting...see you again soon!");
                    if new_tot < initial {
                        println!("Your grade improved by {} points!", initial - new_tot);
                    } else if new_tot > initial {
                        println!(
                            "Your grade worsened by {} points, but I'm sure you'll do better next time :)",
                            new_tot - initial
                        );
                    }
                    let q_len = ql.questions.len();
                    let average_score = q_len * 8; // 8 considered average score for a question
                    if new_tot < average_score {
                        println!(
                            "Your current grade is better than the expected average by {} points!",
                            average_score - new_tot
                        );
                    } else if new_tot > average_score {
                        println!(
                            "Your current grade is worse than the expected average by {} points",
                            new_tot - average_score
                        );
                    }
                    println!("\n\nPress any key to close");
                    term.read_char().unwrap();
                    done = true;
                    break;
                }
                _ => {
                    valid_input = false;
                    println!("I didn't expect that! Please try again :)\n\n");
                }
            }
        }
        ql.add_question(q); // `extract_random()` removes question from list, so have to re-add
    }
    ql.save_to_json().unwrap();
}

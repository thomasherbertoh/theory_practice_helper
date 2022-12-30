use std::{
    fs::{self, File},
    io::BufReader,
};

use rand::Rng;
use serde_derive::{Deserialize, Serialize};
use serde_json::Error;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Question {
    text: String,
    priority: usize,
}

impl Question {
    pub fn new(text: String) -> Self {
        Self { text, priority: 8 }
    }

    pub fn get_text(&self) -> String {
        self.text.clone()
    }

    pub fn good_answer(&mut self) {
        if self.priority == 1 {
            return;
        }
        self.priority /= 2; // answered well so question has lower priority
    }

    pub fn bad_answer(&mut self) {
        self.priority *= 4; // answered badly so question has higher priority
    }

    pub fn so_so_answer(&mut self) {
        self.priority *= 2; // should probably practise this one more
    }

    pub fn reset(&mut self) {
        self.priority = 8;
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuestionList {
    pub questions: Vec<Question>,
    tot: usize, // sum of priorities/effective total of questions
}

impl QuestionList {
    pub fn default() -> Self {
        QuestionList::load_from_json()
    }

    pub fn get_tot(&self) -> usize {
        self.tot
    }

    fn sort_questions(&mut self) {
        self.questions.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    fn load_from_json() -> Self {
        let file = File::open("questions.json");

        match file {
            Ok(file) => {
                let reader = BufReader::new(file);
                let mut ql: QuestionList = serde_json::from_reader(reader).unwrap_or(Self {
                    questions: Vec::new(),
                    tot: 0,
                });
                ql.sort_questions();
                ql.tot = 0;
                for q in &ql.questions {
                    ql.tot += q.priority;
                }
                ql
            }
            Err(_) => {
                println!("error");
                Self {
                    questions: Vec::new(),
                    tot: 0,
                }
            }
        }
    }

    pub fn save_to_json(&self) -> Result<(), Error> {
        let serialised = serde_json::to_string_pretty(self)?;
        std::fs::write("questions.json", serialised).unwrap();
        Ok(())
    }

    pub fn add_question(&mut self, q: Question) {
        self.tot += q.priority;
        self.questions.push(q);
        self.sort_questions();
        self.save_to_json().unwrap();
    }

    pub fn extract_random(&mut self) -> Question {
        let mut num = rand::thread_rng().gen_range(0..self.tot);
        for (i, q) in self.questions.iter().enumerate() {
            if q.priority > num {
                // picked this question
                let ret_q = q.clone();
                self.tot -= q.priority;
                self.questions.remove(i);
                return ret_q;
            } else {
                num -= q.priority;
            }
        }
        self.questions.last().unwrap().clone()
    }

    pub fn read_questions_from_txt(&mut self, dir: String) {
        let qs_file = fs::read_to_string(dir.clone())
            .unwrap_or_else(|_| panic!("unable to read the file at path `{dir}`"));
        for mut line in qs_file.lines() {
            if line.is_empty() {
                continue;
            }
            if line.chars().next().unwrap().eq(&'-') {
                line = &line[1..];
            }
            line = line.trim();
            self.add_question(Question::new(line.to_string()));
        }
        self.sort_questions();
    }
}

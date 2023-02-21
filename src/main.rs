use crate::questions::QuestionList;

mod ui;

mod questions;

fn main() {
    let app = QuestionList::default();

    let native_options = eframe::NativeOptions::default();

    eframe::run_native("Study Helper", native_options, Box::new(|_| Box::new(app)));
}

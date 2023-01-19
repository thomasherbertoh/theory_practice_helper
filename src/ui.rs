use std::env;

use egui::{
    FontFamily::Proportional,
    FontId,
    TextStyle::{Body, Button, Heading, Monospace, Small},
};

use crate::questions::QuestionList;

impl eframe::App for QuestionList {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (Heading, FontId::new(30.0, Proportional)),
            (Body, FontId::new(20.0, Proportional)),
            (Monospace, FontId::new(14.0, Proportional)),
            (Button, FontId::new(18.0, Proportional)),
            (Small, FontId::new(10.0, Proportional)),
        ]
        .into();
        ctx.set_style(style);
        ctx.set_visuals(egui::Visuals::dark());
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                if ui.button("Quit").clicked() {
                    self.save_to_json().unwrap();
                    frame.close();
                }
            })
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Study helper");

            if self.questions.is_empty() {
                ui.heading("！ I don't have any questions to ask you ！");
                ui.label("Please give me a .txt file containing some :)");
                ui.label(format!(
                    "Note that my current directory is {:?}",
                    env::current_dir().unwrap()
                ));
                ui.text_edit_singleline(&mut self.new_question);
                if ui.button("Submit file name").clicked() {
                    self.read_questions_from_txt(self.new_question.clone());
                }
            } else {
                if self.current_question.is_none() {
                    self.current_question = Some(self.extract_random());
                }
                ui.label(format!(
                    "{}",
                    self.current_question.clone().unwrap().get_text()
                ));
                if ui.button(format!("✅ Answered well")).clicked() {
                    self.current_question.as_mut().unwrap().good_answer();
                    self.add_question(self.current_question.clone().unwrap());
                    self.current_question = None;
                    self.calc_tot();
                };
                if ui.button("⊟ Answered so-so").clicked() {
                    self.current_question.as_mut().unwrap().so_so_answer();
                    self.add_question(self.current_question.clone().unwrap());
                    self.current_question = None;
                    self.calc_tot();
                };
                if ui.button("❌ Answered badly").clicked() {
                    self.current_question.as_mut().unwrap().bad_answer();
                    self.add_question(self.current_question.clone().unwrap());
                    self.current_question = None;
                    self.calc_tot();
                };
                let current_score = self.calc_tot();
                let expected_average = (self.questions.len() + 1) * 8; // + 1 because current question has been temporarily removed
                if current_score == expected_average {
                    ui.label("Your score is on par!");
                } else {
                    ui.label(format!(
                        "Your current score is {} than the expected average by {} points",
                        if current_score < expected_average {
                            "lower"
                        } else {
                            "higher"
                        },
                        if current_score < expected_average {
                            expected_average - current_score
                        } else {
                            current_score - expected_average
                        }
                    ));
                }
            }
            egui::warn_if_debug_build(ui);
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Reset priorities").clicked() {
                    for question in self.questions.iter_mut() {
                        question.reset();
                    }
                    if self.current_question.is_some() {
                        self.current_question.as_mut().unwrap().reset();
                    }
                    self.calc_tot();
                    self.save_to_json()
                        .expect("Unable to write questions to file.");
                }
                if ui.button("Quit").clicked() {
                    self.save_to_json().unwrap();
                    frame.close();
                }
            });
        });
    }
}
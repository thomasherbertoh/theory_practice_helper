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
            (Button, FontId::new(24.0, Proportional)),
            (Small, FontId::new(10.0, Proportional)),
        ]
        .into();
        ctx.set_style(style);
        ctx.set_visuals(egui::Visuals::dark());

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Study helper");

            // no questions - need to fetch them
            if self.questions.is_empty() && self.current_question.is_none() {
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

                // display question
                ui.label(self.current_question.as_ref().unwrap().get_text());

                if ui
                    .button(
                        egui::RichText::new("✅ Answered well")
                            .color(egui::Color32::from_rgb(0, 255, 0)),
                    )
                    .clicked()
                {
                    self.current_question.as_mut().unwrap().good_answer();
                    self.add_question(self.current_question.clone().unwrap());
                    self.current_question = None;
                    self.calc_tot();
                };

                if ui
                    .button(
                        egui::RichText::new("⊟ Answered so-so")
                            .color(egui::Color32::from_rgb(255, 255, 0)),
                    )
                    .clicked()
                {
                    self.current_question.as_mut().unwrap().so_so_answer();
                    self.add_question(self.current_question.clone().unwrap());
                    self.current_question = None;
                    self.calc_tot();
                };

                if ui
                    .button(
                        egui::RichText::new("❌ Answered badly")
                            .color(egui::Color32::from_rgb(255, 0, 0)),
                    )
                    .clicked()
                {
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
                        "Your current score is {} than the expected average",
                        if current_score < expected_average {
                            format!("{} points better", expected_average - current_score)
                        } else {
                            format!("{} points worse", current_score - expected_average)
                        }
                    ));
                }
                if ui.button("Remove this question").clicked() {
                    self.remove_question(&self.current_question.clone().unwrap());
                    self.current_question = None;
                    self.calc_tot();
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

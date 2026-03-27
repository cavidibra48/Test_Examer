//"This program is free software: you can redistribute it and/or modify it under the terms of
// the GNU General Public License as published by the Free Software Foundation..."

mod backend;
use eframe::{egui};
use eframe::egui::{ CentralPanel, Color32};
use egui::RichText;
use std::sync::Arc;
use egui::{FontId, FontFamily};

#[derive(PartialEq)]
enum Screen{
    Input,
    Question(usize),
    Finished,
}
#[derive(PartialEq,Clone,Copy,Debug)]
enum Variant{
    None,
    A,
    B,
    C,
    D,
    E
}

struct MyApp{
    time:String,
    count: String,
    selected_path: String,
    current_screen: Screen,
    user_selection:Vec<Variant>,
    user_selection_list:Vec<String>,
    listof30questions: Vec<Vec<String>>,
    correct_answers:Vec<Variant>,
    correct_answers_list:Vec<String>,
    just_questions:Vec<String>,
    pub final_score:i32,
    option_list:Vec<Vec<String>>,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame){
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill =egui::Color32::from_hex("#fafafa").unwrap();
        ctx.set_visuals(visuals);
        let mut style = (*ctx.style()).clone();
        style.override_text_style = Some(egui::TextStyle::Body);
        style.visuals.override_text_color = Some(egui::Color32::from_hex("#fafafa").unwrap());
        ctx.set_style(style);
        CentralPanel::default().show(ctx, |ui| {
            match self.current_screen{
                Screen::Input => {
                    ui.add_space(15.0);
                    MyApp::head(ui);
                    ui.add_space(60.0);
                    let text1:&str = "Time: 30 minutes";
                    MyApp::input_text(ui,&mut self.time,text1);
                    ui.add_space(30.0);
                    let text2:&str = "Number: 30 questions";
                    MyApp::input_text(ui,&mut self.count,text2);
                    ui.add_space(30.0);
                    MyApp::browse_input(ui,&mut self.selected_path);
                    ui.add_space(60.0);
                    let start_button =egui::Button::new(RichText::new("Start")
                        .size(40.0)
                        .color(Color32::from_hex("#fafafa").unwrap()))
                        .fill(Color32::from_hex("#082052").unwrap())
                        .corner_radius(40.0)
                        .min_size(egui::vec2(350.0, 60.0));
                    ui.vertical_centered(|ui|{
                        ui.set_width(400.0);
                        ui.set_height(40.0);
                        if ui.add(start_button).clicked() {
                            let question_list =backend::fileread(&mut self.selected_path);
                            for i in 0..30{
                            let options1 =backend::question_maker(&question_list,i);
                                let options =options1.clone();
                                self.listof30questions.push(options);
                            }
                            let mut i =0;
                            while i<31 {
                                self.current_screen = Screen::Question(i);
                                i += 1;

                            }
                        }
                    });
                }

                Screen::Question(index) => {
                    ui.add_space(15.0);
                    let mut  options:Vec<String>=Vec::new();
                    let mut questions:Vec<String> = Vec::new();
                    questions=self.listof30questions[index].clone();
                    if let Some(question) = questions.get(5).cloned(){
                        self.just_questions[index-1]=question.clone();
                        MyApp::question(ui,question);
                    }
                    else{MyApp::question(ui,"try again".to_string());}

                    options = questions[0..5].to_vec();
                    self.option_list[index-1] = options.clone();
                    //self.option_list.push(options);
                    ui.add_space(40.0);
                    if let Some(current_choice) = self.user_selection.get_mut(index-1){
                        MyApp::variant(ui,options.clone(),current_choice,&mut self.correct_answers,index,&mut self.correct_answers_list,&mut self.user_selection_list);
                    }

                    egui::Frame::new().inner_margin(10.0).show(ui,|ui|{
                        ui.horizontal(|ui|{
                            ui.add_space(ui.available_width()-220.0);
                            // next v' diger dwyme lazim
                            if index ==30{
                                let exit = MyApp::button_next_before(ui, "Exit");
                                if ui.add(exit).clicked(){
                                    self.current_screen = Screen::Input;
                                }
                                let next = MyApp::button_next_before(ui,"Next");
                                if ui.add(next).clicked(){
                                    self.current_screen = Screen::Question(index-1);
                                }

                            }
                                else if index ==1{
                                    let finish = MyApp::button_next_before(ui,"Finish");
                                    if ui.add(finish).clicked(){
                                        let mut score = 0;
                                        for i in 0..30 {
                                            if self.correct_answers[i] == self.user_selection[i] {
                                                score += 1;
                                            }
                                            let current_question_options = &self.option_list[i];
                                            let mut only_variants:Vec<String> = current_question_options.clone();
                                            // if current_question_options.len()==5 {/
                                            //     only_variants = current_question_options[0..5].to_vec();
                                            // }
                                            MyApp::find_user_option(&mut self.user_selection, &mut self.user_selection_list, &only_variants, i);
                                        }
                                        self.final_score = score;
                                        self.current_screen = Screen::Finished;
                                    }
                                }
                            else {
                                let next = MyApp::button_next_before(ui,"Before");
                                if ui.add(next).clicked(){
                                    self.current_screen = Screen::Question(index+1);
                                }

                                let before = MyApp::button_next_before(ui,"Next");
                                if ui.add(before).clicked(){
                                    self.current_screen = Screen::Question(index-1);
                                }
                            }
                        });
                    });
                }

                Screen::Finished => {
                    MyApp::result(ui,self.final_score);
                    MyApp::show_last_result(ui,&mut self.just_questions,&mut self.correct_answers_list,&mut self.user_selection_list)

                }
            }
        }
        );
    }
}


fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions{
        viewport: eframe::egui::ViewportBuilder::default()
            .with_resizable(true)
            // .with_inner_size([320.0, 240.0])
            .with_maximized(true),
        ..Default::default()
    };
    eframe::run_native("Test Examer",
                       options,
                       Box::new(|cc|{
                           setup_custom_fonts(&cc.egui_ctx);

                           Ok(Box::<MyApp>::default())
                       }))
}
fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    let font_data = egui::FontData::from_static(include_bytes!("../assets/fonts/Qdbettercomicsans-jEEeG.ttf"));
    fonts.font_data.insert("my_comic_sans".to_owned(),
    Arc::new(font_data));
    fonts.families
    .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "my_comic_sans".to_owned());
    ctx.set_fonts(fonts);
}
impl Default for MyApp {
    fn default() -> Self {
        Self{
            time:"".to_owned(),
            count:"".to_owned(),
            selected_path:"".to_owned(),
            current_screen: Screen::Input,
            listof30questions: vec![vec![]],
            user_selection:vec![Variant::None;30],
            correct_answers:vec![Variant::None;30],
            just_questions:vec![String::new();30],
            user_selection_list:vec![String::new();30],
            correct_answers_list:vec![String::new();30],
            final_score:0,
            option_list:vec![vec![String::new();5];30]
        }
    }
}

impl MyApp {
    pub fn input_text(ui: &mut egui::Ui, label: &mut String, text1: &str) {
        ui.vertical_centered(|ui|{
            egui::Frame::new()
                .fill(Color32::from_hex("#082052").unwrap())
                .inner_margin(10.0)
                .show(ui, |ui| {
                    ui.set_width(350.0);
                    ui.set_height(40.0);
                    ui.horizontal(|ui|{
                        ui.label(RichText::new(text1).size(40.0));

                    });

                }).inner
        });
    }
    pub fn browse(text:&mut String) {
        let files = rfd::FileDialog::new()
            .add_filter("PDF", &["pdf"])
            .set_directory("/")
            .pick_file();
        if let Some(files) = files {
            *text = files.display().to_string()

        }
    }
    pub fn browse_input( ui:&mut egui::Ui, selected_path:&mut String) {
        ui.vertical_centered(|ui| {
            egui::Frame::new()
                .fill(Color32::from_hex("#082052").unwrap())
                .inner_margin(10.0)
                .show(ui, |ui| {
                    ui.set_width(350.0);
                    ui.set_height(40.0);

                    ui.horizontal(|ui| {
                        ui.set_width(350.0);
                        ui.set_height(50.0);
                        ui.label(RichText::new("File:").size(20.0));
                        ui.add(egui::TextEdit::singleline(selected_path)
                            .text_color(Color32::from_hex("#fafafa").unwrap())
                            .background_color(Color32::from_hex("#082052").unwrap())
                            .desired_width(240.0)
                            .font(FontId::new(20.0, FontFamily::Proportional))
                        );
                        let button = egui::Button::new(RichText::new("Browse")
                            .color(Color32::from_hex("#082052").unwrap())
                        ).fill(Color32::from_hex("#fafafa").unwrap());
                        if ui.add(button).clicked() {
                            Self::browse(selected_path)
                        }
                    });
                })
        });
    }
    pub fn head(ui:&mut egui::Ui) {
            ui.vertical_centered(|ui| {
                egui::Frame::new()
                    .fill(Color32::from_hex("#082052").unwrap())
                    .corner_radius(40.0)
                    .inner_margin(15.0)
                    .show(ui, |ui|{
                        ui.set_width(400.0);
                        ui.set_height(30.0);
                        ui.vertical_centered(|ui| {
                            ui.label(RichText::new("Test Exam")
                                .size(54.0)
                            )
                        })
                    }) });
        }
        pub fn question(ui:&mut egui::Ui,question: String) {
            ui.vertical_centered(|ui|{
                egui::Frame::new()
                .fill(Color32::from_hex("#efede3").unwrap())
                    .show(ui, |ui|{
                        ui.set_width(ui.available_width());
                        ui.set_height(25.0);
                        ui.label(RichText::new(question).size(35.0).color(Color32::from_hex("#302f2c").unwrap()));

                    })
            });

        }
    pub fn variant(ui:&mut egui::Ui, options: Vec<String>, selected:&mut Variant, correct_answers:&mut Vec<Variant>,index:usize, correct_answers_list:&mut Vec<String>,user_selection_list:&mut Vec<String>) {
        let variants = [Variant::A, Variant::B, Variant::C,Variant::D,Variant::E];
        ui.vertical_centered(|ui|{
            egui::Frame::new().fill(Color32::from_hex("#efede3").unwrap()).show(ui, |ui|{
                ui.set_width(ui.available_width()-50.0);
                ui.set_height(30.0);
                ui.vertical(|ui|{

                    for (option,variant) in options.iter().zip(variants){
                        let mut option = option.clone();
                        if option.starts_with("@"){
                            if let Some(correct) = correct_answers.get_mut(index-1){
                                *correct = variant;
                            }
                            let seperate = option.split_whitespace().collect::<Vec<&str>>();
                            option = seperate[1..seperate.len()].join(" ");
                           // correct_answers_list.push(option.clone());
                            correct_answers_list[index-1] = option.clone();
                        }
                        ui.add_space(10.0);
                        ui.radio_value(selected, variant, RichText::new(option.clone()).color(Color32::from_hex("#302f2c").unwrap()).size(30.0));
                        ui.separator();
                    }
                }); }); });
    }


    pub fn button_next_before<'a> (ui:&mut egui::Ui, text:&'a str) -> egui::Button<'a> {

        let next_button = egui::Button::new(RichText::new(text).color(Color32::from_hex("#fafafa").unwrap()).size(30.0))
            .fill(Color32::from_hex("#082052").unwrap())
            .corner_radius(30.0);
        next_button


    }
    pub fn result(ui:&mut egui::Ui,result:i32) {
        let res =result.to_string();
        let mut result_text = "Result: ".to_string() ;
        result_text.push_str(&res);
        ui.vertical_centered(|ui|{
            egui::Frame::new().fill(Color32::from_hex("#082052").unwrap()).show(ui, |ui|{
                ui.label(RichText::new(result_text).size(55.0));

            });
        });

    }
    pub fn find_user_option(user_selection: &Vec<Variant>, user_selection_list: &mut Vec<String>, options: &Vec<String>, i: usize) {

        let user_variant = user_selection[i];
        let user_selection_text = if let Some(idx) = user_variant.to_index() {
            options.get(idx)
                .cloned()
                .unwrap_or_else(|| "Variant did not find".to_string())
        } else {
            "Not answered".to_string()
        };

        if let Some(slot) = user_selection_list.get_mut(i) {
            *slot = user_selection_text;
        }
    }
    pub fn show_last_result(ui:&mut egui::Ui, just_questions:&mut Vec<String>, correct_answers_list:&mut Vec<String>, user_selection_list:&mut Vec<String>) {
       ui.vertical_centered(|ui|{
           egui::ScrollArea::vertical()
               .max_height(ui.available_height())
               .max_width(ui.available_width())
               .show(ui, |ui|{
                   let mut i:usize =30;
                   while i>0 {
                       i-=1;
                       let question = just_questions[i].clone();
                       let mut answer = user_selection_list[i].clone();
                       let mut correct = correct_answers_list[i].clone();
                       correct =format!("Correct answer: {}",correct);
                       ui.scope(|ui|{

                           if answer.starts_with("@"){
                               let seperate = answer.split_whitespace().collect::<Vec<&str>>();
                               answer = seperate[1..seperate.len()].join(" ");
                               answer =format!("Your answer: {}",answer);

                               egui::Frame::new().fill(Color32::from_hex("#06660a").unwrap()).show(ui,|ui|{
                                   ui.label(RichText::new(question).size(20.0).color(Color32::from_hex("#fafafa").unwrap()));
                                   ui.label(RichText::new(correct).size(15.0).color(Color32::from_hex("#fafafa").unwrap()));
                                   ui.label(RichText::new(answer).size(15.0).color(Color32::from_hex("#fafafa").unwrap()));

                               });
                           }
                           else{
                               egui::Frame::new().fill(Color32::from_hex("#a60505").unwrap()).show(ui, |ui|{
                                   ui.label(RichText::new(question).size(20.0).color(Color32::from_hex("#fafafa").unwrap()));
                                   ui.label(RichText::new(correct).size(15.0).color(Color32::from_hex("#fafafa").unwrap()));
                                   ui.label(RichText::new(answer).size(15.0).color(Color32::from_hex("#fafafa").unwrap()));

                               });
                           }
                           ui.separator();
                       });
                   }
               });
       });
    }
}


impl Variant {
    pub fn to_index(&self) -> Option<usize> {
        match self {
            Variant::A => Some(0),
            Variant::B => Some(1),
            Variant::C => Some(2),
            Variant::D => Some(3),
            Variant::E => Some(4),
            Variant::None => None,
        }
    }
}
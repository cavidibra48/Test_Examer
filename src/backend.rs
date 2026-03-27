use std::path::PathBuf;
use pdf_extract;
use pdf_extract::extract_text;
use rand::seq::SliceRandom;
use std::io;
// struct App{
//     selected_path:String,
//     question_list:Vec<String>,
// }


// fn main(){
//     let mut app=App{selected_path:"".to_string()
//     , question_list:Vec::new()};
//
// }

pub fn check_answer(answer:&mut String,correct:&mut String) {
    for i in 0..31{
        println!("Give answer: ");
        io::stdin().read_line(answer).expect("Failed to read answer");
        let answer:String = answer.trim().parse().expect("Please type a number!");
        if correct.to_string() == answer {
            println!("Correct!");
        }
        else {
            println!("Wrong");
            println!("{}", correct);
        }
    }
}

pub fn fileread(file_path:&str) -> Vec<String> {
    let file = PathBuf::from(file_path);
    let text_file = extract_text(&file).unwrap().to_string();
    let list_text0 = text_file.split("\n").collect::<Vec<&str>>();
    let mut list_text:Vec<String> = Vec::new();
    for i in list_text0 {
        if i.len() == 0{
            continue;
        }
        else{
            list_text.push(i.to_string());
        }
    }
    let mut questions_list:Vec<String> = Vec::new();
    let mut rng = rand::rng();
    for sent in 0..list_text.len() {
        let mut question = String::new();
        let sentence = list_text[sent].trim();
        if let Some(first_char) = sentence.chars().nth(0){
            if first_char.is_numeric() {
                if sentence.ends_with("?") ||sentence.ends_with("."){
                    for i in sent .. sent+6{
                        question += &list_text[i];
                        question += "\n";
                    }
                    questions_list.push(question);
                }
            }
        }; }
    questions_list.shuffle(&mut rng);
   // println!("{:?}", questions_list);
    questions_list


}


pub fn question_maker(questions_list:&Vec<String>,i:usize) -> Vec<String> {
    let mut rng = rand::rng();
    let sentences = questions_list[i].split("\n").collect::<Vec<&str>>();
    let mut options:Vec<String> = Vec::new();
    let question_sentence = sentences[0];
    let seperate_question = question_sentence.split_whitespace().collect::<Vec<&str>>();
    let seperated_question_sentence = seperate_question[1..seperate_question.len()].join(" ");

    for t in 1..sentences.len()  {
        let sente =sentences[t].trim();
        if sente.ends_with("✓") || sente.starts_with("√"){
            let mut correct_answer:String = String::new();
            if sente.ends_with("✓"){correct_answer = sente.chars().take(sente.len()-3).collect::<String>();}
            else{ let seperate = sente.split_whitespace().collect::<Vec<&str>>();
                correct_answer = seperate[1..seperate.len()].join(" ");}
            let key:&str = "@";
            correct_answer = format!("{} {}", key, correct_answer);
            options.push(correct_answer);


        }
        else if sente.len() !=0 {
            let seperate = sente.split_whitespace().collect::<Vec<&str>>();
            let seperated_sentence = seperate[1..seperate.len()].join(" ");
            options.push(seperated_sentence);
        }
    }
    options.shuffle(&mut rng);
    options.push(seperated_question_sentence);
    options



}

pub fn correct_question_maker(options:Vec<String>,seperated_question_sentence:&str){
    let mut correct:String = "".to_string();
    let mut paragrahp:String = String::new();
    paragrahp += &seperated_question_sentence;
    paragrahp += "\n";
    for (option, i)  in options.iter().zip(["A","B","C","D","E"])  {
//o biri terefe bu scripti atilacaq
        if option.starts_with("@"){
            //let correct:&str = i; correct düzgün cavabı özündə saxlayır
            correct = i.to_string();
            let seperate = option.split_whitespace().collect::<Vec<&str>>();
            let option = seperate[1..seperate.len()].join(" ");
            // println!("{}",option);
            paragrahp = paragrahp +"\t" + i +") ";
            paragrahp += &option;
            paragrahp += "\n";
        }
        else {
            paragrahp = paragrahp +"\t" + i +") ";
            paragrahp += &option;
            paragrahp += "\n";
        }
    }
    println!("{}", paragrahp);
}

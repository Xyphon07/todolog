use eframe::egui;
use std::io::Seek;
use std::fs;
use std::fmt;
use egui::{RichText, Color32};
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use::std::io::{self,Read};
use std::io::Write;
use std::fmt::Display;
use eframe::App;
fn main() ->  io::Result<()>  {
    let native_options = eframe::NativeOptions::default();
   let _= eframe::run_native(
        "TODOLOG",
        native_options,
        Box::new(|cc| Ok(Box::new(todo_app::new(cc)))),
    );
     Ok(())
}

struct todo_app {
    inputStream: String,
    file: File,
}
impl Default for todo_app {
    fn default() -> Self {
        Self {
            inputStream: String::new(),
            file:OpenOptions::new()
    .read(true)
    .write(true)
    .append(true)
    .create(true)
    .open("todo.txt")
    .expect("could not open file"),           
        }
    }
}
impl todo_app {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

}

impl eframe::App for todo_app {
fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("LIST OF TODOS:");
               let mut  response=BufWriter::new(& mut self.file); 
               let text_response= ui.add(egui::TextEdit::singleline(&mut self.inputStream).hint_text("Enter todo here"));
                if ui.button("ADD").clicked() {
                   writeln!(response,"{}",self.inputStream).expect("could not write the file");
                 response.flush().expect("could not read the file");
                 self.inputStream.clear();
                }

                        });

let mut  some_string:Vec<String>= Vec::new();
             ui.vertical_centered(|ui| {
                   let _ = self.file.rewind();
        let RESPONSE_READ=BufReader::new(& mut self.file);     
 for line in RESPONSE_READ.lines() {
                
       some_string.push(line.unwrap());
 }
        
    for val in some_string { 
         let check= ui.label(&val);
         if check.clicked(){
     
remove_blank_lines_inplace("todo.txt",val);
       }
    }

 });

        
        });
    }

            
}

fn remove_blank_lines_inplace(file_path: &str, val: String) -> io::Result<()> {
    let contents = fs::read_to_string(file_path)?;
    let mut modified_lines = Vec::new();
    for line in contents.lines() {
        if line == val {
            continue;
        }
        if !line.trim().is_empty() {
            modified_lines.push(line);
        }
    }
    
    let new_contents = modified_lines.join("\n");
    
    fs::write(file_path, new_contents)?;
    
    Ok(())
}


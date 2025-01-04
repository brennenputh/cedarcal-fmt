use std::io::prelude::*;
use std::{
    fs::{read_to_string, File},
    process::ExitCode,
};

use clap::Parser;
use icalendar::{Calendar, CalendarComponent, Component};

#[derive(Parser, Debug)]
struct Cli {
    input_file: String,

    /// Output file.  Default: ./output.ics
    #[arg(short, long)]
    output_file: Option<String>,
}

fn translate_building(original: &str) -> Option<&str> {
    match original {
        "Engineering and Science Ctr" => Some("ENS"),
        "Health Sciences Center" => Some("HSC"),
        "Ctr for Bib and Theo Studies" => Some("BTS"),
        "Milner" => Some("MIL"),
        _ => {
            eprintln!("Unable to find acronym for building {original}.");
            None
        }
    }
}

fn translate_description(original: &str) -> Option<String> {
    let information = original.split_once(",").unwrap().1.trim();
    let information_split = information.split_once(",").unwrap();

    let class_type: String = information_split
        .0
        .chars()
        .filter(|c| *c != '[' || *c != ']')
        .collect();
    let professor = information_split.1.replace("taught by", "");
    let professor = professor.trim();

    Some(format!(
        "Class Type: {} | Professor: {}",
        class_type, professor
    ))
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let input_filename = cli.input_file;
    let output_filename = match cli.output_file {
        Some(f) => f,
        None => "./output.ics".to_string(),
    };

    let input_contents = match read_to_string(input_filename) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Could not access file: {e}");
            return ExitCode::FAILURE;
        }
    };

    let Ok(input_calendar): Result<Calendar, _> = input_contents.parse() else {
            eprintln!("Could not parse ICS file.");
            return ExitCode::FAILURE;
    };
    let mut output_calendar = Calendar::new();

    for component in &input_calendar.components {
        if let CalendarComponent::Event(event) = component {
            let mut new_event = event.clone();

            // Put the relevant course name at the beginning of the name of the course.
            let summary_pieces = event.get_summary().unwrap().split_once(" ").unwrap();
            new_event.summary(format!("{} {}", summary_pieces.1, summary_pieces.0).as_str());

            let og_building_pieces = event
                .property_value("LOCATION")
                .unwrap()
                .split_once(",")
                .unwrap();

            let building = match translate_building(og_building_pieces.0) {
                Some(b) => b,
                None => og_building_pieces.0,
            };
            let new_room = og_building_pieces.1.replace("room", "");
            let new_room = new_room.trim();
            new_event.add_property("LOCATION", format!("{} {}", building, new_room));

            if let Some(description) = event.get_description() {
                new_event.description(translate_description(description).unwrap().as_str());
            }

            output_calendar.push(new_event);
        }
    }

    let mut output_file = File::create(output_filename).unwrap();
    write!(output_file, "{}", output_calendar).unwrap();

    ExitCode::SUCCESS
}

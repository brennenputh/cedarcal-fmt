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

    /// Output file, defaults to ./output.ics
    #[arg(short, long)]
    output_file: Option<String>,
}

fn translate_building(original: &str) -> Option<&str> {
    match original {
        "Engineering and Science Ctr" => Some("ENS"),
        "Health Sciences Center" => Some("HSC"),
        "Ctr for Bib and Theo Studies" => Some("BTS"),
        "Milner" => Some("MIL"),
        "Scharnberg Bus and Comm Center" => Some("SBCC"),
        "Callan Athletic Center" => Some("CAL"),
        "Tyler Digital Comm Center" => Some("TYL"),
        "Apple Technology Resource Ctr" => Some("APP"),
        _ => {
            eprintln!("Unable to find acronym for building {original}.");
            None
        }
    }
}

// All errors are parsing errors, I'm just too lazy to make a proper error type
fn translate_description(original: &str) -> Result<String, ()> {
    let Some(information) = original.split_once(',') else {
        return Err(());
    };
    let information = information.1.trim();
    let Some(information_split) = information.split_once(',') else {
        return Err(());
    };

    let class_type = &information_split.0.trim()[1..information_split.0.len() - 1];

    let csv_profs = information_split.1.replace("taught by", "");
    let csv_profs = csv_profs.trim();
    let professor_split: Vec<_> = csv_profs.split(", ").collect();

    let mut professors = Vec::new();

    for i in 0..professor_split.len() {
        if i % 2 != 1 {
            continue;
        }
        let first = professor_split
            .get(i)
            .expect("Failed to parse professor's first name.");
        let second = professor_split
            .get(i - 1)
            .expect("Failed to parse professor's last name.");

        professors.push(format!("{first} {second}"));
    }

    let professor_string = professors.join(", ");

    Ok(format!(
        "Class Type: {class_type} | Professor(s): {professor_string}"
    ))
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let input_filename = cli.input_file;
    let output_filename = match cli.output_file {
        Some(f) => f,
        None => String::from("./output.ics"),
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
            if let Some(summary) = event.get_summary() {
                if let Some((class_number, class_name)) = summary.split_once(' ') {
                    new_event.summary(format!("{class_name} {class_number}").as_str());
                } else {
                    eprintln!("Failed to parse summary data - reusing original.");
                }
            }

            if let Some(location) = event.property_value("LOCATION") {
                if let Some((og_building, og_room)) = location.split_once(',') {
                    let building = match translate_building(og_building) {
                        Some(b) => b,
                        None => {
                            eprintln!("Failed to translate building name - reusing original.");
                            og_building
                        }
                    };
                    let new_room = og_room.replace("room", "");
                    let new_room = new_room.trim();
                    new_event.add_property("LOCATION", format!("{building} {new_room}"));
                } else {
                    eprintln!("Failed to parse location data - reusing original.");
                }
            }

            if let Some(description) = event.get_description() {
                if let Ok(new_description) = translate_description(description) {
                    new_event.description(&new_description);
                } else {
                    eprintln!("Failed to parse description - reusing original.");
                }
            }

            output_calendar.push(new_event);
        }
    }

    let Ok(mut output_file) = File::create(output_filename) else {
        eprintln!("Failed to open output file.");
        return ExitCode::FAILURE;
    };
    if let Err(e) = write!(output_file, "{output_calendar}") {
        eprintln!("Could not write to output file: {e}");
        return ExitCode::FAILURE;
    };

    ExitCode::SUCCESS
}

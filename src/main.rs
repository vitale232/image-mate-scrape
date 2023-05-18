use std::{env, fs::File, io::Read};

use csv::Writer;
use scraper::{Html, Selector};

fn main() {
    let pages_dir = env::current_dir()
        .expect("Could not retrieve the current dir!")
        .join("pages");

    let mut writer =
        Writer::from_path(pages_dir.join("pages_out.csv")).expect("Could not open CsvWriter!");
    let table_selector = Selector::parse("#tblComps").unwrap();
    let tr_selector = Selector::parse("tr").unwrap();
    let td_selector = Selector::parse("td").unwrap();

    for i in 1..=9 {
        let file_name = format!("page_{}.html", i);
        let file_path = pages_dir.join(file_name);
        println!("{:#?}", file_path);
        let mut file = File::open(file_path).expect("Could not open filepath");

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let document = Html::parse_document(&contents);

        if let Some(table_el) = document.select(&table_selector).next() {
            let mut rows: Vec<Vec<String>> = Vec::new();

            for tr_el in table_el.select(&tr_selector) {
                let mut row: Vec<String> = Vec::new();
                for (i, td_el) in tr_el.select(&td_selector).enumerate() {
                    if i != 0 {
                        let cell_text = td_el
                            .text()
                            .collect::<Vec<_>>()
                            .join(" ")
                            .trim()
                            .replace("\n", "")
                            .replace("                                                ", " ")
                            .to_string();
                        if i == 5 {
                            let mut clean_cell_text = String::new();
                            for char in cell_text.chars() {
                                if char == '\u{a0}' || char == '\u{c2}'{
                                    continue;
                                } else {
                                    clean_cell_text.push(char);
                                }
                            }
                            row.push(clean_cell_text);
                        } else {
                            row.push(cell_text);
                        }
                    }
                }
                if !row.is_empty() {
                    rows.push(row);
                } else {
                    println!("Empty row for page {}", i);
                }
            }

            for row in rows {
                writer.write_record(&row).expect("Could not write row!");
            }
            writer.flush().expect("Could not flush writes!");
        } else {
            panic!("No table on iteration {}!", i);
        }
    }
}

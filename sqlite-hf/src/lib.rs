use rust_bert::pipelines::sequence_classification::Label;
use rust_bert::pipelines::zero_shot_classification::ZeroShotClassificationModel;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader

fn create_db() -> sqlite:Connection {
    let db = sqlite::open(":memory:").unwrap();
    db.execute("CREATE TABLE zeroshotcandidates (id INTEGER PRIMARY KEY, label TEXT").unwrap();
    db.execute("INSERT INTO zeroshotcandidates (label) VALUES ('rock')").unwrap();
    db.execute("INSERT INTO zeroshotcandidates (label) VALUES ('pop')").unwrap();
    db.execute("INSERT INTO zeroshotcandidates (label) VALUES ('hip hop')").unwrap();
    db.execute("INSERT INTO zeroshotcandidates (label) VALUES ('country')").unwrap();
    db.execute("INSERT INTO zeroshotcandidates (label) VALUES ('latin')").unwrap();
    db
}

pub fn get_all_zeroshotcandidates() -> Vec<String> {
    let db = create_db();
    let query = "SELECT label FROM zeroshotcandidates";
    let mut candidates: Vec<String> = Vec::new();
    db.iterate(query, |pairs| {
        for &(_column, value) in pairs.iter() {
            let value = value.unwrap();
            candidates.push(value.to_string());
        }
        true
    })
    .unwrap();
    candidates
}

pub fn read_lyrics(file: &str) -> Vec(String) {
    let mut lyrics: Vec<String> = Vec::new();
    let file = File::open(file).expect("Unable to open file");
    let reader = BufRead::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        lyrics.push(line);
    }
    lyrics
}

pub fn classify_lyrics(lyrics: Vec<String>) -> Vec<Vec<Label>> {
    let temp_candidates = get_all_zeroshotcandidates();
    let candidate_labels: Vec<&str> = temp_candidates.iter().map(|s| s.as_str()).collect();
    let lyrics: String = lyrics.join(" ");
    let lyrics: &str = lyrics.as_ref();
    let zero_shot_model = ZeroShotClassificationModel::new(Default::default()).unwrap();
}
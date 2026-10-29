use rorschach_rs::questionnaire_presets::QuestionnairePresets;
use rorschach_rs::questionnaire_response::{Answer, QuestionnaireResponse};
use rorschach_rs::traits::ToCsv;
use chrono::{DateTime, NaiveDateTime, Utc};

fn parse_ts(s: &str) -> Option<DateTime<Utc>> {
    NaiveDateTime::parse_from_str(s.trim(), "%Y-%m-%dT%H:%M:%S")
        .ok()
        .map(|ndt| ndt.and_utc())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hamd = QuestionnairePresets::HAMD21.build();

    // 21 item columns; for item 18 the severity column hamd_18b is used
    let item_cols: Vec<String> = (1..=21)
        .map(|i| if i == 18 { "hamd_18b".to_string() } else { format!("hamd_{i}") })
        .collect();
    //open csv file with patient data
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .from_path("data/hamd_patients.csv")?;
    //looking for headers in the csv file, and get the index of the columns we need
    let headers = rdr.headers()?.clone();
    let idx = |name: &str| headers.iter().position(|h| h == name).expect(name);

    let i_id = idx("record_id");
    let i_ts = idx("hamd_timestamp");
    let i_items: Vec<usize> = item_cols.iter().map(|c| idx(c)).collect();

    //go through each row
    let mut results = Vec::new();
    for rec in rdr.records() {
        let rec = rec?;
        let answers: Vec<Answer> = i_items
            .iter()
            .enumerate()
            .map(|(item, &col)| {
                Answer::new(item, rec[col].trim().parse::<f32>().ok())
            })
            .collect();

        let id = rec[i_id].trim();
        
        let taken_at = parse_ts(&rec[i_ts]);
        match hamd.evaluate(id, QuestionnaireResponse::new(answers, taken_at)) {
            Ok(r) => results.push(r),
            Err(e) => eprintln!("skipping {id} ({}): {e}", &rec[i_ts]),
        }
    }

    //write file
    let mut out = std::fs::File::create("out/hamd_hpo.csv")?;
    results.to_csv(&mut out, true)?;
    Ok(())
}

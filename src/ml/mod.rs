use anyhow::Result;
use ndarray::{Array1, Array2};
use serde_json;
use std::{collections::HashMap, fs::File};
use linfa_preprocessing::tf_idf_vectorization::FittedTfIdfVectorizer;

pub fn load_and_predict(text: &str) -> Result<String> {
    let vecf = File::open("model/tfidf.json")?;
    let fitted_vec: FittedTfIdfVectorizer = serde_json::from_reader(vecf)?;

    let weights:  Array2<f64> = serde_json::from_reader(File::open("model/weights.json")?)?;
    let intercept: Array1<f64> = serde_json::from_reader(File::open("model/intercept.json")?)?;

    let mapf: File = File::open("model/labels.json")?;
    let label_to_id: HashMap<String, usize> = serde_json::from_reader(mapf)?;
    let mut id_to_label = vec![String::new(); label_to_id.len()];
    for (lbl, &i) in &label_to_id {
        id_to_label[i] = lbl.clone();
    }

    let single = Array1::from(vec![text.to_string()]);
    let sparse = fitted_vec.transform(&single);
    let dense2d = sparse.to_dense();             // shape (1, n_features)
    let features: Array1<f64> = dense2d.row(0).to_owned();

    let mut scores: Array1<f64> = features.dot(&weights);
    scores += &intercept;

    let class_id = scores
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .unwrap()
        .0;

    let prediction = id_to_label[class_id].clone();
    Ok(prediction)
}

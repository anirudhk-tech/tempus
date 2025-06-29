use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

use ndarray::{Array1, Array2};
use linfa::DatasetBase;
use linfa::dataset::Records;
use linfa::traits::Fit;
use linfa_logistic::MultiLogisticRegression;
use linfa_preprocessing::tf_idf_vectorization::TfIdfVectorizer;

#[derive(Serialize, Deserialize)]
struct Example {
    text: String,
    label: String,
}

pub fn train_model() -> Result<()> {
    // ----- load JSON ---------------------------------------------------------
    let raw = fs::read_to_string("ml_data/training_data.json")?;
    let examples: Vec<Example> = serde_json::from_str(&raw)?;

    // split into texts / labels
    let texts:  Vec<String> = examples.iter().map(|e| e.text.clone()).collect();
    let labels: Vec<String> = examples.iter().map(|e| e.label.clone()).collect();

    // ----- TF–IDF ------------------------------------------------------------
    let text_array = ndarray::Array1::from(texts.clone());
    let vectorizer = TfIdfVectorizer::default();
    let fitted_vec = vectorizer.fit(&text_array)?;
    let sparse     = fitted_vec.transform(&text_array);   // CsMat<f64>

    // make it dense for linfa-logistic
    let dense: Array2<f64> = sparse.to_dense();

    // ----- encode labels -----------------------------------------------------
    let mut label_to_id: HashMap<String, usize> = HashMap::new();
    let numeric: Vec<usize> = labels
        .into_iter()
        .map(|lbl| {
            let id = label_to_id.len();
            *label_to_id.entry(lbl).or_insert(id)
        })
        .collect();
    let label_array: Array1<usize> = Array1::from(numeric);

    // ----- dataset & training -----------------------------------------------
    let dataset = DatasetBase::new(dense, label_array);

    println!(
        "dataset has {} features and {} samples. training...",
        dataset.nfeatures(),
        dataset.nsamples()
    );

    let model   = MultiLogisticRegression::default().fit(&dataset)?;

    println!(
        "trained on {} examples across {} classes",
        dataset.nsamples(),
        label_to_id.len()
    );

    Ok(())
}

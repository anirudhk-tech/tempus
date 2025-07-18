use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File};

use ndarray::{Array1, Array2};
use linfa::DatasetBase;
use linfa::traits::Fit;
use linfa_logistic::{MultiLogisticRegression, MultiFittedLogisticRegression};
use linfa_preprocessing::tf_idf_vectorization::TfIdfVectorizer;
use linfa::dataset::Records;

#[derive(Serialize, Deserialize)]
struct Example {
    text: String,
    label: String,
}

pub fn _train_model() -> Result<()> {
    let raw = std::fs::read_to_string("ml_data/training_data.json")?;
    let examples: Vec<Example> = serde_json::from_str(&raw)?;
    let texts:  Vec<String> = examples.iter().map(|e| e.text.clone()).collect();
    let labels: Vec<String> = examples.iter().map(|e| e.label.clone()).collect();

    let text_arr = ndarray::Array1::from(texts.clone());
    let vectorizer = TfIdfVectorizer::default();
    let fitted_vec = vectorizer.fit(&text_arr)?;
    let sparse     = fitted_vec.transform(&text_arr);      

    let dense: Array2<f64> = sparse.to_dense();

    let mut label_to_id = HashMap::new();
    let numeric: Vec<usize> = labels.into_iter()
        .map(|lbl| {
            let next = label_to_id.len();
            *label_to_id.entry(lbl).or_insert(next)
        })
        .collect();
    let label_arr = Array1::from(numeric);

    let dataset = DatasetBase::new(dense.clone(), label_arr);
    println!("training on {} samples…", dataset.nsamples());
    
    let fitted: MultiFittedLogisticRegression<f64, _> = MultiLogisticRegression::default().fit(&dataset)?;
    let weights: Array2<f64>   = fitted.params().clone();
    let intercept: Array1<f64> = fitted.intercept().clone(); 

    serde_json::to_writer(File::create("model/tfidf.json")?, &fitted_vec)?;
    serde_json::to_writer(File::create("model/weights.json")?, &weights)?;
    serde_json::to_writer(File::create("model/intercept.json")?, &intercept)?;
    serde_json::to_writer(File::create("model/labels.json")?, &label_to_id)?;

    println!("done—saved TF–IDF, weights, intercept, and {} labels", label_to_id.len());
    Ok(())
}

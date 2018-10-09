use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufRead;

//TODO: Clean strings from some chars  
//TODO: Stems words!
//TODO: Refactor code
//TODO: Data Augmentation 
//TODO: Add different data sets 
//TODO: Add documentation
//TODO: Add means for cross validation
//TODO: Make smoothing hyperparameter, rework smoothing!
//TODO: Rustfmt!

struct LabelEncoder {
    map : HashMap<String,u32>,
    num_classes : u32,   
}

impl LabelEncoder {
    fn new() -> LabelEncoder {
        LabelEncoder{map : HashMap::new() , num_classes : 0}
    }

    fn fit(&mut self, labels : &[&str]) {
        let mut num_classes = self.num_classes;
        //labels.iter().for_each(|l| self.map.entry(l.to_string()).or_insert_with(|| { self.num_classes += 1; self.num_classes - 1 } ));
        for label in labels.iter() {
            self.map.entry(label.to_string()).or_insert_with(|| { num_classes += 1; num_classes - 1 });
        }
        self.num_classes = num_classes;
    }

    fn transform(&self, labels : &[&str]) -> Vec<u32> {
        labels.iter().map(|l| *self.map.get(*l).expect("Label not yet encoded!")).collect()
    }
}

struct StopWordFilter {
    stop_words : Vec<String>,
}

impl StopWordFilter {
    fn new() -> StopWordFilter {
        StopWordFilter{ stop_words : vec![] }
    }

    fn filter(&self, data : &str) -> String {
        //TODO: implement
        String::new()
    }
}

struct Classifier {
    count_vecs : Vec<HashMap<String,u64>>,
    smoothing : f64,
}

impl Classifier {
    fn new() -> Classifier {
        Classifier { count_vecs : vec![] , smoothing : 1.0f64 }
    }

    fn train(&mut self, data_input : &[&str], labels_input : &[u64]) {
        assert!(data_input.len() == labels_input.len(), "Number of class labels does not match number of sample data!");

        let num_classes = *labels_input.iter().max().unwrap() + 1;
        (0..num_classes).for_each(|_| self.count_vecs.push(HashMap::new()));
        
        for (idx, data) in data_input.iter().enumerate() {
            for word in data.split_whitespace() {
                let word_count = self.count_vecs[labels_input[idx] as usize].entry(word.to_string()).or_insert(0u64);
                *word_count += 1;
            }        
        }
    }

    fn vocabulary_size(&self, class : usize) -> u64 {
        self.count_vecs[class].iter().fold(0u64, |sum, (_,count)| sum + count)
    } 

    fn classify(&self,input : &[&str])-> Vec<u64> {
        assert!(self.count_vecs.len() > 0, "Classifier must be trained first!");

        let probas = self.classify_proba(input);
        probas.iter().map(|class_probas| {
            // find index (class label) of maximum probability
            let mut max_idx = 0;
            for (idx,&p) in class_probas.iter().enumerate() {
                if p > class_probas[max_idx] { max_idx = idx; }
            }
            max_idx as u64
        }).collect()
    }

    fn classify_proba(&self, input : &[&str]) -> Vec<Vec<f64>> {
        assert!(self.count_vecs.len() > 0, "Classifier must be trained first!");
        
        let num_classes = self.count_vecs.len();
        let mut probas = Vec::with_capacity(input.len());
        for (idx, data) in input.iter().enumerate() {
            probas.push(vec![1.0f64;num_classes]);

            for word in data.split_whitespace() {
                for i in 0..num_classes {
                    probas[idx][i] *= (*self.count_vecs[i].get(word).unwrap_or(&0) as f64 + self.smoothing) / (self.vocabulary_size(i) as f64 + self.smoothing);
                }    
            }
        }        
        probas
    }

    fn accuracy(&self, input : &[&str], known_labels : &[u64]) -> f64 {
        assert_eq!(input.len(), known_labels.len(),"TODO: Add msg!");
        let predictions = self.classify(input);
        
        let count = predictions.iter().zip(known_labels.iter()).fold(0,|count, (predicted_label,known_label)| {
            if predicted_label == known_label { count + 1 }
            else { count }
        }); 
        count as f64 / input.len() as f64
    }
}

fn main() {    let mut data : Vec<String> = vec![];
    let mut labels : Vec<u64> = vec![];
    
    let filename = "";
    let f = File::open(filename).expect("file not found");
    let file = BufReader::new(&f);
    for line in file.lines() {
        let mut l = line.unwrap();
        if l.starts_with("ham") {
            data.push(l.split_off(3)
            .to_lowercase()
            .replace("."," ")
            .replace(","," ")
            .replace("!"," "));
            
            labels.push(0);
        } else {
            data.push(l.split_off(4)
            .to_lowercase()
            .replace("."," ")
            .replace(","," ")
            .replace("!"," "));

            labels.push(1);
        }
    }

    let mut input : Vec<&str> = vec![];
    for l in data.iter() {
        input.push(&l);
    }

    let mut clf = Classifier::new();
    clf.train(&input[..5000],&labels[..5000]);
    //println!("{}",clf.classify(&input[4001..]).len());
    println!("{}",clf.accuracy(&input[5001..],&labels[5001..]));
}

#[test]
fn test_class_label_encoding() {
    let mut class_label_encoder = LabelEncoder::new();

    let labels = vec!["class_a", "class_b", "class_a", "class_c"];
    class_label_encoder.fit(&labels);

    let encoded_labels = class_label_encoder.transform(&labels);
    assert_eq!(encoded_labels[0],0);
    assert_eq!(encoded_labels[1],1);
    assert_eq!(encoded_labels[2],0);
    assert_eq!(encoded_labels[3],2);
}

#[test]
fn test_vocabulary_size() {
    let data = vec!["such a mad mad world"];
    let classes = [0];
    let mut clf = Classifier::new();

    clf.train(&data,&classes);
    assert_eq!(clf.vocabulary_size(0), 5);
}

#[test]
fn test_count_words() {
    let data = vec!["mad mad world","beautiful world"];
    let classes = [0,1];
    let mut clf = Classifier::new();

    clf.train(&data, &classes);
    assert_eq!(clf.count_vecs.len(), 2);
    assert_eq!(*clf.count_vecs[0].get("world").unwrap(), 1u64);
    assert_eq!(*clf.count_vecs[0].get("mad").unwrap(), 2u64);
    assert_eq!(clf.count_vecs[0].get("beautiful"), Option::None);
    assert_eq!(*clf.count_vecs[1].get("beautiful").unwrap(), 1u64);
    assert_eq!(clf.count_vecs[1].get("mad"), Option::None);
}

#[test]
fn test_classify() {
    let data = vec!["mad mad world","beautiful world"];
    let classes = [0,1];
    let mut clf = Classifier::new();

    clf.train(&data,&classes);
    assert_eq!(clf.classify(&["mad mad world"])  ,[0]);
    assert_eq!(clf.classify(&["beautiful world"]),[1]);
}

#[test]
fn test_accuracy() {
    let data = vec!["mad mad world","beautiful world"];
    let classes = [0,1];
    let mut clf = Classifier::new();

    clf.train(&data,&classes);
    assert_eq!(clf.accuracy(&data,&classes),1.0);
}

#[test]
#[should_panic]
fn test_untrained_classifier() {
    let data = vec!["mad mad world"];
    let mut clf = Classifier::new();

    clf.classify(&data);
}

#[test]
fn test_stop_word_filter() {
    let data = "a mad mad world";
    let stop_word_filter = StopWordFilter::new();

    assert_eq!(&stop_word_filter.filter(data), "mad mad world");
}
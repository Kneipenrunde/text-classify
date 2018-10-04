use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufRead;

//TODO: clean strings from some chars  
//TODO: stems words!
//TODO: refactor code

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

struct Classifier {
    count_vecs : Vec<HashMap<String,u64>>
}

impl Classifier {
    fn new() -> Classifier {
        Classifier { count_vecs : vec![] }
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
                    probas[idx][i] *= (*self.count_vecs[i].get(word).unwrap_or(&0) + 1) as f64 / self.vocabulary_size(i) as f64;
                }    
            }
        }        
        probas
    }
}

fn vocabulary_size(word_count_vec : &HashMap<String,u64>) -> u64 {
    word_count_vec.iter().fold(0u64, |sum, (_,count)| sum + count)
}

fn union(some : &HashMap<String,u64>, other : &HashMap<String,u64>) -> HashMap<String,u64> {
    let mut map = HashMap::new();
    for (word, &count) in some {
        map.insert(word.clone(), count) ;
    }
    for (word, &count) in other {
        let word_count = map.entry(word.to_string()).or_insert(0u64);
        *word_count += count;
    }
    map
}

fn count_words(data : &str) -> HashMap<String, u64> {
    let mut word_vec = HashMap::new();
    for word in data.split_whitespace() {
        let count = word_vec.entry(String::from(word)).or_insert(0u64);
        *count += 1;
    }
    word_vec
}

fn main() {
    let data = vec!["mad mad world","beautiful world"];
    let classes = [0,1];
    let mut clf = Classifier::new();

    clf.train(&data,&classes);
    println!("{:?}",clf.classify_proba(&["mad mad world"]));
    println!("{:?}",clf.classify_proba(&["beautiful world"]));
    
    //let mut ham_lines : Vec<String> = vec![];
    //let mut spam_lines : Vec<String> = vec![];

    //TODO: insert proper file name
    //let filename = "";
    //let mut f = File::open(filename).expect("file not found");
    //let mut file = BufReader::new(&f);
    //for line in file.lines() {
    //    let mut l = line.unwrap();
    //    if l.starts_with("ham") {
    //        //println!("{}",l);
    //        ham_lines.push(l.split_off(3));
    //    } else {
    //        spam_lines.push(l.split_off(4));
    //    }

    //}

    //println!("Found {} ham lines and {} spam lines!",ham_lines.len(),spam_lines.len());
    //// produce word vectors 
    //let mut word_vec_ham = HashMap::new();
    //for line in ham_lines.iter().take(2000) {
    //    word_vec_ham = union(&count_words(&line), &word_vec_ham);
    //}
    //println!("{:?}",word_vec_ham);
    
    //let mut word_vec_spam = HashMap::new();
    //for line in spam_lines.iter().take(700) {
    //    word_vec_spam = union(&count_words(&line), &word_vec_ham);
    //}

    //println!("Should be ham:");
    //classify(&ham_lines[2050] , &word_vec_ham, &word_vec_spam);
    //classify(&ham_lines[2051] , &word_vec_ham, &word_vec_spam);
    //println!("Should be spam:");
    //classify(&spam_lines[710], &word_vec_ham, &word_vec_spam);
    //classify(&spam_lines[711], &word_vec_ham, &word_vec_spam);
}

// computation of resubstitutions error
fn compute_accuracy(word_vec_ham : &HashMap<String,u64>, word_vec_spam : &HashMap<String,u64>) {
    let mut correct_classifications = 0u64;
    
}

fn classify(data : &str, word_vec_ham : &HashMap<String,u64>, word_vec_spam : &HashMap<String,u64>) -> Vec<f64> {
    let mut propabilities : Vec<f64> = vec![1.0,1.0];
    //TODO: more generic smoothing than just plain 1
    let vocabulary_size_ham = vocabulary_size(word_vec_ham) + 1;
    let vocabulary_size_spam = vocabulary_size(word_vec_spam) + 1;
    for word in data.split_whitespace() {
        propabilities[0] *= (*word_vec_ham.get(word).unwrap_or(&0) + 1) as f64 / vocabulary_size_ham as f64;
        propabilities[1] *= (*word_vec_spam.get(word).unwrap_or(&0) + 1) as f64 / vocabulary_size_spam as f64;
    }
    println!("{:?}",propabilities);
    if propabilities[0] > propabilities[1] {
        println!("It is ham!");
    }
    else {
        println!("It is spam!");
    }
    propabilities
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
    let data = vec!["Hallo schöne schöne Welt!"];
    let classes = [0];
    let mut clf = Classifier::new();

    clf.train(&data,&classes);
    assert_eq!(clf.vocabulary_size(0), 4);
}

#[test]
fn test_count_words() {
    let data = vec!["Hallo schöne schöne Welt!","Schlechte Welt!"];
    let classes = [0,1];
    let mut clf = Classifier::new();

    clf.train(&data, &classes);
    assert_eq!(clf.count_vecs.len(), 2);
    assert_eq!(*clf.count_vecs[0].get("Hallo").unwrap(),     1u64);
    assert_eq!(*clf.count_vecs[0].get("schöne").unwrap(),    2u64);
    assert_eq!(*clf.count_vecs[1].get("Schlechte").unwrap(), 1u64);
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
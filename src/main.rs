// Import Crates
use std::fs::File;
use std::io::{BufRead, BufReader};
use sha2::{Sha256, Digest};

#[derive(Debug)]
pub struct HashNode{
    index: usize,
    hash:std::string::String,
    string: String,
    containedValues: Vec<usize>,
}

impl HashNode {
    fn change_hash(&mut self, stringVal: String) {
        self.hash = stringVal;
    }
}

fn main()  {
    // Read Input Data from txt file
    let mut AllData = getInput();

    println!("{:#?}",AllData);

    //todo!()

    



    //Create vector of strings for leaves
    for i in (0..AllData.len()){
        println!("{}",i);
        let mut index = AllData[i].len();
        for j in (0..i){
            if j % 2 == 0{
                let mut hasher = Sha256::new();
                let total_string = format!("{}{}", AllData[i][j].string,AllData[i][j+1].string);
                hasher.update(total_string);
                let result1 = hasher.finalize();
                let hex1 = hex::encode(&result1);
                
                AllData[i][index].change_hash(hex1)
                AllData[i][index].index = index;
            }else{
                let mut hasher = Sha256::new();
                let total_string = format!("{}{}", AllData[i][j+1].string,AllData[i][j].string);
                hasher.update(total_string);
                let result1 = hasher.finalize();
                let hex1 = hex::encode(&result1);

                AllData[i][index].change_hash(hex1);
                AllData[i][index].index = index;
            }

            index = index / 2
        }
    }
    

    // Hash inputs and append to vector
    

    // Then Write an algorithm that calculates the ROOT


    // Return the root hash as a String
}

fn getInput() -> Vec<Vec<HashNode>>{

    let mut dataVector = Vec::new();

    

    for input in 1..5{
        let input = File::open(format!("input{}.txt",input))
        .expect("Should have been able to read the file");
        let input = BufReader::new(input);
        let mut InputVec = Vec::new();
        for (i,line) in input.lines().enumerate() {
            let line = line.expect("Unable to read line");
            let mut hasher = Sha256::new();
            hasher.update(line.to_string());
            let result = hasher.finalize();
            let hex = hex::encode(&result);
            let nodeString = HashNode{index:i,hash:hex.to_string().to_owned(),string:line,containedValues:vec![]};
            InputVec.push(nodeString);
            setHashes(i);
        }
        dataVector.push(InputVec);
        
        println!("{:?}",dataVector);
    }
    dataVector
}

fn setHashes(index:usize){
    println!("{:?}",index);
}

// You can use templates below or just remove
// Helper function to create a next leaves level may help you :)
fn create_next_level(current_level: Vec::<String>) -> Vec::<String> {
    todo!();
}


// Helper function may help you to hash an input or You can write macro rules
fn hash_input(a: &str) -> String {
    todo!();
}




// Pass all tests!
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn input_0() {
        let result = main("input0.txt".to_string());
        assert_eq!(result, "ff41418be11ed77612aeb83ee0bcf97a5853a4c291e23bd4d4cc6435fcfabdf9");
    }

    #[test]
    fn input_1() {
        let result = main("input1.txt".to_string());
        assert_eq!(result, "98a77b2c3ff5f6c2aca697f60b2aa2a1a2733be36dbd35bae23d517c2ad5985e");
    }

    #[test]
    fn input_2() {
        let result = main("input2.txt".to_string());
        assert_eq!(result, "3c0fb0638de91551eae4e9d984d72034aa0693be37b51737e6b81bc489866e5e");
    }

    #[test]
    fn input_3() {
        let result = main("input3.txt".to_string());
        assert_eq!(result, "f03b1c9163babeb728ac011fe0c2c9c69700a2f8ddde211ec07d621cdb322cfe");
    }

    #[test]
    fn input_4() {
        let result = main("input4.txt".to_string());
        assert_eq!(result, "f83e74742fda659dfc07615881af796abafc434f591aeb23b9f4366abe03c597");
    }
}
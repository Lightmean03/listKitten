use std::io::Write;
use std::fs;
use std::fs::read_to_string;
use std::fs::File;
use clap::{command, Arg};
#[allow(non_snake_case)]
fn write_file(content: &str, output_file: &str){
    let output = File::create(output_file);
    let mut output = match output{
        Ok(file) => file,
        Err(error) => panic!("Cant open file: {:?}", error),
    };
    output.write(content.as_bytes()).expect("unable to write data"); 
}
fn combine(list_input_1: &str, list_input_2: &str) -> String{
    let mut output = String::from("");
    let I1 = fs::read_to_string(list_input_1).expect("unable to read file 1");
    let I2 = fs::read_to_string(list_input_2).expect("unable to read file 2");
    for line in I1.lines(){
        for line2 in I2.lines(){ 
            let mut line = line.to_string();
            line.push_str(&line2);
            line.push_str("\n");
            output.push_str(&line);
        }

    }
    return output; 
}
fn perm_write(path: &str, output: &str){
    write_file(perm(path).as_str(), output);
}
fn perm(path: &str) -> String{
    let mut words : String  = String::from("");
    let I1 = fs::read_to_string(&path).expect("unable to read");
    let I2 = fs::read_to_string(&path).expect("unable to read");
    for line in I1.lines(){
        let mut linef:String= String::from("");
        for line2 in I2.lines(){
            linef.push_str(&line);
            linef.push_str(&line2);
            linef.push_str("\n");
        }

        words.push_str(&linef);
    }
    return words;
}
fn combine_with_spaces(list_input_1: &str, list_input_2: &str) -> String{
    let mut output = String::from("");
    let I1 = fs::read_to_string(list_input_1).expect("unable to read file 1");
    let I2 = fs::read_to_string(list_input_2).expect("unable to read file 2");
    for line in I1.lines(){
        for line2 in I2.lines(){ 
            let mut line = line.to_string();
            line.push_str(" ");
            line.push_str(&line2);
            line.push_str("\n");
            output.push_str(&line);
        }

    }
    return output; 

}
fn combine_all(list_input_1: &str, list_input_2: &str) -> String{
    let mut all = combine(&list_input_1, &list_input_2);
    all.push_str(&combine(list_input_2, list_input_1));
    return all; 
}
fn copy_file_contents(file_path: &str, list_input: &str){
    let mut input: File = match File::create(&list_input){
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    for line in read_to_string(file_path).unwrap().lines(){
        input.write(line.as_bytes()).expect("Unable to write data");
        input.write("\n".as_bytes()).expect("Unable to write data");
    } 
}
fn pattern(pattern: &str, input_1: &str, _list_input_2: &str, list_output: &str){

     let alphabet: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 
        'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
    ];

   let mut pattern = String::from(pattern);
   pattern.push_str("_");
   let list_input_write = String::from("tmp_list_write.txt");
   let list_input_read = String::from("tmp_list_read.txt");
    let output = File::create(list_output);
    let mut output = match output{
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    copy_file_contents(input_1, &list_input_write);

    for letter in pattern.chars(){
        copy_file_contents(&list_input_write, &list_input_read);
        let mut input: File = match File::create(&list_input_write){
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
        let mut append = String::new(); 
        match letter{
            'N' => {
                    for line in read_to_string(&list_input_read).unwrap().lines(){
                        for n in 0..10 {
                            let line = line.to_string();
                            append.push_str(&line.to_string());
                            append.push_str(&n.to_string());
                            append.push_str("\n");
                        }
                    }
            },
            '_' => {
                for line in read_to_string(&list_input_read).unwrap().lines(){
                        let line = line.to_string();
                        append.push_str(&line);
                        append.push_str( " \n");
                }
            }
            'Z' =>{
                for line in read_to_string(&list_input_read).unwrap().lines(){
                    for letter in alphabet.iter() {
                        append.push_str(&line.to_string());
                        append.push_str(&letter.to_string());
                        append.push_str("\n");
                    }
                }
            }
            _ => println!("Invalid pattern")
        }
        input.write(append.as_bytes()).expect("Unable to write data");
    }
    for line in read_to_string(&list_input_read).unwrap().lines(){
        output.write(line.as_bytes()).expect("Unable to write data");
        output.write("\n".as_bytes()).expect("Unable to write data");
    }

}
fn main() {
  
    let res = command!()
        .about("This is a program to combine two lists of words into one list of words.")
        .arg(
           Arg::new("Mode")
                .short('m')
                .long("mode")
                .aliases([ "M","MODE"])
                .required(true)
                .help("Options: left, right, all_space, all, perm")
            )
        .arg(
            Arg::new("Pattern")
                .short('p')
                .long("pattern")
                .aliases([ "P","PATTERN"])
                .required(false)
                .help("The pattern to use Example:-p N_Z\nOptions:\nN = Numbers 0-9\nZ = Alphabet\n_ = Space\n")
            )
            .arg(
                Arg::new("File Path 1")
                .short('f')
                .long("file1")
                .required(true)
                .help("The first file path \"path1.txt\"")
            )
            .arg(
                Arg::new("File Path 2")
                .short('F')
                .long("file2")
                .required(false)
                .help("The seconed file path \"path2.txt\"")
            )
            .arg(
                Arg::new("Output File")
                .short('o')
                .long("output")
                .aliases([ "O","OUTPUT","output-file"])
                .required(true)
                .help("The output file path \"output.txt\"")
            ).get_matches();
    let mode =  res.get_one::<String>("Mode").unwrap(); 
    let file_path_1 = res.get_one::<String>("File Path 1").unwrap();
    let file_path_2 = res.get_one::<String>("File Path 2").unwrap();
    let output_file = res.get_one::<String>("Output File").unwrap();
        match mode.as_str(){
            "left" => write_file(combine(&file_path_1,&file_path_2).as_str(), &output_file), 
            "right" => write_file(&combine(&file_path_2, &file_path_1), &output_file),
            "all_space" => write_file(&combine_with_spaces(&file_path_1, &file_path_2), &output_file),
            "all" => write_file(&combine_all(&file_path_1, &file_path_2), &output_file),
            "pattern" => pattern(&res.get_one::<String>("Pattern").unwrap(), &file_path_1, &file_path_2, &output_file),
            "perm" => perm_write(&file_path_1, &output_file),
            _ => println!("Invalid mode")
            
    }
  println!("{}", mode);
  //combine_all("list_input_1.txt", "list_input_2.txt", "list_output.txt"); 
}

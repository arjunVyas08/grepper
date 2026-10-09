use std::env;
use std::path::Path;
use std::io::Result;
use std::fs;


fn main() -> Result<()>{
    let args:Vec<String> = env::args().collect(); // so that it can be called from the terminal


    if args.len() <= 2{
        println!("enter the path of a .txt file\nenter a space then a word to search for it");
    }else{
        //dbg!(&args[1]);

        

        let file_path:&Path = Path::new(&args[1]);
       
        let word = &args[2];

        let contents:String = get_file(file_path)?; //get text

        let cleaned = contents.replace("\r" , "").replace("\n", " ");
        let contents:Vec<&str> = cleaned.split(" ").collect();


        if contents.len() != 0{
    
            word_search(contents,word);
        }else{
            return Ok(());
        }
        



    }
    

    Ok(())
}

fn get_file(name:&Path)->Result<String>{
    let mut contents:String = String::new();

    if fs::exists(name)?{
    
        contents =  fs::read_to_string(name)?;

    }else{
        println!("file not found")
    }
    
    Ok(contents)
}


fn word_search(contents:Vec<&str> , word:&str){
    for x in 0..contents.len(){
        if contents[x].to_lowercase() == word{
            println!("{} found at position {}", word , x);
        }
    }

}

use compiler::{handle_file_or_dir_given::handle_file_or_dir};
use compiler::compilation_engine::{CompilationEngine};
use std::env;


fn main() {
        let os_args = env::args();
        let os_args:Vec<String> = os_args.collect();

        let amount_of_args = os_args.len() - 1;
        if amount_of_args != 1{
            eprintln!("Error unexpected amount of args given expected 1.");
            eprintln!("Amount given: {amount_of_args}");
            panic!("PLEASE give correct amount of args");
        }

 

        let file_readers = handle_file_or_dir(&os_args[1]);

        let Ok(files) = file_readers else{
            panic!("expected file name or directory name to be given");
        };
        println!("files received {:?}",files);
        

        if let Ok(mut compiler_final) = CompilationEngine::new(files){
        println!("{:?}",compiler_final);
        compiler_final.begin();
        }else{
            panic!("Error occured trying to create compiler from args")
        }

       
}

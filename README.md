# easy_file_system


Coming from Python to Rust is hard, I'm not saying its impossible but the syntax is weird for someone coming from Python. The `fs` in 
Python is easy, but in Rust its pretty hard for a beginner so I have spent my time on this package and tried to make it as easy as possible.
You should some basics of Rust. 



**Examples**


```rs
use easy_fs::fs::FileHandler;


fn main()  {
    // We are making a FileHandler
    let something = FileHandler {
        path: "src/file.txt".to_owned(),  
        file_name: "file.txt".to_owned()

    };

    // Reading the file
    match something.read() {
        Ok(content) => println!("{:?}", content),
        Err(e) => println!("{:?}", e)
    }


    // We are making a FileHandler
    let f = FileHandler {
        path: "src/foo.txt".to_owned(),
        file_name: "foo.txt".to_owned()
    };
    
    // We are writing content into the file.
    match f.write("Hello people") {
        Ok(_) => println!("Passed"),
        Err(e) => println!("{:?}", e)
    }

}
```
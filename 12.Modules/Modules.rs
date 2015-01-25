fn function() {
    println!("called `funtion()`");
}

mod my {
    #[allow(dead_code)]
    fn function() {
        println!("called `my::function()`");
    }

    mod nested {
        #[allow(dead_code)]
        fn function() {
            println!("called `my::nested::function()`");
        }
    }
}

fn main() {
    function();

    std::io::stdio:println("Hello, world!!");

    my::function();
}

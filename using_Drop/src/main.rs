struct CustomPointer {
    data: String,
}

impl Drop for CustomPointer {
    fn drop(&mut self) {
        println!("Dropping CustomPointer with data '{}'!", self.data)
    }
}

fn main() {
    let c = CustomPointer {
        data: String::from("my stuff"),
    };

    let d = CustomPointer {
        data: String::from("other stuff"),
    };

    println!("CustomPointer crated.");

    drop(c);

    println!("CustomPointer dropped before the end of main.");
}
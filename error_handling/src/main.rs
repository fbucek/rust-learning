use errlib;

fn main() {
    let filename = "hello.txt";

    errlib::result_function(&filename);
    errlib::option_function(&filename);
}

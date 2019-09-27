use futures::executor::block_on;

async fn print_info() {
    println!("Info from async function");
    next_info().await
}

async fn next_info() {
    println!("Next info Info from async function");
}

async fn third_info() {
    println!("Third info Info from async function");
}


fn main() {
    let future = print_info();
    let future2 = third_info();
    block_on(future2);
    block_on(future);
}

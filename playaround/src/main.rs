// mod m7_async;
mod m9_decl_macros;
mod m10_proc_macros;
mod m12_concurrency;

const OUR_COURSE: &str = "Rust with AutoGPT";

#[tokio::main]
async fn main(){
    println!("Welcome to this course on {}!", OUR_COURSE);
    
}
mod log;

fn main() {
    println!("Hello, world!");

	let log_i = log::new_log("hello timebr".to_string());
	println!("{}", log_i.brief);
}

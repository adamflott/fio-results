use fio_results::parse_file;

fn main() {
    let filename = std::env::args().nth(1).unwrap();

    match parse_file(filename) {
        Ok(result) => {
            println!("{:#?}", result);
        }
        Err(err) => {
            panic!("{}", err);
        }
    }
}

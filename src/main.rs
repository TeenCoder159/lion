use std::env;
mod writer;

struct Input {
    filename: String,
}

fn main() {
    let help =
        "Command list:\n lion <fileName.extension> -> Creates a file with filler code of the extension type\n";
    let file_name = env::args()
        .nth(1)
        .expect("No file name given.\nPlease provide a file name and try again");
    //let _ext_dep = env::args().nth(2).expect("no path given");

    let args = Input {
        filename: file_name,
    };

    if args.filename.to_lowercase() == "help" {
        println!("Help command called.\n{help}");
        return;
    }
    let extension = args.filename.split('.').last().unwrap_or("");
    writer::write(extension, &args.filename);
    println!("Created .{extension} file");
}

extern crate clipboard_win;
extern crate image;
use clipboard_win::get_clipboard_string;
use clipboard_win::set_clipboard_string;
use clipboard_win::{formats, get_clipboard, set_clipboard};

use std::env;
use std::io::{self, Read, Write, Cursor};

fn help() {
    println!("clipboard.exe – Access the Windows clipboard (copy/paste)");
    println!("");
    println!("Usage:");
    println!("  clipboard.exe --copy < echo Hello");
	println!("  clipboard.exe --paste");
	println!("  clipboard.exe --copy --image < test.png");
    println!("  clipboard.exe --paste --image ");
	println!("");
    println!("    --copy  - stores stdin into clipboard");
	println!("    --paste - pastes clipboard content to stdout");
    println!("    --copy --image - converts stdin to bmp then stores into clipboard");
    println!("    --paste --image - converts clipboard content to png then pastes to stdout");
    println!("");
    println!("MIT © Sindre Sorhus");
}

fn copy_string() -> std::io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
	set_clipboard_string(&buffer).unwrap();
	Ok(())
}

fn paste_string() -> std::io::Result<()> {
    io::stdout().write(get_clipboard_string().unwrap().as_bytes()).unwrap();
    Ok(())
}

fn copy_image() -> std::io::Result<()> {
	let mut buffer: Vec<u8> = Vec::new();
	io::stdin().read_to_end(&mut buffer).unwrap();
	let reader = image::io::Reader::new(Cursor::new(buffer)).with_guessed_format().unwrap();
	let img = reader.decode().unwrap();
	let mut bmp: Vec<u8> = Vec::new();
	img.write_to(&mut bmp, image::ImageOutputFormat::Bmp).unwrap();
	set_clipboard(formats::Bitmap, bmp).unwrap();
	Ok(())
}

fn paste_image() -> std::io::Result<()> {
	let bytes = &get_clipboard(formats::Bitmap).unwrap();
	let img = image::io::Reader::with_format(Cursor::new(bytes), image::ImageFormat::Bmp).decode().unwrap();
	let mut png: Vec<u8> = Vec::new();
	img.write_to(&mut png, image::ImageOutputFormat::Png).unwrap();
	io::stdout().write(&png).unwrap();
	Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 3 {
        println!("You should specify `--copy` or `--paste` mode. See `--help` for usage examples.");
        return;
    }

    if args.len() == 2 {
        let cmd = &args[1];

        match &cmd[..] {
            "--copy" => copy_string().expect("Error: Could not copy to clipboard"),
            "--paste" => paste_string().expect("Error: Could not paste from clipboard"),
            _ => help(),
        }

        return;
    }

    if !args.contains(&String::from("--image")) {
        help();
        return;
    }

	if args.contains(&String::from("--copy")) {
		copy_image().expect("Error: Could not paste from clipboard");
		return;
    }

    if args.contains(&String::from("--paste")) {
		paste_image().expect("Error: Could not paste from clipboard");
		return;
    }
}

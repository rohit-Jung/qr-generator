use clap::Parser;
use image::Luma;
use qrcode::QrCode;

#[derive(Parser, Debug)]
#[command(name = "qrgen")]
#[command(about = "Generate a QR code from a URL", long_about = None)]
struct Args {
    #[arg(long)]
    url: String,
}

fn print_ascii_qr(code: &QrCode) {
    let matrix = code.to_colors();

    let padding = 4;
    let width = code.width();

    // ANSI escape codes
    const BLACK_BLOCK: &str = "\x1b[40m  \x1b[0m";
    const WHITE_BLOCK: &str = "\x1b[47m  \x1b[0m";

    for _ in 0..padding {
        println!("{}", WHITE_BLOCK.repeat(width + padding * 2));
    }

    for y in 0..width {
        // Left padding
        print!("{}", WHITE_BLOCK.repeat(padding));
        for x in 0..width {
            let idx = y * width + x;
            let cell = &matrix[idx];
            print!(
                "{}",
                if *cell == qrcode::Color::Dark {
                    BLACK_BLOCK
                } else {
                    WHITE_BLOCK
                }
            );
        }

        // Right padding
        println!("{}", WHITE_BLOCK.repeat(padding));
    }

    for _ in 0..padding {
        println!("{}", WHITE_BLOCK.repeat(width + padding * 2));
    }
}

fn main() {
    let args = Args::parse();
    println!("Generating qr for {}", args.url);
    let code = QrCode::new(args.url).unwrap();

    let image = code
        .render::<Luma<u8>>()
        .min_dimensions(300, 300)
        .quiet_zone(true)
        .build();

    image.save("qrcode.png").unwrap();
    println!("Saved QR code as qrcode.png");

    println!("\nANsCII QR code:\n");
    print_ascii_qr(&code);
}

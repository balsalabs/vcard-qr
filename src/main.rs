use std::fs::File;
use std::{error::Error, fs};
use std::path::Path;
use csv::{Reader, Writer};
use qrcode::QrCode;
use image::{ImageFormat, Rgba};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input CSV file
    #[arg(long)]
    data: String,

    /// Output directory for QR codes and result CSV
    #[arg(long)]
    output: String,
}

fn generate_qr_code(fullname: &str, email: &str, filename: &str) -> Result<(), Box<dyn Error>> {
    let vcard = format!(
        "BEGIN:VCARD\nVERSION:3.0\nFN:{}\nEMAIL:{}\nEND:VCARD",
        fullname, email
    );
    let code = QrCode::new(vcard)?;
    let image = code.render::<Rgba<u8>>()
        .module_dimensions(2, 2)
        .build();

    // Ensure the image is exactly 106x106 pixels
    let resized = image::imageops::resize(&image, 106, 106, image::imageops::FilterType::Nearest);

    // Save the image
    resized.save_with_format(filename, ImageFormat::Png)?;
    Ok(())
}

fn process_csv(input_path: &str, output_dir: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = Reader::from_path(input_path)?;
    println!("read successfull !");
    let output_csv_path = Path::new(output_dir).join("result.csv");

    // Create output directory if it doesn't exist
    fs::create_dir_all(output_dir)?;

    // Create or open the result.csv file
    let file = File::create(&output_csv_path)?;
    let mut writer = Writer::from_writer(file);

    writer.write_record(&["full name", "email", "filename"])?;

    for (index, result) in reader.records().enumerate() {
        let record = result?;
        let fullname = &record[0];
        let email = &record[1];
        let filename = format!("{}.png", index + 1);
        let qr_path = Path::new(output_dir).join(&filename);

        generate_qr_code(fullname, email, qr_path.to_str().unwrap())?;

        writer.write_record(&[fullname, email, &filename])?;
    }

    writer.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    process_csv(&args.data, &args.output)?;

    println!("QR codes generated and result.csv created in: {}", args.output);
    Ok(())
}

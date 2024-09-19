use std::fs::File;
// use std::io::Cursor;
use std::{error::Error, fs};
use std::path::Path;
use csv::{Reader, Writer};
use qrcode::QrCode;
use image::Luma;
// use base64::{Engine as _, engine::general_purpose};
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

fn generate_qr_code(
    fullname: &str,
    email: &str,
    title: &str,
    company: &str,
    filename: &str
) -> Result<(), Box<dyn Error>> {
  let namesplit = fullname.split_whitespace().collect::<Vec<&str>>();
  let firstname = namesplit[0];
  let lastname = namesplit[1..].join(" ");

  let vcard = format!(
  "BEGIN:VCARD
VERSION:4.0
FN:{}
N:{};{}
EMAIL;TYPE=work:{}
ORG:{}
TITLE:{}
END:VCARD",
    fullname, firstname, lastname ,email, company, title
  );
  let code = QrCode::new(vcard.as_bytes())?;
  let base_image = code.render::<Luma<u8>>()
      .min_dimensions(100, 100)  // Increased module size
      .build();

  let result = base_image.save_with_format(filename, image::ImageFormat::Png);
  match result {
    Ok(_) => println!("saved file {} to disk!", filename),
    Err(e) => {eprint!("Error occured {} !", e);}
  }

  // // Convert to base64
  // let mut buffer = Cursor::new(Vec::new());
  // base_image.write_to(&mut buffer, image::ImageFormat::Png)
  //   .unwrap_or_else(|err| panic!("Error writing image to buffer: {}", err));

  // let base64_image = general_purpose::STANDARD.encode(&buffer.into_inner());

  // let result = std::fs::write(format!("{}.txt", filename), base64_image);
  // if let Err(e) = result {
  //     eprintln!("Error writing QR code to file: {}", e);
  // }

  Ok(())
}

fn process_csv(input_path: &str, output_dir: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = Reader::from_path(input_path)?;
    println!("Read successful!");
    let output_csv_path = Path::new(output_dir).join("result.csv");
    fs::create_dir_all(output_dir)?;
    let file = File::create(&output_csv_path)?;
    let mut writer = Writer::from_writer(file);
    writer.write_record(&["full name", "email", "title", "company", "filename"])?;

    for (index, result) in reader.records().enumerate() {
        let record = result?;
        let fullname = &record[0];
        let email = &record[1];
        let title = &record[2];
        let company = &record[3];
        let filename = format!("{}.png", index + 1);
        let qr_path = Path::new(output_dir).join(&filename);
        generate_qr_code(fullname, email, title, company, qr_path.to_str().unwrap())?;
        writer.write_record(&[fullname, email, title, company, &filename])?;
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

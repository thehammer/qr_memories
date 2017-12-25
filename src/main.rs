extern crate qrcodegen;
use qrcodegen::QrCode;
use qrcodegen::QrCodeEcc;

extern crate handlebars;
#[macro_use]
extern crate serde_json;
use handlebars::Handlebars;

use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let svgs = stdin
        .lock()
        .lines()
        .map(|r| svg_qr_code(&r.unwrap()))
        .collect::<Vec<_>>();

    let rows = svgs
        .chunks(3)
        .collect::<Vec<_>>();

    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file("qr_codes", "./templates/qr_codes.hbs")
        .ok()
        .unwrap();

    println!(
        "{}",
        handlebars
            .render("qr_codes", &json!({"svgs": rows}))
            .unwrap()
    );
}

fn svg_qr_code(text: &str) -> String {
    let errcorlvl: QrCodeEcc = QrCodeEcc::Low;
    let qr: QrCode = QrCode::encode_text(text, errcorlvl).unwrap();
    let svg = qr.to_svg_string(4);
    let lines: Vec<&str> = svg.lines().skip(2).collect();
    let result = lines.join("\n");

    result
}

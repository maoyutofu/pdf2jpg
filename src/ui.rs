use fltk::{app, button, dialog, frame, group, input, prelude::*, window};

use crate::tojpeg::pdf_to_jpegs;

fn input_pack() -> (group::Pack, input::Input) {
    let mut pack = group::Pack::new(0, 20, 500, 40, "");
    pack.set_type(group::PackType::Horizontal);
    pack.set_spacing(10);
    frame::Frame::default().with_size(80, 0).with_label("输入:");
    let inpt = input::Input::default().with_size(300, 0);
    let mut btn = button::Button::default()
        .with_size(80, 0)
        .with_label("@filenew");
    let mut inpt_clone = inpt.clone();
    btn.set_callback(move |_btn| {
        let mut dialog = dialog::NativeFileChooser::new(dialog::NativeFileChooserType::BrowseFile);
        dialog.set_filter("*.{pdf}");
        dialog.show();
        inpt_clone.set_value(dialog.filename().to_str().unwrap());
    });
    pack.end();
    (pack, inpt)
}

fn out_pack() -> (group::Pack, input::Input) {
    let mut pack = group::Pack::new(0, 80, 500, 40, "");
    pack.set_type(group::PackType::Horizontal);
    pack.set_spacing(10);
    frame::Frame::default().with_size(80, 0).with_label("输出:");
    let inpt = input::Input::default().with_size(300, 0);
    let mut btn = button::Button::default()
        .with_size(80, 0)
        .with_label("@fileopen");
    let mut inpt_clone = inpt.clone();
    btn.set_callback(move |_btn| {
        let mut dialog = dialog::NativeFileChooser::new(dialog::NativeFileChooserType::BrowseDir);
        dialog.show();
        inpt_clone.set_value(dialog.filename().to_str().unwrap());
    });
    pack.end();
    (pack, inpt)
}

fn action_pack(ipt: input::Input, opt: input::Input) -> group::Pack {
    let mut pack = group::Pack::new(400, 140, 500, 40, "");
    pack.set_type(group::PackType::Horizontal);
    let mut btn = button::Button::default()
        .with_size(80, 0)
        .with_label("@filesaveas");
    btn.set_callback(move |_btn| {
        let input_path = ipt.value();
        let out_path = opt.value();
        if input_path == "" {
            dialog::alert_default("请选择 pdf 文件");
            return;
        }
        if out_path == "" {
            dialog::alert_default("请选择 jpg 输出目录");
            return;
        }
        match pdf_to_jpegs(&input_path, &out_path) {
            Ok(_) => dialog::alert_default("已完成"),
            Err(e) => dialog::alert_default(&format!("错误:{}", e.to_string())),
        }
    });
    pack.end();
    pack
}

pub fn run() {
   let app = app::App::default();

    let mut wd = window::Window::new(100, 100, 500, 200, "pdf2jpg").center_screen();

    let (_, ipt) = input_pack();
    let (_, opt) = out_pack();
    action_pack(ipt, opt);

    wd.end();
    wd.show();

    app.run().unwrap(); 
}

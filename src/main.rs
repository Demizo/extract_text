use tesseract;
use cli_clipboard::{self, ClipboardContext, ClipboardProvider};
use screenshot_rs;
use notify_rust;

fn main() {
    let path = "/tmp/ExtractTxt/tmpss.png";

    let mut ctx = ClipboardContext::new().expect("failed to create clipboard context");
    
    screenshot_rs::screenshot_area(path.to_string(), true);
    
    let output = tesseract::ocr(path, "eng").unwrap_or_else(|err| {
        panic!("{err}");
    });
    
    ctx.set_contents(output.to_string()).expect("couldn't write contents to clipboard");

    notify_rust::Notification::new()
    .summary("Text Extracted")
    .body(ctx.get_contents().unwrap().as_str())
    .timeout(notify_rust::Timeout::Milliseconds(6000)) //milliseconds
    .show().unwrap();
    
    
    println!("{}", ctx.get_contents().unwrap().to_string());
    
    
}
 
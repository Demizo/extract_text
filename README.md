# Extract Text
### A simple and streamlined Linux application to extract the text from any on screen selection using your native screenshot selection tool. The text is copied automatically to your clipboard.

# How It Works
The program will automatically launch your system's screenshot utility allowing you to select the area of the screen to extract text from.
Next, text will be extracted from the area you selected and sent to your system's clipboard alongside a notification to show you what was extracted.

### Compatibility:
The app should work with a variety of Linux distributions, desktop environments, and screenshot utilities. It may even function on other platforms, but this has not been tested.

### Usage:
The app is intended as a quality of life utility and should be bound to a *custom keyboard shortcut* or *command* on your system since it is essentially just a script.

### Limitations:
Currently it only supports English text.
The screenshots you take are saved to the tmp system directory ("/tmp/ExtractTxt/tmpss.png") this way the captured image will automatically be deleted.
# Dependencies
This app was built using Rust and depends on [Tesseract](https://github.com/tesseract-ocr/tesseract).

# To-do
- [ ] Support more languages

- [ ] Add app icon
 
# Development / Compiling
This project is build with Rust and Cargo. To develop and compile this project you can simply clone the repository and use `cargo run` to build and launch the application.

mod backend;
mod bitstream;
mod to_monochrome_png;
use gtk::prelude::*;

fn main() {
    let application = gtk::Application::builder()
        .application_id("org.samuel.barcode_generator")
        .build();

    application.connect_activate(|app| {
        let window = gtk::ApplicationWindow::builder()
            .application(app)
            .title("Barcode Generator")
            .window_position(gtk::WindowPosition::CenterAlways)
            .build();

        let grid = gtk::Grid::builder()
            .margin(24)
            .row_spacing(8)
            .column_spacing(8)
            .build();

        // Make things to put into the grid
        let label = gtk::Label::with_mnemonic("Enter a 12-digit number");
        let text_buffer = gtk::EntryBuffer::new(None);
        let textbox = gtk::Entry::with_buffer(&text_buffer);
        let ok_button = gtk::Button::with_label("OK");
        let message = gtk::Label::new(None);

        // Attach everything in the grid
        grid.attach(&label, 0, 0, 1, 1);
        grid.attach(&textbox, 0, 1, 1, 1);
        grid.attach(&ok_button, 1, 2, 1, 1);
        grid.attach(&message, 0, 3, 1, 1);

        window.add(&grid);

        // Define how the button calls backend
        ok_button.connect_clicked(move |_| {
            // Get textbox text
            let barcode_text = text_buffer.text();

            // Run the actual generator
            backend::main(&barcode_text);

            // TODO: Return results=
            let new_label = "Done";
            message.set_label(new_label);
            println!("{}", text_buffer.text());
        });

        window.show_all();
    });

    application.run();

    // backend::main();
}

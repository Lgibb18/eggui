slint::slint! {
    export component App inherits Window {
        width: 800px;
        height: 480px;

        Text {
            text: "abc абв";
        }
    }
}

fn main() {
    App::new().unwrap().run().unwrap();
}
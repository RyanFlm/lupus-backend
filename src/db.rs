use postgres::{Client, NoTls};

fn setup() {
    let mut client = Client::connect("host=localhost user=lupus", NoTls);
}
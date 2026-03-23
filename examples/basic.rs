use nue_sys::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx = App::context()?;
    let mut app = App::uart(&mut ctx)?;
    let incoming = app.incoming();

    for card in incoming {
        match card {
            Ok((_id, card)) => println!("Card: {:?}", card.as_slice()),
            Err(e) => return Err(e.into()),
        }
    }

    Ok(())
}

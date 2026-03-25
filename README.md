# nue

An experiemental NFC reading / writing system for the (currently) PN532.

## Example

> [!Warning]  
> This project at the current stage isn't something you should use in a real enviroment.
>

```rs
use nue_sys::{App, Result};

fn main() -> Result<()> {
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
```

## Crates split:

- `nue-model`: core data models shared across the other crates, Currently splits into two crates.
  - `raw` is a `no_std` crate for an NFC card used by `nue-sys`.
  - `card` is specifically used by `nue-storage` and `nue-web` to represent more infomation about an NFC card,
    This information usually lives somewhere in a database, see `lib/schema.sql`, requires `extras` feature.
- `nue-sys`: the NFC read / write system implementation.
- `nue-storage`: Provides interfaces and default implementations for storing core data models.
- `nue-web`: API & Web interface for managing stored cards data, unimplemented, ...currently.
- `nue-cli`: CLI Utilities to read / write NFC cards.

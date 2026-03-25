///! A CLI tool for quickly, Read, Write, and Provision NFC cards.
use colored::Colorize;
use inquire::{Confirm, Select, Text};

use nue_sys::{App, CardID, RawCard, Token};

#[derive(Debug)]
enum Action {
    ReadCard,
    WriteCard,
    ProvisionCard,
    Quit,
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::ReadCard => write!(f, "Read card"),
            Action::WriteCard => write!(f, "Write card"),
            Action::ProvisionCard => write!(f, "Provision new card"),
            Action::Quit => write!(f, "Quit"),
        }
    }
}

fn print_banner() {
    println!("{}", "╭─────────────────────────────╮".cyan());
    println!("{}", "│       NFC Card Manager      │".cyan());
    println!("{}", "╰─────────────────────────────╯".cyan());
    println!();
}

fn print_card(_card: &RawCard, id: &CardID) {
    println!(
        "  {} {}",
        "Card ID:".dimmed(),
        format!("{}", id).yellow().bold()
    );
    // println!("  {} {:?}", "Tier:   ".dimmed(), card.tier());
    // println!(
    //     "  {} {}",
    //     "Active: ".dimmed(),
    //     if card.active() {
    //         "yes".green().bold()
    //     } else {
    //         "no".red().bold()
    //     }
    // );
}

fn run_read(app: &mut App) {
    println!("\n{}", "  Tap a card to read...".dimmed());
    match app.incoming().next() {
        Some(Ok((id, card))) => {
            println!("\n  {} card found\n", "✓".green().bold());
            print_card(&card, &id);
        }
        // Some(Ok(_)) => {
        //     println!("\n  {} blank or foreign card", "✗".red());
        // }
        Some(Err(e)) => {
            println!("\n  {} {}", "error:".red().bold(), e);
        }
        None => {}
    }
    println!();
}

fn run_write(app: &mut App) {
    // let tier = match Select::new("Tier:", vec!["Basic", "Vip"]).prompt().unwrap() {
    //     "Vip" => Tier::Vip,
    //     _ => Tier::Basic,
    // };
    // let active = Confirm::new("Active?").with_default(true).prompt().unwrap();
    let dummy_token = Token::default();
    println!("\n  {}", "Tap card to write...".dimmed());

    match app.incoming().next() {
        Some(Ok((id, _))) => match app.write(&RawCard::new(dummy_token)) {
            Ok(_) => {
                println!(
                    "\n  {} written to {}\n",
                    "✓".green().bold(),
                    format!("{}", id).yellow()
                );
            }
            Err(e) => println!("\n  {} {}", "error:".red().bold(), e),
        },
        Some(Err(e)) => println!("\n  {} {}", "error:".red().bold(), e),
        None => {}
    }
}

// fn run_provision(app: &mut App) {
//     let pwd_str = Text::new("Password (8 hex chars):")
//         .with_placeholder("DEADBEEF")
//         .with_validator(|s: &str| {
//             if s.len() == 8 && u32::from_str_radix(s, 16).is_ok() {
//                 Ok(inquire::validator::Validation::Valid)
//             } else {
//                 Ok(inquire::validator::Validation::Invalid(
//                     "Must be exactly 8 hex characters".into(),
//                 ))
//             }
//         })
//         .prompt()
//         .unwrap();
//
//     let pwd = u32::from_str_radix(&pwd_str, 16).unwrap().to_be_bytes();
//     let pack = [0xAB, 0xCD]; // your fixed PACK
//
//     println!("\n  {}", "Tap card to provision...".dimmed());
//
//     match app.incoming().next() {
//         Some(Ok((id, _))) => match app.provision_password(&pwd, &pack) {
//             Ok(_) => println!(
//                 "\n  {} provisioned {}\n",
//                 "✓".green().bold(),
//                 format!("{}", id).yellow()
//             ),
//             Err(e) => println!("\n  {} {}", "error:".red().bold(), e),
//         },
//         Some(Err(e)) => println!("\n  {} {}", "error:".red().bold(), e),
//         None => {}
//     }
// }

pub fn interact() -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx = App::context()?;
    let mut app = App::uart(&mut ctx)?;

    print_banner();
    println!("  {} {}\n", "reader:".dimmed(), app.device_name().cyan());

    loop {
        let action = Select::new(
            "What would you like to do?",
            vec![
                Action::ReadCard,
                Action::WriteCard,
                Action::ProvisionCard,
                Action::Quit,
            ],
        )
        .prompt();

        match action {
            Ok(Action::ReadCard) => run_read(&mut app),
            Ok(Action::WriteCard) => run_write(&mut app),
            Ok(Action::ProvisionCard) => unimplemented!(),
            Ok(Action::Quit) | Err(_) => {
                println!("\n  {}\n", "bye!".dimmed());
                break;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_main() {
        interact().unwrap();
    }
}

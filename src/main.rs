use crate::prelude::*;

pub mod password_file;
pub mod prelude;
pub mod pw_entry;

fn main() -> Result<()> {
    loop {
        let Ok(mut pwf) = open_file() else {
            break
        };
        if let Err(_) = pwf.dec() {
            println!("{}", format!("Incorrect Password!").red());
            pwf.close(false);
            continue
        }
        if menu(&mut pwf)? == false {
            pwf.del()?;
        } else {
            pwf.close(true);
        }
    }

    Ok(())
}

fn menu(pwf: &mut PasswordFile) -> Result<bool> {
    lazy_static! {
        static ref MENU: Vec<&'static str> = vec!["Seek", "Add", "Edit", "Delete", "DELETE FILE"];
    };
    loop {
        let choice = Select::new("What would you like to do?", MENU.to_vec())
            .prompt_skippable()?
            .unwrap_or_default();
        match choice {
            "Seek" => pwf.seek_entry(),
            "Add" => pwf.add_entry(),
            "Edit" => pwf.edit_entry(),
            "Delete Entry" => pwf.delete_entry(),
            "DELETE FILE" => return Ok(false),
            _ => break
        }
    }

    Ok(true)
}

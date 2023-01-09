use aead_io::{ArrayBuffer, DecryptBE32BufReader, EncryptBE32BufWriter};
use chacha20poly1305::ChaCha20Poly1305;

use crate::prelude::*;
use std::collections::BTreeMap;
use std::fs::{remove_file, File};
use std::io::{Read, Seek, Write};

pub struct PasswordFile {
    file: File,
    path: String,
    key: Vec<u8>,
    data: BTreeMap<String, PWEntry>,
}

impl PasswordFile {
    pub fn new(path: String) -> Result<Self> {
        let mut file = File::options()
            .append(true)
            .read(true)
            .write(true)
            .create_new(true)
            .open(&path)?;

        let mut key: Vec<u8> = Password::new("Enter a new password for this password file:")
            .prompt()?
            .into();

        file.write_all(serde_json::to_string(&BTreeMap::<String, PWEntry>::new())?.as_bytes())?;
        file.seek(std::io::SeekFrom::Start(0))?;
        key.resize(32, 0);

        Ok(Self {
            file,
            path,
            key,
            data: BTreeMap::<String, PWEntry>::new(),
        })
    }

    pub fn open(path: String) -> Result<Self> {
        let file = File::options()
            .append(true)
            .read(true)
            .write(true)
            .open(&path)?;
        
        let mut p = Password::new("Enter the password:");
        p.enable_confirmation = false;
        let mut key: Vec<u8> = p.prompt()?.into();
        key.resize(32, 0);

        Ok(Self {
            file,
            path,
            key,
            data: BTreeMap::<String, PWEntry>::new(),
        })
    }

    pub fn seek_entry(&self) {
        let mut pwes = Vec::<PWEntry>::new();
        for entry in &self.data {
            pwes.push(entry.1.clone());
        }

        if pwes.len() == 0 {
            println!("No entries found");
            return
        }

        pwes.sort();
        let options = pwes.iter().map(|e| e.get_name()).collect::<Vec<String>>();
        let choice = Select::new("Select the entry you wish to view:", options)
            .prompt()
            .unwrap_or_default();
        let pwe = self.data.get(&choice).unwrap();

        println!("{}", pwe);
    }

    pub fn add_entry(&mut self) {
        let new = PWEntry::new();
        self.data.insert(new.get_name(), new);
    }

    pub fn edit_entry(&mut self) {
        let mut pwes = Vec::<PWEntry>::new();
        for entry in &self.data {
            pwes.push(entry.1.clone());
        }
        pwes.sort();
        let options = pwes.iter().map(|e| e.get_name()).collect::<Vec<String>>();
        let choice = Select::new("Select the entry you wish to edit:", options)
            .prompt()
            .unwrap_or_default();
        let pwe = self.data.get_mut(&choice).unwrap();

        loop {
            match Select::new(
                "What would you like to do:",
                vec!["Edit Name", "Edit Password"],
            )
            .prompt()
            {
                Ok(s) => match s {
                    "Edit Name" => {
                        let s = Text::new("Enter the new name").prompt().unwrap_or_default();
                        pwe.set_name(s);
                    }
                    "Edit Password" => {
                        let mut p = Password::new("Enter the new password");
                        p.enable_confirmation = false;
                        let s = p.prompt().unwrap_or_default();
                        pwe.set_name(s);
                    }
                    _ => break,
                },
                Err(_) => break,
            }
        }
    }

    pub fn delete_entry(&mut self) {
        let mut pwes = Vec::<PWEntry>::new();
        for entry in &self.data {
            pwes.push(entry.1.clone());
        }
        pwes.sort();
        let options = pwes.iter().map(|e| e.get_name()).collect::<Vec<String>>();
        let choice = Select::new("Select the entry you wish to edit:", options)
            .prompt()
            .unwrap_or_default();
        self.data.remove(&choice);
    }

    fn enc(&mut self) -> Result<Vec<u8>> {
        let mut out = Vec::<u8>::new();
        let key_gen = self.key.as_slice().into();
        let mut text = serde_json::to_vec(&self.data)?;
        {   
            let mut writer = EncryptBE32BufWriter::<ChaCha20Poly1305, _, _>::new(
                key_gen,
                &Default::default(),
                ArrayBuffer::<128>::new(),
                &mut out,
            )?;
            writer.write_all(&mut text)?;
            writer.flush()?;
        }
        Ok(out)
    }

    pub fn dec(&mut self) -> Result<()> {
        let mut cipher = Vec::<u8>::new();
        self.file.read_to_end(&mut cipher)?;
        let key_gen = self.key.as_slice().into();
        let mut data = Vec::<u8>::new();
        {
            let mut reader = DecryptBE32BufReader::<ChaCha20Poly1305, _, _>::new(
                key_gen,
                ArrayBuffer::<128>::new(),
                cipher.as_slice(),
            )?;
            reader.read_to_end(&mut data)?;
        }
        self.data = serde_json::from_slice::<BTreeMap<String, PWEntry>>(&data)?;
        Ok(())
    }

    pub fn del(self) -> Result<()> {
        let mut pass = Password::new("Enter the password for this file")
            .prompt()?
            .as_bytes()
            .to_vec();
        pass.resize(32, 0);
        if pass == self.key {
            if inquire::Confirm::new("Are you sure you want to delete it?").prompt()? {
                remove_file(&self.path)?;
            }
        }
        Ok(())
    }

    pub fn close(mut self, save: bool) {
        if save {
            self.file = File::options()
                .write(true)
                .truncate(true)
                .open(&self.path)
                .unwrap();
            let data = &self.enc().unwrap();
            self.file.write_all(&data).unwrap();
        }
    }
}
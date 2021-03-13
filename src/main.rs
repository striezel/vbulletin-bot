/*
 -------------------------------------------------------------------------------
    This file is part of the vBulletin moderation bot.
    Copyright (C) 2021  Dirk Stolle
    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.
    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.
    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
 -------------------------------------------------------------------------------
*/

use vbulletin_bot::VBulletinApi;

fn main() {
    println!("This is still a proof of concept!");

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 4
    {
        eprintln!("Not enough command line parameters!");
      eprintln!();
        eprintln!("Usage:");
        eprintln!();
        eprintln!("program https://forum.example.com/vb4/ Username SecretPassword [BasicAuthUser BasicAuthPass]");
        std::process::exit(1);
    }

    let base_url = &args[1];
    let user = &args[2];
    let password = &args[3];
    let basic_auth_user = if args.len() > 4 { args[4].clone() } else { String::from("") };
    let basic_auth_pass = if args.len() > 5 { args[5].clone() } else { String::from("") };


    let vb_api = match VBulletinApi::new(&base_url, &basic_auth_user, &basic_auth_pass)
    {
      Ok(api) => api,
      Err(e) =>
      {
        eprintln!("API initialization failed!");
        eprintln!("{}", e);
        std::process::exit(1);
      }
    };

    match vb_api.login(&user, &password)
    {
        Ok(_) => println!("Login as {} was successful!", user),
        Err(e) =>
        {
          eprintln!("Login as {} failed!", user);
          eprintln!("Error: {}", e);
        }
    }

    println!("The show ends here, since more stuff is not implemented yet.");
}

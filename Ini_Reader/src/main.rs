use ini::Ini;

fn main() -> Result<(), Box<dyn std::error::Error>> {
   // load ini file
   let ini_content = std::fs::read_to_string("config/myFile.ini")?;
   let config = Ini::load_from_str(&ini_content)?;

   let uname = config
                  .section(Some("settings"))
                  .and_then(|section| section.get("username"))
                  .unwrap_or("default_user");

   let my_uname: String = uname.to_string();

   ///////////////////////////////////////////////

   let max_number: i32 = config
                        .section(Some("settings"))
                        .and_then(|section| section.get("max_number"))
                        .and_then(|value|value.parse().ok())
                        .unwrap_or(1);

   ///////////////////////////////////////////////

   println!("Username from ini is: {0}\nMax number from ini is: {1}", my_uname, max_number);
   Ok(())

}

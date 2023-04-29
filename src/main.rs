use std::fs::OpenOptions;
use std::io::prelude::*;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct DiscordBuildInfo {
    release_channel: String,
    version: String,
}

fn main() ->std::io::Result<()>{
    /* Updates the discord build info json file to a given version */

    // Grab the desired discord version to update to
    let new_version = std::env::args().nth(1).expect("Error: forgot to pass in new version!");
    println!("Updating Discord to version: {}", new_version);

    // Define the discord build info file
    let mut build_info_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/opt/discord/resources/build_info.json")?;

    // Read the discord build info file into a string
    let mut file_contents = String::new();
    build_info_file.read_to_string(&mut file_contents)?;

    // Parse json string into a struct
    let mut discord_build_info: DiscordBuildInfo = serde_json::from_str(&file_contents)
        .expect("Error parsing build info json...");

    // Update the discord version
    discord_build_info.version = new_version;

    // Temp version of discord build info file
    let mut temp_file = OpenOptions::new()
        .read(true)
        .write(true)
        .truncate(true)
        .open("/opt/discord/resources/build_info.json")?; 

    // Save the updated discord build info file
    let new_json_string = serde_json::to_string(&discord_build_info)?;
    match temp_file.write(&new_json_string.as_bytes()) {
        Ok(_) => {
            println!("Successfully Updated Discord Internals !!!");
            Ok(())
        },
        Err(e) => {
            println!("Error Updating Discord Internals :( !!!");
            println!("Error: {:?}", e);
            Err(e)
        }
    }
}

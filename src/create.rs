/*
    Made by BartuV
    github: https://github.com/BartuV
*/
use serde_json::*;
use colored::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::str::FromStr;
use std::{io, fs};

//Source of this function: https://stackoverflow.com/questions/26958489/how-to-copy-a-folder-recursively-in-rust
fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            if entry.file_name() == "pack.mcmeta"{
                continue
            }else {
                fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
            }
        }
    }
    Ok(())
}

pub fn create(packversion:i32, location:String,packname:String){
    let path = Path::new(&location);
    let example = Path::new(r"src\examplepack");
    let copyres = copy_dir_all(example, path);
    if copyres.is_ok(){println!("{} {}", "Sucsessfully copied to".to_owned().green().bold() , &location.green().bold());}else{println!("{} {} {}","Failed to copy".to_owned().red().bold().to_string() ,&location.red().bold(),format!("{:?}",copyres.err()).red().bold());}    

    let packmeta = path.as_os_str().to_str().unwrap().to_string();
    edit_mcpack(&packmeta, packversion, &packname);
    rename_pack(&location, &packname.to_string());
    generate_json(&location, "tick.json".to_string(), &packname,r#"{"values": "place_holder:tick"}"#.to_string());
    generate_json(&location, "load.json".to_string(), &packname,r#"{"values": "place_holder:load"}"#.to_string());
    generate_info(&location, &packname);
}

//generatin pack.mcmeta
fn edit_mcpack(filepath:&String,version:i32,description:&String){
    //Spesifying content of pack.mcmeta
    let content = r#"
    {
        "pack": {
          "pack_format": 9,
          "description": "Tutorial Data Pack"
        }
    }
    "#;

    //Reading the content and editing it
    let mut res = serde_json::Value::from_str(content).expect("Can't read the file");
    let mut map = serde_json::Map::new();
    map.insert("pack_format".to_string(), json!(version));
    map.insert("description".to_string(), json!(description));
    res["pack"] = serde_json::Value::Object(map);

    //creating and writing to the file
    let mut file = File::create(filepath.to_string()+&r"\pack.mcmeta".to_string()).expect("Error");
    file.write_all(res.to_string().as_bytes()).expect("Error");
    
}

fn rename_pack(filepath: &String,packname:&String){
    let placeholderpath = filepath.to_string()+r"\data\place_holder";

    //Renaming the file and getting the result
    let res = fs::rename(placeholderpath, filepath.to_string()+r"\data\"+&packname.to_string());
    if res.is_err(){println!("{} {}","Something gone wrong.".red().bold(),format!("{:?}",res.err()).red().bold())}
}

//generating file
fn generate_json(path:&String,filename:String,name:&String,content:String){
    //Spagetti mess but it works!
    let mut file = File::create(path.to_string()+&r"\data\minecraft\tags\functions\".to_string()+&filename.to_string()).expect("Something went wrong.");
    let mut res1 = Value::from_str(&content).expect("Oops. Something went wrong.");

    //Editing and generating the file
    res1["values"] = Value::Array(vec![Value::String(res1["values"].to_string().replace('"', "").replace("place_holder",&name))]);
    file.write_all(res1.to_string().replace(r"\","").as_bytes()).expect(format!("{}","Something went wrong.".red().bold()).as_str()); 
}

fn generate_info(path:&String,packname:&String){
    let content = r#"
    {
        "functionfolder":"/data/place_holder/functions"
    }
    "#;
    let mut file = File::create(path.to_owned()+r"\packinfo.json").expect("Something went wrong.");
    let mut res1 = Value::from_str(&content).expect("Oops. Something went wrong.");
    
    res1["functionfolder"] = json!(res1["functionfolder"].to_string().replace('"',"").replace("place_holder",packname));
    file.write_all(res1.to_string().as_bytes()).expect(format!("{}","Something went wrong.".red().bold()).as_str());
}
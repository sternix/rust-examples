// https://stackoverflow.com/questions/59452193/difficulties-deserializing-xml-using-rust-and-serde-where-document-has-optional

/*
[dependencies]
quick-xml = { version = "0.22", features = [ "serialize" ] }
serde = { version = "1.0", features = [ "derive" ] }
*/

use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct DEFECTS {
    #[serde(rename = "DEFECT", default)]
    pub defects: Vec<DEFECT>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct DEFECT {
    #[serde(default)]
    pub SFA: SFA,
    pub DEFECTCODE: String,
    pub DESCRIPTION: String,
    pub FUNCTION: String,
    pub DECORATED: String,
    pub FUNCLINE: String,
    pub PATH: PATH,
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct SFA {
    pub FILEPATH: String,
    pub FILENAME: String,
    pub LINE: String,
    pub COLUMN: String,
}

#[derive(Default, Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct PATH {
    SFA: Option<SFA>, // burası Option, yani opsiyonel
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Invalid argument count. Specify a single file to process.");
    }

    let processing_file = &args[1];
    println!("Will attempt to process file: '{}'", &processing_file);

    // Try to load the contents of the file
    let file_content: String = match std::fs::read_to_string(&processing_file) {
        Ok(file_content) => file_content,
        Err(e) => {
            panic!("Failed to read file: '{}' -- {}", &processing_file, e);
        }
    };

    // Now, try to deserialize the XML we have in file_content
    let defect_list: DEFECTS = from_str(&file_content).unwrap();

    // Assuming the unwrap above didn't blow up, we should get a count here
    println!(
        "Retrieved {} defects from file '{}'",
        defect_list.defects.len(),
        &processing_file
    );
}

//  örnek xml dosyası
/*

<?xml version="1.0" encoding="utf-8"?>
<DEFECTS>
  <DEFECT>
    <SFA>
      <FILEPATH>c:\projects\source\repos\defecttest\defecttest</FILEPATH>
      <FILENAME>source.cpp</FILENAME>
      <LINE>8</LINE>
      <COLUMN>5</COLUMN>
    </SFA>
    <DEFECTCODE>26496</DEFECTCODE>
    <DESCRIPTION>The variable 'y' is assigned only once, mark it as const (con.4).</DESCRIPTION>
    <FUNCTION>main</FUNCTION>
    <DECORATED>main</DECORATED>
    <FUNCLINE>6</FUNCLINE>
    <PATH></PATH>
  </DEFECT>
  <DEFECT>
    <SFA>
      <FILEPATH>c:\projects\source\repos\defecttest\defecttest</FILEPATH>
      <FILENAME>source.cpp</FILENAME>
      <LINE>9</LINE>
      <COLUMN>5</COLUMN>
    </SFA>
    <DEFECTCODE>26496</DEFECTCODE>
    <DESCRIPTION>The variable 'z' is assigned only once, mark it as const (con.4).</DESCRIPTION>
    <FUNCTION>main</FUNCTION>
    <DECORATED>main</DECORATED>
    <FUNCLINE>6</FUNCLINE>
    <PATH>
      <SFA>
        <FILEPATH>c:\projects\source\repos\defecttest\defecttest</FILEPATH>
        <FILENAME>source.cpp</FILENAME>
        <LINE>12</LINE>
        <COLUMN>3</COLUMN>
      </SFA>
    </PATH>
  </DEFECT>
</DEFECTS>

*/

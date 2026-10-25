use std::fs::File;
use std::io::Write;

fn main(){
    let header = vec!["Student Name","Matric.Number","Department","Level"];
    let row_1 = vec!["Oluchi Mordi","ACC10211111","Accounting","300"];
    let row_2 = vec!["Adams Aliyu","ECO10110101","Economics","100"];
    let row_3 = vec!["Shania Bolade","CSC10328828","Computer","200"];
    let row_4 = vec!["Adekunle Gold","EEE11020202","Electrical","200"];
    let row_5 = vec!["Blanca Edemoh","MEE10202001","Mechanical","100"];
    
    let mut file = File::create("PAU-SIMS.csv").expect("Failed");
    let line_1 = format!("{},{},{},{}\n",header[0],header[1],header[2],header[3]);
    file.write_all(line_1.as_bytes()).expect("Failed");
    let line_2 = format!("{},{},{},{}\n",row_1[0],row_1[1],row_1[2],row_1[3]);
    file.write_all(line_2.as_bytes()).expect("Failed");
    let line_3 = format!("{},{},{},{}\n",row_2[0],row_2[1],row_2[2],row_2[3]);
    file.write_all(line_3.as_bytes()).expect("Failed");
    let line_4 = format!("{},{},{},{}\n",row_3[0],row_3[1],row_3[2],row_3[3]);
    file.write_all(line_4.as_bytes()).expect("Failed");
    let line_5 = format!("{},{},{},{}\n",row_4[0],row_4[1],row_4[2],row_4[3]);
    file.write_all(line_5.as_bytes()).expect("Failed");
    let line_6 = format!("{},{},{},{}\n",row_5[0],row_5[1],row_5[2],row_5[3]);
    file.write_all(line_6.as_bytes()).expect("Failed");
    println!("SUCCESFUL");    
}
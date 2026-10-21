use std::io::Write;

fn main(){
    let mut header = "Lager,Stout,Non Alcoholic\n";
    let mut row_1 = "33 Export,Legend,Maltina\n";
    let mut row_2 = "Desperados,Turbo_King, Amstel malta\n";
    let mut row_3 = "Goldberg,Williams,Malta Gold\n";
    let mut row_4 = "Gulder,,Fayrouz\n";
    let mut row_5 = "Heineken,,\n";
    let mut row_6 = "Star,,\n";

    let mut file = std::fs::File::create("data.csv").expect("create failed");
    file.write_all("Nigerian Breweries Plc\n".as_bytes()).expect("write failed");
    file.write_all(header.as_bytes()).expect("write failed");
    file.write_all(row_1.as_bytes()).expect("write failed");
    file.write_all(row_2.as_bytes()).expect("write failed");
    file.write_all(row_3.as_bytes()).expect("write failed");
    file.write_all(row_4.as_bytes()).expect("write failed");
    file.write_all(row_5.as_bytes()).expect("write failed");
    file.write_all(row_6.as_bytes()).expect("write failed");

    println!("Your Project has been done");
   

}
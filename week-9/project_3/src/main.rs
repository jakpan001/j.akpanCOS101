use std::fs::File;
use std::io::Write;

fn main(){
    let commisioners = vec!["Aigbogun Alamba Daudu","Murtala Afeez Bendu","Okorocha Calistus Ogbona","Adewale Jimoh Akanbi","Osazuwa Faith Etieye"];
    let ministry = vec!["Internal Affairs","Justice","Defense","Power & Steel","Petroleum"];
    let zone = vec!["South west","North east","South south","South west","South east"];
    let headers = vec!["Commisioner","Ministry","Zone"];

    let mut merge  = File::create("EFCC-Merge.csv").expect("Failed");
    let header = format!("{},{},{}\n",headers[0],headers[1],headers[2]);
    merge.write_all(header.as_bytes()).expect("Failed");
    let line_1 = format!("{},{},{}\n",commisioners[0],ministry[0],zone[0]);
    merge.write_all(line_1.as_bytes()).expect("Failed");
    let line_2 = format!("{},{},{}\n",commisioners[1],ministry[1],zone[1]);
    merge.write_all(line_2.as_bytes()).expect("Failed");
     let line_3 = format!("{},{},{}\n",commisioners[2],ministry[2],zone[2]);
    merge.write_all(line_3.as_bytes()).expect("Failed");
     let line_4 = format!("{},{},{}\n",commisioners[3],ministry[3],zone[3]);
    merge.write_all(line_4.as_bytes()).expect("Failed");
     let line_5 = format!("{},{},{}\n",commisioners[4],ministry[4],zone[4]);
    merge.write_all(line_5.as_bytes()).expect("Failed");
    println!("SUCCESSFUL");
}
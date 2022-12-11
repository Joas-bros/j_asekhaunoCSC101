use std::io::Write;

fn main() {
    let larger = vec!["33 Export","Desperados","Goldberg","Gulder","Heineken","Star"];
    let stout = vec!["Legend","Turbo King","Willams","--","--","--"];
    let nonalc = vec!["Maltina","Amstel Malta","Malta Gold","fayrouz","--","--"];

    let mut file = std::fs::File::create("Nigerian Brewery Limited.txt").expect("create failed");

    for i in 0..larger.len() {
    file.write_all(larger[i].as_bytes()).expect("write failed");
    file.write_all(stout[i].as_bytes()).expect("write failed");
    file.write_all(nonalc[i].as_bytes()).expect("write failed");
    print!("\nLarger: {}\nStout: {}\nNon-Alchoholic: {}\n",larger[i],stout[i],nonalc[i]);
    }

    
}

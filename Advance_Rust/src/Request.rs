use reqwest::blocking::Client;

pub fn Request(){
   let http_client= Client::new();
   let http_resutl=http_client.get("https://catfact.ninja/fact").send();
   if http_resutl.is_ok(){
    println!("{:?}",http_resutl.ok().unwrap().content_length().unwrap());
   }else{
    println!("{:?}",http_resutl.err());
   }
//    let http_Post=http_client.post("https://catfact.ninja/fact").body("my name is Devansh").send();
}
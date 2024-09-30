use serde::{Deserialize,Serialize};
use serde_json::to_string;
#[derive(Serialize,Deserialize)]
struct Animal{
    name:String,
    year_born:i32,
}
pub fn Serde() {
   let dog01:Animal=Animal{
    name:"Tommy".to_string(),
    year_born:2011
   };
   let dog_serialize=to_string(&dog01);
   if dog_serialize.is_ok(){
    println!("{}",dog_serialize.ok().unwrap());
   }else{
    println!("{:?}",dog_serialize.err());
   }
}
struct Person<PetType: Animal> {
    first_name: String,
    pet: PetType,
}
trait Animal {
    fn make_sound(&self) -> ();
}

struct Dog {}
impl Animal for Dog {
    fn make_sound(&self) -> () {
        println!("Bark");
    }
}
struct Cat {}
impl Animal for Cat {
    fn make_sound(&self) -> () {
        println!("Meow");
    }
}

pub fn create_person() {
    let pet1: Dog = Dog {};
    let p1: Person<Dog> = Person {
        first_name: "Devansh".to_string(),
        pet: pet1,
    };
    println!(
        "THe name of firstPerson is : {} and pet made sound {:?}",
        p1.first_name,
        p1.pet.make_sound()
    );
}

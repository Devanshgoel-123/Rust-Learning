pub mod vectors_learning {

    pub fn test_vec_int() {
        let mut my_ints: Vec<i32> = Vec::new(); //necessary to specify type so rust knows what type is stored here
        let _my_ints2 = vec![1, 2, 3, 4, 5];
        my_ints.push(2);
        my_ints.push(3);

        println!("THe number is : {}", &my_ints[0]); // One way to get the number
        println!("THe number is : {:?}", my_ints.get(0));
        println!("The capacity of vector is :{}", my_ints.capacity());
    }

    pub fn test_vec_string() {
        let names = vec!["Devansh", "Goel", "Aditya", "Random"];
        for name in names {
            println!("The name of the person is : {}", name);
        }
    }
    #[derive(Debug)]
    struct Car {
        manufacturer: String,
        model: String,
    }

    pub fn test_vec_car() {
        let mut my_car = vec![Car {
            manufacturer: "Maruti".to_string(),
            model: "Ertiga".to_string(),
        }];
        println!("My car type is : {:?}", my_car.get(0).unwrap().manufacturer);
        let value_removed = &my_car.remove(0);
        println!("{:?}", value_removed);
    }
}

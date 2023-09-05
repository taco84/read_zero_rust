trait Location {
    fn address(&self) -> &str;
}

trait Person {
    fn name(&self) -> &str;
}

trait House: Location + Person {}

fn print_house_info(house: &dyn House) {
    println!("所有者: {}",house.name());
    println!("住所： {}",house.address());
}

struct MyHouse {
    owner: String,
    address: String,
}

impl Location for MyHouse {
    fn address(&self) -> &str {
        &self.address
    }
}

impl Person for MyHouse {
    fn name(&self) -> &str{
        &self.owner
    }
}

impl House for MyHouse {}

fn main() {
    let my_house = MyHouse {
        owner: "かぐや姫".to_string(),
        address: "ムーンベース3丁目1番地".to_string(),
    };

    print_house_info(&my_house);
}

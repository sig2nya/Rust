pub struct Student {
    id: i32, // private
    pub name: String, // public
    pub email: String, // public
}

impl Student {
    pub fn new(id: i32, name: String, email: String) -> Student {
        Student { id, name, email }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    fn set_name(&mut self, name: String) {
        self.name = name.clone();
    }
}

fn main() {
    let student = Student::new(1, String::from("luna"), String::from("luna@email.me"));
    println!("Name : {}", student.get_name());
}

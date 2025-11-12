mod network {
    fn connect() {} // private
}

mod network {
    pub fn connect() {} // public
}

mod ineternal_api {
    pub(crate) fn process() {}
}

mod parent {
    mod child {
        pub(super) fn hi() { println!("Hi!"); }
    }

    pub fn call_child() {
        child::hi(); // Ok
    }
}

mod a {
    pub mod b {
        pub(in crate::a) fn restricated() {}
    }

    pub fn test() {
        b::restricted();
    }
}

fn main() {
    network::connect();
    parent::call_child();
    parent::child::hi();
}

use ezaccess::Getters;

mod pog {
    pub mod hehe {
        pub type Wooooooooooooooow = String;
    }
}

#[derive(Getters)]
struct Banana {
    #[get]
    pub public_field: pog::hehe::Wooooooooooooooow,
}

fn main() {
    let pog = Banana {
        public_field: "test".to_string(),
    };

    pog.get_public_field();
}

use getset::Getters;

#[allow(dead_code)]
mod example {
    struct Example {
        pub public_field: String,
        private_field: String,
        hidden_field: String,
    }

    impl Example {
        fn get_public_field(&self) -> &String {
            &self.public_field
        }

        fn set_public_field(&mut self, value: String) {
            self.public_field = value
        }

        fn private_field_mut(&mut self) -> &mut String {
            &mut self.private_field
        }
    }

    fn _example() {
        let mut example = Example {
            public_field: "this is a public field".to_string(),
            private_field: "this is a private field".to_string(),
            hidden_field: "this should be hidden".to_string(),
        };

        (*example.private_field_mut()) = "hehe".to_string();
    }
}

#[derive(Getters)]
struct Example {
    pub public_field: String,
    private_field: String,
}

#[derive(Getters)]
struct Dunno<T> {
    some_field: T,
}

fn main() {
    let example = Example {
        public_field: "private public".to_string(),
        private_field: "private value".to_string(),
    };

    let hehe = Dunno { some_field: "pog" };
    hehe.get_some_field();

    example.get_public_field();
}

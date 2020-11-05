#[cfg(test)]
mod hello_world;

#[cfg(test)]
mod tests {
    use super::hello_world;
    use hello_world::HelloWorldContract;

    #[test]
    fn should_run() {
        const KEY_NAME: &str = "special_value";
        let hello_world = HelloWorldContract::deploy();
        let check: String = hello_world.query_contract(&KEY_NAME).unwrap();
        let value: String = String::from("hello world");
        assert_eq!(value, check);
    }

    #[test]
    fn should_update() {
        const KEY_NAME: &str = "special_value";
        let mut hello_world = HelloWorldContract::deploy();
        let new_string = String::from("Goodbye friend");
        hello_world.call_update(new_string);
        let check: String = hello_world.query_contract(&KEY_NAME).unwrap();
        assert_eq!(check, String::from("Goodbye friend"))
    }
}
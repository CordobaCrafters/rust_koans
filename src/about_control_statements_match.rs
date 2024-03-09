#[cfg(test)]
mod tests {
    #[test]
    fn test_match_statement() {
        fn tell_me_about(number: i32) -> String {
            match number {
                // Match a single value
                1 => format!("One!"),
                // Match several values
                2 | 3 | 5 | 7 | 11 => format!("This is a prime"),
                // TODO ^ Try adding 13 to the list of prime values
                // Match an inclusive range
                13..=19 => format!("A teen"),
                // Handle the rest of cases
                _ => format!("Ain't special"),
                // TODO ^ Try commenting out this catch-all arm
            }
        }

        assert_eq!("", tell_me_about(1));
        assert_eq!("", tell_me_about(13));
        assert_eq!("", tell_me_about(19));
        assert_eq!("", tell_me_about(20));
    }

    #[test]
    fn test_match_destructuring_tuples() {
        fn tell_me_about(triple: (i32, i32, i32)) -> String {
            // Match can be used to destructure a tuple
            match triple {
                // Destructure the second and third elements
                (0, y, z) => format!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
                (1, ..) => format!("First is `1` and the rest doesn't matter"),
                (.., 2) => format!("last is `2` and the rest doesn't matter"),
                (3, .., 4) => format!("First is `3`, last is `4`, and the rest doesn't matter"),
                // `..` can be used to ignore the rest of the tuple
                _ => format!("It doesn't matter what they are"),
                // `_` means don't bind the value to a variable
            }
        }

        assert_eq!("", tell_me_about((0, -2, 3)));
        assert_eq!("", tell_me_about((1, -2, 3)));
        assert_eq!("", tell_me_about((10, -2, 2)));
        assert_eq!("", tell_me_about((3, -2, 4)));
        assert_eq!("", tell_me_about((10, -2, 3)));
    }

    #[test]
    fn test_match_destructuring_arrays() {
        fn tell_me_about(array: [i32; 3]) -> String {
            match array {
                // Binds the second and the third elements to the respective variables
                [0, second, third] => {
                    format!("array[0] = 0, array[1] = {}, array[2] = {}", second, third)
                }

                // Single values can be ignored with _
                [1, _, third] => format!(
                    "array[0] = 1, array[2] = {} and array[1] was ignored",
                    third
                ),

                // You can also bind some and ignore the rest
                [2, second, ..] => format!(
                    "array[0] = 2, array[1] = {} and all the other ones were ignored",
                    second
                ),
                // The code below would not compile
                // [2, second] => ...

                // Or store them in another array/slice (the type depends on
                // that of the value that is being matched against)
                [3, second, tail @ ..] => format!(
                    "array[0] = 3, array[1] = {} and the other elements were {:?}",
                    second, tail
                ),

                // Combining these patterns, we can, for example, bind the first and
                // last values, and store the rest of them in a single array
                [first, middle @ .., last] => format!(
                    "array[0] = {}, middle = {:?}, array[2] = {}",
                    first, middle, last
                ),
            }
        }

        // Try changing the values in the array, or make it a slice!
        assert_eq!("", tell_me_about([0, -2, 6]));
        assert_eq!("", tell_me_about([1, -2, 6]));
        assert_eq!("", tell_me_about([2, -2, 6]));
        assert_eq!("", tell_me_about([3, -2, 6]));
        assert_eq!("", tell_me_about([4, -2, 6]));
    }

    #[test]
    fn test_match_destructuring_enums() {
        enum Beverage {
            Water,
            Coffee(String),
            Wine(String, u32),
        }

        fn favorite_beverage(beverage: Beverage) -> String {
            match beverage {
                Beverage::Water => format!("Favorite water"),
                Beverage::Coffee(flavor) => format!("Favorite coffee: {}", flavor),
                Beverage::Wine(grape, year) => format!("Favorite wine: {}, {}", grape, year),
                // Don't need another arm because all variants have been examined
            }
        }

        let water = Beverage::Water;
        assert_eq!("", favorite_beverage(water));
        let coffee = Beverage::Coffee("Arabica".to_string());
        assert_eq!("", favorite_beverage(coffee));
        let wine = Beverage::Wine("Tinta".to_string(), 2018);
        assert_eq!("", favorite_beverage(wine));
    }

    #[test]
    fn test_match_destructuring_structs() {
        struct Foo {
            x: (u32, u32),
            y: u32,
        }

        fn tell_me_about(foo: Foo) -> String {
            match foo {
                Foo { x: (1, b), y } => format!("First of x is 1, b = {},  y = {} ", b, y),

                // you can destructure structs and rename the variables,
                // the order is not important
                Foo { y: 2, x: i } => format!("y is 2, i = {:?}", i),

                // and you can also ignore some variables:
                Foo { y, .. } => format!("y = {}, we don't care about x", y),
                // this will give an error: pattern does not mention field `x`
                //Foo { y } => format!("y = {}", y),
            }
        }

        // Try changing the values in the struct to see what happens
        assert_eq!("", tell_me_about(Foo { x: (1, 2), y: 3 }));
        assert_eq!("", tell_me_about(Foo { x: (10, 2), y: 2 }));
        assert_eq!("", tell_me_about(Foo { x: (10, 2), y: 3 }));
    }

    #[test]
    fn test_match_guards() {
        enum PositiveNumber {
            Integer(u32),
        }

        fn classify_number(number: PositiveNumber) -> String {
            match number {
                PositiveNumber::Integer(x) if x % 2 == 0 => format!("{} is an even integer.", x),
                PositiveNumber::Integer(x) if x % 2 != 0 => format!("{} is an odd integer.", x),
                _ => {
                    unreachable!("The compiler ignores guard conditions in match pattern coverage.")
                }
            }
        }

        assert_eq!("", classify_number(PositiveNumber::Integer(1)));
        assert_eq!("", classify_number(PositiveNumber::Integer(2)));
    }

    #[test]
    fn test_match_bindings() {
        fn temperature_description(temperature: i32) -> String {
            match temperature {
                value if value < 0 => "It's freezing!".to_string(),
                value @ 0..=30 => format!("It's a cool day at {}°C.", value),
                value @ 31..=45 => format!("It's hot at {}°C.", value),
                value => format!("It's extremely hot at {}°C.", value),
            }
        }

        assert_eq!("", temperature_description(-1));
        assert_eq!("", temperature_description(0));
        assert_eq!("", temperature_description(31));
        assert_eq!("", temperature_description(46));
    }

    #[test]
    fn test_match_bindings_to_destructuring_enums() {
        enum PositiveNumber {
            Integer(u32),
        }

        fn classify_number(number: PositiveNumber) -> String {
            match number {
                PositiveNumber::Integer(x @ 0) => format!("{} is a zero!", x),
                PositiveNumber::Integer(x) if x % 2 == 0 => format!("{} is an even integer.", x),
                PositiveNumber::Integer(x) if x % 2 != 0 => format!("{} is an odd integer.", x),
                _ => {
                    unreachable!("The compiler ignores guard conditions in match pattern coverage.")
                }
            }
        }

        assert_eq!("", classify_number(PositiveNumber::Integer(0)));
    }
}

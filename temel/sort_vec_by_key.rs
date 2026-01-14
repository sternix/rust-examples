struct City {
    name: String,
    population: i64,
}

// city_population_descending, takes a City record and extracts the key, the field by which we want to sort our data.

// it returns a negative number because sort arranges numbers
// in increasing order, and we want decreasing order: the most populous city first

// The sort_by_key method takes this key-function as a parameter.

fn city_population_descending(city: &City) -> i64 {
    -city.population
}

fn sort_cities(cities: &mut Vec<City>) {
    cities.sort_by_key(city_population_descending);
}

fn main() {
    let mut cities = vec![
        City {
            name: "A".to_string(),
            population: 1,
        },
        City {
            name: "B".to_string(),
            population: 2,
        },
        City {
            name: "C".to_string(),
            population: 3,
        },
    ];
    sort_cities(&mut cities);

    for city in &cities {
        println!("{}", city.name);
    }
}

/*

This works fine, but it's more concise to write the helper function as a closure, an anonymous function expression:

fn sort_cities(cities: &mut Vec<City>) {
    cities.sort_by_key(|city| -city.population);
}

the closure here is |city| -city.population
it takes an argument city and returns -city.population. Rust infers the argument type and return type from how the closure is used.

*/

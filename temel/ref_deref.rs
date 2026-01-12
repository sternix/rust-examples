fn main() {
    struct Anime {
        name: &'static str,
    }

    let aria = Anime {
        name: "Aria: The Animation",
    };

    let anime_ref = &aria;
    assert_eq!(anime_ref.name, "Aria: The Animation");

    // Equivalent to the above, but with the dereference written out:
    assert_eq!((*anime_ref).name, "Aria: The Animation");
}

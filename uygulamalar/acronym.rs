// https://github.com/olgZZZ/100-days-of-code/tree/master/Round1/R1D55/acronym

fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_whitespace() || c == '-' || c == '_')
        .flat_map(|word| {
            word.chars().take(1).chain(
                word.chars()
                    .skip_while(|c| c.is_uppercase())
                    .filter(|c| c.is_uppercase()),
            )
        })
        .collect::<String>()
        .to_ascii_uppercase()
}

fn main() {
    assert_eq!(abbreviate("Portable Network Graphics"), "PNG");
    assert_eq!(abbreviate("Ruby on Rails"), "ROR");
    assert_eq!(abbreviate("HyperText Markup Language"), "HTML");
    assert_eq!(abbreviate("First In, First Out"), "FIFO");
    assert_eq!(abbreviate("GNU Image Manipulation Program"), "GIMP");
    assert_eq!(abbreviate("PHP: Hypertext Preprocessor"), "PHP");
    assert_eq!(
        abbreviate("Complementary metal-oxide semiconductor"),
        "CMOS"
    );
    assert_eq!(
        abbreviate("Rolling On The Floor Laughing So Hard That My Dogs Came Over And Licked Me"),
        "ROTFLSHTMDCOALM"
    );
    assert_eq!(abbreviate("Something - I made up from thin air"), "SIMUFTA");
    assert_eq!(abbreviate("Halley's Comet"), "HC");
    assert_eq!(abbreviate("The Road _Not_ Taken"), "TRNT");
}

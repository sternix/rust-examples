fn main() {
    let message = "1sdfgdgd dsgdfgds\
                   2dfgdgdfgd\r\nds fhfghdfghfdgh\
                   3\r\ndgdsgdsfgds\
                   4dsgdsfgdsgdsfgds\r\n\
                   \r\n\
                   5dsfgdsfgds\
                   6\r\ndgfdsfgdsgf\
                   7dsfgdfgdgdfg\
                   8dfgdfgdfgdff\r\n";

    for header in message.lines().take_while(|l| !l.is_empty()) {
        println!("{}", header);
    }
}

/*
4. satırdan sonrasını almıyor
The take_while adapter terminates the iteration as soon as it sees that blank line.
*/

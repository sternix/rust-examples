/*

Since push and pop need to modify StrQueue, they both take &mut self. However, when you call a method,
you don't need to borrow the mutable reference yourself; the ordinary method call syntax takes care of that implicity
Simple writing q.push() borrows a mutable reference to q, as if you had written (&mut q).push() since that's what the push method's self requires.

Eğer bir metod parametre olarak self alıyorsa o tipin sahipliğini alıyordur.
bu metodu kullandıktan sonra nesne  tanımlanmamış olur.

The fact that any type can have methods is one reason,
Rust doesn't use the term object much preferring to call everything a value

*/

struct StrQueue {
    v: Vec<String>,
}

impl StrQueue {
    fn new() -> Self {
        StrQueue { v: Vec::new() }
    }

    fn push(&mut self, s: String) {
        self.v.push(s);
    }

    fn pop(&mut self) -> String {
        self.v.pop().unwrap()
    }
}

fn main() {
    let mut q = StrQueue::new();
    q.push("aaa".to_string());
    println!("{}", q.pop());

    // yukarıdaki ile aynı yukarıdaki'ler implicit olarak aynı işi yapıyor
    (&mut q).push("bbb".to_string());
    println!("{}", (&mut q).pop());
}

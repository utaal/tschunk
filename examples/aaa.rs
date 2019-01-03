extern crate tschunk;

tschunk::dothing! {
    struct AAA {
        v: u32,
    }
}

fn main() {
    let a = AAA {
        v: 32,
    };

}

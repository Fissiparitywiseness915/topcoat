fn main() {
    let names = ["carl", "julien", "joey"];

    let content = topcoat::view! {
        html {
            head {
                title { "hello world" }
            }
            body {
                for name in names {
                    div {
                        "hello " (name)
                    }
                }
            }
        }
    };
    println!("{}", content);
}

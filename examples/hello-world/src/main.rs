fn main() {
    let name = "carl";

    let content = topcoat::view! {
        div {
            match name {
                "carl" => b { "hi carl" },
                "joey" => b { "hi joey" },
                "julien" => b { "im julien" },
                _ => {},
            }
        }
    };

    println!("{}", content);
}

fn primeira_palavra(s: &str) -> &str {
    if s.starts_with(' ') {
        return "";
    }

    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s
}

fn main() {
    let frases = vec!["hello world", " rust", "rust", "", "   "];
    
    for frase in &frases {
        let palavra = primeira_palavra(frase);
        println!("Frase: '{}', Primeira palavra: '{}', Tamanho original: {}", frase, palavra, frase.len());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_tests() {
        assert_eq!(primeira_palavra("hello world"), "hello");
        assert_eq!(primeira_palavra(" rust"), "");
        assert_eq!(primeira_palavra("rust"), "rust");
        assert_eq!(primeira_palavra(""), "");
        assert_eq!(primeira_palavra("   "), "");
    }
}


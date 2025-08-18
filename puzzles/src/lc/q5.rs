use stdext::prelude::*;

fn is_palindrome(s: &String) -> bool {
    match s.len() {
        0 | 1 => true,
        _ => s.chars().nth(0) == s.chars().nth(s.len() - 1)
            && is_palindrome(&s.substr(1, s.len() - 1))
    }
}

pub fn answer(s: String) -> String {
    let mut longest = "".to_string();
    for i in 0..s.len() {
        for j in i + 1..=s.len() {
            if j - i > longest.len() {
                let substr = s.substr(i, j).to_string();
                if is_palindrome(&substr) {
                    longest = substr;
                }
            }
        }
    }
    longest
}

pub fn test() {
    // answer("babad".to_string()).must_be("bab");
    // answer("cbbd".to_string()).must_be("bb");
    // answer("a".to_string()).should_be("a").log_err().discard();

    answer("kyyrjtdplseovzwjkykrjwhxquwxsfsorjiumvxjhjmgeueafubtonhlerrgsgohfosqssmizcuqryqomsipovhhodpfyudtusjhonlqabhxfahfcjqxyckycstcqwxvicwkjeuboerkmjshfgiglceycmycadpnvoeaurqatesivajoqdilynbcihnidbizwkuaoegmytopzdmvvoewvhebqzskseeubnretjgnmyjwwgcooytfojeuzcuyhsznbcaiqpwcyusyyywqmmvqzvvceylnuwcbxybhqpvjumzomnabrjgcfaabqmiotlfojnyuolostmtacbwmwlqdfkbfikusuqtupdwdrjwqmuudbcvtpieiwteqbeyfyqejglmxofdjksqmzeugwvuniaxdrunyunnqpbnfbgqemvamaxuhjbyzqmhalrprhnindrkbopwbwsjeqrmyqipnqvjqzpjalqyfvaavyhytetllzupxjwozdfpmjhjlrnitnjgapzrakcqahaqetwllaaiadalmxgvpawqpgecojxfvcgxsbrldktufdrogkogbltcezflyctklpqrjymqzyzmtlssnavzcquytcskcnjzzrytsvawkavzboncxlhqfiofuohehaygxidxsofhmhzygklliovnwqbwwiiyarxtoihvjkdrzqsnmhdtdlpckuayhtfyirnhkrhbrwkdymjrjklonyggqnxhfvtkqxoicakzsxmgczpwhpkzcntkcwhkdkxvfnjbvjjoumczjyvdgkfukfuldolqnauvoyhoheoqvpwoisniv".to_string()).log();
}
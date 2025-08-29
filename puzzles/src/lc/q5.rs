// https://leetcode.com/problems/longest-palindromic-substring/
use std::cmp::{max, min};
use stdext::prelude::*;

fn is_palindrome(s: &String) -> bool {
    match s.len() {
        0 | 1 => true,
        _ => s.chars().nth(0) == s.chars().nth(s.len() - 1)
            && is_palindrome(&s.substr(1, s.len() - 1))
    }
}

// pub fn answer(s: String) -> String {
//     let mut longest = "".to_string();
//     for i in 0..s.len() {
//         for j in i + 1..=s.len() {
//             if j - i > longest.len() {
//                 let substr = s.substr(i, j).to_string();
//                 if is_palindrome(&substr) {
//                     longest = substr;
//                 }
//             }
//         }
//     }
//     longest
// }

pub fn answer(s: String) -> String {
    let mut start = 0i32;
    let mut end = 0i32;
    let s_len = s.len() as i32;
    let mut longest = s.chars().nth(0).unwrap().to_string();
    loop {
        let substring = s.substr(max(0, start as usize), min((end + 1) as usize, s_len as usize));
        if is_palindrome(&substring) {
            if substring.len() > longest.len() {
                longest = substring;
            }
            start = max(0, start - 1);
            end += 1;
        } else {
            start = min(end, start + 1);
        }
        if end >= s_len {
            break;
        }
    }
    longest
}

pub fn test() {
    // answer("babad".to_string()).must_be("bab");
    // answer("babab".to_string()).must_be("babab");
    // answer("cbbd".to_string()).must_be("bb");
    // answer("a".to_string()).must_be("a");
    answer("raedvmtyxveocfyhluuozodpxlroyjcsfslqmjthsbxhteeinpmnejxxcsyjgugclkehagpemfrnqtrkiropblcqdboztxtsaxqnsktrhzelynbzkxcghhfntrdauyzhzgujhniazijshesigzckgxentepeohcltpydumougjkmgoscchqsryaiamoujnyfpcsbwqtwikedbsjxxtnrpfgeqymwfngixslwlifimdapgzanuqwhwpesaigeoiwoyxzjmxukbsvsjvnjhwdbqzuurfolcngefdpsewrpvwivrsjnttrewkytdvvguatidyemrswpdmeqjrxgfdmcdlrcgiqdkyaaykdqigcrldcmdfgxrjqemdpwsrmeyditaugvvdtykwerttnjsrviwvprwespdfegnclofruuzqbdwhjnvjsvsbkuxmjzxyowioegiasepwhwqunazgpadmifilwlsxignfwmyqegfprntxxjsbdekiwtqwbscpfynjuomaiayrsqhccsogmkjguomudyptlchoepetnexgkczgisehsjizainhjugzhzyuadrtnfhhgcxkzbnylezhrtksnqxastxtzobdqclbporikrtqnrfmepgaheklcgugjyscxxjenmpnieethxbshtjmqlsfscjyorlxpdozouulhyfcoevxytmvdear".to_string()).must_be("qahaq");
    // answer("kyyrjtdplseovzwjkykrjwhxquwxsfsorjiumvxjhjmgeueafubtonhlerrgsgohfosqssmizcuqryqomsipovhhodpfyudtusjhonlqabhxfahfcjqxyckycstcqwxvicwkjeuboerkmjshfgiglceycmycadpnvoeaurqatesivajoqdilynbcihnidbizwkuaoegmytopzdmvvoewvhebqzskseeubnretjgnmyjwwgcooytfojeuzcuyhsznbcaiqpwcyusyyywqmmvqzvvceylnuwcbxybhqpvjumzomnabrjgcfaabqmiotlfojnyuolostmtacbwmwlqdfkbfikusuqtupdwdrjwqmuudbcvtpieiwteqbeyfyqejglmxofdjksqmzeugwvuniaxdrunyunnqpbnfbgqemvamaxuhjbyzqmhalrprhnindrkbopwbwsjeqrmyqipnqvjqzpjalqyfvaavyhytetllzupxjwozdfpmjhjlrnitnjgapzrakcqahaqetwllaaiadalmxgvpawqpgecojxfvcgxsbrldktufdrogkogbltcezflyctklpqrjymqzyzmtlssnavzcquytcskcnjzzrytsvawkavzboncxlhqfiofuohehaygxidxsofhmhzygklliovnwqbwwiiyarxtoihvjkdrzqsnmhdtdlpckuayhtfyirnhkrhbrwkdymjrjklonyggqnxhfvtkqxoicakzsxmgczpwhpkzcntkcwhkdkxvfnjbvjjoumczjyvdgkfukfuldolqnauvoyhoheoqvpwoisniv".to_string()).log();
}
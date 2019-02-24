use rustius::search_longest_2char_substr;

#[test]
fn test0() {
    assert_eq!(search_longest_2char_substr(""), "");
}

#[test]
fn test1() {
    assert_eq!(search_longest_2char_substr("aaaaabbb"), "aaaaabbb");
}

#[test]
fn test2() {
    assert_eq!(search_longest_2char_substr("aaaaabbbc"), "aaaaabbb");
}

#[test]
fn test3() {
    assert_eq!(
        search_longest_2char_substr("aaaaabbbcaaaaabbbb"),
        "aaaaabbbb"
    );
}

#[test]
fn test4() {
    assert_eq!(search_longest_2char_substr("abcdef"), "ab");
}

#[test]
fn test5() {
    assert_eq!(search_longest_2char_substr("раз два три"), "ра");
}

#[test]
fn test6() {
    assert_eq!(
        search_longest_2char_substr("трололололололотроло"),
        "ололололололо"
    );
}

#[test]
fn test7() {
    assert_eq!(
        search_longest_2char_substr("trolololollolotrolo"),
        "olololollolo"
    );
}

#[test]
fn test8() {
    assert_eq!(search_longest_2char_substr("abcababababcbaba"), "abababab");
}

#[test]
fn test9() {
    assert_eq!(search_longest_2char_substr("abababc"), "ababab");
}

#[test]
fn test10() {
    assert_eq!(search_longest_2char_substr("abababac"), "abababa");
}

#[test]
fn test11() {
    assert_eq!(search_longest_2char_substr("abababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababacabababac"), "abababa");
}

#[test]
fn test12() {
    assert_eq!(search_longest_2char_substr("0123012301230123012301230123012301230123012301230123012301230123012301230123012301230123012301230123012301230123012301230123012301230123012301230123012301230123012301230123012301230123012301230123012301230123012301230123012301230123012301230123012301230123012301230123012301230123012301230123012301230123012301230123012301230123012301230123012301230123012301230123012301230123012301230123012301230123012301230123012301230123012301230123012301230123012301230123"), "01");
}

#[test]
fn test13() {
    assert_eq!(
        search_longest_2char_substr("caaaaaaaaaaaaaaabbbbbbbbbb"),
        "aaaaaaaaaaaaaaabbbbbbbbbb"
    );
}

#[test]
fn test14() {
    assert_eq!(search_longest_2char_substr("   2  2  2 "), "   2  2  2 ");
}

#[test]
fn test15() {
    assert_eq!(search_longest_2char_substr("   12  2  2 "), "2  2  2 ");
}

#[test]
fn test16() {
    assert_eq!(search_longest_2char_substr(" "), " ");
}

#[test]
fn test17() {
    assert_eq!(search_longest_2char_substr("a"), "a");
}

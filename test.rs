use std;
use search;

fn find_str(haystack: str, needle: str) -> option<uint> {
    find_str_between(haystack, needle, 0u, str::len(haystack))
}

fn find_str_between(haystack: str, needle: str, start: uint, end:uint)
  -> option<uint> {
    let found = findn_str_between(haystack, needle, 1u, start, end);
    alt vec::len(found) {
        0u  { ret option::none; }
        _nn { ret option::some(found[0u]); }
    }
}

#[doc = "Returns up to `nn` byte positions of matched substrings"]
fn findn_str(haystack: str, needle: str, nn: uint) -> [uint] {
    findn_str_between(haystack, needle, nn, 0u, str::len(haystack))
}

fn findn_str_between (haystack: str, needle: str,
                      nn: uint,
                      start: uint, end: uint) -> [uint] {

    let hl = str::len(haystack);
    let nl = str::len(needle);

    // numbers subject to change...
    if hl > 10u * nl + 1500u
       && nl > 10u
    {
        ret search::boyer_moore_search(haystack, needle, nn, start, end);
    } else {
        ret search::simple_search(haystack, needle, nn, start, end);
    }
}


#[test]
fn test_findn_str_between() {
  let data = "abcabc";
  assert findn_str_between(data, "ab", 2u, 0u, 6u) == [0u, 3u];
  assert findn_str_between(data, "ab", 1u, 0u, 6u) == [0u];
  assert findn_str_between(data, "ax", 1u, 0u, 6u) == [];
}

#[test]
fn test_simple_search() {
  let data = "abcabc";
  assert search::simple_search(data, "ab", 2u, 0u, 6u) == [0u, 3u];
  assert search::simple_search(data, "ab", 1u, 0u, 6u) == [0u];
  assert search::simple_search(data, "ax", 1u, 0u, 6u) == [];
}

#[test]
fn test_boyer_moore_search() {
  let data = "abcabc";
  assert search::boyer_moore_search(data, "ab", 2u, 0u, 6u) == [0u, 3u];
  assert search::boyer_moore_search(data, "ab", 1u, 0u, 6u) == [0u];
  assert search::boyer_moore_search(data, "ax", 1u, 0u, 6u) == [];
}

#[test]
fn test_findn_str() {
  assert []       == findn_str("banana", "apple pie", 1u);
  assert [0u]     == findn_str("abcxxxxxx", "abc", 1u);
  assert [3u]     == findn_str("xxxabcxxx", "abc", 1u);
  assert [6u]     == findn_str("xxxxxxabc", "abc", 1u);
  assert [3u]     == findn_str("xxxabcabc", "abc", 1u);
  assert [3u, 6u] == findn_str("xxxabcabc", "abc", 5u);
  assert [3u, 7u] == findn_str("xxxabcxabc", "abc", 5u);
  assert [3u, 8u] == findn_str("xxxabcxxabc", "abc", 5u);
}


#[test]
fn test_unmatched_chars_ascii () {
  let ct = search::boyer_moore_unmatched_chars("ANPANMAN");

  assert 1u == ct['A' as uint];
  assert 2u == ct['M' as uint];
  assert 3u == ct['N' as uint];
  assert 5u == ct['P' as uint];

  // others
  assert 8u == ct['z' as uint];
  assert 8u == ct['w' as uint];
  assert 8u == ct['x' as uint];
}

#[test]
fn test_unmatched_chars_utf8() {
  let ct = search::boyer_moore_unmatched_chars("ะเ"); //e0b8b0 e0b980

  assert 2u == ct[0x_e0_u];
  assert 4u == ct[0x_b8_u];
  assert 3u == ct[0x_b0_u];
  assert 2u == ct[0x_e0_u];
  assert 1u == ct[0x_b9_u];
  assert 6u == ct[0x_80_u];
}

#[test]
fn test_boyer_moore_largest_suffixes() {
  assert search::boyer_moore_largest_suffixes("")
      == [];

  assert search::boyer_moore_largest_suffixes("x")
      == [1u];

  assert search::boyer_moore_largest_suffixes("heyyheyyheyy")
      == [0u,0u,1u,4u,0u,0u,1u,8u,0u,0u,1u,12u];

  assert search::boyer_moore_largest_suffixes("gcagagag")
      == [1u,0u,0u,2u,0u,4u,0u,8u];
}

#[test]
fn test_matching_suffixes_ascii() {
  assert [] == search::boyer_moore_matching_suffixes("");

  let test1 = search::boyer_moore_matching_suffixes("gcagagag");
  assert test1 == [1u,7u,4u,7u,2u,7u,7u,7u];


  let pt = search::boyer_moore_matching_suffixes("ANPANMAN");

  assert 1u == pt[0u]; //        (n)
  assert 8u == pt[1u]; //       (a)n
  assert 3u == pt[2u]; //      (m)an
  assert 6u == pt[3u]; //     (n)man
  assert 6u == pt[4u]; //    (a)nman
  assert 6u == pt[5u]; //   (p)anman
  assert 6u == pt[6u]; //  (n)panman
  assert 6u == pt[7u]; // (a)npanman
}

#[test]
fn test_matching_suffixes_utf8() {
  let pt = search::boyer_moore_matching_suffixes("ประเ");

  assert  1u == pt[0u];
  assert 12u == pt[3u];
  assert 12u == pt[6u];
  assert 12u == pt[9u];
}

#[test]
fn test_find_str() {
  assert find_str("banana", "apple pie") == none;
  assert find_str("", "") == some(0u);

  let data = "ประเทศไทย中华Việt Nam";
  assert find_str(data, "")     == some(0u);
  assert find_str(data, "ประเ") == some( 0u);
  assert find_str(data, "ะเ")   == some( 6u);
  assert find_str(data, "中华") == some(27u);
  assert find_str(data, "ไท华") == none;
}

#[test]
fn test_find_str_between_ascii() {
  assert find_str_between("", "", 0u, 0u) == some(0u);
  assert find_str_between("", "pow", 0u, 0u) == none;
  assert find_str_between("donatello", "don", 0u, 9u) == some(0u);
  assert find_str_between("don", "donatello", 0u, 3u) == none;

  let data = "abcabc";
  assert find_str_between(data, "ab", 0u, 6u) == some(0u);
  assert find_str_between(data, "ab", 2u, 6u) == some(3u);
  assert find_str_between(data, "ab", 2u, 4u) == none;
}

#[test]
fn test_find_str_between_utf8() {
  let mut data = "ประเทศไทย中华Việt Nam";
  data += data;
  assert find_str_between(data, "", 0u, 43u) == some(0u);
  assert find_str_between(data, "", 6u, 43u) == some(6u);

  assert find_str_between(data, "ประ", 0u, 43u) == some( 0u);
  assert find_str_between(data, "ทศไ", 0u, 43u) == some(12u);
  assert find_str_between(data, "ย中", 0u, 43u) == some(24u);
  assert find_str_between(data, "iệt", 0u, 43u) == some(34u);
  assert find_str_between(data, "Nam", 0u, 43u) == some(40u);

  assert find_str_between(data, "ประ", 43u, 86u) == some(43u);
  assert find_str_between(data, "ทศไ", 43u, 86u) == some(55u);
  assert find_str_between(data, "ย中", 43u, 86u) == some(67u);
  assert find_str_between(data, "iệt", 43u, 86u) == some(77u);
  assert find_str_between(data, "Nam", 43u, 86u) == some(83u);
}

#[test]
fn test_find_str_ascii() {
  assert option::some(0u) == find_str("", "");
  assert option::none     == find_str("banana", "apple pie");
  assert option::some(0u) == find_str("abcxxxxxx", "abc");
  assert option::some(3u) == find_str("xxxabcxxx", "abc");
  assert option::some(6u) == find_str("xxxxxxabc", "abc");
}

#[test]
fn test_find_str_utf8() {
  let data = "ประเทศไทย中华Việt Nam";

  assert option::some( 0u) == find_str(data, "");
  assert option::none      == find_str(data, "ไท华");
  assert option::some( 0u) == find_str(data, "ประเ");
  assert option::some( 3u) == find_str(data, "ระ");
  assert option::some( 6u) == find_str(data, "ะเ");
  assert option::some(15u) == find_str(data, "ศไทย中华");
  assert option::some(18u) == find_str(data, "ไทย中华");
  assert option::some(24u) == find_str(data, "ย中华");
  assert option::some(27u) == find_str(data, "中华");
}



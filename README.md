# Boyer-Moore string search

This is an implementation of the Boyer-Moore string search algorithm in Rust (0.2-pre). *See [Wikipedia](http://en.wikipedia.org/wiki/Boyer%E2%80%93Moore_string_search_algorithm), and [Charras and Lecroq](http://www-igm.univ-mlv.fr/~lecroq/string/node14.html).*

I've tried to calculate the prefix table as Charras and Lecroq do, but through my mistakes (or through Rust 0.1's inefficiencies) Boyer-Moore-Horspool outperforms Boyer-Moore, across the board.

Although I had high hopes, given how small the searches done in existing Rust code tend to be I don't think either algorithm as they stand makes sense to add to the Rust core::str library.  (Currently, the average needle is about 8 bytes and average haystack is about 18 bytes...)  But someone may find these functions useful, later.

// Kevin Cantu

## Comparison
![Boyer-Moore and naive search performance with random strings](http://kevincantu.org/code/rust/boyer-moore/data_bm.svg)

![Boyer-Moore-Horspool and naive search performance with random strings](http://kevincantu.org/code/rust/boyer-moore/data_bmh.svg)



// An implementation of the Boyer-Moore search algorithm in Rust (0.2 pre)

export
   // brute force
   simple_search,

   // boyer-moore
   boyer_moore_search, 
   boyer_moore_unmatched_chars,
   boyer_moore_largest_suffixes,
   boyer_moore_matching_suffixes,

   // boyer-moore-horspool
   boyer_moore_horspool_search;


#[doc = "
Returns up to `nn` byte positions of matched substrings
between `start` and `end`
(using a naive search algorithm)
"]
fn simple_search (haystack: str, needle: str,
                      nn: uint,
                      start: uint, end: uint) -> [uint] {
    let mut results = [];

    let nlen = str::len(needle);

    assert start <= end;
    assert end <= str::len(haystack);
    let hlen = end - start;

    // empty needle
    if nlen == 0u {
        ret [start];
    }

    // haystack empty, or smaller than needle
    if hlen == 0u || hlen < nlen {
        ret [];
    }

    let mut ii = start, match_start = 0u, match_i = 0u;

    while ii < end {
        if haystack[ii] == needle[match_i] {
            if match_i == 0u { match_start = ii; }
            match_i += 1u;
            // Found a match
            if match_i == nlen {
                vec::push(results, match_start);
                match_i = 0u;

                if vec::len(results) >= nn { ret results; }
            }
            ii += 1u;
        } else {
            // Failed match, backtrack
            if match_i > 0u {
                match_i = 0u;
                ii = match_start + 1u;
            } else {
                ii += 1u;
            }
        }
    }

    ret results;
}

#[doc = "
Returns up to `nn` byte positions of matched substrings
between `start` and `end`
(using Boyer-Moore)
"]
fn boyer_moore_search (haystack: str, needle: str,
                      nn: uint,
                      start: uint, end: uint) -> [uint] {
    let mut results = [];

    let nlen = str::len(needle);

    assert start <= end;
    assert end <= str::len(haystack);
    let hlen = end - start;

    // empty needle
    if nlen == 0u {
        ret [start];
    }

    // haystack empty, or smaller than needle
    if hlen == 0u || hlen < nlen {
        ret [];
    }

    // generate the tables
    let ct = boyer_moore_unmatched_chars(needle);
    let pt = boyer_moore_matching_suffixes(needle);

    // query both tables based on position
    // within the needle and character in haystack
    let getShift = fn@(pos: uint, ch: u8) -> uint {
        let matchedSoFar = nlen - 1u - pos;
        let rawCharShift = ct[ch as uint];
        let prefShift    = pt[matchedSoFar];

        if rawCharShift >= matchedSoFar {
           let adjCharShift = rawCharShift - matchedSoFar;

           if adjCharShift > prefShift {
               ret adjCharShift;
           }
        }

        ret prefShift;
    };

    // step up through the haystack
    let mut outerii = start;
    while outerii + nlen <= end {

        // step back through needle
        // (checking outer range again)
        let mut windowii = nlen;
        while 0u < windowii {

            windowii -= 1u;

            // matching byte?
            if needle[windowii] == haystack[outerii+windowii] {

                // needle fully matched?
                // note: last decremented windowii
                if windowii == 0u {
                    vec::push(results, outerii);

                    if vec::len(results) >= nn { ret results; }

                    outerii += nlen;
                }

                // if not fully matched, leave outerii alone
                // but decrement the windowii

            } else {
                // no match or a partial match
                outerii += getShift(windowii, haystack[outerii+windowii]);
                break;
            }
        }
    }

    ret results;
}

// compute the table used to choose a shift based on
// an unmatched character's possible position within the search string
// (a.k.a. the bad-character table)
fn boyer_moore_unmatched_chars(needle: str) -> [uint] {
    let len = str::len(needle);
    let deltas = vec::to_mut(vec::from_elem(255u, len));

    assert 0u < len;
    let mut jj = len - 1u; // drop the last byte

    // from last-1 to first
    while jj > 0u {
        jj -= 1u;

        let key = needle[jj] as uint;

        // if we haven't set it yet, set it now
        // (besides default)
        if deltas[key] == len {
            deltas[key] = len - 1u - jj;
        }
    }

    ret vec::from_mut(deltas);
}

// for each prefix of the search string
// find the largest suffix which is a suffix of the search string
fn boyer_moore_largest_suffixes(needle: str) -> [uint] {
    let len = str::len(needle);

    if len == 0u { ret []; }

    let mut suffs = vec::to_mut(vec::from_elem(len, 0u));
    suffs[len - 1u] = len;

    let mut ii   = len - 1u;
    let mut head = len; // index starting the previous found suffix
    let mut tail = len; // index after the previous found suffix

    // loop through each smaller prefix,
    // keeping track of the last suffix of a prefix
    // which was found to be a suffix of the needle
    while 0u < ii {
        ii -= 1u;

        if head < ii + 1u
           && suffs[(len - 1u) - ((tail - 1u) - ii)] + head < ii + 1u
        {
            // The needle is a suffix of itself, stored before this loop,
            // so each prefix of that is matched
            // with its largest possible suffix...
            //
            // So (bear with me) when considering prefixes
            // of another matched prefix (i.e., when head <= ii < tail)
            // if the corresponding maximum prefix's match is
            // smaller than the space left within the current match,
            // then we know this prefix's matching suffix is the same.

            // Consider:
            //     01234567
            //     heyyheyy
            //       ^   ^
            //
            // When testing i=2, a match from 0-3 has already been found
            // ("heyy"), and the match at i=6 ("y") fits
            // in the remaining space within the current match,
            // we know that suffs[2]=sufs[6].
            //
            // If, however, sufs[6] was much larger, we'd have to work more.

            suffs[ii] = suffs[(len - 1u) - ((tail-1u) - ii)];

        } else {
            // Here, find the largest suffix of the needle which matches
            // the prefix ending at ii.

            // move the head left
            //
            // Note that if the head is already further left,
            // we've already explored that far and eliminated the possibility
            // of smaller match, above.
            if ii + 1u <= head {
                 head = ii + 1u;
            }

            // put the tail here (the ending of this suffix)
            tail = ii + 1u;

            // move the head left until it is before the matching suffix
            while 1u <= head
               && needle[head-1u] == needle[(len - 1u) - (tail - head)]
            {
                head -= 1u;
            }

            // store the length of this suffix
            suffs[ii] = tail - head;
        }
    }

    ret vec::from_mut(suffs);
}

// compute the table used to choose a shift based on
// a partially matched suffix of the search string
// (a.k.a. the good-suffix table)
fn boyer_moore_matching_suffixes(needle: str) -> [uint] {
    let len   = str::len(needle);

    // compute the largest suffix of each prefix
    let suffs = boyer_moore_largest_suffixes(needle);

    // (1) initialize deltas
    let deltas = vec::to_mut(vec::from_elem(len, len));

    // (2) step to smaller suffixes ending with ii, and
    // if a whole prefix is a suffix
    // set all the deltas for indexes smaller than length - 1 - ii
    // to length - 1 - ii
    let mut ii = len;
    let mut jj = 0u;
    while 0u < ii {
        ii -= 1u;

        if suffs[ii] == ii + 1u {
            // do not reset jj, only do this once
            while ii < len - 1u - jj {
                if deltas[len - 1u - jj] == len {
                    deltas[len - 1u - jj] = len - 1u - ii;
                }
                jj += 1u;
            }
        }
    }

    // (3) then for each different matched suffix size, set the delta
    let mut kk = 0u;
    while 2u <= len && kk <= len - 2u {
        deltas[suffs[kk]] = len - 1u - kk;
        kk += 1u;
    }

    ret vec::from_mut(deltas);
}


#[doc = "the same, but Boyer-Moore-Horspool"]
fn boyer_moore_horspool_search (haystack: str, needle: str,
                      nn: uint,
                      start: uint, end: uint) -> [uint] {
    let mut results = [];

    let nlen = str::len(needle);

    assert start <= end;
    assert end <= str::len(haystack);
    let hlen = end - start;

    // empty needle
    if nlen == 0u {
        ret [start];
    }

    // haystack empty, or smaller than needle
    if hlen == 0u || hlen < nlen {
        ret [];
    }

    // generate the tables
    let ct = boyer_moore_unmatched_chars(needle);
//    let pt = boyer_moore_matching_suffixes(needle);

    // query both tables based on position
    // within the needle and character in haystack
    let getShift = fn@(pos: uint, ch: u8) -> uint {
        let matchedSoFar = nlen - 1u - pos;
        let rawCharShift = ct[ch as uint];
//        let prefShift    = pt[matchedSoFar];

        if rawCharShift >= matchedSoFar {
           let adjCharShift = rawCharShift - matchedSoFar;

//           if adjCharShift > prefShift {
               ret adjCharShift;
//           }
        }

//        ret prefShift;
        ret 1u;
    };

    // step up through the haystack
    let mut outerii = start;
    while outerii + nlen <= end {

        // step back through needle
        // (checking outer range again)
        let mut windowii = nlen;
        while 0u < windowii {

            windowii -= 1u;

            // matching byte?
            if needle[windowii] == haystack[outerii+windowii] {

                // needle fully matched?
                // note: last decremented windowii
                if windowii == 0u {
                    vec::push(results, outerii);

                    if vec::len(results) >= nn { ret results; }

                    outerii += nlen;
                }

                // if not fully matched, leave outerii alone
                // but decrement the windowii

            } else {
                // no match or a partial match
                outerii += getShift(windowii, haystack[outerii+windowii]);
                break;
            }
        }
    }

    ret results;
}


use unicode_segmentation::UnicodeSegmentation;

pub fn pattern_finder(pattern: &str, content: &str) -> Vec<usize> {
    let mut result = Vec::new();
    let t: Vec<_> = content.graphemes(true).collect(); 
    let p: Vec<_> = pattern.graphemes(true).collect();
    let n = t.len();
    let m = p.len();

    let pi = pi(&p);
    println!("{:?}", pi);
    let mut q = 0;

    for i in 0..n {
        while q > 0 && p[q] != t[i] {
            q = pi[q - 1];
        }

        if p[q] == t[i] {
            q += 1;
        }

        if q == m {
            result.push(i + 1 - m);
            q = pi[q - 1];
        }
    }

    result
}

fn pi(pattern: &Vec<&str>) -> Vec<usize> {
    let mut result = vec![0];
    let m = pattern.len();
    let mut k = 0;

    for q in 1..m {
        while k > 0 && pattern[k] != pattern[q] {
            k = result[k - 1];
        }

        if pattern[k] == pattern[q] {
            k += 1;
        }

        result.push(k);
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pi() {
        let text: Vec<_> = "ababababca".graphemes(true).collect();
        assert_eq!(pi(&text), vec![0, 0, 1, 2, 3, 4, 5, 6, 0, 1]);
    }

    #[test]
    fn pattern_finder_on_greek() {
        let content = "γδδβγβγββδβγδγβγγβγβδβγδγδδββδβγδβδβδγγγγγδδγδβγδδδδβββδδδδδγβγβγγδβδγγββδγδγγγβγδδβδβββδδββββγδβββββββββββγβγδββγγβββδβγγγγγδδβγγβδδγγδδβδγδδβγβδγββγγγδγγδδγδβδβγγγβββγβγδβββγδβββδγββγδββδγβγββδγβγδδβδδγβγβγββββγδγδββδδβδδδγδγγδγδβδβδγβββδδδγδβγγγδβγγγβδβγβδβδβγδδγγδγβββγδββγδδγγββγγβδγγβγγβδβγββγβδγγδββδγδδβδββββββββδβδβδβδβδβδβδβδβδβδββγββγγβδδδδββδδγβββββγβδβββγβγδδβββγβγβδβγγγβδγδβδδβδγδγγδγββδβγββδγδββδγβγγββδδβγγδδβγγγβγββδ";

        assert_eq!(pattern_finder("α", content), vec![]);
        assert_eq!(pattern_finder("αγ", content), vec![]);
        assert_eq!(
            pattern_finder("β", content),
            vec![
                3, 5, 7, 8, 10, 14, 17, 19, 21, 27, 28, 30, 33, 35, 46, 52, 53, 54, 61, 63, 67, 71,
                72, 79, 83, 85, 86, 87, 90, 91, 92, 93, 96, 97, 98, 99, 100, 101, 102, 103, 104,
                105, 106, 108, 111, 112, 115, 116, 117, 119, 127, 130, 137, 142, 144, 147, 148,
                159, 161, 165, 166, 167, 169, 172, 173, 174, 177, 178, 179, 182, 183, 186, 187,
                190, 192, 193, 196, 200, 204, 206, 208, 209, 210, 211, 216, 217, 220, 231, 233,
                236, 237, 238, 244, 249, 253, 255, 257, 259, 261, 269, 270, 271, 274, 275, 281,
                282, 285, 289, 292, 294, 296, 297, 299, 304, 305, 310, 312, 313, 314, 315, 316,
                317, 318, 319, 321, 323, 325, 327, 329, 331, 333, 335, 337, 339, 340, 342, 343,
                346, 351, 352, 356, 357, 358, 359, 360, 362, 364, 365, 366, 368, 372, 373, 374,
                376, 378, 380, 384, 388, 391, 399, 400, 402, 404, 405, 409, 410, 413, 416, 417,
                420, 425, 429, 431, 432
            ]
        );
        assert_eq!(
            pattern_finder("ββ", content),
            vec![
                7, 27, 52, 53, 71, 85, 86, 90, 91, 92, 96, 97, 98, 99, 100, 101, 102, 103, 104,
                105, 111, 115, 116, 147, 165, 166, 172, 173, 177, 178, 182, 186, 192, 208, 209,
                210, 216, 236, 237, 269, 270, 274, 281, 296, 304, 312, 313, 314, 315, 316, 317,
                318, 339, 342, 351, 356, 357, 358, 359, 364, 365, 372, 373, 399, 404, 409, 416,
                431
            ]
        );
        assert_eq!(
            pattern_finder("βββ", content),
            vec![
                52, 85, 90, 91, 96, 97, 98, 99, 100, 101, 102, 103, 104, 115, 165, 172, 177, 208,
                209, 236, 269, 312, 313, 314, 315, 316, 317, 356, 357, 358, 364, 372
            ]
        );
        assert_eq!(
            pattern_finder("βδβ", content),
            vec![
                8, 19, 28, 33, 83, 117, 159, 231, 253, 257, 259, 292, 310, 319, 321, 323, 325, 327,
                329, 331, 333, 335, 337, 362, 378, 400
            ]
        );
        assert_eq!(
            pattern_finder("γδ", content),
            vec![
                0, 11, 22, 24, 31, 41, 44, 47, 65, 74, 80, 94, 109, 124, 134, 139, 151, 154, 157,
                170, 175, 184, 197, 212, 214, 224, 227, 229, 242, 247, 262, 266, 272, 276, 302,
                307, 369, 386, 393, 396, 407, 422
            ]
        );
        assert_eq!(
            pattern_finder("δββ", content),
            vec![
                26, 51, 84, 89, 95, 110, 171, 176, 185, 215, 273, 303, 311, 338, 350, 363, 371, 408
            ]
        );
        assert_eq!(
            pattern_finder("βδγβ", content),
            vec![144, 179, 187, 193, 233, 410]
        );
        assert_eq!(pattern_finder("βγδδγ", content), vec![261, 275]);
    }
}


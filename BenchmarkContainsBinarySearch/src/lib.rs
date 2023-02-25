// Is binary_search() faster than contains() for 35 words?
// binary_search requires a ramp up, so there will be a break-even number of words, when this ramp up is worthwile.

// COLLECTED RUN TIMES -- Renaming the functions changes the result!
//   test tests::bench_35_words_binary_search ... bench:          25 ns/iter (+/- 0)
//   test tests::bench_35_words_contains      ... bench:          25 ns/iter (+/- 0)
//   test tests::bench_85_words_binary_search ... bench:          33 ns/iter (+/- 1)
//   test tests::bench_85_words_contains      ... bench:          63 ns/iter (+/- 1)
//   test tests::bench_xor                    ... bench:          92 ns/iter (+/- 3)
//   -----------------
//   test tests::bench_binary_search_with_35_words ... bench:          25 ns/iter (+/- 0)
//   test tests::bench_binary_search_with_85_words ... bench:          33 ns/iter (+/- 1)
//   test tests::bench_contains_with_35_word       ... bench:          21 ns/iter (+/- 0)
//   test tests::bench_contains_with_85_words      ... bench:          58 ns/iter (+/- 5)
//   test tests::bench_xor                         ... bench:          93 ns/iter (+/- 2)

// PRELIMINARY RESULT
//   contains is faster for 35 words
//   binary_search is faster for 85 words
//   The break-even point must be between 35 and 85 points

// TEST PROCEDURE
//    Test 4 times for a word absent from the list, 1 times for a word present in the list.

// THIS REGARDS
//   https://github.com/ogham/exa/pull/1146


// USAGE
//   cargo +nightly bench

// FOUND AT
//   https://doc.rust-lang.org/nightly/unstable-book/library-features/test.html



#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    // Running bench_xor influences the other 2: they run in less time. I leave bench_xor until I understand why
    #[bench]
    fn bench_xor(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(2000);
            (0..n).fold(0, |a, b| a ^ b)
        })
    }



    const CHOICES35 : [&str; 34] = [
        "BUILD",
        "BUILD.bazel",
        "Brewfile",
        "CMakeLists.txt",
        "Cargo.toml",
        "Containerfile",
        "Dockerfile",
        "GNUmakefile",
        "Gemfile",
        "Gruntfile.coffee",
        "Gruntfile.js",
        "Justfile",
        "Makefile",
        "PKGBUILD",
        "Pipfile",
        "Podfile",
        "Procfile",
        "Rakefile",
        "RoboFile.php",
        "SConstruct",
        "Vagrantfile",
        "WORKSPACE",
        "bsconfig.json",
        "build.gradle",
        "build.sbt",
        "build.xml",
        "composer.json",
        "makefile",
        "meson.build",
        "mix.exs",
        "package.json",
        "pom.xml",
        "tsconfig.json",
        "webpack.config.js",
    ];


    // thanks to https://randomwordgenerator.com/
    const CHOICES85 : [&str; 84] = [
        "BUILD",
        "BUILD.bazel",
        "Brewfile",
        "CMakeLists.txt",
        "Cargo.toml",
        "Containerfile",
        "Dockerfile",
        "GNUmakefile",
        "Gemfile",
        "Gruntfile.coffee",
        "Gruntfile.js",
        "Justfile",
        "Makefile",
        "PKGBUILD",
        "Pipfile",
        "Podfile",
        "Procfile",
        "Rakefile",
        "RoboFile.php",
        "SConstruct",
        "Vagrantfile",
        "WORKSPACE",
        "accident",
        "bsconfig.json",
        "build.gradle",
        "build.sbt",
        "build.xml",
        "camp",
        "clarify",
        "command",
        "composer.json",
        "confront",
        "consciousness",
        "conversation",
        "criticism",
        "dedicate",
        "disaster",
        "drain",
        "equip",
        "fault",
        "federation",
        "feed",
        "feminine",
        "function",
        "goalkeeper",
        "house",
        "invite",
        "junior",
        "linear",
        "makefile",
        "march",
        "marketing",
        "meson.build",
        "mix.exs",
        "noise",
        "overall",
        "owl",
        "package.json",
        "parade",
        "pest",
        "pom.xml",
        "produce",
        "pull",
        "reality",
        "recession",
        "redundancy",
        "restaurant",
        "section",
        "shine",
        "shorts",
        "spot",
        "spy",
        "stick",
        "talkative",
        "tidy",
        "ton",
        "trend",
        "tsconfig.json",
        "unanimous",
        "undermine",
        "webpack.config.js",
        "white",
        "whole",
        "accept",
    ];



    #[bench]
    fn bench_35_words_contains(b: &mut Bencher) {
        b.iter(|| {
            let no =  &CHOICES35.contains(&"foo1"); assert!(! no);
            let no =  &CHOICES35.contains(&"foo2"); assert!(! no);
            let no =  &CHOICES35.contains(&"foo3"); assert!(! no);
            let no =  &CHOICES35.contains(&"foo4"); assert!(! no);
            let yes = &CHOICES35.contains(&"tsconfig.json"); assert!(yes);
        })
    }


    #[bench]
    fn bench_35_words_binary_search(b: &mut Bencher) {
        b.iter(|| {
            let no =  &CHOICES35.binary_search(&"foo1").is_ok(); assert!(! no);
            let no =  &CHOICES35.binary_search(&"foo2").is_ok(); assert!(! no);
            let no =  &CHOICES35.binary_search(&"foo3").is_ok(); assert!(! no);
            let no =  &CHOICES35.binary_search(&"foo4").is_ok(); assert!(! no);
            let yes = &CHOICES35.binary_search(&"tsconfig.json").is_ok(); assert!(yes);
        })
    }


    #[bench]
    fn bench_85_words_contains(b: &mut Bencher) {

        b.iter(|| {
            let no =  &CHOICES85.contains(&"foo1"); assert!(! no);
            let no =  &CHOICES85.contains(&"foo2"); assert!(! no);
            let no =  &CHOICES85.contains(&"foo3"); assert!(! no);
            let no =  &CHOICES85.contains(&"foo4"); assert!(! no);
            let yes = &CHOICES85.contains(&"tsconfig.json"); assert!(yes);
        })
    }


    #[bench]
    fn bench_85_words_binary_search(b: &mut Bencher) {
        b.iter(|| {
            let no =  &CHOICES85.binary_search(&"foo1").is_ok(); assert!(! no);
            let no =  &CHOICES85.binary_search(&"foo2").is_ok(); assert!(! no);
            let no =  &CHOICES85.binary_search(&"foo3").is_ok(); assert!(! no);
            let no =  &CHOICES85.binary_search(&"foo4").is_ok(); assert!(! no);
            let yes = &CHOICES85.binary_search(&"tsconfig.json").is_ok(); assert!(yes);
        })
    }

}

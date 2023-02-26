// Is binary_search() faster than contains() for 35 words?
// binary_search requires a ramp up, so there will be a break-even number of words, when this ramp up is worthwile.

// COLLECTED RUN TIMES
//   test tests::bench_35_words_binary_search         ... bench:          73 ns/iter (+/- 7)
//   test tests::bench_35_words_contains              ... bench:          77 ns/iter (+/- 27)
//   test tests::bench_35_words_hashmap_local_mutable ... bench:          58 ns/iter (+/- 1)
//   test tests::bench_35_words_hashmap_static_phf    ... bench:          70 ns/iter (+/- 2)
//   test tests::bench_35_words_lazy_hashset_1        ... bench:          62 ns/iter (+/- 2)
//   test tests::bench_35_words_lazy_hashset_2        ... bench:          62 ns/iter (+/- 1)
//   test tests::bench_35_words_match                 ... bench:           0 ns/iter (+/- 0)
//   test tests::bench_35_words_matchmacro            ... bench:           0 ns/iter (+/- 0)
//   test tests::bench_35_words_static_phf_set        ... bench:          70 ns/iter (+/- 1)
//   test tests::bench_85_words_binary_search         ... bench:         100 ns/iter (+/- 3)
//   test tests::bench_85_words_contains              ... bench:         139 ns/iter (+/- 2)
//   test tests::bench_85_words_hashmap_local_mutable ... bench:          60 ns/iter (+/- 1)
//   test tests::bench_85_words_hashmap_static_phf    ... bench:          70 ns/iter (+/- 2)
//   test tests::bench_85_words_lazy_hashset_1        ... bench:          59 ns/iter (+/- 4)
//   test tests::bench_85_words_lazy_hashset_2        ... bench:          57 ns/iter (+/- 3)
//   test tests::bench_85_words_match                 ... bench:         121 ns/iter (+/- 5)
//   test tests::bench_85_words_matchmarco            ... bench:         118 ns/iter (+/- 3)
//   test tests::bench_85_words_static_phf_set        ... bench:          70 ns/iter (+/- 5)
//   test tests::bench_xor                            ... bench:          93 ns/iter (+/- 2)


// Renaming the functions changes the result!
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
//   https://doc.rust-lang.org/rust-by-example/std/hash.html



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


    use lazy_static::lazy_static;
    use std::collections::HashSet;
    lazy_static! {
        static ref LAZY_CHOICES_35: HashSet<&'static str> = initialize_data_35();
        static ref LAZY_CHOICES_85: HashSet<&'static str> = initialize_data_85();
    }
    fn initialize_data_35() -> HashSet<&'static str> {
        let set:HashSet<&'static str> = CHOICES35.into_iter().collect();
        set
    }
    fn initialize_data_85() -> HashSet<&'static str> {
        let set:HashSet<&'static str> = CHOICES85.into_iter().collect();
        set
    }


    use phf::phf_map;
    use phf::{phf_set, Set};

    static PHF_SET_35: Set<&'static str> = phf_set! {
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
    };

    static PHF_SET_85: Set<&'static str> = phf_set! {
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
    };

    static PHF_HASH_35: phf::Map<&'static str, bool> = phf_map! {
        "BUILD" => true,
        "BUILD.bazel" => true,
        "Brewfile" => true,
        "CMakeLists.txt" => true,
        "Cargo.toml" => true,
        "Containerfile" => true,
        "Dockerfile" => true,
        "GNUmakefile" => true,
        "Gemfile" => true,
        "Gruntfile.coffee" => true,
        "Gruntfile.js" => true,
        "Justfile" => true,
        "Makefile" => true,
        "PKGBUILD" => true,
        "Pipfile" => true,
        "Podfile" => true,
        "Procfile" => true,
        "Rakefile" => true,
        "RoboFile.php" => true,
        "SConstruct" => true,
        "Vagrantfile" => true,
        "WORKSPACE" => true,
        "bsconfig.json" => true,
        "build.gradle" => true,
        "build.sbt" => true,
        "build.xml" => true,
        "composer.json" => true,
        "makefile" => true,
        "meson.build" => true,
        "mix.exs" => true,
        "package.json" => true,
        "pom.xml" => true,
        "tsconfig.json" => true,
        "webpack.config.js" => true,
    };

    static PHF_HASH_85: phf::Map<&'static str, bool> = phf_map! {
        "BUILD" => true,
        "BUILD.bazel" => true,
        "Brewfile" => true,
        "CMakeLists.txt" => true,
        "Cargo.toml" => true,
        "Containerfile" => true,
        "Dockerfile" => true,
        "GNUmakefile" => true,
        "Gemfile" => true,
        "Gruntfile.coffee" => true,
        "Gruntfile.js" => true,
        "Justfile" => true,
        "Makefile" => true,
        "PKGBUILD" => true,
        "Pipfile" => true,
        "Podfile" => true,
        "Procfile" => true,
        "Rakefile" => true,
        "RoboFile.php" => true,
        "SConstruct" => true,
        "Vagrantfile" => true,
        "WORKSPACE" => true,
        "accident" => true,
        "bsconfig.json" => true,
        "build.gradle" => true,
        "build.sbt" => true,
        "build.xml" => true,
        "camp" => true,
        "clarify" => true,
        "command" => true,
        "composer.json" => true,
        "confront" => true,
        "consciousness" => true,
        "conversation" => true,
        "criticism" => true,
        "dedicate" => true,
        "disaster" => true,
        "drain" => true,
        "equip" => true,
        "fault" => true,
        "federation" => true,
        "feed" => true,
        "feminine" => true,
        "function" => true,
        "goalkeeper" => true,
        "house" => true,
        "invite" => true,
        "junior" => true,
        "linear" => true,
        "makefile" => true,
        "march" => true,
        "marketing" => true,
        "meson.build" => true,
        "mix.exs" => true,
        "noise" => true,
        "overall" => true,
        "owl" => true,
        "package.json" => true,
        "parade" => true,
        "pest" => true,
        "pom.xml" => true,
        "produce" => true,
        "pull" => true,
        "reality" => true,
        "recession" => true,
        "redundancy" => true,
        "restaurant" => true,
        "section" => true,
        "shine" => true,
        "shorts" => true,
        "spot" => true,
        "spy" => true,
        "stick" => true,
        "talkative" => true,
        "tidy" => true,
        "ton" => true,
        "trend" => true,
        "tsconfig.json" => true,
        "unanimous" => true,
        "undermine" => true,
        "webpack.config.js" => true,
        "white" => true,
        "whole" => true,
        "accept" => true,
    };


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


    fn using_match_for_lookup_35(word: &str) -> bool {
        match word {
            "BUILD" => true,
            "BUILD.bazel" => true,
            "Brewfile" => true,
            "CMakeLists.txt" => true,
            "Cargo.toml" => true,
            "Containerfile" => true,
            "Dockerfile" => true,
            "GNUmakefile" => true,
            "Gemfile" => true,
            "Gruntfile.coffee" => true,
            "Gruntfile.js" => true,
            "Justfile" => true,
            "Makefile" => true,
            "PKGBUILD" => true,
            "Pipfile" => true,
            "Podfile" => true,
            "Procfile" => true,
            "Rakefile" => true,
            "RoboFile.php" => true,
            "SConstruct" => true,
            "Vagrantfile" => true,
            "WORKSPACE" => true,
            "bsconfig.json" => true,
            "build.gradle" => true,
            "build.sbt" => true,
            "build.xml" => true,
            "composer.json" => true,
            "makefile" => true,
            "meson.build" => true,
            "mix.exs" => true,
            "package.json" => true,
            "pom.xml" => true,
            "tsconfig.json" => true,
            "webpack.config.js" => true,
            _   => false,
        }
    }

    fn using_match_for_lookup_85(word: &str) -> bool {
        match word {
        "BUILD" => true,
        "BUILD.bazel" => true,
        "Brewfile" => true,
        "CMakeLists.txt" => true,
        "Cargo.toml" => true,
        "Containerfile" => true,
        "Dockerfile" => true,
        "GNUmakefile" => true,
        "Gemfile" => true,
        "Gruntfile.coffee" => true,
        "Gruntfile.js" => true,
        "Justfile" => true,
        "Makefile" => true,
        "PKGBUILD" => true,
        "Pipfile" => true,
        "Podfile" => true,
        "Procfile" => true,
        "Rakefile" => true,
        "RoboFile.php" => true,
        "SConstruct" => true,
        "Vagrantfile" => true,
        "WORKSPACE" => true,
        "accident" => true,
        "bsconfig.json" => true,
        "build.gradle" => true,
        "build.sbt" => true,
        "build.xml" => true,
        "camp" => true,
        "clarify" => true,
        "command" => true,
        "composer.json" => true,
        "confront" => true,
        "consciousness" => true,
        "conversation" => true,
        "criticism" => true,
        "dedicate" => true,
        "disaster" => true,
        "drain" => true,
        "equip" => true,
        "fault" => true,
        "federation" => true,
        "feed" => true,
        "feminine" => true,
        "function" => true,
        "goalkeeper" => true,
        "house" => true,
        "invite" => true,
        "junior" => true,
        "linear" => true,
        "makefile" => true,
        "march" => true,
        "marketing" => true,
        "meson.build" => true,
        "mix.exs" => true,
        "noise" => true,
        "overall" => true,
        "owl" => true,
        "package.json" => true,
        "parade" => true,
        "pest" => true,
        "pom.xml" => true,
        "produce" => true,
        "pull" => true,
        "reality" => true,
        "recession" => true,
        "redundancy" => true,
        "restaurant" => true,
        "section" => true,
        "shine" => true,
        "shorts" => true,
        "spot" => true,
        "spy" => true,
        "stick" => true,
        "talkative" => true,
        "tidy" => true,
        "ton" => true,
        "trend" => true,
        "tsconfig.json" => true,
        "unanimous" => true,
        "undermine" => true,
        "webpack.config.js" => true,
        "white" => true,
        "whole" => true,
        "accept" => true,
        _   => false,
       }
    }


    fn using_matchmacro_for_lookup_35(word: &str) -> bool {
        matches! (word,
            "BUILD"
            | "BUILD.bazel"
            | "Brewfile"
            | "CMakeLists.txt"
            | "Cargo.toml"
            | "Containerfile"
            | "Dockerfile"
            | "GNUmakefile"
            | "Gemfile"
            | "Gruntfile.coffee"
            | "Gruntfile.js"
            | "Justfile"
            | "Makefile"
            | "PKGBUILD"
            | "Pipfile"
            | "Podfile"
            | "Procfile"
            | "Rakefile"
            | "RoboFile.php"
            | "SConstruct"
            | "Vagrantfile"
            | "WORKSPACE"
            | "bsconfig.json"
            | "build.gradle"
            | "build.sbt"
            | "build.xml"
            | "composer.json"
            | "makefile"
            | "meson.build"
            | "mix.exs"
            | "package.json"
            | "pom.xml"
            | "tsconfig.json"
            | "webpack.config.js"        )
    }

    fn using_matchmacro_for_lookup_85(word: &str) -> bool {
        matches! (word,
            "BUILD"
            | "BUILD.bazel"
            | "Brewfile"
            | "CMakeLists.txt"
            | "Cargo.toml"
            | "Containerfile"
            | "Dockerfile"
            | "GNUmakefile"
            | "Gemfile"
            | "Gruntfile.coffee"
            | "Gruntfile.js"
            | "Justfile"
            | "Makefile"
            | "PKGBUILD"
            | "Pipfile"
            | "Podfile"
            | "Procfile"
            | "Rakefile"
            | "RoboFile.php"
            | "SConstruct"
            | "Vagrantfile"
            | "WORKSPACE"
            | "accident"
            | "bsconfig.json"
            | "build.gradle"
            | "build.sbt"
            | "build.xml"
            | "camp"
            | "clarify"
            | "command"
            | "composer.json"
            | "confront"
            | "consciousness"
            | "conversation"
            | "criticism"
            | "dedicate"
            | "disaster"
            | "drain"
            | "equip"
            | "fault"
            | "federation"
            | "feed"
            | "feminine"
            | "function"
            | "goalkeeper"
            | "house"
            | "invite"
            | "junior"
            | "linear"
            | "makefile"
            | "march"
            | "marketing"
            | "meson.build"
            | "mix.exs"
            | "noise"
            | "overall"
            | "owl"
            | "package.json"
            | "parade"
            | "pest"
            | "pom.xml"
            | "produce"
            | "pull"
            | "reality"
            | "recession"
            | "redundancy"
            | "restaurant"
            | "section"
            | "shine"
            | "shorts"
            | "spot"
            | "spy"
            | "stick"
            | "talkative"
            | "tidy"
            | "ton"
            | "trend"
            | "tsconfig.json"
            | "unanimous"
            | "undermine"
            | "webpack.config.js"
            | "white"
            | "whole"
            | "accept"
            )
    }


    #[bench]
    fn bench_35_words_matchmacro(b: &mut Bencher) {
        b.iter(|| {
            let no =  using_matchmacro_for_lookup_35(&"foo1"); assert!(! no);
            let no =  using_matchmacro_for_lookup_35(&"foo2"); assert!(! no);
            let no =  using_matchmacro_for_lookup_35(&"foo3"); assert!(! no);
            let no =  using_matchmacro_for_lookup_35(&"foo4"); assert!(! no);
            let yes = using_matchmacro_for_lookup_35(&"tsconfig.json"); assert!(yes);
        })
    }


    #[bench]
    fn bench_85_words_matchmarco(b: &mut Bencher) {
        b.iter(|| {
            let no =  using_matchmacro_for_lookup_85(&"foo1"); assert!(! no);
            let no =  using_matchmacro_for_lookup_85(&"foo2"); assert!(! no);
            let no =  using_matchmacro_for_lookup_85(&"foo3"); assert!(! no);
            let no =  using_matchmacro_for_lookup_85(&"foo4"); assert!(! no);
            let yes = using_matchmacro_for_lookup_85(&"tsconfig.json"); assert!(yes);
        })
    }


    #[bench]
    fn bench_35_words_match(b: &mut Bencher) {
        b.iter(|| {
            let no =  using_match_for_lookup_35(&"foo1"); assert!(! no);
            let no =  using_match_for_lookup_35(&"foo2"); assert!(! no);
            let no =  using_match_for_lookup_35(&"foo3"); assert!(! no);
            let no =  using_match_for_lookup_35(&"foo4"); assert!(! no);
            let yes = using_match_for_lookup_35(&"tsconfig.json"); assert!(yes);
        })
    }


    #[bench]
    fn bench_85_words_match(b: &mut Bencher) {
        b.iter(|| {
            let no =  using_match_for_lookup_85(&"foo1"); assert!(! no);
            let no =  using_match_for_lookup_85(&"foo2"); assert!(! no);
            let no =  using_match_for_lookup_85(&"foo3"); assert!(! no);
            let no =  using_match_for_lookup_85(&"foo4"); assert!(! no);
            let yes = using_match_for_lookup_85(&"tsconfig.json"); assert!(yes);
        })
    }


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
    fn bench_35_words_hashmap_local_mutable(b: &mut Bencher) {
        use std::collections::HashMap;
        let mut choiceshash = HashMap::new();
        for val in &CHOICES35 {
            choiceshash.insert(val, 1);
        }

        b.iter(|| {
            let no =  &choiceshash.contains_key(&"foo1"); assert!(! no);
            let no =  &choiceshash.contains_key(&"foo2"); assert!(! no);
            let no =  &choiceshash.contains_key(&"foo3"); assert!(! no);
            let no =  &choiceshash.contains_key(&"foo4"); assert!(! no);
            let yes = &choiceshash.contains_key(&"tsconfig.json"); assert!(yes);
        })
    }


    #[bench]
    fn bench_35_words_hashmap_static_phf(b: &mut Bencher) {
        b.iter(|| {
            let no =  &PHF_HASH_35.contains_key(&"foo1"); assert!(! no);
            let no =  &PHF_HASH_35.contains_key(&"foo2"); assert!(! no);
            let no =  &PHF_HASH_35.contains_key(&"foo3"); assert!(! no);
            let no =  &PHF_HASH_35.contains_key(&"foo4"); assert!(! no);
            let yes = &PHF_HASH_35.contains_key(&"tsconfig.json"); assert!(yes);
        })
    }


    #[bench]
    fn bench_35_words_static_phf_set(b: &mut Bencher) {
        b.iter(|| {
            let no =  &PHF_SET_35.contains(&"foo1"); assert!(! no);
            let no =  &PHF_SET_35.contains(&"foo2"); assert!(! no);
            let no =  &PHF_SET_35.contains(&"foo3"); assert!(! no);
            let no =  &PHF_SET_35.contains(&"foo4"); assert!(! no);
            let yes = &PHF_SET_35.contains(&"tsconfig.json"); assert!(yes);
        })
    }


    #[bench]
    fn bench_35_words_lazy_hashset_1(b: &mut Bencher) {
        b.iter(|| {
            let no =  &LAZY_CHOICES_35.contains(&"foo1"); assert!(! no);
            let no =  &LAZY_CHOICES_35.contains(&"foo2"); assert!(! no);
            let no =  &LAZY_CHOICES_35.contains(&"foo3"); assert!(! no);
            let no =  &LAZY_CHOICES_35.contains(&"foo4"); assert!(! no);
            let yes = &LAZY_CHOICES_35.contains(&"tsconfig.json"); assert!(yes);
        })
    }


    #[bench]
    fn bench_35_words_lazy_hashset_2(b: &mut Bencher) {
        b.iter(|| {
            let no =  &LAZY_CHOICES_35.contains(&"foo1"); assert!(! no);
            let no =  &LAZY_CHOICES_35.contains(&"foo2"); assert!(! no);
            let no =  &LAZY_CHOICES_35.contains(&"foo3"); assert!(! no);
            let no =  &LAZY_CHOICES_35.contains(&"foo4"); assert!(! no);
            let yes = &LAZY_CHOICES_35.contains(&"tsconfig.json"); assert!(yes);
        })
    }



    #[bench]
    fn bench_85_words_lazy_hashset_1(b: &mut Bencher) {
        b.iter(|| {
            let no =  &LAZY_CHOICES_85.contains(&"foo1"); assert!(! no);
            let no =  &LAZY_CHOICES_85.contains(&"foo2"); assert!(! no);
            let no =  &LAZY_CHOICES_85.contains(&"foo3"); assert!(! no);
            let no =  &LAZY_CHOICES_85.contains(&"foo4"); assert!(! no);
            let yes = &LAZY_CHOICES_85.contains(&"tsconfig.json"); assert!(yes);
        })
    }


    #[bench]
    fn bench_85_words_lazy_hashset_2(b: &mut Bencher) {
        b.iter(|| {
            let no =  &LAZY_CHOICES_85.contains(&"foo1"); assert!(! no);
            let no =  &LAZY_CHOICES_85.contains(&"foo2"); assert!(! no);
            let no =  &LAZY_CHOICES_85.contains(&"foo3"); assert!(! no);
            let no =  &LAZY_CHOICES_85.contains(&"foo4"); assert!(! no);
            let yes = &LAZY_CHOICES_85.contains(&"tsconfig.json"); assert!(yes);
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


    #[bench]
    fn bench_85_words_hashmap_local_mutable(b: &mut Bencher) {
        use std::collections::HashMap;
        let mut choiceshash = HashMap::new();
        for val in &CHOICES85 {
            choiceshash.insert(val, 1);
        }

        b.iter(|| {
            let no =  &choiceshash.contains_key(&"foo1"); assert!(! no);
            let no =  &choiceshash.contains_key(&"foo2"); assert!(! no);
            let no =  &choiceshash.contains_key(&"foo3"); assert!(! no);
            let no =  &choiceshash.contains_key(&"foo4"); assert!(! no);
            let yes = &choiceshash.contains_key(&"tsconfig.json"); assert!(yes);
        })
    }


    #[bench]
    fn bench_85_words_hashmap_static_phf(b: &mut Bencher) {
        b.iter(|| {
            let no =  &PHF_HASH_85.contains_key(&"foo1"); assert!(! no);
            let no =  &PHF_HASH_85.contains_key(&"foo2"); assert!(! no);
            let no =  &PHF_HASH_85.contains_key(&"foo3"); assert!(! no);
            let no =  &PHF_HASH_85.contains_key(&"foo4"); assert!(! no);
            let yes = &PHF_HASH_85.contains_key(&"tsconfig.json"); assert!(yes);
        })
    }


    #[bench]
    fn bench_85_words_static_phf_set(b: &mut Bencher) {
        b.iter(|| {
            let no =  &PHF_SET_85.contains(&"foo1"); assert!(! no);
            let no =  &PHF_SET_85.contains(&"foo2"); assert!(! no);
            let no =  &PHF_SET_85.contains(&"foo3"); assert!(! no);
            let no =  &PHF_SET_85.contains(&"foo4"); assert!(! no);
            let yes = &PHF_SET_85.contains(&"tsconfig.json"); assert!(yes);
        })
    }



}

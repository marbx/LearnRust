// Is binary_search() faster than contains() for 35 words?
// binary_search requires a ramp up, so there will be a break-even number of words, when this ramp up is worthwile.

// COLLECTED RUN TIMES
//   test tests::bench_binary_search_with_35_words ... bench:          25 ns/iter (+/- 0)
//   test tests::bench_binary_search_with_85_words ... bench:          33 ns/iter (+/- 1)
//   test tests::bench_contains_with_35_word       ... bench:          21 ns/iter (+/- 0)
//   test tests::bench_contains_with_85_words      ... bench:          58 ns/iter (+/- 5)
//   test tests::bench_xor                         ... bench:          93 ns/iter (+/- 2)

// PRELIMINARY RESULT
//   contains is faster for 35 words
//   binary_search is faster for 85 words
//   The break-even point must be between 35 and 85 points


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

    #[bench]
    fn bench_contains_with_35_word(b: &mut Bencher) {
        let choices = [
            "Makefile", "Cargo.toml", "SConstruct", "CMakeLists.txt",
            "build.gradle", "pom.xml", "Rakefile", "package.json", "Gruntfile.js",
            "Gruntfile.coffee", "BUILD", "BUILD.bazel", "WORKSPACE", "build.xml", "Podfile",
            "webpack.config.js", "meson.build", "composer.json", "RoboFile.php", "PKGBUILD",
            "Justfile", "Procfile", "Dockerfile", "Containerfile", "Vagrantfile", "Brewfile",
            "Gemfile", "Pipfile", "build.sbt", "mix.exs", "bsconfig.json", "tsconfig.json"
        ];

        b.iter(|| {
            let no = &choices.contains(&"foo");
            let yes = &choices.contains(&"tsconfig.json");
            assert!(! no);
            assert!(yes);
        })
    }

    #[bench]
    fn bench_binary_search_with_35_words(b: &mut Bencher) {
        let choices = [
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

        b.iter(|| {
            let no = &choices.binary_search(&"foo").is_ok();
            let yes = &choices.binary_search(&"tsconfig.json").is_ok();
            assert!(! no);
            assert!(yes);
        })
    }




    // thanks to https://randomwordgenerator.com/
    #[bench]
    fn bench_contains_with_85_words(b: &mut Bencher) {
        let choices = [
            "Makefile", "Cargo.toml", "SConstruct", "CMakeLists.txt",
            "build.gradle", "pom.xml", "Rakefile", "package.json", "Gruntfile.js",
            "Gruntfile.coffee", "BUILD", "BUILD.bazel", "WORKSPACE", "build.xml", "Podfile",
            "webpack.config.js", "meson.build", "composer.json", "RoboFile.php", "PKGBUILD",
            "Justfile", "Procfile", "Dockerfile", "Containerfile", "Vagrantfile", "Brewfile",
            "Gemfile", "Pipfile", "build.sbt", "mix.exs", "bsconfig.json", "tsconfig.json",
            "spot","accept","reality","march","fault","spy","pull","parade","restaurant",
            "redundancy","criticism","produce","recession","equip","marketing","stick","confront",
            "clarify","consciousness","talkative","whole","federation","goalkeeper","shorts","white","command",
            "drain","linear","invite","feminine","dedicate","accident","overall","trend","section","camp",
            "unanimous","disaster","noise","house","shine","owl","conversation","tidy","undermine","junior",
            "ton","pest","feed","function"
        ];

        b.iter(|| {
            let no = &choices.contains(&"foo");
            let yes = &choices.contains(&"function");
            assert!(! no);
            assert!(yes);
        })
    }

    #[bench]
    fn bench_binary_search_with_85_words(b: &mut Bencher) {
        let choices = [
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

        b.iter(|| {
            let no = &choices.binary_search(&"foo").is_ok();
            let yes = &choices.binary_search(&"tsconfig.json").is_ok();
            assert!(! no);
            assert!(yes);
        })
    }



}

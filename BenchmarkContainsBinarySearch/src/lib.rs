// Is binary_search() faster than contains() for 35 strings?

// COLLECTED RESULTS
//   test tests::bench_binary_search_with_35_file_names ... bench:          25 ns/iter (+/- 1)
//   test tests::bench_contains_with_35_file_names      ... bench:          21 ns/iter (+/- 3)
//   test tests::bench_xor                              ... bench:          94 ns/iter (+/- 2)


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
    fn bench_contains_with_35_file_names(b: &mut Bencher) {
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
    fn bench_binary_search_with_35_file_names(b: &mut Bencher) {
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



}

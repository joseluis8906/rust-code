load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "gazelle_rust",
    sha256 = "312f581a19cc4828c93df5a30c20e3f34a67a3e21853c70f64e7a9e74e670230",
    strip_prefix = "gazelle_rust-00e88bceaa1a1c35d9c3019f65f3e20459fafe33",
    url = "https://github.com/Calsign/gazelle_rust/archive/00e88bceaa1a1c35d9c3019f65f3e20459fafe33.zip",
)


# To find additional information on this release or newer ones visit:
# https://github.com/bazelbuild/rules_rust/releases
http_archive(
    name = "rules_rust",
    patches = ["@gazelle_rust//patches:rules_rust.patch"],
    sha256 = "36ab8f9facae745c9c9c1b33d225623d976e78f2cc3f729b7973d8c20934ab95",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.31.0/rules_rust-v0.31.0.tar.gz"],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains(
    edition = "2021",
    versions = [
        "1.74.0"
    ],
)

load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")

crate_universe_dependencies()

# Load gazelle_rust transitive dependencies (includes gazelle). You can also load gazelle yourself,
# before these macros.
load("@gazelle_rust//:deps1.bzl", "gazelle_rust_dependencies1")

gazelle_rust_dependencies1()

load("@gazelle_rust//:deps2.bzl", "gazelle_rust_dependencies2")

gazelle_rust_dependencies2()

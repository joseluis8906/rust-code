workspace(name = "com_github_joseluis8906_rust_code")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "toolchains_llvm",
    canonical_id = "0.10.3",
    sha256 = "b7cd301ef7b0ece28d20d3e778697a5e3b81828393150bed04838c0c52963a01",
    strip_prefix = "toolchains_llvm-0.10.3",
    url = "https://github.com/grailbio/bazel-toolchain/releases/download/0.10.3/toolchains_llvm-0.10.3.tar.gz",
)

load("@toolchains_llvm//toolchain:deps.bzl", "bazel_toolchain_dependencies")

bazel_toolchain_dependencies()

load("@toolchains_llvm//toolchain:rules.bzl", "llvm_toolchain")

llvm_toolchain(
    name = "llvm_toolchain",
    llvm_version = "16.0.0",
)

load("@llvm_toolchain//:toolchains.bzl", "llvm_register_toolchains")

llvm_register_toolchains()

http_archive(
    name = "bazel_gazelle",
    integrity = "sha256-MpOL2hbmcABjA1R5Bj2dJMYO2o15/Uc5Vj9Q0zHLMgk=",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/bazel-gazelle/releases/download/v0.35.0/bazel-gazelle-v0.35.0.tar.gz",
        "https://github.com/bazelbuild/bazel-gazelle/releases/download/v0.35.0/bazel-gazelle-v0.35.0.tar.gz",
    ],
)

GAZELLE_RUST_COMMIT = "5d978adc1af6571a49d129fe50b31a28d0fc98cd"
GAZELLE_RUST_SHA256 = "sha256-5oP0mqebVi6YoPhWZE/j/qwTi7pIAAIpKh/zDO8e6iU="

http_archive(
    name = "gazelle_rust",
    integrity = GAZELLE_RUST_SHA256,
    strip_prefix = "gazelle_rust-{}".format(GAZELLE_RUST_COMMIT),
    url = "https://github.com/Calsign/gazelle_rust/archive/{}.zip".format(GAZELLE_RUST_COMMIT),
)


# To find additional information on this release or newer ones visit:
# https://github.com/bazelbuild/rules_rust/releases
http_archive(
    name = "rules_rust",
    patches = ["@gazelle_rust//patches:rules_rust.patch"],
    sha256 = "6501960c3e4da32495d1e1007ded0769a534cb195c30dea36aa54f9d8a3f0361",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.38.0/rules_rust-v0.38.0.tar.gz"],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains(
    edition = "2021",
    versions = [
        # "1.76.0"
      "nightly/2024-02-04"
    ],
)

# load("@rules_rust//crate_universe:repositories.bzl", "crate_universe_dependencies")

# crate_universe_dependencies()
load("@rules_rust//crate_universe:defs.bzl", "crates_repository")

# Example of using crate_universe. For gazelle_rust to work correctly with crate_universe
# dependencies, this must be paired with two gazelle directives; see BUILD.bazel.
crates_repository(
    name = "crates_index",
    cargo_lockfile = "//:Cargo.lock",
    lockfile = "//:Cargo.bazel.lock",
    manifests = ["//:Cargo.toml"],
    # packages = {
    #     "clap": crate.spec(
    #         features = ["derive"],
    #         version = "3.2",
    #     ),
    # },
    rust_version = "nightly/2024-02-04",
    # rust_version = "1.76.0",
)

load("@crates_index//:defs.bzl", "crate_repositories")

crate_repositories()

# Load gazelle_rust transitive dependencies (includes gazelle). You can also load gazelle yourself,
# before these macros.
load("@gazelle_rust//:deps1.bzl", "gazelle_rust_dependencies1")

gazelle_rust_dependencies1()

load("@gazelle_rust//:deps2.bzl", "gazelle_rust_dependencies2")

gazelle_rust_dependencies2()

"""A module defining the transitive dependencies of cargo-raze"""

load("@rules_foreign_cc//foreign_cc:repositories.bzl", "rules_foreign_cc_dependencies")
load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies")

def cargo_raze_transitive_deps():
    """Loads all dependnecies from repositories required for cargo-raze"""
    rules_foreign_cc_dependencies()
    rules_rust_dependencies()

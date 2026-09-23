#!/usr/bin/env python3
"""Check the boundary rules that a regex can decide exactly.

Every rule here comes from a section of [ARCHITECTURE.md](../docs/ARCHITECTURE.md)
and holds only where the allowed paths can be named without guessing. A rule
that would need to read intent belongs in review, not here, so this file grows
one exact rule at a time rather than one heuristic at a time.
"""

import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
CORE = ROOT / "core/gemstone/src"

SOURCES = [("ios/Features", "ios/Packages", "ios/Gem", "ios/GemPriceWidget"), ("android",)]
SUFFIXES = {".swift", ".kt"}
SKIP_DIRS = {"build", ".build", "generated", "Submodules", "DerivedData", "test", "androidTest", "testFixtures", "Tests", "TestKit"}
SKIP_FILES = {"Gemstone.swift", "gemstone.kt"}

SERVICE_STRUCT = re.compile(r"pub struct (Gem\w*Service)\s*(\{[^}]*\})", re.S)
SERVICE_FIELD = re.compile(r"^\s*(?:pub(?:\([^)]*\))?\s+)?\w+\s*:", re.M)
SERVICE_CALL = re.compile(r"\b(Gem\w*Service)\s*\(")
# The composition roots § 8 names: the factory that owns the graph, the
# per-screen factory it hands to views, the gateway both build from, the
# keystore layer that builds what it will not hand out, and Android's
# dependency injection modules.
COMPOSITION = re.compile(r"(ServicesFactory\.swift|ViewModelFactory[^/]*\.swift|Gateway/GatewayService\.swift|LocalKeystore\+Services\.swift|/di/)")

LOCALIZED_MAPPER = re.compile(r"(?:extension GemLocalizedText\b(?!: Sendable)|fun GemLocalizedText\.)")
LOCALIZED_HOMES = {"Gemstone+Localized.swift", "GemstoneText.kt"}

KEYSTORE = re.compile(r"\bGemKeystore\b")
KEYSTORE_LAYERS = re.compile(r"(ios/Packages/GemstoneServices/|android/data/services/gemstone/)")


def app_files():
    for roots in SOURCES:
        for root in roots:
            for path in sorted((ROOT / root).rglob("*")):
                if path.suffix not in SUFFIXES or path.name in SKIP_FILES:
                    continue
                if set(path.relative_to(ROOT).parts) & SKIP_DIRS:
                    continue
                yield path


def stateless_services():
    """A service Core declares with no fields carries no state to substitute."""
    names = set()
    for path in CORE.rglob("*.rs"):
        for name, body in SERVICE_STRUCT.findall(path.read_text()):
            if not SERVICE_FIELD.search(body):
                names.add(name)
    return names


def services_are_injected():
    """§ 8: a service comes from the composition root, never from a call site."""
    stateless = stateless_services()
    for path in app_files():
        relative = str(path.relative_to(ROOT))
        if COMPOSITION.search(relative):
            continue
        for number, line in enumerate(path.read_text().splitlines(), start=1):
            for name in SERVICE_CALL.findall(line):
                if name not in stateless:
                    yield f"{relative}:{number} builds {name} outside the composition root"


def one_localization_mapper():
    """One mapper per app names every Core key it renders, in one place."""
    for path in app_files():
        if path.name in LOCALIZED_HOMES:
            continue
        for number, line in enumerate(path.read_text().splitlines(), start=1):
            if LOCALIZED_MAPPER.search(line):
                yield f"{path.relative_to(ROOT)}:{number} renders GemLocalizedText outside its module mapper"


def the_keystore_stays_in_its_layer():
    """§ 8: an app takes the services that sign, never the keystore they sign with."""
    for path in app_files():
        relative = str(path.relative_to(ROOT))
        if KEYSTORE_LAYERS.search(relative):
            continue
        for number, line in enumerate(path.read_text().splitlines(), start=1):
            if KEYSTORE.search(line):
                yield f"{relative}:{number} reaches for the keystore outside its layer"


BACKEND = ROOT / "core"
INFRA_CRATES = {"storage", "cacher", "streamer", "search_index", "pusher"}
INFRA_DEPENDENTS = {
    "services": INFRA_CRATES,
    "api": {"cacher", "pusher", "search_index", "storage", "streamer"},
    "daemon": {"cacher", "pusher", "search_index", "storage", "streamer"},
    "fiat": {"cacher", "storage", "streamer"},
    "rewards": {"cacher", "storage"},
}
CARGO_SECTION = re.compile(r"^\[(.+)\]\s*$")
CARGO_KEY = re.compile(r"^([A-Za-z0-9_-]+)\s*=\s*(.*)$")
DEPENDENCY_SECTIONS = {"dependencies", "dev-dependencies", "build-dependencies"}


def cargo_packages():
    for path in sorted(BACKEND.rglob("Cargo.toml")):
        if "target" in path.relative_to(BACKEND).parts or path.parent == BACKEND:
            continue
        name, section, dependencies = None, None, set()
        for line in path.read_text().splitlines():
            header = CARGO_SECTION.match(line)
            if header:
                section = header.group(1)
                continue
            key = CARGO_KEY.match(line)
            if not key or section is None:
                continue
            if section == "package" and key.group(1) == "name":
                name = key.group(2).strip().strip('"')
            elif section.split(".")[-1] in DEPENDENCY_SECTIONS:
                dependencies.add(key.group(1))
        yield path.relative_to(ROOT), name, dependencies


def only_services_reach_infra():
    """core/skills/architecture.md § Backend Layers: only services depends on infra crates."""
    for path, name, dependencies in cargo_packages():
        if name in INFRA_CRATES:
            continue
        used = dependencies & INFRA_CRATES
        allowed = INFRA_DEPENDENTS.get(name, set())
        for crate in sorted(used - allowed):
            yield f"{path} depends on infra crate {crate}"
        if name != "services":
            for crate in sorted(allowed - used):
                yield f"{path} no longer depends on {crate}; remove it from INFRA_DEPENDENTS"


RULES = [
    ("services are injected, never constructed at a call site", services_are_injected),
    ("one localization mapper names every Core key it renders", one_localization_mapper),
    ("the keystore stays in its layer", the_keystore_stays_in_its_layer),
    ("only services depends on infra crates", only_services_reach_infra),
]


def main():
    failures = 0
    for rule, check in RULES:
        found = sorted(check())
        for line in found:
            print(f"  {line}")
        failures += len(found)
    print(f"checked {len(RULES)} boundary rules")

    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())

#!/usr/bin/env python3
"""
spec.py - Technical Spec Validation Tool

Basic internal consistency checks for spec.yaml.
For semantic comparison and drift detection, use AI.

Usage:
    python spec.py validate              # Run all checks
    python spec.py validate --fix        # Auto-fix cross-references
    python spec.py orphans               # Types defined but never used
    python spec.py missing               # Types referenced but not defined
    python spec.py refs                  # Verify all cross-references
    python spec.py naming                # Check naming conventions
    python spec.py architecture          # Check architecture patterns
    python spec.py usages <type>         # Show where a type is used
    python spec.py deps <component>      # Show component dependencies
"""

import sys
import re
from pathlib import Path
from typing import Any

try:
    import yaml
except ImportError:
    print("Error: PyYAML required. Install with: pip install pyyaml")
    sys.exit(1)


SPEC_FILE = "spec.yaml"


def load_spec(path: str = SPEC_FILE) -> dict:
    """Load and parse spec.yaml."""
    spec_path = Path(path)
    if not spec_path.exists():
        print(f"Error: {path} not found")
        sys.exit(1)

    with open(spec_path) as f:
        return yaml.safe_load(f)


def save_spec(spec: dict, path: str = SPEC_FILE) -> None:
    """Save spec.yaml."""
    with open(path, 'w') as f:
        yaml.dump(spec, f, default_flow_style=False, sort_keys=False, allow_unicode=True)


def get_defined_types(spec: dict) -> set[str]:
    """Get all type names defined in spec."""
    types_section = spec.get('types', {})
    return set(types_section.keys())


def get_used_types(spec: dict) -> dict[str, list[str]]:
    """Get all types referenced in components with their locations."""
    used = {}
    components = spec.get('components', {})

    for comp_name, comp in components.items():
        # Check provides
        for func in comp.get('provides', []):
            if isinstance(func, dict):
                for func_name, func_def in func.items():
                    location = f"{comp_name}.{func_name}"

                    # Parse input type
                    input_type = func_def.get('input', '')
                    if input_type:
                        for t in parse_type_refs(input_type):
                            used.setdefault(t, []).append(location)

                    # Parse output type
                    output_type = func_def.get('output', '')
                    if output_type:
                        for t in parse_type_refs(output_type):
                            used.setdefault(t, []).append(location)

        # Check emits
        for event in comp.get('emits', []):
            if isinstance(event, dict):
                for event_name, event_def in event.items():
                    location = f"{comp_name}.emits.{event_name}"
                    used.setdefault(event_name, []).append(location)

                    payload = event_def.get('payload', '')
                    if payload:
                        for t in parse_type_refs(payload):
                            used.setdefault(t, []).append(location)

        # Check subscribes
        for sub in comp.get('subscribes', []):
            if isinstance(sub, dict):
                for event_name in sub.keys():
                    location = f"{comp_name}.subscribes"
                    used.setdefault(event_name, []).append(location)

        # Check owns_data
        for owned in comp.get('owns_data', []):
            location = f"{comp_name}.owns_data"
            used.setdefault(owned, []).append(location)

    return used


def parse_type_refs(type_str: str) -> list[str]:
    """Parse type string and extract all type references.

    Handles: Type, Type | Error, Option<Type>, Result<T, E>, etc.
    """
    # Remove common wrappers
    type_str = re.sub(r'Result<([^,]+),\s*([^>]+)>', r'\1 | \2', type_str)
    type_str = re.sub(r'Option<([^>]+)>', r'\1', type_str)
    type_str = re.sub(r'Vec<([^>]+)>', r'\1', type_str)
    type_str = re.sub(r'List<([^>]+)>', r'\1', type_str)
    type_str = re.sub(r'\[\]', '', type_str)

    # Split on | and extract type names
    parts = re.split(r'\s*\|\s*', type_str)
    types = []

    for part in parts:
        part = part.strip()
        # Skip primitive types
        if part.lower() in ('string', 'int', 'bool', 'float', 'none', 'null', 'void', ''):
            continue
        # Extract PascalCase type names
        matches = re.findall(r'[A-Z][a-zA-Z0-9]*', part)
        types.extend(matches)

    return types


def get_declared_usages(spec: dict) -> dict[str, list[str]]:
    """Get declared usages from types section."""
    declared = {}
    types_section = spec.get('types', {})

    for type_name, type_def in types_section.items():
        if isinstance(type_def, dict):
            declared[type_name] = type_def.get('used', [])

    return declared


def check_orphans(spec: dict) -> list[str]:
    """Find types defined but never used."""
    defined = get_defined_types(spec)
    used = set(get_used_types(spec).keys())
    return sorted(defined - used)


def check_missing(spec: dict) -> list[str]:
    """Find types referenced but not defined."""
    defined = get_defined_types(spec)
    used = set(get_used_types(spec).keys())
    return sorted(used - defined)


def check_refs(spec: dict) -> list[dict]:
    """Check that declared 'used' fields match actual usage."""
    declared = get_declared_usages(spec)
    actual = get_used_types(spec)

    issues = []

    for type_name, declared_refs in declared.items():
        actual_refs = actual.get(type_name, [])

        declared_set = set(declared_refs)
        actual_set = set(actual_refs)

        missing_in_declared = actual_set - declared_set
        extra_in_declared = declared_set - actual_set

        if missing_in_declared:
            issues.append({
                'type': type_name,
                'issue': 'missing_in_declared',
                'refs': sorted(missing_in_declared)
            })

        if extra_in_declared:
            issues.append({
                'type': type_name,
                'issue': 'extra_in_declared',
                'refs': sorted(extra_in_declared)
            })

    return issues


VALID_GLOBAL_PATTERNS = {
    'dependency_injection': {'constructor', 'interface', 'container'},
    'error_handling': {'result_types', 'exceptions', 'error_codes'},
    'async': {'async_await', 'channels', 'actors', 'sync'}
}

VALID_COMPONENT_PATTERNS = {
    'strategy', 'factory', 'state_machine', 'repository', 'observer', 'decorator'
}


def check_architecture(spec: dict) -> list[dict]:
    """Check architecture section validity."""
    issues = []
    architecture = spec.get('architecture', {})

    if not architecture:
        return issues  # Architecture section is optional

    # Check global patterns
    global_patterns = architecture.get('global_patterns', {})
    for pattern_name, pattern_def in global_patterns.items():
        if pattern_name not in VALID_GLOBAL_PATTERNS:
            issues.append({
                'issue': 'unknown_global_pattern',
                'name': pattern_name,
                'location': 'architecture.global_patterns'
            })
        elif isinstance(pattern_def, dict):
            approach = pattern_def.get('approach', '')
            valid_approaches = VALID_GLOBAL_PATTERNS.get(pattern_name, set())
            if approach and approach not in valid_approaches:
                issues.append({
                    'issue': 'invalid_approach',
                    'name': pattern_name,
                    'approach': approach,
                    'valid': sorted(valid_approaches),
                    'location': f'architecture.global_patterns.{pattern_name}'
                })

    # Check component patterns reference valid components
    component_patterns = architecture.get('component_patterns', {})
    defined_components = set(spec.get('components', {}).keys())

    for comp_name, pattern_def in component_patterns.items():
        if comp_name not in defined_components:
            issues.append({
                'issue': 'undefined_component',
                'name': comp_name,
                'location': 'architecture.component_patterns'
            })

        if isinstance(pattern_def, dict):
            pattern = pattern_def.get('pattern', '')
            if pattern and pattern not in VALID_COMPONENT_PATTERNS:
                issues.append({
                    'issue': 'invalid_component_pattern',
                    'name': comp_name,
                    'pattern': pattern,
                    'valid': sorted(VALID_COMPONENT_PATTERNS),
                    'location': f'architecture.component_patterns.{comp_name}'
                })

    return issues


def check_naming(spec: dict) -> list[dict]:
    """Check naming conventions."""
    issues = []
    conventions = spec.get('conventions', {})

    type_case = conventions.get('type_case', 'PascalCase')
    function_case = conventions.get('function_case', 'snake_case')

    # Check type names
    for type_name in spec.get('types', {}).keys():
        if type_case == 'PascalCase' and not re.match(r'^[A-Z][a-zA-Z0-9]*$', type_name):
            issues.append({
                'name': type_name,
                'expected': 'PascalCase',
                'location': 'types'
            })

    # Check component and function names
    for comp_name, comp in spec.get('components', {}).items():
        if function_case == 'snake_case' and not re.match(r'^[a-z][a-z0-9_]*$', comp_name):
            issues.append({
                'name': comp_name,
                'expected': 'snake_case',
                'location': 'components'
            })

        for func in comp.get('provides', []):
            if isinstance(func, dict):
                for func_name in func.keys():
                    if function_case == 'snake_case' and not re.match(r'^[a-z][a-z0-9_]*$', func_name):
                        issues.append({
                            'name': func_name,
                            'expected': 'snake_case',
                            'location': f'components.{comp_name}.provides'
                        })
                    elif function_case == 'camelCase' and not re.match(r'^[a-z][a-zA-Z0-9]*$', func_name):
                        issues.append({
                            'name': func_name,
                            'expected': 'camelCase',
                            'location': f'components.{comp_name}.provides'
                        })
                    elif function_case == 'PascalCase' and not re.match(r'^[A-Z][a-zA-Z0-9]*$', func_name):
                        issues.append({
                            'name': func_name,
                            'expected': 'PascalCase',
                            'location': f'components.{comp_name}.provides'
                        })

    return issues


def fix_refs(spec: dict) -> dict:
    """Auto-fix the 'used' fields to match actual usage."""
    actual = get_used_types(spec)

    types_section = spec.get('types', {})
    for type_name in types_section:
        if isinstance(types_section[type_name], dict):
            types_section[type_name]['used'] = sorted(actual.get(type_name, []))
        else:
            types_section[type_name] = {
                'for': types_section[type_name] if isinstance(types_section[type_name], str) else '',
                'used': sorted(actual.get(type_name, []))
            }

    return spec


def show_usages(spec: dict, type_name: str) -> None:
    """Show where a type is used."""
    actual = get_used_types(spec)

    if type_name not in get_defined_types(spec):
        print(f"Warning: '{type_name}' is not defined in types section")

    usages = actual.get(type_name, [])
    if usages:
        print(f"Type '{type_name}' is used in:")
        for usage in sorted(usages):
            print(f"  - {usage}")
    else:
        print(f"Type '{type_name}' is not used anywhere")


def show_deps(spec: dict, component_name: str) -> None:
    """Show component dependencies."""
    components = spec.get('components', {})

    if component_name not in components:
        print(f"Error: Component '{component_name}' not found")
        return

    comp = components[component_name]

    print(f"Component '{component_name}' dependencies:")

    # Internal dependencies
    consumes = comp.get('consumes', [])
    if consumes:
        print("  Consumes (internal):")
        for dep in consumes:
            print(f"    - {dep}")

    # External dependencies
    integrations = spec.get('integrations', [])
    external = [i['name'] for i in integrations if component_name in i.get('consumed_by', [])]
    if external:
        print("  Integrations (external):")
        for dep in external:
            print(f"    - {dep}")

    # Data ownership
    owns = comp.get('owns_data', [])
    if owns:
        print("  Owns data:")
        for data in owns:
            print(f"    - {data}")

    # Events
    emits = comp.get('emits', [])
    if emits:
        print("  Emits events:")
        for event in emits:
            if isinstance(event, dict):
                for event_name in event.keys():
                    print(f"    - {event_name}")

    subscribes = comp.get('subscribes', [])
    if subscribes:
        print("  Subscribes to:")
        for sub in subscribes:
            if isinstance(sub, dict):
                for event_name in sub.keys():
                    print(f"    - {event_name}")


def cmd_validate(spec: dict, fix: bool = False) -> int:
    """Run all validation checks."""
    print("Validating spec.yaml...\n")

    issues_found = 0

    # Check orphans
    orphans = check_orphans(spec)
    if orphans:
        print(f"⚠ Orphan types (defined but never used): {len(orphans)}")
        for t in orphans:
            print(f"    - {t}")
        issues_found += len(orphans)
    else:
        print("✓ No orphan types")

    # Check missing
    missing = check_missing(spec)
    if missing:
        print(f"\n✗ Missing types (referenced but not defined): {len(missing)}")
        for t in missing:
            print(f"    - {t}")
        issues_found += len(missing)
    else:
        print("✓ No missing types")

    # Check refs
    ref_issues = check_refs(spec)
    if ref_issues:
        print(f"\n⚠ Cross-reference issues: {len(ref_issues)}")
        for issue in ref_issues:
            if issue['issue'] == 'missing_in_declared':
                print(f"    {issue['type']}: missing in 'used' field:")
                for ref in issue['refs']:
                    print(f"      - {ref}")
            else:
                print(f"    {issue['type']}: extra in 'used' field:")
                for ref in issue['refs']:
                    print(f"      - {ref}")
        issues_found += len(ref_issues)

        if fix:
            print("\n  Auto-fixing cross-references...")
            fixed_spec = fix_refs(spec)
            save_spec(fixed_spec)
            print("  ✓ Cross-references fixed")
            issues_found -= len(ref_issues)
    else:
        print("✓ Cross-references valid")

    # Check naming
    naming_issues = check_naming(spec)
    if naming_issues:
        print(f"\n⚠ Naming convention issues: {len(naming_issues)}")
        for issue in naming_issues:
            print(f"    {issue['name']}: expected {issue['expected']} (in {issue['location']})")
        issues_found += len(naming_issues)
    else:
        print("✓ Naming conventions followed")

    # Check architecture
    arch_issues = check_architecture(spec)
    if arch_issues:
        print(f"\n⚠ Architecture issues: {len(arch_issues)}")
        for issue in arch_issues:
            if issue['issue'] == 'unknown_global_pattern':
                print(f"    Unknown global pattern: {issue['name']}")
            elif issue['issue'] == 'invalid_approach':
                print(f"    Invalid approach for {issue['name']}: '{issue['approach']}' (valid: {issue['valid']})")
            elif issue['issue'] == 'undefined_component':
                print(f"    Component pattern for undefined component: {issue['name']}")
            elif issue['issue'] == 'invalid_component_pattern':
                print(f"    Invalid pattern for {issue['name']}: '{issue['pattern']}' (valid: {issue['valid']})")
        issues_found += len(arch_issues)
    else:
        print("✓ Architecture patterns valid")

    # Summary
    print(f"\n{'─' * 40}")
    if issues_found == 0:
        print("✓ All checks passed")
        return 0
    else:
        print(f"✗ {issues_found} issue(s) found")
        return 1


def cmd_orphans(spec: dict) -> int:
    """Show orphan types."""
    orphans = check_orphans(spec)
    if orphans:
        print("Orphan types (defined but never used):")
        for t in orphans:
            print(f"  - {t}")
        return 1
    else:
        print("No orphan types found")
        return 0


def cmd_missing(spec: dict) -> int:
    """Show missing types."""
    missing = check_missing(spec)
    if missing:
        print("Missing types (referenced but not defined):")
        for t in missing:
            print(f"  - {t}")
        return 1
    else:
        print("No missing types found")
        return 0


def cmd_refs(spec: dict) -> int:
    """Show cross-reference issues."""
    issues = check_refs(spec)
    if issues:
        print("Cross-reference issues:")
        for issue in issues:
            print(f"  {issue['type']}:")
            if issue['issue'] == 'missing_in_declared':
                print("    Missing in 'used' field:")
            else:
                print("    Extra in 'used' field (not actually used):")
            for ref in issue['refs']:
                print(f"      - {ref}")
        return 1
    else:
        print("All cross-references valid")
        return 0


def cmd_naming(spec: dict) -> int:
    """Show naming convention issues."""
    issues = check_naming(spec)
    if issues:
        print("Naming convention issues:")
        for issue in issues:
            print(f"  {issue['name']}: expected {issue['expected']} (in {issue['location']})")
        return 1
    else:
        print("All naming conventions followed")
        return 0


def cmd_architecture(spec: dict) -> int:
    """Show architecture issues."""
    issues = check_architecture(spec)
    if issues:
        print("Architecture issues:")
        for issue in issues:
            if issue['issue'] == 'unknown_global_pattern':
                print(f"  Unknown global pattern: {issue['name']}")
            elif issue['issue'] == 'invalid_approach':
                print(f"  Invalid approach for {issue['name']}: '{issue['approach']}'")
                print(f"    Valid approaches: {issue['valid']}")
            elif issue['issue'] == 'undefined_component':
                print(f"  Component pattern for undefined component: {issue['name']}")
            elif issue['issue'] == 'invalid_component_pattern':
                print(f"  Invalid pattern for {issue['name']}: '{issue['pattern']}'")
                print(f"    Valid patterns: {issue['valid']}")
        return 1
    else:
        print("All architecture patterns valid")
        return 0


def main():
    if len(sys.argv) < 2:
        print(__doc__)
        return 1

    cmd = sys.argv[1]

    if cmd == 'validate':
        spec = load_spec()
        fix = '--fix' in sys.argv
        return cmd_validate(spec, fix)

    elif cmd == 'orphans':
        spec = load_spec()
        return cmd_orphans(spec)

    elif cmd == 'missing':
        spec = load_spec()
        return cmd_missing(spec)

    elif cmd == 'refs':
        spec = load_spec()
        return cmd_refs(spec)

    elif cmd == 'naming':
        spec = load_spec()
        return cmd_naming(spec)

    elif cmd == 'architecture':
        spec = load_spec()
        return cmd_architecture(spec)

    elif cmd == 'usages':
        if len(sys.argv) < 3:
            print("Usage: python spec.py usages <type_name>")
            return 1
        spec = load_spec()
        show_usages(spec, sys.argv[2])
        return 0

    elif cmd == 'deps':
        if len(sys.argv) < 3:
            print("Usage: python spec.py deps <component_name>")
            return 1
        spec = load_spec()
        show_deps(spec, sys.argv[2])
        return 0

    else:
        print(f"Unknown command: {cmd}")
        print(__doc__)
        return 1


if __name__ == '__main__':
    sys.exit(main())

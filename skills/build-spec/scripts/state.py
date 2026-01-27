#!/usr/bin/env python3
"""
State manager for build-spec phased skill.

Usage:
    python state.py init --skill-root PATH  Initialize state file with skill root
    python state.py status                  Show current progress
    python state.py get-skill-root          Get skill root path
    python state.py check-phase N           Check if phase N is complete
    python state.py start-phase N           Mark phase N as in progress
    python state.py complete-phase N        Mark phase N as complete
    python state.py get-data N key          Get data from phase N
    python state.py set-data N key value    Store data for phase N
    python state.py set-components JSON     Set all components list
    python state.py set-current NAME        Set current component
    python state.py mark-implemented NAME   Mark component as implemented (Phase 3 done)
    python state.py mark-verified NAME      Mark component as verified (Phase 4 done, requires extracted file)
    python state.py complete-component      Mark current component as completed (requires verified status)
    python state.py add-extra ...           Add extra item with classification
    python state.py get-extras              Get all extras grouped by classification
    python state.py resolve-extra ...       Resolve extra with decision
    python state.py clear-extra INDEX       Clear an extra by index
"""

import argparse
import json
import sys
from pathlib import Path
from datetime import datetime

SKILL_NAME = "build-spec"
STATE_DIR = Path(".opensdd")
STATE_FILE = STATE_DIR / f"{SKILL_NAME}.state.yaml"


def load_state():
    """Load state from file or return empty state."""
    if not STATE_FILE.exists():
        return None

    import yaml
    with open(STATE_FILE, "r") as f:
        return yaml.safe_load(f)


def save_state(state):
    """Save state to file."""
    import yaml
    STATE_DIR.mkdir(exist_ok=True)
    with open(STATE_FILE, "w") as f:
        yaml.dump(state, f, default_flow_style=False, sort_keys=False)


def cmd_init(args):
    """Initialize new state file."""
    if STATE_FILE.exists() and not args.force:
        print(json.dumps({"error": f"State file already exists: {STATE_FILE}", "hint": "Use --force to overwrite"}))
        return 1

    if not args.skill_root:
        print(json.dumps({"error": "Missing --skill-root argument"}))
        return 1

    state = {
        "skill": SKILL_NAME,
        "skill_root": args.skill_root,
        "created_at": datetime.now().isoformat(),
        "updated_at": datetime.now().isoformat(),
        "current_phase": 0,
        "phases": {},
        # Build-spec specific state
        "all_components": [],
        "completed_components": [],
        "current_component": None,
        "component_status": {},  # Tracks: selected -> implemented -> verified -> completed
        "extras": [],
        # Scaffold tracking
        "scaffold_completed": False,
        "scaffold_files": []
    }
    save_state(state)
    print(json.dumps({"status": "success", "message": f"Initialized state at {STATE_FILE}", "skill_root": args.skill_root}))
    return 0


def cmd_status(args):
    """Show current progress."""
    state = load_state()
    if not state:
        print(json.dumps({"error": "No state file found", "hint": "Run 'python state.py init' first"}))
        return 1

    print(json.dumps({
        "status": "success",
        "skill": state["skill"],
        "skill_root": state["skill_root"],
        "current_phase": state["current_phase"],
        "phases": state["phases"],
        "all_components": state.get("all_components", []),
        "completed_components": state.get("completed_components", []),
        "current_component": state.get("current_component"),
        "component_status": state.get("component_status", {}),
        "extras_count": len(state.get("extras", [])),
        "scaffold_completed": state.get("scaffold_completed", False),
        "updated_at": state["updated_at"]
    }, indent=2))
    return 0


def cmd_get_skill_root(args):
    """Get skill root path."""
    state = load_state()
    if not state:
        print(json.dumps({"error": "No state file found"}))
        return 1

    print(state["skill_root"])
    return 0


def cmd_check_phase(args):
    """Check if phase N is complete."""
    state = load_state()
    if not state:
        print(json.dumps({"error": "No state file found"}))
        return 1

    phase_key = f"phase_{args.phase}"
    phase_data = state["phases"].get(phase_key, {})
    is_complete = phase_data.get("status") == "complete"

    print(json.dumps({
        "phase": args.phase,
        "complete": is_complete,
        "status": phase_data.get("status", "not_started")
    }))
    return 0 if is_complete else 1


def cmd_start_phase(args):
    """Mark phase N as in progress."""
    state = load_state()
    if not state:
        print(json.dumps({"error": "No state file found"}))
        return 1

    phase_key = f"phase_{args.phase}"
    if phase_key not in state["phases"]:
        state["phases"][phase_key] = {}

    state["phases"][phase_key]["status"] = "in_progress"
    state["phases"][phase_key]["started_at"] = datetime.now().isoformat()
    state["current_phase"] = args.phase
    state["updated_at"] = datetime.now().isoformat()

    save_state(state)
    print(json.dumps({"status": "success", "phase": args.phase, "phase_status": "in_progress"}))
    return 0


def cmd_complete_phase(args):
    """Mark phase N as complete."""
    state = load_state()
    if not state:
        print(json.dumps({"error": "No state file found"}))
        return 1

    phase_key = f"phase_{args.phase}"
    if phase_key not in state["phases"]:
        state["phases"][phase_key] = {}

    state["phases"][phase_key]["status"] = "complete"
    state["phases"][phase_key]["completed_at"] = datetime.now().isoformat()
    state["current_phase"] = args.phase + 1
    state["updated_at"] = datetime.now().isoformat()

    # Mark scaffold as completed when phase 1 completes
    if args.phase == 1:
        state["scaffold_completed"] = True

    save_state(state)
    print(json.dumps({"status": "success", "phase": args.phase, "phase_status": "complete"}))
    return 0


def cmd_get_data(args):
    """Get data from phase N."""
    state = load_state()
    if not state:
        print(json.dumps({"error": "No state file found"}))
        return 1

    phase_key = f"phase_{args.phase}"
    phase_data = state["phases"].get(phase_key, {})
    data = phase_data.get("data", {})
    value = data.get(args.key)

    if value is None:
        print(json.dumps({"error": f"Key '{args.key}' not found in phase {args.phase}"}))
        return 1

    print(json.dumps({"phase": args.phase, "key": args.key, "value": value}))
    return 0


def cmd_set_data(args):
    """Store data for phase N."""
    state = load_state()
    if not state:
        print(json.dumps({"error": "No state file found"}))
        return 1

    phase_key = f"phase_{args.phase}"
    if phase_key not in state["phases"]:
        state["phases"][phase_key] = {}
    if "data" not in state["phases"][phase_key]:
        state["phases"][phase_key]["data"] = {}

    # Try to parse value as JSON, otherwise store as string
    try:
        value = json.loads(args.value)
    except json.JSONDecodeError:
        value = args.value

    state["phases"][phase_key]["data"][args.key] = value
    state["updated_at"] = datetime.now().isoformat()

    save_state(state)
    print(json.dumps({"status": "success", "phase": args.phase, "key": args.key, "value": value}))
    return 0


def cmd_set_components(args):
    """Set all components list."""
    state = load_state()
    if not state:
        print(json.dumps({"error": "No state file found"}))
        return 1

    try:
        components = json.loads(args.components)
    except json.JSONDecodeError:
        print(json.dumps({"error": "Components must be valid JSON array"}))
        return 1

    state["all_components"] = components
    state["updated_at"] = datetime.now().isoformat()
    save_state(state)
    print(json.dumps({"status": "success", "all_components": components}))
    return 0


def cmd_set_current(args):
    """Set current component. Fails if previous component wasn't completed."""
    state = load_state()
    if not state:
        print(json.dumps({"error": "No state file found"}))
        return 1

    # Guard: If there's already a current component, it must be completed first
    existing = state.get("current_component")
    if existing and existing != args.component:
        if existing not in state.get("completed_components", []):
            comp_status = state.get("component_status", {}).get(existing, {})
            print(json.dumps({
                "error": f"Cannot select new component. Previous component '{existing}' was not completed.",
                "previous_component": existing,
                "previous_status": comp_status.get("status", "not_started"),
                "hint": "You must complete Phase 4 (Verify) for the previous component first. Run mark-verified and complete-component."
            }))
            return 1

    state["current_component"] = args.component
    state["updated_at"] = datetime.now().isoformat()
    save_state(state)
    print(json.dumps({"status": "success", "current_component": args.component}))
    return 0


def cmd_mark_implemented(args):
    """Mark component as implemented (Phase 3 complete)."""
    state = load_state()
    if not state:
        print(json.dumps({"error": "No state file found"}))
        return 1

    component = args.component
    if "component_status" not in state:
        state["component_status"] = {}

    state["component_status"][component] = {
        "status": "implemented",
        "implemented_at": datetime.now().isoformat()
    }
    state["updated_at"] = datetime.now().isoformat()
    save_state(state)
    print(json.dumps({"status": "success", "component": component, "component_status": "implemented"}))
    return 0


def cmd_mark_verified(args):
    """Mark component as verified (Phase 4 complete). Validates extracted file exists."""
    state = load_state()
    if not state:
        print(json.dumps({"error": "No state file found"}))
        return 1

    component = args.component
    extracted_file = args.extracted_file

    # Validate extracted file exists
    if not Path(extracted_file).exists():
        print(json.dumps({
            "error": f"Extracted file not found: {extracted_file}",
            "hint": "You must run spec-extract before marking component as verified",
            "component": component
        }))
        return 1

    if "component_status" not in state:
        state["component_status"] = {}

    # Check component was implemented first
    comp_status = state["component_status"].get(component, {})
    if comp_status.get("status") not in ["implemented", "verified"]:
        print(json.dumps({
            "error": f"Component '{component}' must be implemented before it can be verified",
            "current_status": comp_status.get("status", "not_started"),
            "hint": "Run mark-implemented first"
        }))
        return 1

    state["component_status"][component] = {
        **comp_status,
        "status": "verified",
        "verified_at": datetime.now().isoformat(),
        "extracted_file": extracted_file
    }
    state["updated_at"] = datetime.now().isoformat()
    save_state(state)
    print(json.dumps({"status": "success", "component": component, "component_status": "verified", "extracted_file": extracted_file}))
    return 0


def cmd_complete_component(args):
    """Mark current component as completed. Requires component to be verified first."""
    state = load_state()
    if not state:
        print(json.dumps({"error": "No state file found"}))
        return 1

    component = state.get("current_component")
    if not component:
        print(json.dumps({"error": "No current component set"}))
        return 1

    # Check component was verified
    if "component_status" not in state:
        state["component_status"] = {}

    comp_status = state["component_status"].get(component, {})
    if comp_status.get("status") != "verified":
        print(json.dumps({
            "error": f"Component '{component}' must be verified before it can be completed",
            "current_status": comp_status.get("status", "not_started"),
            "hint": "Run spec-extract and mark-verified first. Skipping verification is not allowed."
        }))
        return 1

    # Mark as completed
    state["component_status"][component]["status"] = "completed"
    state["component_status"][component]["completed_at"] = datetime.now().isoformat()

    if component not in state["completed_components"]:
        state["completed_components"].append(component)

    state["current_component"] = None
    state["updated_at"] = datetime.now().isoformat()
    save_state(state)
    print(json.dumps({"status": "success", "completed_component": component, "completed_components": state["completed_components"]}))
    return 0


def cmd_add_extra(args):
    """Add an extra item found during verify."""
    state = load_state()
    if not state:
        print(json.dumps({"error": "No state file found"}))
        return 1

    extra = {
        "component": args.component,
        "item": args.item,
        "signature": args.signature if args.signature else None,
        "file": args.file,
        "line": args.line if args.line else None,
        "classification": args.classification if args.classification else "new_functionality",
        "used_by": args.used_by.split(",") if args.used_by else [],
        "recommendation": args.recommendation if args.recommendation else "review_for_spec",
        "resolved": False,
        "decision": None
    }
    state["extras"].append(extra)
    state["updated_at"] = datetime.now().isoformat()
    save_state(state)
    print(json.dumps({"status": "success", "extra_added": extra, "total_extras": len(state["extras"])}))
    return 0


def cmd_resolve_extra(args):
    """Resolve an extra item with decision."""
    state = load_state()
    if not state:
        print(json.dumps({"error": "No state file found"}))
        return 1

    extras = state.get("extras", [])
    found = False
    for extra in extras:
        if extra["item"] == args.item:
            extra["resolved"] = True
            extra["decision"] = args.decision
            extra["resolved_at"] = datetime.now().isoformat()
            found = True
            break

    if not found:
        print(json.dumps({"error": f"Extra '{args.item}' not found"}))
        return 1

    state["extras"] = extras
    state["updated_at"] = datetime.now().isoformat()
    save_state(state)
    print(json.dumps({"status": "success", "resolved": args.item, "decision": args.decision}))
    return 0


def cmd_get_extras(args):
    """Get all extras grouped by classification."""
    state = load_state()
    if not state:
        print(json.dumps({"error": "No state file found"}))
        return 1

    extras = state.get("extras", [])

    # Group by classification
    by_classification = {
        "helper": [],
        "infrastructure": [],
        "test": [],
        "new_functionality": []
    }

    for extra in extras:
        classification = extra.get("classification", "new_functionality")
        if classification in by_classification:
            by_classification[classification].append(extra)
        else:
            by_classification["new_functionality"].append(extra)

    # Count resolved vs unresolved
    resolved = [e for e in extras if e.get("resolved", False)]
    unresolved = [e for e in extras if not e.get("resolved", False)]

    print(json.dumps({
        "status": "success",
        "total": len(extras),
        "resolved_count": len(resolved),
        "unresolved_count": len(unresolved),
        "by_classification": by_classification,
        "extras": extras
    }, indent=2))
    return 0


def cmd_clear_extra(args):
    """Clear an extra by index."""
    state = load_state()
    if not state:
        print(json.dumps({"error": "No state file found"}))
        return 1

    extras = state.get("extras", [])
    if args.index < 0 or args.index >= len(extras):
        print(json.dumps({"error": f"Invalid index {args.index}, have {len(extras)} extras"}))
        return 1

    removed = extras.pop(args.index)
    state["extras"] = extras
    state["updated_at"] = datetime.now().isoformat()
    save_state(state)
    print(json.dumps({"status": "success", "removed": removed, "remaining_extras": len(extras)}))
    return 0


def cmd_add_scaffold_file(args):
    """Track a file created during scaffold."""
    state = load_state()
    if not state:
        print(json.dumps({"error": "No state file found"}))
        return 1

    if "scaffold_files" not in state:
        state["scaffold_files"] = []

    state["scaffold_files"].append({
        "path": args.path,
        "type": args.type,
        "created_at": datetime.now().isoformat()
    })
    state["updated_at"] = datetime.now().isoformat()
    save_state(state)
    print(json.dumps({"status": "success", "scaffold_file_added": args.path}))
    return 0


def main():
    parser = argparse.ArgumentParser(description=f"State manager for {SKILL_NAME}")
    subparsers = parser.add_subparsers(dest="command", help="Commands")

    # init
    init_parser = subparsers.add_parser("init", help="Initialize state file")
    init_parser.add_argument("--skill-root", required=True, help="Path to skill root directory")
    init_parser.add_argument("--force", action="store_true", help="Overwrite existing state")

    # status
    subparsers.add_parser("status", help="Show current progress")

    # get-skill-root
    subparsers.add_parser("get-skill-root", help="Get skill root path")

    # check-phase
    check_parser = subparsers.add_parser("check-phase", help="Check if phase is complete")
    check_parser.add_argument("phase", type=int, help="Phase number")

    # start-phase
    start_parser = subparsers.add_parser("start-phase", help="Mark phase as in progress")
    start_parser.add_argument("phase", type=int, help="Phase number")

    # complete-phase
    complete_parser = subparsers.add_parser("complete-phase", help="Mark phase as complete")
    complete_parser.add_argument("phase", type=int, help="Phase number")

    # get-data
    get_parser = subparsers.add_parser("get-data", help="Get data from phase")
    get_parser.add_argument("phase", type=int, help="Phase number")
    get_parser.add_argument("key", help="Data key")

    # set-data
    set_parser = subparsers.add_parser("set-data", help="Store data for phase")
    set_parser.add_argument("phase", type=int, help="Phase number")
    set_parser.add_argument("key", help="Data key")
    set_parser.add_argument("value", help="Data value (JSON or string)")

    # set-components
    comp_parser = subparsers.add_parser("set-components", help="Set all components list")
    comp_parser.add_argument("components", help="JSON array of component names")

    # set-current
    curr_parser = subparsers.add_parser("set-current", help="Set current component")
    curr_parser.add_argument("component", help="Component name")

    # mark-implemented
    impl_parser = subparsers.add_parser("mark-implemented", help="Mark component as implemented (Phase 3 done)")
    impl_parser.add_argument("component", help="Component name")

    # mark-verified
    verify_parser = subparsers.add_parser("mark-verified", help="Mark component as verified (Phase 4 done)")
    verify_parser.add_argument("component", help="Component name")
    verify_parser.add_argument("--extracted-file", required=True, help="Path to extracted YAML file (must exist)")

    # complete-component
    subparsers.add_parser("complete-component", help="Mark current component as completed (requires verified status)")

    # add-extra
    extra_parser = subparsers.add_parser("add-extra", help="Add extra item found during verify")
    extra_parser.add_argument("--component", required=True, help="Component name")
    extra_parser.add_argument("--item", required=True, help="Item name")
    extra_parser.add_argument("--signature", help="Full signature")
    extra_parser.add_argument("--file", required=True, help="File path")
    extra_parser.add_argument("--line", type=int, help="Line number")
    extra_parser.add_argument("--classification", choices=["helper", "infrastructure", "test", "new_functionality"], help="Classification from agent")
    extra_parser.add_argument("--used-by", help="Comma-separated list of spec functions that use this")
    extra_parser.add_argument("--recommendation", choices=["keep_internal", "review_for_spec", "review_for_removal"], help="Agent recommendation")

    # get-extras
    subparsers.add_parser("get-extras", help="Get all extras")

    # resolve-extra
    resolve_parser = subparsers.add_parser("resolve-extra", help="Resolve an extra with decision")
    resolve_parser.add_argument("--item", required=True, help="Item name to resolve")
    resolve_parser.add_argument("--decision", required=True, choices=["add_to_spec", "keep_internal", "remove"], help="Decision")

    # clear-extra
    clear_parser = subparsers.add_parser("clear-extra", help="Clear an extra by index")
    clear_parser.add_argument("index", type=int, help="Index of extra to clear")

    # add-scaffold-file
    scaffold_parser = subparsers.add_parser("add-scaffold-file", help="Track scaffold file")
    scaffold_parser.add_argument("--path", required=True, help="File path")
    scaffold_parser.add_argument("--type", required=True, help="File type (config, entrypoint, type, deployment, test)")

    args = parser.parse_args()

    if not args.command:
        parser.print_help()
        return 1

    commands = {
        "init": cmd_init,
        "status": cmd_status,
        "get-skill-root": cmd_get_skill_root,
        "check-phase": cmd_check_phase,
        "start-phase": cmd_start_phase,
        "complete-phase": cmd_complete_phase,
        "get-data": cmd_get_data,
        "set-data": cmd_set_data,
        "set-components": cmd_set_components,
        "set-current": cmd_set_current,
        "mark-implemented": cmd_mark_implemented,
        "mark-verified": cmd_mark_verified,
        "complete-component": cmd_complete_component,
        "add-extra": cmd_add_extra,
        "get-extras": cmd_get_extras,
        "resolve-extra": cmd_resolve_extra,
        "clear-extra": cmd_clear_extra,
        "add-scaffold-file": cmd_add_scaffold_file,
    }

    return commands[args.command](args)


if __name__ == "__main__":
    sys.exit(main())

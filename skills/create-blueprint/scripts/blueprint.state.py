#!/usr/bin/env python3
"""
State manager for create-blueprint phased skill.

Usage:
    python blueprint.state.py init --skill-root PATH  Initialize state file with skill root
    python blueprint.state.py status                  Show current progress
    python blueprint.state.py get-skill-root          Get skill root path
    python blueprint.state.py check-phase N           Check if phase N is complete
    python blueprint.state.py start-phase N           Mark phase N as in progress
    python blueprint.state.py complete-phase N        Mark phase N as complete
    python blueprint.state.py get-data N key          Get data from phase N
    python blueprint.state.py set-data N key value    Store data for phase N
"""

import argparse
import json
import sys
from pathlib import Path
from datetime import datetime

SKILL_NAME = "create-blueprint"
STATE_DIR = Path(".opensdd")
STATE_FILE = STATE_DIR / "blueprint.state.yaml"


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
        "phases": {}
    }
    save_state(state)
    print(json.dumps({"status": "success", "message": f"Initialized state at {STATE_FILE}", "skill_root": args.skill_root}))
    return 0


def cmd_status(args):
    """Show current progress."""
    state = load_state()
    if not state:
        print(json.dumps({"error": "No state file found", "hint": "Run 'python blueprint.state.py init' first"}))
        return 1

    print(json.dumps({
        "status": "success",
        "skill": state["skill"],
        "skill_root": state["skill_root"],
        "current_phase": state["current_phase"],
        "phases": state["phases"],
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
    }

    return commands[args.command](args)


if __name__ == "__main__":
    sys.exit(main())

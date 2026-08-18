import sys
import json
import time
import hashlib

def simulate():
    replay_path = None
    for i, arg in enumerate(sys.argv):
        if arg == '--replay' and i + 1 < len(sys.argv):
            replay_path = sys.argv[i + 1]
            break

    if not replay_path:
        print("Error: Missing --replay <path>", file=sys.stderr)
        sys.exit(1)

    parse_start = time.perf_counter()
    with open(replay_path, 'r', encoding='utf-8') as f:
        data = json.load(f)
    parse_duration_ms = (time.perf_counter() - parse_start) * 1000.0

    replay_start = time.perf_counter()
    hasher = hashlib.sha256()

    cfg = data.get('configuration')
    if cfg and 'seed' in cfg and cfg['seed'] is not None:
        hasher.update(f"seed:{cfg['seed']};".encode('utf-8'))

    steps = data.get('steps', [])
    steps_processed = 0
    total_cards = 0
    action_transitions = 0

    for i, step_batch in enumerate(steps):
        steps_processed += 1
        for j, step in enumerate(step_batch):
            action = step.get('action')
            if action:
                action_transitions += len(action)
                # compact separators for identical json serialization
                act_str = json.dumps(action, separators=(',', ':'))
                hasher.update(f"act:{i}:{j}:{act_str};".encode('utf-8'))

            obs = step.get('observation')
            if obs and isinstance(obs, dict):
                current = obs.get('current')
                if current and isinstance(current, dict):
                    players = current.get('players', [])
                    for p, player in enumerate(players):
                        deck_list = player.get('deck') or []
                        hand_list = player.get('hand') or []
                        active_list = player.get('active') or []
                        bench_list = player.get('bench') or []
                        deck_len = len(deck_list)
                        hand_len = len(hand_list)
                        active_len = len(active_list)
                        bench_len = len(bench_list)
                        total_cards += deck_len + hand_len
                        hasher.update(f"p:{p}:d{deck_len}:h{hand_len}:a{active_len}:b{bench_len};".encode('utf-8'))

            status = step.get('status')
            if status:
                hasher.update(f"st:{status};".encode('utf-8'))

    hasher.update(f"final:steps={steps_processed}:cards={total_cards}:acts={action_transitions}".encode('utf-8'))
    checksum = hasher.hexdigest()

    replay_duration_ms = (time.perf_counter() - replay_start) * 1000.0
    total_duration_ms = parse_duration_ms + replay_duration_ms
    steps_per_sec = (steps_processed / (total_duration_ms / 1000.0)) if total_duration_ms > 0 else 0.0

    output = {
        "target": "python-cli",
        "steps_processed": steps_processed,
        "parse_duration_ms": round(parse_duration_ms, 2),
        "replay_duration_ms": round(replay_duration_ms, 2),
        "total_duration_ms": round(total_duration_ms, 2),
        "steps_per_sec": round(steps_per_sec, 2),
        "checksum": checksum
    }
    print(json.dumps(output))

if __name__ == '__main__':
    simulate()

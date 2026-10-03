import os
import subprocess
import tomllib


target = next(
    line.removeprefix("host: ")
    for line in subprocess.check_output(["rustc", "-vV"], text=True).splitlines()
    if line.startswith("host: ")
)

with open(".cargo/config.toml", "rb") as config_file:
    config = tomllib.load(config_file)

target_flags = config.get("target", {}).get(target, {}).get("rustflags", [])
with open(os.environ["GITHUB_ENV"], "a", encoding="utf-8") as environment:
    environment.write(f"RUSTFLAGS={' '.join(['-D', 'warnings', *target_flags])}\n")

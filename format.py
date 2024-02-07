#!/usr/bin/env python3

import pathlib
import json
import argparse


def main():
    repo_root = pathlib.Path(__name__).parent
    runs_root = repo_root / "runs"
    default_run_path = sorted(runs_root.glob("*.json"))[-1]

    parser = argparse.ArgumentParser()
    parser.add_argument("--run", metavar="PATH", type=pathlib.Path, default=default_run_path, help="Default: %(default)s")
    args = parser.parse_args()

    data = json.loads(args.run.read_text())
    cases = sorted(data["libs"].values(), key=lambda c: (c["crate"] if c["crate"] else "", c["name"]))

    print("Name | Overhead (release) | Build (debug) | Parse (release) | Downloads | Version")
    print("-----|--------------------|---------------|-----------------|-----------|--------")
    for case in cases:
        if case["name"] != "null":
            count_link = "![Download count](https://img.shields.io/crates/dr/{})".format(case["crate"])
        else:
            count_link = "-"
        row = [
            case["name"],
            fmt_size(case, cases[0]),
            fmt_time(case, "build"),
            fmt_time(case, "run"),
            count_link,
            case["version"] if case["version"] else "-",
        ]
        print(" | ".join(row))
    print()
    print(f"*System: {data['os']} {data['os_ver']} ({data['arch']}), {data.get('rustc', '')} w/ `-j {data['cpus']}`*")


def fmt_time(case, bench):
    bench = case[bench]
    if bench is None:
        return "N/A"

    value = bench["results"][0]["median"]
    if value < 1:
        value *= 1000
        return "{:.0f}ms".format(value)
    else:
        return "{:.0f}s".format(value)


def fmt_size(case, null_case):
    delta = (case["size"] - null_case["size"]) / 1024
    return "{:,.0f} KiB".format(delta)


if __name__ == "__main__":
    main()

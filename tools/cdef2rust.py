#!/usr/bin/env python3
import sys
import re

lineregex = re.compile(r"#define\s+(\S+)\s+(\S+)")
intregex = re.compile(r"_u\((\d+)\)")
u32regex = re.compile(r"_u\((0x[0-9a-fA-F]{,8})\)")
strregex = re.compile(r"\"(\S+)\"")

if __name__=="__main__":
    with open(sys.argv[1]) as f:
        for line in f.read().splitlines():
            if line.startswith("//"):
                print(line)
            if line.startswith("#define"):
                matches = lineregex.findall(line)
                if matches:
                    v = None
                    if u32regex.match(matches[0][1]):
                        v = ("u32", u32regex.findall(matches[0][1])[0])
                    if intregex.match(matches[0][1]):
                        v = ("i32", intregex.findall(matches[0][1])[0])
                    if strregex.match(matches[0][1]):
                        v = ("&str", "\"{}\"".format(strregex.findall(matches[0][1])[0]))

                    if not v: raise Exception("Not properly parsed")
                    print("pub(super) const {}: {};".format(matches[0][0], "{} = {}".format(v[0], v[1])))